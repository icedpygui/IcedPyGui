#![allow(unused)]
use std::collections::HashMap;
use std::result::Result;

use crate::app::{self, Message};
use crate::{access_callbacks, access_state, add_callback_to_mutex, find_parent_uid};
use crate::style::styling::{get_theme_color, IpgStyleStandard};
use super::callbacks::{get_set_widget_callback_data, WidgetCallbackIn, WidgetCallbackOut};
use super::helpers::{try_extract_boolean, try_extract_f64, try_extract_string, try_extract_u64, try_extract_vec_f32};
use super::ipg_theme_colors::{get_alt_color, IpgColorAction};
use super::ipg_button;
use crate::style::styling::{lighten, darken};

use iced::widget::scrollable::{RelativeOffset, Viewport};
use iced::{Border, Color, Point, Shadow};
use iced::mouse::Interaction;
use iced::widget::text::{LineHeight, Style};
use iced::{alignment, theme, Background, Element, Length, Padding, Renderer, Theme};
use iced::alignment::Alignment;
use iced::widget::{center, container, mouse_area, opaque, stack, text, Button, Checkbox, Column, Container, Image, MouseArea, Row, Scrollable, Space, Text, Toggler};
use iced::alignment::Horizontal;
use iced::widget::svg;
use iced::advanced::image;


use pyo3::{pyclass, PyErr, PyObject, Python};


#[derive(Debug, Clone)]
pub enum TableData {
    Row,
    Column,
}


#[derive(Debug, Clone)]
pub struct IpgTable {
        pub id: usize,
        pub title: String,
        pub data: PyObject,
        pub data_length: usize,
        pub width: f32,
        pub height: f32,
        pub row_highlight: Option<IpgTableRowHighLight>,
        pub highlight_amount: f32,
        pub column_widths: Vec<f32>,
        pub button_ids: Vec<(usize, usize, usize, bool)>, // id, row, column, toggled
        pub checkbox_ids: Vec<(usize, usize, usize, bool)>,
        pub toggler_ids: Vec<(usize, usize, usize, bool)>,
        pub button_fill_style_id: Option<String>,
        pub checkbox_fill_style_id: Option<String>,
        pub toggler_fill_style_id: Option<String>,
        pub mixed_widgets_column_style_ids: Option<HashMap<usize, Vec<String>>>,
        pub modal_show: bool,
        pub show: bool,
        pub user_data: Option<PyObject>,
        pub scroller_id: usize,
}

impl IpgTable {
    pub fn new( 
        id: usize,
        title: String,
        data: PyObject,
        data_length: usize,
        width: f32,
        height: f32,
        row_highlight: Option<IpgTableRowHighLight>,
        highlight_amount: f32,
        column_widths: Vec<f32>,
        button_ids: Vec<(usize, usize, usize, bool)>,
        checkbox_ids: Vec<(usize, usize, usize, bool)>,
        toggler_ids: Vec<(usize, usize, usize, bool)>,
        button_fill_style_id: Option<String>,
        checkbox_fill_style_id: Option<String>,
        toggler_fill_style_id: Option<String>,
        mixed_widgets_column_style_ids: Option<HashMap<usize, Vec<String>>>,
        show: bool,
        user_data: Option<PyObject>,
        scroller_id: usize,
        ) -> Self {
        Self {
            id,
            title,
            data,
            data_length,
            width,
            height,
            row_highlight,
            highlight_amount,
            column_widths,
            button_ids,
            checkbox_ids,
            toggler_ids,
            button_fill_style_id,
            checkbox_fill_style_id,
            toggler_fill_style_id,
            mixed_widgets_column_style_ids,
            modal_show: false,
            show,
            user_data,
            scroller_id,
        }
    }
}

#[derive(Debug, Clone)]
pub enum TableMessage {
    TableButton((usize, usize)),
    TableCheckbox(bool, (usize, usize)),
    TableToggler(bool, (usize, usize)),
    TableScrolled(Viewport, usize),
}

#[derive(Debug, Clone, Copy)]
enum DataTypes {
    Bool,
    F64,
    String,
}

