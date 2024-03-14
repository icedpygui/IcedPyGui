use iced::{Alignment, Padding, Length, Element};
use iced::widget::Row;
use crate::app::Message;



#[derive(Debug, Clone)]
pub struct IpgRow {
    pub id: usize,
    pub show: bool,

    pub spacing: f32,
    pub padding: Padding,
    pub width: Length,
    pub height: Length,
    pub align_items: Alignment,
}

impl IpgRow {
    pub fn new(
        id: usize,
        show: bool,
        spacing: f32,
        padding: Padding,
        width: Length,
        height: Length,
        align_items: Alignment,
    ) -> Self {
        Self {
            id,
            show,
            spacing,
            padding,
            width,
            height,
            align_items,
        }
    }
}

pub fn construct_row(row: &IpgRow, content: Vec<Element<'static, Message>>) -> Element<'static, Message> {

    Row::with_children(content)
                        .align_items(row.align_items)
                        .height(row.height)
                        .padding(row.padding)
                        .spacing(row.spacing)
                        .width(row.width)
                        .into()
}
