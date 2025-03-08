//! ipg_events
#![allow(unused_assignments)]
#![allow(clippy::enum_variant_names)]
use std::collections::HashMap;

use crate::ipg_widgets::ipg_window::get_ipg_mode;
use crate::{access_events, access_user_data, access_window_actions, IpgState};

use iced::event::Event;
use iced::keyboard::Event::{KeyPressed, KeyReleased, ModifiersChanged};
use iced::keyboard::{Key, Location, Modifiers};
use iced::mouse::Event::{ButtonPressed, ButtonReleased, CursorEntered, 
                        CursorLeft, CursorMoved, WheelScrolled};
use iced::mouse::Button::{Left, Right, Middle, Back, Forward, Other,};
use iced::mouse::ScrollDelta;

use iced::window;
use pyo3::Python;

use super::ipg_enums::IpgContainers;

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
                            window_id: window::Id,
                        ) -> bool
{
    let event_id = state.window_event_id_enabled.0;
    let event_enabled = state.window_event_id_enabled.1; 

    let mut hmap_s_f: Option<HashMap<String, f32>> = None;
    let mut hmap_s_s: Option<HashMap<String, String>> = None;
   
    let ipg_id = match state.windows_iced_ipg_ids.get(&window_id) {
        Some(id) => *id,
        None => panic!("Process window event: Unable to find the ipg window id using the iced id {:?}.", window_id)
    };

    match event {
        Event::Window(event) => {
            if ((event == iced::window::Event::CloseRequested) || 
                event == iced::window::Event::Closed) && !event_enabled {
                let mut actions = access_window_actions();
                        actions.mode.push((ipg_id, window::Mode::Hidden));
                        drop(actions);

                state.windows_opened -= 1;
                if state.windows_opened == 0 {return true}
                handle_window_closing(state, window_id, window::Mode::Hidden);
                
            } else if !event_enabled {
                return false
            }
            let event_name = match event {
                // Cannot use window open since a window need to be predefined.
                // Py user will use show and hide to the same effect.
                iced::window::Event::Opened { position: _, size: _ } => {
                    state.windows_opened += 1;
                    "opened".to_string()
                },
                iced::window::Event::Closed => {
                    state.windows_opened -= 1;
                    handle_window_closing(state, window_id, window::Mode::Hidden);
                    "closed".to_string()
                },
                iced::window::Event::Moved(point) => {
                    hmap_s_f = Some(HashMap::from([
                                    ("x".to_string(), point.x),
                                    ("y".to_string(), point.y),
                                    ]));
                    "moved".to_string()
                },
                iced::window::Event::Resized (size) => {
                    
                    hmap_s_f = Some(HashMap::from([
                                    ("width".to_string(), size.width),
                                    ("height".to_string(), size.height),
                                    ]));
                    "resized".to_string()
                },
                iced::window::Event::RedrawRequested(_) => {
                    "redraw requested".to_string()
                },
                iced::window::Event::CloseRequested => {
                    //  if callback present, don't close window
                    let name = "close requested".to_string();
                    let cb = check_callback_if_none(event_id, name.clone());
                    
                    if !cb {
                        let mut actions = access_window_actions();
                        actions.mode.push((ipg_id, window::Mode::Hidden));
                        drop(actions);
                        let is_empty = handle_window_closing(state, window_id, window::Mode::Hidden);
                       
                        if is_empty {
                            return true;
                        }
                    }
                    
                    name
                    
                },
                iced::window::Event::Focused => {
                    "focused".to_string()
                },
                iced::window::Event::Unfocused => {
                    "unfocused".to_string()
                },
                iced::window::Event::FileHovered(path) => {
                    hmap_s_s = Some(HashMap::from([
                                                ("file path".to_string(), 
                                                path.display().to_string()),
                                                ]));
                    "file hovered".to_string()
                },
                iced::window::Event::FileDropped(path) => {
                    hmap_s_s = Some(HashMap::from([
                                                ("file path".to_string(), 
                                                path.display().to_string()),
                                                ]));
                    "file dropped".to_string()
                },
                iced::window::Event::FilesHoveredLeft => {
                    "files hovered left".to_string()
                },
            };

            process_window_callback(ipg_id,
                                    event_name, 
                                    hmap_s_f,
                                    hmap_s_s,
                                    );
        },
        Event::Keyboard(_) => (),
        Event::Mouse(_) => (),
        Event::Touch(_) => (),   
    }
    false
}

pub fn handle_window_closing(state: &mut IpgState, iced_id: window::Id, mode: window::Mode) -> bool {

    let mut all_hidden = true;
    for (ic_id, (_ipg_id, md)) in state.window_mode.iter_mut() {
        if ic_id == &iced_id {
            *md = mode;
        }
        if *md != window::Mode::Hidden {
            all_hidden = false;
        }
    }

    if all_hidden {
        return true;
    }

    // Windows are never destroyed, just hidden
    let ipg_id_opt = state.windows_iced_ipg_ids.get(&iced_id);

    let ipg_id_found = match ipg_id_opt {
        Some(id) => *id,
        None => panic!("Events: handle_window_closing: Unable to find ipg_id based on Iced_id {:?}", iced_id),
    };

    // Needed a clone here because can't borrow again from state below
    let iced_ipg_ids = state.windows_iced_ipg_ids.clone();

    // if any of the remaining windows are visible, then return false
    for (_iced_id, ipg_id) in iced_ipg_ids {

        if let Some(IpgContainers::IpgWindow(wnd)) = 
            state.containers.get_mut(&ipg_id) {
                if wnd.id == ipg_id_found {
                    wnd.mode = get_ipg_mode(mode);
                }
            }
    }
    false
}

