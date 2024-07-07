#![allow(unused_imports)]

use crate::graphics::colors::{get_color, match_ipg_color, IpgColor};
use crate::ipg_widgets::helpers::{try_extract_boolean, try_extract_f64, try_extract_string};
use crate::style::styling::{darken, get_text_pair, lighten, IpgColorPalette, IpgStyleStandard, IpgStylingStandard, StyleBorder};
use crate::{access_callbacks, access_state};
use crate::app;
use super::helpers::{get_radius, get_shaping, get_width, try_extract_style_standard};
use super::callbacks::{WidgetCallbackIn, 
                        WidgetCallbackOut, 
                        get_set_widget_callback_data,
                        };

use crate::graphics::BOOTSTRAP_FONT;
use crate::graphics::bootstrap_icon::{Icon, icon_to_char};

use iced::advanced::text;
use iced::border::Radius;
use iced::{Border, Color, Element, Font, Length, Pixels, Theme};
use iced::theme::palette::{Background, Pair};
use iced::widget::text::{LineHeight, Shaping};
use iced::widget::{Checkbox, Space};
use iced::widget::checkbox::{self, primary, Status};

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
    pub style: Option<String>,
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
        style: Option<String>,
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
                style,
                style_standard,
            }
    }
}


#[derive(Debug, Clone, Default)]
pub struct IpgCheckboxStyle {
    pub id: usize,
    pub base: Option<Color>,
    pub strong: Option<Color>,
    pub strong_factor: Option<f32>,
    pub weak: Option<Color>,
    pub weak_factor: Option<f32>,
    pub border: Option<Color>,
    pub border_radius: Vec<f32>,
    pub border_width: f32,
    pub icon_color: Option<Color>,
    pub text: Option<Color>,
}

impl IpgCheckboxStyle {
    pub fn new(
        id: usize,
        base: Option<Color>,
        strong: Option<Color>,
        strong_factor: Option<f32>,
        weak: Option<Color>,
        weak_factor: Option<f32>,
        border: Option<Color>,
        border_radius: Vec<f32>,
        border_width: f32,
        icon_color: Option<Color>,
        text: Option<Color>,
    ) -> Self {
        Self {
            id,
            base,
            strong,
            strong_factor,
            weak,
            weak_factor,
            border,
            border_radius,
            border_width,
            icon_color,
            text,
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
                                    chk.style.clone(), 
                                    chk.style_standard.clone(),
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
pub enum IpgCheckboxParams {
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
        IpgCheckboxParams::IconSize => {
            chk.icon_size = try_extract_f64(value) as f32;
        },
        IpgCheckboxParams::IconX => {
            chk.icon_x = try_extract_boolean(value);
        },
        IpgCheckboxParams::IsChecked => {
            chk.is_checked = try_extract_boolean(value);
        },
        IpgCheckboxParams::Label => {
            chk.label = try_extract_string(value);
        },
        IpgCheckboxParams::Show => {
            chk.show = try_extract_boolean(value);
        },
        IpgCheckboxParams::Size => {
            chk.size = try_extract_f64(value) as f32;
        },
        IpgCheckboxParams::Spacing => {
            chk.spacing = try_extract_f64(value) as f32;
        },
        IpgCheckboxParams::TextLineHeight => {
            let tlh = try_extract_f64(value);
            chk.text_line_height =  LineHeight::Relative(tlh as f32);
        },
        IpgCheckboxParams::TextShaping => {
            let ts =try_extract_string(value);
            chk.text_shaping = get_shaping(ts); 
        },
        IpgCheckboxParams::TextSize => {
            chk.text_size = try_extract_f64(value) as f32;
        },
        IpgCheckboxParams::Style => {
            chk.style = Some(try_extract_string(value))
        },
        IpgCheckboxParams::StyleStandard => {
            let val = try_extract_style_standard(value);
            chk.style_standard = Some(val)
        },
        IpgCheckboxParams::Width => {
            let wd = try_extract_f64(value);
            chk.width =  get_width(Some(wd as f32), false);
        },
        IpgCheckboxParams::WidthFill => {
            let wd = try_extract_boolean(value);
            chk.width =  get_width(None, wd);
        },
    }
}


pub fn try_extract_checkbox_update(update_obj: PyObject) -> IpgCheckboxParams {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgCheckboxParams>(py);

        match res {
            Ok(update) => update,
            Err(_) => panic!("Checkbox update extraction failed"),
        }
    })
}

