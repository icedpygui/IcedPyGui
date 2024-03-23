
use crate::access_callbacks;
use crate::app;
use super::callbacks::get_set_container_callback_data;
use super::callbacks::WidgetCallbackIn;
use super::callbacks::WidgetCallbackOut;
use crate::iced_widgets::scrollable::{Direction, Scrollable, Viewport};

use iced::{Element, Length};
use iced::widget::Column;

use pyo3::types::IntoPyDict;
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

    let mut offsets: Vec<(String, f32)> = vec![];
    offsets.push(("abs_offset_x".to_string(), vp.absolute_offset().x));
    offsets.push(("abs_offset_y".to_string(), vp.absolute_offset().y));
    offsets.push(("rel_offset_x".to_string(), vp.relative_offset().x));
    offsets.push(("rel_offset_y".to_string(), vp.relative_offset().y));
    offsets.push(("rev_offset_x".to_string(), vp.absolute_offset_reversed().x));
    offsets.push(("rev_offset_y".to_string(), vp.absolute_offset_reversed().y));
    
    let mut wci: WidgetCallbackIn = WidgetCallbackIn::default();
    wci.id = id;
    
    let mut wco = get_set_container_callback_data(wci);
    wco.id = id;
    wco.scroll_pos = offsets;
    wco.event_name = "on_scroll".to_string();
    process_callback(wco);
}


pub fn process_callback(wco: WidgetCallbackOut) 
{
    let app_cbs = access_callbacks();

    let callback_present = app_cbs.callbacks.get(&(wco.id, wco.event_name.clone()));
       
    let callback_opt = match callback_present {
        Some(cb) => cb,
        None => return,
    };

    let callback = match callback_opt {
        Some(cb) => cb,
        None => panic!("Scrollable Callback could not be found with id {}", wco.id),
    };
                  
    Python::with_gil(|py| {
            if wco.user_data.is_some() {
                let user_data = match wco.user_data {
                    Some(ud) => ud,
                    None => panic!("Scrollable callback user_data not found."),
                };
                let res = callback.call1(py, (
                                                                    wco.id.clone(), 
                                                                    wco.scroll_pos.into_py_dict(py), 
                                                                    user_data
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(_) => panic!("Scrollable: 3 parameters (id, value, user_data) are required or possibly a non-fatal python error in this function."),
                }
            } else {
                let res = callback.call1(py, (
                                                                    wco.id.clone(), 
                                                                    wco.scroll_pos.into_py_dict(py), 
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(_) => panic!("Scrollable: 2 parameters (id, value,) are required or possibly a non-fatal python error in this function."),
                }
            } 
    });

    drop(app_cbs); 

}
