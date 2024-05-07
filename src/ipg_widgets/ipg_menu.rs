#![allow(unused)]
use std::collections::{BTreeMap, HashMap};

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
use super::helpers::{convert_vecs, try_extract_dict, try_extract_vec_f32, try_extract_vec_f64, try_extract_vec_str};

#[derive(Debug, Clone)]
pub struct IpgMenu {
    pub id: usize,
    pub items: PyObject,
    pub widths: Vec<f32>,
    pub spacing: Vec<f32>,
    pub separators: Option<Vec<(usize, usize, IpgMenuSepTypes)>>,
    pub sep_types: Option<Vec<IpgMenuSepTypes>>,
    pub sep_label_names: Option<Vec<String>>,
    pub user_data: Option<PyObject>,
    menu_width: usize,
    new_menu: bool, 
    updating_separators: bool,
}

impl IpgMenu {
    pub fn new(
        id: usize,
        items: PyObject,
        widths: Vec<f32>,
        spacing: Vec<f32>,
        separators: Option<Vec<(usize, usize, IpgMenuSepTypes)>>,
        sep_types: Option<Vec<IpgMenuSepTypes>>,
        sep_label_names: Option<Vec<String>>,
        user_data: Option<PyObject>,
    ) -> Self {
        Self {
            id,
            items,
            widths,
            spacing,
            separators,
            sep_types,
            sep_label_names,
            user_data,
            menu_width: 0,
            new_menu: false,
            updating_separators: false,
        }
    }    
}


#[derive(Debug, Clone)]
pub enum MenuMessage {
    ItemPress(String),
}

pub fn construct_menu(mut mn: IpgMenu) -> Element<'static, app::Message> {

    let items = try_extract_dict(mn.items);

    let labels: Vec<String> = items.clone().into_keys().collect();

    mn.menu_width = labels.len();

    // default the spacing and widths if new menu.
    if mn.new_menu {
        mn.spacing = vec![5.0; mn.menu_width];
        mn.widths = vec![100.0; mn.menu_width];
        mn.new_menu = false;
    }
    
    // let menu_layer_1 = |items| Menu::new(items).max_width(180.0).offset(15.0).spacing(5.0);
    let mut menu_bar = vec![];

    // Since spacing and widths can have a vector of 1 value then after testing, they are expanded.
    if mn.spacing.len() != 1 && mn.menu_width != mn.spacing.len() {
        panic!("Menu: Spaces and the Menu dictionary must be of the same width")
    } else if mn.spacing.len() == 1 {
        mn.spacing = vec![mn.spacing[0]; mn.menu_width];
    }
    if mn.widths.len() != 1 && mn.menu_width != mn.widths.len() {
        panic!("Menu: Widths and the Menu dictionary must be of the same width")
    } else if mn.widths.len() == 1 {
        mn.widths = vec![mn.widths[0]; mn.menu_width];
    }

    let mut menu_bar_items = vec![];

    for (bar_index, label) in labels.iter().enumerate() {
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
                                        .spacing(mn.spacing[bar_index]) 
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
                    Err(er) => panic!("Menu: 2 parameters (id, user_data) are required or a python error in this function. {er}"),
                }
            } else {
                let res = callback.call1(py, (
                                                                    wco.id.clone(),
                                                                    item_selected  
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Menu: 1 parameter (id) is required or a python error in this function. {er}"),
                }
            } 
    });
    
    drop(app_cbs);
         
}


fn try_extract_separators(seps: PyObject) -> Vec<(usize, usize, IpgMenuSepTypes)> {
    Python::with_gil(|py| {

        let res = seps.extract::<Vec<(usize, usize, IpgMenuSepTypes)>>(py);
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
                sep_label_names: &Option<Vec<String>>) 
                -> Option<Item<'static, MenuMessage, Theme, Renderer>> {

    // false if all label parameters don't match
    let (checked, message) = check_label_separators(separators, sep_label_names);
    if !checked {
        panic!("{}", message)
    }

    let sln = match sep_label_names.clone() {
        Some(labels) => labels,
        None => vec![], // since a check was done, this won't be used
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
                IpgMenuSepTypes::Delete => (),
            }
        }
    }
    None
}


