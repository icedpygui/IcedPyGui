//! ipg_column
use iced::{Element, Length, Padding};
use iced::widget::Column;
use pyo3::{pyclass, PyObject, Python};

use crate::app::Message;

use super::helpers::{get_alignment, get_height, get_padding_f64, get_width, try_extract_boolean, try_extract_f64, try_extract_ipg_alignment, try_extract_vec_f64};
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
    pub align: IpgAlignment,
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
            max_width,
            align,
            clip,
        }
    }
}

#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgColumnParam {
    Align,
    Clip,
    Padding,
    Width,
    WidthFill,
    Height,
    HeightFill,
    Spacing,
}

pub fn construct_column<'a>(col: &IpgColumn, content: Vec<Element<'a, Message>> ) -> Element<'a, Message> {

    let align_x = get_alignment(col.align.clone());

    Column::with_children(content)
                        .align_x(align_x)
                        .height(col.height)
                        .padding(col.padding)
                        .spacing(col.spacing)
                        .width(col.width)
                        .clip(col.clip)
                        .into()
}


pub fn column_item_update(col: &mut IpgColumn,
                            item: &PyObject,
                            value: &PyObject,
                            )
{
    let update = try_extract_column_update(item);
    let name = "Column".to_string();
    match update {
        IpgColumnParam::Align => {
            col.align = try_extract_ipg_alignment(value).unwrap();
        },
        IpgColumnParam::Clip => {
            col.clip = try_extract_boolean(value, name);
        },
        IpgColumnParam::Padding => {
            col.padding =  get_padding_f64(try_extract_vec_f64(value, name));
        },
        IpgColumnParam::Width => {
            let val = try_extract_f64(value, name);
            col.width = get_width(Some(val as f32), false);
        },
        IpgColumnParam::WidthFill => {
            let val = try_extract_boolean(value, name);
            col.width = get_width(None, val);
        },
        IpgColumnParam::Height => {
            let val = try_extract_f64(value, name);
            col.height = get_height(Some(val as f32), false);
        },
        IpgColumnParam::HeightFill => {
            let val = try_extract_boolean(value, name);
            col.height = get_height(None, val);
        },
        IpgColumnParam::Spacing => {
            col.spacing = try_extract_f64(value, name) as f32;
        },
    }
}

pub fn try_extract_column_update(update_obj: &PyObject) -> IpgColumnParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgColumnParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Column update extraction failed"),
        }
    })
}
