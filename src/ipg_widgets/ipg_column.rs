use iced::{Element, Length, Padding};
use iced::alignment::Alignment;
use iced::widget::Column;
use crate::app::Message;
// use crate::iced_widgets::column::Column;

#[derive(Debug, Clone)]
pub struct IpgColumn {
    pub id: usize,
    pub show: bool,
    pub spacing: f32,
    pub padding: Padding,
    pub width: Length,
    pub height: Length,
    pub max_width: f32,
    pub align_items: Alignment,
}

impl IpgColumn {
    pub fn new(
        id: usize,
        show: bool,
        spacing: f32,
        padding: Padding,
        width: Length,
        height: Length,
        max_width: f32,
        align_items: Alignment,
    ) -> Self {
        Self {
            id,
            show,
            spacing,
            padding,
            width,
            height,
            max_width,
            align_items,
        }
    }
}

pub fn construct_column(col: &IpgColumn, content: Vec<Element<'static, Message>> ) -> Element<'static, Message> {

    Column::with_children(content)
                        // .id(col.id)
                        .align_items(col.align_items)
                        .height(col.height)
                        .padding(10)
                        .spacing(col.spacing)
                        .width(col.width)
                        .into()
}


