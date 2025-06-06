//! ipg_button

use crate::graphics::colors::get_color;
use crate::style::styling::IpgStyleStandard;
use crate::{access_callbacks, access_user_data2, access_user_data1, app};
use super::helpers::{get_height, get_horizontal_alignment, get_padding_f64, get_radius, get_vertical_alignment, get_width, try_extract_boolean, try_extract_f32, try_extract_f64, try_extract_ipg_color, try_extract_ipg_horizontal_alignment, try_extract_ipg_vertical_alignment, try_extract_rgba_color, try_extract_string, try_extract_style_standard, try_extract_vec_f32, try_extract_vec_f64};
use super::ipg_enums::IpgWidgets;

use iced::widget::button::{self, Status, Style};
use pyo3::{pyclass, PyObject, Python};

use iced::widget::{text, Button, Text};
use iced::{alignment, Border, Color, Element, Length, Padding, Shadow, Theme, Vector };

use crate::graphics::bootstrap::{self, icon_to_char, icon_to_string};


#[derive(Debug, Clone)]
pub struct IpgButton {
    pub id: usize,
    pub parent_id: String,
    pub show: bool,

    pub label: String,
    pub width: Length,
    pub height: Length,
    pub padding: Padding,
    pub text_align_x: alignment::Horizontal,
    pub text_align_y: alignment::Vertical,
    pub text_size: f32,
    pub clip: bool,
    pub style_id: Option<usize>,
    pub style_standard: Option<IpgStyleStandard>,
    pub style_arrow: Option<IpgButtonArrow>,
}

impl IpgButton {
    pub fn new( 
        id: usize,
        parent_id: String,
        show: bool,

        label: String,
        width: Length,
        height: Length,
        padding: Padding,
        text_align_x: alignment::Horizontal,
        text_align_y: alignment::Vertical,
        text_size: f32,
        clip: bool,
        style_id: Option<usize>,
        style_standard: Option<IpgStyleStandard>,
        style_arrow: Option<IpgButtonArrow>,
        ) -> Self {
        Self {
            id,
            parent_id,
            show,
            label,
            width,
            height,
            padding,
            text_align_x,
            text_align_y,
            text_size,
            clip,
            style_id,
            style_standard,
            style_arrow,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct IpgButtonStyle {
    pub id: usize,
    pub background_color: Option<Color>,
    pub background_color_hovered: Option<Color>,
    pub border_color: Option<Color>,
    pub border_radius: Vec<f32>,
    pub border_width: f32,
    pub shadow_color: Option<Color>,
    pub shadow_offset_x: f32,
    pub shadow_offset_y: f32,
    pub shadow_blur_radius: f32,
    pub text_color: Option<Color>,
}

impl IpgButtonStyle {
    pub fn new(
        id: usize,
        background_color: Option<Color>,
        background_color_hovered: Option<Color>,
        border_color: Option<Color>,
        border_radius: Vec<f32>,
        border_width: f32,
        shadow_color: Option<Color>,
        shadow_offset_x: f32,
        shadow_offset_y: f32,
        shadow_blur_radius: f32,
        text_color: Option<Color>,
    ) -> Self {
        Self {
            id,
            background_color,
            background_color_hovered,
            border_color,
            border_radius,
            border_width,
            shadow_color,
            shadow_offset_x,
            shadow_offset_y,
            shadow_blur_radius,
            text_color,
        }
    }
}

#[derive(Debug, Clone)]
pub enum BTNMessage {
    OnPress,
}


pub fn construct_button<'a>(btn: &'a IpgButton, 
                        style_opt: Option<&IpgWidgets>) 
                        -> Option<Element<'a, app::Message>> {

    if !btn.show {
        return None
    }

    let style = get_btn_style(style_opt);

    let mut label = text(btn.label.clone())
                                                        .align_x(btn.text_align_x)
                                                        .align_y(btn.text_align_y)
                                                        .size(btn.text_size);

    if btn.style_arrow.is_some() {
        let arrow = get_bootstrap_arrow(&btn.style_arrow);
        label = Text::new(arrow).font(iced::Font::with_name("bootstrap-icons"));
    }
    
    let ipg_btn: Element<BTNMessage> = Button::new(label)
                                .height(btn.height)
                                .padding(btn.padding)
                                .width(btn.width)
                                .on_press(BTNMessage::OnPress)
                                .clip(btn.clip)
                                .style(move|theme: &Theme, status| {   
                                    get_styling(theme, status,
                                        style.clone(),
                                        btn.style_standard.clone(),
                                    )  
                                    })
                                .into();

    Some(ipg_btn.map(move |message| app::Message::Button(btn.id, message)))
    
}


pub fn button_callback(id: usize, message: BTNMessage) {

    match message {
        BTNMessage::OnPress => {
            process_callback(id, "on_press".to_string());
        }
    }
}


pub fn process_callback(
        id: usize, 
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
            if let Err(err) = callback.call1(py, (id, user_data)) {
                panic!("Button callback error: {err}");
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
            if let Err(err) = callback.call1(py, (id, user_data)) {
                panic!("Button callback error: {err}");
            }
        });
        drop(ud2); // Drop ud2 after processing
        return;
    }
    drop(ud2); // Drop ud2 if no user data is found

    // If no user data is found in both ud1 and ud2, call the callback with only the id
    Python::with_gil(|py| {
        if let Err(err) = callback.call1(py, (id,)) {
            panic!("Button callback error: {err}");
        }
    });
}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgButtonParam {
    ArrowStyle,
    Height,
    HeightFill,
    Label,
    Padding,
    Clip,
    Show,
    StyleId,
    StyleStandard,
    TextAlignX,
    TextAlignY,
    TextSize,
    Width,
    WidthFill,
}


