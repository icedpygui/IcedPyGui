
use crate::ipg_widgets::ipg_enums::{IpgWidgets, get_set_widget_data};
use crate::app;
use crate::{access_state, access_callbacks};

use iced::{Length, Element};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::text::{LineHeight, Shaping};
use iced::widget::{MouseArea, Text};
use iced::mouse::Interaction;

use pyo3::{PyObject, Python};

#[derive(Debug, Clone)]
pub struct IpgSelectableText {
        pub id: usize,
        pub content: String,
        pub width: Length,
        pub height: Length,
        pub horizontal_alignment: Horizontal,
        pub vertical_alignment: Vertical,
        pub line_height: LineHeight,
        pub size: f32,
        pub show: bool,
        // pub font: Font,
        pub shaping: Shaping,
        // pub style: Style,
        pub user_data: Option<PyObject>,
        pub cb_on_press: Option<String>,
        pub cb_on_release: Option<String>,
        pub cb_on_right_press: Option<String>,
        pub cb_on_right_release: Option<String>,
        pub cb_on_middle_press: Option<String>,
        pub cb_on_middle_release: Option<String>,
}

impl IpgSelectableText {
    pub fn new( 
        id: usize,
        content: String,
        width: Length,
        height: Length,
        horizontal_alignment: Horizontal,
        vertical_alignment: Vertical,
        line_height: LineHeight,
        size: f32,
        show: bool,
        // font: Font,
        shaping: Shaping,
        // style: Style,
        user_data: Option<PyObject>,
        cb_on_press: Option<String>,
        cb_on_release: Option<String>,
        cb_on_right_press: Option<String>,
        cb_on_right_release: Option<String>,
        cb_on_middle_press: Option<String>,
        cb_on_middle_release: Option<String>,
        ) -> Self {
        Self {
            id,
            content,
            width,
            height,
            horizontal_alignment,
            vertical_alignment,
            line_height,
            size,
            show,
            // font: Font,
            shaping,
            // style: Style,
            user_data,
            cb_on_press,
            cb_on_release,
            cb_on_right_press,
            cb_on_right_release,
            cb_on_middle_press,
            cb_on_middle_release,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SLTXTMessage {
    OnPress,
    OnRelease,
    OnRightPress,
    OnRightRelease,
    OnMiddlePress,
    OnMiddleRelease,
}

pub fn construct_selectable_text(sl_text: IpgSelectableText) -> Element<'static, app::Message> {
    
    let content: Element<'_, SLTXTMessage> = 
                        Text::new(sl_text.content.clone())
                            .size(sl_text.size)
                            .line_height(sl_text.line_height)
                            .width(sl_text.width)
                            .height(sl_text.height)
                            .horizontal_alignment(sl_text.horizontal_alignment)
                            .vertical_alignment(sl_text.vertical_alignment)
                            // font: Font,
                            .shaping(sl_text.shaping)
                            // style: Style,
                            .into();

    let ma: Element<'_, SLTXTMessage> = 
                MouseArea::new(content)
                    .on_press(SLTXTMessage::OnPress)
                    .on_release(SLTXTMessage::OnRelease)
                    .on_right_press(SLTXTMessage::OnRightPress)
                    .on_right_release(SLTXTMessage::OnRightRelease)
                    .on_middle_press(SLTXTMessage::OnMiddlePress)
                    .on_middle_release(SLTXTMessage::OnMiddleRelease)
                    .interaction(Interaction::Pointer)
                    .into();

    ma.map(move |message| app::Message::SelectableText(sl_text.id, message))

}

pub fn selectable_text_update(id: usize, message: SLTXTMessage) {

    let (_, user_data, _, _) = 
                                get_set_widget_data(
                                                    id,
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    );

    match message {
        SLTXTMessage::OnPress => {
            let event_name = "on_press".to_string();
            process_callback(id, 
                                event_name,
                                user_data,
                                Some("cb_on_press".to_string())
                                );
        },
        SLTXTMessage::OnRelease => {
            let event_name = "on_release".to_string();
            process_callback(id, 
                                event_name,
                                user_data,
                                Some("cb_on_release".to_string())
                            );
        },
        SLTXTMessage::OnRightPress => {
            let event_name = "on_right_press".to_string();
            process_callback(id, event_name,
                                user_data,
                                Some("cb_on_right_press".to_string())
                            );
        },
        SLTXTMessage::OnRightRelease => {
            let event_name = "on_right_release".to_string();
            process_callback(id, 
                                event_name,
                                user_data,
                                Some("cb_on_right_release".to_string())
                            );
        },
        SLTXTMessage::OnMiddlePress => {
            let event_name = "on_middle_press".to_string();
            process_callback(id, 
                                event_name,
                                user_data,
                                Some("cb_on_middle_press".to_string())
                            );
        },
        SLTXTMessage::OnMiddleRelease => {
            let event_name = "on_middle_release".to_string();
            process_callback(id, 
                                event_name,
                                user_data,
                                Some("cb_on_middle_release".to_string())
                            );
        },
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
            None => {drop(app_cbs); panic!("Callback could not be found with id {}", id)},
        };
            break;
        }                   
    };

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
    None => panic!("Selectable text callback could not be found"),
    };

}
