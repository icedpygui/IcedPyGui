
use crate::ipg_widgets::ipg_enums::{IpgWidgets, get_set_widget_data};
use crate::app;
use crate::{access_state, access_callbacks};

use iced::{Length, Element, Theme};
use iced::widget::Slider;

use pyo3::{PyObject, Python};


#[derive(Debug, Clone)]
pub struct IpgSlider {
    pub id: usize,
    pub show: bool,
    pub user_data: Option<PyObject>,
    
    pub min: f32,
    pub max: f32,
    pub step: f32,
    pub value: f32,
    pub width: Length,
    pub height: f32,
    // style: <Renderer::Theme as StyleSheet>::Style,
    pub cb_name_change: Option<String>,
    pub cb_name_release: Option<String>,
}

impl IpgSlider {
    pub fn new( 
        id: usize,
        show: bool,
        user_data: Option<PyObject>,
        min: f32,
        max: f32,
        step: f32,
        value: f32,
        width: Length,
        height: f32,
        cb_name_change: Option<String>,
        cb_name_release: Option<String>,
    ) -> Self {
        Self {
            id,
            show,
            user_data,
            min,
            max,
            step,
            value,
            width,
            height,
            cb_name_change,
            cb_name_release,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SLMessage {
    OnChange(f32),
    OnRelease,
}

pub fn construct_slider(slider: IpgSlider) -> Element<'static, app::Message> {

    let sld: Element<SLMessage, Theme> = Slider::new(slider.min..=slider.max, 
                                                    slider.value, 
                                                    SLMessage::OnChange
                                                    )
                                                    .on_release(SLMessage::OnRelease)
                                                    .step(slider.step)
                                                    .width(slider.width)
                                                    .height(slider.height)
                                                    .into();

    sld.map(move |message| app::Message::Slider(slider.id, message))
}

pub fn slider_update(id: usize, message: SLMessage) {
    match message {
        SLMessage::OnChange(value) => {
            slider_on_change(id, value);
        },
        SLMessage::OnRelease => {
            slider_on_release(id);
        },
    }
}

fn slider_on_release(id: usize) {

    let (cb_name, user_data,_, value_opt) = 
                                    get_set_widget_data(
                                                        id, 
                                                        None, 
                                                        None, 
                                                        None, 
                                                        None,
                                                        );

    let event_name = "Slider".to_string();

    let value = match value_opt {
        Some(v) => v,
        None => panic!("Could not get the value of the slider"),
    };

    process_callback(id, 
                        event_name, 
                        value,
                        user_data,
                        cb_name
                    );
                    
}

pub fn slider_on_change(id: usize, value: f32) {

    let (cb_name, user_data,_,_) = 
                                    get_set_widget_data(
                                                        id, 
                                                        None, 
                                                        None, 
                                                        Some(vec![value as f64]), 
                                                        None,
                                                        );
    let event_name = "Slider".to_string();
                                    
    process_callback(
                        id, 
                        event_name, 
                        value as f64,
                        user_data,
                        cb_name
                    );
}

fn process_callback(id: usize, 
                    event_name: String,
                    value: f64,
                    user_data: Option<PyObject>, 
                    cb_name: Option<String>) 
{

    if !cb_name.is_some() {return};

    let app_cbs = access_callbacks();

    let mut found_callback = None;

    for callback in app_cbs.callbacks.iter() {

        if id == callback.id && cb_name == callback.name {

            found_callback = match callback.cb.clone() {
                                        Some(cb) => Some(cb),
                                        None => {
                                            panic!("Callback could not be found with id {}", id)},
                                    };
            break;
            }                   
    };
    drop(app_cbs);

    match found_callback {

    Some(cb) => Python::with_gil(|py| {
                match user_data {
                    Some(ud) => cb.call1(py, 
                                                    (
                                                        id.clone(),
                                                        event_name,
                                                        value, 
                                                        ud,
                                                    )).unwrap(),
                    None => cb.call1(py, 
                                    (
                                        id.clone(), 
                                        event_name,
                                        value,
                                    )).unwrap(),
                };
            }),
    None => panic!("Slider callback not found"),
    };

}
