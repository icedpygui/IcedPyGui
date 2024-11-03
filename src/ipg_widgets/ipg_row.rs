//! ipg_row
use iced::{Alignment, Padding, Length, Element};
use iced::widget::Row;

use crate::app::Message;

use super::ipg_enums::IpgAlignment;


#[derive(Debug, Clone)]
pub struct IpgRow {
    pub id: usize,
    pub show: bool,

    pub spacing: f32,
    pub padding: Padding,
    pub width: Length,
    pub height: Length,
    pub align_items: IpgAlignment,
    pub clip: bool,
}

impl IpgRow {
    pub fn new(
        id: usize,
        show: bool,
        spacing: f32,
        padding: Padding,
        width: Length,
        height: Length,
        align_items: IpgAlignment,
        clip: bool,
    ) -> Self {
        Self {
            id,
            show,
            spacing,
            padding,
            width,
            height,
            align_items,
            clip
        }
    }
}

pub fn construct_row(row: &IpgRow, content: Vec<Element<'static, Message>>) -> Element<'static, Message> {

    let align = get_alignment(row.align_items.clone());

    Row::with_children(content)
                        .align_y(align)
                        .height(row.height)
                        .padding(row.padding)
                        .spacing(row.spacing)
                        .width(row.width)
                        .clip(row.clip)
                        .into()
}


fn get_alignment(align: IpgAlignment) -> Alignment {

    match align {
        IpgAlignment::Start => Alignment::Start,
        IpgAlignment::Center => Alignment::Center,
        IpgAlignment::End => Alignment::End,
    }
}