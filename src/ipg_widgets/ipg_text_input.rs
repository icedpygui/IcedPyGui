
use crate::access_callbacks;
use crate::access_state;
use crate::app;
use crate::style::styling::is_dark;
use crate::style::styling::strong;
use crate::style::styling::weak;
use super::callbacks::{WidgetCallbackIn, 
                        WidgetCallbackOut, 
                        get_set_widget_callback_data};
use super::helpers::get_padding_f64;
use super::helpers::get_radius;
use super::helpers::get_width;
use super::helpers::try_extract_boolean;
use super::helpers::try_extract_f64;
use super::helpers::try_extract_string;
use super::helpers::try_extract_u16;
use super::helpers::try_extract_vec_f64;

use iced::theme::palette::Pair;
use iced::widget::text::LineHeight;
use iced::widget::text_input;
use iced::widget::text_input::{Style, Status};
use iced::Border;
use iced::Color;
use iced::Pixels;
use iced::Theme;
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
    style: Option<String>,
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
        style: Option<String>,
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
            style,
            show,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IpgTextInputStyle {
    pub id: usize,
    pub background_color: Option<Color>,
    pub background_color_strong: Option<Color>,
    pub background_strong_factor: Option<f32>,
    pub background_color_weak: Option<Color>,
    pub background_weak_factor: Option<f32>,
    pub border_color: Option<Color>,
    pub border_color_hovered: Option<Color>,
    pub border_color_focused: Option<Color>,
    pub border_width: Option<f32>,
    pub border_radius: Option<Vec<f32>>,
    pub icon_color: Option<Color>,
    pub placeholder_color: Option<Color>,
    pub value_color: Option<Color>,
    pub selection_color: Option<Color>,
}

