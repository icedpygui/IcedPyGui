#![allow(unused)]
use std::collections::HashMap;
use std::result::Result;

use crate::app::Message;

// use crate::{access_state, access_callbacks};

use iced::{theme, Element, Length, Padding};
use iced::alignment::Alignment;
use iced::widget::{Container, container, text, Column, Row, Scrollable, Text};

use pyo3::{PyObject, Python};


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
        pub column_widths: Vec<f32>,
        pub show: bool,
        pub user_data: Option<PyObject>,
        pub cb_name:  Option<String>,
}

impl IpgTable {
    pub fn new( 
        id: usize,
        title: String,
        data: Vec<PyObject>,
        width: f32,
        height: f32,
        column_widths: Vec<f32>,
        show: bool,
        user_data: Option<PyObject>,
        cb_name: Option<String>,
        ) -> Self {
        Self {
            id,
            title,
            data,
            width,
            height,
            column_widths,
            show,
            user_data,
            cb_name,
        }
    }
}

enum DataTypes {
    Bool,
    F64,
    String,
}

struct Data {
    index: usize,
    d_type: DataTypes,
    data: Vec<DataTypes>
}

pub fn contruct_table(table: IpgTable) -> Element<'static, Message> {

    let mut headers: Vec<Element<Message>>= vec![];

    let mut row_elements: Vec<Element<Message>> = vec![];
    
    Python::with_gil(|py| {

        for py_data in table.data {

            let data: Result<HashMap<String, Vec<bool>>, _> = py_data.extract::<HashMap<String, Vec<bool>>>(py);
            if !data.is_err() { 
                match data {
                    Ok(dt) => {

                        for key in dt.keys() {
                            headers.push(fill_header(key.to_owned()));                                          
                        }
                        
                        let mut col_values: Vec<Element<Message>> = vec![];

                        for value in dt.values() {
                            for (i, v) in value.iter().enumerate() {

                                let txt: Element<Message> = 
                                    if *v {
                                        text("True").into()
                                    } else {
                                        text("False").into()
                                    };

                                let cell: Element<Message> = set_row_theme(i, txt);

                                col_values.push(cell);
                            }
                        }
                        row_elements.push(fill_column(col_values));
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
                            headers.push(fill_header(key.to_owned())); 
                        }

                        let mut col_values: Vec<Element<Message>> = vec![];

                        for value in dt.values() {
                            for (i, v) in value.iter().enumerate() {
                                let txt = text(v.to_string()).into();
                                let cell: Element<Message> = set_row_theme(i, txt);
                                col_values.push(cell);
                            }
                        }
                        row_elements.push(fill_column(col_values));
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
                            headers.push(fill_header(key.to_owned())); 
                        }

                        let mut col_values: Vec<Element<Message>> = vec![];

                        for value in dt.values() {
                            for (i, v) in value.iter().enumerate() {
                                let txt = text(format!("{:.1}", v)).into();
                                let cell: Element<Message> = set_row_theme(i, txt);
                                col_values.push(cell);
                            }
                        }
                        row_elements.push(fill_column(col_values));
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
                            headers.push(fill_header(key.to_owned())); 
                        }

                        let mut col_values: Vec<Element<Message>> = vec![];

                        for value in dt.values() {
                            for (i, v) in value.iter().enumerate() {
                                let txt = text(v).into();
                                let cell: Element<Message> = set_row_theme(i, txt);
                                col_values.push(cell)
                            }
                        }
                        row_elements.push(fill_column(col_values));
                    },
                    Err(_) => (),
                };
                continue; 
            }
        }
    });

    let title: Element<Message> = Text::new(table.title.clone()).into();

    let table_title: Element<Message> = Container::new(title)
                                            .style(theme::Container::Box)
                                            .width(Length::Fill)
                                            .center_x()
                                            .center_y()
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
        fill_table_body(row_elements),
    ])
        .width(Length::Fixed(table.width))
        .height(Length::Fixed(table.height))
        .into();
    
    final_col
        
}


fn fill_header(key: String) -> Element<'static, Message>{

    let txt: Element<Message> = Text::new(key.to_owned()).into();

    Column::new()
            .push(txt)
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .into() 
} 

fn fill_column(col_values: Vec<Element<'static, Message>>) -> Element<'static, Message> {

    let col: Element<'static, Message> = Column::with_children(col_values)
                                            .align_items(Alignment::Center)
                                            .into();
    Row::with_children(vec![col])
                                .width(Length::Fill)
                                .into()
}

fn fill_table_body(rows: Vec<Element<'static, Message>>) -> Element<'static, Message>{
    
    let col: Element<'static, Message> = 
                Column::with_children(vec![
                        Row::with_children(rows)
                                .width(Length::Fill)
                                .into()
                        ])
                        .into();

    Scrollable::new(col)
                    .height(Length::Fill)
                    .into()
}

fn set_row_theme(index: usize, widget: Element<'static, Message>) -> Element<'static, Message> {

    let row_theme =                       
        if index % 2 != 0 {
            theme::Container::Transparent
        } else {
            theme::Container::Box
        };

    container(widget)
            .style(row_theme)
            .width(Length::Fill)
            .center_x()
            .center_y()
            .into()
}

