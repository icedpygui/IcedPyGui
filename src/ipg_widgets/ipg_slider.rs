//! ipg_slider
use crate::graphics::colors::get_color;
use crate::{access_callbacks, access_user_data, app, IpgState};
use super::callbacks::{set_or_get_widget_callback_data, 
    WidgetCallbackIn};

use super::helpers::{get_radius, get_width, try_extract_ipg_color, 
    try_extract_rgba_color, try_extract_u16, try_extract_vec_f32, 
    try_extract_boolean, try_extract_f64};
use super::ipg_enums::IpgWidgets;

use iced::border::Radius;
use iced::widget::slider::{self, HandleShape, Status, Style};
use iced::{Background, Color, Element, Length, Theme};
use iced::widget::Slider;

use pyo3::{PyObject, pyclass, Python};


#[derive(Debug, Clone)]
pub struct IpgSlider {
    pub id: usize,
    pub show: bool,
    
    pub min: f32,
    pub max: f32,
    pub step: f32,
    pub value: f32,
    pub width: Length,
    pub height: f32,
    pub style_id: Option<usize>,
}

impl IpgSlider {
    pub fn new( 
        id: usize,
        show: bool,
        min: f32,
        max: f32,
        step: f32,
        value: f32,
        width: Length,
        height: f32,
        style_id: Option<usize>,
    ) -> Self {
        Self {
            id,
            show,
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

pub fn construct_slider<'a>(slider: &'a IpgSlider, 
                        style_opt: Option<&IpgWidgets>) 
                        -> Option<Element<'a, app::Message>> {

    if !slider.show {
        return None
    }

    let style = get_slider_style(style_opt);

    let sld: Element<SLMessage, Theme> = 
        Slider::new(slider.min..=slider.max, 
                    slider.value, 
                    SLMessage::OnChange
                    )
                    .on_release(SLMessage::OnRelease)
                    .step(slider.step)
                    .width(slider.width)
                    .height(slider.height)
                    .style(move|theme, status|
                    get_styling(theme, status,
                        style.clone()
                    ))
                    .into();

    Some(sld.map(move |message| app::Message::Slider(slider.id, message)))
}

pub fn slider_callback(state: &mut IpgState, id: usize, message: SLMessage) {

    let mut wci: WidgetCallbackIn = WidgetCallbackIn{id, ..Default::default()};
           
    match message {
        SLMessage::OnChange(value) => {
            wci.value_float_64 = Some(value as f64);
            let _ = set_or_get_widget_callback_data(state, wci);
            process_callback(id, "on_change".to_string(), Some(value));
        },
        SLMessage::OnRelease => {
            process_callback(id, "on_release".to_string(), None);
        },
    }
}

pub fn process_callback(id: usize, event_name: String, on_change_value: Option<f32>) 
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
        if user_data_opt.is_some() && on_change_value.is_some() {
            
            let res = cb.call1(py, (
                                                        id, 
                                                        on_change_value.unwrap(), 
                                                        user_data_opt,
                                                        ));
            match res {
                Ok(_) => (),
                Err(er) => panic!("Slider: 3 parameters (id, value, user_data) 
                                    are required or a python error in this function. {er}"),
            }
        } else if on_change_value.is_some() {
            let res = cb.call1(py, (
                                                        id, 
                                                        on_change_value.unwrap(), 
                                                        ));
            match res {
                Ok(_) => (),
                Err(er) => panic!("Slider: 2 parameters (id, value) 
                                    are required or a python error in this function. {er}"),
            }
        } else {
            let res = cb.call1(py, (
                                                        id,  
                                                        ));
            match res {
                Ok(_) => (),
                Err(er) => panic!("Slider: 1 parameter (id) 
                                    is required or a python error in this function. {er}"),
            }
        }
    });

    drop(ud); 

}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
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

pub fn slider_item_update(sldr: &mut IpgSlider, 
                            item: &PyObject, 
                            value: &PyObject) {

    let update = try_extract_slider_update(item);
    let name = "Slider".to_string();
    match update {
        IpgSliderParam::Min => {
            sldr.min = try_extract_f64(value, name) as f32;
        },
        IpgSliderParam::Max => {
            sldr.max = try_extract_f64(value, name) as f32;
        },
        IpgSliderParam::Step => {
            sldr.step = try_extract_f64(value, name) as f32;
        },
        IpgSliderParam::Value => {
            sldr.value = try_extract_f64(value, name) as f32;
        },
        IpgSliderParam::Width => {
            let val = try_extract_f64(value, name);
            sldr.width = get_width(Some(val as f32), false);
        },
        IpgSliderParam::WidthFill => {
            let val = try_extract_boolean(value, name);
            sldr.width = get_width(None, val);
        },
        IpgSliderParam::Height => {
            sldr.height = try_extract_f64(value, name) as f32;
        },
        IpgSliderParam::Style => {
            sldr.style_id = Some(try_extract_f64(value, name) as usize);
        }
        IpgSliderParam::Show => {
            sldr.show = try_extract_boolean(value, name);
        },
    }
}


fn try_extract_slider_update(update_obj: &PyObject) -> IpgSliderParam {

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
        base_style.handle.background = Background::Color(style.handle_color.unwrap());
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

    let mut hovered_style = base_style;

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

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgSliderStyleParam {
    RailIpgColor,
    RailRbgaColor,
    RailIpgColorHovered,
    RailIpgRgbaHovered,
    RailBorderRadius,
    RailWidth,

    HandleIpgColor,
    HandleRgbaColor,
    HandleBorderIpgColor,
    HandleBorderRgbaColor,
    HandleBorderWidth,
    HandleCircleRadius,
    HandleRectangleWidth,
    HandleRectangleBorderRadius,
}

pub fn slider_style_update_item(style: &mut IpgSliderStyle,
                            item: &PyObject,
                            value: &PyObject,) 
{
    let update = try_extract_slider_style_update(item);
    let name = "SliderStyle".to_string();
    match update {
        IpgSliderStyleParam::RailIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.rail_color = get_color(None, Some(color), 1.0, false);
        },
        IpgSliderStyleParam::RailRbgaColor => {
            style.rail_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgSliderStyleParam::RailIpgColorHovered => {
            let color = try_extract_ipg_color(value, name);
            style.rail_color_hovered = get_color(None, Some(color), 1.0, false);
        },
        IpgSliderStyleParam::RailIpgRgbaHovered => {
            style.rail_color_hovered = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgSliderStyleParam::RailBorderRadius => {
            style.rail_border_radius = Some(try_extract_vec_f32(value, name));
        },
        IpgSliderStyleParam::RailWidth => {
            style.rail_width = Some(try_extract_f64(value, name) as f32);
        },
        IpgSliderStyleParam::HandleIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.handle_color = get_color(None, Some(color), 1.0, false);
        },
        IpgSliderStyleParam::HandleRgbaColor => {
            style.handle_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgSliderStyleParam::HandleBorderIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.handle_border_color = get_color(None, Some(color), 1.0, false);
        },
        IpgSliderStyleParam::HandleBorderRgbaColor => {
            style.handle_border_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgSliderStyleParam::HandleBorderWidth => {
            style.handle_border_width = Some(try_extract_f64(value, name) as f32);
        },
        IpgSliderStyleParam::HandleCircleRadius => {
            style.handle_circle_radius = Some(try_extract_f64(value, name) as f32);
        },
        IpgSliderStyleParam::HandleRectangleWidth => {
            style.handle_rectangle_width = Some(try_extract_u16(value, name));
        },
        IpgSliderStyleParam::HandleRectangleBorderRadius => {
            style.handle_rectangle_border_radius = Some(try_extract_vec_f32(value, name));
        },
    }
}

pub fn try_extract_slider_style_update(update_obj: &PyObject) -> IpgSliderStyleParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgSliderStyleParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Slider style update extraction failed"),
        }
    })
}

fn get_slider_style(style: Option<&IpgWidgets>) -> Option<IpgSliderStyle>{
    match style {
        Some(st) => {
            match st {
                IpgWidgets::IpgSliderStyle(style) => {
                    Some(style.clone())
                }
                _ => None,
            }
        },
        None => None,
    }
}