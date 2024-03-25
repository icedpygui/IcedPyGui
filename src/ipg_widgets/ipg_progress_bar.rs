
use iced::{Element, Length};
use iced::widget::ProgressBar;
use pyo3::PyObject;
use crate::app;

use super::helpers::try_extract_f64;


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

pub fn progress_bar_item_update(pb: &mut IpgProgressBar,
                                item: String,
                                value: PyObject,
                                )
{

    if item == "value".to_string() {
        pb.value = try_extract_f64(value) as f32;
    }
    
}
