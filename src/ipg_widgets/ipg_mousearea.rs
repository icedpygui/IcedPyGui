//! ipg_mousearea
use crate::{access_callbacks, access_user_data1, access_user_data2, IpgState};
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


fn process_callback(
    id: usize, 
    event_name: String, 
    points_opt: Option<(String, f32, String, f32)>) 
{
    let ud1 = access_user_data1();
    let ud_opt = ud1.user_data.get(&id);

    let app_cbs = access_callbacks();
    let callback = match app_cbs.callbacks.get(&(id, event_name)) {
        Some(cb) => cb,
        None => return,
    };

    let cb = Python::with_gil(|py| callback.clone_ref(py));
    drop(app_cbs);

    // Execute the callback with user data from ud1
    if let Some(user_data) = ud_opt {
        Python::with_gil(|py| {
            let res = match points_opt {
                Some(ref points) => cb.call1(py, (id, points.clone(), user_data)),
                None => cb.call1(py, (id, user_data)),
            };

            match res {
                Ok(_) => (),
                Err(err) => panic!("MouseArea callback error with user_data from ud1: {err}")
            }
        });
        drop(ud1); // Drop ud1 after processing
        return;
    }
    drop(ud1); // Drop ud1 if no user data is found

    // Execute the callback with user data from ud2
    let ud2 = access_user_data2();
    
    if let Some(user_data) = ud2.user_data.get(&id) {
        Python::with_gil(|py| {
            let res = match points_opt {
                Some(ref points) => cb.call1(py, (id, points.clone(), user_data)),
                None => cb.call1(py, (id, user_data)),
            };

            match res {
                Ok(_) => (),
                Err(err) => panic!("MouseArea callback error with user_data from ud2: {err}")
            }
        });
        drop(ud2); // Drop ud2 after processing
        return;
    }
    drop(ud2); // Drop ud2 if no user data is found

    // Execute the callback without user data
    Python::with_gil(|py| {
        let res = match points_opt {
                Some(ref points) => cb.call1(py, (id, points.clone())),
                None => cb.call1(py, (id,)),
            };

            match res {
                Ok(_) => (),
                Err(err) => panic!("MouseArea callback error without user_data: {err}")
            }
    });

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