pub fn get_styling(theme: &Theme, status: Status,
                    style: Option<String>,
                    style_standard: Option<IpgStyleStandard>, 
                    ) -> checkbox::Style 
{
    if style_standard.is_none() && style.is_none() {
        return checkbox::primary(theme, status)
    }
    
    let state = access_state();

    let style_opt = if style.is_some() {
        state.checkbox_style.get(&style.unwrap())
    } else {
        None
    };

    let mut style = IpgCheckboxStyle::default();
    let mut border = Border::default();

    if style_opt.is_some() {
        style = style_opt.unwrap().clone();
        if style.border.is_some() {
            border.color = style.border.unwrap();
        }
    
        border.radius = get_radius(style.border_radius.clone());
        border.width = style.border_width;
    }
  
    if style_standard.is_some() {
        let style_std = style_standard.clone().unwrap();

        match style_std {
            IpgStyleStandard::Primary => {
                let mut style = checkbox::primary(theme, status) ;
                style.border.width = border.width;
                style.border.radius = border.radius;
                return style
            },
            IpgStyleStandard::Success => {
                let mut style = checkbox::success(theme, status);
                style.border.width = border.width;
                style.border.radius = border.radius;
                return style
            },
            IpgStyleStandard::Danger => {
                let mut style = checkbox::danger(theme, status);
                style.border.width = border.width;
                style.border.radius = border.radius;
                return style
            },
            IpgStyleStandard::Text => panic!("StandardStyle::Text not valid for checkbox"),
        };
    }

    if style.base.is_none() && (style.strong.is_some() || style.weak.is_some()) {
        panic!("Checkbox style: if you define style.strong or style.weak, you must define style.base too")
    }

    if (style.strong.is_some() && style.weak.is_none()) || 
        (style.strong.is_none() && style.weak.is_some()) {
        panic!("Checkbox style: if you define style.strong or style.weak, you must define both strong and weak")
    }

    // Initialize strong_pair with anything
    let mut base = Color::default();
    let mut strong = Color::default();
    let mut weak = Color::default();
    let mut icon_color = Color::default();

    // all custom colors
    if style.base.is_some() && style.strong.is_some() && style.weak.is_some() {
        
        base = style.base.unwrap();
        strong = style.strong.unwrap();
        weak = style.weak.unwrap();
        
        if style.icon_color.is_some() {
            icon_color = style.icon_color.unwrap();
        } else {
            icon_color = get_text_pair(style.icon_color, style.strong.unwrap());
        }
    }

    // Generate strong and weak
    if style.base.is_some() && style.strong.is_none() && style.weak.is_none() {
        let text_color = get_text_pair(style.text, style.base.unwrap());
        let background = theme.palette().background;
        let palette = IpgColorPalette::generate(style.base.unwrap(),
                                                                background,
                                                                text_color, 
                                                                style.strong_factor,
                                                                style.weak_factor);
        base = palette.base.color;
        strong = palette.strong.color;
        weak = palette.weak.color;

        if style.icon_color.is_some() {
            icon_color = style.icon_color.unwrap();
        } else {
            icon_color = palette.strong.text;
        }
        
    }

    match status {
        Status::Active { is_checked } => return styled(
            icon_color,
            base,
            strong,
            is_checked,
            border,
            style.text,
        ),
        Status::Hovered { is_checked } => return styled(
            icon_color,
            weak,
            base,
            is_checked,
            border,
            style.text,
        ),
        Status::Disabled { is_checked } => return styled(
            icon_color,
            weak,
            strong,
            is_checked,
            border,
            style.text,
        ),
    }

}

fn styled(
    icon_color: Color,
    base: Color,
    accent: Color,
    is_checked: bool,
    border: Border,
    text_color: Option<Color>,
) -> checkbox::Style {
    checkbox::Style {
        background: iced::Background::Color(if is_checked {
            accent
        } else {
            base
        }),
        icon_color,
        border,
        text_color,
    }
}

