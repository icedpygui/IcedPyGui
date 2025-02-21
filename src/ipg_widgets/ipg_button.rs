//! ipg_button

use crate::graphics::colors::get_color;
use crate::style::styling::IpgStyleStandard;
use crate::{access_callbacks, app, IpgState};
use super::helpers::{get_height, get_horizontal_alignment, get_padding_f64, get_radius, get_vertical_alignment, get_width, try_extract_boolean, try_extract_f64, try_extract_ipg_color, try_extract_ipg_horizontal_alignment, try_extract_ipg_vertical_alignment, try_extract_rgba_color, try_extract_string, try_extract_style_standard, try_extract_vec_f32, try_extract_vec_f64};
use super::callbacks::{
    set_or_get_widget_callback_data, WidgetCallbackIn, WidgetCallbackOut
};
use super::ipg_enums::IpgWidgets;

use iced::widget::button::{self, Status, Style};
use pyo3::{pyclass, PyObject, Python};

use iced::widget::{text, Button, Text};
use iced::{alignment, Border, Color, Element, Length, Padding, Shadow, Theme, Vector };

use crate::graphics::bootstrap::{self, icon_to_char, icon_to_string};


#[derive(Debug, Clone)]
pub struct IpgButton {
    pub id: usize,
    pub show: bool,
    pub user_data: Option<PyObject>,

    pub label: String,
    pub width: Length,
    pub height: Length,
    pub padding: Padding,
    pub text_align_x: alignment::Horizontal,
    pub text_align_y: alignment::Vertical,
    pub clip: bool,
    pub style_id: Option<usize>,
    pub style_standard: Option<IpgStyleStandard>,
    pub style_arrow: Option<IpgButtonArrow>,
}

