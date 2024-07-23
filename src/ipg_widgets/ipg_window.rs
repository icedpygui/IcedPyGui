#![allow(dead_code)]
use std::collections::HashMap;

use crate::app;
use crate::{access_state, access_callbacks};

use iced::window;
use iced::{Command, Element, Theme, Size};
use iced::widget::Column;

use pyo3::{pyclass, PyObject, Python};

use super::callbacks::WidgetCallbackOut;
use super::helpers::try_extract_boolean;


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

    for i in 0..state.windows.len() {
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

            windows.insert(id, state.windows[i].clone());
        }
    }
    drop(state);

    // Set the state of the window debug to be used in the views
    let mut state = access_state();
    for (id, window) in windows.iter() {
        state.window_debug.insert(id.clone(), (window.id, window.debug.clone()));
        state.window_theme.insert(id.clone(), (window.id, window.theme.clone()));
    }


    (windows, spawn_window)

}

pub fn construct_window(content: Vec<Element<'static, app::Message>>) -> Element<'static, app::Message> {
    Column::with_children(content).into()
}

pub fn window_callback(message:WndMessage) -> Command<app::Message> {
    
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
            let res = callback.call1(py, (
                                                                wco.id.clone(), 
                                                                value, 
                                                                user_data
                                                                )
                                                                );
            match res {
                Ok(_) => (),
                Err(er) => panic!("Window: 3 parameters (id, value, user_data) are required or a python error in this function. {er}"),
            }
        } else {
            let res = callback.call1(py, (
                                                                wco.id.clone(), 
                                                                value, 
                                                                )
                                                                );
            match res {
                Ok(_) => (),
                Err(er) => panic!("Window: 2 parameters (id, value) are required or a python error in this function. {er}"),
            }
        } 
    });

    drop(app_cbs);

}


#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgWindowTheme {
    Dark,
    Light,
    CatppuccinLatte,
    CatppuccinFrappe,
    CatppuccinMacchiato,
    CatppuccinMocha,
    Dracula,
    Ferra,
    GruvboxLight,
    GruvboxDark,
    KanagawaWave,
    KanagawaDragon,
    KanagawaLotus,
    Moonfly,
    Nightfly,
    Nord,
    Oxocarbon,
    SolarizedLight,
    SolarizedDark,
    TokyoNight,
    TokyoNightStorm,
    TokyoNightLight,
}


fn extract_theme(theme_opt: Option<PyObject>) -> IpgWindowTheme {

    let theme = match theme_opt {
        Some(th) => th,
        None => return IpgWindowTheme::Dark,
    };

    Python::with_gil(|py| {
        let res = theme.extract::<IpgWindowTheme>(py);
            
        match res {
            Ok(theme) => theme,
            Err(_) => panic!("Window theme extraction failed."),
        }
    }) 
}

pub fn get_iced_window_theme(theme: IpgWindowTheme) -> Theme {

    match theme {
        IpgWindowTheme::Dark => Theme::Dark,
        IpgWindowTheme::Light => Theme::Light,
        IpgWindowTheme::CatppuccinLatte => Theme::CatppuccinLatte,
        IpgWindowTheme::CatppuccinFrappe => Theme::CatppuccinFrappe,
        IpgWindowTheme::CatppuccinMacchiato => Theme::CatppuccinMacchiato,
        IpgWindowTheme::CatppuccinMocha => Theme::CatppuccinMocha,
        IpgWindowTheme::Dracula => Theme::Dracula,
        IpgWindowTheme::Ferra => Theme::Ferra,
        IpgWindowTheme::GruvboxLight => Theme::GruvboxLight,
        IpgWindowTheme::GruvboxDark => Theme::GruvboxDark,
        IpgWindowTheme::KanagawaWave => Theme::KanagawaWave,
        IpgWindowTheme::KanagawaDragon => Theme::KanagawaDragon,
        IpgWindowTheme::KanagawaLotus => Theme::KanagawaLotus,
        IpgWindowTheme::Moonfly => Theme::Moonfly,
        IpgWindowTheme::Nightfly => Theme::Nightfly,
        IpgWindowTheme::Nord => Theme::Nord,
        IpgWindowTheme::Oxocarbon => Theme::Oxocarbon,
        IpgWindowTheme::SolarizedDark => Theme::SolarizedDark,
        IpgWindowTheme::SolarizedLight => Theme::SolarizedLight,
        IpgWindowTheme::TokyoNight => Theme::TokyoNight,
        IpgWindowTheme::TokyoNightLight => Theme::TokyoNightLight,
        IpgWindowTheme::TokyoNightStorm => Theme::TokyoNightStorm,
    }
}


#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgWindowParam {
    Debug,
    Theme,
}


pub fn window_item_update(wid: usize,
                            item: PyObject,
                            value: PyObject
                            )
{
    let update = try_extract_window_update(item);

    let mut state = access_state();

    match update {
        IpgWindowParam::Debug => {
            let val = try_extract_boolean(value);

            for (wnd_id, (id, _debug)) in state.window_debug.iter_mut() {
                if wid == *id {
                    let wnd_id = wnd_id.clone();
                    state.window_debug.entry(wnd_id).and_modify(|e| { e.1 = val });
                    drop(state);
                    return;
                }
            }
            drop(state);
        },
        IpgWindowParam::Theme => {
            let ipg_theme = try_extract_ipg_theme(value);
            let iced_theme = get_iced_window_theme(ipg_theme);

            for (wnd_id, (id, _theme)) in state.window_theme.iter_mut() {
                if wid == *id {
                    let wnd_id = wnd_id.clone();
                    state.window_theme.entry(wnd_id).and_modify(|e| { e.1 = iced_theme });
                    drop(state);
                    return;
                }
            }
            drop(state);
        },
    }

}


fn try_extract_window_update(update_obj: PyObject) -> IpgWindowParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgWindowParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Window update extraction failed"),
        }
    })
}

fn try_extract_ipg_theme(theme: PyObject) -> IpgWindowTheme {

    Python::with_gil(|py| {
        let res = theme.extract::<IpgWindowTheme>(py);
        match res {
            Ok(theme) => theme,
            Err(_) => panic!("Window theme extraction failed"),
        }
    })
}
