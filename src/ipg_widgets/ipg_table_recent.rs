//! ipg_table

use crate::app::{self, Message};
use crate::{access_callbacks, IpgState, TABLE_INTERNAL_IDS_START};
use crate::style::styling::get_theme_color;
use super::callbacks::{set_or_get_widget_callback_data, 
    WidgetCallbackIn, WidgetCallbackOut};
use super::helpers::{get_height, get_width, try_extract_boolean, try_extract_f64, try_extract_string, try_extract_vec_f32};

use crate::style::styling::{lighten, darken};

use iced::widget::scrollable::Viewport;
use iced::{alignment, Alignment, Background, Element, Length, Theme};
use iced::alignment::Horizontal;
use iced::widget::{container, 
    text, Column, Container, Row, Scrollable, horizontal_space, Text};

use pyo3::types::IntoPyDict;
use pyo3::{pyclass, PyObject, Python};

#[derive(Debug, Clone)]
pub struct IpgTable {
        pub id: usize,
        pub title: String,
        pub rows: usize,
        pub columns: usize,
        pub width: Length,
        pub height: Length,
        pub header: bool,
        pub control_row: bool,
        pub add_data_row_wise: bool,
        pub add_date_column_wise: bool,
        pub row_highlight: Option<IpgTableRowHighLight>,
        pub highlight_amount: f32,
        pub column_widths: Vec<f32>,
        pub column_spacing: f32,
        pub row_spacing: f32,
        pub modal_show: bool,
        pub show: bool,
        pub scroller_user_data: Option<PyObject>,
        pub scroller_id: usize,
        _scroller_pos: Vec<(String, f32)>,
}

impl IpgTable {
    pub fn new( 
        id: usize,
        title: String,
        rows: usize,
        columns: usize,
        width: Length,
        height: Length,
        header: bool,
        control_row: bool,
        add_data_row_wise: bool,
        add_date_column_wise: bool,
        row_highlight: Option<IpgTableRowHighLight>,
        highlight_amount: f32,
        column_widths: Vec<f32>,
        column_spacing: f32,
        row_spacing: f32,
        show: bool,
        modal_show: bool,
        scroller_user_data: Option<PyObject>,
        scroller_id: usize,
        ) -> Self {
        Self {
            id,
            title,
            rows,
            columns,
            width,
            height,
            header,
            control_row,
            add_data_row_wise,
            add_date_column_wise,
            row_highlight,
            highlight_amount,
            column_widths,
            column_spacing,
            row_spacing,
            modal_show,
            show,
            scroller_user_data,
            scroller_id,
            _scroller_pos: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub enum TableMessage {
    Scrolled(Viewport, usize)
}

#[derive(Debug, Clone, Copy)]
#[pyclass]
pub enum IpgTableRowHighLight {
    Darker,
    Lighter,
}


pub fn construct_table(table: IpgTable, 
                        mut content: Vec<Element<Message>>, 
                        ) 
                        -> Element<Message> {
                             
    let header = if table.header {
        let mut head = vec![];
        for _  in  0..table.columns {
            head.push(content.remove(0));
        }
        container(Row::with_children(head)
                            .spacing(table.column_spacing))
                            .into()
    } else {
        container(horizontal_space()).into()
    };

    if table.add_data_row_wise {
        let mut rows = vec![];
        for _ in 0..table.columns {
            let mut data_row: Vec<Element<Message>>= vec![];
            for _ in 0..table.rows {
                data_row.push(content.remove(0));
            }
            let row = Row::with_children(data_row)
                                            .spacing(table.column_spacing);
            let cont = container(row);
            rows.push(cont.into());
        } 

        if table.header {
            rows.insert(0, header);
        }

        let col = Column::with_children(rows)
                                            .spacing(table.row_spacing);
        container(col).into()
    } else {
        


        iced::widget::Space::new(0.0, 0.0).into()
    }
      
}


fn add_scroll(body: Element<'static, Message>, 
                height: f32,
                scroller_id: usize,
                ) -> Element<'static, Message>{
    
    Scrollable::new(body)
                    .on_scroll(move|vp| app::Message::Scrolled(vp, scroller_id))
                    .height(Length::Fixed(height))
                    .into()
    
}

fn add_header_text (header: String, width: f32) -> Element<'static, Message> {
    let txt : Element<Message> = text(header)
                                    .width(Length::Fixed(width))
                                    .align_x(Horizontal::Center)
                                    .into();
    txt
}

fn add_text_widget(label: String, width: f32) -> Element<'static, Message> {

    let txt: Element<Message> = Text::new(label)
        .width(Length::Fixed(width))
        .height(Length::Fixed(30.0))
        .align_x(Horizontal::Center)
        .into();

    txt
}

fn add_row_container(content: Element<Message>, row_index: usize,
                    highlight_amount: f32, row_highlight: Option<IpgTableRowHighLight>) 
                    -> Element<Message> {
    // Using container because text has no background 
    Container::new(content)
            .width(Length::Shrink)
            .align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center)
            .style(move|theme| table_row_theme(theme, row_index, 
                        highlight_amount,
                        row_highlight))
            .clip(true)
            .into()
}



pub fn table_callback(state: &mut IpgState, table_id: usize, message: TableMessage) {

    let mut wci = WidgetCallbackIn{id: table_id, ..Default::default()};

    match message {
        TableMessage::Scrolled(vp, scroller_id ) => {
            wci.id = scroller_id - TABLE_INTERNAL_IDS_START;
            wci.value_str = Some("scroller".to_string());
            let offsets: Vec<(String, f32)> = vec![
                ("abs_offset_x".to_string(), vp.absolute_offset().x),
                ("abs_offset_y".to_string(), vp.absolute_offset().y),
                ("rel_offset_x".to_string(), vp.relative_offset().x),
                ("rel_offset_y".to_string(), vp.relative_offset().y),
                ("rev_offset_x".to_string(), vp.absolute_offset_reversed().x),
                ("rev_offset_y".to_string(), vp.absolute_offset_reversed().y)];

            let mut wco: WidgetCallbackOut = set_or_get_widget_callback_data(state, wci);
            wco.id = table_id;
            wco.event_name = "on_scroll".to_string();
            wco.scroll_pos = offsets;
            process_callback(wco);
        }
    }
}


