#![allow(unused)]
use std::collections::HashMap;
use std::result::Result;

use crate::app::{self, Message};
use crate::{access_callbacks, access_state, add_callback_to_mutex, find_parent_uid};
use crate::ipg_widgets::ipg_container::{IpgContainerTheme, table_row_theme};
use super::callbacks::{get_set_widget_callback_data, WidgetCallbackIn, WidgetCallbackOut};
use super::ipg_theme_colors::{get_alt_color, IpgColorAction};

use iced::widget::text::Style;
use iced::{alignment, theme, Background, Element, Length, Padding, Renderer, Theme};
use iced::alignment::Alignment;
use iced::widget::{Container, container, text, Column, Row, Scrollable, Text};
use iced::alignment::Horizontal;

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
        pub widget_header: Option<String>,
        pub widget: Option<TableWidget>,
        pub widget_column: usize,
        pub widget_label: Option<String>,
        pub widget_ids: Vec<usize>,
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
        widget_header: Option<String>,
        widget: Option<TableWidget>,
        widget_column: usize,
        widget_label: Option<String>,
        widget_ids: Vec<usize>,
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
            widget_header,
            widget,
            widget_column,
            widget_label,
            widget_ids,
            show,
            user_data,
            container_id,
            window_id,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TableMessage {
    TableButton(usize),
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
    Image,
    Svg,
}

struct Data {
    index: usize,
    d_type: DataTypes,
    data: Vec<DataTypes>
}


pub fn contruct_table(table: IpgTable) -> Element<'static, Message> {

    let mut headers: Vec<Element<Message>>= vec![];

    let mut column_elements: Vec<Element<Message>> = vec![];
    
    Python::with_gil(|py| {

        let mut column_index = 0;

        // Gets the entire column at each iteration
        for py_data in table.data {

            if table.widget.is_some() && column_index == table.widget_column {
                
                column_elements.push(add_widget_column(table.widget, table.id, 
                                                table.widget_ids.len(), table.widget_label.clone()));
                
                let wh = match table.widget_header.clone() {
                    Some(header) => header,
                    None => " ".to_string(),
                };

                headers.push(text(wh)
                                        .width(Length::Fill)
                                        .horizontal_alignment(Horizontal::Center)
                                        .into())
            }
            column_index += 1;

            let data: Result<HashMap<String, Vec<bool>>, _> = py_data.extract::<HashMap<String, Vec<bool>>>(py);
            if !data.is_err() { 
                match data {
                    Ok(dt) => {
                        for key in dt.keys() {
                            headers.push(text(key.to_owned())
                                            .width(Length::Fill)
                                            .horizontal_alignment(Horizontal::Center)
                                            .into());                                          
                        }
                        
                        //Column values are put into individual columns then columns into  a row.
                        let mut col_values: Vec<Element<Message>> = vec![];

                        for value in dt.values() {
                            for (i, v) in value.iter().enumerate() {

                                let txt: Element<Message> = 
                                    if *v {
                                        Container::new(text("True")
                                                                .width(Length::Fill)
                                                                .horizontal_alignment(Horizontal::Center)
                                                        )
                                                        .width(Length::Fill)
                                                        .style(move|theme| table_row_theme(theme, i.clone(), 
                                                                        table.highlight_amount.clone(),
                                                                        table.row_highlight))
                                                        .into()
                                    } else {
                                        Container::new(text("False")
                                                                .width(Length::Fill)
                                                                .horizontal_alignment(Horizontal::Center)
                                                        )
                                                        .width(Length::Fill)
                                                        .style(move|theme| table_row_theme(theme, i.clone(), 
                                                                    table.highlight_amount.clone(),
                                                                    table.row_highlight))
                                                        .into()
                                    };

                                col_values.push(txt);
                            }
                        }
                        column_elements.push(fill_column(col_values));
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
                            headers.push(text(key.to_owned())
                                            .width(Length::Fill)
                                            .horizontal_alignment(Horizontal::Center)
                                            .into());  
                        }

                        let mut col_values: Vec<Element<Message>> = vec![];

                        for value in dt.values() {
                            for (i, v) in value.iter().enumerate() {
                                let txt: Element<Message> = Container::new(text(v.to_string())
                                                                        .width(Length::Fill)
                                                                        .horizontal_alignment(Horizontal::Center)
                                                                    )
                                                                    .width(Length::Fill)
                                                                    .style(move|theme| table_row_theme(theme, i.clone(), 
                                                                                table.highlight_amount.clone(),
                                                                                table.row_highlight))
                                                                    .into();

                                col_values.push(txt);
                            }
                        }
                        column_elements.push(fill_column(col_values));
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
                            headers.push(text(key.to_owned())
                                            .width(Length::Fill)
                                            .horizontal_alignment(Horizontal::Center)
                                            .into());  
                        }

                        let mut col_values: Vec<Element<Message>> = vec![];

                        for value in dt.values() {
                            for (i, v) in value.iter().enumerate() {
                                let txt: Element<Message> = Container::new(text(v)
                                                                        .width(Length::Fill)
                                                                        .horizontal_alignment(Horizontal::Center)
                                                                    )
                                                                    .width(Length::Fill)
                                                                    .style(move|theme| table_row_theme(theme, i.clone(), 
                                                                                table.highlight_amount.clone(),
                                                                                table.row_highlight))
                                                                    .into();


                                col_values.push(txt);
                            }
                        }
                        column_elements.push(fill_column(col_values));
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
                            headers.push(text(key.to_owned())
                                            .width(Length::Fill)
                                            .horizontal_alignment(Horizontal::Center)
                                            .into());  
                        }

                        let mut col_values: Vec<Element<Message>> = vec![];

                        for value in dt.values() {
                            for (i, v) in value.iter().enumerate() {
                                let txt: Element<Message> = Container::new(text(v.clone())
                                                                        .width(Length::Fill)
                                                                        .horizontal_alignment(Horizontal::Center)
                                                                    )
                                                                    .width(Length::Fill)
                                                                    .style(move|theme| table_row_theme(theme, i.clone(), 
                                                                                table.highlight_amount.clone(),
                                                                                table.row_highlight))
                                                                    .into();


                                col_values.push(txt)
                            }
                        }
                        column_elements.push(fill_column(col_values));
                    },
                    Err(_) => (),
                };
                continue; 
            }
        }
        
    });

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
        add_scroll(body, table.height),
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

