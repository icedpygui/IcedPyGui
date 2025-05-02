//!ipg_image
#![allow(clippy::enum_variant_names)]
use std::collections::HashMap;

use crate::access_user_data1;
use crate::access_user_data2;
use crate::app;
use crate::access_callbacks;
use super::helpers::{get_height, get_padding_f64, get_width, 
    try_extract_boolean, try_extract_f64, try_extract_string, 
    try_extract_vec_f64};
use super::ipg_mousearea::get_interaction;
use super::ipg_mousearea::IpgMousePointer;
    
use iced::widget::image::FilterMethod;
use iced::{Length, Element, Padding, Point, Radians, Rotation};
use iced::widget::{Container, Image, MouseArea};
use iced::mouse::Interaction;
use iced::advanced::image;

use pyo3::pyclass;
use pyo3::{PyObject, Python};


#[derive(Debug, Clone)]
pub struct IpgImage {
        pub id: usize,
        pub parent_id: String,
        pub image_path: String,
        pub width: Length,
        pub height: Length,
        pub padding: Padding,
        pub content_fit: IpgImageContentFit,
        pub filter_method: IpgImageFilterMethod,
        pub rotation: IpgImageRotation,
        pub rotation_radians: f32,
        pub opacity: f32,
        pub mouse_pointer: Option<IpgMousePointer>,
        pub show: bool,
}

