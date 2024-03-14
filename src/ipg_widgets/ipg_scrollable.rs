#![allow(unused)]
use std::collections::HashMap;

use crate::access_callbacks;
use crate::app;
use crate::access_state;
use crate::ipg_widgets::ipg_enums::{IpgContainers, IpgWidgets};
use crate::iced_widgets::scrollable::{Direction, Scrollable, Viewport};


use iced::{Element, Length};
use iced::widget::Column;

use pyo3::{Python, PyObject};

#[derive(Debug, Clone)]
pub struct IpgScrollable {
    pub id: usize,
    pub width: Length,
    pub height: Length,
    pub direction: Direction,
    pub user_data: Option<PyObject>,
    // pub style: Default,
    pub cb_name: Option<String>,
}

impl IpgScrollable {
    pub fn new(
        id: usize,
        width: Length,
        height: Length,
        direction: Direction,
        user_data: Option<PyObject>,
        // style: Default::default(),
        cb_name: Option<String>,
    ) -> Self {
        Self {
            id,
            width,
            height,
            direction,
            user_data,
            // style,
            cb_name,
        }
    }
}


pub fn construct_scrollable(scroll: &IpgScrollable, content: Vec<Element<'static, app::Message>> ) 
                                                            -> Element<'static, app::Message> {

    let content: Element<'static, app::Message> = Column::with_children(content).into();

    Scrollable::new(content)
                    .ipg_id(scroll.id)
                    .width(scroll.width)
                    .height(scroll.height)
                    .direction(scroll.direction)
                    .on_scroll(app::Message::Scrolled)
                    .into()
  
}

pub fn scrollable_update(id: usize, vp: Viewport) {

    let mut offsets: HashMap<String, f32> = HashMap::new();
    offsets.insert("abs_offset_x".to_string(), vp.absolute_offset().x);
    offsets.insert("abs_offset_y".to_string(), vp.absolute_offset().y);
    offsets.insert("rel_offset_x".to_string(), vp.relative_offset().x);
    offsets.insert("rel_offset_y".to_string(), vp.relative_offset().y);
    offsets.insert("rev_offset_x".to_string(), vp.absolute_offset_reversed().x);
    offsets.insert("rev_offset_y".to_string(), vp.absolute_offset_reversed().y);
    
    let (user_data, cb_name) = get_set_data(id);

    let event_name = "Scrolled".to_string();

    process_callback(id, 
                        event_name, 
                        offsets,
                        cb_name
                    );
}

fn get_set_data(id: usize) -> (Option<PyObject>, Option<String>) {
    let state = access_state();

    let container_type_opt = state.containers.get(&id);

    let container_type = match container_type_opt {
        Some(cont) => cont,
        None => panic!("Container with id {id} could not be found"),
    };
    
    match container_type {
        IpgContainers::IpgScrollable(scroll) => {
            let user_data = scroll.user_data.clone();
            let cb_name = scroll.cb_name.clone();
            drop(state);
            return (user_data, cb_name) 
        }
        IpgContainers::IpgColumn(_) => return (None, None),
        IpgContainers::IpgContainer(_) => return (None, None),
        IpgContainers::IpgPaneGrid(_) => return (None, None),
        IpgContainers::IpgPane(_) => return (None, None),
        IpgContainers::IpgRow(_) => return (None, None),
        IpgContainers::IpgToolTip(_) => return (None, None),
        IpgContainers::IpgWindow(_) => return (None, None),
    }
        
}

fn process_callback(id: usize, 
                    event_name: String,
                    offsets: HashMap<String, f32>,
                    cb_name: Option<String>) 
{

    if !cb_name.is_some() {return};

    let app_cbs = access_callbacks();

    let mut found_callback = None;

    for callback in app_cbs.callbacks.iter() {

    if id == callback.id && cb_name == callback.name {

        found_callback = match callback.cb.clone() {
                            Some(cb) => Some(cb),
                            None => {
                                panic!("Callback could not be found with id {}", id)
                            },
                        };
        break;
        }                   
    };
    drop(app_cbs);

    match found_callback {

        Some(cb) => Python::with_gil(|py| {

            cb.call1(py, (id.clone(), 
                            event_name,
                            offsets,  
                            )
                        ).unwrap();          
        }),
        None => panic!("Scrollable callback not found"),
    };

}