#[derive(Debug, Clone, Copy)]
#[pyclass]
pub enum IpgTableRowHighLight {
    Darker,
    Lighter,
}

#[derive(Debug, Clone, Copy)]
#[pyclass]
pub enum IpgTableWidget {
    Button,
    Checkbox,
    Toggler,
}

#[derive(Debug, Clone)]
struct Data {
    index: usize,
    d_type: DataTypes,
    data: Vec<DataTypes>
}

#[derive(Debug, Clone)]
pub struct TableScrollerPosition {
    pub table_id: usize,
    pub relative_offset_x: f32,
    pub relative_offset_y: f32,
    pub absolute_offset_x: f32,
    pub absolute_offset_y: f32,
    pub content_bound_height: f32,
    pub bounds_height: f32,
}

impl Default for TableScrollerPosition {
    fn default() -> Self {
        Self { 
            table_id: 0, 
            relative_offset_x: 0.0, 
            relative_offset_y: 0.0,
            absolute_offset_x: 0.0,
            absolute_offset_y: 0.0,
            content_bound_height: 0.0,
            bounds_height: 0.0,
        }
    }
}


pub fn contruct_table(table: IpgTable, content: Vec<Element<'static, Message>>) -> Element<'static, Message> {

    let mut headers: Vec<Element<Message>>= vec![];

    let mut column_elements: Vec<Element<Message>> = vec![];

    let mut data_rows: Vec<Vec<String>> = vec![];
    data_rows.push(vec![]);

    let mut state = access_state();
    let mut scroller_pos_opt = state.table_internal_ids.get(&table.scroller_id);
    let scroller_pos = if scroller_pos_opt.is_some() {
        scroller_pos_opt.unwrap()
    } else {
        panic!("Table: Scroller id '{}' not found", table.scroller_id)
    };

    drop(state);

    Python::with_gil(|py| {
        let table_data = match table.data.extract::<Vec<PyObject>>(py) {
            Ok(dt) => dt,
            Err(e) => panic!("Table: Unable to extract Table data {e}"),
        };
        // Gets the entire column at each iteration
        for (col_index, py_data) in table_data.iter().enumerate() {
            
            let width = if col_index >= table.column_widths.len() {
                                    table.column_widths[0]
                                } else {
                                    table.column_widths[col_index]
                                };
            let mut error: Option<PyErr> = None;
            let data: Result<HashMap<String, Vec<bool>>, _> = py_data.extract::<HashMap<String, Vec<bool>>>(py);

            match data {
                Ok(dt) => {
                    //Only pushes the one header
                    for key in dt.keys() {
                        dbg!(&key);
                        headers.push(add_header_text(key.to_owned(), width));                                         
                    }

                    // dt.values are the columns in the table
                    for values in dt.values() {
                        for (i, v) in values.iter().enumerate() {
                            let mut label = if *v {
                                "True".to_string()
                            } else {
                                "False".to_string()
                            };
                            if data_rows.len() <= i {
                                data_rows.push(vec![])
                            }
                            data_rows[i].push(label);
                        }
                    }
                    continue;
                },
                Err(e) => error = Some(e),
            };
                 
            let data: Result<HashMap<String, Vec<i64>>, _> = py_data.extract::<HashMap<String, Vec<i64>>>(py); 
            match data {
                Ok(dt) => {
                    for key in dt.keys() {
                        headers.push(add_header_text(key.to_owned(), width));  
                    }

                    for values in dt.values() {
                        for (i, v) in values.iter().enumerate() {
                            let label = v.to_string();
                            if data_rows.len() <= i {
                                data_rows.push(vec![])
                            }
                            data_rows[i].push(label);
                        }
                    }
                    continue;
                },
                Err(e) => error = Some(e),
            };
                 
            let data: Result<HashMap<String, Vec<f64>>, _> = py_data.extract::<HashMap<String, Vec<f64>>>(py);
            match data {
                Ok(dt) => {
                    for key in dt.keys() {
                        dbg!(&key);
                        headers.push(add_header_text(key.to_owned(), width));  
                    }

                    for values in dt.values() {
                        for (i, v) in values.iter().enumerate() {
                            let label = v.to_string();
                            if data_rows.len() <= i {
                                data_rows.push(vec![])
                            }
                            data_rows[i].push(label);
                        }
                    }
                    continue;
                },
                Err(e) => error = Some(e),
            };
             
            let data: Result<HashMap<String, Vec<String>>, _> = py_data.extract::<HashMap<String, Vec<String>>>(py);
                match data {
                    Ok(dt) => {
                        for key in dt.keys() {
                            headers.push(add_header_text(key.to_owned(), width));  
                        }

                        for values in dt.values() {
                            for (i, v) in values.iter().enumerate() {
                                let label = v.to_string();
                                if data_rows.len() <= i {
                                    data_rows.push(vec![])
                                }
                                data_rows[i].push(label);
                            }
                        }
                        continue; 
                    },
                    Err(e) => error = Some(e),
                };
        }
        
    });

    // construct the table elements and widgets.
    let mut body_column_vec: Vec<Element<Message>> = vec![];

    for (row_index, row) in data_rows.iter().enumerate() {

        let mut row_vec: Vec<Element<Message>> = vec![];

        for (col_index, label) in row.iter().enumerate() {
            let mut row_element: Element<Message> = Space::new(0.0, 0.0).into();
            let mut widget_found = false;

            let col_width = if col_index >= table.column_widths.len() {
                                        table.column_widths[0]
                                    } else {
                                        table.column_widths[col_index]
                                    };

            let index = check_for_widget(&table.button_ids, row_index, col_index);
            if index.is_some() {
                widget_found = true;
                let (wid_id, row, col, bl) = table.button_ids[index.unwrap()];
                row_element = add_widget(IpgTableWidget::Button,
                                    table.id, 
                                    label.clone(), 
                                    row, 
                                    col,
                                    col_width, 
                                    bl,
                                    table.button_fill_style_id.clone(),
                                    );
            }

            let index = check_for_widget(&table.checkbox_ids, row_index, col_index);
            if index.is_some() {
                widget_found = true;
                let (wid_id, row, col, bl) = table.checkbox_ids[index.unwrap()];
                row_element = add_widget(IpgTableWidget::Checkbox,
                                    table.id, 
                                    label.clone(), 
                                    row, 
                                    col,
                                    col_width, 
                                    bl,
                                    table.checkbox_fill_style_id.clone(),
                                    );
            }

            let index = check_for_widget(&table.toggler_ids, row_index, col_index);
            if index.is_some() {
                widget_found = true;
                let (wid_id, row, col, bl) = table.toggler_ids[index.unwrap()];
                row_element = add_widget(IpgTableWidget::Toggler,
                                    table.id, 
                                    label.clone(), 
                                    row, 
                                    col,
                                    col_width, 
                                    bl,
                                    table.toggler_fill_style_id.clone(),
                                    );
            }

            if !widget_found {
                row_element = add_text_widget(label.clone(), col_width);
            }
            
            let cnt = add_row_container(
                                                row_element, 
                                                row_index, 
                                                table.highlight_amount, 
                                                table.row_highlight);
            
            row_vec.push(cnt);
        }
        
        let row_widget: Element<Message> = Row::with_children(row_vec)
                                                .into();
        
        body_column_vec.push(row_widget.into());
    }
    
    let body_column: Element<Message> = Column::with_children(body_column_vec)
                                            .height(Length::Shrink)
                                            .padding([0, 5, 0, 5])
                                            .into();

    let title: Element<Message> = Text::new(table.title.clone()).into();

    let table_title: Element<Message> = Container::new(title)
                                            // .style()
                                            .width(Length::Fixed(table.width))
                                            .height(Length::Shrink)
                                            .align_x(Horizontal::Center)
                                            .into();

    let body: Element<Message> = Row::with_children(column_elements)
                                        .width(Length::Fill)
                                        .spacing(5.0)
                                        .into();
    let content: Element<Message> = Column::with_children(content).into();
    let mousearea: Element<Message> =  mouse_area(center(
                        opaque(content))
                        .style(|_theme| {
                        container::Style {
                            background: Some(
                                Color {
                                    a: 0.8,
                                    ..Color::BLACK
                                }
                                .into(),
                            ),
                            ..container::Style::default()
                        }
                    })).into();
                    // .on_press(on_blur)

    let table_header_row: Element<Message> = Row::with_children(headers)
                .width(Length::Fill)
                .padding(Padding::from([0, 5, 5, 2])) //bottom only
                .into();

        let scroller: Element<Message> = add_scroll(body_column, table.height, table.scroller_id);

    let final_column: Element<Message> = if table.modal_show {
        Column::with_children(vec![
            // set title
            table_title,
            table_header_row,
            // table body
            stack![
            scroller,
            mousearea,
        ]
        .into(),
        ])
            .width(Length::Fixed(table.width))
            .height(Length::Fixed(table.height))
            .padding([5.0, 10.0, 2.0, 5.0])
            .into()

    } else {
        Column::with_children(vec![
            table_title,
            table_header_row,
            scroller,
        ])
            .width(Length::Fixed(table.width))
            .height(Length::Fixed(table.height))
            .padding([5.0, 10.0, 2.0, 5.0])
            .into()
    };
    
    final_column
        
}


fn fill_column(col_values: Vec<Element<'static, Message>>) -> Element<'static, Message> {

    Column::with_children(col_values)
                                            .align_items(Alignment::Center)
                                            .width(Length::Fill)
                                            .into()
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
                                    .horizontal_alignment(Horizontal::Center)
                                    .into();
    txt
}

fn add_text_widget(label: String, width: f32) -> Element<'static, Message> {

    let txt: Element<Message> = Text::new(label)
        .width(Length::Fixed(width))
        .height(Length::Fixed(30.0))
        .horizontal_alignment(Horizontal::Center)
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
            .style(move|theme| table_row_theme(theme, row_index.clone(), 
                        highlight_amount.clone(),
                        row_highlight))
            .clip(true)
            .into()
}

fn add_widget(widget_type: IpgTableWidget, 
                table_id: usize, 
                label: String, 
                row_index: usize, 
                col_index: usize,
                column_width: f32,
                is_toggled: bool,
                style_id: Option<String>) 
                -> Element<'static, Message> {

    match widget_type {
        IpgTableWidget::Button => {
            let txt = 
                    Text::new(label)
                                .horizontal_alignment(Horizontal::Center)
                                .width(Length::Fixed(column_width));

            let btn_style: Option<IpgStyleStandard> = Some(IpgStyleStandard::Primary);

            let btn: Element<TableMessage> = 
                    Button::new(txt)
                                .padding(Padding::ZERO)
                                .width(Length::Shrink)
                                .on_press(TableMessage::TableButton((row_index, col_index))) 
                                .style(move|theme, status|
                                    ipg_button::get_standard_style(theme, status, btn_style.clone(), 
                                                                None, None))
                                .into(); 
            btn.map(move |message| app::Message::Table(table_id, message))
        },
        IpgTableWidget::Checkbox => {
            let chk: Element<TableMessage> = 
                    Checkbox::new(label, is_toggled)
                                .width(Length::Fixed(column_width))
                                .on_toggle(move|b| TableMessage::TableCheckbox(b, (row_index, col_index)))
                                .into();
            chk.map(move |message| app::Message::Table(table_id, message))
        },
        IpgTableWidget::Toggler => {
            let tog: Element<TableMessage> = Toggler::new(Some(label), is_toggled,
                                    move|b| TableMessage::TableToggler(b, (row_index, col_index)))
                                    .width(Length::Fixed(column_width))
                                    .into();


            tog.map(move |message| app::Message::Table(table_id, message))
        }
    }
}


