#![allow(unused)]
use std::collections::HashMap;
use std::result::Result;

use crate::app::{self, Message};
use crate::{access_callbacks, access_state, add_callback_to_mutex, find_parent_uid};
use crate::ipg_widgets::ipg_container::{IpgContainerTheme, table_row_theme};
use super::callbacks::{get_set_container_callback_data, get_set_widget_callback_data, WidgetCallbackIn, WidgetCallbackOut};
use super::ipg_theme_colors::{get_alt_color, IpgColorAction};
use crate::iced_widgets::checkbox::Checkbox;
use crate::iced_widgets::mousearea_table::{MouseArea, PointIdRC};

use iced::mouse::Interaction;
use iced::widget::text::Style;
use iced::{alignment, theme, Background, Element, Length, Padding, Renderer, Theme};
use iced::alignment::Alignment;
use iced::widget::{container, text, Column, Container, Image, Row, Scrollable, Text};
use iced::alignment::Horizontal;
use iced::advanced::image::{self, Handle};

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
        pub widgets_using_columns: Option<HashMap<usize, Vec<TableWidget>>>, // column#, widget type
        pub widget_ids: Option<HashMap<usize, Vec<usize>>>, // column, ids
        pub on_toggled: Option<HashMap<usize, Vec<bool>>>,
        pub image_path: Option<String>,
        pub image_root_name: Option<String>,
        pub image_root_pattern: Option<String>,
        pub image_list_names: Option<Vec<String>>,
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
        widgets_using_columns: Option<HashMap<usize, Vec<TableWidget>>>,
        widget_ids: Option<HashMap<usize, Vec<usize>>>,
        on_toggled: Option<HashMap<usize, Vec<bool>>>,
        image_path: Option<String>,
        image_root_name: Option<String>,
        image_root_pattern: Option<String>,
        image_list_names: Option<Vec<String>>,
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
            widgets_using_columns,
            widget_ids,
            on_toggled,
            image_path,
            image_root_name,
            image_root_pattern,
            image_list_names,
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
    TableCheckbox((usize, usize), bool),

    MouseAreaOnPress((usize, usize)),
    MouseAreaOnRelease((usize, usize)),
    MouseAreaOnRightPress((usize, usize)),
    MouseAreaOnRightRelease((usize, usize)),
    MouseAreaOnMiddlePress((usize, usize)),
    MouseAreaOnMiddleRelease((usize, usize)),
    MouseAreaOnEnter((usize, usize)),
    MouseAreaOnMove(PointIdRC),
    MouseAreaOnExit((usize, usize)),
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

    let mut widgets_construct = false;

    let table_widgets: HashMap<usize, Vec<TableWidget>> = if table.widgets_using_columns.is_some() {
        widgets_construct = true;
        table.widgets_using_columns.unwrap()
    } else {
        HashMap::new()
    };

     let table_widgets_ids = match table.widget_ids {
            Some(ids) => ids.clone(),
            None => HashMap::new()
        };
    
    Python::with_gil(|py| {

        let mut widget_column_pos: Vec<usize> = vec![];
        if widgets_construct {
            widget_column_pos = table_widgets.keys().into_iter().map(|key| *key).collect();
            
        }

        // Gets the entire column at each iteration
        for (col_index, py_data) in table.data.iter().enumerate() {

            let mut widgets = vec![];

            if widgets_construct && widget_column_pos.contains(&col_index){
                
                let wid_opt = table_widgets.get(&col_index);
                widgets = match wid_opt {
                    Some(wid) => wid.clone(),
                    None => vec![],
                }
            }
            
            let data: Result<HashMap<String, Vec<bool>>, _> = py_data.extract::<HashMap<String, Vec<bool>>>(py);
            if !data.is_err() { 
                match data {
                    Ok(dt) => {
                        for key in dt.keys() {
                            headers.push(add_header_text(key.to_owned()));                                          
                        }
                        
                        //Column values are put into individual columns then columns into  a row.
                        let mut col_values: Vec<Element<Message>> = vec![];

                        // dt.values are the columns in the table
                        for value in dt.values() {
                            for (i, v) in value.iter().enumerate() {

                                let mut label = "False".to_string();
                                    if *v {label = "True".to_string();}

                                let col_element: Element<Message> = 
                                if widget_column_pos.contains(&col_index) {
                                    let is_checked = get_checked(&table.on_toggled, 
                                                                        &col_index, 
                                                                        i);
                                    add_widget(widgets[i], 
                                                table.id, 
                                                label, i, 
                                                &col_index, is_checked,
                                                table.image_path.clone())
                                }else {
                                    add_text_widget(label)
                                };        
                                let cnt: Element<Message> = add_row_container(
                                                                col_element, 
                                                                i, 
                                                                table.highlight_amount, 
                                                                table.row_highlight);
                                    

                                col_values.push(cnt);
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
                            headers.push(add_header_text(key.to_owned()));  
                        }

                        let mut col_values: Vec<Element<Message>> = vec![];

                        for value in dt.values() {
                            for (i, v) in value.iter().enumerate() {
                                let label = v.to_string();

                                let col_element: Element<Message> = 
                                if widget_column_pos.contains(&col_index) {
                                    let is_checked = get_checked(&table.on_toggled, 
                                                                        &col_index, 
                                                                        i);
                                    add_widget(widgets[i], 
                                                table.id, 
                                                label, i, 
                                                &col_index, is_checked,
                                                table.image_path.clone())
                                }else {
                                    add_text_widget(label)
                                };

                                let cnt: Element<Message> = add_row_container(
                                                                col_element, 
                                                                i, 
                                                                table.highlight_amount, 
                                                                table.row_highlight);

                                col_values.push(cnt);
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
                            headers.push(add_header_text(key.to_owned()));  
                        }

                        let mut col_values: Vec<Element<Message>> = vec![];

                        for value in dt.values() {
                            for (i, v) in value.iter().enumerate() {
                                let label = v.to_string();

                                let col_element: Element<Message> = 
                                if widget_column_pos.contains(&col_index) {
                                    let is_checked = get_checked(&table.on_toggled, 
                                                                        &col_index, 
                                                                        i);
                                    add_widget(widgets[i], 
                                                table.id, 
                                                label, i, 
                                                &col_index, is_checked,
                                                table.image_path.clone())
                                }else { 
                                    add_text_widget(label)
                                };

                                let cnt: Element<Message> = add_row_container(
                                                        col_element, 
                                                        i, 
                                                        table.highlight_amount, 
                                                        table.row_highlight);

                                col_values.push(cnt);
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
                            headers.push(add_header_text(key.to_owned()));  
                        }

                        let mut col_values: Vec<Element<Message>> = vec![];

                        for value in dt.values() {
                            for (i, v) in value.iter().enumerate() {
                                let label = v.to_string();
                                  
                                let col_element: Element<Message> = 
                                if widget_column_pos.contains(&col_index) {
                                    let is_checked = get_checked(&table.on_toggled, 
                                                                        &col_index, 
                                                                        i);
                                    add_widget(widgets[i],
                                                table.id, 
                                                label, i, 
                                                &col_index, is_checked,
                                                table.image_path.clone())
                                }else { 
                                    add_text_widget(label)
                                };

                                let cnt: Element<Message> = add_row_container(
                                                                col_element, 
                                                                i, 
                                                                table.highlight_amount, 
                                                                table.row_highlight);

                                col_values.push(cnt)
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

fn add_row_container(label: Element<Message>, row_index: usize,
                    highlight_amount: f32, row_highlight: Option<TableRowHighLight>) 
                    -> Element<Message> {
    // Using container because text has no background 
    Container::new(label)
            .width(Length::Fill)
            .style(move|theme| table_row_theme(theme, row_index.clone(), 
                        highlight_amount.clone(),
                        row_highlight))
            .center_x()
            
            .into()
}

use iced::widget::Button;
fn add_widget(widget: TableWidget, table_id: usize, 
                        label: String, index: usize, 
                        col_index: &usize, is_checked: bool,
                        image_path: Option<String>) 
                        -> Element<'static, Message> {

    match widget {
        TableWidget::Button => {
            let txt = Text::new(label)
                                                            .horizontal_alignment(Horizontal::Center)
                                                            .width(Length::Fill);
                                                            
            let btn: Element<TableMessage> = Button::new(txt)
                                                            .padding(Padding::ZERO)
                                                            .width(Length::Fill)
                                                            .on_press(TableMessage::TableButton((*col_index, index))) 
                                                            .into(); 
            btn.map(move |message| app::Message::Table(table_id, message))
        },
        TableWidget::Checkbox => {
            let chk: Element<TableMessage> = Checkbox::new(label, is_checked)
                                                    .index((*col_index, index))
                                                    .width(Length::Shrink)
                                                    .on_toggle(TableMessage::TableCheckbox)
                                                    .into();
            chk.map(move |message| app::Message::Table(table_id, message))
        },
        TableWidget::Image | TableWidget::Svg => {
            let path = match image_path {
                Some(path) => path,
                None => panic!("Table: An image path must be defined when using images.")
            };
            let img: Element<TableMessage> = Image::<Handle>::new(path).into();

            let cont: Element<TableMessage> = Container::new(img)
                                                .width(Length::Fill)
                                                .padding(Padding::ZERO)
                                                .into();

            add_mousearea(cont, table_id, index, *col_index)
        },
    }
}


fn add_mousearea(content: Element<TableMessage>, table_id: usize, row: usize, col: usize) -> Element<Message> {
    
    let ma: Element<TableMessage> = 
                    MouseArea::new(content)
                    .id(table_id)
                    .table_pos((row, col))
                    .on_press(TableMessage::MouseAreaOnPress((row, col)))
                    .on_release(TableMessage::MouseAreaOnRelease((row, col)))
                    .on_right_press(TableMessage::MouseAreaOnRightPress((row, col)))
                    .on_right_release(TableMessage::MouseAreaOnRightRelease((row, col)))
                    .on_middle_press(TableMessage::MouseAreaOnMiddlePress((row, col)))
                    .on_middle_release(TableMessage::MouseAreaOnMiddleRelease((row, col)))
                    .on_enter(TableMessage::MouseAreaOnEnter((row, col)))
                    .on_move(TableMessage::MouseAreaOnMove)
                    .on_exit(TableMessage::MouseAreaOnExit((row, col)))
                    .interaction(Interaction::Pointer)
                    .into();

    ma.map(move |message| app::Message::Table(table_id, message))
    
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
        TableMessage::TableButton((col_index, row_index)) => {
            let mut wco: WidgetCallbackOut = get_set_widget_callback_data(wci);
            wco.id = table_id; 
            wco.index_table = Some((col_index, row_index));
            wco.event_name = "on_press_button".to_string();
            process_callback(wco);
        },
        TableMessage::TableCheckbox((col_index, row_index), on_toggle) => {
            wci.value_str =  Some("checkbox".to_string());
            wci.on_toggle = Some(on_toggle);
            wci.index_table = Some((col_index, row_index));
            let mut wco: WidgetCallbackOut = get_set_widget_callback_data(wci);
            wco.id = table_id;
            wco.value_str =  Some("checkbox".to_string());
            wco.event_name = "on_toggle_checkbox".to_string();
            wco.on_toggle = Some(on_toggle);
            wco.index_table = Some((col_index, row_index));
            process_callback(wco);
        },
        TableMessage::MouseAreaOnPress((col_index, row_index)) => {
            mousearea_callback(table_id, col_index, row_index, "on_press".to_string());
        },
        TableMessage::MouseAreaOnRelease((col_index, row_index)) => {
            mousearea_callback(table_id, col_index, row_index, "on_release".to_string());
        },
        TableMessage::MouseAreaOnRightPress((col_index, row_index)) => {
            mousearea_callback(table_id, col_index, row_index, "on_right_press".to_string());
        },
        TableMessage::MouseAreaOnRightRelease((col_index, row_index)) => {
            mousearea_callback(table_id, col_index, row_index, "on_right_release".to_string());
        },
        TableMessage::MouseAreaOnMiddlePress((col_index, row_index)) => {
            mousearea_callback(table_id, col_index, row_index, "on_middle_press".to_string());
        },
        TableMessage::MouseAreaOnMiddleRelease((col_index, row_index)) => {
            mousearea_callback(table_id, col_index, row_index, "on_middle_release".to_string());
        },
        TableMessage::MouseAreaOnEnter((col_index, row_index)) => {
            mousearea_callback(table_id, col_index, row_index, "on_enter".to_string());
        },
        TableMessage::MouseAreaOnMove(pointidrc) => {
            mousearea_callback_pointidrc(pointidrc, "on_move".to_string());
        },
        TableMessage::MouseAreaOnExit((col_index, row_index)) => {
            mousearea_callback(table_id, col_index, row_index, "on_exit".to_string());
        },
    }
}


pub fn mousearea_callback(table_id: usize, col_index: usize, row_index: usize, event_name: String) {
    
    let mut wci: WidgetCallbackIn = WidgetCallbackIn::default();
    wci.id = table_id;

    let mut wco = get_set_container_callback_data(wci);
    wco.id = table_id;
    wco.event_name = event_name;
    process_callback(wco);

}


fn mousearea_callback_pointidrc(pointid: PointIdRC, event_name: String) {

    let mut points: Vec<(String, f32)> = vec![];
    points.push(("x".to_string(), pointid.x));
    points.push(("y".to_string(), pointid.y));
    let id = pointid.id;
    
    let mut wci: WidgetCallbackIn = WidgetCallbackIn::default();
    wci.id = id;

    let mut wco = get_set_container_callback_data(wci);
    wco.id = id;
    wco.event_name = event_name;
    wco.points = Some(points);
    process_callback(wco);
    
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

    let mut widget_index: (usize, usize) = (0, 0);
    if wco.index_table.is_some() {
        widget_index = wco.index_table.unwrap();
    }
    
    if wco.value_str == Some("checkbox".to_string()) {
        
        let is_toggled = match wco.on_toggle {
            Some(checked) => checked,
            None => panic!("Table: no value for is_checked found")
        };
        Python::with_gil(|py| {
                if wco.user_data.is_some() {
                    let user_data = wco.user_data.unwrap();
                    let res = callback.call1(py, (
                                                                        wco.id.clone(),
                                                                        widget_index,
                                                                        is_toggled,  
                                                                        user_data
                                                                        ));
                    match res {
                        Ok(_) => (),
                        Err(er) => panic!("Table: 4 parameters (id, widget_index, is_checked, user_data) are required or a python error in this function. {er}"),
                    }
                    
                } else {
                    
                    let res = callback.call1(py, (
                                                                        wco.id.clone(),
                                                                        widget_index,
                                                                        is_toggled,  
                                                                        ));
                    match res {
                        Ok(_) => (),
                        Err(er) => panic!("Table: 3 parameter (id, widget_index, is_checked) are required or possibly a python error in this function. {er}"),
                    }
                }
        });
    } else {
        Python::with_gil(|py| {
            if wco.user_data.is_some() {
                let user_data = wco.user_data.unwrap();
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
                                                                    widget_index,  
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Table: 2 parameter (id, widget_index) are required or possibly a python error in this function. {er}"),
                }
                
            } 
        });
    }

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