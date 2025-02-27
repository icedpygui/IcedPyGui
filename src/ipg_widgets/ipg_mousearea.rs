//! ipg_mousearea
use crate::{access_callbacks, IpgState};
use crate::app::Message;
use super::helpers::try_extract_boolean;

use iced::widget::MouseArea;
use iced::{Element, Point};
use iced::widget::Column;
use iced::mouse::Interaction;

use pyo3::pyclass;
use pyo3::{PyObject, Python};


#[derive(Debug, Clone)]
pub struct IpgMouseArea {
        pub id: usize,
        pub mouse_pointer: Option<IpgMousePointer>,
        pub show: bool,
}

impl IpgMouseArea {
    pub fn new( 
        id: usize,
        mouse_pointer: Option<IpgMousePointer>,
        show: bool,
        ) -> Self {
        Self {
            id,
            mouse_pointer,
            show,
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
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

pub fn construct_mousearea<'a>(m_area: &'a IpgMouseArea, 
                            content: Vec<Element<'a, Message>>) 
                            -> Element<'a, Message> {

    let pointer: Interaction = get_interaction(&m_area.mouse_pointer);

    let cont: Element<Message> = Column::with_children(content).into();

    // Had to use the Message because the content already has Message.  Typical problem
    // with containers that are also like widgets with Message.
    
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
        .into()
}

pub fn get_interaction(pointer: &Option<IpgMousePointer>) -> Interaction {
    if pointer.is_none() {
        return Interaction::None
    }

    match pointer.clone().unwrap() {
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

pub fn mousearea_callback(_state: &mut IpgState, id: usize, event_name: String) {
    
    process_callback(id, event_name, None);

}

pub fn mousearea_callback_point(_state: &mut IpgState, 
                                id: usize, 
                                point: Point, 
                                event_name: String,
                                ) {

    let points: Option<(String, f32, String, f32)> = Some(
                ("x".to_string(), point.x,
                "y".to_string(), point.y));

    process_callback(id, event_name, points);
}


fn process_callback(id: usize, event_name: String, points_opt: Option<(String, f32, String, f32)>) 
{
    let app_cbs = access_callbacks();

    let callback_present = app_cbs.callbacks.get(&(id, event_name));

    let callback_opt = match callback_present {
        Some(cb) => cb,
        None => return,
    };
       
    let callback = match callback_opt {
        Some(cb) => cb,
        None => panic!("Image Callback could not be found with id {}", id),
    };
               
    let user_data_opt = app_cbs.user_data.get(&id);

    Python::with_gil(|py| {
        if user_data_opt.is_some() && points_opt.is_some() {
                let res = callback.call1(py, (
                                                                    id, 
                                                                    points_opt.unwrap(), 
                                                                    user_data_opt.unwrap()
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Image: 3 parameter (id, points, user_data) are required or a python error in this function. {er}"),
                }
            } else if points_opt.is_some() && user_data_opt.is_none() {
                let res = callback.call1(py, (
                                                                    id, 
                                                                    points_opt.unwrap(), 
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Image: 2 parameter (id, points) are required or a python error in this function. {er}"),
                }
            } else if user_data_opt.is_some() {
                let res = callback.call1(py, (
                                                                    id, 
                                                                    user_data_opt.unwrap()
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Image: 2 parameter (id, user_data) are required or a python error in this function. {er}"),
                }
            
            } else {
                let res = callback.call1(py, (
                                                                    id, 
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Image: 1 parameter (id) are required or a python error in this function. {er}"),
                }
            }
    
    });

    drop(app_cbs);   

}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgMouseAreaParam {
    Show,
}


pub fn mousearea_item_update(img: &mut IpgMouseArea,
                                item: &PyObject,
                                value: &PyObject,
                                )
{

    let update = try_extract_mousearea_update(item);

    match update {
        IpgMouseAreaParam::Show => {
            img.show = try_extract_boolean(value, "MouseArea".to_string());
        },
    }
}

pub fn try_extract_mousearea_update(update_obj: &PyObject) -> IpgMouseAreaParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgMouseAreaParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("MouseArea update extraction failed"),
        }
    })
}