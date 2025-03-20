//! ipg_timer
use crate::graphics::colors::get_color;
use crate::style::styling::IpgStyleStandard;
use crate::{access_callbacks, access_user_data1, app, IpgState};
use super::callbacks::{set_or_get_widget_callback_data, WidgetCallbackIn, WidgetCallbackOut};
use super::helpers::{get_height, get_padding_f64, get_radius, get_width, try_extract_boolean, try_extract_f64, try_extract_i64, try_extract_ipg_color, try_extract_rgba_color, try_extract_string, try_extract_style_standard, try_extract_u64, try_extract_vec_f32, try_extract_vec_f64};
use super::ipg_button::{get_bootstrap_arrow, get_standard_style, try_extract_button_arrow, IpgButtonArrow};
use super::ipg_enums::IpgWidgets;

use iced::widget::button::{self, Status};
use iced::widget::{Button, Text};
use iced::{Border, Color, Element, Length, Padding, Shadow, Theme, Vector};

use pyo3::{pyclass, PyObject, Python};

#[derive(Debug, Clone)]
pub struct IpgTimer {
    pub id: usize,
    pub parent_id: String,
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

impl IpgTimer {
    pub fn new(
        id: usize,
        parent_id: String,
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
            parent_id,
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
pub struct IpgTimerStyle {
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

impl IpgTimerStyle {
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
pub enum TIMMessage {
    OnStartStop,
}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgTimerParam {
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

pub fn construct_timer<'a>(tim: &'a IpgTimer, 
                        style_opt: Option<&IpgWidgets>) 
                        -> Option<Element<'a, app::Message>> {

    if !tim.show {
        return None
    }

    let style = get_timer_style(style_opt);

    let mut label = Text::new(tim.label.clone());
    
    if tim.style_arrow.is_some() {
        let arrow = get_bootstrap_arrow(&tim.style_arrow);
        label = Text::new(arrow).font(iced::Font::with_name("bootstrap-icons"));
    }
    
    let timer_btn: Element<TIMMessage> = Button::new(label)
                                .height(tim.height)
                                .padding(tim.padding)
                                .width(tim.width)
                                .on_press(TIMMessage::OnStartStop)
                                .style(move|theme: &Theme, status| {
                                    get_styling(theme, status,
                                        style.clone(),
                                        tim.style_standard.clone(),
                                    )  })
                                .into();
    
    Some(timer_btn.map(move |message: TIMMessage| app::Message::Timer(tim.id, message)))

    
}

pub fn timer_callback(state: &mut IpgState, id: usize, started: bool) -> u64 {
    // The timer callback tooggles the timer.started boolean value
    // No variable is passed since this toggle corresponds to the
    // timer event toggle
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

pub fn tick_callback(state: &mut IpgState) 
{
    let id= state.timer_event_id_enabled.0;
    let mut wci = WidgetCallbackIn{id, ..Default::default()};
    wci.value_str = Some("on_tick".to_string());
    let wco: WidgetCallbackOut = set_or_get_widget_callback_data(state, wci);
    process_callback(id, "on_tick".to_string(), wco.counter);
}

fn process_callback(id: usize, event_name: String, counter: Option<u64>)
{
    let ud = access_user_data1();
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
            if user_data_opt.is_some() && counter.is_some(){
                let res = cb.call1(py, (
                                                            id,
                                                            counter.unwrap(),  
                                                            user_data_opt.unwrap()
                                                            ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Timer: 3 parameters (id, counter, user_data) 
                                        are required or a python error in this function. {er}"),
                }
            } else if user_data_opt.is_none() && counter.is_some() {
                let res = cb.call1(py, (
                                                            id,
                                                            counter.unwrap(),  
                                                            ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Timer: 2 parameters (id, counter) 
                                        are required or a python error in this function. {er}"),
                }
            } else {
                let res = cb.call1(py, (
                                                            id,  
                                                            ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Timer: 1 parameter (id) 
                                        is required or a python error in this function. {er}"),
                }
            }
    });
    
    drop(ud);
}


pub fn timer_item_update(tim: &mut IpgTimer,
                        item: &PyObject,
                        value: &PyObject,
                        )
{
    let update = try_extract_timer_update(item);
    let name = "Timer".to_string();
    match update {
        IpgTimerParam::DurationMs => {
            tim.duration_ms = try_extract_i64(value, name) as u64;
        },
       IpgTimerParam::ArrowStyle => {
            tim.style_arrow = Some(try_extract_button_arrow(value));
        },
        IpgTimerParam::Counter => {
            tim.counter = try_extract_u64(value, name);
        }
        IpgTimerParam::Label => {
            tim.label = try_extract_string(value, name);
        },
        IpgTimerParam::Height => {
            let val = try_extract_f64(value, name);
            tim.height = get_height(Some(val as f32), false);
        },
        IpgTimerParam::HeightFill => {
            let val = try_extract_boolean(value, name);
            tim.height = get_height(None, val);
        },
        IpgTimerParam::Padding => {
            tim.padding =  get_padding_f64(try_extract_vec_f64(value, name));
        },
        IpgTimerParam::Clip => {
            tim.clip = try_extract_boolean(value, name);
        }
        IpgTimerParam::Show => {
            tim.show = try_extract_boolean(value, name);
        },
        IpgTimerParam::StyleId => {
            tim.style_id = Some(try_extract_f64(value, name) as usize);
        },
        IpgTimerParam::StyleStandard => {
            tim.style_standard = Some(try_extract_style_standard(value, name));
        },
        IpgTimerParam::Width => {
            let val = try_extract_f64(value, name);
            tim.width = get_width(Some(val as f32), false);
        },
        IpgTimerParam::WidthFill => {
            let val = try_extract_boolean(value, name);
            tim.width = get_width(None, val);
        }, 
    }

}

pub fn try_extract_timer_update(update_obj: &PyObject) -> IpgTimerParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgTimerParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Timer update extraction failed"),
        }
    })
}

pub fn get_styling(theme: &Theme, status: Status,
                    style_opt: Option<IpgTimerStyle>,
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
        Status::Active | Status::Pressed => base_style,
        Status::Hovered => hover_style,
        Status::Disabled => disabled(base_style),
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
pub enum IpgTimerStyleParam {
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

pub fn timer_style_update_item(style: &mut IpgTimerStyle,
                            item: &PyObject,
                            value: &PyObject,) 
{
    let update = try_extract_timer_style_update(item);
    let name = "TimerStyle".to_string();
    match update {
        IpgTimerStyleParam::BackgroundIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.background_color = get_color(None, Some(color), 1.0, false);
        },
        IpgTimerStyleParam::BackgroundRbgaColor => {
            style.background_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgTimerStyleParam::BackgroundIpgColorHovered => {
            let color = try_extract_ipg_color(value, name);
            style.background_color_hovered = get_color(None, Some(color), 1.0, false);
        },
        IpgTimerStyleParam::BackgroundIpgRgbaHovered => {
            style.background_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgTimerStyleParam::BorderIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.border_color = get_color(None, Some(color), 1.0, false);
        },
        IpgTimerStyleParam::BorderRgbaColor => {
            style.border_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgTimerStyleParam::BorderRadius => {
            style.border_radius = try_extract_vec_f32(value, name);
        },
        IpgTimerStyleParam::BorderWidth => {
            style.border_width = try_extract_f64(value, name) as f32;
        },
        IpgTimerStyleParam::ShadowIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.shadow_color = get_color(None, Some(color), 1.0, false);
        },
        IpgTimerStyleParam::ShadowRgbaColor => {
            style.border_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgTimerStyleParam::ShadowOffsetX => {
            style.shadow_offset_x = try_extract_f64(value, name) as f32;
        },
        IpgTimerStyleParam::ShadowOffsetY => {
            style.shadow_offset_y = try_extract_f64(value, name) as f32;
        },
        IpgTimerStyleParam::ShadowBlurRadius => {
            style.shadow_blur_radius = try_extract_f64(value, name) as f32;
        },
        IpgTimerStyleParam::TextIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.text_color = get_color(None, Some(color), 1.0, false);
        },
        IpgTimerStyleParam::TextRgbaColor => {
            style.text_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
    }
}

fn get_timer_style(style: Option<&IpgWidgets>) -> Option<IpgTimerStyle>{
    match style {
        Some(IpgWidgets::IpgTimerStyle(style)) => {
            Some(style.clone())
        }
        _ => None,   
    }
}

fn try_extract_timer_style_update(update_obj: &PyObject) -> IpgTimerStyleParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgTimerStyleParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Timer style update extraction failed"),
        }
    })
}
