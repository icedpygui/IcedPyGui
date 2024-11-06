//! ipg_slider
use crate::{access_callbacks, app, IpgState};
use super::callbacks::{set_or_get_widget_callback_data, WidgetCallbackIn, WidgetCallbackOut};

use super::helpers::{get_radius, get_width};
use super::helpers::{try_extract_boolean, try_extract_f64, 
    try_extract_string};

use iced::border::Radius;
use iced::widget::slider::{self, HandleShape, Status, Style};
use iced::{Background, Color, Element, Length, Theme};
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
    pub style_id: Option<String>,
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
        style_id: Option<String>,
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
            style_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IpgSliderStyle {
    pub id: usize,
    pub rail_color: Option<Color>,
    pub rail_color_hovered: Option<Color>,
    pub rail_width: Option<f32>,
    pub rail_border_radius: Option<Vec<f32>>,
    pub handle_circle_radius: Option<f32>,
    pub handle_rectangle_width: Option<u16>,
    pub handle_rectangle_border_radius: Option<Vec<f32>>,
    pub handle_color: Option<Color>,
    pub handle_border_width: Option<f32>,
    pub handle_border_color: Option<Color>,
}

impl IpgSliderStyle {
    pub fn new( 
        id: usize,
        rail_color: Option<Color>,
        rail_color_hovered: Option<Color>,
        rail_width: Option<f32>,
        rail_border_radius: Option<Vec<f32>>,
        handle_circle_radius: Option<f32>,
        handle_rectangle_width: Option<u16>,
        handle_rectangle_border_radius: Option<Vec<f32>>,
        handle_color: Option<Color>,
        handle_border_width: Option<f32>,
        handle_border_color: Option<Color>,
    ) -> Self {
        Self {
            id,
            rail_color,
            rail_color_hovered,
            rail_width,
            rail_border_radius,
            handle_circle_radius,
            handle_rectangle_width,
            handle_rectangle_border_radius,
            handle_color,
            handle_border_width,
            handle_border_color,
        }
    }
}


#[derive(Debug, Clone)]
pub enum SLMessage {
    OnChange(f32),
    OnRelease,
}

pub fn construct_slider(slider: IpgSlider, 
                        style: Option<&IpgSliderStyle>) 
                        -> Element<'static, app::Message> {

    // extracted here due to lifetime in map statement
    let style_opt = match style {
        Some(st) => Some(st.clone()),
        None => None,
    };

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
                                                    .style(move|theme, status|
                                                    get_styling(theme, status,
                                                        style_opt.clone()
                                                    ))
                                                    .into();

    sld.map(move |message| app::Message::Slider(slider.id, message))
}

pub fn slider_callback(state: &mut IpgState, id: usize, message: SLMessage) {

    let mut wci: WidgetCallbackIn = WidgetCallbackIn::default();
    wci.id = id;
           
    match message {
        SLMessage::OnChange(value) => {
            wci.value_float = Some(value as f64);
            let mut wco = set_or_get_widget_callback_data(state, wci);
            wco.id = id;
            wco.event_name = "on_change".to_string();
            process_callback(wco);
        },
        SLMessage::OnRelease => {
            let mut wco = set_or_get_widget_callback_data(state, wci);
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
pub enum IpgSliderParam {
    Min,
    Max,
    Step,
    Value,
    Width,
    WidthFill,
    Height,
    Style,
    Show,
}

pub fn slider_item_update(sldr: &mut IpgSlider, item: PyObject, value: PyObject) {

    let update = try_extract_slider_update(item);

    match update {
        IpgSliderParam::Min => {
            sldr.min = try_extract_f64(value) as f32;
        },
        IpgSliderParam::Max => {
            sldr.max = try_extract_f64(value) as f32;
        },
        IpgSliderParam::Step => {
            sldr.step = try_extract_f64(value) as f32;
        },
        IpgSliderParam::Value => {
            sldr.value = try_extract_f64(value) as f32;
        },
        IpgSliderParam::Width => {
            let val = try_extract_f64(value);
            sldr.width = get_width(Some(val as f32), false);
        },
        IpgSliderParam::WidthFill => {
            let val = try_extract_boolean(value);
            sldr.width = get_width(None, val);
        },
        IpgSliderParam::Height => {
            sldr.height = try_extract_f64(value) as f32;
        },
        IpgSliderParam::Style => {
            sldr.style_id = Some(try_extract_string(value));
        }
        IpgSliderParam::Show => {
            sldr.show = try_extract_boolean(value);
        },
    }
}


fn try_extract_slider_update(update_obj: PyObject) -> IpgSliderParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgSliderParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Slider update extraction failed"),
        }
    })
}

fn get_styling(theme: &Theme, 
                status: Status,
                style_opt: Option<IpgSliderStyle>) 
                -> Style {

    if style_opt.is_none() {
        return slider::default(theme, status)
    }     

    let style = style_opt.unwrap();

    let mut base_style = slider::default(theme, status);

    if style.handle_color.is_some() {
        base_style.handle.border_color = style.handle_color.unwrap()
    };


    if style.rail_color.is_some() {
        base_style.rail.backgrounds = (Background::Color(style.rail_color.unwrap()), 
                                        Background::Color(style.rail_color.unwrap()));
    }

    if style.rail_width.is_some() {
        base_style.rail.width = style.rail_width.unwrap();
    }

    if style.rail_border_radius.is_some() {
        base_style.rail.border.radius = get_radius(style.rail_border_radius.clone().unwrap(),
                                            "Slider".to_string());
    }

    if style.handle_circle_radius.is_some() {
        base_style.handle.shape = HandleShape::Circle{radius: style.handle_circle_radius.unwrap() };
    }

    let mut shape_rect: (u16, Radius) = (14, get_radius(vec![1.0],
                                                "Slider".to_string())); 

    if style.handle_rectangle_border_radius.is_some() {
        shape_rect.1 = get_radius(style.handle_rectangle_border_radius.clone().unwrap(),
                                "Slider".to_string());
    }

    if style.handle_rectangle_width.is_some() {
        shape_rect.0 = style.handle_rectangle_width.unwrap();
    }

    if style.handle_rectangle_border_radius.is_some() || style.handle_rectangle_width.is_some() {
        base_style.handle.shape = HandleShape::Rectangle { width: shape_rect.0, border_radius: shape_rect.1 }
    }

    if style.handle_border_color.is_some() {
        base_style.handle.border_color = style.handle_border_color.unwrap();
    }

    if style.handle_border_width.is_some() {
        base_style.handle.border_width = style.handle_border_width.unwrap();
    }

    let mut hovered_style = base_style.clone();

    if style.rail_color_hovered.is_some() {
        hovered_style.rail.border.color = style.rail_color_hovered.unwrap();
    }

    match status 
    {
        Status::Active => base_style,
        Status::Hovered => hovered_style,
        Status::Dragged => base_style, // active and drag are same
    }


}
