//! ipg_events
#![allow(unused_assignments)]
#![allow(clippy::enum_variant_names)]
use std::collections::HashMap;

use crate::{access_events, access_user_data1, access_window_actions, IpgState};

use iced::event::Event;
use iced::keyboard::Event::{KeyPressed, KeyReleased, ModifiersChanged};
use iced::keyboard::{Key, Location, Modifiers};
use iced::mouse::Event::{ButtonPressed, ButtonReleased, CursorEntered, 
                        CursorLeft, CursorMoved, WheelScrolled};
use iced::mouse::Button::{Left, Right, Middle, Back, Forward, Other,};
use iced::mouse::ScrollDelta;

use iced::window;
use pyo3::Python;


#[derive(Debug, Clone, Eq, PartialEq, Default)]
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

pub fn process_keyboard_events(event: Event, event_id: usize) 
{   
    match event {    
        Event::Keyboard(KeyPressed { key, 
                                    location, 
                                    modifiers, 
                                    text: _ ,
                                    physical_key: _, 
                                    modified_key: _ }) => {

            let event_name = "key pressed".to_string();
            
            let key_str: String = process_key(key.as_ref());
            
            let mod_key = process_modifier(modifiers);

            let location_str: String = process_location(location);

            let hmap_s_s: HashMap<String, String> = 
                HashMap::from([
                    ("name".to_string(), event_name.clone()),
                    ("key".to_string(), key_str),
                    ("modifier".to_string(), mod_key),
                    ("location".to_string(), location_str)
                ]);

            process_keyboard_callback(event_id, 
                                        event_name,
                                        hmap_s_s, 
                                        );
            
        },
        Event::Keyboard(KeyReleased { key, location, modifiers, }) => {

            let event_name = "key released".to_string();
            
            let key_str: String = process_key(key.as_ref());

            let mod_key = process_modifier(modifiers);

            let location_str: String = process_location(location);

            let hmap_s_s: HashMap<String, String> = HashMap::from([
                ("name".to_string(), event_name.clone()),
                ("key".to_string(), key_str),
                ("modifier".to_string(), mod_key),
                ("location".to_string(), location_str)
            ]);

            process_keyboard_callback(event_id,
                                        event_name, 
                                        hmap_s_s, 
                                        );
            
        },
        // This event occurs when command keys are pressed but these 
        // also show up under key pressed and release so not sure any advantage on using this.
        Event::Keyboard(ModifiersChanged(_)) => (),
        Event::Mouse(_) => (),
        Event::Window(_) => (),
        Event::Touch(_) => (),
    }
}


pub fn process_mouse_events(event: Event, event_id: usize)
{
    let mut hmap_s_f: Option<HashMap<String, f32>> = None;
    
    match event {
        Event::Mouse(m_event) => {
            let event_name = match m_event {
                CursorEntered => {
                    "enter window".to_string()
                },
                CursorLeft => {
                    "exit window".to_string()
                },
                CursorMoved { position } => {
                    hmap_s_f = Some(HashMap::from([("x".to_string(), position.x),
                                    ("y".to_string(), position.y)]));
                    "move".to_string()
                },
                ButtonPressed(btn) => {
                    match btn {
                        Left => {
                            "left press".to_string()
                        },
                        Right => {
                            "right press".to_string()
                        },
                        Middle => {
                            "middle press".to_string()
                        },
                        Back => {
                            "back press".to_string()
                        },
                        Forward => {
                            "forward press".to_string()
                        },
                        Other(other) => {
                            hmap_s_f = Some(HashMap::from([("other".to_string(), other as f32)]));
                            "other press".to_string()
                        },
                    }
                    },
                    ButtonReleased(btn) => {
                        match btn {
                            Left => {
                                "left release".to_string()
                            },
                            Right => {
                                "right release".to_string()
                            },
                            Middle => {
                                "middle release".to_string()
                            },
                            Back => {
                                "back release".to_string()
                            },
                            Forward => {
                                "forward release".to_string()
                            },
                            Other(other) => {
                                hmap_s_f = Some(HashMap::from([("other".to_string(), other as f32)]));
                                "other release".to_string()
                            },
                        }
                    },
                    WheelScrolled { delta } => {
                        match delta {
                            ScrollDelta::Lines { x, y } => {
                                hmap_s_f = Some(HashMap::from([("x".to_string(), x),
                                                                ("y".to_string(), y)]));
                                "middle scroll line".to_string()
                            },
                            ScrollDelta::Pixels { x, y } => {
                                hmap_s_f = Some(HashMap::from([("x".to_string(), x),
                                                                ("y".to_string(), y)]));
                                "middle scroll pixel".to_string()
                            },
                        }
                    }
            };

            process_mouse_callback(event_id,
                                    event_name,
                                    hmap_s_f,
                                    )
        },
        Event::Keyboard(_) => (),
        Event::Window(_) => (),
        Event::Touch(_) => (),
    }
        
}

