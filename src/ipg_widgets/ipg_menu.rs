#![allow(unused)]
use std::collections::HashMap;

use iced::widget::{Button, button, Container, Text, text};
use iced::{alignment, Border, Color, Element, Length};

use iced_aw::graphics::icons::{BootstrapIcon, BOOTSTRAP_FONT, BOOTSTRAP_FONT_BYTES};
use iced_aw::menu::{self, Item, Menu, MenuBar, StyleSheet};
use iced_aw::style::MenuBarStyle;
use iced_aw::{menu_bar, menu_items};
use pyo3::types::PyDict;
use pyo3::{PyObject, Python};

use crate::{access_callbacks, app};

use super::callbacks::{get_set_widget_callback_data, WidgetCallbackIn, WidgetCallbackOut};


#[derive(Debug, Clone)]
pub struct IpgMenu {
    pub id: usize,
    pub labels: Vec<String>,
    pub items: PyObject,
    pub user_data: Option<PyObject>,
}

impl IpgMenu {
    pub fn new(
        id: usize,
        labels: Vec<String>,
        items: PyObject,
        user_data: Option<PyObject>,
    ) -> Self {
        Self {
            id,
            labels,
            items,
            user_data, 
        }
    }    
}


#[derive(Debug, Clone)]
pub enum MenuMessage {
    ItemPress(String),
}

pub fn construct_menu(mn: IpgMenu) -> Element<'static, app::Message> {

    let items = try_extract_dict(mn.items);

    // let menu_layer_1 = |items| Menu::new(items).max_width(180.0).offset(15.0).spacing(5.0);
    let mut menu_bar = vec![];

    let mut menu_bar_items = vec![];

    for label in mn.labels {
        menu_bar_items = vec![];
        let item_labels = items.get(&label);
        let list = match item_labels {
            Some(list) => list,
            None => panic!("Menu label does not match items dictionary key")
        };
        for item in list {
            menu_bar_items.push(Item::new(menu_button(item.clone())));
        }
        menu_bar.push(Item::with_menu(
                        menu_bar_button(label.clone()),
                        Menu::new(menu_bar_items)
                                        .width(Length::Fixed(100.0))
                                        .spacing(5.0) 
                        ));
    }


    let mb = MenuBar::new(
        menu_bar
    )
    .draw_path(menu::DrawPath::Backdrop)
    .style(|theme:&iced::Theme| menu::Appearance{
        path_border: Border{
            radius: [6.0; 4].into(),
            ..Default::default()
        },
        ..theme.appearance(&MenuBarStyle::Default)
    });

    let ipg_menu: Element<MenuMessage> = Container::new(mb).into();

    ipg_menu.map(move |message| app::Message::Menu(mn.id, message))

}

pub fn menu_callback(id: usize, message: MenuMessage) {
    let mut wci = WidgetCallbackIn::default();
    wci.id = id;
    
    match message {
        MenuMessage::ItemPress(item) => {
            let mut wco: WidgetCallbackOut = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.value_str = Some(item);
            wco.event_name = "on_select".to_string();
            process_callback(wco);
        }
    }
}

pub fn process_callback(wco: WidgetCallbackOut) 
{
    let app_cbs = access_callbacks();

    let callback_present = app_cbs.callbacks.get(&(wco.id, wco.event_name.clone()));

    let callback_opt = match callback_present {
        Some(cb) => cb,
        None => return,
    };

    let callback = match callback_opt {
        Some(cb) => cb,
        None => panic!("Menu callback could not be found with id {}", wco.id),
    };

    let item_selected = match wco.value_str {
        Some(item) => item,
        None => panic!("Menu item selected could not be processed")
    };

    Python::with_gil(|py| {
            if wco.user_data.is_some() {
                let user_data = match wco.user_data {
                    Some(ud) => ud,
                    None => panic!("User Data could not be found in Menu callback"),
                };
                let res = callback.call1(py, (
                                                                    wco.id.clone(),
                                                                    item_selected,  
                                                                    user_data
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(_) => panic!("Menu: 2 parameters (id, user_data) are required or possibly a non-fatal python error in this function."),
                }
            } else {
                let res = callback.call1(py, (
                                                                    wco.id.clone(),
                                                                    item_selected  
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(_) => panic!("Menu: 1 parameter (id) is required or possibly a non-fatal python error in this function."),
                }
            } 
    });
    
    drop(app_cbs);
         
}


fn try_extract_dict(items: PyObject) -> HashMap<String, Vec<String>> {
    Python::with_gil(|py| {

        let res = items.extract::<HashMap<String, Vec<String>>>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("Unable to extract python dict"),
        }
    })

}

fn menu_button(label: String) -> Element<'static, MenuMessage> {
    
    let label_txt: Element<MenuMessage> = Text::new(label.clone()).into();

    let btn: Element<MenuMessage> = Button::new(label_txt)
                                    .on_press(MenuMessage::ItemPress(label))
                                    .width(Length::Fill)
                                    .style(iced::theme::Button::Custom(Box::new(ButtonStyle {})))
                                    .into();
    btn
}

fn menu_bar_button(label: String) -> Element<'static, MenuMessage> {
    
    let label_txt: Element<MenuMessage> = Text::new(label.clone())
                                            .vertical_alignment(alignment::Vertical::Center)
                                            .into();

    let btn: Element<MenuMessage> = Button::new(label_txt)
                                    .on_press(MenuMessage::ItemPress(label))
                                    .width(Length::Shrink)
                                    .style(iced::theme::Button::Custom(Box::new(ButtonStyle {})))
                                    .into();
    btn
}


struct ButtonStyle;
impl button::StyleSheet for ButtonStyle {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            text_color: style.extended_palette().background.base.text,
            background: Some(Color::TRANSPARENT.into()),
            // background: Some(Color::from([1.0; 3]).into()),
            border: Border {
                radius: [6.0; 4].into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let plt = style.extended_palette();

        button::Appearance {
            background: Some(plt.primary.weak.color.into()),
            text_color: plt.primary.weak.text,
            ..self.active(style)
        }
    }
}

