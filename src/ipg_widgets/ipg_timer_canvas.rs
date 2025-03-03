//! ipg_timer
use crate::graphics::colors::get_color;
use crate::style::styling::IpgStyleStandard;
use crate::{access_callbacks, access_user_data, app, IpgState};
use super::callbacks::{set_or_get_widget_callback_data, 
    WidgetCallbackIn, WidgetCallbackOut};
use super::helpers::{get_height, get_padding_f64, get_radius, 
    get_width, try_extract_boolean, try_extract_f64, try_extract_i64, 
    try_extract_ipg_color, try_extract_rgba_color, try_extract_string, 
    try_extract_style_standard, try_extract_u64, try_extract_vec_f32, 
    try_extract_vec_f64};
use super::ipg_button::{get_bootstrap_arrow, get_standard_style, 
    try_extract_button_arrow, IpgButtonArrow};
use super::ipg_enums::IpgWidgets;

use iced::widget::{button, Button, Text};
use iced::{Border, Color, Element, Length, Padding, Shadow, Theme, Vector};

use pyo3::{pyclass, PyObject, Python};

#[derive(Debug, Clone)]
pub struct IpgCanvasTimer {
    pub id: usize,
    pub duration_ms: u64,
    pub label: String,
    pub width: Length,
    pub height: Length,
    pub padding: Padding,
    pub clip: bool,
    pub style_id: Option<usize>,
    pub style_standard: Option<IpgStyleStandard>,
    pub style_arrow: Option<IpgButtonArrow>,
    pub counter: u64,
    pub started: bool,
    pub ticking: bool,
    pub show: bool,
}