pub fn button_item_update(btn: &mut IpgButton,
                            item: &PyObject,
                            value: &PyObject,
                            )
{
    let update = try_extract_button_update(item);
    let name = "Button".to_string();
    match update {
       IpgButtonParam::ArrowStyle => {
            btn.style_arrow = Some(try_extract_button_arrow(value));
        },
        IpgButtonParam::Label => {
            btn.label = try_extract_string(value, name);
        },
        IpgButtonParam::Height => {
            let val = try_extract_f64(value, name);
            btn.height = get_height(Some(val as f32), false);
        },
        IpgButtonParam::HeightFill => {
            let val = try_extract_boolean(value, name);
            btn.height = get_height(None, val);
        },
        IpgButtonParam::Padding => {
            btn.padding =  get_padding_f64(try_extract_vec_f64(value, name));
        },
        IpgButtonParam::Clip => {
            btn.clip = try_extract_boolean(value, name);
        }
        IpgButtonParam::Show => {
            btn.show = try_extract_boolean(value, name);
        },
        IpgButtonParam::StyleId => {
            btn.style_id = Some(try_extract_f64(value, name) as usize);
        },
        IpgButtonParam::StyleStandard => {
            btn.style_standard = Some(try_extract_style_standard(value, name));
        },
        IpgButtonParam::TextAlignX => {
            let h_align = try_extract_ipg_horizontal_alignment(value).unwrap();
            btn.text_align_x = get_horizontal_alignment(&h_align);
        },
        IpgButtonParam::TextAlignY => {
            let v_align = try_extract_ipg_vertical_alignment(value).unwrap();
            btn.text_align_y= get_vertical_alignment(&v_align);
        },
        IpgButtonParam::TextSize => {
            btn.text_size = try_extract_f32(value, name);
        },
        IpgButtonParam::Width => {
            let val = try_extract_f64(value, name);
            btn.width = get_width(Some(val as f32), false);
        },
        IpgButtonParam::WidthFill => {
            let val = try_extract_boolean(value, name);
            btn.width = get_width(None, val);
        },
    }

}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgButtonStyleParam {
    BackgroundIpgColor,
    BackgroundRbgaColor,
    BackgroundIpgColorHovered,
    BackgroundIpgRgbaHovered,
    BorderIpgColor,
    BorderRgbaColor,
    BorderRadius,
    BorderWidth,
    ShadowIpgColor,
    ShadowRgbaColor,
    ShadowOffsetX,
    ShadowOffsetY,
    ShadowBlurRadius,
    TextIpgColor,
    TextRgbaColor,
}

