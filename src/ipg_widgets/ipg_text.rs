
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
pub enum IpgTextUpdate {
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
        IpgTextUpdate::Content => {
            txt.content = try_extract_string(value);
        },
        IpgTextUpdate::Height => {
            let val = try_extract_f64(value);
            txt.height = get_height(Some(val as f32), false); 
        },
        IpgTextUpdate::HeightFill => {
            let val = try_extract_boolean(value);
            txt.height = get_height(None, val);
        },
        IpgTextUpdate::HzAlignLeft => {
            txt.horizontal_alignment = Horizontal::Left;
        },
        IpgTextUpdate::HzAlignCenter => {
            txt.horizontal_alignment = Horizontal::Center;
        },
        IpgTextUpdate::HzAlignRight => {
            txt.horizontal_alignment = Horizontal::Right;
        },
        IpgTextUpdate::LineHeight => {
            let val = try_extract_f64(value) as f32;
            txt.line_height = LineHeight::Relative(val);
        },
        IpgTextUpdate::Show => {
            txt.show = try_extract_boolean(value);
        },
        IpgTextUpdate::Size => {
            txt.size = try_extract_f64(value) as f32;
        },
        IpgTextUpdate::VtAlignTop => {
            txt.vertical_alignment = Vertical::Top;
        },
        IpgTextUpdate::VtAlignCenter => {
            txt.vertical_alignment = Vertical::Center;
        },
        IpgTextUpdate::VtAlignBottom => {
            txt.vertical_alignment = Vertical::Bottom;
        },
        IpgTextUpdate::Width => {
            let val = try_extract_f64(value);
            txt.width = get_width(Some(val as f32), false);
        },
        IpgTextUpdate::WidthFill => {
            let val = try_extract_boolean(value);
            txt.width = get_width(None, val);
        },
    }
}


fn try_extract_text_update(update_obj: PyObject) -> IpgTextUpdate {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgTextUpdate>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Text update extraction failed"),
        }
    })
}