fn check_label_separators(separators: &Vec<(usize, usize, IpgMenuSepTypes)>, 
                            sep_label_names: &Option<Vec<String>>) -> (bool, String) {

    // Check to see if a label type is present then check that the
    // sep_label_names is not None or equals count.
    let message = "".to_string();
    let mut checked = (true, message);
    let mut label_count = 0;
    for st in separators {
        match st.2 {
            IpgMenuSepTypes::Line => (),
            IpgMenuSepTypes::Dot => (),
            IpgMenuSepTypes::Label => {
                label_count += 1;
                if sep_label_names.is_none() {
                    checked = (false, "Menu:  Since you are using IpgMenuSepTypes::Label, them you must supply a sep_label_names item in a list".to_string())
                }
            },
            IpgMenuSepTypes::Delete => (),
        }
    }

    match sep_label_names {
        Some(sln) => {
            if sln.len() != label_count { 
                checked =(false, "Menu: Separations label count must equal IpgMenuSepTypes::Label count".to_string());
            }
        },
        None => {
                if label_count != 0 {
                    checked = (false, "Menu: Unable to match sep_label_names".to_string());
                } else {
                    ()
                }
            }
    }

    checked
    
}

#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgMenuParams {
    MenuUpdate,
    Separators,
    Spacing,
    Widths,
}

pub fn menu_item_update(mn: &mut IpgMenu,
                            item: PyObject,
                            value: PyObject,
                            )
{
    let update = try_extract_menu_update(item);

    match update {
        IpgMenuParams::MenuUpdate => {
            mn.items = value;
            // Need to default the following items after a new menu is added
            mn.separators = None;
            mn.sep_label_names = None;
            mn.sep_types = None;

            mn.new_menu = true;
        },
        IpgMenuParams::Separators => {
            let extracted_seps = try_extract_separators(value);
            // If never set, equate and return
            let mut menu_seps = match mn.separators.clone() {
                Some(seps) => seps,
                None => {
                    mn.separators = Some(extracted_seps);
                    mn.updating_separators = false;
                    return
                }
            };
            mn.separators = Some(delete_insert_separators(&extracted_seps, menu_seps.clone()));
        }
        IpgMenuParams::Spacing => {
            mn.spacing = try_extract_vec_f32(value);
        },
        IpgMenuParams::Widths => {
            mn.widths = try_extract_vec_f32(value);
        },
    }
}


fn delete_insert_separators(extracted_seps: &Vec<(usize, usize, IpgMenuSepTypes)>, 
                            mut menu_seps: Vec<(usize, usize, IpgMenuSepTypes)>) 
                            -> Vec<(usize, usize, IpgMenuSepTypes)> {

    // Since the seps can be in any order, need to iterate through 
    // figure out what needs to be done and based on actions perform
    // the actions based on the index and finally delete any.
    // action 0=delete, 1=insert, 2=replace
    // actions = [(mn_idx, es_idx, action)]
    let mut actions: Vec<(usize, usize, usize)> = vec![];
    let mut deleted = false;
    let mut replaced = false;
    for (es_idx, (es_br_idx, es_it_idx, es_tp)) in extracted_seps.iter().enumerate() {
        for (mn_idx, (mn_br_idx, mn_it_idx, mn_tp)) in menu_seps.iter().enumerate() {
            if mn_br_idx == es_br_idx && mn_it_idx == es_it_idx {
                match es_tp {
                    IpgMenuSepTypes::Line => {
                        actions.push((mn_idx, es_idx, 2));
                        replaced = true;
                    },
                    IpgMenuSepTypes::Dot => {
                        actions.push((mn_idx, es_idx, 2));
                        replaced = true;
                    },
                    IpgMenuSepTypes::Label => {
                        actions.push((mn_idx, es_idx, 2));
                        replaced = true;
                    },
                    IpgMenuSepTypes::Delete => {
                        deleted = true;
                        actions.push((mn_idx, es_idx, 0));
                    },
                }
            } 
        }
        if !replaced && !deleted {
            // 0 arbitray number because not used for inserts
            actions.push((0, es_idx, 1));
        }
        replaced = false;
        deleted = false;
    }
    // sort the deletes so that the end ones are deleted first
    // thereby not changing the index of the earlier ones.
    // Then go in reverse order to select the end ones first.
    let mut deletes: Vec<usize> = vec![];

    for (mn_idx, es_idx, act) in actions {
        if act == 0 {
            deletes.push(mn_idx);
        }
        if act == 1 {
            menu_seps.push(extracted_seps[es_idx]);
        }
        if act == 2 {
            menu_seps[mn_idx] = extracted_seps[es_idx];
        }
    }

    if deletes.len() > 0 {
        for idx in deletes.iter().rev() {
            menu_seps.remove(*idx);
        }
    }

    menu_seps

}

