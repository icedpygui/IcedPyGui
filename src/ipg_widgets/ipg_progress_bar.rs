
use iced::{Color, Element, Length, Theme};
use iced::widget::{progress_bar, ProgressBar, Space};
use pyo3::{pyclass, PyObject, Python};
use crate::style::styling::IpgStyleStandard;
use crate::{access_state, app};

use super::helpers::{get_height, get_radius, get_width, try_extract_boolean, try_extract_f64, try_extract_string, try_extract_style_standard};


#[derive(Debug, Clone)]
pub struct IpgProgressBar {
    pub id: usize,
    pub show: bool,
    
    pub min: f32,
    pub max: f32,
    pub value: f32,
    pub width: Length,
    pub height: Length,
    pub style_standard: Option<IpgStyleStandard>,
    pub style: Option<String>,
}

impl IpgProgressBar {
    pub fn new( 
        id: usize,
        show: bool,
        min: f32,
        max: f32,
        value: f32,
        width: Length,
        height: Length,
        style_standard: Option<IpgStyleStandard>,
        style: Option<String>,
    ) -> Self {
        Self {
            id,
            show,
            min,
            max,
            value,
            width,
            height,
            style_standard,
            style,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct IpgProgressBarStyle {
    pub id: usize,
    pub background_color: Option<Color>,
    pub bar_color: Option<Color>,
    pub border_color: Option<Color>,
    pub border_radius: Option<Vec<f32>>,
    pub border_width: Option<f32>,
}

impl IpgProgressBarStyle {
    pub fn new(
        id: usize,
        background_color: Option<Color>,
        bar_color: Option<Color>,
        border_color: Option<Color>,
        border_radius: Option<Vec<f32>>,
        border_width: Option<f32>,
    ) -> Self {
        Self {
            id,
            background_color,
            bar_color,
            border_color,
            border_radius,
            border_width,
        }
    }
}


pub fn construct_progress_bar(bar: IpgProgressBar) -> Element<'static, app::Message> {
    
    if !bar.show {
        return Space::new(0.0, 0.0).into();
    }

    ProgressBar::new(bar.min..=bar.max, bar.value)
                            .width(bar.width)
                            .height(bar.height)
                            .style(move|theme: &Theme | {   
                                get_styling(theme, 
                                    bar.style_standard.clone(), 
                                    bar.style.clone(), 
                                    )  
                                })
                            .into()
}


#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgProgressBarParam {
    Height,
    Min,
    Max,
    Show,
    StyleStandard,
    Style,
    Value,
    Width,
    WidthFill,
}

pub fn progress_bar_item_update(pb: &mut IpgProgressBar,
                                item: PyObject,
                                value: PyObject,
                                )
{
    let update = try_extract_progress_bar_update(item);

    match update {
        IpgProgressBarParam::Height => {
            let val = try_extract_f64(value);
            pb.height = get_height(Some(val as f32), false);
        },
        IpgProgressBarParam::Min => {
            pb.min = try_extract_f64(value) as f32;
        },
        IpgProgressBarParam::Max => {
            pb.max = try_extract_f64(value) as f32;
        },
        IpgProgressBarParam::Show => {
            pb.show = try_extract_boolean(value);
        },
        IpgProgressBarParam::StyleStandard => {
            pb.style_standard = Some(try_extract_style_standard(value))
        },
        IpgProgressBarParam::Style => {
            pb.style = Some(try_extract_string(value))
        },
        IpgProgressBarParam::Value => {
            pb.value = try_extract_f64(value) as f32;
        },
        IpgProgressBarParam::Width => {
            let val = try_extract_f64(value);
            pb.width = get_width(Some(val as f32), false);
        },
        IpgProgressBarParam::WidthFill => {
            let val = try_extract_boolean(value);
            pb.width = get_width(None, val);
        },
    }
}


pub fn try_extract_progress_bar_update(update_obj: PyObject) -> IpgProgressBarParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgProgressBarParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("ProgressBar update extraction failed"),
        }
    })
}

pub fn get_styling(theme: &Theme,
                    style_standard: Option<IpgStyleStandard>,
                    style_str: Option<String>, 
                    ) -> progress_bar::Style 
{
    let state = access_state();

    if style_standard.is_none() && style_str.is_none() {
        return progress_bar::primary(theme)
    }

    let style_opt = if style_str.is_some() {
        state.progress_bar_style.get(&style_str.clone().unwrap())
    } else {
        None
    };

    if style_str.is_some() && style_opt.is_none() {
        panic!("ProgressBar style: Unable to find style_id {}.", style_str.unwrap())
    }


    if style_standard.is_some() {
        let style_std = style_standard.unwrap().clone();
        
        let mut std_style = match style_std {
            IpgStyleStandard::Primary => {
                progress_bar::primary(theme)
            },
            IpgStyleStandard::Success => {
                progress_bar::success(theme)
            },
            IpgStyleStandard::Danger => {
                progress_bar::danger(theme)
            },
            IpgStyleStandard::Text => panic!("IpgStandardStyle.Text is not valid for progress bar"),
        };

        if style_opt.is_some() {
            let custom = style_opt.unwrap();
            if custom.border_color.is_some() {
                std_style.border.color = custom.border_color.unwrap();
            }
            if custom.border_width.is_some() {
                 std_style.border.width = custom.border_width.unwrap();
            }
            if custom.border_radius.is_some() {
                std_style.border.radius = get_radius(custom.border_radius.clone().unwrap(),
                                            "ProgressBar".to_string());
            }
        }
        return std_style
    }


    let mut custom = progress_bar::primary(theme);

    //tested above so should unwrap()
    let style = style_opt.unwrap();
    
    if style.background_color.is_some() {
        custom.background = style.background_color.unwrap().into();
    }

    if style.bar_color.is_some() {
        custom.bar = style.bar_color.unwrap().into();
    }

    if style.border_color.is_some() {
        custom.border.color = style.border_color.unwrap();
    }
    if style.border_width.is_some() {
         custom.border.width = style.border_width.unwrap();
    }
    if style.border_radius.is_some() {
        custom.border.radius = get_radius(style.border_radius.clone().unwrap(),
                                    "ProgressBar".to_string());
    }

    custom
 
}
