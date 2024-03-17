
use std::collections::HashMap;

use crate::ipg_widgets::ipg_enums::{IpgWidgets, get_set_widget_data};
use crate::app;
use crate::{access_state, access_callbacks};

use iced::{Length, Element, Point};
use iced::widget::{Container, Image, MouseArea};
use iced::mouse::Interaction;
use iced::advanced::image;

use pyo3::{PyObject, Python};

#[derive(Debug, Clone)]
pub struct IpgImage {
        pub id: usize,
        pub image_path: String,
        pub width: Length,
        pub height: Length,
        pub show: bool,
        pub user_data: Option<PyObject>,
        pub cb_on_press: ImageMessage,
        pub cb_on_release: ImageMessage,
        pub cb_on_right_press: ImageMessage,
        pub cb_on_right_release: ImageMessage,
        pub cb_on_middle_press: ImageMessage,
        pub cb_on_middle_release: ImageMessage,
        pub cb_on_enter: ImageMessage,
        pub cb_on_exit: ImageMessage,
}

impl IpgImage {
    pub fn new( 
        id: usize,
        image_path: String,
        width: Length,
        height: Length,
        show: bool,
        user_data: Option<PyObject>,
        cb_on_press: ImageMessage,
        cb_on_release: ImageMessage,
        cb_on_right_press: ImageMessage,
        cb_on_right_release: ImageMessage,
        cb_on_middle_press: ImageMessage,
        cb_on_middle_release: ImageMessage,
        cb_on_enter: ImageMessage,
        cb_on_exit: ImageMessage,
        ) -> Self {
        Self {
            id,
            image_path,
            width,
            height,
            show,
            user_data,
            cb_on_press,
            cb_on_release,
            cb_on_right_press,
            cb_on_right_release,
            cb_on_middle_press,
            cb_on_middle_release,
            cb_on_enter,
            cb_on_exit,
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
    None,
}

pub fn construct_image(image: IpgImage) -> Element<'static, app::Message> {

    let img: Element<ImageMessage> = Image::<image::Handle>::new(image.image_path).into();

    let cont: Element<ImageMessage> = Container::new(img)
                                                .width(image.width)
                                                .height(image.height)
                                                .into();

    let ma: Element<ImageMessage> = 
                MouseArea::new(cont)
                    .on_press(image.cb_on_press)
                    .on_release(image.cb_on_release)
                    .on_right_press(image.cb_on_right_press)
                    .on_right_release(image.cb_on_right_release)
                    .on_middle_press(image.cb_on_middle_press)
                    .on_middle_release(image.cb_on_middle_release)
                    .on_enter(image.cb_on_enter)
                    .on_move(ImageMessage::OnMove)
                    .on_exit(image.cb_on_exit)
                    .interaction(Interaction::Pointer)
                    .into();

    ma.map(move |message| app::Message::Image(image.id, message))

}

pub fn image_update(id: usize, message: ImageMessage) {

    let (_, user_data, _, _,_) = 
                                get_set_widget_data(
                                                    id,
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    );

    match message {
        ImageMessage::OnPress => {
            let event_name = "on_press".to_string();
            process_callback(id, 
                                event_name,
                                None,
                                user_data,
                                Some("on_press".to_string())
                                );
        },
        ImageMessage::OnRelease => {
            let event_name = "on_release".to_string();
            process_callback(id, 
                            event_name,
                            None,
                            user_data,
                            Some("on_release".to_string())
                            );
        },
        ImageMessage::OnRightPress => {
            let event_name = "on_right_press".to_string();
            process_callback(id, 
                            event_name,
                            None,
                            user_data,
                            Some("on_right_press".to_string())
                            );
        },
        ImageMessage::OnRightRelease => {
            let event_name = "on_right_release".to_string();
            process_callback(id, 
                            event_name,
                            None,
                            user_data,
                            Some("on_right_release".to_string())
                            );
        },
        ImageMessage::OnMiddlePress => {
            let event_name = "on_middle_press".to_string();
            process_callback(id, 
                                event_name,
                                None,
                                user_data,
                                Some("on_middle_press".to_string())
                            );
        },
        ImageMessage::OnMiddleRelease => {
            let event_name = "on_middle_release".to_string();
            process_callback(id, 
                                event_name,
                                None,
                                user_data,
                                Some("on_middle_release".to_string())
                            );
        },
        ImageMessage::OnEnter => {
            let event_name = "on_enter".to_string();
            process_callback(id, 
                                event_name,
                                None,
                                user_data,
                                Some("on_enter".to_string())
                            );
        },
        ImageMessage::OnMove(point) => {
            let event_name = "on_move".to_string();
            let mut points: HashMap<String, f32> = HashMap::new();
            points.insert("x".to_string(), point.x);
            points.insert("y".to_string(), point.y);

            process_callback(id, 
                                event_name,
                                Some(points),
                                user_data,
                                Some("on_move".to_string())
                            );
        },
        ImageMessage::OnExit => {
            let event_name = "on_exit".to_string();
            process_callback(id, 
                                event_name,
                                None,
                                user_data,
                                Some("on_exit".to_string())
                            );
        },
        ImageMessage::None => {

        },
    }
}


fn process_callback(id: usize,
                    event_name: String,
                    point: Option<HashMap<String, f32>>, 
                    user_data: Option<PyObject>, 
                    cb_name: Option<String>) 
{
    // TODO: No error for not finding a callback since the on_enter cannot
    // be switched to a None enum as the rest are done when not being used.
    // Will ne to figure out a way to get an error if no cb found unless its
    // on_move.

    if !cb_name.is_some() {return}

    let app_cbs = access_callbacks();

    let mut found_callback = None;

    for callback in app_cbs.callbacks.iter() {

        if id == callback.id && cb_name == callback.name {

            found_callback = match callback.cb.clone() {
                Some(cb) => Some(cb),
                None => panic!("Callback could not be found with id {}", id),
            };
                break;
        }                   
    }

    drop(app_cbs);

    if point.is_some() {

        let points = match point {
            Some(pt) => pt,
            None => panic!("Could not find the Point for Image mouse move")
        };

        match found_callback {

        Some(cb) => Python::with_gil(|py| {
                    match user_data {
                        Some(ud) => cb.call1(py, 
                                                        (id.clone(),
                                                        event_name,
                                                        points, 
                                                        ud,
                                                        )).unwrap(),
                        None => cb.call1(py, 
                                        (id.clone(), 
                                                event_name,
                                                points,
                                            )).unwrap(),
                    }
                }),
        None => return,
        };
    } else {
        match found_callback {

            Some(cb) => Python::with_gil(|py| {
                        match user_data {
                            Some(ud) => cb.call1(py, 
                                                            (id.clone(),
                                                            event_name, 
                                                            ud,
                                                            )).unwrap(),
                            None => cb.call1(py, 
                                            (id.clone(), 
                                                    event_name
                                                )).unwrap(),
                        }
                    }),
            None => return,
            };
    }

}
