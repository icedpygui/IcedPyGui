
use iced::{Color, Element, Length, Theme, theme};
use iced::widget::{progress_bar, ProgressBar, Space};
use pyo3::{pyclass, PyObject, Python};
use crate::style::styling::IpgStyleStandard;
use crate::{access_state, app};

use super::helpers::{get_height, get_width, try_extract_boolean, try_extract_f64, try_extract_string, try_extract_style_standard};


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
    pub style_color: Option<String>,
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
        style_standard: Option<IpgStyleStandard>,
        style_color: Option<String>,
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
            style_standard,
            style_color,
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
                                    bar.style_standard.clone(), 
                                    bar.style_color.clone(), 
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
    StyleStandard,
    StyleColor,
    StyleBorder,
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
        IpgProgressBarParams::StyleStandard => {
            pb.style_standard = Some(try_extract_style_standard(value))
        },
        IpgProgressBarParams::StyleColor => {
            pb.style_color = Some(try_extract_string(value))
        },
        IpgProgressBarParams::StyleBorder => {
            pb.style_border = Some(try_extract_string(value))
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

pub fn get_styling(theme: &Theme,
                    style_standard: Option<IpgStyleStandard>,
                    style_color: Option<String>,
                    style_border: Option<String>, 
                    ) -> progress_bar::Style 
{
    let state = access_state();

    if style_standard.is_none() && style_color.is_none() {
        return progress_bar::primary(theme)
    }

    let border_opt = if style_border.is_some() {
        state.styling_border.get(&style_border.unwrap())
    } else {
        None
    };

    let mut base_style = progress_bar::primary(theme);

    if border_opt.is_some() {
        let border = border_opt.unwrap();
        base_style.border.radius = border.radius;
        base_style.border.width = border.width;
    }

    let palette = theme.extended_palette();

    if style_standard.is_some() {
        let style_std = style_standard.unwrap().clone();
        
        // if border is used, will use the standard color
        let mut border_color = palette.primary.strong.color;

        let mut style = match style_std {
            IpgStyleStandard::Primary => {
                progress_bar::primary(theme)
            },
            IpgStyleStandard::Success => {
                border_color = palette.success.strong.color;
                progress_bar::success(theme)
            },
            IpgStyleStandard::Danger => {
                border_color = palette.danger.strong.color;
                progress_bar::danger(theme)
            },
            IpgStyleStandard::Text => panic!("IpgStandardStyle.Text is not valid for progress bar"),
        };

        if border_opt.is_some() {
            style.border.color = border_color;
            style.border.width = base_style.border.width;
            style.border.radius = base_style.border.radius;
        }

        return style
    }

    let color_palette_opt = if style_color.is_some() {
        state.styling_color.get(&style_color.unwrap())
    } else {
        None
    };
    
    if color_palette_opt.is_some() {
        let text = if palette.is_dark {
            Color::WHITE
        } else {
            Color::BLACK
        };

        let mut color_palette = color_palette_opt.unwrap().clone();
        
        if color_palette.base.is_none() {
            color_palette.base = Some(Color::TRANSPARENT);
        }

        let background = theme::palette::Background::new(color_palette.base.unwrap(), text);
        base_style.background = iced::Background::Color(background.weak.color);
        
        if color_palette.border.is_some() {
            base_style.border.color = color_palette.border.unwrap();
        }

    }

    base_style
 
}