pub fn button_style_update_item(style: &mut IpgButtonStyle,
                                item: &PyObject,
                                value: &PyObject,) 
{
    let update = try_extract_button_style_update(item);
    let name = "ButtonStyle".to_string();
    match update {
        IpgButtonStyleParam::BackgroundIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.background_color = get_color(None, Some(color), 1.0, false);
        },
        IpgButtonStyleParam::BackgroundRbgaColor => {
            style.background_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgButtonStyleParam::BackgroundIpgColorHovered => {
            let color = try_extract_ipg_color(value, name);
            style.background_color_hovered = get_color(None, Some(color), 1.0, false);
        },
        IpgButtonStyleParam::BackgroundIpgRgbaHovered => {
            style.background_color_hovered = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgButtonStyleParam::BorderIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.border_color = get_color(None, Some(color), 1.0, false);
        },
        IpgButtonStyleParam::BorderRgbaColor => {
            style.border_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgButtonStyleParam::BorderRadius => {
            style.border_radius = try_extract_vec_f32(value, name);
        },
        IpgButtonStyleParam::BorderWidth => {
            style.border_width = try_extract_f64(value, name) as f32;
        },
        IpgButtonStyleParam::ShadowIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.shadow_color = get_color(None, Some(color), 1.0, false);
        },
        IpgButtonStyleParam::ShadowRgbaColor => {
            style.border_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgButtonStyleParam::ShadowOffsetX => {
            style.shadow_offset_x = try_extract_f64(value, name) as f32;
        },
        IpgButtonStyleParam::ShadowOffsetY => {
            style.shadow_offset_y = try_extract_f64(value, name) as f32;
        },
        IpgButtonStyleParam::ShadowBlurRadius => {
            style.shadow_blur_radius = try_extract_f64(value, name) as f32;
        },
        IpgButtonStyleParam::TextIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.text_color = get_color(None, Some(color), 1.0, false);
        },
        IpgButtonStyleParam::TextRgbaColor => {
            style.text_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
    }
}

pub fn get_btn_style(style: Option<&IpgWidgets>) -> Option<IpgButtonStyle>{
    match style {
        Some(IpgWidgets::IpgButtonStyle(style)) => {
            Some(style.clone())
        }
        _ => None,
    }
}

pub fn get_standard_style(theme: &Theme, status: Status, 
                            std_style: Option<IpgStyleStandard>,
                            border: Option<Border>, shadow: Option<Shadow>) -> Style {

    match std_style {
        Some(IpgStyleStandard::Primary) => {
            let mut style = button::primary(theme, status);
            if border.is_some() {
                style.border = border.unwrap();
            }
            if shadow.is_some() {
                style.shadow = shadow.unwrap();
            }
            style
        },
        Some(IpgStyleStandard::Success) => {
            let mut style = button::success(theme, status);
            if border.is_some() {
                style.border = border.unwrap();
            }
            if shadow.is_some() {
                style.shadow = shadow.unwrap();
            }
            style
        },
        Some(IpgStyleStandard::Danger) => {
            let mut style = button::danger(theme, status);
            if border.is_some() {
                style.border = border.unwrap();
            }
            if shadow.is_some() {
                style.shadow = shadow.unwrap();
            }
            style
        },
        Some(IpgStyleStandard::Text) => {
            button::text(theme, status)
        },
        None => {
            let mut style = button::primary(theme, status);
            if border.is_some() {
                style.border = border.unwrap();
            }
            if shadow.is_some() {
                style.shadow = shadow.unwrap();
            }
            style
        }
    }
}

pub fn get_styling(theme: &Theme, status: Status,
                    style_opt: Option<IpgButtonStyle>,
                    style_standard: Option<IpgStyleStandard>,
                    ) -> button::Style 
{
    if style_standard.is_none() && style_opt.is_none() {
        return button::primary(theme, status)
    }

    if style_opt.is_none() && style_standard.is_some() {
            return get_standard_style(theme, status, style_standard, None, None)
    }

    let mut border = Border::default();
    let mut shadow = Shadow::default();

    let mut base_style = button::primary(theme, status);
    let mut hover_style = button::primary(theme, status);

    let style = style_opt.unwrap_or_default();

    if style.border_color.is_some() {
        border.color = style.border_color.unwrap();
    }

    border.radius = get_radius(style.border_radius.clone(), 
                                "Button".to_string());
    border.width = style.border_width;

    if style.shadow_color.is_some() {
        shadow.color = style.shadow_color.unwrap();
        shadow.offset = Vector{ x: style.shadow_offset_x, y: style.shadow_offset_y };
        shadow.blur_radius = style.shadow_blur_radius;
    }

    // style_standard overrides style except for border and shadow
    let style_standard = get_standard_style(theme, status, 
                                    style_standard, 
                                    Some(border), Some(shadow));
    
    base_style.background = if style.background_color.is_some() {
        Some(style.background_color.unwrap().into())
    } else {
        style_standard.background
    };

    hover_style.background = if style.background_color_hovered.is_some() {
        Some(style.background_color_hovered.unwrap().into())
    } else {
        style_standard.background
    };

    base_style.border = border;
    hover_style.border = border;

    base_style.shadow = shadow;
    hover_style.shadow = shadow;

    if style.text_color.is_some() {
        base_style.text_color = style.text_color.unwrap();
        hover_style.text_color = style.text_color.unwrap();
    }

    match status {
        Status::Active | Status::Pressed => base_style,
        Status::Hovered => hover_style,
        Status::Disabled => disabled(base_style),
    }
    
}

fn disabled(style: Style) -> Style {
    Style {
        background: style
            .background
            .map(|background| background.scale_alpha(0.5)),
        text_color: style.text_color.scale_alpha(0.5),
        ..style
    }
}

pub fn try_extract_button_update(update_obj: &PyObject) -> IpgButtonParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgButtonParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Button update extraction failed"),
        }
    })
}

