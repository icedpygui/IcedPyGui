//! ipg_text_input
#![allow(clippy::enum_variant_names)]
use crate::{access_callbacks, app, IpgState};
use super::callbacks::{set_or_get_widget_callback_data, WidgetCallbackIn, WidgetCallbackOut};
use super::helpers::{get_padding_f64, get_radius, get_width};
use super::helpers::{try_extract_boolean, try_extract_f64, 
    try_extract_string, try_extract_u16, try_extract_vec_f64};

use iced::widget::text::LineHeight;
use iced::widget::text_input;
use iced::widget::text_input::{Style, Status};
use iced::{Color, Element, Length, Padding, Pixels, Theme};
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
    pub style_id: Option<String>,
    pub show: bool,
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
        style_id: Option<String>,
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
            style_id,
            show,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IpgTextInputStyle {
    pub id: usize,
    pub background_color: Option<Color>,
    pub border_color: Option<Color>,
    pub border_color_hovered: Option<Color>,
    pub border_color_focused: Option<Color>,
    pub border_width: Option<f32>,
    pub border_radius: Option<Vec<f32>>,
    // pub icon_color: Option<Color>,
    pub placeholder_color: Option<Color>,
    pub value_color: Option<Color>,
    pub selection_color: Option<Color>,
}

impl IpgTextInputStyle {
    pub fn new(
        id: usize,
        background_color: Option<Color>,
        border_color: Option<Color>,
        border_color_hovered: Option<Color>,
        border_color_focused: Option<Color>,
        border_width: Option<f32>,
        border_radius: Option<Vec<f32>>,
        // icon_color: Option<Color>,
        placeholder_color: Option<Color>,
        value_color: Option<Color>,
        selection_color: Option<Color>,
    )  -> Self {
        Self {
            id,
            background_color,
            border_color,
            border_color_hovered,
            border_color_focused,
            border_width,
            border_radius,
            // icon_color,
            placeholder_color,
            value_color,
            selection_color,
        }
    }
}


#[derive(Debug, Clone)]
pub enum TIMessage {
    OnInput(String),
    OnSubmit(String),
    OnPaste(String),
}

pub fn construct_text_input(input: IpgTextInput, 
                            style_opt: Option<IpgTextInputStyle>) 
                            -> Element<'static, app::Message> {
    
    if !input.show {
        return Space::new(0.0, 0.0).into()
    }
    
    let txt: Element<TIMessage> =  TextInput::new(input.placeholder.as_str(), 
                                                input.value.as_str()
                                            )
                                            .on_input(TIMessage::OnInput)
                                            .on_submit(TIMessage::OnSubmit(input.value))
                                            .on_paste(TIMessage::OnPaste)
                                            .secure(input.is_secure)
                                            .width(input.width)
                                            .padding(input.padding)
                                            .size(input.size)
                                            .line_height(input.line_height)
                                            // .icon(text_input::Icon {
                                            //     font: BOOTSTRAP_FONT,
                                            //     code_point: required::icon_to_char(required::Icon::CaretRightFill),
                                            //     size: Some(Pixels(60.0)),
                                            //     spacing: 5.0,
                                            //     side: text_input::Side::Right,
                                            // })
                                            .style(move|theme, status|
                                                get_styling(theme, status, 
                                                    style_opt.clone(),
                                                ))
                                            .into();

    txt.map(move |message| app::Message::TextInput(input.id, message))

}

