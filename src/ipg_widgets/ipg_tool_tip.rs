//! ipg_tool_tip
use iced::{Border, Color, Element, Shadow, Theme, Vector};
use iced::widget::{container, text, Column, Tooltip};
use iced::widget::tooltip::Position;
use pyo3::{pyclass, PyObject, Python};
use crate::app::Message;
use crate::graphics::colors::get_color;

use super::helpers::{get_radius, try_extract_array_2, 
    try_extract_boolean, try_extract_f32, try_extract_ipg_color, 
    try_extract_rgba_color, try_extract_string, try_extract_u16, 
    try_extract_usize, try_extract_vec_f32};
use super::ipg_enums::IpgWidgets;


#[derive(Debug, Clone)]
pub struct IpgToolTip {
    pub id: usize,
    pub position: IpgToolTipPosition,
    pub text_to_display: String,
    pub gap: u16,
    pub padding: f32,
    pub snap_within_viewport: bool,
    pub style_id: Option<usize>,
}

impl IpgToolTip {
    pub fn new( 
            id: usize,
            position: IpgToolTipPosition,
            text_to_display: String,
            gap: u16,
            padding: f32,
            snap_within_viewport: bool,
            style_id: Option<usize>,
        ) -> Self {
        Self {
            id,
            position,
            text_to_display,
            gap,
            padding,
            snap_within_viewport,
            style_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IpgToolTipStyle {
    pub id: usize,
    pub background_color: Option<Color>,
    pub border_color: Option<Color>,
    pub border_radius: Vec<f32>,
    pub border_width: f32,
    pub shadow_color: Option<Color>,
    pub shadow_offset_xy: [f32; 2],
    pub shadow_blur_radius: f32,
    pub text_color: Option<Color>,
}

impl IpgToolTipStyle {
    pub fn new(
        id: usize,
        background_color: Option<Color>,
        border_color: Option<Color>,
        border_radius: Vec<f32>,
        border_width: f32,
        shadow: Option<Color>,
        shadow_offset_xy: [f32; 2],
        shadow_blur_radius: f32,
        text_color: Option<Color>,
    ) -> Self {
        Self {
            id,
            background_color,
            border_color,
            border_radius,
            border_width,
            shadow_color: shadow,
            shadow_offset_xy,
            shadow_blur_radius,
            text_color,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgToolTipPosition {
    FollowCursor,
    Top,
    Bottom,
    Left,
    Right,
}

pub fn construct_tool_tip<'a>(
        tool: &IpgToolTip, 
        content: Vec<Element<'a, Message>>,
        style_opt: Option<&'a IpgWidgets>) 
        -> Element<'a, Message>
{
        let style = get_tool_style(style_opt);

        let position: Position = match tool.position {
                            IpgToolTipPosition::FollowCursor => Position::FollowCursor,
                            IpgToolTipPosition::Top => Position::Top,
                            IpgToolTipPosition::Bottom => Position::Bottom,
                            IpgToolTipPosition::Left   => Position::Left,
                            IpgToolTipPosition::Right  => Position::Right,
        };
        
        let content: Element<'a, Message> = Column::with_children(content).into();

        let tt: Element<Message> = Tooltip::new(
                content,
                text(tool.text_to_display.clone()),
                position,
                )
                .gap(tool.gap)
                .padding(tool.padding)
                .snap_within_viewport(tool.snap_within_viewport)
                .style(move|theme|
                    get_styling(theme, 
                        style.clone(),
                        ))
                .into();
        tt
}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgToolTipParam {
    Position,
    TextToDisplay,
    Gap,
    Padding,
    SnapWithinViewport,
    StyleId,
}

pub fn tooltip_item_update(tt: &mut IpgToolTip,
                            item: &PyObject,
                            value: &PyObject,
                            )
{
    let update = try_extract_tooltip_update(item);
    let name = "ToolTip".to_string();
    match update {
        IpgToolTipParam::Position => {
            tt.position = try_extract_position(value, name);
        },
        IpgToolTipParam::TextToDisplay => {
            tt.text_to_display = try_extract_string(value, name);
        },
        IpgToolTipParam::Gap => {
            tt.gap = try_extract_u16(value, name);
        },
        IpgToolTipParam::Padding => {
            tt.padding = try_extract_f32(value, name);
        },
        IpgToolTipParam::SnapWithinViewport => {
            tt.snap_within_viewport = try_extract_boolean(value, name);
        },
        IpgToolTipParam::StyleId => {
            tt.style_id = Some(try_extract_usize(value, name));
        },
    }
}

pub fn try_extract_tooltip_update(update_obj: &PyObject) -> IpgToolTipParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgToolTipParam>(py);

        match res {
            Ok(update) => update,
            Err(_) => panic!("ToolTip update extraction failed"),
        }
    })
}

pub fn try_extract_position(value: &PyObject, name: String) -> IpgToolTipPosition {
    Python::with_gil(|py| {
        let res = value.extract::<IpgToolTipPosition>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("{}-Unable to extract tooltip position", name),
        }
    })  
}

pub fn get_tool_style(style: Option<&IpgWidgets>) -> Option<IpgToolTipStyle>{
    match style {
        Some(IpgWidgets::IpgToolTipStyle(style)) => {
            Some(style.clone())
        }
        _ => None,
    }
}

pub fn get_styling(theme: &Theme,
                style_opt: Option<IpgToolTipStyle>,  
                ) -> container::Style {
    
    if style_opt.is_none() {
        return container::transparent(theme);
    }

    let style = style_opt.unwrap();

    let background_color = if style.background_color.is_some() {
        style.background_color.unwrap()
    } else {
        Color::TRANSPARENT
    };

    let mut border = Border::default();
    let mut shadow = Shadow::default();

    if style.border_color.is_some() {
        border.color = style.border_color.unwrap();
    }

    border.radius = get_radius(style.border_radius.clone(), "ToolTip".to_string());
    
    border.width = style.border_width;
    
    if style.shadow_color.is_some() {
        shadow.color = style.shadow_color.unwrap();
        shadow.blur_radius = style.shadow_blur_radius;
        shadow.offset = Vector{ x: style.shadow_offset_xy[0], y: style.shadow_offset_xy[1] }
    }

    container::Style {
        background: Some(background_color.into()),
        border,
        shadow,
        text_color: style.text_color,
    }
    
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgToolTipStyleParam {
    BackgroundIpgColor,
    BackgroundRgbaColor,
    BorderIpgColor,
    BorderRgbaColor,
    BorderRadius,
    BorderWidth,
    ShadowIpgColor,
    ShadowRgbaColor,
    ShadowOffsetXY,
    ShadowBlurRadius,
    TextIpgColor,
    TextRgbaColor,
}

pub fn tool_tip_style_update_item(
    style: &mut IpgToolTipStyle,
    item: &PyObject,
    value: &PyObject,) 
{
    let update = try_extract_container_style_update(item);
    let name = "ToolTipStyle".to_string();
    match update {
        IpgToolTipStyleParam::BackgroundIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.background_color = get_color(None, Some(color), 1.0, false);
        },
        IpgToolTipStyleParam::BackgroundRgbaColor => {
            style.background_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgToolTipStyleParam::BorderIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.border_color = get_color(None, Some(color), 1.0, false);
        },
        IpgToolTipStyleParam::BorderRgbaColor => {
            style.border_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgToolTipStyleParam::BorderRadius => {
            style.border_radius = try_extract_vec_f32(value, name);
        },
        IpgToolTipStyleParam::BorderWidth => {
            style.border_width = try_extract_f32(value, name);
        },
        IpgToolTipStyleParam::ShadowIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.shadow_color = get_color(None, Some(color), 1.0, false);
        },
        IpgToolTipStyleParam::ShadowRgbaColor => {
            style.border_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgToolTipStyleParam::ShadowOffsetXY => {
            style.shadow_offset_xy = try_extract_array_2(value, name);
        },
        IpgToolTipStyleParam::ShadowBlurRadius => {
            style.shadow_blur_radius = try_extract_f32(value, name);
        },
        IpgToolTipStyleParam::TextIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.text_color = get_color(None, Some(color), 1.0, false);
        },
        IpgToolTipStyleParam::TextRgbaColor => {
            style.text_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
    }

}

pub fn try_extract_container_style_update(update_obj: &PyObject) -> IpgToolTipStyleParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgToolTipStyleParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("ToolTip style parameter update extraction failed"),
        }
    })
}
