#![allow(unused)]
use iced::widget::{Container, Text, text};
use iced::{Color, Element};

// use iced_aw::{MenuTree, MenuBar};
use crate::app;


#[derive(Debug, Clone)]
pub struct IpgMenuBar {
    pub id: usize,
    pub items: Vec<String>,
    
}

impl IpgMenuBar {
    pub fn new(
        id: usize,
        items: Vec<String>,
    ) -> Self {
        Self {
            id,
            items, 
        }
    }    
}

#[derive(Debug, Clone)]
pub struct IpgMenuItem {
    pub id: usize,
    pub item: String,
}

impl IpgMenuItem {
    pub fn new(
        id: usize,
        item: String,
    ) -> Self {
        Self {
            id,
            item, 
        }
    }    
}

#[derive(Debug)]
enum Message {

}

pub fn construct_menu_bar(_mn: &IpgMenuBar) -> Element<'static, app::Message> {

    // let root_1: MenuTree<'_, Message, Renderer> = MenuTree::with_children(
    //     Button::new("Button").into(),
    //     vec![
    //         MenuTree::new(Button::new("Button").into()),
    //         MenuTree::new(Button::new("Button").into()),
    //         MenuTree::new(Button::new("Button").into()),
    //     ]
    // );

    // let menu_bar: Element<'_, Message, Renderer> = MenuBar::new(vec![root_1]);

    // let container: Element<'_, Message, Renderer<Theme>> = Container::new(menu_bar);

    text("Text").into()
}

pub fn menu_bar_update(_mn: MenuMessage) {
    
}

pub fn construct_menu_item(_mi: &IpgMenuItem) -> Element<'static, app::Message> {

    let txt: Element<'static, app::Message> = Text::new("text").into();
    let cnt: Element<'static, app::Message> = Container::new(txt).into();
    cnt
}

pub fn menu_item_update(_mn: MenuMessage) {
    
}

#[derive(Debug, Clone)]
pub enum MenuMessage {
    Debug(String),
    ValueChange(u8),
    CheckChange(bool),
    ToggleChange(bool),
    ColorChange(Color),
    FlipHorizontal,
    FlipVertical,
    ThemeChange(bool),
    TextChange(String),
    SizeOption(String),
}
