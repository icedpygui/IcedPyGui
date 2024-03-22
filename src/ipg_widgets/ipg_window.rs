#![allow(dead_code)]
use std::collections::HashMap;

use crate::app;
use crate::{access_state, access_callbacks};

use iced::window;
use iced::{Command, Element, Theme, Size};
use iced::widget::Column;

use pyo3::{PyObject, Python};

use super::callbacks::WidgetCallbackOut;


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
    pub debug: bool,
    pub user_data: Option<PyObject>,
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
        debug: bool,
        user_data: Option<PyObject>,
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
            debug,
            user_data,
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
                exit_on_close_request: true,
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
                Command::none()
            },
            WndMessage::NewWindow => {
                Command::none()
            },
            WndMessage::ScaleInputChanged(_id, _something) => {
                Command::none()
            },
            WndMessage::ScaleChanged(_id, _scale) => {
                Command::none()
            }, 
    }

}

fn process_callback(wco: WidgetCallbackOut) 
{
    let app_cbs = access_callbacks();

    let callback_present = app_cbs.callbacks.get(&(wco.id, wco.event_name.clone()));

    let callback_opt = match callback_present {
        Some(cb) => cb,
        None => return,
    };

    let callback = match callback_opt {
        Some(cb) => cb,
        None => panic!("Window Callback could not be found with id {}", wco.id),
    };

    let value = match wco.value_str {
        Some(vl) => vl,
        None => panic!("Window value in callback could not be found"),
    };

    Python::with_gil(|py| {
        if wco.user_data.is_some() {
            let user_data = match wco.user_data {
                Some(ud) => ud,
                None => panic!("Window callback user_data not found."),
            };
            callback.call1(py, (
                                    wco.id.clone(), 
                                    value, 
                                    user_data
                                    )
                            ).unwrap();
        } else {
            callback.call1(py, (
                                    wco.id.clone(), 
                                    value, 
                                    )
                            ).unwrap();
        } 
    });

    drop(app_cbs);

}
