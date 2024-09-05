//! ipg_column
use iced::{Element, Length, Padding};
use iced::widget::Column;

use crate::app::Message;

use super::helpers::get_alignment;
use super::ipg_enums::IpgAlignment;


#[derive(Debug, Clone)]
pub struct IpgColumn {
    pub id: usize,
    pub show: bool,
    pub spacing: f32,
    pub padding: Padding,
    pub width: Length,
    pub height: Length,
    pub max_width: f32,
    pub align_items: IpgAlignment,
    pub clip: bool,
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
            max_width,
            align_items,
            clip,
        }
    }
}


pub fn construct_column(col: &IpgColumn, content: Vec<Element<'static, Message>> ) -> Element<'static, Message> {

    let align_x = get_alignment(col.align_items.clone());

    Column::with_children(content)
                        .align_x(align_x)
                        .height(col.height)
                        .padding(col.padding)
                        .spacing(col.spacing)
                        .width(col.width)
                        .clip(col.clip)
                        .into()
}
