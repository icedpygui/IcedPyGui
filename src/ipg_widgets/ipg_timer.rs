//!ipg_timer
use crate::{access_callbacks, app};

use iced::widget::{Button, Text};
use iced::{Element, Length, Font, Padding, Theme};

// use iced_aw::BOOTSTRAP_FONT;
const ICON_FONT: Font = Font::with_name("icons");

use pyo3::{pyclass, PyObject, Python};

use super::callbacks::{get_set_widget_callback_data, WidgetCallbackIn, WidgetCallbackOut};
use super::helpers::try_extract_i64;
use super::ipg_button::{get_button_style, try_extract_button_arrow};



#[derive(Debug, Clone)]
pub struct IpgTimer {
    pub id: usize,
    pub duration_ms: u64,
    pub start_label: String,
    pub stop_label: String,
    pub width: Length,
    pub height: Length,
    pub padding: Padding,
    pub corner_radius: f32,
    pub style: Option<PyObject>,
    pub arrow_style: Option<PyObject>,
    pub user_data: Option<PyObject>,
    pub counter: u64,
    pub started: bool,
    pub ticking: bool,
}

impl IpgTimer {
    pub fn new(
        id: usize,
        duration_ms: u64,
        start_label: String,
        stop_label: String,
        width: Length,
        height: Length,
        padding: Padding,
        corner_radius: f32,
        style: Option<PyObject>,
        arrow_style: Option<PyObject>,
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
            corner_radius,
            style,
            arrow_style,
            user_data,
            counter: 0,
            started: false,
            ticking: false,
        }
    }
}


#[derive(Debug, Clone)]
pub enum TIMMessage {
    OnStart,
    OnStop,
}


#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgTimerParams {
    DurationMs,
}

pub fn construct_timer(tim: IpgTimer) -> Element<'static, app::Message> {

    let mut on_press = TIMMessage::OnStart;
    let mut label = Text::new(tim.start_label.clone());
    if tim.started {
        on_press = TIMMessage::OnStop;
        label = Text::new(tim.stop_label.clone());
    }

    if tim.arrow_style.is_some() {
        let arrow_style = try_extract_button_arrow(tim.arrow_style);
        label = match arrow_style {
            Some(ar) => Text::new(ar).font(ICON_FONT),
            None => panic!("Timer: Could not get extract arrow_style")
        };
    }
    
    let timer_btn: Element<TIMMessage> = Button::new(label)
                                .height(tim.height)
                                .padding(tim.padding)
                                .width(tim.width)
                                .on_press(on_press)
                                .style(move|theme: &Theme, status| {
                                    get_button_style(tim.style.clone(), theme, status, tim.corner_radius)})
                                .into();
    
    timer_btn.map(move |message: TIMMessage| app::Message::Timer(tim.id, message))

    
}

pub fn timer_callback(id: usize, message: TIMMessage) -> u64 {

    let mut wci = WidgetCallbackIn::default();
    wci.id = id;
    let mut duration: u64 = 0;

    match message {
        TIMMessage::OnStart => {
            wci.started = Some(true);
            wci.counter = Some(0);
            let mut wco: WidgetCallbackOut = get_set_widget_callback_data(wci);
            wco.id = id;
            duration = match wco.duration {
                Some(dur) => dur,
                None => 0,
            };
            wco.event_name = "on_start".to_string();
            process_callback(wco);
        }
        TIMMessage::OnStop => {
            wci.started = Some(false);
            let mut wco: WidgetCallbackOut = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = "on_stop".to_string();
            process_callback(wco);
        },
    }
    duration
}

pub fn tick_callback(id: usize) 
{
    let mut wci = WidgetCallbackIn::default();
    wci.id = id;

    wci.counter = Some(1);
    let mut wco: WidgetCallbackOut = get_set_widget_callback_data(wci);
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

    let counter = match wco.counter {
        Some(ct) => ct,
        None => 0,
    };

    Python::with_gil(|py| {
            if wco.user_data.is_some() {
                let user_data = match wco.user_data {
                    Some(ud) => ud,
                    None => panic!("User Data could not be found in Timer callback"),
                };
                let res = callback.call1(py, (
                                                                    wco.id.clone(),
                                                                    counter,  
                                                                    user_data
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Timer: 3 parameters (id, counter, user_data) are required or a python error in this function. {er}"),
                }
            } else {
                let res = callback.call1(py, (
                                                                    wco.id.clone(),
                                                                    counter,  
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Timer: 2 parameters (id, counter) are required or a python error in this function. {er}"),
                }
            } 
    });
    
    drop(app_cbs);
}


pub fn timer_item_update(tim: &mut IpgTimer,
                            item: PyObject,
                            value: PyObject,
                            )
{
    let update = try_extract_timer_update(item);

    match update {
        IpgTimerParams::DurationMs => {
            tim.duration_ms = try_extract_i64(value) as u64;
        },
        
    }

}


pub fn try_extract_timer_update(update_obj: PyObject) -> IpgTimerParams {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgTimerParams>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Timer update extraction failed"),
        }
    })
}
