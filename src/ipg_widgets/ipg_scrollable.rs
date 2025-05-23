//! ipg_scrollable
use std::collections::HashMap;

use crate::graphics::colors::get_color;
use crate::{access_callbacks, access_user_data1, access_user_data2, app, IpgState};
use super::helpers::{get_height, get_radius, get_width, try_extract_f32, 
    try_extract_ipg_color, try_extract_rgba_color, try_extract_vec_f32};
use super::ipg_enums::IpgWidgets;

use iced::widget::container;
use iced::widget::scrollable;
use iced::widget::scrollable::Anchor;
use iced::widget::scrollable::Rail;
use iced::widget::scrollable::Scrollbar;
use iced::widget::scrollable::Scroller;
use iced::widget::scrollable::{Direction, Scrollable, Viewport, Status, Style};
use iced::Rectangle;
use iced::{Border, Color, Element, Length, Shadow, Vector, Theme};
use iced::widget::Column;

use pyo3::pyclass;
use pyo3::{Python, PyObject};


#[derive(Debug, Clone)]
pub struct IpgScrollable {
    pub id: usize,
    pub width: Length,
    pub height: Length,
    pub direction: IpgScrollableDirection,
    pub h_bar_width: f32,
    pub h_bar_margin: f32,
    pub h_scroller_width: f32,
    pub h_spacing: f32,
    pub h_bar_alignment: IpgScrollableAlignment,
    pub v_bar_width: f32,
    pub v_bar_margin: f32,
    pub v_scroller_width: f32,
    pub v_spacing: f32,
    pub v_bar_alignment: IpgScrollableAlignment,
    pub style_id: Option<usize>,
    pub scroll_y_pos: f32,
    pub scroll_x_pos: f32,
    pub bounds: Rectangle,
    pub content_bounds: Rectangle,
}

impl IpgScrollable {
    pub fn new(
        id: usize,
        width: Length,
        height: Length,
        direction: IpgScrollableDirection,
        h_bar_width: f32,
        h_bar_margin: f32,
        h_scroller_width: f32,
        h_spacing: f32,
        h_bar_alignment: IpgScrollableAlignment,
        v_bar_width: f32,
        v_bar_margin: f32,
        v_scroller_width: f32,
        v_spacing: f32,
        v_bar_alignment: IpgScrollableAlignment,
        style_id: Option<usize>,
    ) -> Self {
        Self {
            id,
            width,
            height,
            direction,
            h_bar_width,
            h_bar_margin,
            h_scroller_width,
            h_spacing,
            h_bar_alignment,
            v_bar_width,
            v_bar_margin,
            v_scroller_width,
            v_spacing,
            v_bar_alignment,
            style_id,
            scroll_y_pos: 0.0,
            scroll_x_pos: 0.0,
            bounds: Rectangle { x: 0.0, y: 0.0, width: 0.0, height: 0.0 },
            content_bounds: Rectangle { x: 0.0, y: 0.0, width: 0.0, height: 0.0 },
        }
    }
}


#[derive(Debug, Clone)]
pub struct IpgScrollableStyle {
    pub id: usize,
    pub background_color: Option<Color>,
    pub border_color: Option<Color>,
    pub border_radius: Vec<f32>,
    pub border_width: f32,
    pub shadow_color: Option<Color>,
    pub shadow_offset_x: f32,
    pub shadow_offset_y: f32,
    pub shadow_blur_radius: f32,
    pub text_color: Option<Color>,
    // above container style
    pub scrollbar_color: Option<Color>,
    pub scrollbar_border_radius: Vec<f32>,
    pub scrollbar_border_width: f32,
    pub scrollbar_border_color: Option<Color>,
    pub scroller_color: Option<Color>,
    pub scroller_color_hovered: Option<Color>,
    pub scroller_color_dragged: Option<Color>,
}

