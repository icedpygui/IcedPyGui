
use iced::border::Radius;
use iced::{Background, Border, Element, Length, Theme};
use iced::widget::{progress_bar, ProgressBar, Space};
use pyo3::{pyclass, PyObject, Python};
use crate::graphics::colors::{match_ipg_color, IpgColor};
use crate::{access_state, app};

use super::helpers::{get_height, get_width, try_extract_boolean, try_extract_f64};


#[derive(Debug, Clone)]
pub struct IpgProgressBar {
    pub id: usize,
    pub show: bool,
    
    pub min: f32,
    pub max: f32,
    pub value: f32,
    pub width: Length,
    pub height: Length,
    pub style_background: Option<String>,
    pub style_bar_color: Option<String>,
    pub style_border: Option<String>,
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
        style_background: Option<String>,
        style_bar_color: Option<String>,
        style_border: Option<String>,
    ) -> Self {
        Self {
            id,
            show,
            min,
            max,
            value,
            width,
            height,
            style_background,
            style_bar_color,
            style_border,
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
                                    bar.style_background.clone(), 
                                    bar.style_bar_color.clone(), 
                                    bar.style_border.clone(),
                                    )  
                                })
                            .into()
}


#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgProgressBarParams {
    Height,
    Min,
    Max,
    Show,
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
        IpgProgressBarParams::Height => {
            let val = try_extract_f64(value);
            pb.height = get_height(Some(val as f32), false);
        },
        IpgProgressBarParams::Min => {
            pb.min = try_extract_f64(value) as f32;
        },
        IpgProgressBarParams::Max => {
            pb.max = try_extract_f64(value) as f32;
        },
        IpgProgressBarParams::Show => {
            pb.show = try_extract_boolean(value);
        },
        IpgProgressBarParams::Value => {
            pb.value = try_extract_f64(value) as f32;
        },
        IpgProgressBarParams::Width => {
            let val = try_extract_f64(value);
            pb.width = get_width(Some(val as f32), false);
        },
        IpgProgressBarParams::WidthFill => {
            let val = try_extract_boolean(value);
            pb.width = get_width(None, val);
        },
    }
}


pub fn try_extract_progress_bar_update(update_obj: PyObject) -> IpgProgressBarParams {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgProgressBarParams>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("ProgressBar update extraction failed"),
        }
    })
}

pub fn get_styling(_theme: &Theme, 
                    style_background: Option<String>,
                    style_bar_color: Option<String>,
                    style_border: Option<String>, 
                    ) -> progress_bar::Style 
{
    let state = access_state();

    let background_opt = if style_background.is_some() {
        state.styling_background.get(&style_background.unwrap())
    } else {
        None
    };
    
    let background = match background_opt {
        Some(bg) => Background::Color(bg.color),
        None => Background::Color(match_ipg_color(IpgColor::TRANSPARENT)),
    };

    let border_opt = if style_border.is_some() {
        state.styling_border.get(&style_border.unwrap())
    } else {
        None
    };

    let border: Border = match border_opt {
        Some(bd) => Border {
            color: bd.color,
            radius: bd.radius,
            width: bd.width,
        },
        None => { Border {
                color: match_ipg_color(IpgColor::ANTIQUE_WHITE),
                radius: <Radius as std::default::Default>::default(),
                width: 1.0,
            }
        },
    };

    let bar_color_opt = if style_bar_color.is_some() {
        state.styling_icon_color.get(&style_bar_color.unwrap())
    } else {
        None
    };

    let bar = match bar_color_opt {
        Some(bc) => {
            Background::Color(bc.color)
        },
        None => Background::Color(match_ipg_color(IpgColor::GRAY)),
    };

    progress_bar::Style {
            background,
            bar,
            border,
    }
 

}