pub fn process_callback(wco: WidgetCallbackOut) 
{
    let app_cbs = access_callbacks();

    let callback_present = app_cbs.callbacks.get(&(wco.id, wco.event_name.clone()));

    let callback_opt = match callback_present {
        Some(cb) => cb,
        None => return,
    };

    let callback = match callback_opt {
        Some(cb) => cb,
        None => panic!("Table callback could not be found with id {}", wco.id),
    };

    let table_index: (usize, usize) = match wco.index_table {
        Some(ti) => ti,
        None => panic!("Table: Unable to find table index for callback.")
    };
    
    Python::with_gil(|py| {
        
        if wco.user_data.is_some() {
            let user_data = wco.user_data.unwrap();
            let res = 
                if wco.event_name == "on_button" {
                    callback.call1(py, (
                                wco.id,
                                table_index, 
                                user_data
                                ))
                    } else if wco.event_name == "on_checkbox" || wco.event_name == "on_toggler" {
                        callback.call1(py, (
                            wco.id,
                            table_index,
                            wco.on_toggle,  
                            user_data
                            ))
                    } else if wco.event_name == "on_scroll" {
                        callback.call1(py, (
                            wco.id,
                            wco.scroll_pos.into_py_dict_bound(py),  
                            user_data
                            ))
                    } else {
                        panic!("Table callback: Event name {} could not be found", wco.event_name)
                    };
                    
            match res {
                Ok(_) => (),
                Err(er) => panic!("Table: 4 parameters (id, widget_index, on_toggle, user_data) are required or a python error in this function. {er}"),
            }
        } else {
            
            let res = 
                if wco.event_name == "on_button" {
                    callback.call1(py, (
                                wco.id,
                                table_index, 
                                ))
                    } else if wco.event_name == "on_checkbox" || wco.event_name == "on_toggler" {
                        callback.call1(py, (
                            wco.id,
                            table_index,
                            wco.on_toggle,  
                            ))
                    } else if wco.event_name == "on_scroll" {
                        callback.call1(py, (
                            wco.id,
                            wco.scroll_pos.into_py_dict_bound(py),  
                            ))
                    } else {
                        panic!("Table callback: Event name {} could not be found", wco.event_name)
                };
            match res {
                Ok(_) => (),
                Err(er) => panic!("Table: if on_scroll, 2 parameters (id, scroll_pos), else 3 parameter (id, widget_index, on_toggle) are required or possibly a python error in this function. {er}"),
            }
        }
    });
 
    drop(app_cbs);
         
}

// widgets = (id, row idx, col idx, bool)
fn check_for_widget(widgets: &[(usize, usize, usize, bool)], row_index: usize, col_index: usize) -> Option<usize> {
    // if empty return
    if widgets.is_empty() {return None}
    // if not column return
    if widgets[0].2 != col_index {return None}

    // Because of possible mixed columns of widgets, need the index
    for (index, widget) in widgets.iter().enumerate() {
        if row_index == widget.1 {
            return Some(index)
        } 
    }
    None
}

fn table_row_theme(theme: &Theme, idx: usize, amount: f32, 
                        highlighter: Option<IpgTableRowHighLight>) -> container::Style {

    let mut background = get_theme_color(theme);

    if idx % 2 == 0 {
        background = match highlighter {
                Some(hl) => 
                    match hl {
                        IpgTableRowHighLight::Darker => darken(background, amount),
                        IpgTableRowHighLight::Lighter => lighten(background, amount),
                        },
                None => background,
            }
    }; 
    
    container::Style {
        background: Some(Background::Color(background)),
        ..Default::default()
    }
}


#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgTableParam {
    Title,
    Width,
    Height,
    RowHighlight,
    HighlightAmount,
    ColumnWidths,
    ModalShow,
    Show,
}

pub fn table_item_update( 
                    table: &mut IpgTable,
                    item: PyObject,
                    value: PyObject,
                    ) 
{
    
    let update = try_extract_table_update(item);
    
    match update {
        IpgTableParam::Title => {
            table.title = try_extract_string(value);
        },
        IpgTableParam::Width => {
            let width = Some(try_extract_f64(value) as f32);
            table.width = get_width(width, false);
        },
        IpgTableParam::Height => {
            let height = Some(try_extract_f64(value) as f32);
            table.height = get_height(height, false);
        },
        IpgTableParam::RowHighlight => {
            table.row_highlight = Some(try_extract_row_highlight(value));
        },
        IpgTableParam::HighlightAmount => {
            table.highlight_amount = try_extract_f64(value) as f32;
        },
        IpgTableParam::ColumnWidths => {
            table.column_widths = try_extract_vec_f32(value);
        },
        IpgTableParam::ModalShow => {
            table.modal_show = try_extract_boolean(value);
        },
        IpgTableParam::Show => {
            table.show = try_extract_boolean(value);
        },
    }
}

pub fn try_extract_table_update(update_obj: PyObject) -> IpgTableParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgTableParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Table update extraction failed"),
        }
    })
}

fn try_extract_row_highlight(value: PyObject) -> IpgTableRowHighLight {
    Python::with_gil(|py| {
        let res = value.extract::<IpgTableRowHighLight>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Table update extraction of IpgTableRowHighLight failed"),
        }
    })
}
