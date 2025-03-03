#![allow(dead_code)]

use crate::app::{self, Message};
use crate::{access_window_actions, IpgState};

use iced::window;
use iced::{Element, Task, Theme, Size};
use iced::widget::Column;

use pyo3::{pyclass, PyObject, Python};

use super::helpers::{try_extract_boolean, try_extract_f64, try_extract_u64, try_extract_vec_f32};


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

pub fn add_windows(state: &mut IpgState) -> Vec<Task<app::Message>> {

    let mut modes: Vec<(usize, window::Mode)> = vec![];

    let mut spawn_window: Vec<Task<app::Message>> = vec![];

    for i in 0..state.windows.len() {
        let visible = match state.windows[i].mode {
            IpgWindowMode::Windowed => {
                modes.push((state.windows[i].id, window::Mode::Windowed));
                true
            },
            IpgWindowMode::FullScreen => {
                modes.push((state.windows[i].id, window::Mode::Fullscreen));
                true
            },
            IpgWindowMode::Closed => {
                modes.push((state.windows[i].id, window::Mode::Hidden));
                false
            },
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
        let id = state.windows[i].id;
        let debug = state.windows[i].debug;
        let theme = state.windows[i].theme.clone();
        let mode = state.windows[i].mode.clone();

        state.window_debug.insert(iced_id, (id, debug));
        state.window_theme.insert(iced_id, (id, theme));
        state.window_mode.insert(iced_id, (id, get_iced_mode(&mode)));

        let ipg_id = state.windows[i].id;
        state.windows_iced_ipg_ids.insert(iced_id, ipg_id);
        let size = state.windows[i].size;
        spawn_window.push(open.map(move|_|Message::WindowOpened(iced_id, None, size)));
        
    }

    spawn_window

}

pub fn construct_window(content: Vec<Element<app::Message>>) -> Element<app::Message> {
    Column::with_children(content).into()
}

// fn process_callback(wco: WidgetCallbackOut) 
// {
//     let app_cbs = access_callbacks();

//     let callback_present = app_cbs.callbacks.get(&(wco.id, wco.event_name.clone()));

//     let callback_opt = match callback_present {
//         Some(cb) => cb,
//         None => return,
//     };

//     let callback = match callback_opt {
//         Some(cb) => cb,
//         None => panic!("Window Callback could not be found with id {}", wco.id),
//     };

//     let value = match wco.value_str {
//         Some(vl) => vl,
//         None => panic!("Window value in callback could not be found"),
//     };

//     Python::with_gil(|py| {
//         if wco.user_data.is_some() {
//             let user_data = match wco.user_data {
//                 Some(ud) => ud,
//                 None => panic!("Window callback user_data not found."),
//             };
//             let res = callback.call1(py, (
//                                                                 wco.id, 
//                                                                 value, 
//                                                                 user_data
//                                                                 )
//                                                                 );
//             match res {
//                 Ok(_) => (),
//                 Err(er) => panic!("Window: 3 parameters (id, value, user_data) are required or a python error in this function. {er}"),
//             }
//         } else {
//             let res = callback.call1(py, (
//                                                                 wco.id, 
//                                                                 value, 
//                                                                 )
//                                                                 );
//             match res {
//                 Ok(_) => (),
//                 Err(er) => panic!("Window: 2 parameters (id, value) are required or a python error in this function. {er}"),
//             }
//         } 
//     });

//     drop(app_cbs);

// }


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
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


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgWindowParam {
    Decorations,
    Debug,
    Level,
    Mode,
    Position,
    Size,
    Theme,
    ScaleFactor,
}

pub fn window_item_update(wnd: &mut IpgWindow,
                            item: &PyObject,
                            value: &PyObject
                            )
{
    let update = try_extract_window_update(item);
    let name = "Window".to_string();
    match update {
        IpgWindowParam::Debug => {
            wnd.debug = try_extract_boolean(value, name);
        },
        IpgWindowParam::Theme => {
            let val = try_extract_ipg_theme(value);
            wnd.theme = get_iced_window_theme(val);
        },
        IpgWindowParam::ScaleFactor => {
            wnd.scale_factor = try_extract_f64(value, name);
        },
        IpgWindowParam::Mode => {
            let ipg_mode = try_extract_mode(value);
            let mode = get_iced_mode(&ipg_mode);
            wnd.mode = ipg_mode;
            let mut state = access_window_actions();
            state.mode.push((wnd.id, mode));
            drop(state)
        },
        IpgWindowParam::Decorations => {
            let val = try_extract_u64(value, name) as usize;
            let mut state = access_window_actions();
            state.decorations.push(val);
            drop(state)
        },
        IpgWindowParam::Level => {
            let ipg_level = try_extract_level(value);
            let level = get_level(&ipg_level);
            wnd.level = ipg_level;
            let mut state = access_window_actions();
            state.level.push((wnd.id, level));
            drop(state)
        },
        IpgWindowParam::Position => {
            let val = try_extract_vec_f32(value, name);
            let mut state = access_window_actions();
            state.position.push((wnd.id, val[0], val[1]));
            drop(state)
        },
        IpgWindowParam::Size => {
            let val = try_extract_vec_f32(value, name);
            let mut state = access_window_actions();
            state.resize.push((wnd.id, val[0], val[1]));
            drop(state)
        },
    }

}


fn try_extract_window_update(update_obj: &PyObject) -> IpgWindowParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgWindowParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Window update extraction failed"),
        }
    })
}

fn try_extract_ipg_theme(theme: &PyObject) -> IpgWindowTheme {

    Python::with_gil(|py| {
        let res = theme.extract::<IpgWindowTheme>(py);
        match res {
            Ok(theme) => theme,
            Err(_) => panic!("Window theme extraction failed"),
        }
    })
}

fn try_extract_mode(mode: &PyObject) -> IpgWindowMode {
    Python::with_gil(|py| {
        let res = mode.extract::<IpgWindowMode>(py);
        match res {
            Ok(mode) => mode,
            Err(e) => panic!("Window mode extraction failed with error {}", e),
        }
    })
}

fn try_extract_level(level: &PyObject) -> IpgWindowLevel {
    Python::with_gil(|py| {
        let res = level.extract::<IpgWindowLevel>(py);
        match res {
            Ok(level) => level,
            Err(e) => panic!("Window level extraction failed with error {}", e),
        }
    })
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
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
#[pyclass(eq, eq_int)]
pub enum IpgWindowMode {
    Windowed,
    FullScreen,
    Closed,
}

pub fn get_iced_mode(mode: &IpgWindowMode) -> window::Mode {
    match mode {
        IpgWindowMode::Windowed => window::Mode::Windowed,
        IpgWindowMode::FullScreen => window::Mode::Fullscreen,
        IpgWindowMode::Closed => window::Mode::Hidden,
    }
}

pub fn get_ipg_mode(mode: window::Mode) -> IpgWindowMode {
    match mode {
        window::Mode::Windowed => IpgWindowMode::Windowed,
        window::Mode::Fullscreen => IpgWindowMode::FullScreen,
        window::Mode::Hidden => IpgWindowMode::Closed,
    }
}