pub fn try_extract_button_arrow(update_obj: &PyObject) -> IpgButtonArrow {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgButtonArrow>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Button arrow extraction failed"),
        }
    })
}

pub fn try_extract_button_style_update(update_obj: &PyObject) -> IpgButtonStyleParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgButtonStyleParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Button style update extraction failed"),
        }
    })
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgButtonArrow {
    ArrowBarLeft,
    ArrowBarRight,
    ArrowBarUp,
    ArrowClockwise,
    ArrowCounterclockwise,
    ArrowDown,
    ArrowDownCircle,
    ArrowDownCircleFill,
    ArrowDownLeft,
    ArrowDownLeftCircle,
    ArrowDownLeftCircleFill,
    ArrowDownLeftSquare,
    ArrowDownLeftSquareFill,
    ArrowDownRight,
    ArrowDownRightCircle,
    ArrowDownRightCircleFill,
    ArrowDownRightSquare,
    ArrowDownRightSquareFill,
    ArrowDownShort,
    ArrowDownSquare,
    ArrowDownSquareFill,
    ArrowDownUp,
    ArrowLeft,
    ArrowLeftCircle,
    ArrowLeftCircleFill,
    ArrowLeftRight,
    ArrowLeftShort,
    ArrowLeftSquare,
    ArrowLeftSquareFill,
    ArrowNinezerodegDown,
    ArrowNinezerodegLeft,
    ArrowNinezerodegRight,
    ArrowNinezerodegUp,
    ArrowRepeat,
    ArrowReturnLeft,
    ArrowReturnRight,
    ArrowRight,
    ArrowRightCircle,
    ArrowRightCircleFill,
    ArrowRightShort,
    ArrowRightSquare,
    ArrowRightSquareFill,
    ArrowThroughHeart,
    ArrowThroughHeartFill,
    ArrowUp,
    ArrowUpCircle,
    ArrowUpCircleFill,
    ArrowUpLeft,
    ArrowUpLeftCircle,
    ArrowUpLeftCircleFill,
    ArrowUpLeftSquare,
    ArrowUpLeftSquareFill,
    ArrowUpRight,
    ArrowUpRightCircle,
    ArrowUpRightCircleFill,
    ArrowUpRightSquare,
    ArrowUpRightSquareFill,
    ArrowUpShort,
    ArrowUpSquare,
    ArrowUpSquareFill,
    Arrows,
    ArrowsAngleContract,
    ArrowsAngleExpand,
    ArrowsCollapse,
    ArrowsCollapseVertical,
    ArrowsExpand,
    ArrowsExpandVertical,
    ArrowsFullscreen,
    ArrowsMove,
    ArrowsVertical,
}