impl IpgScrollableStyle {
    pub fn new(
        id: usize,
        background_color: Option<Color>,
        border_color: Option<Color>,
        border_radius: Vec<f32>,
        border_width: f32,
        shadow: Option<Color>,
        shadow_offset_x: f32,
        shadow_offset_y: f32,
        shadow_blur_radius: f32,
        text_color: Option<Color>,
        // above container style
        scrollbar_color: Option<Color>,
        scrollbar_border_radius: Vec<f32>,
        scrollbar_border_width: f32,
        scrollbar_border_color: Option<Color>,
        scroller_color: Option<Color>,
        scroller_color_hovered: Option<Color>,
        scroller_color_dragged: Option<Color>,
    ) -> Self {
        Self {
            id,
            background_color,
            border_color,
            border_radius,
            border_width,
            shadow_color: shadow,
            shadow_offset_x,
            shadow_offset_y,
            shadow_blur_radius,
            text_color,
            // above container style
            scrollbar_color,
            scrollbar_border_radius,
            scrollbar_border_width,
            scrollbar_border_color,
            scroller_color,
            scroller_color_hovered,
            scroller_color_dragged,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgScrollableDirection {
    Vertical,
    Horizontal,
    Both,
}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgScrollableAlignment {
    Start,
    End,
}


pub fn construct_scrollable<'a>(scroll: &'a IpgScrollable, 
                            content: Vec<Element<'a, app::Message>>,
                            style_opt: Option<&IpgWidgets> ) 
                            -> Element<'a, app::Message> {
    
    let style = get_scroll_style(style_opt);

    let content: Element<'a, app::Message> = Column::with_children(content).into();

    let direction: Direction = 
        get_direction(scroll.direction.clone(),
                        scroll.h_bar_width,
                        scroll.h_bar_margin,
                        scroll.h_scroller_width,
                        scroll.h_spacing,
                        scroll.h_bar_alignment.clone(),
                        scroll.v_bar_width,
                        scroll.v_bar_margin,
                        scroll.v_scroller_width,
                        scroll.v_spacing,
                        scroll.v_bar_alignment.clone()
                    );

    Scrollable::with_direction(content, direction)
                    .width(scroll.width)
                    .height(scroll.height)
                    .on_scroll(move|vp| app::Message::Scrolled(vp, scroll.id))
                    .style(move|theme, status| {
                        get_styling(theme, status,
                                    style.clone(),
                                    )
                    })
                    .into()
    
}


fn get_direction(direction: IpgScrollableDirection, 
                    h_width: f32,
                    h_margin: f32,
                    h_scroller_width: f32,
                    h_spacing: f32,
                    h_alignment: IpgScrollableAlignment,
                    v_width: f32,
                    v_margin: f32,
                    v_scroller_width: f32,
                    v_spacing: f32,
                    v_alignment: IpgScrollableAlignment
                ) -> Direction {

    let h_alignment = match h_alignment {
        IpgScrollableAlignment::Start => Anchor::Start,
        IpgScrollableAlignment::End => Anchor::End,
    };

    let v_alignment = match v_alignment {
        IpgScrollableAlignment::Start => Anchor::Start,
        IpgScrollableAlignment::End => Anchor::End,
    };

    let h_properties = Scrollbar::new()
                                    .anchor(h_alignment)
                                    .width(h_width)
                                    .margin(h_margin)
                                    .scroller_width(h_scroller_width)
                                    .spacing(h_spacing);

    let v_properties = Scrollbar::new()
                                    .anchor(v_alignment)
                                    .width(v_width)
                                    .margin(v_margin)
                                    .scroller_width(v_scroller_width)
                                    .spacing(v_spacing);


    match direction {
        IpgScrollableDirection::Vertical => Direction::Vertical(v_properties),
        IpgScrollableDirection::Horizontal => Direction::Horizontal(h_properties),
        IpgScrollableDirection::Both => Direction::Both { vertical: v_properties, 
                            horizontal: h_properties },
    }

}

pub fn scrollable_callback(_state: &mut IpgState, id: usize, vp: Viewport) {
    let mut hmap = HashMap::new();
    hmap.insert("abs_x".to_string(), vp.absolute_offset().x);
    hmap.insert("abs_y".to_string(), vp.absolute_offset().y);
    hmap.insert("rel_x".to_string(), vp.relative_offset().x);
    hmap.insert("rel_y".to_string(), vp.relative_offset().y);
    hmap.insert("rev_x".to_string(), vp.absolute_offset_reversed().x);
    hmap.insert("rev_y".to_string(), vp.absolute_offset_reversed().y);
    
    process_callback(id, "on_scroll".to_string(), hmap);
}


pub fn process_callback(id: usize, 
                        event_name: String, 
                        hmap: HashMap<String, f32>) 
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
            if let Err(err) = callback.call1(py, (id, hmap, user_data)) {
                panic!("Scollable callback error: {err}");
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
            if let Err(err) = callback.call1(py, (id, hmap, user_data)) {
                panic!("Scrollable callback error: {err}");
            }
        });
        drop(ud2); // Drop ud2 after processing
        return;
    }
    drop(ud2); // Drop ud2 if no user data is found

    // If no user data is found in both ud1 and ud2, call the callback with the id and hmap
    Python::with_gil(|py| {
        if let Err(err) = callback.call1(py, (id, hmap)) {
            panic!("Scollable callback error: {err}");
        }
    });

}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgScrollableParam {
    Width,
    Height,
    HBarWidth,
    HBarMargin,
    HScrollerWidth,
    HSpacing,
    HBarAlignment,
    VBarWidth,
    VBarMargin,
    VScrollerWidth,
    VSpacing,
    VBarAlignment,
    ScrollXTo,
    ScrollYTo,
}


