use iced::{Alignment, Padding, Length, Element};
use iced::widget::Row;
use pyo3::pyclass;
use crate::app::Message;


#[derive(Debug, Clone)]
pub struct IpgRow {
    pub id: usize,
    pub show: bool,

    pub spacing: f32,
    pub padding: Padding,
    pub width: Length,
    pub height: Length,
    pub align_items: IpgRowAlignment,
}

impl IpgRow {
    pub fn new(
        id: usize,
        show: bool,
        spacing: f32,
        padding: Padding,
        width: Length,
        height: Length,
        align_items: IpgRowAlignment,
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

    let align = get_alignment(row.align_items.clone());

    Row::with_children(content)
                        .align_items(align)
                        .height(row.height)
                        .padding(row.padding)
                        .spacing(row.spacing)
                        .width(row.width)
                        .into()
}


#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgRowAlignment {
    Start,
    Center,
    End,
}


fn get_alignment(align: IpgRowAlignment) -> Alignment {

    match align {
        IpgRowAlignment::Start => Alignment::Start,
        IpgRowAlignment::Center => Alignment::Center,
        IpgRowAlignment::End => Alignment::End,
    }
}