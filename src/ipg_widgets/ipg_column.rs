
use iced::{Element, Length, Padding};
use iced::alignment::Alignment;
use iced::widget::Column;
use pyo3::pyclass;
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
    pub align_items: IpgColumnAlignment,
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
        align_items: IpgColumnAlignment,
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


#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgColumnAlignment {
    Start,
    Center,
    End,
}

pub fn construct_column(col: &IpgColumn, content: Vec<Element<'static, Message>> ) -> Element<'static, Message> {

    let align_items = get_alignment(col.align_items.clone());

    Column::with_children(content)
                        .align_items(align_items)
                        .height(col.height)
                        .padding(col.padding)
                        .spacing(col.spacing)
                        .width(col.width)
                        .into()
}


fn get_alignment(align: IpgColumnAlignment) -> Alignment {

    match align {
        IpgColumnAlignment::Start => Alignment::Start,
        IpgColumnAlignment::Center => Alignment::Center,
        IpgColumnAlignment::End => Alignment::End,
    }
}