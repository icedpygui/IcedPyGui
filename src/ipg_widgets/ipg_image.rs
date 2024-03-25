
use crate::app;
use crate::access_callbacks;
use super::callbacks::{WidgetCallbackIn, 
                        WidgetCallbackOut, 
                        get_set_widget_callback_data};
use iced::{Length, Element, Padding, Point};
use iced::widget::{Container, Image, MouseArea};
use iced::mouse::Interaction;
use iced::advanced::image;

use pyo3::types::IntoPyDict;
use pyo3::{PyObject, Python};


#[derive(Debug, Clone)]
pub struct IpgImage {
        pub id: usize,
        pub image_path: String,
        pub width: Length,
        pub height: Length,
        pub padding: Padding,
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
        show: bool,
        user_data: Option<PyObject>,
        ) -> Self {
        Self {
            id,
            image_path,
            width,
            height,
            padding,
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

pub fn construct_image(image: IpgImage) -> Element<'static, app::Message> {

    let img: Element<ImageMessage> = Image::<image::Handle>::new(image.image_path).into();

    let cont: Element<ImageMessage> = Container::new(img)
                                                .width(image.width)
                                                .height(image.height)
                                                .padding(image.padding)
                                                .into();

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
                    .interaction(Interaction::Pointer)
                    .into();

    ma.map(move |message| app::Message::Image(image.id, message))

}

pub fn image_callback(id: usize, message: ImageMessage) {

    let mut wci: WidgetCallbackIn = WidgetCallbackIn::default();
    wci.id = id;

    match message {
        ImageMessage::OnPress => {
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = "on_press".to_string();
            process_callback(wco);
        },
        ImageMessage::OnRelease => {
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = "on_release".to_string();
            process_callback(wco);
        },
        ImageMessage::OnRightPress => {
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = "on_right_press".to_string();
            process_callback(wco);
        },
        ImageMessage::OnRightRelease => {
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = "on_right_release".to_string();
            process_callback(wco);
        },
        ImageMessage::OnMiddlePress => {
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = "on_middle_press".to_string();
            process_callback(wco);
        },
        ImageMessage::OnMiddleRelease => {
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = "on_middle_release".to_string();
            process_callback(wco);
        },
        ImageMessage::OnEnter => {
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = "on_enter".to_string();
            process_callback(wco);
        },
        ImageMessage::OnMove(point) => {
            let mut points: Vec<(String, f32)> = vec![];
            points.push(("x".to_string(), point.x));
            points.push(("y".to_string(), point.y));
            
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = "on_move".to_string();
            wco.points = Some(points);
            process_callback(wco);
        },
        ImageMessage::OnExit => {
            let mut wco = get_set_widget_callback_data(wci);
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
              
    if wco.event_name == "on_move".to_string() {

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
                                                                    wco.id.clone(), 
                                                                    points.into_py_dict(py), 
                                                                    user_data
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(_) => panic!("Image: 3 parameter (id, points, user_data) are required or possibly a non-fatal python error in this function."),
                }
            } else {
                let res = callback.call1(py, (
                                                                    wco.id.clone(), 
                                                                    points.into_py_dict(py), 
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(_) => panic!("Image: 2 parameter (id, points) are required or possibly a non-fatal python error in this function."),
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
                                                                    wco.id.clone(), 
                                                                    user_data
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(_) => panic!("Image: 2 parameter (id, user_data) are required or possibly a non-fatal python error in this function."),
                }
            } else {
                let res = callback.call1(py, (
                                                                    wco.id.clone(),  
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(_) => panic!("Image: Only 1 parameter (id) is required or possibly a non-fatal python error in this function."),
                }
            } 
        });
    }
    
    drop(app_cbs);   

}


pub enum IpgImageUpdate {
    Height,
    HeightFill,
    ImagePath,
    Padding,
    Show,
    Width,
    WidthFill,
}

pub fn image_item_update((img: &mut IpgImage,
                            item: PyObject,
                            value: PyObject,
                            )) 
{

    let update = try_extract_button_update(item);

    match update {
        IpgImageUpdate::Height -> {
            img.
        }
    }
}

pub fn try_extract_button_update(update_obj: PyObject) -> IpgImageUpdate {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgImageUpdate>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Image update extraction failed"),
        }
    })
}