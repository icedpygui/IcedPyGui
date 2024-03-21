
use crate::app;
use crate::access_callbacks;
use super::callbacks::{WidgetCallbackIn, 
                        WidgetCallbackOut, 
                        get_set_widget_callback_data};
use iced::{Length, Element, Padding, Point};
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
            wco.event_name = Some("on_press".to_string());
            process_callback(wco);
        },
        ImageMessage::OnRelease => {
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = Some("on_release".to_string());
            process_callback(wco);
        },
        ImageMessage::OnRightPress => {
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = Some("on_right_press".to_string());
            process_callback(wco);
        },
        ImageMessage::OnRightRelease => {
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = Some("on_right_release".to_string());
            process_callback(wco);
        },
        ImageMessage::OnMiddlePress => {
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = Some("on_middle_press".to_string());
            process_callback(wco);
        },
        ImageMessage::OnMiddleRelease => {
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = Some("on_middle_release".to_string());
            process_callback(wco);
        },
        ImageMessage::OnEnter => {
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = Some("on_enter".to_string());
            process_callback(wco);
        },
        ImageMessage::OnMove(point) => {
            wci.point = Some(point);
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = Some("on_move".to_string());
            process_callback(wco);
        },
        ImageMessage::OnExit => {
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = Some("on_exit".to_string());
            process_callback(wco);
        },
    }
}


fn process_callback(wco: WidgetCallbackOut) 
{
    // TODO: No error for not finding a callback since the on_enter cannot
    // be switched to a None enum as the rest are done when not being used.
    // Will ne to figure out a way to get an error if no cb found unless its
    // on_move.

    if !wco.event_name.is_some() {return}

    let app_cbs = access_callbacks();

    let mut found_callback = None;

    for callback in app_cbs.callbacks.iter() {

        if wco.id == callback.id && wco.event_name == Some(callback.event_name.clone()) {

            found_callback = match callback.cb.clone() {
                Some(cb) => Some(cb),
                None => panic!("Callback could not be found with id {}", wco.id),
            };
                break;
        }                   
    }

    drop(app_cbs);

    if wco.points.is_some() {

        let points = match wco.points {
            Some(pt) => pt,
            None => panic!("Could not find the Point for Image mouse move")
        };

        match found_callback {

        Some(cb) => Python::with_gil(|py| {
                    match wco.user_data {
                        Some(ud) => cb.call1(py, 
                                                        (
                                                            wco.id.clone(),
                                                            wco.event_name,
                                                            points, 
                                                            ud,
                                                            )).unwrap(),
                        None => cb.call1(py, 
                                        (
                                                wco.id.clone(), 
                                                wco.event_name,
                                                points,
                                            )).unwrap(),
                    }
                }),
        None => return,
        };
    } else {
        match found_callback {

            Some(cb) => Python::with_gil(|py| {
                        match wco.user_data {
                            Some(ud) => cb.call1(py, 
                                                            (
                                                                wco.id.clone(),
                                                                wco.event_name, 
                                                                ud,
                                                            )).unwrap(),
                            None => cb.call1(py, 
                                            (
                                                    wco.id.clone(), 
                                                    wco.event_name
                                                )).unwrap(),
                        }
                    }),
            None => return,
            };
    }

}
