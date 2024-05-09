
use crate::access_callbacks;
use crate::app::Message;
use super::callbacks::{WidgetCallbackIn, 
                        WidgetCallbackOut, 
                        get_set_container_callback_data};
use super::helpers::try_extract_boolean;

use crate::iced_widgets::mousearea::{MouseArea, PointId};

use iced::Element;
use iced::widget::Column;
use iced::mouse::Interaction;

use pyo3::pyclass;
use pyo3::types::IntoPyDict;
use pyo3::{PyObject, Python};


#[derive(Debug, Clone)]
pub struct IpgMouseArea {
        pub id: usize,
        pub show: bool,
        pub user_data: Option<PyObject>,
}

impl IpgMouseArea {
    pub fn new( 
        id: usize,
        show: bool,
        user_data: Option<PyObject>,
        ) -> Self {
        Self {
            id,
            show,
            user_data,
        }
    }
}

pub fn construct_mousearea(m_area: &IpgMouseArea, content: Vec<Element<'static, Message>>) -> Element<'static, Message> {

    let cont: Element<Message> = Column::with_children(content).into();

    let ma: Element<Message> = 
                    MouseArea::new(m_area.id, cont)
                    .on_press(Message::MouseAreaOnPress(m_area.id))
                    .on_release(Message::MouseAreaOnRelease(m_area.id))
                    .on_right_press(Message::MouseAreaOnRightPress(m_area.id))
                    .on_right_release(Message::MouseAreaOnRightRelease(m_area.id))
                    .on_middle_press(Message::MouseAreaOnMiddlePress(m_area.id))
                    .on_middle_release(Message::MouseAreaOnMiddleRelease(m_area.id))
                    .on_enter(Message::MouseAreaOnEnter(m_area.id))
                    .on_move(Message::MouseAreaOnMove)
                    .on_exit(Message::MouseAreaOnExit(m_area.id))
                    .interaction(Interaction::Pointer)
                    .into();

    ma

}

pub fn mousearea_callback(id: usize, event_name: String) {
    
    let mut wci: WidgetCallbackIn = WidgetCallbackIn::default();
    wci.id = id;

    let mut wco = get_set_container_callback_data(wci);
    wco.id = id;
    wco.event_name = event_name;
    process_callback(wco);

}

pub fn mousearea_callback_pointid(pointid: PointId, event_name: String) {

    let mut points: Vec<(String, f32)> = vec![];
    points.push(("x".to_string(), pointid.x));
    points.push(("y".to_string(), pointid.y));
    let id = pointid.id;
    
    let mut wci: WidgetCallbackIn = WidgetCallbackIn::default();
    wci.id = id;

    let mut wco = get_set_container_callback_data(wci);
    wco.id = id;
    wco.event_name = event_name;
    wco.points = Some(points);
    process_callback(wco);
    
}


fn process_callback(wco: WidgetCallbackOut) 
{
    let app_cbs = access_callbacks();

    let callback_present = app_cbs.callbacks.get(&(wco.id, wco.event_name.clone()));

    let callback_opt = match callback_present {
        Some(cb) => cb,
        None => return,
    };
       
    let callback = match callback_opt {
        Some(cb) => cb,
        None => panic!("MouseArea Callback could not be found with id {}", wco.id),
    };
              
    if wco.event_name == "on_move".to_string() {

        let points = match wco.points {
            Some(pts) => pts,
            None => panic!("MouseArea Points not found"),
        };

        Python::with_gil(|py| {
            if wco.user_data.is_some() {
                let user_data = match wco.user_data {
                    Some(ud) => ud,
                    None => panic!("MouseArea callback user_data not found."),
                };
                let res = callback.call1(py, (
                                                                    wco.id.clone(), 
                                                                    points.into_py_dict_bound(py), 
                                                                    user_data
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("MouseArea: 3 parameter (id, points, user_data) are required or a python error in this function. {er}"),
                }
            } else {
                let res = callback.call1(py, (
                                                                    wco.id.clone(), 
                                                                    points.into_py_dict_bound(py), 
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("MouseArea: 2 parameter (id, points) are required or a python error in this function. {er}"),
                }
            } 
        });

    } else {
        Python::with_gil(|py| {
            if wco.user_data.is_some() {
                let user_data = match wco.user_data {
                    Some(ud) => ud,
                    None => panic!("MouseArea callback user_data not found."),
                };
                let res = callback.call1(py, (
                                                                    wco.id.clone(), 
                                                                    user_data
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("MouseArea: 2 parameter (id, user_data) are required or a python error in this function. {er}"),
                }
            } else {
                let res = callback.call1(py, (
                                                                    wco.id.clone(),  
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("MouseArea: Only 1 parameter (id) is required or a python error in this function. {er}"),
                }
            } 
        });
    }
    
    drop(app_cbs);   

}


#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgMouseAreaParams {
    Show,
}


pub fn mousearea_item_update(img: &mut IpgMouseArea,
                                item: PyObject,
                                value: PyObject,
                                )
{

    let update = try_extract_MouseArea_update(item);

    match update {
        IpgMouseAreaParams::Show => {
            img.show = try_extract_boolean(value);
        },
    }
}

pub fn try_extract_MouseArea_update(update_obj: PyObject) -> IpgMouseAreaParams {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgMouseAreaParams>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("MouseArea update extraction failed"),
        }
    })
}