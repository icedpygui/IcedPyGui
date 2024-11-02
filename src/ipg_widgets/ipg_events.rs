//! ipg_events
#![allow(unused_assignments)]

use std::collections::HashMap;

use crate::ipg_widgets::ipg_window::get_ipg_mode;
use crate::{access_callbacks, access_state, access_window_actions, IpgState};

use iced::event::Event;
use iced::keyboard::Event::{KeyPressed, KeyReleased, ModifiersChanged};
use iced::keyboard::{Key, Location, Modifiers};
use iced::mouse::Event::{ButtonPressed, ButtonReleased, CursorEntered, 
                        CursorLeft, CursorMoved, WheelScrolled};
use iced::mouse::Button::{Left, Right, Middle, Back, Forward, Other,};
use iced::mouse::ScrollDelta;

use iced::window;
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

pub fn process_keyboard_events(event: Event, event_id: usize) 
{      
    match event {    
        Event::Keyboard(KeyPressed { key, 
                                    location, 
                                    modifiers, 
                                    text: _ ,
                                    physical_key: _, 
                                    modified_key: _ }) => {
        
            let user_data = get_event_user_data(event_id);

            let event_name = "key pressed".to_string();
            
            let cb = get_callback(event_id, event_name.clone());
            
            let key_str: String = process_key(key.as_ref());
            
            let mod_key = process_modifier(modifiers);

            let location_str: String = process_location(location);

            let hmap_s_s: HashMap<String, String> = 
                HashMap::from([
                    ("key".to_string(), key_str),
                    ("modifier".to_string(), mod_key),
                    ("location".to_string(), location_str)
                ]);

            process_keyboard_callback(event_id, 
                                        cb,
                                        event_name,
                                        hmap_s_s,
                                        user_data, 
                                        );
            
        },
        Event::Keyboard(KeyReleased { key, location, modifiers, }) => {

            let user_data = get_event_user_data(event_id);

            let event_name = "key released".to_string();
            
            let cb = get_callback(event_id, event_name.clone());
            
            let key_str: String = process_key(key.as_ref());

            let mod_key = process_modifier(modifiers);

            let location_str: String = process_location(location);

            let hmap_s_s: HashMap<String, String> = HashMap::from([
                ("key".to_string(), key_str),
                ("modifier".to_string(), mod_key),
                ("location".to_string(), location_str)
            ]);

            process_keyboard_callback(event_id,
                                        cb,
                                        event_name, 
                                        hmap_s_s,
                                        user_data, 
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
    let user_data = get_event_user_data(event_id);
    let mut hmap_s_f: Option<HashMap<String, f32>> = None;
    let mut event_name = "".to_string();
    
    match event {
        Event::Mouse(m_event) => {
            let cb = match m_event {
                CursorEntered => {
                    event_name = "enter window".to_string();
                    get_callback(event_id, event_name.clone())
                },
                CursorLeft => {
                    event_name = "exit window".to_string();
                    get_callback(event_id, event_name.clone())
                },
                CursorMoved { position } => {
                    hmap_s_f = Some(HashMap::from([("x".to_string(), position.x),
                                    ("y".to_string(), position.y)]));
                    event_name = "move".to_string();
                    get_callback(event_id, event_name.clone())
                },
                ButtonPressed(btn) => {
                    match btn {
                        Left => {
                            event_name = "left press".to_string();
                            get_callback(event_id, event_name.clone())
                        },
                        Right => {
                            event_name = "right press".to_string();
                            get_callback(event_id, event_name.clone())
                        },
                        Middle => {
                            event_name = "middle press".to_string();
                            get_callback(event_id, event_name.clone())
                        },
                        Back => {
                            event_name = "back press".to_string();
                            get_callback(event_id, event_name.clone())
                        },
                        Forward => {
                            event_name = "forward press".to_string();
                            get_callback(event_id, event_name.clone())
                        },
                        Other(other) => {
                            hmap_s_f = Some(HashMap::from([("other".to_string(), other as f32)]));
                            event_name = "other press".to_string();
                            get_callback(event_id, event_name.clone())
                        },
                    }
                    },
                    ButtonReleased(btn) => {
                        match btn {
                            Left => {
                                event_name = "left release".to_string();
                                get_callback(event_id, event_name.clone())
                            },
                            Right => {
                                event_name = "right release".to_string();
                                get_callback(event_id, event_name.clone())
                            },
                            Middle => {
                                event_name = "middle release".to_string();
                                get_callback(event_id, event_name.clone())
                            },
                            Back => {
                                event_name = "back release".to_string();
                                get_callback(event_id, event_name.clone())
                            },
                            Forward => {
                                event_name = "forward release".to_string();
                                get_callback(event_id, event_name.clone())
                            },
                            Other(other) => {
                                hmap_s_f = Some(HashMap::from([("other".to_string(), other as f32)]));
                                event_name = "other release".to_string();
                                get_callback(event_id, event_name.clone())
                            },
                        }
                    },
                    WheelScrolled { delta } => {
                        match delta {
                            ScrollDelta::Lines { x, y } => {
                                hmap_s_f = Some(HashMap::from([("x".to_string(), x),
                                                                ("y".to_string(), y)]));
                                event_name = "middle scroll line".to_string();
                                get_callback(event_id, event_name.clone())
                            },
                            ScrollDelta::Pixels { x, y } => {
                                hmap_s_f = Some(HashMap::from([("x".to_string(), x),
                                                                ("y".to_string(), y)]));
                                event_name = "middle scroll pixel".to_string();
                                get_callback(event_id, event_name.clone())
                            },
                        }
                    }
            };
            process_mouse_callback(event_id,
                                    cb,
                                    event_name,
                                    hmap_s_f,
                                    user_data,
                                    )
        },
        Event::Keyboard(_) => (),
        Event::Window(_) => (),
        Event::Touch(_) => (),
    }
        
}

pub fn process_touch_events(event: Event, event_id: usize) {
    let user_data = get_event_user_data(event_id);
    let mut event_name = "".to_string();
    match event {
        Event::Touch(tch) => {
            let (cb, hmap_s_fg, hmap_s_pt) = match tch {
                iced::touch::Event::FingerPressed { id, position } => {
                    let hmap_s_fg = HashMap::from([("finger".to_string(), id.0)]);
                    let hmap_s_pt = HashMap::from([("position".to_string(), (position.x, position.y))]);
                    event_name = "finger pressed".to_string();
                    let cb = get_callback(event_id, event_name.clone());
                    (cb, hmap_s_fg, hmap_s_pt)
                },
                iced::touch::Event::FingerMoved { id, position } => {
                    let hmap_s_fg = HashMap::from([("finger".to_string(), id.0)]);
                    let hmap_s_pt = HashMap::from([("position".to_string(), (position.x, position.y))]);
                    event_name = "finger moved".to_string();
                    let cb = get_callback(event_id, event_name.clone());
                    (cb, hmap_s_fg, hmap_s_pt)
                },
                iced::touch::Event::FingerLifted { id, position } => {
                    let hmap_s_fg = HashMap::from([("finger".to_string(), id.0)]);
                    let hmap_s_pt =HashMap::from([("position".to_string(), (position.x, position.y))]);
                    event_name = "finger lifted".to_string();
                    let cb = get_callback(event_id, event_name.clone());
                    (cb, hmap_s_fg, hmap_s_pt)
                },
                iced::touch::Event::FingerLost { id, position } => {
                    let hmap_s_fg = HashMap::from([("finger".to_string(), id.0)]);
                    let hmap_s_pt = HashMap::from([("position".to_string(), (position.x, position.y))]);
                    event_name = "finger lost".to_string();
                    let cb = get_callback(event_id, event_name.clone());
                    (cb, hmap_s_fg, hmap_s_pt)
                },
            };
            process_touch_callback(event_id,
                                cb,
                                event_name,
                                hmap_s_fg,
                                hmap_s_pt,
                                user_data,
                                )
            
        },
        Event::Window(_) => (),
        Event::Keyboard(_) => (),
        Event::Mouse(_) => (),
    }
}


pub fn process_window_event(state: &mut IpgState,
                            event: Event,
                            event_id: usize,
                            event_enabled: bool,
                            window_id: window::Id
                        ) -> bool
{
    let mut hmap_s_f: Option<HashMap<String, f32>> = None;
    let mut hmap_s_s: Option<HashMap<String, String>> = None;


    let user_data = if event_enabled {
        get_event_user_data(event_id)
    } else {
        None
    };
   
    let ipg_window_id = match state.windows_iced_ipg_ids.get(&window_id) {
        Some(id) => id,
        None => panic!("Process window event: Unable to find the ipg window id using the iced id {:?}.", window_id)
    };
    let ipg_id = ipg_window_id.clone();

    match event {
        Event::Window(event) => {
            if (event == iced::window::Event::Closed && !event_enabled) || 
                (event == iced::window::Event::CloseRequested && !event_enabled) {
                let mut actions = access_window_actions();
                        actions.mode.push((ipg_id, window::Mode::Hidden));
                        drop(actions);
                        
                let is_empty = handle_window_closing(window_id, window::Mode::Hidden);
                return is_empty;
            } else if !event_enabled {
                return false
            }
            let (cb, event_name) = match event {
                // Cannot use window open since a window need to be predefined.
                // Py user will use show and hide to the same effect.
                iced::window::Event::Opened { position: _, size: _ } => {
                    let name = "opened".to_string();
                   (None, name)
                },
                iced::window::Event::Closed => {
                    let is_empty = handle_window_closing(window_id, window::Mode::Hidden);
                    if is_empty {
                        return true;
                    }
                    let name = "closed".to_string();
                    let cb = get_callback(event_id, name.clone());
                    (cb, name)
                },
                iced::window::Event::Moved(point) => {
                    let name = "moved".to_string();
                    let cb = get_callback(event_id, name.clone());
                    hmap_s_f = Some(HashMap::from([
                                                ("x".to_string(), point.x),
                                                ("y".to_string(), point.y),
                                                ]));
                    (cb, name)
                },
                iced::window::Event::Resized (size) => {
                    let name = "resized".to_string();
                    let cb = get_callback(event_id, name.clone());
                    hmap_s_f = Some(HashMap::from([
                                                ("width".to_string(), size.width),
                                                ("height".to_string(), size.height),
                                                ]));
                    (cb, name)
                },
                iced::window::Event::RedrawRequested(_) => {
                    let name = "redraw requested".to_string();
                    let cb = get_callback(event_id, name.clone());
                    (cb, name)
                },
                iced::window::Event::CloseRequested => {
                    //  if callback present, don't close window
                    let name = "close requested".to_string();
                    let cb = get_callback(event_id, name.clone());
                    
                    if cb.is_none() {
 
                        let mut actions = access_window_actions();
                        actions.mode.push((ipg_id, window::Mode::Hidden));
                        drop(actions);
                        let is_empty = handle_window_closing(window_id, window::Mode::Hidden);
                       
                        if is_empty {
                            return true;
                        }
                    }
                    
                    (cb, name)
                    
                },
                iced::window::Event::Focused => {
                    let name = "focused".to_string();
                    let cb = get_callback(event_id, name.clone());
                    (cb, name)
                },
                iced::window::Event::Unfocused => {
                    let name = "unfocused".to_string();
                    let cb = get_callback(event_id, name.clone());
                    (cb, name)
                },
                iced::window::Event::FileHovered(path) => {
                    let name = "file hovered".to_string();
                    let cb = get_callback(event_id, name.clone());
                    hmap_s_s = Some(HashMap::from([
                                                ("file path".to_string(), path.display().to_string()),
                                                ]));
                    (cb, name)
                },
                iced::window::Event::FileDropped(path) => {
                    let name = "file dropped".to_string();
                    let cb = get_callback(event_id, name.clone());
                    hmap_s_s = Some(HashMap::from([
                                                ("file path".to_string(), path.display().to_string()),
                                                ]));
                    (cb, name)
                },
                iced::window::Event::FilesHoveredLeft => {
                    let name = "files hovered left".to_string();
                    let cb = get_callback(event_id, name.clone());
                    (cb, name)
                },
            };

            process_window_callback(ipg_id,
                                    cb,
                                    event_name, 
                                    hmap_s_f,
                                    hmap_s_s,
                                    user_data,
                                    );
        },
        Event::Keyboard(_) => (),
        Event::Mouse(_) => (),
        Event::Touch(_) => (),   
    }
    return false
}

pub fn handle_window_closing(iced_id: window::Id, mode: window::Mode) -> bool {
    let mut state = access_state();

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
        Some(id) => id.clone(),
        None => panic!("Events: handle_window_closing: Unable to find ipg_id based on Iced_id {:?}", iced_id),
    };

    // Needed a clone here because can't borrow again from state below
    let iced_ipg_ids = state.windows_iced_ipg_ids.clone();

    // if any of the remaining windows are visible, then return false
    for (_iced_id, ipg_id) in iced_ipg_ids {

        match state.containers.get_mut(&ipg_id) {
            Some(cnt) => {
                match cnt {
                    super::ipg_enums::IpgContainers::IpgWindow(wnd) => {
                        if wnd.id == ipg_id_found {
                            wnd.mode = get_ipg_mode(mode);
                        }
                    },
                    _ => ()
                }
            },
            None => (),
        }
        
    }
    drop(state);

    return false;

}

fn get_callback(id: usize, event_name: String) -> Option<PyObject> {
    let cbs = access_callbacks();

    let cb_opt = cbs.callback_events.get(&(id, event_name));

    let cb = match cb_opt {
        Some(cb) => Some(cb.clone()),
        None => None
    };
    drop(cbs);
    cb
}

pub fn get_event_user_data(id: usize) -> Option<PyObject> {

    let cb = access_callbacks();

    for data in cb.user_data.iter() {
                    if data.0 == id {
                        let opt_py = data.1.clone();
                        drop(cb);
                        return opt_py
                    }
                }
    panic!("Event user data not found using id {}", id)

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
                    cb: Option<PyObject>,
                    event_name: String, 
                    hmap_s_s: HashMap<String, String>,
                    user_data: Option<PyObject>, 
                    ) 
{
    if cb.is_none() {
        return
    }

    let cb = cb.unwrap();
    Python::with_gil(|py| {

        let dict = hmap_s_s.into_py_dict_bound(py);
        
        let result = match user_data {
            Some(user_data) => {
                cb.call1(py, (id, event_name, dict, user_data))
            },
            None => {
                cb.call1(py, (id, event_name, dict))
            },
        };
        match result {
                Ok(_) => (),
                Err(er) => panic!("Keyboard Event: 3 parameters (id, event name) and 4 if using user_data are required or a python error in this function. {er}"),
            }                   
    });           

}

fn process_mouse_callback(id: usize,
                    cb: Option<PyObject>,
                    event_name: String, 
                    hmap_s_f: Option<HashMap<String, f32>>,
                    user_data: Option<PyObject>, 
                    ) 
{
    if cb.is_none() {
        return
    }
    let cb = cb.unwrap();
    Python::with_gil(|py| {

        if !hmap_s_f.is_some() {
            let result = match user_data {
                Some(user_data) => {
                    cb.call1(py, (id, event_name, user_data))
                },
                None => {
                    cb.call1(py, (id, event_name))
                },
            };
            match result {
                Ok(_) => (),
                Err(er) => panic!("Window Event: 2 parameters (id, event name) and 3 if using user_data are required or a python error in this function. {er}"),
            }
        } else {
            match hmap_s_f {
                Some(sf) => {
                    let dict = sf.into_py_dict_bound(py);
                    let result = match user_data {
                        Some(user_data) => {
                            cb.call1(py, (id, event_name, dict, user_data))
                        },
                        None => {
                            cb.call1(py, (id, event_name, dict))
                        },
                    };
                    match result {
                        Ok(_) => (),
                        Err(er) => panic!("Window Event: 3 parameters (id, event name, dict) and 4 if using user_data are required or a python error in this function. {er}"),
                    }
                },
                None => (),
            }
        }
    });

}

fn process_window_callback(id: usize,
                    cb: Option<PyObject>,
                    event_name: String, 
                    hmap_s_f: Option<HashMap<String, f32>>,
                    hmap_s_s: Option<HashMap<String, String>>,
                    user_data: Option<PyObject>, 
                    ) 
{
    if cb.is_none() {
        return
    }
    let cb = cb.unwrap();

    Python::with_gil(|py| {

        if hmap_s_f.is_none() && hmap_s_s.is_none() {
            let result = match user_data {
                Some(user_data) => {
                    cb.call1(py, (id, event_name, user_data))
                },
                None => {
                    cb.call1(py, (id, event_name))
                },
            };
            match result {
                Ok(_) => (),
                Err(er) => panic!("Window Event: 2 parameters (id, event name) and 3 if using user_data are required or a python error in this function. {er}"),
            }
            return
        }
        
        if hmap_s_f.is_some() {
            match hmap_s_f {
                Some(sf) => {
                    let dict = sf.into_py_dict_bound(py);
                    let result = match user_data {
                        Some(user_data) => {
                            cb.call1(py, (id, event_name, dict, user_data))
                        },
                        None => {
                            cb.call1(py, (id, event_name, dict))
                        },
                    };
                    match result {
                        Ok(_) => (),
                        Err(er) => panic!("Window Event: 3 parameters (id, event name, dict) and 4 if using user_data are required or a python error in this function. {er}"),
                    }
                },
                None => (),
            }
        } else if hmap_s_s.is_some() {
            match hmap_s_s {
                Some(ss) => {
                    let dict = ss.into_py_dict_bound(py);
                    let result = match user_data {
                        Some(user_data) => {
                            cb.call1(py, (id, event_name, dict, user_data))
                        },
                        None => {
                            cb.call1(py, (id, event_name, dict))
                        },
                    };
                    match result {
                        Ok(_) => (),
                        Err(er) => panic!("Window Event: 3 parameters (id, event name, dict) and 4 if using user_data are required or a python error in this function. {er}"),
                    }
                },
                None => (),
            }
        }
            
        
    });

}

fn process_touch_callback(id: usize,
                    cb: Option<PyObject>,
                    event_name: String,
                    hmap_s_fg: HashMap<String, u64>,
                    hmap_s_pt: HashMap<String, (f32, f32)>,
                    user_data: Option<PyObject>, 
                    ) 
{
    if cb.is_none() {
        return
    }
    let cb = cb.unwrap();
    Python::with_gil(|py| {

        let dict1 = hmap_s_fg.into_py_dict_bound(py);
        let dict2 = hmap_s_pt.into_py_dict_bound(py);
        let result = match user_data {
            Some(user_data) => {
                cb.call1(py, (id, event_name, dict1, dict2, user_data))
            },
            None => {
                cb.call1(py, (id, event_name, dict1, dict2))
            },
        };
        match result {
                Ok(_) => (),
                Err(er) => panic!("Touch Event: 4 parameters (id, event name) and 5 if using user_data are required or a python error in this function. {er}"),
            }
    });
    
}
