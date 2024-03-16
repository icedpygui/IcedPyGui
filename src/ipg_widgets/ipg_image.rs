
use crate::ipg_widgets::ipg_enums::{IpgWidgets, get_set_widget_data};
use crate::app;
use crate::{access_state, access_callbacks};

use iced::{Length, Element};
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
        pub cb_on_press: Option<String>,
        pub cb_on_release: Option<String>,
        pub cb_on_right_press: Option<String>,
        pub cb_on_right_release: Option<String>,
        pub cb_on_middle_press: Option<String>,
        pub cb_on_middle_release: Option<String>,
        pub callback_made: bool,
}

impl IpgImage {
    pub fn new( 
        id: usize,
        image_path: String,
        width: Length,
        height: Length,
        show: bool,
        user_data: Option<PyObject>,
        cb_on_press: Option<String>,
        cb_on_release: Option<String>,
        cb_on_right_press: Option<String>,
        cb_on_right_release: Option<String>,
        cb_on_middle_press: Option<String>,
        cb_on_middle_release: Option<String>,
        callback_made: bool,
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
            callback_made,
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
}

pub fn construct_image(image: IpgImage) -> Element<'static, app::Message> {
    
    let img: Element<ImageMessage> = Image::<image::Handle>::new(image.image_path).into();

    let cont: Element<ImageMessage> = Container::new(img)
                                                .width(image.width)
                                                .height(image.height)
                                                .into();

    let ma: Element<ImageMessage> = 
                MouseArea::new(cont)
                    .on_press(ImageMessage::OnPress)
                    .on_release(ImageMessage::OnRelease)
                    .on_right_press(ImageMessage::OnRightPress)
                    .on_right_release(ImageMessage::OnRightRelease)
                    .on_middle_press(ImageMessage::OnMiddlePress)
                    .on_middle_release(ImageMessage::OnMiddleRelease)
                    .interaction(Interaction::Pointer)
                    .into();

    ma.map(move |message| app::Message::Image(image.id, message))

}

pub fn image_update(id: usize, message: ImageMessage) {

    let (_, user_data, _, _,cb_made) = 
                                get_set_widget_data(
                                                    id,
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    );

    // Since Iced responds to messages regardless if the user sets thenm or not
    // then each widget has a variable for callback_made which will return if none set.
    match cb_made {
        Some(false) => return,
        Some(true) => (),
        None => return,
    }
    
    match message {
        ImageMessage::OnPress => {
            let event_name = "on_press".to_string();
            process_callback(id, 
                                event_name,
                                user_data,
                                Some("cb_on_press".to_string())
                                );
        },
        ImageMessage::OnRelease => {
            let event_name = "on_release".to_string();
            process_callback(id, 
                                event_name,
                                user_data,
                                Some("cb_on_release".to_string())
                            );
        },
        ImageMessage::OnRightPress => {
            let event_name = "on_right_press".to_string();
            process_callback(id, event_name,
                                user_data,
                                Some("cb_on_right_press".to_string())
                            );
        },
        ImageMessage::OnRightRelease => {
            let event_name = "on_right_release".to_string();
            process_callback(id, 
                                event_name,
                                user_data,
                                Some("cb_on_right_release".to_string())
                            );
        },
        ImageMessage::OnMiddlePress => {
            let event_name = "on_middle_press".to_string();
            process_callback(id, 
                                event_name,
                                user_data,
                                Some("cb_on_middle_press".to_string())
                            );
        },
        ImageMessage::OnMiddleRelease => {
            let event_name = "on_middle_release".to_string();
            process_callback(id, 
                                event_name,
                                user_data,
                                Some("cb_on_middle_release".to_string())
                            );
        }
    }
}


fn process_callback(id: usize,
                    event_name: String, 
                    user_data: Option<PyObject>, 
                    cb_name: Option<String>) 
{
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
    None => panic!("Image callback could not be found"),
    };

}
