
use iced::{Length, Element};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::text::{LineHeight, Shaping};
use iced::widget::{Space, Text};
use crate::app::Message;

use pyo3::{pyclass, PyObject, Python};

use super::helpers::{get_height, get_width, try_extract_boolean, try_extract_f64, try_extract_string};

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
    // pub style: Style,
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
        // style: Style,
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
            // style: Style,
        }
    }
}

pub fn construct_text(text: &IpgText) -> Element<'static, Message> {

    if !text.show {
        return Space::new(Length::Shrink, Length::Shrink).into()
    }

    Text::new(text.content.clone()
                        )
                        .size(text.size)
                        .line_height(text.line_height)
                        .width(text.width)
                        .height(text.height)
                        .horizontal_alignment(text.horizontal_alignment)
                        .vertical_alignment(text.vertical_alignment)
                        // .font()
                        .shaping(text.shaping)
                        // style: Style,
                        .into() 
}


#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgTextParams {
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
    // Style,
    VtAlignTop,
    VtAlignCenter,
    VtAlignBottom,
    Width,
    WidthFill,
}

pub fn text_item_update(txt: &mut IpgText, item: PyObject, value: PyObject) {

    let update = try_extract_text_update(item);

    match update {
        IpgTextParams::Content => {
            txt.content = try_extract_string(value);
        },
        IpgTextParams::Height => {
            let val = try_extract_f64(value);
            txt.height = get_height(Some(val as f32), false); 
        },
        IpgTextParams::HeightFill => {
            let val = try_extract_boolean(value);
            txt.height = get_height(None, val);
        },
        IpgTextParams::HzAlignLeft => {
            txt.horizontal_alignment = Horizontal::Left;
        },
        IpgTextParams::HzAlignCenter => {
            txt.horizontal_alignment = Horizontal::Center;
        },
        IpgTextParams::HzAlignRight => {
            txt.horizontal_alignment = Horizontal::Right;
        },
        IpgTextParams::LineHeight => {
            let val = try_extract_f64(value) as f32;
            txt.line_height = LineHeight::Relative(val);
        },
        IpgTextParams::Show => {
            txt.show = try_extract_boolean(value);
        },
        IpgTextParams::Size => {
            txt.size = try_extract_f64(value) as f32;
        },
        IpgTextParams::VtAlignTop => {
            txt.vertical_alignment = Vertical::Top;
        },
        IpgTextParams::VtAlignCenter => {
            txt.vertical_alignment = Vertical::Center;
        },
        IpgTextParams::VtAlignBottom => {
            txt.vertical_alignment = Vertical::Bottom;
        },
        IpgTextParams::Width => {
            let val = try_extract_f64(value);
            txt.width = get_width(Some(val as f32), false);
        },
        IpgTextParams::WidthFill => {
            let val = try_extract_boolean(value);
            txt.width = get_width(None, val);
        },
    }
}


fn try_extract_text_update(update_obj: PyObject) -> IpgTextParams {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgTextParams>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Text update extraction failed"),
        }
    })
}
