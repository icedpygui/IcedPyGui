
use crate::access_callbacks;
use crate::app;
use super::callbacks::{WidgetCallbackIn, 
                        WidgetCallbackOut, 
                        get_set_widget_callback_data};

use iced::widget::text::LineHeight;
use iced::{Padding, Length, Element};
use iced::widget::{TextInput, Space};

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
            show,
        }
    }
}

#[derive(Debug, Clone)]
pub enum TIMessage {
    OnInput(String),
    OnSubmit(String),
    OnPast(String),
}

pub fn construct_text_input(input: IpgTextInput) -> Element<'static, app::Message> {

    if !input.show {
        return Space::new(0.0, 0.0).into()
    }
    
    let txt: Element<TIMessage> =  TextInput::new(input.placeholder.as_str(), 
                                                input.value.as_str()
                                            )
                                            .on_input(TIMessage::OnInput)
                                            .on_submit(TIMessage::OnSubmit(input.value))
                                            .on_paste(TIMessage::OnPast)
                                            .width(input.width)
                                            .padding(input.padding)
                                            .size(input.size)
                                            .line_height(input.line_height)
                                            .into();

    txt.map(move |message| app::Message::TextInput(input.id, message))
}

pub fn text_input_callback(id: usize, message: TIMessage) {
    // During the input, the widget is assigned the value so that it shows
    // during typing.  On submit, the text box is cleared, so no value.
    // However, in both cases the value is passed to the callback.
    let mut wci: WidgetCallbackIn = WidgetCallbackIn::default();
    wci.id = id;
           
    match message {
        TIMessage::OnInput(value) => {
            wci.value_str = Some(value);
            let mut wco: WidgetCallbackOut = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = "on_input".to_string();
            process_callback(wco);
        },
        TIMessage::OnSubmit(value) => {
            wci.value_str = Some(value);
            let mut wco: WidgetCallbackOut = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = "on_submit".to_string();
            process_callback(wco);
        }
        TIMessage::OnPast(value) => {
            wci.value_str = Some(value);
            let mut wco: WidgetCallbackOut = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = "on_paste".to_string();
            process_callback(wco);
        }
            
    }
}

pub fn process_callback(wco: WidgetCallbackOut) 
{
    let app_cbs = access_callbacks();

    let callback_present = app_cbs.callbacks.get(&(wco.id, wco.event_name.clone()));

    let callback_opt = match callback_present {
        Some(cb) => cb,
        None => return,
    };

    let callback = match callback_opt {
        Some(cb) => cb,
        None => panic!("TextInput Callback could not be found with id {}", wco.id),
    };

    let value = match wco.value_str {
        Some(vl) => vl,
        None => panic!("TextInput value in callback could not be found"),
    };
                  
    Python::with_gil(|py| {
        if wco.user_data.is_some() {
            let user_data = match wco.user_data {
                Some(ud) => ud,
                None => panic!("TextInput callback user_data not found."),
            };
            callback.call1(py, (
                                    wco.id.clone(), 
                                    value, 
                                    user_data
                                    )
                            ).unwrap();
        } else {
            callback.call1(py, (
                                    wco.id.clone(), 
                                    value, 
                                    )
                            ).unwrap();
        } 
    });

    drop(app_cbs); 

}                   