pub fn process_touch_events(event: Event, event_id: usize) {
    let mut event_name = "".to_string();
    match event {
        Event::Touch(tch) => {
            let (hmap_s_fg, hmap_s_pt) = match tch {
                iced::touch::Event::FingerPressed { id, position } => {
                    let hmap_s_fg = HashMap::from([("finger".to_string(), id.0)]);
                    let hmap_s_pt = HashMap::from([("position".to_string(), (position.x, position.y))]);
                    event_name = "finger pressed".to_string();
                    (hmap_s_fg, hmap_s_pt)
                },
                iced::touch::Event::FingerMoved { id, position } => {
                    let hmap_s_fg = HashMap::from([("finger".to_string(), id.0)]);
                    let hmap_s_pt = HashMap::from([("position".to_string(), (position.x, position.y))]);
                    event_name = "finger moved".to_string();
                    (hmap_s_fg, hmap_s_pt)
                },
                iced::touch::Event::FingerLifted { id, position } => {
                    let hmap_s_fg = HashMap::from([("finger".to_string(), id.0)]);
                    let hmap_s_pt =HashMap::from([("position".to_string(), (position.x, position.y))]);
                    event_name = "finger lifted".to_string();
                    (hmap_s_fg, hmap_s_pt)
                },
                iced::touch::Event::FingerLost { id, position } => {
                    let hmap_s_fg = HashMap::from([("finger".to_string(), id.0)]);
                    let hmap_s_pt = HashMap::from([("position".to_string(), (position.x, position.y))]);
                    event_name = "finger lost".to_string();
                    (hmap_s_fg, hmap_s_pt)
                },
            };
            process_touch_callback(event_id,
                                event_name,
                                hmap_s_fg,
                                hmap_s_pt,
                                )
            
        },
        Event::Window(_) => (),
        Event::Keyboard(_) => (),
        Event::Mouse(_) => (),
    }
}


pub fn process_window_event(state: &mut IpgState,
                            event: Event,
                            window_id: window::Id)
{
    let event_id = state.window_event_id_enabled.0;
    let _event_enabled = state.window_event_id_enabled.1;

    let mut hmap_s_f: Option<HashMap<String, f32>> = None;
    let mut hmap_s_s: Option<HashMap<String, String>> = None;
   
    let ipg_id = match state.windows_iced_ipg_ids.get(&window_id) {
        Some(id) => *id,
        None => panic!("Process window event: Unable to find the ipg window id using the iced id {:?}.", window_id)
    };

    let event_name: Option<String> = match event {
        Event::Window(window::Event::Opened { position: _, size: _ } )=> {
            Some("opened".to_string())
        },
        Event::Window(window::Event::Closed) => {
            let mut actions = access_window_actions();
                actions.mode.push((ipg_id, window::Mode::Hidden));
                drop(actions);

            if !state.windows_hidden.contains(&window_id){
                state.windows_hidden.push(window_id);
            }
            Some("closed".to_string())
        },
        Event::Window(window::Event::Moved(point)) => {
            hmap_s_f = Some(HashMap::from([
                            ("x".to_string(), point.x),
                            ("y".to_string(), point.y),
                            ]));
            Some("moved".to_string())
        },
        Event::Window(window::Event::Resized (size)) => {
            hmap_s_f = Some(HashMap::from([
                            ("width".to_string(), size.width),
                            ("height".to_string(), size.height),
                            ]));
            Some("resized".to_string())
        },
        Event::Window(window::Event::RedrawRequested(_)) => {
            Some("redraw requested".to_string())
        },
        Event::Window(window::Event::CloseRequested ) => {
            let mut actions = access_window_actions();
                actions.mode.push((ipg_id, window::Mode::Hidden));
                drop(actions);

            if !state.windows_hidden.contains(&window_id){
                state.windows_hidden.push(window_id);
            }
            Some("close requested".to_string())
        },
        Event::Window(window::Event::Focused) => {
            Some("focused".to_string())
        },
        Event::Window(window::Event::Unfocused) => {
            Some("unfocused".to_string())
        },
        Event::Window(window::Event::FileHovered(path)) => {
            hmap_s_s = Some(HashMap::from([
                                        ("file path".to_string(), 
                                        path.display().to_string()),
                                        ]));
            Some("file hovered".to_string())
        },
        Event::Window(window::Event::FileDropped(path)) => {
            hmap_s_s = Some(HashMap::from([
                                        ("file path".to_string(), 
                                        path.display().to_string()),
                                        ]));
            Some("file dropped".to_string())
        },
        Event::Window(window::Event::FilesHoveredLeft) => {
            Some("files hovered left".to_string())
        },
        Event::Keyboard(_) => None,
        Event::Mouse(_) => None,

        Event::Touch(_) => None,
    };

    if event_name.is_some() {
        process_window_callback(
            ipg_id,
            event_id,
            event_name.unwrap(), 
            hmap_s_f,
            hmap_s_s,
            );
    }
        
}

