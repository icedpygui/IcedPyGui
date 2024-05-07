

use crate::app;
use crate::access_callbacks;
use super::callbacks::{WidgetCallbackIn, 
                        WidgetCallbackOut, 
                        get_set_widget_callback_data};

use super::helpers::get_width;
use super::helpers::try_extract_boolean;
use super::helpers::try_extract_f64;

use iced::{Length, Element, Theme};
use iced::widget::{Slider, Space};

use pyo3::pyclass;
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

    if !slider.show {
        return Space::new(0.0, 0.0).into()
    }

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
            wco.event_name = "on_change".to_string();
            process_callback(wco);
        },
        SLMessage::OnRelease => {
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = "on_release".to_string();
            process_callback(wco);
        },
    }
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
        None => panic!("Slider Callback could not be found with id {}", wco.id),
    };

    let value = match wco.value_float {
        Some(vl) => vl,
        None => panic!("Slider value in callback could not be found"),
    };
                  
    Python::with_gil(|py| {
        if wco.user_data.is_some() {
            let user_data = match wco.user_data {
                Some(ud) => ud,
                None => panic!("Slider callback user_data not found."),
            };
            let res = callback.call1(py, (
                                                                wco.id.clone(), 
                                                                value, 
                                                                user_data
                                                                ));
            match res {
                Ok(_) => (),
                Err(er) => panic!("Slider: 3 parameters (id, value, user_data) are required or a python error in this function. {er}"),
            }
        } else {
            let res = callback.call1(py, (
                                                                wco.id.clone(), 
                                                                value, 
                                                                ));
            match res {
                Ok(_) => (),
                Err(er) => panic!("Slider: 2 parameters (id, value) are required or a python error in this function. {er}"),
            }
        } 
    });

    drop(app_cbs); 

}

#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgSliderParams {
    Min,
    Max,
    Step,
    Value,
    Width,
    WidthFill,
    Height,
    Show,
}

pub fn slider_item_update(sldr: &mut IpgSlider, item: PyObject, value: PyObject) {

    let update = try_extract_slider_update(item);

    match update {
        IpgSliderParams::Min => {
            sldr.min = try_extract_f64(value) as f32;
        },
        IpgSliderParams::Max => {
            sldr.max = try_extract_f64(value) as f32;
        },
        IpgSliderParams::Step => {
            sldr.step = try_extract_f64(value) as f32;
        },
        IpgSliderParams::Value => {
            sldr.value = try_extract_f64(value) as f32;
        },
        IpgSliderParams::Width => {
            let val = try_extract_f64(value);
            sldr.width = get_width(Some(val as f32), false);
        },
        IpgSliderParams::WidthFill => {
            let val = try_extract_boolean(value);
            sldr.width = get_width(None, val);
        },
        IpgSliderParams::Height => {
            sldr.height = try_extract_f64(value) as f32;
        },
        IpgSliderParams::Show => {
            sldr.show = try_extract_boolean(value);
        },
    }
}


fn try_extract_slider_update(update_obj: PyObject) -> IpgSliderParams {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgSliderParams>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Slider update extraction failed"),
        }
    })
}