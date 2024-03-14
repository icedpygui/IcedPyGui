use iced::{Length, Element};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::text::{LineHeight, Shaping};
use iced::widget::{Space, Text};
use crate::app::Message;

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