use iced::widget::Button;
fn add_widget_column(widget_opt: Option<TableWidget>, table_id: usize, 
                        col_len: usize, label: Option<String>) 
                        -> Element<'static, Message, Theme, Renderer> {

    let widget = match widget_opt {
        Some(wid) => wid,
        None => panic!("TableWidget could not be found")
    };

    match widget {
        TableWidget::Button => {
            let mut buttons: Vec<Element<'static, TableMessage, Theme, Renderer>> = vec![];
            for i in 0..col_len {
                let lbl = match label.clone() {
                    Some(lbl) => lbl,
                    None => " ".to_string(),
                };

                let txt_label = text(lbl);
                let button: Element<'static, TableMessage, Theme, Renderer> = Button::new(txt_label)
                                                            // .height(Length::Fixed(25.0))
                                                            .on_press(TableMessage::TableButton(i)) 
                                                            .into(); 
                buttons.push(button);
            }
            
            let col: Element<'static, TableMessage, Theme, Renderer> = Column::with_children(buttons).into();
            col.map(move |message| app::Message::Table(table_id, message))
        },
        TableWidget::Checkbox => todo!(),
        TableWidget::Image => todo!(),
        TableWidget::Svg => todo!(),
    }

}


pub fn table_callback(id: usize, message: TableMessage) {

    let mut wci = WidgetCallbackIn::default();
    wci.id = id;

    match message {
        TableMessage::TableButton(index) => {
            wci.index = Some(index);
            let mut wco: WidgetCallbackOut = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = "table".to_string();
            process_callback(wco);
        }
    }
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

    let mut widget_index = 0;
    if wco.index.is_some() {
        widget_index = match wco.index {
                Some(idx) => idx,
                None => panic!("table: widget_index not found"),
            };
    }
    

    Python::with_gil(|py| {
            if wco.user_data.is_some() {
                let user_data = match wco.user_data {
                    Some(ud) => ud,
                    None => panic!("User Data could not be found in Table callback"),
                };
                if wco.index.is_some() {
                    let res = callback.call1(py, (
                                                                        wco.id.clone(),
                                                                        widget_index,  
                                                                        user_data
                                                                        ));
                    match res {
                        Ok(_) => (),
                        Err(er) => panic!("Table: 3 parameters (id, widget_index, user_data) are required or a python error in this function. {er}"),
                    }
                } else {
                    let res = callback.call1(py, (
                                                                        wco.id.clone(),  
                                                                        user_data
                                                                        ));
                    match res {
                        Ok(_) => (),
                        Err(er) => panic!("Table: 2 parameters (id, user_data) are required or a python error in this function. {er}"),
                        }
                    }
                
            } else {
                if wco.index.is_some() {
                    let res = callback.call1(py, (
                                                                        wco.id.clone(),
                                                                        widget_index,  
                                                                        ));
                    match res {
                        Ok(_) => (),
                        Err(er) => panic!("Table: 2 parameter (id, widget_index) are required or possibly a python error in this function. {er}"),
                    }
                } else {
                    let res = callback.call1(py, (
                                                                        wco.id.clone(),  
                                                                        ));
                    match res {
                        Ok(_) => (),
                        Err(er) => panic!("Table: 1 parameter (id) is required or possibly a python error in this function. {er}"),
                        }
                }
            } 
    });
    
    drop(app_cbs);
         
}
