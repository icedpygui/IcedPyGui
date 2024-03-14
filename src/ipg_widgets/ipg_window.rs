#![allow(unused)]
use std::collections::HashMap;

use crate::app;
use crate::{access_state, access_callbacks};

use iced::window;
use iced::{Command, Element, Length, Theme, Size};
use iced::widget::Column;

use crate::iced_widgets::scrollable::Direction;

use pyo3::{PyObject, Python};


#[derive(Debug, Clone)]
pub struct IpgWindow {
    pub id: usize,
    pub window_index: usize,
    pub window_id: String,
    pub title: String,
    pub width: f32,
    pub height: f32,
    pub theme: Theme,
    pub position: window::Position,
    pub exit_on_close_request: bool,
    pub resizable: bool,
    pub visible: bool,
    pub scroll: bool,
    pub scroll_width: Length,
    pub scroll_height: Length,
    pub scroll_direction: Direction,
    pub debug: bool,
    pub user_data: Option<PyObject>,
    pub cb_name: Option<String>,
}

impl IpgWindow {
    pub fn new(
        id: usize, 
        window_index: usize,
        window_id: String,
        title: String,
        width: f32,
        height: f32,
        position: window::Position,
        exit_on_close_request: bool,
        theme: Theme,
        resizable: bool,
        visible: bool,
        scroll: bool,
        scroll_width: Length,
        scroll_height: Length,
        scroll_direction: Direction,
        debug: bool,
        user_data: Option<PyObject>,
        cb_name: Option<String>,
        ) -> Self {
        Self {
            id,
            window_index,
            window_id,
            title,
            width,
            height,
            position,
            exit_on_close_request,
            theme,
            resizable,
            visible,
            scroll,
            scroll_width,
            scroll_height,
            scroll_direction,
            debug,
            user_data,
            cb_name,
        }
    }
}


#[derive(Debug, Clone)]
pub enum WndMessage {
    TitleChanged(window::Id, String),
    NewWindow,
    ScaleInputChanged(window::Id, String),
    ScaleChanged(window::Id, String), 
}

pub fn add_windows() -> (HashMap<window::Id, IpgWindow>, Vec<Command<app::Message>>) {

    let state = access_state();

    let mut windows = HashMap::from([(window::Id::MAIN, state.windows[0].clone())]);

    let mut spawn_window: Vec<Command<app::Message>> = vec![];

    for (i, ipg_window) in state.windows.iter().enumerate() {
        // The first window i=0 is handled differently
        if i > 0 {
            let (id, spawn) = window::spawn(window::Settings {
                size: Size::new(state.windows[i].width, state.windows[i].height),
                position: state.windows[i].position,
                visible: state.windows[i].visible,
                resizable: state.windows[i].resizable,
                exit_on_close_request: true,//state.windows[i].exit_on_close_request,
                ..Default::default()
            }) as (window::Id, Command<app::Message>);

            spawn_window.push(spawn);

            windows.insert(id, ipg_window.clone());

        }
        
    }
    drop(state);
    (windows, spawn_window)

}

pub fn construct_window(content: Vec<Element<'static, app::Message>>) -> Element<'static, app::Message> {
    Column::with_children(content).into()
}

pub fn window_update(message:WndMessage) -> Command<app::Message> {
    
    let mut _state = access_state();

    match message {
            WndMessage::TitleChanged(_id, _title) => {

            },
            WndMessage::NewWindow => {

            },
            WndMessage::ScaleInputChanged(_id, _something) => {

            },
            WndMessage::ScaleChanged(_id, _scale) => {

            }, 
    }
    Command::none()
}

fn process_callback_resized(id: usize,
                    event_name: String,
                    width: f32,
                    height: f32,
                    user_data: Option<PyObject>, 
                    cb_name: Option<String>) 
{
    if !cb_name.is_some() {return}

    let app_cbs = access_callbacks();

    let mut found_callback = None;

    for callback in app_cbs.callbacks.iter() {

        if id == callback.id && cb_name == callback.name {

            found_callback = match callback.cb.clone() {
                Some(cb) => Some(cb),
                None => {drop(app_cbs); panic!("Callback could not be found with id {}", id)},
            };
            break;
        }                   
    };
    drop(app_cbs);

    match found_callback {

    Some(cb) => Python::with_gil(|py| {
                            match user_data {
                                Some(ud) => cb.call1(py, 
                                                                (id.clone(),
                                                                event_name,
                                                                width,
                                                                height, 
                                                                ud)).unwrap(),
                                None => cb.call1(py, 
                                                (id.clone(),
                                                width,
                                                height, 
                                                event_name)).unwrap(),
                            }
                        }),
    None => panic!("Button callback could not be found"),
    };

}