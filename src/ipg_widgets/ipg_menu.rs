#![allow(unused)]
use std::collections::HashMap;

use iced::widget::{button, row, text, Button, Container, Row, Text};
use iced::{alignment, Border, Color, Element, Length, Renderer, Theme};

use iced_aw::graphics::icons::{BootstrapIcon, BOOTSTRAP_FONT, BOOTSTRAP_FONT_BYTES};
use iced_aw::menu::{self, Item, Menu, MenuBar, StyleSheet};
use iced_aw::native::InnerBounds;
use iced_aw::style::MenuBarStyle;
use iced_aw::{menu_bar, menu_items, quad};
use pyo3::types::PyDict;
use pyo3::{pyclass, PyObject, Python};

use crate::{access_callbacks, app};

use super::callbacks::{get_set_widget_callback_data, WidgetCallbackIn, WidgetCallbackOut};


#[derive(Debug, Clone)]
pub struct IpgMenu {
    pub id: usize,
    pub labels: Vec<String>,
    pub items: PyObject,
    pub widths: Vec<f32>,
    pub separators: Option<Vec<(usize, usize, IpgMenuSepTypes)>>,
    pub sep_types: Option<Vec<IpgMenuSepTypes>>,
    pub sep_label_names: Option<Vec<String>>,
    pub user_data: Option<PyObject>,
}

