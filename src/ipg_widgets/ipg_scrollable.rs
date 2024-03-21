
use std::collections::HashMap;

use crate::access_callbacks;
use crate::app;
use crate::access_state;
use crate::iced_widgets::scrollable::{Direction, Scrollable, Viewport};


use iced::{Element, Length};
use iced::widget::Column;

use pyo3::{Python, PyObject};

use super::ipg_enums::IpgContainers;

#[derive(Debug, Clone)]
pub struct IpgScrollable {
    pub id: usize,
    pub width: Length,
    pub height: Length,
    pub direction: Direction,
    pub user_data: Option<PyObject>,
    // pub style: Default,
}

impl IpgScrollable {
    pub fn new(
        id: usize,
        width: Length,
        height: Length,
        direction: Direction,
        user_data: Option<PyObject>,
        // style: Default::default(),
    ) -> Self {
        Self {
            id,
            width,
            height,
            direction,
            user_data,
            // style,
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

pub fn scrollable_callback(id: usize, vp: Viewport) {

    let mut offsets: HashMap<String, f32> = HashMap::new();
    offsets.insert("abs_offset_x".to_string(), vp.absolute_offset().x);
    offsets.insert("abs_offset_y".to_string(), vp.absolute_offset().y);
    offsets.insert("rel_offset_x".to_string(), vp.relative_offset().x);
    offsets.insert("rel_offset_y".to_string(), vp.relative_offset().y);
    offsets.insert("rev_offset_x".to_string(), vp.absolute_offset_reversed().x);
    offsets.insert("rev_offset_y".to_string(), vp.absolute_offset_reversed().y);
    
    let user_data = get_set_data(id);

    let event_name = "on_scroll".to_string();

    process_callback(id, 
                        event_name, 
                        offsets,
                        user_data,
                    );
}

fn get_set_data(id: usize) -> Option<PyObject> {
    let state = access_state();

    let container_type_opt = state.containers.get(&id);

    let container_type = match container_type_opt {
        Some(cont) => cont,
        None => panic!("Container with id {id} could not be found"),
    };
    
    match container_type {
        IpgContainers::IpgScrollable(scroll) => {
            let user_data = scroll.user_data.clone();
            drop(state);
            return user_data
        }
        IpgContainers::IpgColumn(_) => return None,
        IpgContainers::IpgContainer(_) => return None,
        IpgContainers::IpgPaneGrid(_) => return None,
        IpgContainers::IpgPane(_) => return None,
        IpgContainers::IpgRow(_) => return None,
        IpgContainers::IpgToolTip(_) => return None,
        IpgContainers::IpgWindow(_) => return None,
    }
        
}

fn process_callback(id: usize, 
                    event_name: String,
                    offsets: HashMap<String, f32>,
                    _user_data: Option<PyObject>) 
{
    let app_cbs = access_callbacks();

    let mut found_callback = None;

    for callback in app_cbs.callbacks.iter() {

    if id == callback.id && event_name == callback.event_name.clone() {

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

