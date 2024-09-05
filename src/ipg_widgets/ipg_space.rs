//! ipg_space
use iced::{Element, Length};
use iced::widget::Space;
use crate::app;


#[derive(Debug, Clone)]
pub struct IpgSpace {
    pub id: usize,
    pub width: Length,
    pub height: Length,
}

impl IpgSpace {
    pub fn new(id: usize, width: Length, height: Length, ) -> Self {
        Self {
            id,
            width,
            height,
        }
    }
}


pub fn construct_space(sp: &IpgSpace) -> Element<'static, app::Message> {

    Space::new(sp.width, sp.height).into()

}
