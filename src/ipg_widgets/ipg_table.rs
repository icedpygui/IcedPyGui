#![allow(unused)]
use std::collections::HashMap;
use std::result::Result;

use crate::app::{self, Message};
use crate::{access_callbacks, access_state, add_callback_to_mutex, find_parent_uid};
use crate::style::styling::{get_theme_color, IpgStyleStandard};
use super::callbacks::{get_set_widget_callback_data, WidgetCallbackIn, WidgetCallbackOut};
use super::ipg_theme_colors::{get_alt_color, IpgColorAction};
use super::ipg_button;
use crate::style::styling::{lighten, darken};

use iced::{Border, Point, Shadow};
use iced::mouse::Interaction;
use iced::widget::text::{LineHeight, Style};
use iced::{alignment, theme, Background, Element, Length, Padding, Renderer, Theme};
use iced::alignment::Alignment;
use iced::widget::{Button, container, Checkbox, Column, Container, Image, 
    MouseArea, Row, Scrollable, Space, Text, text, Toggler};
use iced::alignment::Horizontal;
use iced::widget::svg;
use iced::advanced::image;


use pyo3::{PyObject, Python, pyclass};


#[derive(Debug, Clone)]
pub enum TableData {
    Row,
    Column,
}


#[derive(Debug, Clone)]
pub struct IpgTable {
        pub id: usize,
        pub title: String,
        pub data: Vec<PyObject>,
        pub width: f32,
        pub height: f32,
        pub row_highlight: Option<TableRowHighLight>,
        pub highlight_amount: f32,
        pub column_widths: Vec<f32>,
        pub table_length: u32,
        pub button_style: Option<HashMap<usize, IpgStyleStandard>>,
        pub button_ids: Vec<(usize, usize, usize, bool)>,
        pub check_ids: Vec<(usize, usize, usize, bool)>,
        pub toggler_ids: Vec<(usize, usize, usize, bool)>,
        pub show: bool,
        pub user_data: Option<PyObject>,
        pub container_id: usize,
        pub window_id: usize,
}

impl IpgTable {
    pub fn new( 
        id: usize,
        title: String,
        data: Vec<PyObject>,
        width: f32,
        height: f32,
        row_highlight: Option<TableRowHighLight>,
        highlight_amount: f32,
        column_widths: Vec<f32>,
        table_length: u32,
        button_style:  Option<HashMap<usize, IpgStyleStandard>>,
        button_ids: Vec<(usize, usize, usize, bool)>,
        check_ids: Vec<(usize, usize, usize, bool)>,
        toggler_ids: Vec<(usize, usize, usize, bool)>,
        show: bool,
        user_data: Option<PyObject>,
        container_id: usize,
        window_id: usize,
        ) -> Self {
        Self {
            id,
            title,
            data,
            width,
            height,
            row_highlight,
            highlight_amount,
            column_widths,
            table_length,
            button_style,
            button_ids,
            check_ids,
            toggler_ids,
            show,
            user_data,
            container_id,
            window_id,
        }
    }
}

#[derive(Debug, Clone)]
pub enum TableMessage {
    TableButton((usize, usize)),
    TableCheckbox(bool, (usize, usize)),
    TableToggler(bool, (usize, usize)),
}

#[derive(Debug, Clone, Copy)]
enum DataTypes {
    Bool,
    F64,
    String,
}

#[derive(Debug, Clone, Copy)]
#[pyclass]
pub enum TableRowHighLight {
    Darker,
    Lighter,
}

#[derive(Debug, Clone, Copy)]
#[pyclass]
pub enum TableWidget {
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


pub fn contruct_table(table: IpgTable) -> Element<'static, Message> {

    let mut headers: Vec<Element<Message>>= vec![];

    let mut column_elements: Vec<Element<Message>> = vec![];

    let mut data_rows: Vec<Vec<String>> = vec![];
    let mut table_length: usize = 0;