pub fn scrollable_item_update(scroll: &mut IpgScrollable,
                                item: &PyObject,
                                value: &PyObject,
                            ) 
{
    let update = try_extract_scrollable_update(item);
    let name = "Scrollable".to_string();
    match update {
        IpgScrollableParam::Width => {
            let val = try_extract_f32(value, name);
            scroll.width = get_width(Some(val), false);
        },
        IpgScrollableParam::Height => {
            let val = try_extract_f32(value, name);
            scroll.height = get_height(Some(val), false);
        },
        IpgScrollableParam::HBarWidth => {
            scroll.h_bar_width = try_extract_f32(value, name);
        },
        IpgScrollableParam::HBarMargin => {
            scroll.h_bar_margin = try_extract_f32(value, name);
        },
        IpgScrollableParam::HScrollerWidth => {
            scroll.h_scroller_width = try_extract_f32(value, name);
        },
        IpgScrollableParam::HSpacing => {
            scroll.h_spacing = try_extract_f32(value, name);
        },
        IpgScrollableParam::HBarAlignment => {
            scroll.h_bar_alignment = try_extract_alignment(value);
        },
        IpgScrollableParam::VBarWidth => {
            scroll.v_bar_width = try_extract_f32(value, name);
        },
        IpgScrollableParam::VBarMargin => {
            scroll.v_bar_margin = try_extract_f32(value, name);
        },
        IpgScrollableParam::VScrollerWidth => {
            scroll.v_scroller_width = try_extract_f32(value, name);
        },
        IpgScrollableParam::VSpacing => {
            scroll.v_spacing = try_extract_f32(value, name);
        },
        IpgScrollableParam::VBarAlignment => {
            scroll.v_bar_alignment = try_extract_alignment(value);
        },
        IpgScrollableParam::ScrollXTo => todo!(),
        IpgScrollableParam::ScrollYTo => todo!(),
    }
}


pub fn try_extract_scrollable_update(update_obj: &PyObject) -> IpgScrollableParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgScrollableParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Scrollable update extraction failed"),
        }
    })
}


pub fn try_extract_alignment(direct_obj: &PyObject) -> IpgScrollableAlignment {
    Python::with_gil(|py| {
        let res = direct_obj.extract::<IpgScrollableAlignment>(py);
            
        match res {
            Ok(align) => align,
            Err(_) => panic!("ScrollableAlignment failed to extract."),
        }
    })  
}

