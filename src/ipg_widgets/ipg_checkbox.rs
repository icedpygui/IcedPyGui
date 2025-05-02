//! ipg_checkbox
use crate::graphics::colors::get_color;
use crate::style::styling::IpgStyleStandard;
use crate::{access_callbacks, access_user_data1, access_user_data2, IpgState};
use crate::app;
use super::helpers::{get_radius, get_shaping, get_width, try_extract_boolean, 
    try_extract_f64, try_extract_ipg_color, try_extract_rgba_color, 
    try_extract_string, try_extract_style_standard, try_extract_vec_f32};
use super::callbacks::{set_or_get_widget_callback_data, WidgetCallbackIn};
use super::ipg_enums::IpgWidgets;

use crate::graphics::BOOTSTRAP_FONT;
use crate::graphics::bootstrap_icon::{Icon, icon_to_char};

use iced::advanced::text;
use iced::{Color, Element, Length, Theme};
use iced::widget::text::{LineHeight, Shaping};
use iced::widget::Checkbox;
use iced::widget::checkbox::{self, Status};

use pyo3::{pyclass, PyObject, Python};


#[derive(Debug, Clone)]
pub struct IpgCheckBox {
    pub id: usize,
    pub parent_id: String,
    pub show: bool,
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
    pub style_id: Option<usize>,
    pub style_standard: Option<IpgStyleStandard>,
}

impl IpgCheckBox {
    pub fn new(
        id: usize,
        parent_id: String,
        show: bool,
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
        style_id: Option<usize>,
        style_standard: Option<IpgStyleStandard>,
        ) -> Self {
            Self {
                id,
                parent_id,
                show,
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

pub fn construct_checkbox<'a>(chk: &'a IpgCheckBox, 
                        style_opt: Option<&IpgWidgets>) 
                        -> Option<Element<'a, app::Message>> {

    if !chk.show {
        return None
    };

    let style = get_chk_style(style_opt);

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
                                    style.clone(), 
                                    chk.style_standard.clone(),
                                    chk.is_checked,
                                    )  
                                })
                            .into();

    Some(ipg_chk.map(move |message| app::Message::CheckBox(chk.id, message)))
}

pub fn checkbox_callback(state: &mut IpgState, id: usize, message: CHKMessage) {

    match message {
        CHKMessage::OnToggle(on_toggle) => {
            let mut wci: WidgetCallbackIn = WidgetCallbackIn{id, ..Default::default()};
            wci.on_toggle = Some(on_toggle);
            let _ = set_or_get_widget_callback_data(state, wci);

            process_callback(id, on_toggle, "on_toggle".to_string());
        }
    }
}

pub fn process_callback(
        id: usize, 
        is_checked: bool, 
        event_name: String) 
{
    let ud1 = access_user_data1();
    let app_cbs = access_callbacks();

    // Retrieve the callback
    let callback = match app_cbs.callbacks.get(&(id, event_name)) {
        Some(cb) => Python::with_gil(|py| cb.clone_ref(py)),
        None => return,
    };

    drop(app_cbs);

    // Check user data from ud1
    if let Some(user_data) = ud1.user_data.get(&id) {
        Python::with_gil(|py| {
            if let Err(err) = callback.call1(py, (id, is_checked, user_data)) {
                panic!("Checkbox callback error: {err}");
            }
        });
        drop(ud1); // Drop ud1 before processing ud2
        return;
    }
    drop(ud1); // Drop ud1 if no user data is found

    // Check user data from ud2
    let ud2 = access_user_data2();
    if let Some(user_data) = ud2.user_data.get(&id) {
        Python::with_gil(|py| {
            if let Err(err) = callback.call1(py, (id, is_checked, user_data)) {
                panic!("Checkbox callback error: {err}");
            }
        });
        drop(ud2); // Drop ud2 after processing
        return;
    }
    drop(ud2); // Drop ud2 if no user data is found

    // If no user data is found in both ud1 and ud2, call the callback with only the id and is_checked
    Python::with_gil(|py| {
        if let Err(err) = callback.call1(py, (id, is_checked)) {
            panic!("Checkbox callback error: {err}");
        }
    });
}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
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
                            item: &PyObject,
                            value: &PyObject,
                            )
{
    let update = try_extract_checkbox_update(item);
    let name = "Checkbox".to_string();
    match update {
        IpgCheckboxParam::IconSize => {
            chk.icon_size = try_extract_f64(value, name) as f32;
        },
        IpgCheckboxParam::IconX => {
            chk.icon_x = try_extract_boolean(value, name);
        },
        IpgCheckboxParam::IsChecked => {
            chk.is_checked = try_extract_boolean(value, name);
        },
        IpgCheckboxParam::Label => {
            chk.label = try_extract_string(value, name);
        },
        IpgCheckboxParam::Show => {
            chk.show = try_extract_boolean(value, name);
        },
        IpgCheckboxParam::Size => {
            chk.size = try_extract_f64(value, name) as f32;
        },
        IpgCheckboxParam::Spacing => {
            chk.spacing = try_extract_f64(value, name) as f32;
        },
        IpgCheckboxParam::TextLineHeight => {
            let tlh = try_extract_f64(value, name);
            chk.text_line_height =  LineHeight::Relative(tlh as f32);
        },
        IpgCheckboxParam::TextShaping => {
            let ts =try_extract_string(value, name);
            chk.text_shaping = get_shaping(ts); 
        },
        IpgCheckboxParam::TextSize => {
            chk.text_size = try_extract_f64(value, name) as f32;
        },
        IpgCheckboxParam::Style => {
            chk.style_id = Some(try_extract_f64(value, name) as usize)
        },
        IpgCheckboxParam::StyleStandard => {
            let val = try_extract_style_standard(value, name);
            chk.style_standard = Some(val)
        },
        IpgCheckboxParam::Width => {
            let wd = try_extract_f64(value, name);
            chk.width =  get_width(Some(wd as f32), false);
        },
        IpgCheckboxParam::WidthFill => {
            let wd = try_extract_boolean(value, name);
            chk.width =  get_width(None, wd);
        },
    }
}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgCheckboxStyleParam {
    BackgroundIpgColor,
    BackgroundRgbaColor,
    BackgroundIpgColorHovered,
    BackgroundRgbaColorHovered,
    AccentIpgColor,
    AccentRgbaColor,
    AccentIpgColorHovered,
    AccentRgbaColorHovered,
    BorderIpgColor,
    BorderRgbaColor,
    BorderRadius,
    BorderWidth,
    IconIpgColor,
    IconRgbaColor,
    TextIpgColor,
    TextRgbaColor,
}