    Python::with_gil(|py| {

        // Gets the entire column at each iteration
        for (col_index, py_data) in table.data.iter().enumerate() {

            let data: Result<HashMap<String, Vec<bool>>, _> = py_data.extract::<HashMap<String, Vec<bool>>>(py);
            if !data.is_err() { 
                match data {
                    Ok(dt) => {
                        for key in dt.keys() {
                            headers.push(add_header_text(key.to_owned()));                                          
                        }

                        // dt.values are the columns in the table
                        for value in dt.values() {
                            if table_length == 0 {
                                table_length = value.len();
                                for _ in 0..table_length {
                                    data_rows.push(vec![])
                                }
                            }
                            for (i, v) in value.iter().enumerate() {

                                let mut label = "False".to_string();
                                    if *v {label = "True".to_string()}

                                data_rows[i].push(label);
                            }
                        }
                    },
                    Err(_) => (),
                };
                continue; 
            }

            let data: Result<HashMap<String, Vec<i64>>, _> = py_data.extract::<HashMap<String, Vec<i64>>>(py);
            if !data.is_err() { 
                match data {
                    Ok(dt) => {
                        for key in dt.keys() {
                            headers.push(add_header_text(key.to_owned()));  
                        }

                        for value in dt.values() {
                            if table_length == 0 {
                                table_length = value.len();
                                for _ in 0..table_length {
                                    data_rows.push(vec![])
                                }
                            }
                            for (i, v) in value.iter().enumerate() {
                                let label = v.to_string();
                                data_rows[i].push(label);
                            }
                        }
                    },
                    Err(_) => (),
                };
                continue; 
            }

            let data: Result<HashMap<String, Vec<f64>>, _> = py_data.extract::<HashMap<String, Vec<f64>>>(py);
            if !data.is_err() {
                match data {
                    Ok(dt) => {
                        for key in dt.keys() {
                            headers.push(add_header_text(key.to_owned()));  
                        }

                        for value in dt.values() {
                            if table_length == 0 {
                                table_length = value.len();
                                for _ in 0..table_length {
                                    data_rows.push(vec![])
                                }
                            }
                            for (i, v) in value.iter().enumerate() {
                                let label = v.to_string();

                                data_rows[i].push(label);
                            }
                        }
                    },
                    Err(_) => (),
                };
                continue; 
            }

            let data: Result<HashMap<String, Vec<String>>, _> = py_data.extract::<HashMap<String, Vec<String>>>(py);
            if !data.is_err() { 
                match data {
                    Ok(dt) => {
                        for key in dt.keys() {
                            headers.push(add_header_text(key.to_owned()));  
                        }

                        for value in dt.values() {
                            if table_length == 0 {
                                table_length = value.len();
                                for _ in 0..table_length {
                                    data_rows.push(vec![])
                                }
                            }
                            for (i, v) in value.iter().enumerate() {
                                let label = v.to_string();

                                data_rows[i].push(label);

                            }
                        }
                    },
                    Err(_) => (),
                };
                continue; 
            }
        }
        
    });

    // construct the table elements and widgets.
    let mut body_column_vec: Vec<Element<Message>> = vec![];

    for (row_index, row) in data_rows.iter().enumerate() {

        let mut row_vec: Vec<Element<Message>> = vec![];

        for (col_index, label) in row.iter().enumerate() {
            let mut row_element: Element<Message> = Space::new(0.0, 0.0).into();
            let mut widget_found = false;

            let index = check_for_widget(&table.button_ids, row_index, col_index);
            if index.is_some() {
                widget_found = true;
                let (wid_id, row, col, bl) = table.button_ids[index.unwrap()];
                row_element = add_widget(TableWidget::Button,
                                    table.id, 
                                    label.clone(), 
                                    row, 
                                    col, 
                                    bl,
                                    table.button_style.clone());
            }

            let index = check_for_widget(&table.check_ids, row_index, col_index);
            if index.is_some() {
                widget_found = true;
                let (wid_id, row, col, bl) = table.check_ids[index.unwrap()];
                row_element = add_widget(TableWidget::Checkbox,
                                    table.id, 
                                    label.clone(), 
                                    row, 
                                    col, 
                                    bl,
                                    None, 
                                    );
            }

            let index = check_for_widget(&table.toggler_ids, row_index, col_index);
            if index.is_some() {
                widget_found = true;
                let (wid_id, row, col, bl) = table.toggler_ids[index.unwrap()];
                row_element = add_widget(TableWidget::Toggler,
                                    table.id, 
                                    label.clone(), 
                                    row, 
                                    col, 
                                    bl,
                                    None, 
                                    );
            }

            if !widget_found {
                row_element = add_text_widget(label.clone());
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

    let body_column: Element<Message> = Column::with_children(body_column_vec).into();

    let title: Element<Message> = Text::new(table.title.clone()).into();

    let table_title: Element<Message> = Container::new(title)
                                            // .style(theme::Container::Box)
                                            .width(Length::Fixed(table.width))
                                            .height(Length::Shrink)
                                            .align_x(Horizontal::Center)
                                            .into();

    let body: Element<Message> = Row::with_children(column_elements)
                                        .width(Length::Fill)
                                        .spacing(5.0)
                                        .into();

    let final_col: Element<'_, Message> = Column::with_children(vec![
        // set title
        table_title,
        // table header row
        Row::with_children(headers)
                .width(Length::Fill)
                .padding(Padding::from([0, 0, 5, 0])) //bottom only
                .into(),
        // table body
        add_scroll(body_column, table.height),
    ])
        .width(Length::Fixed(table.width))
        .height(Length::Fixed(table.height))
        .into();
    
    final_col
        
}

fn fill_column(col_values: Vec<Element<'static, Message>>) -> Element<'static, Message> {

    Column::with_children(col_values)
                                            .align_items(Alignment::Center)
                                            .width(Length::Fill)
                                            .into()
}

fn add_scroll(body: Element<'static, Message>, height: f32) -> Element<'static, Message>{
    
    Scrollable::new(body)
                    .height(Length::Fixed(height))
                    .into()
    
}

fn add_header_text (header: String) -> Element<'static, Message> {
    let txt : Element<Message> = text(header)
                                    .width(Length::Fill)
                                    .horizontal_alignment(Horizontal::Center)
                                    .into();
    txt
}

fn add_text_widget(label: String) -> Element<'static, Message> {
    let txt: Element<Message> = text(label)
        .width(Length::Fill)
        .horizontal_alignment(Horizontal::Center
        ).into();

    txt
}

fn add_row_container(content: Element<Message>, row_index: usize,
                    highlight_amount: f32, row_highlight: Option<TableRowHighLight>) 
                    -> Element<Message> {
    // Using container because text has no background 
    Container::new(content)
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center)
            .style(move|theme| table_row_theme(theme, row_index.clone(), 
                        highlight_amount.clone(),
                        row_highlight))
            .clip(true)
            .into()
}