fn check_callback_if_none(id: usize, event_name: String) -> bool {
    let cbs = access_events();

    let cb_opt= cbs.events
                                    .get(&(id, event_name));
    
    let check = match cb_opt {
        Some(_) => true,
        None => false,
    };
    drop(cbs);
    check
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

fn process_keyboard_callback(id: usize,
                    event_name: String, 
                    hmap_s_s: HashMap<String, String>,
                    ) 
{   
    let ud = access_user_data();
    let user_data_opt = ud.user_data.get(&id);
    
    let app_event = access_events();
    
    let event_present = 
        app_event.events.get(&(id, event_name));
    
    let event = match event_present {
        Some(cb) => cb,
        None => return,
    };
    
    let cb = 
        Python::with_gil(|py| {
            event.clone_ref(py)
        });
    
    drop(app_event);
    
    Python::with_gil(|py| {
            if user_data_opt.is_some() {
                let res = cb.call1(py, (
                                                            id,
                                                            hmap_s_s,
                                                            user_data_opt.unwrap()
                                                            ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("KeyBoard Event: 3 parameters (id, dict, user_data) 
                                        are required or a python error in this function. {er}"),
                }
            } else {
                let res = cb.call1(py, (
                                                                    id,
                                                                    hmap_s_s,  
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Keyboard Event: 2 parameter (id, dict) 
                                        is required or possibly a python error in this function. {er}"),
                }
            } 
    });
    
    drop(ud);      

}

fn process_mouse_callback(id: usize,
                    event_name: String, 
                    hmap_s_f: Option<HashMap<String, f32>>, 
                    ) 
{
    let ud = access_user_data();
    let user_data_opt = ud.user_data.get(&id);
    
    let app_event = access_events();
    
    let event_present = 
        app_event.events.get(&(id, event_name));
    
    let event = match event_present {
        Some(cb) => cb,
        None => return,
    };
    
    let cb = 
        Python::with_gil(|py| {
            event.clone_ref(py)
        });
    
    drop(app_event);

    Python::with_gil(|py| {
            if user_data_opt.is_some() {
                let res = cb.call1(py, (
                                                                    id,
                                                                    hmap_s_f,
                                                                    user_data_opt.unwrap()
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Mouse Event: 3 parameters (id, dict, user_data) 
                                        are required or a python error in this function. {er}"),
                }
            } else {
                let res = cb.call1(py, (
                                                                    id,
                                                                    hmap_s_f,  
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Mouse Event: 2 parameter (id, dict) 
                                        is required or possibly a python error in this function. {er}"),
                }
            } 
    });
    
    drop(ud);

}

fn process_window_callback(id: usize,
                    event_name: String, 
                    hmap_s_f: Option<HashMap<String, f32>>,
                    hmap_s_s: Option<HashMap<String, String>>,
                    ) 
{
    let ud = access_user_data();
    let user_data_opt = ud.user_data.get(&id);
    
    let app_event = access_events();
    
    let event_present = 
        app_event.events.get(&(id, event_name));
    
    let event = match event_present {
        Some(cb) => cb,
        None => return,
    };
    
    let cb = 
        Python::with_gil(|py| {
            event.clone_ref(py)
        });
    
    drop(app_event);

    Python::with_gil(|py| {
            if user_data_opt.is_some() {
                let res = cb.call1(py, (
                                                            id,
                                                            hmap_s_f,
                                                            hmap_s_s,
                                                            user_data_opt.unwrap()
                                                            ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Wndow Event: 3 parameters (id, dict, dict, user_data) 
                                        are required or a python error in this function. {er}"),
                }
            } else {
                let res = cb.call1(py, (
                                                                    id,
                                                                    hmap_s_f,
                                                                    hmap_s_s,  
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Wndow Event: 3 parameter (id, dict, dict) 
                                        is required or possibly a python error in this function. {er}"),
                }
            } 
    });
    
    drop(ud);
}

fn process_touch_callback(id: usize,
                    event_name: String,
                    hmap_s_fg: HashMap<String, u64>,
                    hmap_s_pt: HashMap<String, (f32, f32)>,
                    ) 
{
    let ud = access_user_data();
    let user_data_opt = ud.user_data.get(&id);
    
    let app_event = access_events();
    
    let event_present = 
        app_event.events.get(&(id, event_name));
    
    let event = match event_present {
        Some(cb) => cb,
        None => return,
    };
    
    let cb = 
        Python::with_gil(|py| {
            event.clone_ref(py)
        });
    
    drop(app_event);

    Python::with_gil(|py| {
            if user_data_opt.is_some() {
                let res = cb.call1(py, (
                                                                    id,
                                                                    hmap_s_fg,
                                                                    hmap_s_pt,
                                                                    user_data_opt.unwrap()
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Touch Event: 4 parameters (id, dict, dict, user_data) 
                                        are required or a python error in this function. {er}"),
                }
            } else {
                let res = cb.call1(py, (
                                                                    id,
                                                                    hmap_s_fg,
                                                                    hmap_s_pt,  
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Touch Event: 3 parameter (id, dict, dict) 
                                        is required or possibly a python error in this function. {er}"),
                }
            } 
    });
    
    drop(ud);
    
}