impl IpgImage {
    pub fn new( 
        id: usize,
        parent_id: String,
        image_path: String,
        width: Length,
        height: Length,
        padding: Padding,
        content_fit: IpgImageContentFit,
        filter_method: IpgImageFilterMethod,
        rotation: IpgImageRotation,
        rotation_radians: f32,
        opacity: f32,
        mouse_pointer: Option<IpgMousePointer>,
        show: bool,
        ) -> Self {
        Self {
            id,
            parent_id,
            image_path,
            width,
            height,
            padding,
            content_fit,
            filter_method,
            rotation,
            rotation_radians,
            opacity,
            mouse_pointer,
            show,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ImageMessage {
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
pub enum IpgImageContentFit {
    Contain,
    Cover,
    Fill,
    IpgNone,
    ScaleDown,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgImageFilterMethod {
    Linear,
    Nearest,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgImageRotation {
    Floating,
    Solid,
}

pub fn construct_image(image: &IpgImage) 
                        -> Option<Element<'_, app::Message>> {

    if !image.show {
        return None
    }

    let img: Element<ImageMessage> = Image::<image::Handle>::new(image.image_path.clone())
                                        .content_fit(match_content_fit(image.content_fit.clone()))
                                        .filter_method(match_filter_method(image.filter_method.clone()))
                                        .rotation(match_rotation(image.rotation.clone(), 
                                                Radians::from(image.rotation_radians)))
                                        .opacity(image.opacity)
                                        .into();

    let cont: Element<ImageMessage> = Container::new(img)
                                                .width(image.width)
                                                .height(image.height)
                                                .padding(image.padding)
                                                .into();

    let pointer: Interaction = get_interaction(&image.mouse_pointer);

    let ma: Element<ImageMessage> = 
                MouseArea::new(cont)
                    .on_press(ImageMessage::OnPress)
                    .on_release(ImageMessage::OnRelease)
                    .on_right_press(ImageMessage::OnRightPress)
                    .on_right_release(ImageMessage::OnRightRelease)
                    .on_middle_press(ImageMessage::OnMiddlePress)
                    .on_middle_release(ImageMessage::OnMiddleRelease)
                    .on_enter(ImageMessage::OnEnter)
                    .on_move(ImageMessage::OnMove)
                    .on_exit(ImageMessage::OnExit)
                    .interaction(pointer)
                    .into();

    Some(ma.map(move |message| app::Message::Image(image.id, message)))

}

fn match_content_fit(content: IpgImageContentFit) -> iced::ContentFit {
    match content {
        IpgImageContentFit::Contain => iced::ContentFit::Contain,
        IpgImageContentFit::Cover => iced::ContentFit::Cover,
        IpgImageContentFit::Fill => iced::ContentFit::Fill,
        IpgImageContentFit::IpgNone => iced::ContentFit::None,
        IpgImageContentFit::ScaleDown => iced::ContentFit::ScaleDown,
    }
}

fn match_filter_method(fm: IpgImageFilterMethod) -> FilterMethod {
    match fm {
        IpgImageFilterMethod::Linear => FilterMethod::Linear,
        IpgImageFilterMethod::Nearest => FilterMethod::Nearest,
    }
}

fn match_rotation(rot: IpgImageRotation, radians: Radians) -> Rotation {
    match rot {
        IpgImageRotation::Floating => Rotation::Floating(radians),
        IpgImageRotation::Solid => Rotation::Solid(radians),
    }
}

pub fn image_callback(id: usize, message: ImageMessage) {

    match message {
        ImageMessage::OnPress => {
            process_callback(id, "on_press".to_string(), None);
        },
        ImageMessage::OnRelease => {
            process_callback(id, "on_release".to_string(), None);
        },
        ImageMessage::OnRightPress => {
            process_callback(id, "on_right_press".to_string(), None);
        },
        ImageMessage::OnRightRelease => {
            process_callback(id, "on_right_release".to_string(), None);
        },
        ImageMessage::OnMiddlePress => {
            process_callback(id, "on_middle_press".to_string(), None);
        },
        ImageMessage::OnMiddleRelease => {
            process_callback(id, "on_middle_release".to_string(), None);
        },
        ImageMessage::OnEnter => {
            process_callback(id, "on_enter".to_string(), None);
        },
        ImageMessage::OnMove(point) => {
            let points: Option<HashMap<String, f32>> = Some(HashMap::from([
                ("x".to_string(), point.x),
                ("y".to_string(), point.y)
            ]));
            
            process_callback(id, "on_move".to_string(), points);
        },
        ImageMessage::OnExit => {
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
                Err(err) => panic!("Image callback error with user_data from ud1: {err}")
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
                Err(err) => panic!("Image callback error with user_data from ud2: {err}")
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
                Err(err) => panic!("Image callback error without user_data: {err}")
            }
    });

}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgImageParam {
    Height,
    HeightFill,
    ImagePath,
    Padding,
    Show,
    Width,
    WidthFill,
    RotationRadians,
    Opacity,
}


pub fn image_item_update(img: &mut IpgImage,
                            item: &PyObject,
                            value: &PyObject,
                            )
{
    let update = try_extract_button_update(item);
    let name = "Image".to_string();
    match update {
        IpgImageParam::Height => {
            let val = try_extract_f64(value, name);
            img.height = get_height(Some(val as f32), false);
        },
        IpgImageParam::HeightFill => {
            let val = try_extract_boolean(value, name);
            img.height = get_height(None, val);
        },
        IpgImageParam::ImagePath => {
            img.image_path = try_extract_string(value, name);
        },
        IpgImageParam::Padding => {
            let val = try_extract_vec_f64(value, name);
            img.padding =  get_padding_f64(val);
        },
        IpgImageParam::Show => {
            img.show = try_extract_boolean(value, name);
        },
        IpgImageParam::Width => {
            let val = try_extract_f64(value, name);
            img.width = get_width(Some(val as f32), false);
        },
        IpgImageParam::WidthFill => {
            let val = try_extract_boolean(value, name);
            img.width = get_width(None, val);
        },
        IpgImageParam::RotationRadians => {
            let val = try_extract_f64(value, name);
            img.rotation_radians = val as f32;
        },
        IpgImageParam::Opacity => {
            let val = try_extract_f64(value, name);
            img.opacity = val as f32;
        },
    }
}

pub fn try_extract_button_update(update_obj: &PyObject) -> IpgImageParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgImageParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Image update extraction failed"),
        }
    })
}