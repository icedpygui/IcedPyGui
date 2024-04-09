
#![allow(unused_assignments)]

use std::collections::HashMap;

use crate::{access_state, access_callbacks};

use iced::event::Event;

use iced::keyboard::Event::{KeyPressed, KeyReleased, ModifiersChanged};
use iced::keyboard::{Key, Location, Modifiers};
use iced::mouse::Event::{ButtonPressed, ButtonReleased, CursorEntered, 
                        CursorLeft, CursorMoved, WheelScrolled};
use iced::mouse::Button::{Left, Right, Middle, Back, Forward, Other,};
use iced::mouse::ScrollDelta;

use pyo3::{PyObject, Python};
use pyo3::types::IntoPyDict;


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IpgKeyBoardEvent {
    pub id: usize,
    pub enabled: bool,
}

impl IpgKeyBoardEvent {
    pub fn new(
        id: usize,
        enabled: bool,
        ) -> Self {
        Self {
            id,
            enabled,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IpgMouseEvent {
    pub id: usize,
    pub enabled: bool,
}

impl IpgMouseEvent {
    pub fn new( 
        id: usize,
        enabled: bool,
        ) -> Self {
        Self {
            id,
            enabled,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IpgWindowEvent {
    pub id: usize,
    pub enabled: bool,
}

impl IpgWindowEvent {
    pub fn new(
        id: usize,
        enabled: bool,
        ) -> Self {
        Self {
            id,
            enabled,
        }
    }
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub enum IpgEvents {
    Keyboard(IpgKeyBoardEvent),
    Mouse(IpgMouseEvent),
    Window(IpgWindowEvent),
}

#[derive(Debug, Clone, PartialEq)]
pub enum IpgEventCallbacks {
    OnKeyPress,
    OnKeyRelease,
    OnEnterWindow,
    OnExitWindow,
    OnMove,
    OnLeftPress,
    OnRightPress,
    OnMiddlePress,
    OnBackPress,
    OnForwardPress,
    OnOtherPress,
    OnLeftRelease,
    OnRightRelease,
    OnMiddleRelease,
    OnBackRelease,
    OnForwardRelease,
    OnOtherRelease,
    OnMiddleScrollLine,
    OnMiddleScrollPixel,
    WindowOnOpened,
    WindowOnClosed,
    WindowOnMoved,
    WindowOnResized,
    None,
}

pub fn process_events(ipg_event: Event, 
                        key_enabled: (usize, bool), 
                        mouse_enabled: (usize, bool),
                        wnd_enabled: (usize, bool),
                        touch_enabled: (usize, bool)) 
    {   
        match ipg_event {
            Event::Keyboard(KeyPressed { key, 
                                        location, 
                                        modifiers, 
                                        text: _ }) => {
                if key_enabled.1 {
                    // dbg!("Pressed", &key, &location, &modifiers, &text);
                    let user_data = get_event_user_data(key_enabled.0);
                    
                    let key_str: String = process_key(key.as_ref());
                    
                    let mod_key = process_modifier(modifiers);

                    let location_str: String = process_location(location);

                    let hmap_s_s: Option<HashMap<String, String>> = Some(HashMap::from([
                                                                            ("key".to_string(), key_str),
                                                                            ("modifier".to_string(), mod_key),
                                                                            ("location".to_string(), location_str)
                                                                        ]));

                    process_callback(key_enabled.0, 
                                    "KeyPressed".to_string(),
                                    None,
                                    hmap_s_s,
                                    user_data, 
                                    IpgEventCallbacks::OnKeyPress);
                }
            },
            Event::Keyboard(KeyReleased { key, 
                                            location, 
                                            modifiers, 
                                            }) => {
                if key_enabled.1 {
                    // dbg!("Released", &key, location, modifiers);
                    let user_data = get_event_user_data(key_enabled.0);
                    
                    let key_str: String = process_key(key.as_ref());

                    let mod_key = process_modifier(modifiers);

                    let location_str: String = process_location(location);

                    let hmap_s_s: Option<HashMap<String, String>> = Some(HashMap::from([
                        ("key".to_string(), key_str),
                        ("modifier".to_string(), mod_key),
                        ("location".to_string(), location_str)
                    ]));

                    process_callback(key_enabled.0, 
                                    "KeyReleased".to_string(),
                                    None,
                                    hmap_s_s,
                                    user_data, 
                                    IpgEventCallbacks::OnKeyRelease);
                                    }
                                },
            // This event occurs when command keys are pressed but these 
            // also show up under key pressed and release so not sure any advantage on using this.
            Event::Keyboard(ModifiersChanged(_)) => (), 
            Event::Mouse(m_event) => {

                    let mut event_name = "".to_string();
                    let mut hmap_s_f: Option<HashMap<String, f32>> = None;

                    let mut cb_name = IpgEventCallbacks::None;

                    if mouse_enabled.1 {
                        match m_event {
                            CursorEntered => {
                                event_name = "Mouse Enter".to_string();
                                cb_name = IpgEventCallbacks::OnEnterWindow;
                            },
                            CursorLeft => {
                                event_name = "Mouse Exit".to_string();
                                cb_name = IpgEventCallbacks::OnExitWindow;
                            },
                            CursorMoved { position } => {
                                event_name = "Mouse Moved".to_string();
                                hmap_s_f = Some(HashMap::from([("x".to_string(), position.x),
                                                ("y".to_string(), position.y)]));
                                cb_name = IpgEventCallbacks::OnMove;
                            },
                            ButtonPressed(btn) => {
                                match btn {
                                    Left => {
                                        event_name = "Left Press".to_string();
                                        cb_name = IpgEventCallbacks::OnLeftPress;
                                    },
                                    Right => {
                                        event_name = "Right Press".to_string();
                                        cb_name = IpgEventCallbacks::OnRightPress;
                                    },
                                    Middle => {
                                        event_name = "Middle Press".to_string();
                                        cb_name = IpgEventCallbacks::OnMiddlePress;
                                    },
                                    Back => {
                                        event_name = "Back Press".to_string();
                                        cb_name = IpgEventCallbacks::OnBackPress;
                                    },
                                    Forward => {
                                        event_name = "Forward Press".to_string();
                                        cb_name = IpgEventCallbacks::OnForwardPress;
                                    },
                                    Other(other) => {
                                        event_name = "Other Press".to_string();
                                        hmap_s_f = Some(HashMap::from([("other".to_string(), other as f32)]));
                                        cb_name = IpgEventCallbacks::OnOtherPress;
                                    },
                                }
                            },
                            ButtonReleased(btn) => {
                                match btn {
                                    Left => {
                                        event_name = "Left Release".to_string();
                                        cb_name = IpgEventCallbacks::OnLeftRelease;
                                    },
                                    Right => {
                                        event_name = "Right Release".to_string();
                                        cb_name = IpgEventCallbacks::OnRightRelease;
                                    },
                                    Middle => {
                                        event_name = "Middle Release".to_string();
                                        cb_name = IpgEventCallbacks::OnMiddleRelease;
                                    },
                                    Back => {
                                        event_name = "Back Release".to_string();
                                        cb_name = IpgEventCallbacks::OnBackRelease;
                                    },
                                    Forward => {
                                        event_name = "Forward Release".to_string();
                                        cb_name = IpgEventCallbacks::OnForwardRelease;
                                    },
                                    Other(other) => {
                                        event_name = "Other Release".to_string();
                                        hmap_s_f = Some(HashMap::from([("other".to_string(), other as f32)]));
                                        cb_name = IpgEventCallbacks::OnOtherRelease;
                                    },
                                }
                            },
                            WheelScrolled { delta } => {
                                match delta {
                                    ScrollDelta::Lines { x:_, y } => {

                                        event_name = "ScrollDelta Line".to_string();

                                        hmap_s_f = Some(HashMap::from([("y".to_string(), y)]));

                                        cb_name = IpgEventCallbacks::OnMiddleScrollLine;
                                    },
                                    ScrollDelta::Pixels { x, y } => {

                                        event_name = "ScrollDelta Pixel".to_string();

                                        hmap_s_f = Some(HashMap::from([("x".to_string(), x),
                                                ("y".to_string(), y)]));
                                        
                                        cb_name = IpgEventCallbacks::OnMiddleScrollPixel;
                                    },
                                }
                                
                            },
                    }

                    let user_data = get_event_user_data(mouse_enabled.0);

                    process_callback(mouse_enabled.0, 
                                        event_name,
                                        hmap_s_f,
                                        None,
                                        user_data,
                                        cb_name);
                }
            },
            Event::Window(id, wnd_event) => {
                if wnd_enabled.1 {

                    let mut event_name = "".to_string();
                    let mut hmap_s_f: Option<HashMap<String, f32>>  = None;
                    let user_data: Option<PyObject> = get_event_user_data(wnd_enabled.0);
                    let mut cb_name: IpgEventCallbacks = IpgEventCallbacks::None;

                    match wnd_event {
                        iced::window::Event::Opened { position, size } => {
                            event_name = format!("Window {:?} Opened", id);
                            match position {
                                Some(pos) => {
                                    hmap_s_f = Some(HashMap::from([
                                                            ("x".to_string(), pos.x),
                                                            ("y".to_string(), pos.y),
                                                            ("width".to_string(), size.width),
                                                            ("height".to_string(), size.height),
                                                            ]));
                                },
                                None => {
                                    hmap_s_f = Some(HashMap::from([
                                                            ("width".to_string(), size.width),
                                                            ("height".to_string(), size.height),
                                                            ]));
                                },
                            }
                            cb_name = IpgEventCallbacks::WindowOnOpened;
                        },
                        iced::window::Event::Closed => {
                            event_name = format!("window {:?} Closed", id);
                            cb_name = IpgEventCallbacks::WindowOnClosed;
                        },
                        iced::window::Event::Moved { x, y } => {
                            event_name = format!("Window {:?} Moved", id);
                            hmap_s_f = Some(HashMap::from([
                                                        ("x".to_string(), x as f32),
                                                        ("y".to_string(), y as f32),
                                                        ]));
                            cb_name = IpgEventCallbacks::WindowOnMoved;
                        },
                        iced::window::Event::Resized { width, height } => {
                            event_name = format!("Window {:?} Resized", id);
                            hmap_s_f = Some(HashMap::from([
                                                        ("width".to_string(), width as f32),
                                                        ("height".to_string(), height as f32),
                                                        ]));
                            cb_name = IpgEventCallbacks::WindowOnResized;
                        },
                        iced::window::Event::RedrawRequested(_) => {},
                        iced::window::Event::CloseRequested => {},
                        iced::window::Event::Focused => {},
                        iced::window::Event::Unfocused => {},
                        iced::window::Event::FileHovered(_) => {},
                        iced::window::Event::FileDropped(_) => {},
                        iced::window::Event::FilesHoveredLeft => {},
                    }
                    
                    process_callback(wnd_enabled.0, 
                                        event_name,
                                        hmap_s_f,
                                        None,
                                        user_data,
                                        cb_name);
                }
                
            },
            Event::Touch(tch) => {
                if touch_enabled.1 {
                    match tch {
                        iced::touch::Event::FingerPressed { id, position } => {
                            dbg!(id, position);
                        },
                        iced::touch::Event::FingerMoved { id, position } => {
                            dbg!(id, position);
                        },
                        iced::touch::Event::FingerLifted { id, position } => {
                            dbg!(id, position);
                        },
                        iced::touch::Event::FingerLost { id, position } => {
                            dbg!(id, position);
                        },
                    }
                }
                
            },
        }
}

pub fn get_event_user_data(id: usize) -> Option<PyObject> {

    let state = access_state();
    let cb = access_callbacks();

    for event_type in state.events.iter() {
        
        match event_type {
            IpgEvents::Keyboard(_key) => {
                for data in cb.user_data.iter() {
                    if data.0 == id {
                        return data.1.clone()
                    }
                }
            },
            IpgEvents::Mouse(_mouse) => {
                for data in cb.user_data.iter() {
                    if data.0 == id {
                        return data.1.clone()
                    }
                }
            },
            IpgEvents::Window(_wnd) => {
                for data in cb.user_data.iter() {
                    if data.0 == id {
                        return data.1.clone()
                    }
                }
            },
        }
    }

    panic!("Event user data not found")

}

fn process_key(key: Key<&str>) -> String {
   
    match key {
        Key::Named(named) => {
            format!("{:?}", named)
        }
        Key::Character(chr) => {
            chr.to_string()
        },
        Key::Unidentified => {
            "unidentified key".to_string()
        },
    }
}


fn process_modifier(modifier: Modifiers) -> String {
    if modifier.control() {
        return "Control".to_string()
    }
    if modifier.alt() {
        return "Alt".to_string()
    }
    if modifier.shift() {
        return "Shift".to_string()
    }
    if modifier.logo() {
        return "Logo".to_string()
    }
    "None".to_string()
}

fn process_location(location: Location) -> String {
    match location {
        Location::Left => "left".to_string(),
        Location::Numpad => "numpad".to_string(),
        Location::Right => "right".to_string(),
        Location::Standard => "standard".to_string(),
    }
}

fn process_callback(id: usize, 
                    event_name: String,
                    hmap_s_f: Option<HashMap<String, f32>>,
                    hmap_s_s: Option<HashMap<String, String>>,
                    user_data: Option<PyObject>, 
                    cb_name: IpgEventCallbacks) 
{

    let app_cbs = access_callbacks();

    for callback in app_cbs.cb_events.iter() {
        if id == callback.id && cb_name == callback.name {
            let cb = &callback.cb;

            Python::with_gil(|py| {

                // hmap_s_f and hmap_s_s are never Some() together, one is always equal to None 
                // or both are equal to None.
                // Therefore only other option is the user_data. 
                // This simplifies the returning vars, since I prefer not to return a None.
                // Possibilities are (hmap_s_f), (hmap_s_f, user_data), or (hmap_ss), (hmap_s_s, user_data)
                //  or (None, None)

                if !hmap_s_f.is_some() && !hmap_s_s.is_some() {
                    match user_data {
                        Some(user_data) => {
                            cb.call1(py, (id, event_name, user_data)).unwrap()
                        },
                        None => {
                            cb.call1(py, (id, event_name)).unwrap()
                        },
                    };
                } else {
                    match hmap_s_f {
                        Some(sf) => {
                            let dict = sf.into_py_dict_bound(py);
                            match user_data {
                                Some(user_data) => {
                                    cb.call1(py, (id, event_name, dict, user_data)).unwrap()
                                },
                                None => {
                                    cb.call1(py, (id, event_name, dict)).unwrap()
                                },
                            };
                        },
                        None => {
                            match hmap_s_s {
                                Some(ss) => {
                                    let dict = ss.into_py_dict_bound(py);
                                    match user_data {
                                        Some(user_data) => {
                                            cb.call1(py, (id, event_name, dict, user_data)).unwrap()
                                        },
                                        None => {
                                            cb.call1(py, (id, event_name, dict)).unwrap()
                                        },
                                    };
                                },
                                None => (),
                            }
                        }
                    }
                }
            });

            break;
        }                   
    };
    drop(app_cbs);

    
}