pub fn get_bootstrap_arrow(arrow: &Option<IpgButtonArrow>) -> String {
    match arrow {
        &None => unreachable!(),
        Some(IpgButtonArrow::ArrowBarLeft) => icon_to_string(bootstrap::Bootstrap::ArrowBarLeft),
        Some(IpgButtonArrow::ArrowBarRight) => icon_to_string(bootstrap::Bootstrap::ArrowBarRight),
        Some(IpgButtonArrow::ArrowBarUp) => icon_to_string(bootstrap::Bootstrap::ArrowBarUp),
        Some(IpgButtonArrow::ArrowClockwise) => icon_to_string(bootstrap::Bootstrap::ArrowClockwise),
        Some(IpgButtonArrow::ArrowCounterclockwise) => icon_to_string(bootstrap::Bootstrap::ArrowCounterclockwise),
        Some(IpgButtonArrow::ArrowDown) => icon_to_string(bootstrap::Bootstrap::ArrowDown),
        Some(IpgButtonArrow::ArrowDownCircle) => icon_to_string(bootstrap::Bootstrap::ArrowDownCircle),
        Some(IpgButtonArrow::ArrowDownCircleFill) => icon_to_string(bootstrap::Bootstrap::ArrowDownCircleFill),
        Some(IpgButtonArrow::ArrowDownLeft) => icon_to_string(bootstrap::Bootstrap::ArrowDownLeft),
        Some(IpgButtonArrow::ArrowDownLeftCircle) => icon_to_string(bootstrap::Bootstrap::ArrowDownLeftCircle),
        Some(IpgButtonArrow::ArrowDownLeftCircleFill) => icon_to_string(bootstrap::Bootstrap::ArrowDownLeftCircleFill),
        Some(IpgButtonArrow::ArrowDownLeftSquare) => icon_to_string(bootstrap::Bootstrap::ArrowDownLeftSquare),
        Some(IpgButtonArrow::ArrowDownLeftSquareFill) => icon_to_string(bootstrap::Bootstrap::ArrowDownLeftSquareFill),
        Some(IpgButtonArrow::ArrowDownRight) => icon_to_string(bootstrap::Bootstrap::ArrowDownRight),
        Some(IpgButtonArrow::ArrowDownRightCircle) => icon_to_string(bootstrap::Bootstrap::ArrowDownRightCircle),
        Some(IpgButtonArrow::ArrowDownRightCircleFill) => icon_to_string(bootstrap::Bootstrap::ArrowDownRightCircleFill),
        Some(IpgButtonArrow::ArrowDownRightSquare) => icon_to_string(bootstrap::Bootstrap::ArrowDownRightSquare),
        Some(IpgButtonArrow::ArrowDownRightSquareFill) => icon_to_string(bootstrap::Bootstrap::ArrowDownRightSquareFill),
        Some(IpgButtonArrow::ArrowDownShort) => icon_to_string(bootstrap::Bootstrap::ArrowDownShort),
        Some(IpgButtonArrow::ArrowDownSquare) => icon_to_string(bootstrap::Bootstrap::ArrowDownSquare),
        Some(IpgButtonArrow::ArrowDownSquareFill) => icon_to_string(bootstrap::Bootstrap::ArrowDownSquareFill),
        Some(IpgButtonArrow::ArrowDownUp) => icon_to_string(bootstrap::Bootstrap::ArrowDownUp),
        Some(IpgButtonArrow::ArrowLeft) => icon_to_string(bootstrap::Bootstrap::ArrowLeft),
        Some(IpgButtonArrow::ArrowLeftCircle) => icon_to_string(bootstrap::Bootstrap::ArrowLeftCircle),
        Some(IpgButtonArrow::ArrowLeftCircleFill) => icon_to_string(bootstrap::Bootstrap::ArrowLeftCircleFill),
        Some(IpgButtonArrow::ArrowLeftRight) => icon_to_string(bootstrap::Bootstrap::ArrowLeftRight),
        Some(IpgButtonArrow::ArrowLeftShort) => icon_to_string(bootstrap::Bootstrap::ArrowLeftShort),
        Some(IpgButtonArrow::ArrowLeftSquare) => icon_to_string(bootstrap::Bootstrap::ArrowLeftSquare),
        Some(IpgButtonArrow::ArrowLeftSquareFill) => icon_to_string(bootstrap::Bootstrap::ArrowLeftSquareFill),
        Some(IpgButtonArrow::ArrowNinezerodegDown) => icon_to_string(bootstrap::Bootstrap::ArrowNinezerodegDown),
        Some(IpgButtonArrow::ArrowNinezerodegLeft) => icon_to_string(bootstrap::Bootstrap::ArrowNinezerodegLeft),
        Some(IpgButtonArrow::ArrowNinezerodegRight) => icon_to_string(bootstrap::Bootstrap::ArrowNinezerodegRight),
        Some(IpgButtonArrow::ArrowNinezerodegUp) => icon_to_string(bootstrap::Bootstrap::ArrowNinezerodegUp),
        Some(IpgButtonArrow::ArrowRepeat) => icon_to_string(bootstrap::Bootstrap::ArrowRepeat),
        Some(IpgButtonArrow::ArrowReturnLeft) => icon_to_string(bootstrap::Bootstrap::ArrowReturnLeft),
        Some(IpgButtonArrow::ArrowReturnRight) => icon_to_string(bootstrap::Bootstrap::ArrowReturnRight),
        Some(IpgButtonArrow::ArrowRight) => icon_to_string(bootstrap::Bootstrap::ArrowRight),
        Some(IpgButtonArrow::ArrowRightCircle) => icon_to_string(bootstrap::Bootstrap::ArrowRightCircle),
        Some(IpgButtonArrow::ArrowRightCircleFill) => icon_to_string(bootstrap::Bootstrap::ArrowRightCircleFill),
        Some(IpgButtonArrow::ArrowRightShort) => icon_to_string(bootstrap::Bootstrap::ArrowRightShort),
        Some(IpgButtonArrow::ArrowRightSquare) => icon_to_string(bootstrap::Bootstrap::ArrowRightSquare),
        Some(IpgButtonArrow::ArrowRightSquareFill) => icon_to_string(bootstrap::Bootstrap::ArrowRightSquareFill),
        Some(IpgButtonArrow::ArrowThroughHeart) => icon_to_string(bootstrap::Bootstrap::ArrowThroughHeart),
        Some(IpgButtonArrow::ArrowThroughHeartFill) => icon_to_string(bootstrap::Bootstrap::ArrowThroughHeartFill),
        Some(IpgButtonArrow::ArrowUp) => icon_to_string(bootstrap::Bootstrap::ArrowUp),
        Some(IpgButtonArrow::ArrowUpCircle) => icon_to_string(bootstrap::Bootstrap::ArrowUpCircle),
        Some(IpgButtonArrow::ArrowUpCircleFill) => icon_to_string(bootstrap::Bootstrap::ArrowUpCircleFill),
        Some(IpgButtonArrow::ArrowUpLeft) => icon_to_string(bootstrap::Bootstrap::ArrowUpLeft),
        Some(IpgButtonArrow::ArrowUpLeftCircle) => icon_to_string(bootstrap::Bootstrap::ArrowUpLeftCircle),
        Some(IpgButtonArrow::ArrowUpLeftCircleFill) => icon_to_string(bootstrap::Bootstrap::ArrowUpLeftCircleFill),
        Some(IpgButtonArrow::ArrowUpLeftSquare) => icon_to_string(bootstrap::Bootstrap::ArrowUpLeftSquare),
        Some(IpgButtonArrow::ArrowUpLeftSquareFill) => icon_to_string(bootstrap::Bootstrap::ArrowUpLeftSquareFill),
        Some(IpgButtonArrow::ArrowUpRight) => icon_to_string(bootstrap::Bootstrap::ArrowUpRight),
        Some(IpgButtonArrow::ArrowUpRightCircle) => icon_to_string(bootstrap::Bootstrap::ArrowUpRightCircle),
        Some(IpgButtonArrow::ArrowUpRightCircleFill) => icon_to_string(bootstrap::Bootstrap::ArrowUpRightCircleFill),
        Some(IpgButtonArrow::ArrowUpRightSquare) => icon_to_string(bootstrap::Bootstrap::ArrowUpRightSquare),
        Some(IpgButtonArrow::ArrowUpRightSquareFill) => icon_to_string(bootstrap::Bootstrap::ArrowUpRightSquareFill),
        Some(IpgButtonArrow::ArrowUpShort) => icon_to_string(bootstrap::Bootstrap::ArrowUpShort),
        Some(IpgButtonArrow::ArrowUpSquare) => icon_to_string(bootstrap::Bootstrap::ArrowUpSquare),
        Some(IpgButtonArrow::ArrowUpSquareFill) => icon_to_string(bootstrap::Bootstrap::ArrowUpSquareFill),
        Some(IpgButtonArrow::Arrows) => icon_to_string(bootstrap::Bootstrap::Arrows),
        Some(IpgButtonArrow::ArrowsAngleContract) => icon_to_string(bootstrap::Bootstrap::ArrowsAngleContract),
        Some(IpgButtonArrow::ArrowsAngleExpand) => icon_to_string(bootstrap::Bootstrap::ArrowsAngleExpand),
        Some(IpgButtonArrow::ArrowsCollapse) => icon_to_string(bootstrap::Bootstrap::ArrowsCollapse),
        Some(IpgButtonArrow::ArrowsCollapseVertical) => icon_to_string(bootstrap::Bootstrap::ArrowsCollapseVertical),
        Some(IpgButtonArrow::ArrowsExpand) => icon_to_string(bootstrap::Bootstrap::ArrowsExpand),
        Some(IpgButtonArrow::ArrowsExpandVertical) => icon_to_string(bootstrap::Bootstrap::ArrowsExpandVertical),
        Some(IpgButtonArrow::ArrowsFullscreen) => icon_to_string(bootstrap::Bootstrap::ArrowsFullscreen),
        Some(IpgButtonArrow::ArrowsMove) => icon_to_string(bootstrap::Bootstrap::ArrowsMove),
        Some(IpgButtonArrow::ArrowsVertical) => icon_to_string(bootstrap::Bootstrap::ArrowsVertical),
    }
}

