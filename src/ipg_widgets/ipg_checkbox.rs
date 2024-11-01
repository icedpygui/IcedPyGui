//! ipg_checkbox
use crate::style::styling::IpgStyleStandard;
use crate::{access_callbacks, access_state};
use crate::app;
use super::helpers::{get_radius, get_shaping, get_width, 
    try_extract_style_standard, try_extract_boolean, 
    try_extract_f64, try_extract_string};
use super::callbacks::{WidgetCallbackIn, 
    WidgetCallbackOut, get_set_widget_callback_data};

use crate::graphics::BOOTSTRAP_FONT;
use crate::graphics::bootstrap_icon::{Icon, icon_to_char};

use iced::advanced::text;
use iced::{Color, Element, Length, Theme};
use iced::widget::text::{LineHeight, Shaping};
use iced::widget::{Checkbox, Space};
use iced::widget::checkbox::{self, Status};

use pyo3::{pyclass, PyObject, Python};


#[derive(Debug, Clone)]
pub struct IpgCheckBox {
    pub id: usize,
    pub show: bool,
    pub user_data: Option<PyObject>,

    pub is_checked: bool,
    pub label: String,
    pub width: Length,
    pub size: f32,
    pub spacing: f32,
    pub text_size: f32,
    pub text_line_height: LineHeight,
    pub text_shaping: Shaping,
    // font: Option<Font>,
    pub icon_x: bool,
    pub icon_size: f32,
    pub style_id: Option<String>,
    pub style_standard: Option<IpgStyleStandard>,
}

impl IpgCheckBox {
    pub fn new(
        id: usize,
        show: bool,
        user_data: Option<PyObject>,

        is_checked: bool,
        label: String,
        width: Length,
        size: f32,
        spacing: f32,
        text_size: f32,
        text_line_height: LineHeight,
        text_shaping: Shaping,
        icon_x: bool,
        icon_size: f32,
        style_id: Option<String>,
        style_standard: Option<IpgStyleStandard>,
        ) -> Self {
            Self {
                id,
                show,
                user_data,
                is_checked,
                label,
                width,
                size,
                spacing,
                text_size,
                text_line_height,
                text_shaping,
                icon_x,
                icon_size,
                style_id,
                style_standard,
            }
    }
}


#[derive(Debug, Clone, Default)]
pub struct IpgCheckboxStyle {
    pub id: usize,
    pub background_color: Option<Color>,
    pub background_color_hovered: Option<Color>,
    pub accent_color: Option<Color>,
    pub accent_color_hovered: Option<Color>,
    pub border_color: Option<Color>,
    pub border_radius: Vec<f32>,
    pub border_width: f32,
    pub icon_color: Option<Color>,
    pub text_color: Option<Color>,
}

impl IpgCheckboxStyle {
    pub fn new(
        id: usize,
        background_color: Option<Color>,
        background_color_hovered: Option<Color>,
        accent_color: Option<Color>,
        accent_color_hovered: Option<Color>,
        border_color: Option<Color>,
        border_radius: Vec<f32>,
        border_width: f32,
        icon_color: Option<Color>,
        text: Option<Color>,
    ) -> Self {
        Self {
            id,
            background_color,
            background_color_hovered,
            accent_color,
            accent_color_hovered,
            border_color,
            border_radius,
            border_width,
            icon_color,
            text_color: text,
        }
    }
}

#[derive(Debug, Clone)]
pub enum CHKMessage {
    OnToggle(bool),
}

