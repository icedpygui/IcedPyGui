
use iced::{Element, Length};
use iced::widget::{ProgressBar, Space};
use pyo3::{pyclass, PyObject, Python};
use crate::app;

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
    // style: <Renderer::Theme as StyleSheet>::Style,
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
        // style: <Renderer::Theme as StyleSheet>::Style,
    ) -> Self {
        Self {
            id,
            show,
            min,
            max,
            value,
            width,
            height,
            // style,
        }
    }
}

pub fn construct_progress_bar(bar: &IpgProgressBar) -> Element<'static, app::Message> {
    
    if !bar.show {
        return Space::new(0.0, 0.0).into();
    }

    ProgressBar::new(bar.min..=bar.max, bar.value)
                            .width(bar.width)
                            .height(bar.height)
                            .into()
}

// fn progress_bar_custom_style(theme: &Theme) -> progress_bar::Appearance {
//     progress_bar::Appearance {
//         background: theme.extended_palette().background.strong.color.into(),
//         bar: Color::from_rgb8(250, 85, 134).into(),
//         border_radius: 0.0.into(),
//     }
// }


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
