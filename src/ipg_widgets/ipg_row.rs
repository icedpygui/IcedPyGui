//! ipg_row
use iced::{Alignment, Padding, Length, Element};
use iced::widget::Row;
use pyo3::{pyclass, PyObject, Python};

use crate::app::Message;

use super::helpers::{get_height, get_padding_f64, get_width, 
    try_extract_boolean, try_extract_f64, try_extract_ipg_alignment, 
    try_extract_vec_f64};
use super::ipg_enums::IpgAlignment;


#[derive(Debug, Clone)]
pub struct IpgRow {
    pub id: usize,
    pub show: bool,

    pub spacing: f32,
    pub padding: Padding,
    pub width: Length,
    pub height: Length,
    pub align: IpgAlignment,
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
        align: IpgAlignment,
        clip: bool,
    ) -> Self {
        Self {
            id,
            show,
            spacing,
            padding,
            width,
            height,
            align,
            clip
        }
    }
}

pub fn construct_row<'a>(row: &IpgRow, content: Vec<Element<'a, Message>>) -> Element<'a, Message> {

    let align = get_alignment(row.align.clone());

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

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgRowParam {
    Align,
    Clip,
    Padding,
    Width,
    WidthFill,
    Height,
    HeightFill,
    Spacing,
}

pub fn row_item_update(col: &mut IpgRow,
                            item: &PyObject,
                            value: &PyObject,
                            )
{
    let update = try_extract_row_update(item);
    let name = "Row".to_string();
    match update {
        IpgRowParam::Align => {
            col.align = try_extract_ipg_alignment(value).unwrap();
        },
        IpgRowParam::Clip => {
            col.clip = try_extract_boolean(value, name);
        },
        IpgRowParam::Padding => {
            col.padding =  get_padding_f64(try_extract_vec_f64(value, name));
        },
        IpgRowParam::Width => {
            let val = try_extract_f64(value, name);
            col.width = get_width(Some(val as f32), false);
        },
        IpgRowParam::WidthFill => {
            let val = try_extract_boolean(value, name);
            col.width = get_width(None, val);
        },
        IpgRowParam::Height => {
            let val = try_extract_f64(value, name);
            col.height = get_height(Some(val as f32), false);
        },
        IpgRowParam::HeightFill => {
            let val = try_extract_boolean(value, name);
            col.height = get_height(None, val);
        },
        IpgRowParam::Spacing => {
            col.spacing = try_extract_f64(value, name) as f32;
        },
    }
}

pub fn try_extract_row_update(update_obj: &PyObject) -> IpgRowParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgRowParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Row update extraction failed"),
        }
    })
}
