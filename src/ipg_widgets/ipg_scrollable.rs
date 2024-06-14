
use crate::access_callbacks;
use crate::access_state;
use crate::app;
use crate::ipg_widgets::ipg_container;
use crate::ipg_widgets::ipg_theme_colors::lighten;
use super::callbacks::get_set_container_callback_data;
use super::callbacks::WidgetCallbackIn;
use super::callbacks::WidgetCallbackOut;
use super::helpers::get_height;
use super::helpers::get_width;
use super::helpers::try_extract_f64;

use iced::widget::scrollable::Scrollbar;
use iced::widget::scrollable::Scroller;
use iced::widget::scrollable::{Alignment, Direction, Properties, 
    Scrollable, Viewport, Status, Style, default};
use iced::Background;
use iced::Border;
use iced::Color;
use iced::Theme;
use iced::{Element, Length};
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
    pub scroll_bar_style_background: Option<String>,
    pub scroll_bar_style_border: Option<String>,
    pub scroller_style_background: Option<String>,
    pub scroller_style_border: Option<String>,
    pub container_style_background: Option<String>,
    pub container_style_border: Option<String>,
    pub container_style_shadow: Option<String>,
    pub container_style_text_color: Option<String>,
    pub user_data: Option<PyObject>,
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
        scroll_bar_style_background: Option<String>,
        scroll_bar_style_border: Option<String>,
        scroller_style_background: Option<String>,
        scroller_style_border: Option<String>,
        container_style_background: Option<String>,
        container_style_border: Option<String>,
        container_style_shadow: Option<String>,
        container_style_text_color: Option<String>,
        user_data: Option<PyObject>,
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
            scroll_bar_style_background,
            scroll_bar_style_border,
            scroller_style_background,
            scroller_style_border,
            container_style_background,
            container_style_border,
            container_style_shadow,
            container_style_text_color,
            user_data,
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
                    .on_scroll(move|b| app::Message::Scrolled(b, scroll.id))
                    .style(move|theme, status| {
                        get_styling(theme, status,
                                    scroll.scroll_bar_style_background.clone(),
                                    scroll.scroll_bar_style_border.clone(),
                                    scroll.scroller_style_background.clone(),
                                    scroll.scroller_style_border.clone(),
                                    scroll.container_style_background.clone(),
                                    scroll.container_style_border.clone(),
                                    scroll.container_style_shadow.clone(),
                                    scroll.container_style_text_color.clone(),
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
        IpgScrollableAlignment::Start => Alignment::Start,
        IpgScrollableAlignment::End => Alignment::End,
    };

    let v_alignment = match v_alignment {
        IpgScrollableAlignment::Start => Alignment::Start,
        IpgScrollableAlignment::End => Alignment::End,
    };

    let h_properties = Properties::new()
                                    .alignment(h_alignment)
                                    .width(h_width)
                                    .margin(h_margin)
                                    .scroller_width(h_scroller_width);

    let v_properties = Properties::new()
                                    .alignment(v_alignment)
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
pub enum IpgScrollableParams {
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
}


pub fn scrollable_item_update(scroll: &mut IpgScrollable,
                                item: PyObject,
                                value: PyObject,
                            ) 
{
    let update = try_extract_scrollable_update(item);

    match update {
        IpgScrollableParams::Width => {
            let val = try_extract_f64(value);
            scroll.width = get_width(Some(val as f32), false);
        },
        IpgScrollableParams::Height => {
            let val = try_extract_f64(value);
            scroll.height = get_height(Some(val as f32), false);
        },
        IpgScrollableParams::HBarWidth => {
            scroll.h_bar_width = try_extract_f64(value) as f32;
        },
        IpgScrollableParams::HBarMargin => {
            scroll.h_bar_margin = try_extract_f64(value) as f32;
        },
        IpgScrollableParams::HScrollerWidth => {
            scroll.h_scroller_width = try_extract_f64(value) as f32;
        },
        IpgScrollableParams::HBarAlignment => {
            scroll.h_bar_alignment = try_extract_alignment(value);
        },
        IpgScrollableParams::VBarWidth => {
            scroll.v_bar_width = try_extract_f64(value) as f32;
        },
        IpgScrollableParams::VBarMargin => {
            scroll.v_bar_margin = try_extract_f64(value) as f32;
        },
        IpgScrollableParams::VScrollerWidth => {
            scroll.v_scroller_width = try_extract_f64(value) as f32;
        },
        IpgScrollableParams::VBarAlignment => {
            scroll.v_bar_alignment = try_extract_alignment(value);
        },
    }
}


pub fn try_extract_scrollable_update(update_obj: PyObject) -> IpgScrollableParams {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgScrollableParams>(py);
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
                scroll_bar_style_background: Option<String>,
                scroll_bar_style_border: Option<String>,
                scroller_style_background: Option<String>,
                scroller_style_border: Option<String>,
                container_style_background: Option<String>,
                container_style_border: Option<String>,
                container_style_shadow: Option<String>,
                container_style_text_color: Option<String>,
                ) -> Style 
{


    let default_style = default(theme, status);

    let (scroll_bar_bg, _color, _accent) = get_background(scroll_bar_style_background, 
                                                                            default_style);
    let scroll_bar_border = get_border(scroll_bar_style_border, default_style);

    let (_, scroller_color, scroller_accent) = get_background(scroller_style_background, default_style);

    let scroller_border = get_border(scroller_style_border, default_style);

    let container_style = ipg_container::get_styling(theme, 
                                                                container_style_background,
                                                                container_style_border,
                                                                container_style_shadow,
                                                                container_style_text_color,
                                                                );

    let scrollbar = Scrollbar {
        background: scroll_bar_bg,
        border: Border::rounded(scroll_bar_border.radius),
        scroller: Scroller {
            color: scroller_color,
            border: Border::rounded(scroller_border.radius),
        },
    };

    match status {
        Status::Active => Style {
            container: container_style,
            vertical_scrollbar: scrollbar,
            horizontal_scrollbar: scrollbar,
            gap: None,
        },
        Status::Hovered {
            is_horizontal_scrollbar_hovered,
            is_vertical_scrollbar_hovered,
        } => {
            let hovered_scrollbar = Scrollbar {
                scroller: Scroller {
                    color: lighten(scroller_color, scroller_accent),
                    ..scrollbar.scroller
                },
                ..scrollbar
            };

            Style {
                container: container_style,
                vertical_scrollbar: if is_vertical_scrollbar_hovered {
                    hovered_scrollbar
                } else {
                    scrollbar
                },
                horizontal_scrollbar: if is_horizontal_scrollbar_hovered {
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
            let dragged_scrollbar = Scrollbar {
                scroller: Scroller {
                    color: scroller_color,
                    ..scrollbar.scroller
                },
                ..scrollbar
            };

            Style {
                container: container_style,
                vertical_scrollbar: if is_vertical_scrollbar_dragged {
                    dragged_scrollbar
                } else {
                    scrollbar
                },
                horizontal_scrollbar: if is_horizontal_scrollbar_dragged {
                    dragged_scrollbar
                } else {
                    scrollbar
                },
                gap: None,
            }
        }
    }
}

fn get_background(style_bg: Option<String>, default: Style) -> (Option<Background>, Color, f32) {
    
    let state = access_state();

    let background_opt = if style_bg.is_some() {
        state.styling_background.get(&style_bg.unwrap())
    } else {
        None
    };

    let (bkg, color, accent) = if background_opt.is_some() {
        let bg = background_opt.unwrap();
        (Some(Background::Color(bg.color)), bg.color, bg.accent)
    } else {
        (default.horizontal_scrollbar.background, default.horizontal_scrollbar.scroller.color, 0.05)
    };

    drop(state);

    (bkg, color, accent)
}

fn get_border(style_border: Option<String>, default: Style) -> Border {

    let state = access_state();

    let border_opt = if style_border.is_some() {
        state.styling_border.get(&style_border.unwrap())
    } else {
        None
    };

    let border = if border_opt.is_some() {
        let bd = border_opt.unwrap().clone();
        Border{ color: bd.color, width: bd.width, radius: bd.radius }
    } else {
        default.horizontal_scrollbar.scroller.border
    };

    drop(state);

    border

}