pub fn get_bootstrap_arrow_char(arrow: &IpgButtonArrow) -> char {
    match arrow {
        IpgButtonArrow::ArrowBarLeft => icon_to_char(bootstrap::Bootstrap::ArrowBarLeft),
        IpgButtonArrow::ArrowBarRight => icon_to_char(bootstrap::Bootstrap::ArrowBarRight),
        IpgButtonArrow::ArrowBarUp => icon_to_char(bootstrap::Bootstrap::ArrowBarUp),
        IpgButtonArrow::ArrowClockwise => icon_to_char(bootstrap::Bootstrap::ArrowClockwise),
        IpgButtonArrow::ArrowCounterclockwise => icon_to_char(bootstrap::Bootstrap::ArrowCounterclockwise),
        IpgButtonArrow::ArrowDown => icon_to_char(bootstrap::Bootstrap::ArrowDown),
        IpgButtonArrow::ArrowDownCircle => icon_to_char(bootstrap::Bootstrap::ArrowDownCircle),
        IpgButtonArrow::ArrowDownCircleFill => icon_to_char(bootstrap::Bootstrap::ArrowDownCircleFill),
        IpgButtonArrow::ArrowDownLeft => icon_to_char(bootstrap::Bootstrap::ArrowDownLeft),
        IpgButtonArrow::ArrowDownLeftCircle => icon_to_char(bootstrap::Bootstrap::ArrowDownLeftCircle),
        IpgButtonArrow::ArrowDownLeftCircleFill => icon_to_char(bootstrap::Bootstrap::ArrowDownLeftCircleFill),
        IpgButtonArrow::ArrowDownLeftSquare => icon_to_char(bootstrap::Bootstrap::ArrowDownLeftSquare),
        IpgButtonArrow::ArrowDownLeftSquareFill => icon_to_char(bootstrap::Bootstrap::ArrowDownLeftSquareFill),
        IpgButtonArrow::ArrowDownRight => icon_to_char(bootstrap::Bootstrap::ArrowDownRight),
        IpgButtonArrow::ArrowDownRightCircle => icon_to_char(bootstrap::Bootstrap::ArrowDownRightCircle),
        IpgButtonArrow::ArrowDownRightCircleFill => icon_to_char(bootstrap::Bootstrap::ArrowDownRightCircleFill),
        IpgButtonArrow::ArrowDownRightSquare => icon_to_char(bootstrap::Bootstrap::ArrowDownRightSquare),
        IpgButtonArrow::ArrowDownRightSquareFill => icon_to_char(bootstrap::Bootstrap::ArrowDownRightSquareFill),
        IpgButtonArrow::ArrowDownShort => icon_to_char(bootstrap::Bootstrap::ArrowDownShort),
        IpgButtonArrow::ArrowDownSquare => icon_to_char(bootstrap::Bootstrap::ArrowDownSquare),
        IpgButtonArrow::ArrowDownSquareFill => icon_to_char(bootstrap::Bootstrap::ArrowDownSquareFill),
        IpgButtonArrow::ArrowDownUp => icon_to_char(bootstrap::Bootstrap::ArrowDownUp),
        IpgButtonArrow::ArrowLeft => icon_to_char(bootstrap::Bootstrap::ArrowLeft),
        IpgButtonArrow::ArrowLeftCircle => icon_to_char(bootstrap::Bootstrap::ArrowLeftCircle),
        IpgButtonArrow::ArrowLeftCircleFill => icon_to_char(bootstrap::Bootstrap::ArrowLeftCircleFill),
        IpgButtonArrow::ArrowLeftRight => icon_to_char(bootstrap::Bootstrap::ArrowLeftRight),
        IpgButtonArrow::ArrowLeftShort => icon_to_char(bootstrap::Bootstrap::ArrowLeftShort),
        IpgButtonArrow::ArrowLeftSquare => icon_to_char(bootstrap::Bootstrap::ArrowLeftSquare),
        IpgButtonArrow::ArrowLeftSquareFill => icon_to_char(bootstrap::Bootstrap::ArrowLeftSquareFill),
        IpgButtonArrow::ArrowNinezerodegDown => icon_to_char(bootstrap::Bootstrap::ArrowNinezerodegDown),
        IpgButtonArrow::ArrowNinezerodegLeft => icon_to_char(bootstrap::Bootstrap::ArrowNinezerodegLeft),
        IpgButtonArrow::ArrowNinezerodegRight => icon_to_char(bootstrap::Bootstrap::ArrowNinezerodegRight),
        IpgButtonArrow::ArrowNinezerodegUp => icon_to_char(bootstrap::Bootstrap::ArrowNinezerodegUp),
        IpgButtonArrow::ArrowRepeat => icon_to_char(bootstrap::Bootstrap::ArrowRepeat),
        IpgButtonArrow::ArrowReturnLeft => icon_to_char(bootstrap::Bootstrap::ArrowReturnLeft),
        IpgButtonArrow::ArrowReturnRight => icon_to_char(bootstrap::Bootstrap::ArrowReturnRight),
        IpgButtonArrow::ArrowRight => icon_to_char(bootstrap::Bootstrap::ArrowRight),
        IpgButtonArrow::ArrowRightCircle => icon_to_char(bootstrap::Bootstrap::ArrowRightCircle),
        IpgButtonArrow::ArrowRightCircleFill => icon_to_char(bootstrap::Bootstrap::ArrowRightCircleFill),
        IpgButtonArrow::ArrowRightShort => icon_to_char(bootstrap::Bootstrap::ArrowRightShort),
        IpgButtonArrow::ArrowRightSquare => icon_to_char(bootstrap::Bootstrap::ArrowRightSquare),
        IpgButtonArrow::ArrowRightSquareFill => icon_to_char(bootstrap::Bootstrap::ArrowRightSquareFill),
        IpgButtonArrow::ArrowThroughHeart => icon_to_char(bootstrap::Bootstrap::ArrowThroughHeart),
        IpgButtonArrow::ArrowThroughHeartFill => icon_to_char(bootstrap::Bootstrap::ArrowThroughHeartFill),
        IpgButtonArrow::ArrowUp => icon_to_char(bootstrap::Bootstrap::ArrowUp),
        IpgButtonArrow::ArrowUpCircle => icon_to_char(bootstrap::Bootstrap::ArrowUpCircle),
        IpgButtonArrow::ArrowUpCircleFill => icon_to_char(bootstrap::Bootstrap::ArrowUpCircleFill),
        IpgButtonArrow::ArrowUpLeft => icon_to_char(bootstrap::Bootstrap::ArrowUpLeft),
        IpgButtonArrow::ArrowUpLeftCircle => icon_to_char(bootstrap::Bootstrap::ArrowUpLeftCircle),
        IpgButtonArrow::ArrowUpLeftCircleFill => icon_to_char(bootstrap::Bootstrap::ArrowUpLeftCircleFill),
        IpgButtonArrow::ArrowUpLeftSquare => icon_to_char(bootstrap::Bootstrap::ArrowUpLeftSquare),
        IpgButtonArrow::ArrowUpLeftSquareFill => icon_to_char(bootstrap::Bootstrap::ArrowUpLeftSquareFill),
        IpgButtonArrow::ArrowUpRight => icon_to_char(bootstrap::Bootstrap::ArrowUpRight),
        IpgButtonArrow::ArrowUpRightCircle => icon_to_char(bootstrap::Bootstrap::ArrowUpRightCircle),
        IpgButtonArrow::ArrowUpRightCircleFill => icon_to_char(bootstrap::Bootstrap::ArrowUpRightCircleFill),
        IpgButtonArrow::ArrowUpRightSquare => icon_to_char(bootstrap::Bootstrap::ArrowUpRightSquare),
        IpgButtonArrow::ArrowUpRightSquareFill => icon_to_char(bootstrap::Bootstrap::ArrowUpRightSquareFill),
        IpgButtonArrow::ArrowUpShort => icon_to_char(bootstrap::Bootstrap::ArrowUpShort),
        IpgButtonArrow::ArrowUpSquare => icon_to_char(bootstrap::Bootstrap::ArrowUpSquare),
        IpgButtonArrow::ArrowUpSquareFill => icon_to_char(bootstrap::Bootstrap::ArrowUpSquareFill),
        IpgButtonArrow::Arrows => icon_to_char(bootstrap::Bootstrap::Arrows),
        IpgButtonArrow::ArrowsAngleContract => icon_to_char(bootstrap::Bootstrap::ArrowsAngleContract),
        IpgButtonArrow::ArrowsAngleExpand => icon_to_char(bootstrap::Bootstrap::ArrowsAngleExpand),
        IpgButtonArrow::ArrowsCollapse => icon_to_char(bootstrap::Bootstrap::ArrowsCollapse),
        IpgButtonArrow::ArrowsCollapseVertical => icon_to_char(bootstrap::Bootstrap::ArrowsCollapseVertical),
        IpgButtonArrow::ArrowsExpand => icon_to_char(bootstrap::Bootstrap::ArrowsExpand),
        IpgButtonArrow::ArrowsExpandVertical => icon_to_char(bootstrap::Bootstrap::ArrowsExpandVertical),
        IpgButtonArrow::ArrowsFullscreen => icon_to_char(bootstrap::Bootstrap::ArrowsFullscreen),
        IpgButtonArrow::ArrowsMove => icon_to_char(bootstrap::Bootstrap::ArrowsMove),
        IpgButtonArrow::ArrowsVertical => icon_to_char(bootstrap::Bootstrap::ArrowsVertical),
    }
}
