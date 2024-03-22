

use crate::app;
use crate::access_callbacks;
use super::callbacks::{WidgetCallbackIn, 
                        WidgetCallbackOut, 
                        get_set_widget_callback_data};

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

pub fn slider_callback(id: usize, message: SLMessage) {

    let mut wci: WidgetCallbackIn = WidgetCallbackIn::default();
    wci.id = id;
           
    match message {
        SLMessage::OnChange(value) => {
            wci.value_float = Some(value as f64);
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = Some("on_change".to_string());
            process_callback(wco);
        },
        SLMessage::OnRelease => {
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = Some("on_release".to_string());
            process_callback(wco);
        },
    }
}


pub fn process_callback(wco: WidgetCallbackOut) 
{
    if !wco.event_name.is_some() {return}

    let evt_name = match wco.event_name {
        Some(name) => name,
        None => panic!("event_name not found")
    };

    let app_cbs = access_callbacks();

    let callback_opt = app_cbs.callbacks.get(&(wco.id, evt_name.clone())).unwrap();
       
    let callback = match callback_opt {
        Some(cb) => cb,
        None => panic!("Callback could not be found with id {}", wco.id),
    };
                  
    Python::with_gil(|py| {
            if wco.user_data.is_some() {
                callback.call1(py, (
                                        wco.id.clone(), 
                                        evt_name.clone(),
                                        wco.value_float, 
                                        wco.user_data
                                        )
                                ).unwrap();
            } else {
                callback.call1(py, (
                                        wco.id.clone(), 
                                        evt_name.clone(),
                                        wco.value_float, 
                                        )
                                ).unwrap();
            } 
    });

    drop(app_cbs); 

}
