
use std::collections::HashMap;

use crate::access_callbacks;
use crate::app;
use super::callbacks::get_set_container_callback_data;
use super::callbacks::WidgetCallbackIn;
use super::callbacks::WidgetCallbackOut;
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

    let mut wci = WidgetCallbackIn::default();
    wci.id = id;

    let mut offsets: HashMap<String, f32> = HashMap::new();
    offsets.insert("abs_offset_x".to_string(), vp.absolute_offset().x);
    offsets.insert("abs_offset_y".to_string(), vp.absolute_offset().y);
    offsets.insert("rel_offset_x".to_string(), vp.relative_offset().x);
    offsets.insert("rel_offset_y".to_string(), vp.relative_offset().y);
    offsets.insert("rev_offset_x".to_string(), vp.absolute_offset_reversed().x);
    offsets.insert("rev_offset_y".to_string(), vp.absolute_offset_reversed().y);
    
    let mut wci: WidgetCallbackIn = WidgetCallbackIn::default();
    wci.id = id;
    
    let mut wco = get_set_container_callback_data(wci);
    wco.id = id;
    wco.scroll_pos = Some(offsets);
    wco.event_name = Some("on_scroll".to_string());
    process_callback(wco);

}


pub fn process_callback(wco: WidgetCallbackOut) 
{
    if !wco.event_name.is_some() {return}

    let evt_name = match wco.event_name {
        Some(name) => name,
        None => panic!("event_name not found")
    };

    let app_cbs = access_callbacks();

    let callback_opt = app_cbs.callbacks.get(&(wco.id, evt_name.clone())).unwrap();
       
    let callback = match callback_opt {
        Some(cb) => cb,
        None => panic!("Callback could not be found with id {}", wco.id),
    };
                  
    Python::with_gil(|py| {
            if wco.user_data.is_some() {
                callback.call1(py, (
                                        wco.id.clone(), 
                                        evt_name.clone(),
                                        wco.scroll_pos, 
                                        wco.user_data
                                        )
                                ).unwrap();
            } else {
                callback.call1(py, (
                                        wco.id.clone(), 
                                        evt_name.clone(),
                                        wco.scroll_pos, 
                                        )
                                ).unwrap();
            } 
    });

    drop(app_cbs); 

}