fn get_checked(on_toggled: &Option<HashMap<usize, Vec<bool>>>, col_index: &usize, row_index: usize) -> bool {
    let toggled_hmap = on_toggled.as_ref().unwrap();
    let toggled = toggled_hmap.get(&col_index).unwrap();
    toggled[row_index]
}

pub fn table_callback(table_id: usize, message: TableMessage) {

    let mut wci = WidgetCallbackIn::default();
    wci.id = table_id;

    match message {
        TableMessage::TableButton((row_index, col_index)) => {
            let mut wco: WidgetCallbackOut = get_set_widget_callback_data(wci);
            wco.id = table_id; 
            wco.index_table = Some((row_index, col_index));
            wco.event_name = "on_button".to_string();
            process_callback(wco);
        },
        TableMessage::TableCheckbox(on_toggle, (row_index, col_index)) => {
            wci.value_str =  Some("checkbox".to_string());
            wci.on_toggle = Some(on_toggle);
            wci.index_table = Some((row_index, col_index));
            let mut wco: WidgetCallbackOut = get_set_widget_callback_data(wci);
            wco.id = table_id;
            wco.event_name = "on_checkbox".to_string();
            wco.on_toggle = Some(on_toggle);
            wco.index_table = Some((row_index, col_index));
            process_callback(wco);
        },
        TableMessage::TableToggler(on_toggle, (row_index, col_index)) => {
            wci.value_str = Some("toggler".to_string());
            wci.on_toggle = Some(on_toggle);
            wci.index_table = Some((row_index, col_index));
            let mut wco: WidgetCallbackOut = get_set_widget_callback_data(wci);
            wco.id = table_id;
            wco.event_name = "on_toggler".to_string();
            wco.on_toggle = Some(on_toggle);
            wco.index_table = Some((row_index, col_index));
            process_callback(wco);
        },
        TableMessage::TableScrolled(vp, scroller_id ) => {
            let mut state = access_state();
            let scroller_pos_opt = state.table_internal_ids.get_mut(&scroller_id);
            let mut scroller_pos = if scroller_pos_opt.is_some() {
                scroller_pos_opt.unwrap()
            } else {
                panic!("Table: Scroller id '{}' not found", scroller_id)
            };
            
            scroller_pos.relative_offset_x = vp.relative_offset().x;
            scroller_pos.relative_offset_y = vp.relative_offset().y;
            scroller_pos.absolute_offset_x = vp.absolute_offset().x;
            scroller_pos.absolute_offset_y = vp.absolute_offset().y;
            scroller_pos.content_bound_height = vp.content_bounds().height;
            scroller_pos.bounds_height = vp.bounds().height;
            drop(state);
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
                                            wco.id.clone(),
                                            table_index, 
                                            user_data
                                            ))
                                }
                                else {
                                    callback.call1(py, (
                                        wco.id.clone(),
                                        table_index,
                                        wco.on_toggle,  
                                        user_data
                                        ))
                                };
            match res {
                Ok(_) => (),
                Err(er) => panic!("Table: 4 parameters (id, widget_index, on_toggle, user_data) are required or a python error in this function. {er}"),
            }
        } else {
            
            let res = 
                                if wco.event_name == "on_button" {
                                    callback.call1(py, (
                                                wco.id.clone(),
                                                table_index, 
                                                ))
                                    }
                                    else {
                                        callback.call1(py, (
                                            wco.id.clone(),
                                            table_index,
                                            wco.on_toggle,  
                                            ))
                                    };
            match res {
                Ok(_) => (),
                Err(er) => panic!("Table: 3 parameter (id, widget_index, on_toggle) are required or possibly a python error in this function. {er}"),
            }
        }
    });
 
    drop(app_cbs);
         
}

// widgets = (id, row idx, col idx, bool)
fn check_for_widget(widgets: &Vec<(usize, usize, usize, bool)>, row_index: usize, col_index: usize) -> Option<usize> {
    // if empty return
    if widgets.len() == 0 {return None}
    // if not column return
    if widgets[0].2 != col_index {return None}

    // Becuase of possible mixed columns of widgets, need the index
    for (index, widget) in widgets.iter().enumerate() {
        if row_index == widget.1 {
            return Some(index)
        } 
    }
    return None
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
    Data,
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
        IpgTableParam::Data => {
            table.data = value;
        },
        IpgTableParam::Width => {
            table.width = try_extract_f64(value) as f32;
        },
        IpgTableParam::Height => {
            table.height = try_extract_f64(value) as f32;
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