fn process_window_callback(
    win_id: usize,
    event_id: usize,
    name: String,
    hmap_s_f: Option<HashMap<String, f32>>,
    hmap_s_s: Option<HashMap<String, String>>,) 
{
    let ud = access_user_data1();
    let user_data_opt = ud.user_data.get(&win_id);

    let app_event = access_events();
    let event = match app_event.events.get(&(event_id, name.clone())) {
        Some(cb) => Python::with_gil(|py| cb.clone_ref(py)),
        None => return,
    };

    drop(app_event);

    Python::with_gil(|py| {
        let res = match (user_data_opt, hmap_s_f, hmap_s_s) {
            (Some(user_data), Some(hmap_f), Some(hmap_s)) => {
                event.call1(py, (win_id, name, hmap_f, hmap_s, user_data))
            }
            (Some(user_data), Some(hmap_f), None) => {
                event.call1(py, (win_id, name, hmap_f, user_data))
            }
            (Some(user_data), None, Some(hmap_s)) => {
                event.call1(py, (win_id, name, hmap_s, user_data))
            }
            (Some(user_data), None, None) => {
                event.call1(py, (win_id, name, user_data))
            }
            (None, Some(hmap_f), Some(hmap_s)) => {
                event.call1(py, (win_id, name, hmap_f, hmap_s))
            }
            (None, Some(hmap_f), None) => {
                event.call1(py, (win_id, name, hmap_f))
            }
            (None, None, Some(hmap_s)) => {
                event.call1(py, (win_id, name, hmap_s))
            }
            (None, None, None) => {
                event.call1(py, (win_id, name,))
            }
        };

        if let Err(err) = res {
            panic!("Window Event callback error: {err}");
        }
    });

    drop(ud);
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

fn process_keyboard_callback(
        id: usize,
        event_name: String, 
        hmap_s_s: HashMap<String, String>,) 
{   
    let ud = access_user_data1();
    let app_event = access_events();
    
    let user_data_opt = ud.user_data.get(&id);
    
    let event = 
        match app_event.events.get(&(id, event_name)) {
            Some(cb) => cb,
            None => return,
        };
    
    let cb = 
        Python::with_gil(|py| {
            event.clone_ref(py)
        });
    
    drop(app_event);
    
    Python::with_gil(|py| {
        let res = match user_data_opt {
            Some(user_data) => cb.call1(py, (id, hmap_s_s, user_data)),
            None => cb.call1(py, (id, hmap_s_s)),
        };

        if let Err(err) = res {
            panic!("Keyboard Event callback error: {err}");
        }
    });
    
    drop(ud);      

}

fn process_mouse_callback(
        id: usize,
        event_name: String,
        hmap_s_f: Option<HashMap<String, f32>>,) 
{
    let ud = access_user_data1();
    let user_data_opt = ud.user_data.get(&id);

    let app_event = access_events();
    let event = match app_event.events.get(&(id, event_name)) {
        Some(cb) => Python::with_gil(|py| cb.clone_ref(py)),
        None => return,
    };

    drop(app_event);

    Python::with_gil(|py| {
        let res = match (user_data_opt, hmap_s_f) {
            (Some(user_data), Some(hmap)) => event.call1(py, (id, hmap, user_data)),
            (None, Some(hmap)) => event.call1(py, (id, hmap)),
            (Some(user_data), None) => event.call1(py, (id, user_data)),
            (None, None) => event.call1(py, (id,)),
        };

        if let Err(err) = res {
            panic!("Mouse Event callback error: {err}");
        }
    });

    drop(ud);
}

fn process_touch_callback(
    id: usize,
    event_name: String,
    hmap_s_fg: HashMap<String, u64>,
    hmap_s_pt: HashMap<String, (f32, f32)>,) 
{
    let ud = access_user_data1();
    let user_data_opt = ud.user_data.get(&id);

    let app_event = access_events();
    let event = match app_event.events.get(&(id, event_name)) {
        Some(cb) => Python::with_gil(|py| cb.clone_ref(py)),
        None => return,
    };

    drop(app_event);

    Python::with_gil(|py| {
        let res = match user_data_opt {
            Some(user_data) => event.call1(py, (id, hmap_s_fg, hmap_s_pt, user_data)),
            None => event.call1(py, (id, hmap_s_fg, hmap_s_pt)),
        };

        if let Err(err) = res {
            panic!("Touch Event callback error: {err}");
        }
    });

    drop(ud);
}