impl IpgTextInputStyle {
    pub fn new(
        id: usize,
        background_color: Option<Color>,
        background_color_strong: Option<Color>,
        background_strong_factor: Option<f32>,
        background_color_weak: Option<Color>,
        background_weak_factor: Option<f32>,
        border_color: Option<Color>,
        border_color_hovered: Option<Color>,
        border_color_focused: Option<Color>,
        border_width: Option<f32>,
        border_radius: Option<Vec<f32>>,
        icon_color: Option<Color>,
        placeholder_color: Option<Color>,
        value_color: Option<Color>,
        selection_color: Option<Color>,
    )  -> Self {
        Self {
            id,
            background_color,
            background_color_strong,
            background_strong_factor,
            background_color_weak,
            background_weak_factor,
            border_color,
            border_color_hovered,
            border_color_focused,
            border_width,
            border_radius,
            icon_color,
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
                                            .style(move|theme, status|
                                                get_styling(theme, status, 
                                                    input.style.clone()
                                                ))
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
pub enum IpgTextInputParam {
    Placeholder,
    Value,
    IsSecure,
    Width,
    Padding,
    Size,
    LineHeightPixels,
    LineHeightRelative,
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
                style_str: Option<String>
                ) -> Style {

    if style_str.is_none() {
        return text_input::default(theme, status)
    }     

    let state = access_state();

    let style_opt = state.text_input_style.get(&style_str.clone().unwrap());

    let style = match style_opt {
        Some(st) => st,
        None => panic!("TextInput styling: Unable to find the style_id '{}'.", style_str.unwrap())
    };

    let mut active_style = text_input::default(theme, status);

    if style.background_color.is_some() {
        active_style.background = style.background_color.unwrap().into();
    }

    // Border color is the strong color of the background for active.
    // Therefore, if background color is defined then a strong color 
    // is calculated unless the background strong color is defined.
    if style.border_color.is_some() {
        active_style.border.color = style.border_color.unwrap();
    } else {
        if style.background_color.is_some() && style.background_color_strong.is_none() {
            active_style.border.color = strong(style.background_color.unwrap(), 
                                                style.background_strong_factor);
        }

        if style.background_color_strong.is_some() {
            active_style.border.color = style.background_color_strong.unwrap()
        }
    }

    if style.border_width.is_some() {
        active_style.border.width = style.border_width.unwrap();
    }

    if style.border_radius.is_some() {
        active_style.border.radius = get_radius(style.border_radius.clone().unwrap(),
                                        "TextInput".to_string());
    }

    // Icon color is the weak color of the background for active.
    // Therefore, if background color is defined then a weak color 
    // is calculated unless the background weak color is defined.
    if style.icon_color.is_some() {
        active_style.icon = style.icon_color.unwrap();
    } else {
        if style.background_color.is_some() && style.background_color_weak.is_none() {
            active_style.border.color = weak(style.background_color.unwrap(),
                                                theme.palette().background, 
                                                style.background_weak_factor);
        }

        if style.background_color_weak.is_some() {
            active_style.border.color = style.background_color_weak.unwrap()
        } 
    }

    // Placeholder color is the strong color of the background for active.
    // Therefore, if background color is defined then a strong color 
    // is calculated unless the background strong color is defined.
    if style.placeholder_color.is_some() {
        active_style.placeholder = style.placeholder_color.unwrap();
    } else {
        if style.background_color.is_some() && style.background_color_strong.is_none() {
            active_style.placeholder = strong(style.background_color.unwrap(), 
                                                style.background_strong_factor);
        }

        if style.background_color_strong.is_some() {
            active_style.placeholder = style.background_color_strong.unwrap()
        }
    }

    // Value color is the background color, therefore, if the value color
    // is not defined and the background color is defined, the the value
    // color equals the background color.
    if style.value_color.is_some() {
        active_style.value = style.value_color.unwrap();
    } else if style.background_color.is_some() {
        active_style.value = style.background_color.unwrap();
    }

    // Selection color is the weak color of the background for active.
    // Therefore, if background color is defined then a weak color 
    // is calculated unless the background weak color is defined.
    if style.selection_color.is_some() {
        active_style.selection = style.selection_color.unwrap();
    } else {
        if style.background_color.is_some() && style.background_color_weak.is_none() {
            active_style.selection = weak(style.background_color.unwrap(),
                                                theme.palette().background, 
                                                style.background_weak_factor);
        }

        if style.background_color_weak.is_some() {
            active_style.selection = style.background_color_weak.unwrap()
        } 
    }

    let palette = theme.extended_palette();

    let mut hovered_style = Style {
        border: Border {
            color: palette.background.base.text,
            ..active_style.border
        },
        ..active_style
    };


    // For hover, the border color goes from background strong
    // to background text color
    if style.border_color_hovered.is_some() {
        hovered_style.border.color = style.border_color_hovered.unwrap();
    } else if style.background_color.is_some() {
        let bg_color = style.background_color.unwrap();
        let text_color = if is_dark(bg_color) {
            Color::WHITE
        } else {
            Color::BLACK
        };
        
        let pair = Pair::new(bg_color, text_color);
        hovered_style.border.color = pair.text;
    }

    let mut focused_style = Style {
        border: Border {
            color: palette.primary.strong.color,
            ..active_style.border
        },
        ..active_style
    };

    if style.border_color_focused.is_some() {
        focused_style.border.color = style.border_color_focused.unwrap();
    } else if style.background_color_strong.is_some() {
        focused_style.border.color = style.background_color_strong.unwrap();
    }

    let mut disabled_style = Style {
        background: palette.background.weak.color.into(),
        value: active_style.placeholder,
        ..active_style
    };

    if style.background_color_weak.is_some() {
        disabled_style.background = style.background_color_weak.unwrap().into();
    } else if style.background_color.is_some() {
        let bg_color_weak = weak(style.background_color.unwrap(),
                                        theme.palette().background, 
                                        style.background_weak_factor);
        disabled_style.background = bg_color_weak.into();
    }


    match status {
        Status::Active =>active_style,
        Status::Hovered => hovered_style,
        Status::Focused => focused_style,
        Status::Disabled => disabled_style,
    }


}
