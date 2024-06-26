
use crate::access_callbacks;
use crate::app;
use super::callbacks::{WidgetCallbackIn, 
                        WidgetCallbackOut, 
                        get_set_widget_callback_data};
use super::helpers::get_padding;
use super::helpers::get_width;
use super::helpers::try_extract_boolean;
use super::helpers::try_extract_f64;
use super::helpers::try_extract_string;
use super::helpers::try_extract_vec_f64;

use iced::widget::text::LineHeight;
use iced::{Padding, Length, Element};
use iced::widget::{TextInput, Space};

use pyo3::pyclass;
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
                                            .secure(input.is_secure)
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
            wci.value_str = Some(value.clone());
            let mut wco: WidgetCallbackOut = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = "on_input".to_string();
            wco.value_str = Some(value);
            process_callback(wco);
        },
        TIMessage::OnSubmit(value) => {
            // wci.value_str = Some(value.clone());
            let mut wco: WidgetCallbackOut = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = "on_submit".to_string();
            wco.value_str = Some(value);
            process_callback(wco);
        }
        TIMessage::OnPast(value) => {
            wci.value_str = Some(value.clone());
            let mut wco: WidgetCallbackOut = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = "on_paste".to_string();
            wco.value_str = Some(value);
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
            let res = callback.call1(py, (
                                                            wco.id.clone(), 
                                                            value, 
                                                            user_data
                                                            ));
            match res {
                Ok(_) => (),
                Err(er) => panic!("TextInput: 3 parameters (id, value, user_data) are required or a python error in this function. {er}"),
            }
        } else {
            let res = callback.call1(py, (
                                                                wco.id.clone(), 
                                                                value, 
                                                                ));
            match res {
                Ok(_) => (),
                Err(er) => panic!("TextInput: 2 parameters (id, value) are required or a python error in this function. {er}"),
            }
        } 
    });

    drop(app_cbs); 

}


#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgTextInputParams {
    Placeholder,
    Value,
    IsSecure,
    Width,
    Padding,
    Size,
    LineHeight,
}

pub fn text_input_item_update(ti: &mut IpgTextInput,
                                item: PyObject,
                                value: PyObject,
                                )
{
    let update = try_extract_text_input_update(item);

    match update {
        IpgTextInputParams::Placeholder => {
            ti.placeholder = try_extract_string(value);
        },
        IpgTextInputParams::Value => {
            ti.value = try_extract_string(value);
        },
        IpgTextInputParams::IsSecure => {
            ti.is_secure = try_extract_boolean(value);
        },
        IpgTextInputParams::Width => {
            let val = try_extract_f64(value);
            ti.width = get_width(Some(val as f32), false);
        },
        IpgTextInputParams::Padding => {
            let val = try_extract_vec_f64(value);
            ti.padding =  get_padding(val);
        },
        IpgTextInputParams::Size => {
            ti.size = try_extract_f64(value) as f32;
        },
        IpgTextInputParams::LineHeight => {
            let val = try_extract_f64(value) as f32;
            ti.line_height = LineHeight::Relative(val);
        },
    }
}


fn try_extract_text_input_update(update_obj: PyObject) -> IpgTextInputParams {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgTextInputParams>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("TextInput update extraction failed"),
        }
    })
}