impl IpgMenu {
    pub fn new(
        id: usize,
        labels: Vec<String>,
        items: PyObject,
        widths: Vec<f32>,
        separators: Option<Vec<(usize, usize, IpgMenuSepTypes)>>,
        sep_types: Option<Vec<IpgMenuSepTypes>>,
        sep_label_names: Option<Vec<String>>,
        user_data: Option<PyObject>,
    ) -> Self {
        Self {
            id,
            labels,
            items,
            widths,
            separators,
            sep_types,
            sep_label_names,
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

    if items.len() != mn.labels.len() { panic!("Menu: Labels and the Menu dictionary must be of the same width") }
    if items.len() != mn.widths.len() { panic!("Menu: Widths and the Menu dictionary must be of the same width") }

    let mut menu_bar_items = vec![];

    for (bar_index, label) in mn.labels.iter().enumerate() {
        menu_bar_items = vec![];
        let item_labels = items.get(label);
        let list = match item_labels {
            Some(list) => list,
            None => panic!("Menu label does not match items dictionary key")
        };
        for (menu_index, item) in list.iter() .enumerate(){

            menu_bar_items.push(Item::new(menu_button(item.clone())));

            if mn.separators.is_some() {
                match &mn.separators {
                    Some(separators) => {
                        let separator = get_separator(bar_index, 
                                                            menu_index, 
                                                            separators,  
                                                            &mn.sep_label_names);
                        match separator {
                            Some(sep) => menu_bar_items.push(sep),
                            None => (),
                        }
                    },
                    None => (),
                }
            }
        }

        menu_bar.push(Item::with_menu(
                        menu_bar_button(label.clone(), mn.widths[bar_index]),
                        Menu::new(menu_bar_items)
                                        .width(Length::Fixed(mn.widths[bar_index]))
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

fn try_extract_separator_types(s_types: PyObject) -> Vec<IpgMenuSepTypes> {
    Python::with_gil(|py| {

        let res = s_types.extract::<Vec<IpgMenuSepTypes>>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("Unable to extract IpgMenuSepTypes"),
        }
    })
}

fn menu_button(label: String) -> Element<'static, MenuMessage> {
    
    let label_txt: Element<MenuMessage> = Text::new(label.clone())
                                                    .horizontal_alignment(alignment::Horizontal::Center)
                                                    .width(Length::Fill)
                                                    .into();

    let btn: Element<MenuMessage> = Button::new(label_txt)
                                    .on_press(MenuMessage::ItemPress(label))
                                    .width(Length::Fill)
                                    .style(iced::theme::Button::Custom(Box::new(ButtonStyle {})))
                                    .into();
    btn
}

fn menu_bar_button(label: String, width: f32) -> Element<'static, MenuMessage> {
    
    let label_txt: Element<MenuMessage> = Text::new(label.clone())
                                            .vertical_alignment(alignment::Vertical::Center)
                                            .horizontal_alignment(alignment::Horizontal::Center)
                                            .width(Length::Fill)
                                            .into();

    let btn: Element<MenuMessage> = Button::new(label_txt)
                                    .on_press(MenuMessage::ItemPress(label))
                                    .width(Length::Fixed(width))
                                    .style(iced::theme::Button::Custom(Box::new(ButtonStyle {})))
                                    .into();
    btn
}


fn get_separator(bar_index: usize, 
                menu_index: usize, 
                separators: &Vec<(usize, usize, IpgMenuSepTypes)>, 
                sep_label_names: &Option<Vec<String>>) -> Option<Item<'static, MenuMessage, Theme, Renderer>> {

    // Check to see if a label type is present then check that the
    // sep_lable_names is not None.
    for st in separators {
        match st.2 {
            IpgMenuSepTypes::Line => (),
            IpgMenuSepTypes::Dot => (),
            IpgMenuSepTypes::Label => {
                if sep_label_names.is_none() {
                    panic!("Menu:  Since you are using IpgMenuSepTypes::Label, them you must supply a sep_label_names item in a list")
                }
            },
        }
    }

    let sln = match sep_label_names {
        Some(sln) => sln,
        None => panic!("Menu: Unable to match sep_label_names"),
    };
    // This keeps track of the label index since there is not a requirement to
    // match the size of the list for the labels.  So long as the label type match the
    // munber of labels it is OK.  A check is put in to check this before the index is used.
    let mut sln_index = 0;

    for (b_idx, m_idx, s_type) in separators.iter() {

        if b_idx == &bar_index && m_idx == &menu_index {
            match *s_type {
                IpgMenuSepTypes::Line => return Item::new(line_separator()).into(),
                IpgMenuSepTypes::Dot => return Item::new(
                                        dot_separator(Theme::Dark)).into(),
                IpgMenuSepTypes::Label => {
                    if sln_index > sln.len() { panic!("Menu: The number of label type exceeds the number of labels.")}
                   
                    let item = Item::new(
                                                    labeled_separator(sln[sln_index].clone()));
                    sln_index += 1;
                    return item.into()
                },
            }
        }
    }
    None
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


#[derive(Debug, Clone, Copy)]
#[pyclass]
pub enum IpgMenuSepTypes {
    Line,
    Dot,
    Label,
}


fn line_separator() -> quad::Quad {
    quad::Quad {
        quad_color: Color::from([0.5; 3]).into(),
        quad_border: Border {
            radius: [4.0; 4].into(),
            ..Default::default()
        },
        inner_bounds: InnerBounds::Ratio(0.98, 0.2),
        height: Length::Fixed(15.0),
        ..Default::default()
    }
}

fn dot_separator(theme: iced::Theme) -> Element<'static, MenuMessage, iced::Theme, iced::Renderer> {
    row((0..20).map(|_| {
        quad::Quad {
            quad_color: theme.extended_palette().background.base.text.into(),
            inner_bounds: InnerBounds::Square(4.0),
            ..line_separator()
        }
        .into()
    }))
    .height(15.0)
    .into()
}

fn labeled_separator(label: String) -> Element<'static, MenuMessage, iced::Theme, iced::Renderer> {
    
    let q_1: Element<MenuMessage> = quad::Quad {
        width: Length::Fixed(20.0),
        ..line_separator()
    }.into();
    let q_2: Element<MenuMessage> = quad::Quad {
        width: Length::Fixed(20.0),
        ..line_separator()
    }.into();


    Row::with_children(vec![
                            q_1, 
                            Text::new(label).into(),
                            q_2,
                            ])
                            .into()
    
}

fn circle(color: Color) -> quad::Quad {
    let radius = 10.0;

    quad::Quad {
        quad_color: color.into(),
        inner_bounds: InnerBounds::Square(radius * 2.0),
        quad_border: Border {
            radius: [radius; 4].into(),
            ..Default::default()
        },
        height: Length::Fixed(20.0),
        ..Default::default()
    }
}
