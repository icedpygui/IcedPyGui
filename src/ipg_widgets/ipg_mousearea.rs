//! ipg_mousearea
use crate::{access_callbacks, IpgState};
use crate::app::Message;
use super::callbacks::{container_callback_data, WidgetCallbackIn, WidgetCallbackOut};
use super::helpers::try_extract_boolean;

use iced::widget::MouseArea;
use iced::{Element, Point};
use iced::widget::Column;
use iced::mouse::Interaction;

use pyo3::pyclass;
use pyo3::types::IntoPyDict;
use pyo3::{PyObject, Python};


#[derive(Debug, Clone)]
pub struct IpgMouseArea {
        pub id: usize,
        pub mouse_pointer: Option<IpgMousePointer>,
        pub show: bool,
        pub user_data: Option<PyObject>,
}

impl IpgMouseArea {
    pub fn new( 
        id: usize,
        mouse_pointer: Option<IpgMousePointer>,
        show: bool,
        user_data: Option<PyObject>,
        ) -> Self {
        Self {
            id,
            mouse_pointer,
            show,
            user_data,
        }
    }
}


#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgMousePointer {
    None,
    Idle,
    Pointer,
    Grab,
    Text,
    Crosshair,
    Working,
    Grabbing,
    ResizingHorizontally,
    ResizingVertically,
    NotAllowed,
    ZoomIn,
}

pub fn construct_mousearea(m_area: IpgMouseArea, content: Vec<Element<Message>>) -> Element<Message> {

    let pointer: Interaction = get_interaction(m_area.mouse_pointer);

    let cont: Element<Message> = Column::with_children(content).into();
    // Had to use the Message because the content already has Message.  Typical problem
    // with containers that are also like widgets with Message.
    let ma: Element<Message> = 
                    MouseArea::new(cont)
                    .on_press(Message::MouseAreaOnPress(m_area.id))
                    .on_release(Message::MouseAreaOnRelease(m_area.id))
                    .on_right_press(Message::MouseAreaOnRightPress(m_area.id))
                    .on_right_release(Message::MouseAreaOnRightRelease(m_area.id))
                    .on_middle_press(Message::MouseAreaOnMiddlePress(m_area.id))
                    .on_middle_release(Message::MouseAreaOnMiddleRelease(m_area.id))
                    .on_enter(Message::MouseAreaOnEnter(m_area.id))
                    .on_move(move|p| Message::MouseAreaOnMove(p, m_area.id))
                    .on_exit(Message::MouseAreaOnExit(m_area.id))
                    .interaction(pointer)
                    .into();

    ma

}

pub fn get_interaction(pointer: Option<IpgMousePointer>) -> Interaction {
    if pointer.is_none() {
        return Interaction::None
    }

    match pointer.unwrap() {
        IpgMousePointer::None => Interaction::None,
        IpgMousePointer::Idle => Interaction::Idle,
        IpgMousePointer::Pointer => Interaction::Pointer,
        IpgMousePointer::Grab => Interaction::Grab,
        IpgMousePointer::Text => Interaction::Text,
        IpgMousePointer::Crosshair => Interaction::Crosshair,
        IpgMousePointer::Working => Interaction::Working,
        IpgMousePointer::Grabbing => Interaction::Grabbing,
        IpgMousePointer::ResizingHorizontally => Interaction::ResizingHorizontally,
        IpgMousePointer::ResizingVertically => Interaction::ResizingVertically,
        IpgMousePointer::NotAllowed => Interaction::NotAllowed,
        IpgMousePointer::ZoomIn => Interaction::ZoomIn,
    }
}

pub fn mousearea_callback(state: &mut IpgState, id: usize, event_name: String) {
    
    let wci: WidgetCallbackIn = WidgetCallbackIn{id, ..Default::default()};

    let mut wco = container_callback_data(state, wci);
    wco.id = id;
    wco.event_name = event_name;
    process_callback(wco);

}

pub fn mousearea_callback_point(state: &mut IpgState, 
                                id: usize, 
                                point: Point, 
                                event_name: String,
                                ) {

    let points: Vec<(String, f32)> = vec![
        ("x".to_string(), point.x),
        ("y".to_string(), point.y)];
    
    let wci: WidgetCallbackIn = WidgetCallbackIn{id, ..Default::default()};

    let mut wco = container_callback_data(state, wci);
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
              
    if wco.event_name == *"on_move" {

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
                                                                    wco.id, 
                                                                    points.into_py_dict_bound(py), 
                                                                    user_data
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("MouseArea: 3 parameter (id, points, user_data) are required or a python error in this function. {er}"),
                }
            } else {
                let res = callback.call1(py, (
                                                                    wco.id, 
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
                                                                    wco.id, 
                                                                    user_data
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("MouseArea: 2 parameter (id, user_data) are required or a python error in this function. {er}"),
                }
            } else {
                let res = callback.call1(py, (
                                                                    wco.id,  
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
pub enum IpgMouseAreaParam {
    Show,
}


pub fn mousearea_item_update(img: &mut IpgMouseArea,
                                item: PyObject,
                                value: PyObject,
                                )
{

    let update = try_extract_mousearea_update(item);
    let name = "MouseArea".to_string();
    match update {
        IpgMouseAreaParam::Show => {
            img.show = try_extract_boolean(value, name);
        },
    }
}

pub fn try_extract_mousearea_update(update_obj: PyObject) -> IpgMouseAreaParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgMouseAreaParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("MouseArea update extraction failed"),
        }
    })
}