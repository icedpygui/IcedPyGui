//!ipg_image
#![allow(clippy::enum_variant_names)]
use crate::app;
use crate::access_callbacks;
use crate::IpgState;
use super::callbacks::set_or_get_widget_callback_data;
use super::callbacks::{WidgetCallbackIn, 
                        WidgetCallbackOut};
use super::helpers::{get_height, get_padding_f64, get_width, 
    try_extract_boolean, try_extract_f64, try_extract_string, 
    try_extract_vec_f64};
use super::ipg_mousearea::get_interaction;
use super::ipg_mousearea::IpgMousePointer;
    
use iced::widget::image::FilterMethod;
use iced::widget::Space;
use iced::{Length, Element, Padding, Point, Radians, Rotation};
use iced::widget::{Container, Image, MouseArea};
use iced::mouse::Interaction;
use iced::advanced::image;

use pyo3::pyclass;
use pyo3::types::IntoPyDict;
use pyo3::{PyObject, Python};


#[derive(Debug, Clone)]
pub struct IpgImage {
        pub id: usize,
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
        pub user_data: Option<PyObject>,
}

impl IpgImage {
    pub fn new( 
        id: usize,
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
        user_data: Option<PyObject>,
        ) -> Self {
        Self {
            id,
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
            user_data,
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

#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgImageContentFit {
    Contain,
    Cover,
    Fill,
    IpgNone,
    ScaleDown,
}

#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgImageFilterMethod {
    Linear,
    Nearest,
}

#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgImageRotation {
    Floating,
    Solid,
}

pub fn construct_image(image: IpgImage) -> Element<'static, app::Message> {

    if !image.show {
        return Space::new(0.0, 0.0).into()
    }

    let img: Element<ImageMessage> = Image::<image::Handle>::new(image.image_path)
                                        .content_fit(match_content_fit(image.content_fit))
                                        .filter_method(match_filter_method(image.filter_method))
                                        .rotation(match_rotation(image.rotation, Radians::from(image.rotation_radians)))
                                        .opacity(image.opacity)
                                        .into();

    let cont: Element<ImageMessage> = Container::new(img)
                                                .width(image.width)
                                                .height(image.height)
                                                .padding(image.padding)
                                                .into();

    let pointer: Interaction = get_interaction(image.mouse_pointer);

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

    ma.map(move |message| app::Message::Image(image.id, message))

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

pub fn image_callback(state: &mut IpgState, id: usize, message: ImageMessage) {

    let wci: WidgetCallbackIn = WidgetCallbackIn{id, ..Default::default()};

    match message {
        ImageMessage::OnPress => {
            let mut wco = set_or_get_widget_callback_data(state, wci);
            wco.id = id;
            wco.event_name = "on_press".to_string();
            process_callback(wco);
        },
        ImageMessage::OnRelease => {
            let mut wco = set_or_get_widget_callback_data(state, wci);
            wco.id = id;
            wco.event_name = "on_release".to_string();
            process_callback(wco);
        },
        ImageMessage::OnRightPress => {
            let mut wco = set_or_get_widget_callback_data(state, wci);
            wco.id = id;
            wco.event_name = "on_right_press".to_string();
            process_callback(wco);
        },
        ImageMessage::OnRightRelease => {
            let mut wco = set_or_get_widget_callback_data(state, wci);
            wco.id = id;
            wco.event_name = "on_right_release".to_string();
            process_callback(wco);
        },
        ImageMessage::OnMiddlePress => {
            let mut wco = set_or_get_widget_callback_data(state, wci);
            wco.id = id;
            wco.event_name = "on_middle_press".to_string();
            process_callback(wco);
        },
        ImageMessage::OnMiddleRelease => {
            let mut wco = set_or_get_widget_callback_data(state, wci);
            wco.id = id;
            wco.event_name = "on_middle_release".to_string();
            process_callback(wco);
        },
        ImageMessage::OnEnter => {
            let mut wco = set_or_get_widget_callback_data(state, wci);
            wco.id = id;
            wco.event_name = "on_enter".to_string();
            process_callback(wco);
        },
        ImageMessage::OnMove(point) => {
            let points: Vec<(String, f32)> = vec![
                ("x".to_string(), point.x),
                ("y".to_string(), point.y)];
            
            let mut wco = set_or_get_widget_callback_data(state, wci);
            wco.id = id;
            wco.event_name = "on_move".to_string();
            wco.points = Some(points);
            process_callback(wco);
        },
        ImageMessage::OnExit => {
            let mut wco = set_or_get_widget_callback_data(state, wci);
            wco.id = id;
            wco.event_name = "on_exit".to_string();
            process_callback(wco);
        },
    }
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
        None => panic!("Image Callback could not be found with id {}", wco.id),
    };
              
    if wco.event_name == *"on_move" {

        let points = match wco.points {
            Some(pts) => pts,
            None => panic!("Image Points not found"),
        };

        Python::with_gil(|py| {
            if wco.user_data.is_some() {
                let user_data = match wco.user_data {
                    Some(ud) => ud,
                    None => panic!("Image callback user_data not found."),
                };
                let res = callback.call1(py, (
                                                                    wco.id, 
                                                                    points.into_py_dict_bound(py), 
                                                                    user_data
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Image: 3 parameter (id, points, user_data) are required or a python error in this function. {er}"),
                }
            } else {
                let res = callback.call1(py, (
                                                                    wco.id, 
                                                                    points.into_py_dict_bound(py), 
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Image: 2 parameter (id, points) are required or a python error in this function. {er}"),
                }
            } 
        });

    } else {
        Python::with_gil(|py| {
            if wco.user_data.is_some() {
                let user_data = match wco.user_data {
                    Some(ud) => ud,
                    None => panic!("Image callback user_data not found."),
                };
                let res = callback.call1(py, (
                                                                    wco.id, 
                                                                    user_data
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Image: 2 parameter (id, user_data) are required or a python error in this function. {er}"),
                }
            } else {
                let res = callback.call1(py, (
                                                                    wco.id,  
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Image: Only 1 parameter (id) is required or a python error in this function. {er}"),
                }
            } 
        });
    }
    
    drop(app_cbs);   

}


#[derive(Debug, Clone)]
#[pyclass]
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
                            item: PyObject,
                            value: PyObject,
                            )
{

    let update = try_extract_button_update(item);

    match update {
        IpgImageParam::Height => {
            let val = try_extract_f64(value);
            img.height = get_height(Some(val as f32), false);
        },
        IpgImageParam::HeightFill => {
            let val = try_extract_boolean(value);
            img.height = get_height(None, val);
        },
        IpgImageParam::ImagePath => {
            img.image_path = try_extract_string(value);
        },
        IpgImageParam::Padding => {
            let val = try_extract_vec_f64(value);
            img.padding =  get_padding_f64(val);
        },
        IpgImageParam::Show => {
            img.show = try_extract_boolean(value);
        },
        IpgImageParam::Width => {
            let val = try_extract_f64(value);
            img.width = get_width(Some(val as f32), false);
        },
        IpgImageParam::WidthFill => {
            let val = try_extract_boolean(value);
            img.width = get_width(None, val);
        },
        IpgImageParam::RotationRadians => {
            let val = try_extract_f64(value);
            img.rotation_radians = val as f32;
        },
        IpgImageParam::Opacity => {
            let val = try_extract_f64(value);
            img.opacity = val as f32;
        },
    }
}

pub fn try_extract_button_update(update_obj: PyObject) -> IpgImageParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgImageParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Image update extraction failed"),
        }
    })
}