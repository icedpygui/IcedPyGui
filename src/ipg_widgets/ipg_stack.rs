//! ipg_stack
use iced::{Element, Length};
use iced::widget::Stack;
use pyo3::{pyclass, PyObject, Python};

use crate::app::Message;

use super::helpers::{get_height, get_width, try_extract_boolean, try_extract_f32};

#[derive(Debug, Clone)]
pub struct IpgStack {
    pub id: usize,
    pub width: Length,
    pub height: Length,
    pub hide_index: Option<usize>,
    pub show: bool,
}

impl IpgStack {
    pub fn new(
        id: usize,
        width: Length,
        height: Length,
        hide_index: Option<usize>,
        show: bool,
    ) -> Self {
        Self {
            id,
            width,
            height,
            hide_index,
            show,
        }
    }
}


pub fn construct_stack(stk: IpgStack, mut content: Vec<Element<Message>> ) 
                        -> Element<Message> {
    
    content = if stk.hide_index.is_some() {
        let index = stk.hide_index.unwrap();
        if index >= content.len() {
            panic!("Stack: The hide_index exceeds the number of stack containers.");
        }

        for i in (0..index).rev() {
            content.remove(i);
        }
        content
    } else {
        content
    };
    
    Stack::with_children(content)
                .width(stk.width)
                .height(stk.height)
                .into()

}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgStackParam {
    Width,
    WidthFill,
    Height,
    HeightFill,
    Show,
}

pub fn stack_item_update(stk: &mut IpgStack,
                            item: &PyObject,
                            value: &PyObject,) {

    let update = try_extract_stack_update(item);
    let name = "Stack".to_string();
    match update {
        IpgStackParam::Width => {
            let w = Some(try_extract_f32(value, name));
            stk.width = get_width(w, false)
        },
        IpgStackParam::WidthFill => {
            stk.width = get_width(None, try_extract_boolean(value, name));
        },
        IpgStackParam::Height => {
            let h = Some(try_extract_f32(value, name));
            stk.height = get_height(h, false)
        },
        IpgStackParam::HeightFill => {
            stk.height = get_height(None, try_extract_boolean(value, name));
        },
        IpgStackParam::Show => {
            stk.show = try_extract_boolean(value, name);
        },
    }
}

pub fn try_extract_stack_update(update_obj: &PyObject) -> IpgStackParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgStackParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Stack update extraction failed"),
        }
    })
}
