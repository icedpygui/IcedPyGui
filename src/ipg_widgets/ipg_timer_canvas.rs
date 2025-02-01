//! ipg_timer
use crate::style::styling::IpgStyleStandard;
use crate::{access_callbacks, app, IpgState};
use super::callbacks::{set_or_get_widget_callback_data, WidgetCallbackIn, WidgetCallbackOut};
use super::helpers::try_extract_i64;
use super::ipg_button::{get_bootstrap_arrow, get_styling, 
    IpgButtonArrow};

use iced::widget::{Button, Text};
use iced::{Element, Length, Padding, Theme};

use pyo3::{pyclass, PyObject, Python};

#[derive(Debug, Clone)]
pub struct IpgCanvasTimer {
    pub id: usize,
    pub duration_ms: u64,
    pub start_label: String,
    pub stop_label: String,
    pub width: Length,
    pub height: Length,
    pub padding: Padding,
    pub button_style_id: Option<String>,
    pub button_style_standard: Option<IpgStyleStandard>,
    pub button_style_arrow: Option<IpgButtonArrow>,
    pub user_data: Option<PyObject>,
    pub counter: u64,
    pub started: bool,
    pub ticking: bool,
}

impl IpgCanvasTimer {
    pub fn new(
        id: usize,
        duration_ms: u64,
        start_label: String,
        stop_label: String,
        width: Length,
        height: Length,
        padding: Padding,
        button_style_id: Option<String>,
        button_style_standard: Option<IpgStyleStandard>,
        button_style_arrow: Option<IpgButtonArrow>,
        user_data: Option<PyObject>,
        ) -> Self {
        Self {
            id,
            duration_ms,
            start_label,
            stop_label,
            width,
            height,
            padding,
            button_style_id,
            button_style_standard,
            button_style_arrow,
            user_data,
            counter: 0,
            started: false,
            ticking: false,
        }
    }
}


#[derive(Debug, Clone)]
pub enum CanvasTimerMessage {
    OnStart,
    OnStop,
}


#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgCanvasTimerParams {
    DurationMs,
}

pub fn construct_canvas_timer(tim: IpgCanvasTimer) -> Element<'static, app::Message> {

    let mut on_press = CanvasTimerMessage::OnStart;
    let mut label = Text::new(tim.start_label.clone());
    if tim.started {
        on_press = CanvasTimerMessage::OnStop;
        label = Text::new(tim.stop_label.clone());
    }

    if tim.button_style_arrow.is_some() {
        let arrow = get_bootstrap_arrow(tim.button_style_arrow.unwrap());
        label = Text::new(arrow).font(iced::Font::with_name("bootstrap-icons"));
    }
    
    let timer_btn: Element<CanvasTimerMessage> = Button::new(label)
                                .height(tim.height)
                                .padding(tim.padding)
                                .width(tim.width)
                                .on_press(on_press)
                                .style(move|theme: &Theme, status| {
                                    get_styling(theme, status,
                                        None,
                                        tim.button_style_standard.clone(),
                                    )  })
                                .into();
    
    timer_btn.map(move |message: CanvasTimerMessage| app::Message::CanvasTimer(tim.id, message))

    
}

pub fn canvas_timer_callback(state: &mut IpgState, id: usize, message: CanvasTimerMessage) -> u64 {

    let mut wci = WidgetCallbackIn{id, ..Default::default()};

    let mut duration: u64 = 0;

    match message {
        CanvasTimerMessage::OnStart => {
            wci.started = Some(true);
            wci.counter = Some(0);
            let mut wco: WidgetCallbackOut = set_or_get_widget_callback_data(state, wci);
            wco.id = id;
            duration = wco.duration.unwrap_or(0);
            wco.event_name = "on_start".to_string();
            process_callback(wco);
        }
        CanvasTimerMessage::OnStop => {
            wci.started = Some(false);
            let mut wco: WidgetCallbackOut = set_or_get_widget_callback_data(state, wci);
            wco.id = id;
            wco.event_name = "on_stop".to_string();
            process_callback(wco);
        },
    }
    duration
}

pub fn canvas_tick_callback(state: &mut IpgState) 
{
    let id= state.canvas_timer_event_id_enabled.0;
    let wci = WidgetCallbackIn{id, ..Default::default()};

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
        IpgCanvasTimerParams::DurationMs => {
            ctim.duration_ms = try_extract_i64(value) as u64;
        },
    }

}

pub fn try_extract_timer_update(update_obj: PyObject) -> IpgCanvasTimerParams {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgCanvasTimerParams>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("CanvasTimer update extraction failed"),
        }
    })
}
