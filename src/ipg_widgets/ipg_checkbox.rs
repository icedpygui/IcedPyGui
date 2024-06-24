#![allow(unused_imports)]

use crate::graphics::colors::{match_ipg_color, IpgColor};
use crate::ipg_widgets::helpers::{try_extract_boolean, try_extract_f64, try_extract_string};
use crate::style::styling::{lighten, IpgStyleStandard, StyleBorder};
use crate::{access_callbacks, access_state};
use crate::app;
use super::helpers::{get_width, get_shaping};
use super::callbacks::{WidgetCallbackIn, 
                        WidgetCallbackOut, 
                        get_set_widget_callback_data,
                        };

use crate::graphics::BOOTSTRAP_FONT;
use crate::graphics::bootstrap_icon::{Icon, icon_to_char};

use iced::advanced::text;
use iced::border::Radius;
use iced::{Border, Color, Element, Font, Length, Pixels, Theme};
use iced::theme::palette::Background;
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
    pub style_standard: Option<String>,
    pub style_color: Option<String>,
    pub style_border: Option<String>,
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
        style_standard: Option<String>,
        style_color: Option<String>,
        style_border: Option<String>,
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
                style_standard,
                style_color,
                style_border,
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
                                    chk.style_standard.clone(),
                                    chk.style_color.clone(), 
                                    chk.style_border.clone(),
                                    chk.is_checked 
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
    StyleColor,
    StyleBorder,
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
        IpgCheckboxParams::StyleColor => {
            chk.style_color = Some(try_extract_string(value))
        },
        IpgCheckboxParams::StyleBorder => {
            chk.style_border = Some(try_extract_string(value))
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

// pub fn get_standard_style(theme: &Theme, status: Status, style: Option<IpgStyleStandard>) -> checkbox::Style {
//     match style {
//         Some(IpgStyleStandard::Primary) => checkbox::primary(theme, status),
//         Some(IpgStyleStandard::Success) => checkbox::success(theme, status),
//         Some(IpgStyleStandard::Danger) => checkbox::danger(theme, status),
//         Some(IpgStyleStandard::Text) => checkbox::primary(theme, status),
//         None => checkbox::primary(theme, status),
//     }
// }

pub fn get_styling(theme: &Theme, status: Status,
                    style_standard: Option<String>, 
                    style_color: Option<String>, 
                    style_border: Option<String>,
                    is_checked: bool, 
                    ) -> checkbox::Style 
{
    let state = access_state();

    // default active
    let palette = theme.extended_palette();
    let icon_color = palette.primary.strong.text;
    let base = palette.primary.strong.color;
    let accent = palette.primary.strong.color;
    let border: Border = Border {
                                radius: 2.0.into(),
                                width: 1.0,
                                color: palette.primary.strong.color,
                            };
    let mut base_style = styled(icon_color,
                                        base,
                                        accent,
                                        is_checked,
                                        border,
                                        None);

    // default hover that are different
    let base_hover = palette.background.weak.color;
    let accent_hover = palette.primary.base.color;

    let mut hover_style = styled(icon_color,
                                            base_hover,
                                            accent_hover,
                                            is_checked,
                                            border,
                                            None);
    // default desiable that are different
    let base_disabled = palette.background.weak.color;
    let accent_disabled = palette.background.strong.color;
    let disabled_style = styled(icon_color,
                                            base_disabled,
                                            accent_disabled,
                                            is_checked,
                                            border,
                                            None);
   

    let style_std_opt = if style_standard.is_some() {
        state.styling_standard.get(&style_standard.unwrap())
    } else {
        None
    };

    let mut std_selected = false;
    
    if style_std_opt.is_some() {
        std_selected = true;
        let style_std = style_std_opt.unwrap().standard.clone();
        match style_std {
            IpgStyleStandard::Primary => {
                let icon_color = palette.primary.strong.text;
                let base = palette.primary.strong.color;
                let accent = palette.primary.strong.color;
                base_style = styled(icon_color,
                                            base,
                                            accent,
                                            is_checked,
                                            border,
                                            None);

                let base_hover = palette.background.weak.color;
                let accent_hover = palette.primary.base.color;
                hover_style = styled(icon_color,
                                                    base_hover,
                                                    accent_hover,
                                                    is_checked,
                                                    border,
                                                    None);
                
                if style_border.is_some() {
                    base_style.border.color = palette.primary.weak.color;
                    hover_style.border.color = palette.primary.weak.color;
                }
            },
            IpgStyleStandard::Success => {
                let icon_color = palette.success.strong.text;
                let base = palette.success.strong.color;
                let accent = palette.success.strong.color;
                base_style = styled(icon_color,
                                            base,
                                            accent,
                                            is_checked,
                                            border,
                                            None);

                let base_hover = palette.background.weak.color;
                let accent_hover = palette.success.base.color;
                hover_style = styled(icon_color,
                                                    base_hover,
                                                    accent_hover,
                                                    is_checked,
                                                    border,
                                                    None);
                
                if style_border.is_some() {
                    base_style.border.color = palette.success.weak.color;
                    hover_style.border.color = palette.success.weak.color;
                }
            },
            IpgStyleStandard::Danger => {
                let icon_color = palette.danger.strong.text;
                let base = palette.danger.strong.color;
                let accent = palette.danger.strong.color;
                base_style = styled(icon_color,
                                            base,
                                            accent,
                                            is_checked,
                                            border,
                                            None);

                let base_hover = palette.background.weak.color;
                let accent_hover = palette.danger.base.color;
                hover_style = styled(icon_color,
                                                    base_hover,
                                                    accent_hover,
                                                    is_checked,
                                                    border,
                                                    None);
                
                if style_border.is_some() {
                    base_style.border.color = palette.danger.weak.color;
                    hover_style.border.color = palette.danger.weak.color;
                }
            },
            IpgStyleStandard::Text => (),
        }
    }

    let border_opt = if style_border.is_some() {
        state.styling_border.get(&style_border.unwrap())
    } else {
        None
    };

    match border_opt {
        Some(bd) => {
            base_style.border.radius = bd.radius;
            base_style.border.width = bd.width;
            hover_style.border.radius = bd.radius;
            hover_style.border.width = bd.width;
        },
        None => (),
    }

    if !std_selected {

        let color_palette_opt = if style_color.is_some() {
            state.styling_color.get(&style_color.unwrap())
        } else {
            None
        };

        if color_palette_opt.is_some() {
            let color_palette = color_palette_opt.unwrap().clone();
            let mut text: Color = Color::BLACK;
            if color_palette.text.is_some() {
                text = color_palette.text.unwrap();
            }
            let background = Background::new(color_palette.base, text);
            base_style.background = iced::Background::Color(background.weak.color);

            if color_palette.text.is_some() {
                base_style.text_color = Some(text);
                hover_style.text_color = Some(text);
            } else {
                base_style.text_color = Some(background.base.text);
                hover_style.text_color = Some(background.base.text);
            }
            
            if color_palette.border.is_some() {
                base_style.border.color = color_palette.border.unwrap();
                hover_style = base_style.clone();
                hover_style.background = iced::Background::Color(background.base.color);
            }

            if color_palette.icon.is_some() {
                base_style.icon_color = color_palette.icon.unwrap();
                hover_style = base_style.clone();
                hover_style.icon_color = color_palette.icon.unwrap();
                hover_style.background = iced::Background::Color(background.base.color);
            }
        }
        
    }
    
    match status {
        Status::Active { is_checked: _ } => base_style,
        Status::Hovered { is_checked: _ } => hover_style,
        Status::Disabled { is_checked: _ } => disabled_style,
    
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

