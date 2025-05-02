//! ipg_svg
#![allow(clippy::enum_variant_names)]
use std::collections::HashMap;

use crate::access_user_data1;
use crate::access_user_data2;
use crate::app;
use crate::access_callbacks;
use crate::IpgState;
use super::helpers::{get_height, get_width};
use super::helpers::{try_extract_boolean, try_extract_f64, 
    try_extract_string};
use super::ipg_mousearea::get_interaction;
use super::ipg_mousearea::IpgMousePointer;

use iced::{Length, Element, Point, Radians, Rotation};
use iced::widget::{Svg, MouseArea};
use iced::mouse::Interaction;
use iced::advanced::svg;

use pyo3::pyclass;
use pyo3::{PyObject, Python};


#[derive(Debug, Clone)]
pub struct IpgSvg {
        pub id: usize,
        pub parent_id: String,
        pub svg_path: String,
        pub width: Length,
        pub height: Length,
        pub content_fit: IpgSvgContentFit,
        pub rotation: IpgSvgRotation,
        pub rotation_radians: f32,
        pub opacity: f32,
        pub mouse_pointer: Option<IpgMousePointer>,
        pub show: bool,
}

impl IpgSvg {
    pub fn new( 
        id: usize,
        parent_id: String,
        svg_path: String,
        width: Length,
        height: Length,
        content_fit: IpgSvgContentFit,
        rotation: IpgSvgRotation,
        rotation_radians: f32,
        opacity: f32,
        mouse_pointer: Option<IpgMousePointer>,
        show: bool,
        ) -> Self {
        Self {
            id,
            parent_id,
            svg_path,
            width,
            height,
            content_fit,
            rotation,
            rotation_radians,
            opacity,
            mouse_pointer,
            show,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SvgMessage {
    OnPress,
    OnRelease,
    OnRightPress,
    OnRightRelease,
    OnMiddlePress,
    OnMiddleRelease,
    OnEnter,
    OnMove(Point),
    OnExit,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgSvgContentFit {
    Contain,
    Cover,
    Fill,
    IpgNone,
    ScaleDown,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgSvgRotation {
    Floating,
    Solid,
}

pub fn construct_svg(sg: &IpgSvg) 
                    -> Option<Element<'_, app::Message>> {

    if !sg.show {
        return None
    }

    let svg_handle = svg::Handle::from_path(sg.svg_path.clone());

    let svg_widget: Element<SvgMessage> = Svg::new(svg_handle)
                                                .width(sg.width)
                                                .height(sg.height)
                                                .content_fit(match_content_fit(sg.content_fit.clone()))
                                                .rotation(match_rotation(sg.rotation.clone(), Radians::from(sg.rotation_radians)))
                                                .opacity(sg.opacity)
                                                .into();

    let pointer: Interaction = get_interaction(&sg.mouse_pointer.clone());

    let widget: Element<SvgMessage> = 
                MouseArea::new(svg_widget)
                    .on_press(SvgMessage::OnPress)
                    .on_release(SvgMessage::OnRelease)
                    .on_right_press(SvgMessage::OnRightPress)
                    .on_right_release(SvgMessage::OnRightRelease)
                    .on_middle_press(SvgMessage::OnMiddlePress)
                    .on_middle_release(SvgMessage::OnMiddleRelease)
                    .on_enter(SvgMessage::OnEnter)
                    .on_move(SvgMessage::OnMove)
                    .on_exit(SvgMessage::OnExit)
                    //Need to add in the other Interactions
                    .interaction(pointer)
                    .into();

    Some(widget.map(move |message| app::Message::Svg(sg.id, message)))

}

fn match_content_fit(content: IpgSvgContentFit) -> iced::ContentFit {
    match content {
        IpgSvgContentFit::Contain => iced::ContentFit::Contain,
        IpgSvgContentFit::Cover => iced::ContentFit::Cover,
        IpgSvgContentFit::Fill => iced::ContentFit::Fill,
        IpgSvgContentFit::IpgNone => iced::ContentFit::None,
        IpgSvgContentFit::ScaleDown => iced::ContentFit::ScaleDown,
    }
}

fn match_rotation(rot: IpgSvgRotation, radians: Radians) -> Rotation {
    match rot {
        IpgSvgRotation::Floating => Rotation::Floating(radians),
        IpgSvgRotation::Solid => Rotation::Solid(radians),
    }
}

pub fn svg_callback(_state: &mut IpgState, id: usize, message: SvgMessage) {

    match message {
        SvgMessage::OnPress => {
            process_callback(id, "on_press".to_string(), None);
        },
        SvgMessage::OnRelease => {
            process_callback(id, "on_release".to_string(), None);
        },
        SvgMessage::OnRightPress => {
            process_callback(id, "on_right_press".to_string(), None);
        },
        SvgMessage::OnRightRelease => {
            process_callback(id, "on_right_release".to_string(), None);
        },
        SvgMessage::OnMiddlePress => {
            process_callback(id, "on_middle_press".to_string(), None);
        },
        SvgMessage::OnMiddleRelease => {
            process_callback(id, "on_middle_release".to_string(), None);
        },
        SvgMessage::OnEnter => {
            process_callback(id, "on_enter".to_string(), None);
        },
        SvgMessage::OnMove(point) => {
            let points: Option<HashMap<String, f32>> = Some(HashMap::from([
                ("x".to_string(), point.x),
                ("y".to_string(), point.y)
            ]));
            
            process_callback(id, "on_move".to_string(), points);
        },
        SvgMessage::OnExit => {
            process_callback(id, "on_exit".to_string(), None);
        },
    }
}


fn process_callback(
    id: usize,
    event_name: String,
    points_opt: Option<HashMap<String, f32>>,
) {
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
                Err(err) => panic!("SVG callback error with user_data from ud1: {err}")
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
                Err(err) => panic!("SVG callback error with user_data from ud2: {err}")
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
                Err(err) => panic!("SVG callback error without user_data: {err}")
            }
    });

}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgSvgParam {
    Height,
    HeightFill,
    ImagePath,
    Show,
    Width,
    WidthFill,
    RotationRadians,
    Opacity,
}


pub fn svg_item_update(img: &mut IpgSvg,
                            item: &PyObject,
                            value: &PyObject,
                            )
{
    let update = try_extract_svg_update(item);
    let name = "Svg".to_string();
    match update {
        IpgSvgParam::Height => {
            let val = try_extract_f64(value, name);
            img.height = get_height(Some(val as f32), false);
        },
        IpgSvgParam::HeightFill => {
            let val = try_extract_boolean(value, name);
            img.height = get_height(None, val);
        },
        IpgSvgParam::ImagePath => {
            img.svg_path = try_extract_string(value, name);
        },
        IpgSvgParam::Show => {
            img.show = try_extract_boolean(value, name);
        },
        IpgSvgParam::Width => {
            let val = try_extract_f64(value, name);
            img.width = get_width(Some(val as f32), false);
        },
        IpgSvgParam::WidthFill => {
            let val = try_extract_boolean(value, name);
            img.width = get_width(None, val);
        },
        IpgSvgParam::RotationRadians => {
            let val = try_extract_f64(value, name);
            img.rotation_radians = val as f32;
        },
        IpgSvgParam::Opacity => {
            let val = try_extract_f64(value, name);
            img.opacity = val as f32;
        },
    }
}

pub fn try_extract_svg_update(update_obj: &PyObject) -> IpgSvgParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgSvgParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Svg update extraction failed"),
        }
    })
}