pub fn construct_checkbox(chk: IpgCheckBox) -> Element<'static, app::Message> {

    if !chk.show {
        return Space::new(Length::Shrink, Length::Shrink).into()
    };
    
    let ipg_chk: Element<'_, CHKMessage> = Checkbox::new(chk.label.clone(), 
                            chk.is_checked)
                            .on_toggle(CHKMessage::OnToggle)
                            .size(chk.size)
                            .spacing(chk.spacing)
                            .text_line_height(chk.text_line_height)
                            .text_shaping(chk.text_shaping)
                            .text_size(chk.text_size)
                            //TODO: .font(BOOTSTRAP_FONT)
                            .width(chk.width)
                            .icon(if chk.icon_x {
                                checkbox::Icon {
                                    font: BOOTSTRAP_FONT,
                                    code_point: icon_to_char(Icon::X),
                                    size: Some(iced::Pixels(chk.icon_size)),
                                    line_height: text::LineHeight::Relative(1.0),
                                    shaping: text::Shaping::Basic,
                                }
                                    } else {
                                        checkbox::Icon {
                                            font: BOOTSTRAP_FONT,
                                            code_point: icon_to_char(Icon::Check),
                                            size: Some(iced::Pixels(chk.icon_size)),
                                            line_height: text::LineHeight::default(),
                                            shaping: text::Shaping::Basic}
                                    }
                            )
                            .style(move|theme: &Theme, status| {   
                                get_styling(theme, status,
                                    chk.style_id.clone(), 
                                    chk.style_standard.clone(),
                                    chk.is_checked,
                                    )  
                                })
                            .into();

    ipg_chk.map(move |message| app::Message::CheckBox(chk.id, message))
}

pub fn checkbox_callback(id: usize, message: CHKMessage) {

    match message {
        CHKMessage::OnToggle(on_toggle) => {
            let mut wci: WidgetCallbackIn = WidgetCallbackIn::default();
            wci.id = id;
            wci.on_toggle = Some(on_toggle);
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = "on_toggle".to_string();
            process_callback(wco);
        }
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
        None => panic!("Callback could not be found with id {}", wco.id),
    };
             
    Python::with_gil(|py| {
            if wco.user_data.is_some() {
                let user_data = match wco.user_data {
                    Some(ud) => ud,
                    None => panic!("Checkbox callback user_Data could not be found"),
                };
                let res = callback.call1(py, (
                                                                    wco.id.clone(), 
                                                                    wco.is_checked, 
                                                                    user_data
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Checkbox: 3 parameters (id, value, user_data) are required or python error in this function. {er}"),
                }
            } else {
                let res = callback.call1(py, (
                                                                    wco.id.clone(), 
                                                                    wco.is_checked 
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Checkbox: 2 parameters (id, value) are required or a python error in this function. {er}"),
                }
            } 
    });

    drop(app_cbs);

}


#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgCheckboxParam {
    IconSize,
    IconX,
    IsChecked,
    Label,
    Show,
    Size,
    Spacing,
    Style,
    StyleStandard,
    TextLineHeight,
    TextShaping,
    TextSize,
    Width,
    WidthFill,
}

pub fn checkbox_item_update(chk: &mut IpgCheckBox,
                            item: PyObject,
                            value: PyObject,
                            )
{
    let update = try_extract_checkbox_update(item);

    match update {
        IpgCheckboxParam::IconSize => {
            chk.icon_size = try_extract_f64(value) as f32;
        },
        IpgCheckboxParam::IconX => {
            chk.icon_x = try_extract_boolean(value);
        },
        IpgCheckboxParam::IsChecked => {
            chk.is_checked = try_extract_boolean(value);
        },
        IpgCheckboxParam::Label => {
            chk.label = try_extract_string(value);
        },
        IpgCheckboxParam::Show => {
            chk.show = try_extract_boolean(value);
        },
        IpgCheckboxParam::Size => {
            chk.size = try_extract_f64(value) as f32;
        },
        IpgCheckboxParam::Spacing => {
            chk.spacing = try_extract_f64(value) as f32;
        },
        IpgCheckboxParam::TextLineHeight => {
            let tlh = try_extract_f64(value);
            chk.text_line_height =  LineHeight::Relative(tlh as f32);
        },
        IpgCheckboxParam::TextShaping => {
            let ts =try_extract_string(value);
            chk.text_shaping = get_shaping(ts); 
        },
        IpgCheckboxParam::TextSize => {
            chk.text_size = try_extract_f64(value) as f32;
        },
        IpgCheckboxParam::Style => {
            chk.style_id = Some(try_extract_string(value))
        },
        IpgCheckboxParam::StyleStandard => {
            let val = try_extract_style_standard(value);
            chk.style_standard = Some(val)
        },
        IpgCheckboxParam::Width => {
            let wd = try_extract_f64(value);
            chk.width =  get_width(Some(wd as f32), false);
        },
        IpgCheckboxParam::WidthFill => {
            let wd = try_extract_boolean(value);
            chk.width =  get_width(None, wd);
        },
    }
}


pub fn try_extract_checkbox_update(update_obj: PyObject) -> IpgCheckboxParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgCheckboxParam>(py);

        match res {
            Ok(update) => update,
            Err(_) => panic!("Checkbox update extraction failed"),
        }
    })
}