pub fn try_extract_menu_update(update_obj: PyObject) -> IpgMenuParams {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgMenuParams>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Menu update extraction failed"),
        }
    })
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


#[derive(Debug, Clone, Copy, PartialEq)]
#[pyclass]
pub enum IpgMenuSepTypes {
    Line,
    Dot,
    Label,
    Delete,
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


#[test]
fn test_check_label_separators() {

    let menu_separators: Vec<(usize, usize, IpgMenuSepTypes)> =
                            vec![   (0, 0, IpgMenuSepTypes::Dot), 
                                    (1, 1, IpgMenuSepTypes::Line), 
                                    (2, 0, IpgMenuSepTypes::Label)
                                ];

    // testing 1 label and label vec with 1 label, expect true
    let sep_label_names: Option<Vec<String>> = Some(vec!["label".to_string()]);
    let result = check_label_separators(&menu_separators, &sep_label_names);

    assert_eq!(true, result.0);

    // testing 1 label and label vec with 0 label, expect false
    let sep_label_names: Option<Vec<String>> = None;
    let result = check_label_separators(&menu_separators, &sep_label_names);
    assert_eq!(false, result.0);

    // testing 1 label and label vec with 2 labels, expect false
    let sep_label_names: Option<Vec<String>> = Some(vec!["label".to_string(); 2]);
    let result = check_label_separators(&menu_separators, &sep_label_names);
    assert_eq!(false, result.0);

    let menu_separators: Vec<(usize, usize, IpgMenuSepTypes)> =
                                vec![   (0, 0, IpgMenuSepTypes::Dot), 
                                        (1, 1, IpgMenuSepTypes::Line), 
                                    ];
    // testing 0 label and label vec with 0 label, expect true
    let sep_label_names: Option<Vec<String>> = None;
    let result = check_label_separators(&menu_separators, &sep_label_names);
    assert_eq!(true, result.0);

    // testing 0 label and label vec with 1 label, expect false
    let sep_label_names: Option<Vec<String>> = Some(vec!["label".to_string()]);
    let result = check_label_separators(&menu_separators, &sep_label_names);
    assert_eq!(false, result.0);

}


#[test]
fn test_delete_insert_separators() {
    
    let menu_separators: Vec<(usize, usize, IpgMenuSepTypes)> =
                            vec![   (0, 0, IpgMenuSepTypes::Dot), 
                                    (1, 1, IpgMenuSepTypes::Line), 
                                    (2, 0, IpgMenuSepTypes::Label)
                                ];
    
    let extracted_separators: Vec<(usize, usize, IpgMenuSepTypes)> = 
                                vec![(0, 0, IpgMenuSepTypes::Line)];
    
    let menu_seps = delete_insert_separators(&extracted_separators, 
                                                                                menu_separators.clone());

    // Test replace
    assert_eq!(vec![(0, 0, IpgMenuSepTypes::Line), 
                    (1, 1, IpgMenuSepTypes::Line), 
                    (2, 0, IpgMenuSepTypes::Label)], menu_seps);
                                          
    let extracted_separators: Vec<(usize, usize, IpgMenuSepTypes)> = 
                                vec![(1, 1, IpgMenuSepTypes::Delete)];
    
    let menu_seps = delete_insert_separators(&extracted_separators, 
                                                                                menu_separators.clone());

    // Test delete
    assert_eq!(vec![(0, 0, IpgMenuSepTypes::Dot), 
                    (2, 0, IpgMenuSepTypes::Label)], menu_seps);

    let extracted_separators: Vec<(usize, usize, IpgMenuSepTypes)> = 
    vec![(1, 0, IpgMenuSepTypes::Dot)];

    let menu_seps = delete_insert_separators(&extracted_separators, 
                                                                        menu_separators);

    // Test insert
    assert_eq!(vec![(0, 0, IpgMenuSepTypes::Dot), 
                    (1, 1, IpgMenuSepTypes::Line), 
                    (2, 0, IpgMenuSepTypes::Label),
                    (1, 0, IpgMenuSepTypes::Dot)], menu_seps);

}