impl IpgButton {
    pub fn new( 
        id: usize,
        show: bool,
        user_data: Option<PyObject>,

        label: String,
        width: Length,
        height: Length,
        padding: Padding,
        text_align_x: alignment::Horizontal,
        text_align_y: alignment::Vertical,
        clip: bool,
        style_id: Option<usize>,
        style_standard: Option<IpgStyleStandard>,
        style_arrow: Option<IpgButtonArrow>,
        ) -> Self {
        Self {
            id,
            show,
            user_data,
            label,
            width,
            height,
            padding,
            text_align_x,
            text_align_y,
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


pub fn construct_button(btn: IpgButton, 
                        style_opt: Option<IpgWidgets>) 
                        -> Option<Element<'static, app::Message>> {

    if !btn.show {
        return None
    }

    let style = get_btn_style(style_opt);

    let mut label = text(btn.label.clone())
                                                        .align_x(btn.text_align_x)
                                                        .align_y(btn.text_align_y);

    if btn.style_arrow.is_some() {
        let arrow = get_bootstrap_arrow(btn.style_arrow.unwrap());
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


pub fn button_callback(state: &mut IpgState, id: usize, message: BTNMessage) {

    let wci = WidgetCallbackIn{id, ..Default::default()};

    match message {
        BTNMessage::OnPress => {
            // getting only
            let mut wco: WidgetCallbackOut = set_or_get_widget_callback_data(state, wci);
            wco.id = id;
            wco.event_name = "on_press".to_string();
            process_callback(wco);
        }
    }
}


pub fn process_callback(wco: WidgetCallbackOut) 
{
    let app_cbs = access_callbacks();

    let callback_present = 
        app_cbs.callbacks.get(&(wco.id, wco.event_name.clone()));

    let callback_opt = match callback_present {
        Some(cb) => cb,
        None => return,
    };

    let callback = match callback_opt {
        Some(cb) => cb,
        None => panic!("Button callback could not be found with id {}", wco.id),
    };

    Python::with_gil(|py| {
            if wco.user_data.is_some() {
                let user_data = match wco.user_data {
                    Some(ud) => ud,
                    None => panic!("User Data could not be found in Button callback"),
                };
                let res = callback.call1(py, (
                                                                    wco.id,  
                                                                    user_data
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Button: 2 parameters (id, user_data) are required or a python error in this function. {er}"),
                }
            } else {
                let res = callback.call1(py, (
                                                                    wco.id,  
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Button: 1 parameter (id) is required or possibly a python error in this function. {er}"),
                }
            } 
    });
    
    drop(app_cbs);
         
}


#[derive(Debug, Clone)]
#[pyclass]
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
    Width,
    WidthFill,
}


pub fn button_item_update(btn: &mut IpgButton,
                            item: PyObject,
                            value: PyObject,
                            )
{
    let update = try_extract_button_update(item);

    match update {
       IpgButtonParam::ArrowStyle => {
            btn.style_arrow = Some(try_extract_button_arrow(value));
        },
        IpgButtonParam::Label => {
            btn.label = try_extract_string(value);
        },
        IpgButtonParam::Height => {
            let val = try_extract_f64(value);
            btn.height = get_height(Some(val as f32), false);
        },
        IpgButtonParam::HeightFill => {
            let val = try_extract_boolean(value);
            btn.height = get_height(None, val);
        },
        IpgButtonParam::Padding => {
            btn.padding =  get_padding_f64(try_extract_vec_f64(value));
        },
        IpgButtonParam::Clip => {
            btn.clip = try_extract_boolean(value);
        }
        IpgButtonParam::Show => {
            btn.show = try_extract_boolean(value);
        },
        IpgButtonParam::StyleId => {
            btn.style_id = Some(try_extract_f64(value) as usize);
        },
        IpgButtonParam::StyleStandard => {
            btn.style_standard = Some(try_extract_style_standard(value));
        },
        IpgButtonParam::TextAlignX => {
            let h_align = try_extract_ipg_horizontal_alignment(value).unwrap();
            btn.text_align_x = get_horizontal_alignment(h_align);
        },
        IpgButtonParam::TextAlignY => {
            let v_align = try_extract_ipg_vertical_alignment(value).unwrap();
            btn.text_align_y= get_vertical_alignment(v_align);
        },
        IpgButtonParam::Width => {
            let val = try_extract_f64(value);
            btn.width = get_width(Some(val as f32), false);
        },
        IpgButtonParam::WidthFill => {
            let val = try_extract_boolean(value);
            btn.width = get_width(None, val);
        },
    }

}

#[derive(Debug, Clone)]
#[pyclass]
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
                            item: PyObject,
                            value: PyObject,) 
{

    let update = try_extract_button_style_update(item);
    match update {
        IpgButtonStyleParam::BackgroundIpgColor => {
            let color = try_extract_ipg_color(value);
            style.background_color = get_color(None, Some(color), 1.0, false);
        },
        IpgButtonStyleParam::BackgroundRbgaColor => {
            style.background_color = Some(Color::from(try_extract_rgba_color(value)));
        },
        IpgButtonStyleParam::BackgroundIpgColorHovered => {
            let color = try_extract_ipg_color(value);
            style.background_color_hovered = get_color(None, Some(color), 1.0, false);
        },
        IpgButtonStyleParam::BackgroundIpgRgbaHovered => {
            style.background_color_hovered = Some(Color::from(try_extract_rgba_color(value)));
        },
        IpgButtonStyleParam::BorderIpgColor => {
            let color = try_extract_ipg_color(value);
            style.border_color = get_color(None, Some(color), 1.0, false);
        },
        IpgButtonStyleParam::BorderRgbaColor => {
            style.border_color = Some(Color::from(try_extract_rgba_color(value)));
        },
        IpgButtonStyleParam::BorderRadius => {
            style.border_radius = try_extract_vec_f32(value);
        },
        IpgButtonStyleParam::BorderWidth => {
            style.border_width = try_extract_f64(value) as f32;
        },
        IpgButtonStyleParam::ShadowIpgColor => {
            let color = try_extract_ipg_color(value);
            style.shadow_color = get_color(None, Some(color), 1.0, false);
        },
        IpgButtonStyleParam::ShadowRgbaColor => {
            style.border_color = Some(Color::from(try_extract_rgba_color(value)));
        },
        IpgButtonStyleParam::ShadowOffsetX => {
            style.shadow_offset_x = try_extract_f64(value) as f32;
        },
        IpgButtonStyleParam::ShadowOffsetY => {
            style.shadow_offset_y = try_extract_f64(value) as f32;
        },
        IpgButtonStyleParam::ShadowBlurRadius => {
            style.shadow_blur_radius = try_extract_f64(value) as f32;
        },
        IpgButtonStyleParam::TextIpgColor => {
            let color = try_extract_ipg_color(value);
            style.text_color = get_color(None, Some(color), 1.0, false);
        },
        IpgButtonStyleParam::TextRgbaColor => {
            style.text_color = Some(Color::from(try_extract_rgba_color(value)));
        },
    }
}

pub fn get_btn_style(style: Option<IpgWidgets>) -> Option<IpgButtonStyle>{
    match style {
        Some(st) => {
            match st {
                IpgWidgets::IpgButtonStyle(style) => {
                    Some(style)
                }
                _ => None,
            }
        },
        None => None,
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

pub fn try_extract_button_update(update_obj: PyObject) -> IpgButtonParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgButtonParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Button update extraction failed"),
        }
    })
}

pub fn try_extract_button_arrow(update_obj: PyObject) -> IpgButtonArrow {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgButtonArrow>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Button arrow extraction failed"),
        }
    })
}

