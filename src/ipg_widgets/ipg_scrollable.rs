//! ipg_scrollable
use crate::{app, access_callbacks, access_state};
use crate::TABLE_INTERNAL_IDS_END;
use crate::TABLE_INTERNAL_IDS_START;
use super::callbacks::get_set_container_callback_data;
use super::callbacks::WidgetCallbackIn;
use super::callbacks::WidgetCallbackOut;
use super::helpers::{get_height, get_radius, get_width, 
    try_extract_f64};
use super::ipg_table::{TableMessage, table_callback};

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
use pyo3::types::IntoPyDict;
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
    pub h_bar_alignment: IpgScrollableAlignment,
    pub v_bar_width: f32,
    pub v_bar_margin: f32,
    pub v_scroller_width: f32,
    pub v_bar_alignment: IpgScrollableAlignment,
    pub user_data: Option<PyObject>,
    pub style_id: Option<String>,
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
        h_bar_alignment: IpgScrollableAlignment,
        v_bar_width: f32,
        v_bar_margin: f32,
        v_scroller_width: f32,
        v_bar_alignment: IpgScrollableAlignment,
        user_data: Option<PyObject>,
        style_id: Option<String>,
    ) -> Self {
        Self {
            id,
            width,
            height,
            direction,
            h_bar_width,
            h_bar_margin,
            h_scroller_width,
            h_bar_alignment,
            v_bar_width,
            v_bar_margin,
            v_scroller_width,
            v_bar_alignment,
            user_data,
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

#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgScrollableDirection {
    Vertical,
    Horizontal,
    Both,
}


#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgScrollableAlignment {
    Start,
    End,
}


pub fn construct_scrollable(scroll: IpgScrollable, content: Vec<Element<'static, app::Message>> ) 
                                                            -> Element<'static, app::Message> {

    let content: Element<'static, app::Message> = Column::with_children(content).into();

    let direction: Direction = get_direction(scroll.direction.clone(),
                                                        scroll.h_bar_width,
                                                        scroll.h_bar_margin,
                                                        scroll.h_scroller_width,
                                                        scroll.h_bar_alignment.clone(),
                                                        scroll.v_bar_width,
                                                        scroll.v_bar_margin,
                                                        scroll.v_scroller_width,
                                                        scroll.v_bar_alignment.clone()
                                                    );

    Scrollable::with_direction(content, direction)
                    .width(scroll.width)
                    .height(scroll.height)
                    .on_scroll(move|vp| app::Message::Scrolled(vp, scroll.id))
                    .style(move|theme, status| {
                        get_styling(theme, status,
                                    scroll.style_id.clone(),
                                    )
                    })
                    .into()
    
  
}


fn get_direction(direction: IpgScrollableDirection, 
                    h_width: f32,
                    h_margin: f32,
                    h_scroller_width: f32,
                    h_alignment: IpgScrollableAlignment,
                    v_width: f32,
                    v_margin: f32,
                    v_scroller_width: f32,
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
                                    .scroller_width(h_scroller_width);

    let v_properties = Scrollbar::new()
                                    .anchor(v_alignment)
                                    .width(v_width)
                                    .margin(v_margin)
                                    .scroller_width(v_scroller_width);


    match direction {
        IpgScrollableDirection::Vertical => Direction::Vertical(v_properties),
        IpgScrollableDirection::Horizontal => Direction::Horizontal(h_properties),
        IpgScrollableDirection::Both => Direction::Both { vertical: v_properties, 
                            horizontal: h_properties },
    }

}