fn get_styling(theme: &Theme, status: Status,
                style_opt: Option<IpgScrollableStyle>,
                ) -> Style 
{

    if style_opt.is_none() {
        return scrollable::default(theme, status);
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

    border.radius = get_radius(style.border_radius.clone(), "Container".to_string());
    
    border.width = style.border_width;
    
    if style.shadow_color.is_some() {
        shadow.color = style.shadow_color.unwrap();
        shadow.blur_radius = style.shadow_blur_radius;
        shadow.offset = Vector{ x: style.shadow_offset_x, y: style.shadow_offset_y }
    }

    let container_style = container::Style {
            background: Some(background_color.into()),
            border,
            shadow,
            text_color: style.text_color,
        };

    let palette = theme.extended_palette();
    
    let scrollbar_color = if style.scrollbar_color.is_some() {
        style.scrollbar_color.unwrap().into()
    } else {
        palette.background.weak.color.into()
    };

    let border_radius = get_radius(style.scrollbar_border_radius.clone(), "Scrollable".to_string());
    let border_color = if style.scrollbar_border_color.is_some() {
        style.scrollbar_border_color.unwrap()
    } else {
        Color::TRANSPARENT
    };
    let border = Border{ color: border_color, width: style.border_width, radius: border_radius };

    let scroller_color = if style.scroller_color.is_some() {
        style.scroller_color.unwrap()
    } else {
        palette.background.strong.color
    };
    
    let scroller_color_hovered = if style.scroller_color_hovered.is_some() {
        style.scroller_color_hovered.unwrap()
    } else {
        palette.primary.strong.color
    };

    let scroller_color_dragged = if style.scroller_color_dragged.is_some() {
        style.scroller_color_dragged.unwrap()
    } else {
        palette.primary.base.color
    };

    let scrollbar = Rail {
        background: Some(scrollbar_color),
        border,
        scroller: Scroller {
            color: scroller_color,
            border,
        },
    };

    match status {
        Status::Active => Style {
            container: container_style,
            vertical_rail: scrollbar,
            horizontal_rail: scrollbar,
            gap: None,
        },
        Status::Hovered {
            is_horizontal_scrollbar_hovered,
            is_vertical_scrollbar_hovered,
        } => {
            let hovered_scrollbar = Rail {
                scroller: Scroller {
                    color: scroller_color_hovered,
                    ..scrollbar.scroller
                },
                ..scrollbar
            };

            Style {
                container: container_style,
                vertical_rail: if is_vertical_scrollbar_hovered {
                    hovered_scrollbar
                } else {
                    scrollbar
                },
                horizontal_rail: if is_horizontal_scrollbar_hovered {
                    hovered_scrollbar
                } else {
                    scrollbar
                },
                gap: None,
            }
        }
        Status::Dragged {
            is_horizontal_scrollbar_dragged,
            is_vertical_scrollbar_dragged,
        } => {
            let dragged_scrollbar = Rail {
                scroller: Scroller {
                    color: scroller_color_dragged,
                    ..scrollbar.scroller
                },
                ..scrollbar
            };

            Style {
                container: container_style,
                vertical_rail: if is_vertical_scrollbar_dragged {
                    dragged_scrollbar
                } else {
                    scrollbar
                },
                horizontal_rail: if is_horizontal_scrollbar_dragged {
                    dragged_scrollbar
                } else {
                    scrollbar
                },
                gap: None,
            }
        }
    }
    
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgScrollableStyleParam {
    BackgroundIpgColor,
    BackgroundRbgaColor,
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
    // above container style
    ScrollbarIpgColor,
    ScrollbarRgbaColor,
    ScrollbarBorderRadius,
    ScrollbarBorderWidth,
    ScrollbarBorderIpgColor,
    ScrollbarBorderRgbaColor,
    ScrollerIpgColor,
    ScrollerRgbaColor,
    ScrollerIpgColorHovered,
    ScrollerRgbaColorHovered,
    ScrollerIpgColorDragged,
    ScrollerRgbaColorDragged,
}

pub fn scroll_style_update_item(style: &mut IpgScrollableStyle,
                            item: &PyObject,
                            value: &PyObject,) 
{
    let update = try_extract_scroll_style_update(item);
    let name = "ScrollableStyle".to_string();
    match update {
        IpgScrollableStyleParam::BackgroundIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.background_color = get_color(None, Some(color), 1.0, false);
        },
        IpgScrollableStyleParam::BackgroundRbgaColor => {
            style.background_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgScrollableStyleParam::BorderIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.border_color = get_color(None, Some(color), 1.0, false);
        },
        IpgScrollableStyleParam::BorderRgbaColor => {
            style.border_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgScrollableStyleParam::BorderRadius => {
            style.border_radius = try_extract_vec_f32(value, name);
        },
        IpgScrollableStyleParam::BorderWidth => {
            style.border_width = try_extract_f32(value, name);
        },
        IpgScrollableStyleParam::ShadowIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.shadow_color = get_color(None, Some(color), 1.0, false);
        },
        IpgScrollableStyleParam::ShadowRgbaColor => {
            style.border_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgScrollableStyleParam::ShadowOffsetX => {
            style.shadow_offset_x = try_extract_f32(value, name);
        },
        IpgScrollableStyleParam::ShadowOffsetY => {
            style.shadow_offset_y = try_extract_f32(value, name);
        },
        IpgScrollableStyleParam::ShadowBlurRadius => {
            style.shadow_blur_radius = try_extract_f32(value, name);
        },
        IpgScrollableStyleParam::TextIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.text_color = get_color(None, Some(color), 1.0, false);
        },
        IpgScrollableStyleParam::TextRgbaColor => {
            style.text_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgScrollableStyleParam::ScrollbarIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.scrollbar_color = get_color(None, Some(color), 1.0, false);
        },
        IpgScrollableStyleParam::ScrollbarRgbaColor => {
            style.scrollbar_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgScrollableStyleParam::ScrollbarBorderRadius => {
            style.scrollbar_border_radius = try_extract_vec_f32(value, name);
        },
        IpgScrollableStyleParam::ScrollbarBorderWidth => {
            style.scrollbar_border_width = try_extract_f32(value, name);
        },
        IpgScrollableStyleParam::ScrollbarBorderIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.scrollbar_border_color = get_color(None, Some(color), 1.0, false);
        },
        IpgScrollableStyleParam::ScrollbarBorderRgbaColor => {
            style.scrollbar_border_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgScrollableStyleParam::ScrollerIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.scroller_color = get_color(None, Some(color), 1.0, false);
        },
        IpgScrollableStyleParam::ScrollerRgbaColor => {
            style.scroller_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgScrollableStyleParam::ScrollerIpgColorHovered => {
            let color = try_extract_ipg_color(value, name);
            style.scroller_color_hovered = get_color(None, Some(color), 1.0, false);
        },
        IpgScrollableStyleParam::ScrollerRgbaColorHovered => {
            style.scroller_color_hovered = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgScrollableStyleParam::ScrollerIpgColorDragged => {
            let color = try_extract_ipg_color(value, name);
            style.scroller_color_dragged = get_color(None, Some(color), 1.0, false);
        },
        IpgScrollableStyleParam::ScrollerRgbaColorDragged => {
            style.scroller_color_dragged = Some(Color::from(try_extract_rgba_color(value, name)));
        },
    }
}

fn get_scroll_style(style: Option<&IpgWidgets>) -> Option<IpgScrollableStyle>{
    match style {
        Some(IpgWidgets::IpgScrollableStyle(style)) => {
            Some(style.clone())
        }
        _ => None,
    }
}

pub fn try_extract_scroll_style_update(update_obj: &PyObject) -> IpgScrollableStyleParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgScrollableStyleParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Button style update extraction failed"),
        }
    })
}
