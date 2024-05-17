
use crate::access_callbacks;
use crate::app;
use super::callbacks::get_set_container_callback_data;
use super::callbacks::WidgetCallbackIn;
use super::callbacks::WidgetCallbackOut;
use super::helpers::get_height;
use super::helpers::get_width;
use super::helpers::try_extract_f64;
use crate::iced_widgets::scrollable::{Alignment, Direction, Properties, Scrollable, Viewport};

use iced::{Element, Length};
use iced::widget::Column;

use pyo3::pyclass;
use pyo3::types::IntoPyDict;
use pyo3::{Python, PyObject};



#[derive(Debug, Clone)]
pub struct IpgScrollable {
    pub id: usize,
    pub width: Length,
    pub height: Length,
    pub direction: IpgScrollableDirection,
    pub h_bar_width: f32,
    pub h_bar_margin: f32,
    pub h_scroller_width: f32,
    pub h_bar_alignment: IpgScrollableAlignment,
    pub v_bar_width: f32,
    pub v_bar_margin: f32,
    pub v_scroller_width: f32,
    pub v_bar_alignment: IpgScrollableAlignment,
    pub user_data: Option<PyObject>,
    // pub style: Default,
}

impl IpgScrollable {
    pub fn new(
        id: usize,
        width: Length,
        height: Length,
        direction: IpgScrollableDirection,
        h_bar_width: f32,
        h_bar_margin: f32,
        h_scroller_width: f32,
        h_bar_alignment: IpgScrollableAlignment,
        v_bar_width: f32,
        v_bar_margin: f32,
        v_scroller_width: f32,
        v_bar_alignment: IpgScrollableAlignment,
        user_data: Option<PyObject>,
        // style: Default::default(),
    ) -> Self {
        Self {
            id,
            width,
            height,
            direction,
            h_bar_width,
            h_bar_margin,
            h_scroller_width,
            h_bar_alignment,
            v_bar_width,
            v_bar_margin,
            v_scroller_width,
            v_bar_alignment,
            user_data,
            // style,
        }
    }
}


#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgScrollableDirection {
    Vertical,
    Horizontal,
    Both,
}


#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgScrollableAlignment {
    Start,
    End,
}


pub fn construct_scrollable(scroll: &IpgScrollable, content: Vec<Element<'static, app::Message>> ) 
                                                            -> Element<'static, app::Message> {

    let content: Element<'static, app::Message> = Column::with_children(content).into();

    let direction = get_direction(scroll.direction.clone(),
                                                        scroll.h_bar_width,
                                                        scroll.h_bar_margin,
                                                        scroll.h_scroller_width,
                                                        scroll.h_bar_alignment.clone(),
                                                        scroll.v_bar_width,
                                                        scroll.v_bar_margin,
                                                        scroll.v_scroller_width,
                                                        scroll.v_bar_alignment.clone()
                                                    );


    Scrollable::with_direction(content, direction)
                    .ipg_id(scroll.id)
                    .width(scroll.width)
                    .height(scroll.height)
                    .on_scroll(app::Message::Scrolled)
                    .into()
    
  
}


fn get_direction(direction: IpgScrollableDirection, 
                    h_width: f32,
                    h_margin: f32,
                    h_scroller_width: f32,
                    h_alignment: IpgScrollableAlignment,
                    v_width: f32,
                    v_margin: f32,
                    v_scroller_width: f32,
                    v_alignment: IpgScrollableAlignment
                ) -> Direction {

    let h_alignment = match h_alignment {
        IpgScrollableAlignment::Start => Alignment::Start,
        IpgScrollableAlignment::End => Alignment::End,
    };

    let v_alignment = match v_alignment {
        IpgScrollableAlignment::Start => Alignment::Start,
        IpgScrollableAlignment::End => Alignment::End,
    };

    let h_properties = Properties::new()
                                    .alignment(h_alignment)
                                    .width(h_width)
                                    .margin(h_margin)
                                    .scroller_width(h_scroller_width);

    let v_properties = Properties::new()
                                    .alignment(v_alignment)
                                    .width(v_width)
                                    .margin(v_margin)
                                    .scroller_width(v_scroller_width);


    match direction {
        IpgScrollableDirection::Vertical => Direction::Vertical(v_properties),
        IpgScrollableDirection::Horizontal => Direction::Horizontal(h_properties),
        IpgScrollableDirection::Both => Direction::Both { vertical: v_properties, 
                            horizontal: h_properties },
    }

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
                                                                    wco.scroll_pos.into_py_dict_bound(py), 
                                                                    user_data
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Scrollable: 3 parameters (id, value, user_data) are required or a python error in this function. {er}"),
                }
            } else {
                let res = callback.call1(py, (
                                                                    wco.id.clone(), 
                                                                    wco.scroll_pos.into_py_dict_bound(py), 
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Scrollable: 2 parameters (id, value,) are required or a python error in this function. {er}"),
                }
            } 
    });

    drop(app_cbs); 

}


#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgScrollableParams {
    Width,
    Height,
    HBarWidth,
    HBarMargin,
    HScrollerWidth,
    HBarAlignment,
    VBarWidth,
    VBarMargin,
    VScrollerWidth,
    VBarAlignment,
}


pub fn scrollable_item_update(scroll: &mut IpgScrollable,
                                item: PyObject,
                                value: PyObject,
                            ) 
{
    let update = try_extract_scrollable_update(item);

    match update {
        IpgScrollableParams::Width => {
            let val = try_extract_f64(value);
            scroll.width = get_width(Some(val as f32), false);
        },
        IpgScrollableParams::Height => {
            let val = try_extract_f64(value);
            scroll.height = get_height(Some(val as f32), false);
        },
        IpgScrollableParams::HBarWidth => {
            scroll.h_bar_width = try_extract_f64(value) as f32;
        },
        IpgScrollableParams::HBarMargin => {
            scroll.h_bar_margin = try_extract_f64(value) as f32;
        },
        IpgScrollableParams::HScrollerWidth => {
            scroll.h_scroller_width = try_extract_f64(value) as f32;
        },
        IpgScrollableParams::HBarAlignment => {
            scroll.h_bar_alignment = try_extract_alignment(value);
        },
        IpgScrollableParams::VBarWidth => {
            scroll.v_bar_width = try_extract_f64(value) as f32;
        },
        IpgScrollableParams::VBarMargin => {
            scroll.v_bar_margin = try_extract_f64(value) as f32;
        },
        IpgScrollableParams::VScrollerWidth => {
            scroll.v_scroller_width = try_extract_f64(value) as f32;
        },
        IpgScrollableParams::VBarAlignment => {
            scroll.v_bar_alignment = try_extract_alignment(value);
        },
    }
}


pub fn try_extract_scrollable_update(update_obj: PyObject) -> IpgScrollableParams {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgScrollableParams>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Scrollable update extraction failed"),
        }
    })
}


pub fn try_extract_alignment(direct_obj: PyObject) -> IpgScrollableAlignment {
    Python::with_gil(|py| {
        let res = direct_obj.extract::<IpgScrollableAlignment>(py);
            
        match res {
            Ok(align) => align,
            Err(_) => panic!("ScrollableAlignment failed to extract."),
        }
    })  
}