pub fn scrollable_callback(id: usize, vp: Viewport) {

    if id >= TABLE_INTERNAL_IDS_START && id <= TABLE_INTERNAL_IDS_END {
        table_callback(id, TableMessage::TableScrolled(vp, id));
        return
    }

    let mut wci = WidgetCallbackIn::default();
    wci.id = id;

    let mut offsets: Vec<(String, f32)> = vec![];
    offsets.push(("abs_offset_x".to_string(), vp.absolute_offset().x));
    offsets.push(("abs_offset_y".to_string(), vp.absolute_offset().y));
    offsets.push(("rel_offset_x".to_string(), vp.relative_offset().x));
    offsets.push(("rel_offset_y".to_string(), vp.relative_offset().y));
    offsets.push(("rev_offset_x".to_string(), vp.absolute_offset_reversed().x));
    offsets.push(("rev_offset_y".to_string(), vp.absolute_offset_reversed().y));
    
    let mut wci: WidgetCallbackIn = WidgetCallbackIn::default();
    wci.id = id;
    
    let mut wco = get_set_container_callback_data(wci);
    wco.id = id;
    wco.scroll_pos = offsets;
    wco.event_name = "on_scroll".to_string();
    process_callback(wco);
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
        None => panic!("Scrollable Callback could not be found with id {}", wco.id),
    };
                  
    Python::with_gil(|py| {
            if wco.user_data.is_some() {
                let user_data = match wco.user_data {
                    Some(ud) => ud,
                    None => panic!("Scrollable callback user_data not found."),
                };
                let res = callback.call1(py, (
                                                                    wco.id.clone(), 
                                                                    wco.scroll_pos.into_py_dict_bound(py), 
                                                                    user_data
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Scrollable: 3 parameters (id, value, user_data) are required or a python error in this function. {er}"),
                }
            } else {
                let res = callback.call1(py, (
                                                                    wco.id.clone(), 
                                                                    wco.scroll_pos.into_py_dict_bound(py), 
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Scrollable: 2 parameters (id, value,) are required or a python error in this function. {er}"),
                }
            } 
    });

    drop(app_cbs); 

}


#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgScrollableParam {
    Width,
    Height,
    HBarWidth,
    HBarMargin,
    HScrollerWidth,
    HBarAlignment,
    VBarWidth,
    VBarMargin,
    VScrollerWidth,
    VBarAlignment,
    ScrollXTo,
    ScrollYTo,
}


pub fn scrollable_item_update(scroll: &mut IpgScrollable,
                                item: PyObject,
                                value: PyObject,
                            ) 
{
    let update = try_extract_scrollable_update(item);

    match update {
        IpgScrollableParam::Width => {
            let val = try_extract_f64(value);
            scroll.width = get_width(Some(val as f32), false);
        },
        IpgScrollableParam::Height => {
            let val = try_extract_f64(value);
            scroll.height = get_height(Some(val as f32), false);
        },
        IpgScrollableParam::HBarWidth => {
            scroll.h_bar_width = try_extract_f64(value) as f32;
        },
        IpgScrollableParam::HBarMargin => {
            scroll.h_bar_margin = try_extract_f64(value) as f32;
        },
        IpgScrollableParam::HScrollerWidth => {
            scroll.h_scroller_width = try_extract_f64(value) as f32;
        },
        IpgScrollableParam::HBarAlignment => {
            scroll.h_bar_alignment = try_extract_alignment(value);
        },
        IpgScrollableParam::VBarWidth => {
            scroll.v_bar_width = try_extract_f64(value) as f32;
        },
        IpgScrollableParam::VBarMargin => {
            scroll.v_bar_margin = try_extract_f64(value) as f32;
        },
        IpgScrollableParam::VScrollerWidth => {
            scroll.v_scroller_width = try_extract_f64(value) as f32;
        },
        IpgScrollableParam::VBarAlignment => {
            scroll.v_bar_alignment = try_extract_alignment(value);
        },
        IpgScrollableParam::ScrollXTo => todo!(),
        IpgScrollableParam::ScrollYTo => todo!(),
    }
}


pub fn try_extract_scrollable_update(update_obj: PyObject) -> IpgScrollableParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgScrollableParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Scrollable update extraction failed"),
        }
    })
}


pub fn try_extract_alignment(direct_obj: PyObject) -> IpgScrollableAlignment {
    Python::with_gil(|py| {
        let res = direct_obj.extract::<IpgScrollableAlignment>(py);
            
        match res {
            Ok(align) => align,
            Err(_) => panic!("ScrollableAlignment failed to extract."),
        }
    })  
}

fn get_styling(theme: &Theme, status: Status,
                style_id: Option<String>,
                ) -> Style 
{
    let state = access_state();

    if style_id.is_none() {
        return scrollable::default(theme, status);
    }

    let style_opt = state.scrollable_style.get(&style_id.unwrap());

    let style = if style_opt.is_some() {
        style_opt.unwrap()
    } else {
        panic!("Container style: style_id not found.")
    };

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