pub fn get_styling(theme: &Theme, status: Status,
                    style_id: Option<String>,
                    style_standard: Option<IpgStyleStandard>,
                    is_checked: bool, 
                    ) -> checkbox::Style 
{

    if style_standard.is_none() && style_id.is_none() {
        return checkbox::primary(theme, status)
    }
    
    let state = access_state();

    let style_opt = if style_id.is_some() {
        state.checkbox_style.get(&style_id.clone().unwrap())
    } else {
        None
    };
    
    if style_id.is_some() && style_opt.is_none() {
        panic!("Checkbox: Unable to find style_id {}", style_id.unwrap())
    }

    if style_standard.is_some() {
        let style_std = style_standard.clone().unwrap();

        let mut std_style = match style_std {
            IpgStyleStandard::Primary => {
                checkbox::primary(theme, status) 
            },
            IpgStyleStandard::Success => {
                checkbox::success(theme, status)
            },
            IpgStyleStandard::Danger => {
                checkbox::danger(theme, status)
            },
            IpgStyleStandard::Text => panic!("StandardStyle::Text not valid for checkbox"),
        };

        if style_opt.is_some() {
            let style = style_opt.unwrap();
            std_style.border.width = style.border_width;
            std_style.border.radius = get_radius(style.border_radius.clone(), 
                            "Checkbox".to_string());
        }
       
        return std_style
    }

    if style_opt.is_none() {
        return checkbox::primary(theme, status)
    }

    let style = style_opt.unwrap();

    let mut border_style = checkbox::primary(theme, Status::Active { is_checked }).border;
    
    if style.border_color.is_some() {
        border_style.color = style.border_color.unwrap();
    }

    border_style.radius = get_radius(style.border_radius.clone(), "Checkbox".to_string());
    border_style.width = style.border_width;

    match status {
        Status::Active { is_checked } => {
            let mut active_style = checkbox::primary(theme, Status::Active { is_checked });
            if style.background_color.is_some() && !is_checked {
                active_style.background = iced::Background::Color(style.background_color.unwrap());
            } else if style.accent_color.is_some() && is_checked {
                active_style.background = iced::Background::Color(style.accent_color.unwrap());
            }
            if style.icon_color.is_some() {
                active_style.icon_color = style.icon_color.unwrap();
            }

            if style.text_color.is_some() {
                active_style.text_color = style.text_color;
            }
            active_style.border = border_style;

            active_style
        },
        Status::Hovered { is_checked } => {
            let mut hovered_style = checkbox::primary(theme, Status::Hovered { is_checked });
            if style.background_color_hovered.is_some() && !is_checked {
                hovered_style.background = iced::Background::Color(style.background_color_hovered.unwrap());
            } else if style.accent_color_hovered.is_some() && is_checked {
                hovered_style.background = iced::Background::Color(style.accent_color_hovered.unwrap());
            }
            if style.icon_color.is_some() {
                hovered_style.icon_color = style.icon_color.unwrap();
            }

            if style.text_color.is_some() {
                hovered_style.text_color = style.text_color;
            }
            hovered_style.border = border_style;

            hovered_style
        },
        Status::Disabled { is_checked } => {
            checkbox::danger(theme, Status::Disabled { is_checked })
        },
    }
    
}