fn add_widget(widget_type: TableWidget, 
                table_id: usize, 
                label: String, 
                row_index: usize, 
                col_index: usize, 
                is_toggled: bool,
                button_style:  Option<HashMap<usize, IpgStyleStandard>>,) 
                -> Element<'static, Message> {

    match widget_type {
        TableWidget::Button => {
            let txt = 
                    Text::new(label)
                                .horizontal_alignment(Horizontal::Center)
                                .width(Length::Fill);

            let btn_style: Option<IpgStyleStandard> = if button_style.is_some() {
                let style = button_style.unwrap();
                match style.get(&col_index) {
                    Some(st) => Some(st.clone()),
                    None => None,
                }
            } else {
                Some(IpgStyleStandard::Primary)
            };

            let btn: Element<TableMessage> = 
                    Button::new(txt)
                                .padding(Padding::ZERO)
                                .width(Length::Fill)
                                .on_press(TableMessage::TableButton((row_index, col_index))) 
                                .style(move|theme, status|
                                    ipg_button::get_standard_style(theme, status, btn_style.clone(), 
                                                                None, None))
                                .into(); 
            btn.map(move |message| app::Message::Table(table_id, message))
        },
        TableWidget::Checkbox => {
            let chk: Element<TableMessage> = 
                    Checkbox::new(label, is_toggled)
                                .width(Length::Shrink)
                                .on_toggle(move|b| TableMessage::TableCheckbox(b, (row_index, col_index)))
                                .into();
            chk.map(move |message| app::Message::Table(table_id, message))
        },
        
        TableWidget::Toggler => {
            let tog: Element<TableMessage> = Toggler::new(Some(label), is_toggled,
                                    move|b| TableMessage::TableToggler(b, (row_index, col_index)))
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


fn table_item_update( 
                    id: Option<usize>,
                    title: Option<String>,
                    headers: Option<Vec<String>>,
                    data: Option<PyObject>,
                    user_id: Option<String>,
                    on_update: Option<PyObject>,
                    ) 
{
    
    let id: usize = match id {
        Some(id) => id,
        None => 0
    };

    let user_id = match user_id {
        Some(id) => id,
        None => "".to_string()
    };
    
    if &id == &0 && &user_id == &"".to_string() {
        panic!("You must supply either an id or user_id to update the table.")
    }

    let _title = match title {
        Some(title) => title,
        None => "".to_string(),
    };

    let _headers = match headers {
        Some(hd) => hd,
        None => vec![],
    };
    
    // let _data = py_extract_list(data);
    
    if on_update.is_some() {
        add_callback_to_mutex(id, "on_update".to_string(), on_update);
    }
    
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
                        highlighter: Option<TableRowHighLight>) -> container::Style {

    let mut background = get_theme_color(theme);

    if idx % 2 == 0 {
        background = match highlighter {
                Some(hl) => 
                    match hl {
                        TableRowHighLight::Darker => darken(background, amount),
                        TableRowHighLight::Lighter => lighten(background, amount),
                        },
                None => background,
            }
    }; 
    
    container::Style {
        background: Some(Background::Color(background)),
        ..Default::default()
    }
}
