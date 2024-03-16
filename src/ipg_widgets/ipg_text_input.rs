#![allow(unused)]
use crate::{access_state, access_callbacks};
use crate::app;
use crate::ipg_widgets::ipg_enums::{IpgWidgets, get_set_widget_data};

use iced::widget::text::LineHeight;
use iced::{Padding, Length, Element};
use iced::widget::TextInput;

use pyo3::{PyObject, Python};

#[derive(Debug, Clone)]
pub struct IpgTextInput {
    pub id: usize,
    pub placeholder: String,
    pub value: String,
    pub is_secure: bool,
    // font: Option<Font>,
    pub width: Length,
    pub padding: Padding,
    pub size: f32,
    pub line_height: LineHeight,
    pub user_data: Option<PyObject>,
    // icon: Option<Message>,
    // style: Style,
    pub cb_name_input: Option<String>,
    pub cb_name_submit: Option<String>,
    pub cb_name_paste: Option<String>,
    show: bool,
}

impl IpgTextInput {
    pub fn new( 
        id: usize,
        placeholder: String,
        is_secure: bool,
        // font: Option<Font>,
        width: Length,
        padding: Padding,
        size: f32,
        line_height: LineHeight,
        user_data: Option<PyObject>,
        // icon: Option<Message>,
        // style: Style
        cb_name_input: Option<String>,
        cb_name_submit: Option<String>,
        cb_name_paste: Option<String>,
        show: bool,
        ) -> Self {
        Self {
            id,
            placeholder,
            value: "".to_string(),
            is_secure,
            // font,
            width,
            padding,
            size,
            line_height,
            user_data,
            // icon,
            // style: Style,
            cb_name_input,
            cb_name_submit,
            cb_name_paste,
            show,
        }
    }
}

#[derive(Debug, Clone)]
pub enum TIMessage {
    OnInput(String),
    OnSubmitted(String),
    // OnPasted,
}

pub fn construct_text_input(input: IpgTextInput) -> Element<'static, app::Message> {
    
    let txt: Element<TIMessage> =  TextInput::new(input.placeholder.as_str(), 
                                                input.value.as_str()
                                            )
                                            .on_input(TIMessage::OnInput)
                                            .on_submit(TIMessage::OnSubmitted(input.value))
                                            // .on_paste(TIMessage::OnPasted)
                                            .width(input.width)
                                            .padding(input.padding)
                                            .size(input.size)
                                            .line_height(input.line_height)
                                            .into();

    txt.map(move |message| app::Message::TextInput(input.id, message))
}

pub fn text_input_update(id: usize, message: TIMessage) {

    match message {
        TIMessage::OnInput(value) => {
            setup_for_callback(id, value, "on_input".to_string());
        },
        TIMessage::OnSubmitted(value) => {
            setup_for_callback(id, value, "submitted".to_string());
        }
            
    }
}

fn setup_for_callback(id: usize, value: String, name: String) {
    // During the input, the widget is assigned the value so that it shows
    // during typing.  On submit, the text box is cleared, so no value.
    // However, in both cases the value is passed to the callback.
    let mut val  = "".to_string();
    let mut event_name = "on_submit".to_string();

    if name == "on_input".to_string() {
        val = value.clone();
        event_name = "on_input".to_string();
    }

    let (cb_name, user_data,_,_,_) 
                                    = get_set_widget_data(
                                                            id, 
                                                            None, 
                                                            Some(val), 
                                                            None, 
                                                            None,
                                                            );
    process_callback(id.clone(),
                        event_name,
                        value.clone(),   
                        user_data, 
                        cb_name);

}


fn process_callback(id: usize,
                    event_name: String,
                    value: String, 
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
    };
    drop(app_cbs);

    match found_callback {

    Some(cb) => Python::with_gil(|py| {
                            match user_data {
                                Some(ud) => cb.call1(py, 
                                                                (id.clone(),
                                                                        event_name,
                                                                        value, 
                                                                        ud)).unwrap(),
                                None => cb.call1(py, 
                                                (id.clone(), 
                                                        event_name,
                                                        value,
                                                        )).unwrap(),
                            }
                        }),
    None => panic!("TextInput callback could not be found"),
    };

}                   