impl IpgCanvasTimer {
    pub fn new(
        id: usize,
        duration_ms: u64,
        label: String,
        width: Length,
        height: Length,
        padding: Padding,
        clip: bool,
        style_id: Option<usize>,
        style_standard: Option<IpgStyleStandard>,
        style_arrow: Option<IpgButtonArrow>,
        show: bool,
        ) -> Self {
        Self {
            id,
            duration_ms,
            label,
            width,
            height,
            padding,
            clip,
            style_id,
            style_standard,
            style_arrow,
            counter: 0,
            started: false,
            ticking: false,
            show,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct IpgCanvasTimerStyle {
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

impl IpgCanvasTimerStyle {
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
pub enum CanvasTimerMessage {
    OnStartStop,
}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgCanvasTimerParam {
    DurationMs,
    ArrowStyle,
    Counter,
    Height,
    HeightFill,
    Label,
    Padding,
    Clip,
    Show,
    StyleId,
    StyleStandard,
    Width,
    WidthFill,
}

pub fn construct_canvas_timer<'a>(tim: &'a IpgCanvasTimer, 
                                style_opt: Option<&'a IpgWidgets>) 
                                -> Option<Element<'a, app::Message>> {

    if !tim.show {
        return None
    }

    let style = get_canvas_timer_style(style_opt);

    let mut label = Text::new(tim.label.clone());
    
    if tim.style_arrow.is_some() {
        let arrow = get_bootstrap_arrow(&tim.style_arrow);
        label = Text::new(arrow).font(iced::Font::with_name("bootstrap-icons"));
    }
    
    let timer_btn: Element<CanvasTimerMessage> = Button::new(label)
                                .height(tim.height)
                                .padding(tim.padding)
                                .width(tim.width)
                                .on_press(CanvasTimerMessage::OnStartStop)
                                .style(move|theme: &Theme, status| {
                                    get_styling(theme, status,
                                        style.clone(),
                                        tim.style_standard.clone(),
                                    )  })
                                .into();
    
    Some(timer_btn.map(move |message: CanvasTimerMessage| app::Message::CanvasTimer(tim.id, message)))

    
}

pub fn canvas_timer_callback(state: &mut IpgState, id: usize, started: bool) -> u64 {

    let mut wci = WidgetCallbackIn{id, ..Default::default()};
    wci.value_bool = Some(started);
    let mut wco: WidgetCallbackOut = set_or_get_widget_callback_data(state, wci);
    wco.id = id;
    let duration = wco.duration.unwrap_or(1);
    let event_name = if started {
        "on_start".to_string()
    } else {
        "on_stop".to_string()
    };
    
    process_callback(id, event_name, None);
    duration 
}

pub fn canvas_tick_callback(state: &mut IpgState) 
{
    let id= state.canvas_timer_event_id_enabled.0;
    let mut wci = WidgetCallbackIn{id, ..Default::default()};
    wci.value_str = Some("on_tick".to_string());
    let wco: WidgetCallbackOut = set_or_get_widget_callback_data(state, wci);
    process_callback(id, "on_tick".to_string(), wco.counter);
}

fn process_callback(id: usize, event_name: String, counter: Option<u64>)
{
    let ud = access_user_data();
    let user_data_opt = ud.user_data.get(&id);

    let app_cbs = access_callbacks();

    let callback_present = 
        app_cbs.callbacks.get(&(id, event_name));
    
    let callback = match callback_present {
        Some(cb) => cb,
        None => return,
    };

    let cb = 
        Python::with_gil(|py| {
            callback.clone_ref(py)
        });

    drop(app_cbs);

    Python::with_gil(|py| {
            if user_data_opt.is_some() && counter.is_some() {
                let res = cb.call1(py, (
                                                            id,
                                                            counter.unwrap(),  
                                                            user_data_opt.unwrap()
                                                            ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("CanvasTimer: 3 parameters (id, counter, user_data) 
                                        are required or a python error in this function. {er}"),
                }
            } else if counter.is_some() {
                let res = cb.call1(py, (
                                                            id,
                                                            counter.unwrap(),  
                                                            ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("CanvasTimer: 2 parameters (id, counter) 
                                        are required or a python error in this function. {er}"),
                }
            } else {
                let res = cb.call1(py, (
                                                            id, 
                                                            ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("CanvasTimer: 1 parameter (id) 
                                        is required or a python error in this function. {er}"),
                }
            }
    });
    
    drop(ud);
}


pub fn canvas_timer_item_update(ctim: &mut IpgCanvasTimer,
                            item: &PyObject,
                            value: &PyObject,
                            )
{
    let update = try_extract_timer_update(item);
    let name = "CanvasTimer".to_string();
    match update {
        IpgCanvasTimerParam::DurationMs => {
            ctim.duration_ms = try_extract_i64(value, name) as u64;
        },
       IpgCanvasTimerParam::ArrowStyle => {
            ctim.style_arrow = Some(try_extract_button_arrow(value));
        },
        IpgCanvasTimerParam::Counter => {
            ctim.counter = try_extract_u64(value, name);
        }
        IpgCanvasTimerParam::Label => {
            ctim.label = try_extract_string(value, name);
        },
        IpgCanvasTimerParam::Height => {
            let val = try_extract_f64(value, name);
            ctim.height = get_height(Some(val as f32), false);
        },
        IpgCanvasTimerParam::HeightFill => {
            let val = try_extract_boolean(value, name);
            ctim.height = get_height(None, val);
        },
        IpgCanvasTimerParam::Padding => {
            ctim.padding =  get_padding_f64(try_extract_vec_f64(value, name));
        },
        IpgCanvasTimerParam::Clip => {
            ctim.clip = try_extract_boolean(value, name);
        }
        IpgCanvasTimerParam::Show => {
            ctim.show = try_extract_boolean(value, name);
        },
        IpgCanvasTimerParam::StyleId => {
            ctim.style_id = Some(try_extract_f64(value, name) as usize);
        },
        IpgCanvasTimerParam::StyleStandard => {
            ctim.style_standard = Some(try_extract_style_standard(value, name));
        },
        IpgCanvasTimerParam::Width => {
            let val = try_extract_f64(value, name);
            ctim.width = get_width(Some(val as f32), false);
        },
        IpgCanvasTimerParam::WidthFill => {
            let val = try_extract_boolean(value, name);
            ctim.width = get_width(None, val);
        }, 
    }

}

pub fn try_extract_timer_update(update_obj: &PyObject) -> IpgCanvasTimerParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgCanvasTimerParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("CanvasTimer update extraction failed"),
        }
    })
}

fn get_styling(theme: &Theme, status: button::Status,
                    style_opt: Option<IpgCanvasTimerStyle>,
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

    match status {
        button::Status::Active | button::Status::Pressed => base_style,
        button::Status::Hovered => hover_style,
        button::Status::Disabled => disabled(base_style),
    }
    
}

fn disabled(style: button::Style) -> button::Style {
    button::Style {
        background: style
            .background
            .map(|background| background.scale_alpha(0.5)),
        text_color: style.text_color.scale_alpha(0.5),
        ..style
    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgCanvasTimerStyleParam {
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

pub fn canvas_timer_style_update_item(style: &mut IpgCanvasTimerStyle,
                                    item: &PyObject,
                                    value: &PyObject,) 
{
    let update = try_extract_canvas_timer_style_update(item);
    let name = "CanvasTimerStyle".to_string();
    match update {
        IpgCanvasTimerStyleParam::BackgroundIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.background_color = get_color(None, Some(color), 1.0, false);
        },
        IpgCanvasTimerStyleParam::BackgroundRbgaColor => {
            style.background_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgCanvasTimerStyleParam::BackgroundIpgColorHovered => {
            let color = try_extract_ipg_color(value, name);
            style.background_color_hovered = get_color(None, Some(color), 1.0, false);
        },
        IpgCanvasTimerStyleParam::BackgroundIpgRgbaHovered => {
            style.background_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgCanvasTimerStyleParam::BorderIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.border_color = get_color(None, Some(color), 1.0, false);
        },
        IpgCanvasTimerStyleParam::BorderRgbaColor => {
            style.border_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgCanvasTimerStyleParam::BorderRadius => {
            style.border_radius = try_extract_vec_f32(value, name);
        },
        IpgCanvasTimerStyleParam::BorderWidth => {
            style.border_width = try_extract_f64(value, name) as f32;
        },
        IpgCanvasTimerStyleParam::ShadowIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.shadow_color = get_color(None, Some(color), 1.0, false);
        },
        IpgCanvasTimerStyleParam::ShadowRgbaColor => {
            style.border_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgCanvasTimerStyleParam::ShadowOffsetX => {
            style.shadow_offset_x = try_extract_f64(value, name) as f32;
        },
        IpgCanvasTimerStyleParam::ShadowOffsetY => {
            style.shadow_offset_y = try_extract_f64(value, name) as f32;
        },
        IpgCanvasTimerStyleParam::ShadowBlurRadius => {
            style.shadow_blur_radius = try_extract_f64(value, name) as f32;
        },
        IpgCanvasTimerStyleParam::TextIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.text_color = get_color(None, Some(color), 1.0, false);
        },
        IpgCanvasTimerStyleParam::TextRgbaColor => {
            style.text_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
    }
}

fn get_canvas_timer_style(style: Option<&IpgWidgets>) -> Option<IpgCanvasTimerStyle>{
    match style {
        Some(st) => {
            match st {
                IpgWidgets::IpgCanvasTimerStyle(style) => {
                    Some(style.clone())
                }
                _ => None,
            }
        },
        None => None,
    }
}

fn try_extract_canvas_timer_style_update(update_obj: &PyObject) -> IpgCanvasTimerStyleParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgCanvasTimerStyleParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Canvas Timer style update extraction failed"),
        }
    })
}