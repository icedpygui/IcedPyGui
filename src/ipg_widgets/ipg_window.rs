#![allow(dead_code)]
use std::collections::HashMap;

use crate::app::{self, Message};
use crate::{access_callbacks, access_state, access_window_actions};

use iced::window;
use iced::{Element, Task, Theme, Size};
use iced::widget::Column;

use pyo3::{pyclass, PyObject, Python};

use super::callbacks::WidgetCallbackOut;
use super::helpers::{try_extract_boolean, try_extract_f64, try_extract_tup_usize_f32_f32, try_extract_u64};


#[derive(Debug, Clone)]
pub struct IpgWindow {
    pub id: usize,
    pub title: String,
    pub size: Size,
    pub min_size: Option<Size>,
    pub max_size: Option<Size>,
    pub theme: Theme,
    pub position: window::Position,
    pub exit_on_close_request: bool,
    pub resizable: bool,
    pub mode: IpgWindowMode,
    pub decorations: bool,
    pub transparent: bool,
    pub level: IpgWindowLevel,
    pub scale_factor: f64,
    pub debug: bool,
    pub user_data: Option<PyObject>,
}

impl IpgWindow {
    pub fn new(
        id: usize, 
        title: String,
        size: Size,
        min_size: Option<Size>,
        max_size: Option<Size>,
        position: window::Position,
        exit_on_close_request: bool,
        theme: Theme,
        resizable: bool,
        mode: IpgWindowMode,
        decorations: bool,
        transparent: bool,
        level: IpgWindowLevel,
        scale_factor: f64,
        debug: bool,
        user_data: Option<PyObject>,
        ) -> Self {
        Self {
            id,
            title,
            size,
            min_size,
            max_size,
            position,
            exit_on_close_request,
            theme,
            resizable,
            mode,
            decorations,
            transparent,
            level,
            scale_factor,
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

pub fn add_windows() -> (HashMap<window::Id, IpgWindow>, Vec<Task<app::Message>>) {

    let mut state = access_state();

    let mut windows = HashMap::new();

    let mut spawn_window: Vec<Task<app::Message>> = vec![];

    for i in 0..state.windows.len() {
        let visible = match state.windows[i].mode {
            IpgWindowMode::Windowed => true,
            IpgWindowMode::Fullscreen => true,
            IpgWindowMode::Closed => false,
        };
        let (iced_id, open) = window::open(window::Settings {
            size: state.windows[i].size,
            min_size: state.windows[i].min_size,
            max_size: state.windows[i].max_size,
            position: state.windows[i].position,
            visible,
            resizable: state.windows[i].resizable,
            decorations: state.windows[i].decorations,
            transparent: state.windows[i].transparent,
            level: get_level(&state.windows[i].level),
            exit_on_close_request: state.windows[i].exit_on_close_request,
            ..Default::default()
        });

        windows.insert(iced_id, state.windows[i].clone());
        let ipg_id = state.windows[i].id.clone();
        state.windows_iced_ipg_ids.insert(iced_id, ipg_id);
        let size = state.windows[i].size.clone();
        spawn_window.push(open.map(move|_|Message::WindowOpened(iced_id, None, size)));
        
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

pub fn window_callback(message:WndMessage) -> Task<app::Message> {
    
    let mut _state = access_state();

    match message {
            WndMessage::TitleChanged(_id, _title) => {
                Task::none()
            },
            WndMessage::NewWindow => {
                Task::none()
            },
            WndMessage::ScaleInputChanged(_id, _something) => {
                Task::none()
            },
            WndMessage::ScaleChanged(_id, _scale) => {
                Task::none()
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
    DistroyWindow,
    Decorations,
    Debug,
    ExitOnCloseRequest,
    Level,
    MinSize,
    MaxSize,
    Mode,
    Position,
    Resizable,
    Size,
    Theme,
    Transparent,
    ScaleFactor,
}



pub fn window_item_update(wnd: &mut IpgWindow,
                            item: PyObject,
                            value: PyObject
                            )
{
    let update = try_extract_window_update(item);

    match update {
        IpgWindowParam::Debug => {
            wnd.debug = try_extract_boolean(value);
        },
        IpgWindowParam::Theme => {
            let val = try_extract_ipg_theme(value);
            wnd.theme = get_iced_window_theme(val);
        },
        IpgWindowParam::ScaleFactor => {
            wnd.scale_factor = try_extract_f64(value);
        },
        IpgWindowParam::Mode => {
            let ipg_mode = try_extract_mode(value);
            let mode = get_mode(&ipg_mode);
            wnd.mode = ipg_mode;
            let mut state = access_window_actions();
            state.mode.push((mode, wnd.id));
            drop(state)
        },
        IpgWindowParam::Decorations => {
            let val = try_extract_u64(value) as usize;
            let mut state = access_window_actions();
            state.decorations.push(val);
            drop(state)
        },
        IpgWindowParam::ExitOnCloseRequest => {
           
        },
        IpgWindowParam::DistroyWindow => {

        }
        IpgWindowParam::Level => (),
        IpgWindowParam::MinSize => (),
        IpgWindowParam::MaxSize => (),
        IpgWindowParam::Position => {
            let val = try_extract_tup_usize_f32_f32(value);
            let mut state = access_window_actions();
            state.position.push(val);
            drop(state)
        },
        IpgWindowParam::Resizable => (),
        IpgWindowParam::Size => {
            let val = try_extract_tup_usize_f32_f32(value);
            let mut state = access_window_actions();
            state.resize.push(val);
            drop(state)
        },
        IpgWindowParam::Transparent => (),
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

fn try_extract_mode(mode: PyObject) -> IpgWindowMode {
    Python::with_gil(|py| {
        let res = mode.extract::<IpgWindowMode>(py);
        match res {
            Ok(mode) => mode,
            Err(_) => panic!("Window mode extraction failed"),
        }
    })
}

#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgWindowLevel {
    Normal,
    AlwaysOnBottom,
    AlwaysOnTop,
}

fn get_level(level: &IpgWindowLevel) -> iced::window::Level {
    match level {
        IpgWindowLevel::Normal => window::Level::Normal,
        IpgWindowLevel::AlwaysOnBottom => window::Level::AlwaysOnBottom,
        IpgWindowLevel::AlwaysOnTop => window::Level::AlwaysOnTop,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[pyclass]
pub enum IpgWindowMode {
    Windowed,
    Fullscreen,
    Closed,
}

fn get_mode(mode: &IpgWindowMode) -> window::Mode {
    match mode {
        IpgWindowMode::Windowed => window::Mode::Windowed,
        IpgWindowMode::Fullscreen => window::Mode::Fullscreen,
        IpgWindowMode::Closed => window::Mode::Hidden,
    }
}