pub fn checkbox_style_update_item(style: &mut IpgCheckboxStyle,
                            item: &PyObject,
                            value: &PyObject,) 
{
    let update = try_extract_checkbox_style_update(item);
    let name = "CheckboxStyle".to_string();
    match update {
        IpgCheckboxStyleParam::BackgroundIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.background_color = get_color(None, Some(color), 1.0, false);
        },
        IpgCheckboxStyleParam::BackgroundRgbaColor => {
            style.background_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgCheckboxStyleParam::BackgroundIpgColorHovered => {
            let color = try_extract_ipg_color(value, name);
            style.background_color_hovered = get_color(None, Some(color), 1.0, false);
        },
        IpgCheckboxStyleParam::BackgroundRgbaColorHovered => {
            style.background_color_hovered = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgCheckboxStyleParam::AccentIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.accent_color = get_color(None, Some(color), 1.0, false);
        },
        IpgCheckboxStyleParam::AccentRgbaColor => {
            style.accent_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgCheckboxStyleParam::AccentIpgColorHovered => {
            let color = try_extract_ipg_color(value, name);
            style.accent_color_hovered = get_color(None, Some(color), 1.0, false);
        },
        IpgCheckboxStyleParam::AccentRgbaColorHovered => {
            style.accent_color_hovered = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgCheckboxStyleParam::BorderIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.border_color = get_color(None, Some(color), 1.0, false);
        },
        IpgCheckboxStyleParam::BorderRgbaColor => {
            style.border_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgCheckboxStyleParam::BorderRadius => {
            style.border_radius = try_extract_vec_f32(value, name);
        },
        IpgCheckboxStyleParam::BorderWidth => {
            style.border_width = try_extract_f64(value, name) as f32;
        },
        IpgCheckboxStyleParam::IconIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.icon_color = get_color(None, Some(color), 1.0, false);
        },
        IpgCheckboxStyleParam::IconRgbaColor => {
            style.icon_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgCheckboxStyleParam::TextIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.text_color = get_color(None, Some(color), 1.0, false);
        },
        IpgCheckboxStyleParam::TextRgbaColor => {
            style.text_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
    }
}

pub fn try_extract_checkbox_update(update_obj: &PyObject) -> IpgCheckboxParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgCheckboxParam>(py);

        match res {
            Ok(update) => update,
            Err(_) => panic!("Checkbox update extraction failed"),
        }
    })
}

pub fn get_styling(theme: &Theme, status: Status,
                    style_opt: Option<IpgCheckboxStyle>,
                    style_standard: Option<IpgStyleStandard>,
                    is_checked: bool, 
                    ) -> checkbox::Style 
{

    if style_standard.is_none() && style_opt.is_none() {
        return checkbox::primary(theme, status)
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

pub fn try_extract_checkbox_style_update(update_obj: &PyObject) -> IpgCheckboxStyleParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgCheckboxStyleParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Checkbox style update extraction failed"),
        }
    })
}

pub fn get_chk_style(style: Option<&IpgWidgets>) -> Option<IpgCheckboxStyle>{
    match style {
        Some(IpgWidgets::IpgCheckboxStyle(style)) => {
            Some(style.clone())
        }
        _ => None,
    }
}