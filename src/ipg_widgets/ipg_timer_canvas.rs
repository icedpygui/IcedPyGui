//! ipg_timer
use crate::style::styling::IpgStyleStandard;
use crate::{access_callbacks, app, IpgState};
use super::callbacks::{set_or_get_widget_callback_data, WidgetCallbackIn, WidgetCallbackOut};
use super::helpers::{get_height, get_padding_f64, get_width, try_extract_boolean, try_extract_f64, try_extract_i64, try_extract_string, try_extract_style_standard, try_extract_u64, try_extract_vec_f64};
use super::ipg_button::{get_bootstrap_arrow, get_styling, try_extract_button_arrow, IpgButtonArrow};

use iced::widget::{Button, Space, Text};
use iced::{Element, Length, Padding, Theme};

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
    pub style_id: Option<String>,
    pub style_standard: Option<IpgStyleStandard>,
    pub style_arrow: Option<IpgButtonArrow>,
    pub user_data: Option<PyObject>,
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
        style_id: Option<String>,
        style_standard: Option<IpgStyleStandard>,
        style_arrow: Option<IpgButtonArrow>,
        user_data: Option<PyObject>,
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
            user_data,
            counter: 0,
            started: false,
            ticking: false,
            show,
        }
    }
}


#[derive(Debug, Clone)]
pub enum CanvasTimerMessage {
    OnStartStop,
}


#[derive(Debug, Clone)]
#[pyclass]
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

pub fn construct_canvas_timer(tim: IpgCanvasTimer) -> Element<'static, app::Message> {

    if !tim.show {
        return Space::new(Length::Shrink, Length::Shrink).into()
    }

    let mut label = Text::new(tim.label.clone());
    
    if tim.style_arrow.is_some() {
        let arrow = get_bootstrap_arrow(tim.style_arrow.unwrap());
        label = Text::new(arrow).font(iced::Font::with_name("bootstrap-icons"));
    }
    
    let timer_btn: Element<CanvasTimerMessage> = Button::new(label)
                                .height(tim.height)
                                .padding(tim.padding)
                                .width(tim.width)
                                .on_press(CanvasTimerMessage::OnStartStop)
                                .style(move|theme: &Theme, status| {
                                    get_styling(theme, status,
                                        None,
                                        tim.style_standard.clone(),
                                    )  })
                                .into();
    
    timer_btn.map(move |message: CanvasTimerMessage| app::Message::CanvasTimer(tim.id, message))

    
}

pub fn canvas_timer_callback(state: &mut IpgState, id: usize, started: bool) -> u64 {

    let mut wci = WidgetCallbackIn{id, ..Default::default()};
    wci.value_bool = Some(started);
    let mut wco: WidgetCallbackOut = set_or_get_widget_callback_data(state, wci);
    wco.id = id;
    let duration = wco.duration.unwrap_or(1);
    if wco.value_bool.unwrap() {
        wco.event_name = "on_start".to_string();
    } else {
        wco.event_name = "on_stop".to_string();
    }
    
    process_callback(wco);
    duration  
}

pub fn canvas_tick_callback(state: &mut IpgState) 
{
    let id= state.canvas_timer_event_id_enabled.0;
    let mut wci = WidgetCallbackIn{id, ..Default::default()};
    wci.value_str = Some("on_tick".to_string());
    let mut wco: WidgetCallbackOut = set_or_get_widget_callback_data(state, wci);
    wco.id = id;
    wco.event_name = "on_tick".to_string();
    process_callback(wco);
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
        None => panic!("Timer callback could not be found with id {}", wco.id),
    };

    let counter = wco.counter.unwrap_or(0);

    Python::with_gil(|py| {
            if wco.user_data.is_some() {
                let user_data = match wco.user_data {
                    Some(ud) => ud,
                    None => panic!("User Data could not be found in CanvasTimer callback"),
                };
                let res = callback.call1(py, (
                                                                    wco.id,
                                                                    counter,  
                                                                    user_data
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("CanvasTimer: 3 parameters (id, counter, user_data) are required or a python error in this function. {er}"),
                }
            } else {
                let res = callback.call1(py, (
                                                                    wco.id,
                                                                    counter,  
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("CanvasTimer: 2 parameters (id, counter) are required or a python error in this function. {er}"),
                }
            } 
    });
    
    drop(app_cbs);
}


pub fn canvas_timer_item_update(ctim: &mut IpgCanvasTimer,
                            item: PyObject,
                            value: PyObject,
                            )
{
    let update = try_extract_timer_update(item);

    match update {
        IpgCanvasTimerParam::DurationMs => {
            ctim.duration_ms = try_extract_i64(value) as u64;
        },
       IpgCanvasTimerParam::ArrowStyle => {
            ctim.style_arrow = Some(try_extract_button_arrow(value));
        },
        IpgCanvasTimerParam::Counter => {
            ctim.counter = try_extract_u64(value);
        }
        IpgCanvasTimerParam::Label => {
            ctim.label = try_extract_string(value);
        },
        IpgCanvasTimerParam::Height => {
            let val = try_extract_f64(value);
            ctim.height = get_height(Some(val as f32), false);
        },
        IpgCanvasTimerParam::HeightFill => {
            let val = try_extract_boolean(value);
            ctim.height = get_height(None, val);
        },
        IpgCanvasTimerParam::Padding => {
            let val = try_extract_vec_f64(value);
            ctim.padding =  get_padding_f64(val);
        },
        IpgCanvasTimerParam::Clip => {
            ctim.clip = try_extract_boolean(value);
        }
        IpgCanvasTimerParam::Show => {
            ctim.show = try_extract_boolean(value);
        },
        IpgCanvasTimerParam::StyleId => {
            let val = try_extract_string(value);
            ctim.style_id = Some(val);
        },
        IpgCanvasTimerParam::StyleStandard => {
            let val = try_extract_style_standard(value);
            ctim.style_standard = Some(val);
        },
        IpgCanvasTimerParam::Width => {
            let val = try_extract_f64(value);
            ctim.width = get_width(Some(val as f32), false);
        },
        IpgCanvasTimerParam::WidthFill => {
            let val = try_extract_boolean(value);
            ctim.width = get_width(None, val);
        }, 
    }

}

pub fn try_extract_timer_update(update_obj: PyObject) -> IpgCanvasTimerParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgCanvasTimerParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("CanvasTimer update extraction failed"),
        }
    })
}
