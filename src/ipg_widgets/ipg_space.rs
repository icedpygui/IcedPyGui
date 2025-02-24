//! ipg_space
use iced::{Element, Length};
use iced::widget::Space;
use crate::app;


#[derive(Debug, Clone)]
pub struct IpgSpace {
    pub id: usize,
    pub width: Length,
    pub height: Length,
    pub show: bool,
}

impl IpgSpace {
    pub fn new(
        id: usize, 
        width: Length, 
        height: Length, 
        show: bool) -> Self {
        Self {
            id,
            width,
            height,
            show,
        }
    }
}


pub fn construct_space(sp: &IpgSpace) -> Option<Element<'static, app::Message>> {

    if sp.show {
        Some(Space::new(sp.width, sp.height).into())
    } else {
        None
    }
}