pub fn try_extract_button_style_update(update_obj: PyObject) -> IpgButtonStyleParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgButtonStyleParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Button style update extraction failed"),
        }
    })
}

#[derive(Debug, Clone)]
#[pyclass]
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


pub fn get_bootstrap_arrow(arrow: IpgButtonArrow) -> String {
    match arrow {
        IpgButtonArrow::ArrowBarLeft => icon_to_string(bootstrap::Bootstrap::ArrowBarLeft),
        IpgButtonArrow::ArrowBarRight => icon_to_string(bootstrap::Bootstrap::ArrowBarRight),
        IpgButtonArrow::ArrowBarUp => icon_to_string(bootstrap::Bootstrap::ArrowBarUp),
        IpgButtonArrow::ArrowClockwise => icon_to_string(bootstrap::Bootstrap::ArrowClockwise),
        IpgButtonArrow::ArrowCounterclockwise => icon_to_string(bootstrap::Bootstrap::ArrowCounterclockwise),
        IpgButtonArrow::ArrowDown => icon_to_string(bootstrap::Bootstrap::ArrowDown),
        IpgButtonArrow::ArrowDownCircle => icon_to_string(bootstrap::Bootstrap::ArrowDownCircle),
        IpgButtonArrow::ArrowDownCircleFill => icon_to_string(bootstrap::Bootstrap::ArrowDownCircleFill),
        IpgButtonArrow::ArrowDownLeft => icon_to_string(bootstrap::Bootstrap::ArrowDownLeft),
        IpgButtonArrow::ArrowDownLeftCircle => icon_to_string(bootstrap::Bootstrap::ArrowDownLeftCircle),
        IpgButtonArrow::ArrowDownLeftCircleFill => icon_to_string(bootstrap::Bootstrap::ArrowDownLeftCircleFill),
        IpgButtonArrow::ArrowDownLeftSquare => icon_to_string(bootstrap::Bootstrap::ArrowDownLeftSquare),
        IpgButtonArrow::ArrowDownLeftSquareFill => icon_to_string(bootstrap::Bootstrap::ArrowDownLeftSquareFill),
        IpgButtonArrow::ArrowDownRight => icon_to_string(bootstrap::Bootstrap::ArrowDownRight),
        IpgButtonArrow::ArrowDownRightCircle => icon_to_string(bootstrap::Bootstrap::ArrowDownRightCircle),
        IpgButtonArrow::ArrowDownRightCircleFill => icon_to_string(bootstrap::Bootstrap::ArrowDownRightCircleFill),
        IpgButtonArrow::ArrowDownRightSquare => icon_to_string(bootstrap::Bootstrap::ArrowDownRightSquare),
        IpgButtonArrow::ArrowDownRightSquareFill => icon_to_string(bootstrap::Bootstrap::ArrowDownRightSquareFill),
        IpgButtonArrow::ArrowDownShort => icon_to_string(bootstrap::Bootstrap::ArrowDownShort),
        IpgButtonArrow::ArrowDownSquare => icon_to_string(bootstrap::Bootstrap::ArrowDownSquare),
        IpgButtonArrow::ArrowDownSquareFill => icon_to_string(bootstrap::Bootstrap::ArrowDownSquareFill),
        IpgButtonArrow::ArrowDownUp => icon_to_string(bootstrap::Bootstrap::ArrowDownUp),
        IpgButtonArrow::ArrowLeft => icon_to_string(bootstrap::Bootstrap::ArrowLeft),
        IpgButtonArrow::ArrowLeftCircle => icon_to_string(bootstrap::Bootstrap::ArrowLeftCircle),
        IpgButtonArrow::ArrowLeftCircleFill => icon_to_string(bootstrap::Bootstrap::ArrowLeftCircleFill),
        IpgButtonArrow::ArrowLeftRight => icon_to_string(bootstrap::Bootstrap::ArrowLeftRight),
        IpgButtonArrow::ArrowLeftShort => icon_to_string(bootstrap::Bootstrap::ArrowLeftShort),
        IpgButtonArrow::ArrowLeftSquare => icon_to_string(bootstrap::Bootstrap::ArrowLeftSquare),
        IpgButtonArrow::ArrowLeftSquareFill => icon_to_string(bootstrap::Bootstrap::ArrowLeftSquareFill),
        IpgButtonArrow::ArrowNinezerodegDown => icon_to_string(bootstrap::Bootstrap::ArrowNinezerodegDown),
        IpgButtonArrow::ArrowNinezerodegLeft => icon_to_string(bootstrap::Bootstrap::ArrowNinezerodegLeft),
        IpgButtonArrow::ArrowNinezerodegRight => icon_to_string(bootstrap::Bootstrap::ArrowNinezerodegRight),
        IpgButtonArrow::ArrowNinezerodegUp => icon_to_string(bootstrap::Bootstrap::ArrowNinezerodegUp),
        IpgButtonArrow::ArrowRepeat => icon_to_string(bootstrap::Bootstrap::ArrowRepeat),
        IpgButtonArrow::ArrowReturnLeft => icon_to_string(bootstrap::Bootstrap::ArrowReturnLeft),
        IpgButtonArrow::ArrowReturnRight => icon_to_string(bootstrap::Bootstrap::ArrowReturnRight),
        IpgButtonArrow::ArrowRight => icon_to_string(bootstrap::Bootstrap::ArrowRight),
        IpgButtonArrow::ArrowRightCircle => icon_to_string(bootstrap::Bootstrap::ArrowRightCircle),
        IpgButtonArrow::ArrowRightCircleFill => icon_to_string(bootstrap::Bootstrap::ArrowRightCircleFill),
        IpgButtonArrow::ArrowRightShort => icon_to_string(bootstrap::Bootstrap::ArrowRightShort),
        IpgButtonArrow::ArrowRightSquare => icon_to_string(bootstrap::Bootstrap::ArrowRightSquare),
        IpgButtonArrow::ArrowRightSquareFill => icon_to_string(bootstrap::Bootstrap::ArrowRightSquareFill),
        IpgButtonArrow::ArrowThroughHeart => icon_to_string(bootstrap::Bootstrap::ArrowThroughHeart),
        IpgButtonArrow::ArrowThroughHeartFill => icon_to_string(bootstrap::Bootstrap::ArrowThroughHeartFill),
        IpgButtonArrow::ArrowUp => icon_to_string(bootstrap::Bootstrap::ArrowUp),
        IpgButtonArrow::ArrowUpCircle => icon_to_string(bootstrap::Bootstrap::ArrowUpCircle),
        IpgButtonArrow::ArrowUpCircleFill => icon_to_string(bootstrap::Bootstrap::ArrowUpCircleFill),
        IpgButtonArrow::ArrowUpLeft => icon_to_string(bootstrap::Bootstrap::ArrowUpLeft),
        IpgButtonArrow::ArrowUpLeftCircle => icon_to_string(bootstrap::Bootstrap::ArrowUpLeftCircle),
        IpgButtonArrow::ArrowUpLeftCircleFill => icon_to_string(bootstrap::Bootstrap::ArrowUpLeftCircleFill),
        IpgButtonArrow::ArrowUpLeftSquare => icon_to_string(bootstrap::Bootstrap::ArrowUpLeftSquare),
        IpgButtonArrow::ArrowUpLeftSquareFill => icon_to_string(bootstrap::Bootstrap::ArrowUpLeftSquareFill),
        IpgButtonArrow::ArrowUpRight => icon_to_string(bootstrap::Bootstrap::ArrowUpRight),
        IpgButtonArrow::ArrowUpRightCircle => icon_to_string(bootstrap::Bootstrap::ArrowUpRightCircle),
        IpgButtonArrow::ArrowUpRightCircleFill => icon_to_string(bootstrap::Bootstrap::ArrowUpRightCircleFill),
        IpgButtonArrow::ArrowUpRightSquare => icon_to_string(bootstrap::Bootstrap::ArrowUpRightSquare),
        IpgButtonArrow::ArrowUpRightSquareFill => icon_to_string(bootstrap::Bootstrap::ArrowUpRightSquareFill),
        IpgButtonArrow::ArrowUpShort => icon_to_string(bootstrap::Bootstrap::ArrowUpShort),
        IpgButtonArrow::ArrowUpSquare => icon_to_string(bootstrap::Bootstrap::ArrowUpSquare),
        IpgButtonArrow::ArrowUpSquareFill => icon_to_string(bootstrap::Bootstrap::ArrowUpSquareFill),
        IpgButtonArrow::Arrows => icon_to_string(bootstrap::Bootstrap::Arrows),
        IpgButtonArrow::ArrowsAngleContract => icon_to_string(bootstrap::Bootstrap::ArrowsAngleContract),
        IpgButtonArrow::ArrowsAngleExpand => icon_to_string(bootstrap::Bootstrap::ArrowsAngleExpand),
        IpgButtonArrow::ArrowsCollapse => icon_to_string(bootstrap::Bootstrap::ArrowsCollapse),
        IpgButtonArrow::ArrowsCollapseVertical => icon_to_string(bootstrap::Bootstrap::ArrowsCollapseVertical),
        IpgButtonArrow::ArrowsExpand => icon_to_string(bootstrap::Bootstrap::ArrowsExpand),
        IpgButtonArrow::ArrowsExpandVertical => icon_to_string(bootstrap::Bootstrap::ArrowsExpandVertical),
        IpgButtonArrow::ArrowsFullscreen => icon_to_string(bootstrap::Bootstrap::ArrowsFullscreen),
        IpgButtonArrow::ArrowsMove => icon_to_string(bootstrap::Bootstrap::ArrowsMove),
        IpgButtonArrow::ArrowsVertical => icon_to_string(bootstrap::Bootstrap::ArrowsVertical),
    }
}

pub fn get_bootstrap_arrow_char(arrow: IpgButtonArrow) -> char {
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