pub fn text_input_callback(state: &mut IpgState, id: usize, message: TIMessage) {
    // During the input, the widget is assigned the value so that it shows
    // during typing.  On submit, the text box is cleared, so no value.
    // However, in both cases the value is passed to the callback.
    let mut wci: WidgetCallbackIn = WidgetCallbackIn{id, ..Default::default()};
           
    match message {
        TIMessage::OnInput(value) => {
            wci.value_str = Some(value.clone());
            let mut wco: WidgetCallbackOut = set_or_get_widget_callback_data(state, wci);
            wco.id = id;
            wco.event_name = "on_input".to_string();
            wco.value_str = Some(value);
            process_callback(wco);
        },
        TIMessage::OnSubmit(value) => {
            // wci.value_str = Some(value.clone());
            let mut wco: WidgetCallbackOut = set_or_get_widget_callback_data(state, wci);
            wco.id = id;
            wco.event_name = "on_submit".to_string();
            wco.value_str = Some(value);
            process_callback(wco);
        }
        TIMessage::OnPaste(value) => {
            wci.value_str = Some(value.clone());
            let mut wco: WidgetCallbackOut = set_or_get_widget_callback_data(state, wci);
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
                                                            wco.id, 
                                                            value, 
                                                            user_data
                                                            ));
            match res {
                Ok(_) => (),
                Err(er) => panic!("TextInput: 3 parameters (id, value, user_data) are required or a python error in this function. {er}"),
            }
        } else {
            let res = callback.call1(py, (
                                                                wco.id, 
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
pub enum IpgTextInputParam {
    Placeholder,
    Value,
    IsSecure,
    Width,
    Padding,
    Size,
    LineHeightPixels,
    LineHeightRelative,
    StyleId,
}

pub fn text_input_item_update(ti: &mut IpgTextInput,
                                item: PyObject,
                                value: PyObject,
                                )
{
    let update = try_extract_text_input_update(item);

    match update {
        IpgTextInputParam::Placeholder => {
            ti.placeholder = try_extract_string(value);
        },
        IpgTextInputParam::Value => {
            ti.value = try_extract_string(value);
        },
        IpgTextInputParam::IsSecure => {
            ti.is_secure = try_extract_boolean(value);
        },
        IpgTextInputParam::Width => {
            let val = try_extract_f64(value);
            ti.width = get_width(Some(val as f32), false);
        },
        IpgTextInputParam::Padding => {
            let val = try_extract_vec_f64(value);
            ti.padding =  get_padding_f64(val);
        },
        IpgTextInputParam::Size => {
            ti.size = try_extract_f64(value) as f32;
        },
        IpgTextInputParam::LineHeightPixels => {
            let val = try_extract_u16(value);
            ti.line_height = LineHeight::Absolute(Pixels(val.into()));
        },
        IpgTextInputParam::LineHeightRelative => {
            let val = try_extract_f64(value) as f32;
            ti.line_height = LineHeight::Relative(val);
        },
        IpgTextInputParam::StyleId => {
            ti.style_id = Some(try_extract_string(value));
        },
    }
}


fn try_extract_text_input_update(update_obj: PyObject) -> IpgTextInputParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgTextInputParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("TextInput update extraction failed"),
        }
    })
}

fn get_styling(theme: &Theme, 
                status: Status, 
                style_opt: Option<IpgTextInputStyle>
                ) -> Style {

    if style_opt.is_none() {
        return text_input::default(theme, status)
    }     

    let style = style_opt.unwrap();

    let mut style_base = text_input::default(theme, Status::Active);

    if style.background_color.is_some() {
        style_base.background = style.background_color.unwrap().into();
    }

    if style.border_width.is_some() {
        style_base.border.width = style.border_width.unwrap();
    }

    if style.border_radius.is_some() {
        style_base.border.radius = get_radius(style.border_radius.clone().unwrap(),
                                        "TextInput".to_string());
    }

    if style.border_color.is_some() {
        style_base.border.color = style.border_color.unwrap();
    }

    // if style.icon_color.is_some() {
    //     style_base.icon = style.icon_color.unwrap();
    // }

    if style.placeholder_color.is_some() {
        style_base.placeholder = style.placeholder_color.unwrap();
    }

    if style.value_color.is_some() {
        style_base.value = style.value_color.unwrap();
    }

    if style.selection_color.is_some() {
        style_base.selection = style.selection_color.unwrap();
    }

    let palette = theme.extended_palette();

    match status {
        Status::Active =>style_base,
        Status::Hovered => {
            if style.border_color_hovered.is_some() {
                style_base.border.color = style.border_color_hovered.unwrap();
            } else {
                style_base.border.color = palette.background.base.text;
            }
            style_base
        },
        Status::Focused => {
            if style.border_color_focused.is_some() {
                style_base.border.color = style.border_color_focused.unwrap();
            } else {
                style_base.border.color = palette.primary.strong.color;
            }
            style_base
        },
        Status::Disabled => {
            style_base.value = style_base.placeholder;
            style_base.background = palette.background.weak.color.into();
            
            style_base
        }
    }


}
