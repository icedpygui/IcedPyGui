
use iced::{Color, Element, Length};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::text::{LineHeight, Shaping, Style};
use iced::widget::{Space, Text};
use crate::app::Message;
use crate::graphics::colors::get_color;

use pyo3::{pyclass, PyObject, Python};

use super::helpers::{get_height, get_width, try_extract_boolean, try_extract_f64, try_extract_ipg_color, try_extract_string, try_extract_vec_f32};

#[derive(Debug, Clone)]
pub struct IpgText {
    pub id: usize,
    pub content: String,
    pub size: f32,
    pub line_height: LineHeight,
    pub width: Length,
    pub height: Length,
    pub horizontal_alignment: Horizontal,
    pub vertical_alignment: Vertical,
    // pub font: Font,
    pub shaping: Shaping,
    pub show: bool,
    pub style: Option<Color>,
}

impl IpgText {
    pub fn new( 
        id: usize,
        content: String,
        size: f32,
        line_height: LineHeight,
        width: Length,
        height: Length,
        horizontal_alignment: Horizontal,
        vertical_alignment: Vertical,
        // font: Font,
        shaping: Shaping,
        show: bool,
        style: Option<Color>,
        ) -> Self {
        Self {
            id,
            content,
            size,
            line_height,
            width,
            height,
            horizontal_alignment,
            vertical_alignment,
            // font,
            shaping,
            show,
            style,
        }
    }
}

pub fn construct_text(text: IpgText) -> Element<'static, Message> {

    if !text.show {
        return Space::new(Length::Shrink, Length::Shrink).into()
    }

    Text::new(text.content.clone()
                        )
                        .size(text.size)
                        .line_height(text.line_height)
                        .width(text.width)
                        .height(text.height)
                        .align_x(text.horizontal_alignment)
                        .align_y(text.vertical_alignment)
                        // .font()
                        .shaping(text.shaping)
                        .style(move|_theme|{
                            let mut style = Style::default();
                            style.color = text.style;
                            style
                            }
                        )
                        .into() 
}


#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgTextParam {
    Content,
    // Font,
    Height,
    HeightFill,
    HzAlignLeft,
    HzAlignCenter,
    HzAlignRight,
    LineHeight,
    Show,
    // Shaping,
    Size,
    TextColor, 
    TextRgba,
    VtAlignTop,
    VtAlignCenter,
    VtAlignBottom,
    Width,
    WidthFill,
}

pub fn text_item_update(txt: &mut IpgText, item: PyObject, value: PyObject) {

    let update = try_extract_text_update(item);

    match update {
        IpgTextParam::Content => {
            txt.content = try_extract_string(value);
        },
        IpgTextParam::Height => {
            let val = try_extract_f64(value);
            txt.height = get_height(Some(val as f32), false); 
        },
        IpgTextParam::HeightFill => {
            let val = try_extract_boolean(value);
            txt.height = get_height(None, val);
        },
        IpgTextParam::HzAlignLeft => {
            txt.horizontal_alignment = Horizontal::Left;
        },
        IpgTextParam::HzAlignCenter => {
            txt.horizontal_alignment = Horizontal::Center;
        },
        IpgTextParam::HzAlignRight => {
            txt.horizontal_alignment = Horizontal::Right;
        },
        IpgTextParam::LineHeight => {
            let val = try_extract_f64(value) as f32;
            txt.line_height = LineHeight::Relative(val);
        },
        IpgTextParam::Show => {
            txt.show = try_extract_boolean(value);
        },
        IpgTextParam::Size => {
            txt.size = try_extract_f64(value) as f32;
        },
        IpgTextParam::TextColor => {
            let ipg_color = Some(try_extract_ipg_color(value));
            txt.style = get_color(None, ipg_color, 1.0, false);
        },
        IpgTextParam::TextRgba => {
            let v = try_extract_vec_f32(value);
            let color_rgba = Some([v[0], v[1], v[2], v[3]]);
            txt.style = get_color(color_rgba, None, 1.0, false);
        },
        IpgTextParam::VtAlignTop => {
            txt.vertical_alignment = Vertical::Top;
        },
        IpgTextParam::VtAlignCenter => {
            txt.vertical_alignment = Vertical::Center;
        },
        IpgTextParam::VtAlignBottom => {
            txt.vertical_alignment = Vertical::Bottom;
        },
        IpgTextParam::Width => {
            let val = try_extract_f64(value);
            txt.width = get_width(Some(val as f32), false);
        },
        IpgTextParam::WidthFill => {
            let val = try_extract_boolean(value);
            txt.width = get_width(None, val);
        },
    }
}


fn try_extract_text_update(update_obj: PyObject) -> IpgTextParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgTextParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Text update extraction failed"),
        }
    })
}
