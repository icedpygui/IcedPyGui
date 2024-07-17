#![allow(unused)]

use std::collections::BTreeMap;

use iced::widget::text;
use iced::border::Radius;
use iced::theme::palette::Pair;
use iced::widget::button;
use iced::widget::{row, Button, Checkbox, Container, Row, Space, Text, Toggler};
use iced::{alignment, Background, Border, Color, Element, Length, Padding, Renderer, Shadow, Theme, Vector};

use crate::iced_aw_widgets::menu::menu_tree::{Item, Menu};
use crate::iced_aw_widgets::menu::menu_bar::MenuBar;
use crate::iced_aw_widgets::menu::common::DrawPath;
use crate::iced_aw_widgets::menu::quad::{InnerBounds, Quad};
use crate::iced_aw_widgets::menu::style_status::Status;
use crate::iced_aw_widgets::menu::menu_bar_style::{primary, Style};

use crate::graphics::colors::{get_color, match_ipg_color, IpgColor};
use crate::style::styling::{get_text_pair, is_dark, lighten, IpgColorPalette, IpgStyleStandard};
use pyo3::{pyclass, PyObject, Python};
use crate::{access_callbacks, access_state, app};
use super::callbacks::{get_set_widget_callback_data, WidgetCallbackIn, WidgetCallbackOut};
use super::helpers::{get_height, get_width, get_padding_f32, get_padding_f64, get_radius, 
    try_extract_boolean, try_extract_f64, try_extract_vec_f32, try_extract_vec_f64};
use super::ipg_button::{self, get_standard_style};
use super::{ipg_checkbox, ipg_toggle};


#[derive(Debug, Clone)]
pub struct IpgMenu {
    pub id: usize,
    pub items: PyObject, //ordered py dictionary
    pub item_widths: Vec<f32>,
    pub item_spacings: Vec<f32>,
    pub bar_spacing: f32,
    pub bar_padding: Padding,
    pub bar_width: Length,
    pub bar_height: Length,
    pub check_bounds_width: f32,
    pub menu_bar_style: Option<String>, // style_id of add_menu_bar_style()
    pub menu_style: Option<String>, // style_id of add_menu_style()
    // Option<String> in the below styles refer to the style_id of widget styles, not add_menu_style
    pub bar_style_all: Option<(Option<IpgStyleStandard>, Option<String>)>,
    pub button_item_style_all: Option<(Option<IpgStyleStandard>, Option<String>)>,
    pub checkbox_item_style_all: Option<(Option<IpgStyleStandard>, Option<String>)>,
    pub circle_item_style_all: Option<String>,
    pub dot_item_style_all: Option<String>,
    pub label_item_style_all: Option<String>,
    pub line_item_style_all: Option<String>,
    pub text_item_style_all: Option<(Option<IpgStyleStandard>, Option<String>)>,
    pub toggler_item_style_all: Option<(Option<IpgStyleStandard>, Option<String>)>,
    pub item_styles: Option<Vec<(usize, usize, Option<IpgStyleStandard>, Option<String>)>>,
    pub theme: Theme,
    pub show: bool,
    pub user_data: Option<PyObject>,
    menu_width: usize,
    new_menu: bool, 
    updating_separators: bool,
    pub is_checked: bool,
    pub is_toggled: bool,
}

impl IpgMenu {
    pub fn new(
        id: usize,
        items: PyObject,
        item_widths: Vec<f32>,
        item_spacings: Vec<f32>,
        bar_spacing: f32,
        bar_padding: Padding,
        bar_width: Length,
        bar_height: Length,
        check_bounds_width: f32,
        menu_bar_style: Option<String>,
        menu_style: Option<String>,
        bar_style_all: Option<(Option<IpgStyleStandard>, Option<String>)>,
        button_item_style_all: Option<(Option<IpgStyleStandard>, Option<String>)>,
        checkbox_item_style_all: Option<(Option<IpgStyleStandard>, Option<String>)>,
        circle_item_style_all: Option<String>,
        dot_item_style_all: Option<String>,
        label_item_style_all: Option<String>,
        line_item_style_all: Option<String>,
        text_item_style_all: Option<(Option<IpgStyleStandard>, Option<String>)>,
        toggler_item_style_all: Option<(Option<IpgStyleStandard>, Option<String>)>,
        item_styles: Option<Vec<(usize, usize, Option<IpgStyleStandard>, Option<String>)>>,
        theme: Theme,
        show: bool,
        user_data: Option<PyObject>,
    ) -> Self {
        Self {
            id,
            items,
            item_widths,
            item_spacings,
            bar_spacing,
            bar_padding,
            bar_width,
            bar_height,
            check_bounds_width,
            menu_bar_style,
            menu_style,
            bar_style_all,
            button_item_style_all,
            checkbox_item_style_all,
            circle_item_style_all,
            dot_item_style_all,
            label_item_style_all,
            line_item_style_all,
            text_item_style_all,
            toggler_item_style_all,
            item_styles,
            theme,
            show,
            user_data,
            menu_width: 0,
            new_menu: false,
            updating_separators: false,
            is_checked: false,
            is_toggled: false,
        }
    }    
}


#[derive(Debug, Clone, Default)]
pub struct IpgMenuBarStyle {
    pub id: usize,
    pub base: Option<Color>, // background
    pub border_color: Option<Color>,
    pub border_radius: Option<Vec<f32>>,
    pub border_width: Option<f32>,
    pub shadow_color: Option<Color>,
    pub shadow_offset_x: Option<f32>,
    pub shadow_offset_y: Option<f32>,
    pub shadow_blur_radius: Option<f32>,
}

impl IpgMenuBarStyle {
    pub fn new(
        id: usize,
        base: Option<Color>,
        border_color: Option<Color>,
        border_radius: Option<Vec<f32>>,
        border_width: Option<f32>,
        shadow_color: Option<Color>,
        shadow_offset_x: Option<f32>,
        shadow_offset_y: Option<f32>,
        shadow_blur_radius: Option<f32>,
    ) -> Self {
        Self {
            id,
            base,
            border_color,
            border_radius,
            border_width,
            shadow_color,
            shadow_offset_x,
            shadow_offset_y,
            shadow_blur_radius,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct IpgMenuStyle {
    pub id: usize,
    pub base: Option<Color>, // background
    pub border_color: Option<Color>,
    pub border_radius: Option<Vec<f32>>,
    pub border_width: Option<f32>,
    pub shadow_color: Option<Color>,
    pub shadow_offset_x: Option<f32>,
    pub shadow_offset_y: Option<f32>,
    pub shadow_blur_radius: Option<f32>,
}

impl IpgMenuStyle {
    pub fn new(
        id: usize,
        base: Option<Color>,
        border_color: Option<Color>,
        border_radius: Option<Vec<f32>>,
        border_width: Option<f32>,
        shadow_color: Option<Color>,
        shadow_offset_x: Option<f32>,
        shadow_offset_y: Option<f32>,
        shadow_blur_radius: Option<f32>,
    ) -> Self {
        Self {
            id,
            base,
            border_color,
            border_radius,
            border_width,
            shadow_color,
            shadow_offset_x,
            shadow_offset_y,
            shadow_blur_radius,
        }
    }
}


#[derive(Debug, Clone)]
pub enum MenuMessage {
    ItemPressed((usize, usize)),
    ItemCheckToggled(bool, (usize, usize)),
    ItemTogToggled(bool, (usize, usize)),
}


#[derive(Debug, Clone, PartialEq)]
#[pyclass]
pub enum IpgMenuType {
    Button,
    Checkbox,
    Circle,
    Dot,
    Label,
    Line,
    Text,
    Toggler,
}

pub fn construct_menu(mut mn: IpgMenu) -> Element<'static, app::Message, Theme, Renderer> {

    let menu = try_extract_dict(mn.items);
    
    // Labels are the keys of the dictionary items
    let labels: Vec<String> = menu.clone().into_keys().collect();

    mn.menu_width = labels.len();

    // default the spacing and widths if new menu.
    // Occurs during menu updating.
    if mn.new_menu {
        mn.item_spacings = vec![5.0; mn.menu_width];
        mn.item_widths = vec![100.0; mn.menu_width];
        mn.new_menu = false;
    }
    
    let mut menu_bar: Vec<Element<'static, MenuMessage, Theme, Renderer>> = vec![];

    // Checking if spacing and widths match and expanding vecs if len is 1.
    if mn.item_spacings.len() != 1 && mn.menu_width != mn.item_spacings.len() {
        panic!("Menu: Spaces and the Menu dictionary must be of the same width")
    } else if mn.item_spacings.len() == 1 {
        mn.item_spacings = vec![mn.item_spacings[0]; mn.menu_width];
    }
    if mn.item_widths.len() != 1 && mn.menu_width != mn.item_widths.len() {
        panic!("Menu: Widths and the Menu dictionary must be of the same width")
    } else if mn.item_widths.len() == 1 {
        mn.item_widths = vec![mn.item_widths[0]; mn.menu_width];
    }

    let mut bar_items: Vec<Item<MenuMessage, Theme, Renderer>> = vec![];

        for (bar_index, (bar_label, menu_items)) 
            in menu.iter().enumerate() {
            
            let mut items: Vec<Item<'static, MenuMessage, Theme, Renderer>> = vec![];

            let bar_style = mn.bar_style_all.clone();

            let widths = mn.item_widths.clone();

            for (item_index, (item_label, item_type)) 
                in menu_items.iter().enumerate() {

                items.push(get_menu_item(item_label.clone(),
                                        item_type.clone(),
                                        bar_index,
                                        item_index,
                                        mn.item_styles.clone(),
                                        mn.button_item_style_all.clone(),
                                        mn.checkbox_item_style_all.clone(),
                                        mn.circle_item_style_all.clone(),
                                        mn.dot_item_style_all.clone(),
                                        mn.label_item_style_all.clone(),
                                        mn.line_item_style_all.clone(),
                                        mn.text_item_style_all.clone(),
                                        mn.toggler_item_style_all.clone(),
                                        mn.is_checked,
                                        mn.is_toggled,
                                        mn.theme.clone(),
                                        ));
            }

            let menu_tpl = 
            |items| Menu::new(items)
                .max_width(100.0)
                .offset(0.0)
                .spacing(5.0);

            let bar_item = Item::with_menu(menu_bar_button(
                                                            bar_label.clone(),
                                                            widths[bar_index],
                                                            bar_index,
                                                            bar_style), 
                                                            menu_tpl(items)
                                                            .width(widths[bar_index]));

            bar_items.push(bar_item); 
        }

        let mb = MenuBar::new(bar_items)
                    .draw_path(DrawPath::Backdrop)
                    .style(move|theme:&iced::Theme, status: Status | 
                        get_mb_styling(theme, status, 
                            mn.menu_bar_style.clone(), 
                            mn.menu_style.clone()
                        )
                    )
                    .spacing(mn.bar_spacing)
                    .padding(mn.bar_padding)
                    .width(mn.bar_width)
                    .height(mn.bar_height)
                    .check_bounds_width(mn.check_bounds_width);

    let ipg_menu: Element<MenuMessage, Theme, Renderer> = Container::new(mb).into();

    ipg_menu.map(move |message| app::Message::Menu(mn.id, message))

}

fn get_mb_styling(theme: &Theme, 
                    status: Status,
                    bar_style_id: Option<String>,
                    menu_style_id: Option<String>,
                ) -> Style {

    let state = access_state();

    let mut menu_style = primary(theme, status);

    if bar_style_id.is_none() && menu_style_id.is_none() {
        return menu_style
    }

    let style = match state.menu_bar_style.get(&bar_style_id.clone().unwrap()){
        Some(st) => st,
        None => panic!("Bar Menu Style: Unable to find the style_id {}", bar_style_id.unwrap()),
    };

    
    let mut border = menu_style.path_border;

    if style.border_color.is_some() {
        border.color = style.border_color.unwrap();
    }


    primary(theme, status)

}



pub fn menu_callback(id: usize, message: MenuMessage) {
    let mut wci = WidgetCallbackIn::default();
    wci.id = id;
    
    match message {
        MenuMessage::ItemPressed((bar_index, menu_index)) => {
            let mut wco: WidgetCallbackOut = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.bar_index = Some(bar_index);
            wco.menu_index = Some(menu_index);
            wco.event_name = "on_select".to_string();
            process_callback(wco);
        }
        MenuMessage::ItemCheckToggled(is_checked, (bar_index, menu_index)) => {
            wci.is_checked = Some(is_checked);
            let mut wco: WidgetCallbackOut = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.bar_index = Some(bar_index);
            wco.menu_index = Some(menu_index);
            wco.event_name = "on_select".to_string();
            process_callback(wco);
        },
        MenuMessage::ItemTogToggled(togged, (bar_index, menu_index)) => {
            wci.on_toggle = Some(togged)
;            let mut wco: WidgetCallbackOut = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.bar_index = Some(bar_index);
            wco.menu_index = Some(menu_index);
            wco.event_name = "on_select".to_string();
            process_callback(wco);
        },
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

    let bar_index = wco.bar_index.unwrap();

    let menu_index_opt: Option<usize> = match wco.menu_index {
        Some(index) => {
                if index == 999 {
                    None
                } else {
                    Some(index)
                }
            },
        None => panic!("Menu: bar item could not be processed")
    };

    let toggled: Option<bool> = if wco.is_checked.is_some() {
        wco.is_checked
    } else if wco.on_toggle.is_some() {
        wco.on_toggle
    } else {None};

    Python::with_gil(|py| {
            if wco.user_data.is_some() {
                let user_data = match wco.user_data {
                    Some(ud) => ud,
                    None => panic!("User Data could not be found in Menu callback"),
                };
                let res = callback.call1(py, (
                                                                    wco.id.clone(),
                                                                    bar_index,
                                                                    menu_index_opt,
                                                                    toggled,  
                                                                    user_data
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Menu: 5 parameters (id, bar_index, menu_index, toggled, user_data) are required or a python error in this function. {er}"),
                }
            } else {
                let res = callback.call1(py, (
                                                                    wco.id.clone(),
                                                                    bar_index,
                                                                    menu_index_opt,
                                                                    toggled,  
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Menu: 4 parameter (id, bar_index, menu_index, toggled) is required or a python error in this function. {er}"),
                }
            } 
    });
    
    drop(app_cbs);
         
}

#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgMenuParam {
    BarHeight,
    BarHeightFill,
    BarPadding,
    BarSpacing,
    BarWidth,
    BarWidthFill,
    CheckBoundsWidth,
    Show,
}

pub fn menu_item_update(mn: &mut IpgMenu,
                            item: PyObject,
                            value: PyObject,
                            )
{
    let update = try_extract_menu_update(item);

    match update {
        IpgMenuParam::BarHeight => {
            let val = try_extract_f64(value) as f32;
            if val < 32.0 {
                mn.bar_height = get_height(None, false);
            } else {
                mn.bar_height = get_height(Some(val), false);
            }
            
        },
        IpgMenuParam::BarHeightFill => {
            let val = try_extract_boolean(value);
            mn.bar_height = get_height(None, val);
        },
        IpgMenuParam::BarPadding => {
            let val = try_extract_vec_f64(value);
            mn.bar_padding = get_padding_f64(val);
        },
        IpgMenuParam::BarSpacing => {
            mn.bar_spacing = try_extract_f64(value) as f32;
        },
        IpgMenuParam::BarWidth => {
            let val = try_extract_f64(value) as f32;
            mn.bar_width = get_width(Some(val),false);
        },
        IpgMenuParam::BarWidthFill => {
            let val = try_extract_boolean(value);
            mn.bar_width = get_width(None,val);
        },
        IpgMenuParam::CheckBoundsWidth => {
            mn.check_bounds_width = try_extract_f64(value) as f32;
        },
        IpgMenuParam::Show => {
            mn.show = try_extract_boolean(value);
        },
        
    }

}

pub fn try_extract_menu_update(update_obj: PyObject) -> IpgMenuParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgMenuParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Menu update extraction failed"),
        }
    })
}

fn get_menu_item(label: Option<String>,
                item_type: IpgMenuType,
                bar_index: usize,
                item_index: usize,
                item_styles: Option<Vec<(usize, usize, Option<IpgStyleStandard>, Option<String>)>>,
                button_item_style_all: Option<(Option<IpgStyleStandard>, Option<String>)>,
                checkbox_item_style_all: Option<(Option<IpgStyleStandard>, Option<String>)>,
                circle_item_style_all: Option<String>,
                dot_item_style_all: Option<String>,
                label_item_style_all: Option<String>,
                line_item_style_all: Option<String>,
                text_item_style_all: Option<(Option<IpgStyleStandard>, Option<String>)>,
                toggler_item_style_all: Option<(Option<IpgStyleStandard>, Option<String>)>,
                is_checked: bool,
                is_toggled: bool,
                theme: Theme,
                ) -> Item<'static, MenuMessage, Theme, Renderer> {
    

    let mut style: Option<String> = None;
    let mut style_standard: Option<IpgStyleStandard> = None;

    // Check if a certain style was used
    match item_styles {
        Some(s) => {
            for (b_index, itm_index, std_style, istyle) in s.iter() {
                if &bar_index == b_index && &item_index == itm_index {
                    style = istyle.clone();
                    style_standard = std_style.clone();
                    break;
                }
            }
        },
        None => ()
    }
        
    // get the style
    match item_type {
        IpgMenuType::Button => {
            if button_item_style_all.is_some(){
                (style_standard, style) = button_item_style_all.unwrap();
                // Need a default style if only using style
                if style_standard.is_none() && style.is_some() {
                    style_standard = Some(IpgStyleStandard::Primary);
                }
            }
        }
        IpgMenuType::Checkbox => {
            if checkbox_item_style_all.is_some(){
                (style_standard, style) = checkbox_item_style_all.unwrap();
                // Need a default style if only using style
                if style_standard.is_none() && style.is_some() {
                    style_standard = Some(IpgStyleStandard::Primary);
                }
            }
        },
        IpgMenuType::Circle => {
            if circle_item_style_all.is_some(){
                style = circle_item_style_all;  
            }
        },
        IpgMenuType::Dot => {
            if dot_item_style_all.is_some(){
                style = dot_item_style_all;
            }
        },
        IpgMenuType::Label => {
            if label_item_style_all.is_some(){
                style = label_item_style_all;
            }
        },
        IpgMenuType::Line => {
            if line_item_style_all.is_some(){
                style = line_item_style_all;
            }
        },
        IpgMenuType::Text => {
            if text_item_style_all.is_some(){
                (style_standard, style) = text_item_style_all.unwrap();
                // Need a default style if only using style
                if style_standard.is_none() && style.is_some() {
                    style_standard = Some(IpgStyleStandard::Text);
                }
            }
        }
        IpgMenuType::Toggler => {
            if toggler_item_style_all.is_some(){
                (style_standard, style) = toggler_item_style_all.unwrap();
                // Need a default style if only using style
                if style_standard.is_none() && style.is_some() {
                    style_standard = Some(IpgStyleStandard::Primary);
                }
            }
        },
    }
    
    return match_menu_item(item_type, 
                style, 
                style_standard, 
                bar_index, item_index, 
                is_checked, is_toggled, 
                label, theme)
            
}

fn match_menu_item(item_type: IpgMenuType,
                    style: Option<String>,
                    style_standard: Option<IpgStyleStandard>,
                    bar_index: usize, item_index: usize, 
                    is_checked: bool, is_toggled: bool,
                    label: Option<String>,
                    theme: Theme,
                    ) -> Item<'static, MenuMessage, Theme, Renderer> 
{
    let mut lbl = "".to_string();
    if label.is_some() && item_type!= IpgMenuType::Line {
        lbl = label.clone().unwrap();
    }

    match item_type {
        IpgMenuType::Button => {
            let label_txt: Element<MenuMessage, Theme, Renderer> = Text::new(lbl).into();

            let btn: Element<MenuMessage, Theme, Renderer> = 
                            Button::new(label_txt)
                                    .on_press(MenuMessage::ItemPressed((bar_index, item_index)))
                                    .width(Length::Fill)
                                    .style(move|theme: &Theme, status| {
                                        ipg_button::get_styling(theme, status, 
                                                                style.clone(),
                                                                style_standard.clone()) 
                                        })
                                    .into();
            Item::new(btn)
        },
        IpgMenuType::Checkbox => {
            let chkbx: Element<MenuMessage, Theme, Renderer> = 
                        Checkbox::new(lbl, 
                            is_checked)
                            .on_toggle(move|b| MenuMessage::ItemCheckToggled(b, (bar_index, item_index)))
                            .style(move|theme: &Theme, status| {
                                ipg_checkbox::get_styling(theme, status, 
                                                        style.clone(),
                                                        style_standard.clone()) 
                            })
                            .into();
            Item::new(chkbx)
        },
        IpgMenuType::Toggler => {
            let tog: Element<MenuMessage, Theme, Renderer> = 
                        Toggler::new(lbl, is_toggled, move|b| 
                            MenuMessage::ItemTogToggled(b, (bar_index, item_index)))
                            .style(move|theme: &Theme, status| {     
                                ipg_toggle::get_styling(theme, status, 
                                                        style.clone(),
                                                        style_standard.clone()) 
                            })
                                .into();
            Item::new(tog)
        },
        IpgMenuType::Line => Item::new(
            get_separator(&theme, style, IpgMenuSeparatorType::Line, None)
        ),
        IpgMenuType::Dot => Item::new(
            get_separator(&theme, style, IpgMenuSeparatorType::Dot, None)
        ),
        IpgMenuType::Label => Item::new(
            get_separator(&theme, style, IpgMenuSeparatorType::Label, label)
        ),
        IpgMenuType::Circle => Item::new(
            get_separator(&theme, style, IpgMenuSeparatorType::Circle, None)
        ),
        IpgMenuType::Text => {
            let label_txt: Element<MenuMessage, Theme, Renderer> = Text::new(lbl).into();

            let btn: Element<MenuMessage, Theme, Renderer> = 
                            Button::new(label_txt)
                                    .on_press(MenuMessage::ItemPressed((bar_index, item_index)))
                                    .width(Length::Fill)
                                    .style(move|theme: &Theme, status| {
                                        button::text(theme, status)
                                        })
                                    .into();
            Item::new(btn)
        },
                    
    }

}

fn menu_bar_button(label: String, 
                    width: f32, 
                    bar_index: usize, 
                    bar_button_style: Option<(Option<IpgStyleStandard>, Option<String>)>,
                ) -> Element<'static, MenuMessage, Theme, Renderer> {

    let mut style: Option<String> = None;
    let mut style_standard: Option<IpgStyleStandard> = None;

    if bar_button_style.is_some() {
        match bar_button_style {
            Some(bbs) => {
                style_standard = bbs.0;
                style = bbs.1;
            },
            None => (),
        }
    }

    if style_standard.is_none() && style.is_some() {
        style_standard = Some(IpgStyleStandard::Primary);
    }

    let label_txt: Element<'static, MenuMessage, Theme, Renderer> = 
                                Text::new(label.clone())
                                    .vertical_alignment(alignment::Vertical::Center)
                                    .horizontal_alignment(alignment::Horizontal::Center)
                                    .width(Length::Fill)
                                    .into();

    let btn: Element<'static, MenuMessage, Theme, Renderer> = 
                                Button::new(label_txt)
                                    .on_press(MenuMessage::ItemPressed((bar_index, 999)))
                                    .width(Length::Fixed(width))
                                    .style(move|theme: &Theme, status| {
                                        if style_standard.is_none() && style.is_none() {
                                            button::text(theme, status)
                                        } else {
                                            ipg_button::get_styling(theme, status, 
                                                style.clone(), style_standard.clone())
                                        }
                                        })
                                    .into();
    btn
}





#[derive(Debug, Clone)]
pub struct IpgMenuSeparatorStyle {
    pub id: usize,
    pub separator_type: IpgMenuSeparatorType,
    pub width: Length,
    pub height: Length,
    pub quad_ratios: Option<[f32; 2]>,
    pub separator_color: Option<Color>,
    pub separator_border_color: Option<Color>,
    pub separator_border_width: Option<f32>,
    pub separator_border_radius: Option<Vec<f32>>,
    pub separator_shadow_color: Option<Color>,
    pub separator_shadow_offset: Option<[f32; 2]>,
    pub separator_shadow_blur_radius: Option<f32>,
    pub background_color: Option<Color>,
    pub background_border_color: Option<Color>,
    pub background_border_width: Option<f32>,
    pub background_border_radius: Option<Vec<f32>>,
    pub background_shadow_color: Option<Color>,
    pub background_shadow_offset: Option<[f32; 2]>,
    pub background_shadow_blur_radius: Option<f32>,
}

impl IpgMenuSeparatorStyle {
    pub fn new(
        id: usize,
        separator_type: IpgMenuSeparatorType,
        width: Length,
        height: Length,
        quad_ratios: Option<[f32; 2]>,
        separator_color: Option<Color>,
        separator_border_color: Option<Color>,
        separator_border_width: Option<f32>,
        separator_border_radius: Option<Vec<f32>>,
        separator_shadow_color: Option<Color>,
        separator_shadow_offset: Option<[f32; 2]>,
        separator_shadow_blur_radius: Option<f32>,
        background_color: Option<Color>,
        background_border_color: Option<Color>,
        background_border_width: Option<f32>,
        background_border_radius: Option<Vec<f32>>,
        background_shadow_color: Option<Color>,
        background_shadow_offset: Option<[f32; 2]>,
        background_shadow_blur_radius: Option<f32>,

    ) -> Self {
        Self {
            id,
            separator_type,
            width,
            height,
            quad_ratios,
            separator_color,
            separator_border_color,
            separator_border_width,
            separator_border_radius,
            separator_shadow_color,
            separator_shadow_offset,
            separator_shadow_blur_radius,
            background_color,
            background_border_color,
            background_border_width,
            background_border_radius,
            background_shadow_color,
            background_shadow_offset,
            background_shadow_blur_radius,
        }
    }
}

#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgMenuSeparatorType {
    Circle,
    Dot,
    Label,
    Line,
}

fn separator() -> Quad {
    Quad {
        quad_color: Color::from([0.5; 3]).into(),
        quad_border: Border {
            radius: [4.0; 4].into(),
            ..Default::default()
        },
        inner_bounds: InnerBounds::Ratio(0.98, 0.2),
        height: Length::Fixed(20.0),
        ..Default::default()
    }
}

fn try_extract_dict(items: PyObject) -> BTreeMap<String, Vec<(Option<String>, IpgMenuType)>> {
    Python::with_gil(|py| {

        let res = items.extract::<BTreeMap<String, Vec<(Option<String>, IpgMenuType)>>>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("Menu: Unable to extract python menu dict"),
        }
    })
}

pub fn get_separator(theme: &Theme,
                    style_id: Option<String>,
                    sep_type: IpgMenuSeparatorType,
                    label: Option<String>, 
                    ) -> Element<'static, MenuMessage, iced::Theme, iced::Renderer> 
{

    let bg_color = theme.palette().background;

    let mut quad_color = if is_dark(bg_color) {
        Color::WHITE
    } else {
        Color::BLACK
    };

    if style_id.is_none() {
            return default_separator(sep_type.clone(), quad_color, bg_color, label).into()
    }

    let state = access_state();

    // if this far, style is some
    let style_id_str = style_id.unwrap();
    let sep_style = match state.menu_separator_style.get(&style_id_str.clone()){
        Some(st) => st,
        None => panic!("Menu Separator: Unable to find the style_id {}", style_id_str),
    };

    let mut quad = default_quad(sep_type.clone(), 
                                        quad_color, 
                                        bg_color, 
                                        sep_style.width, 
                                        sep_style.height);

    if sep_style.quad_ratios.is_some() {
       let ratios = sep_style.quad_ratios.unwrap(); 
       quad.inner_bounds = InnerBounds::Ratio(ratios[0], ratios[1]);    
    }

    if sep_style.separator_color.is_some() {
        quad.quad_color = Background::Color(sep_style.separator_color.unwrap());
    }

    if sep_style.separator_border_color.is_some() {
        quad.quad_border.color = sep_style.separator_border_color.unwrap()
    }

    if sep_style.separator_border_width.is_some() {
        quad.quad_border.width = sep_style.separator_border_width.unwrap();
    }

    if sep_style.separator_border_radius.is_some() {
        quad.quad_border.radius = get_radius(sep_style.separator_border_radius.clone().unwrap());
    }

    if sep_style.separator_shadow_color.is_some() {
        quad.quad_shadow.color = sep_style.separator_shadow_color.unwrap();
    }

    if sep_style.separator_shadow_offset.is_some() {
        let offset = sep_style.separator_shadow_offset.unwrap();
        quad.quad_shadow.offset = Vector{ x: offset[0], y: offset[1] };
    }
    
    if sep_style.separator_shadow_blur_radius.is_some() {
        quad.quad_shadow.blur_radius = sep_style.separator_shadow_blur_radius.unwrap();
    }

    if sep_style.background_border_color.is_some() {
        quad.bg_border.color = sep_style.background_border_color.unwrap();
    }

    if sep_style.background_border_width.is_some() {
        quad.bg_border.width = sep_style.background_border_width.unwrap();
    }

    if sep_style.background_border_radius.is_some() {
        quad.bg_border.radius = get_radius(sep_style.background_border_radius.clone().unwrap());
    }

    if sep_style.background_shadow_color.is_some() {
        quad.bg_shadow.color = sep_style.background_shadow_color.unwrap();
    }

    if sep_style.background_shadow_offset.is_some() {
        let v = sep_style.background_shadow_offset.unwrap();
        quad.bg_shadow.offset = Vector{ x: v[0], y: v[1] };
    }

    if sep_style.background_shadow_blur_radius.is_some() {
        quad.bg_shadow.blur_radius = sep_style.background_shadow_blur_radius.unwrap();
    }

    match sep_type {
        IpgMenuSeparatorType::Circle => {
            drop(state);
            quad.into()
        },
        IpgMenuSeparatorType::Dot => {
            let row = row((0..20).map(|_| {
                quad.into()
            }))
            .height(sep_style.height)
            .into();
            drop(state);
            row
        },
        IpgMenuSeparatorType::Label => {
            drop(state);
            let q_1 = quad.into();
            let q_2 = quad.into();
            
            let lbl = match label {
                Some(lbl) => lbl,
                None => panic!("Menu Separator: A label is required for IpgMenuSeparatorType::Label.")
            };
        
            Row::with_children(vec![
                                q_1, 
                                Text::new(lbl).into(),
                                q_2,
                                ])
                                .into()
        },
        IpgMenuSeparatorType::Line => {
            drop(state);
            quad.into()
        },
    }

}

fn default_separator(quad_type: IpgMenuSeparatorType, quad_color: Color, bg_color: Color, label: Option<String>) 
-> Element<'static, MenuMessage, iced::Theme, iced::Renderer> 
{
    match quad_type {
        IpgMenuSeparatorType::Circle => {
            let radius = 10.0;
            return Quad {
                height: Length::Fixed(20.0),
                quad_color: Color::from([0.5; 3]).into(),
                inner_bounds: InnerBounds::Square(radius * 2.0),
                quad_border: Border {
                    radius: [radius; 4].into(),
                    ..Default::default()
                },
                
                ..Default::default()
            }.into()
        },
        IpgMenuSeparatorType::Dot => {
            return row((0..20).map(|_| {
                Quad {
                    quad_color: Background::Color(Color::from([0.5; 3]).into()),
                    quad_border: Border {
                        radius: [4.0; 4].into(),
                        ..Default::default()
                    },
                    inner_bounds: InnerBounds::Square(4.0),
                    height: Length::Fixed(20.0),
                    ..Default::default()
                }
                .into()
            }))
            .height(20.0)
            .into()
        },
        IpgMenuSeparatorType::Label => {
            let q_1: Element<MenuMessage, Theme, Renderer> = Quad {
                width: Length::Fixed(20.0),
                ..separator()
            }.into();
            let q_2: Element<MenuMessage, Theme, Renderer> = Quad {
                width: Length::Fixed(20.0),
                ..separator()
            }.into();
        
            let lbl = match label {
                Some(lbl) => lbl,
                None => panic!("Menu Separator: A label is required for IpgMenuSeparatorType::Label.")
            };
        
            return Row::with_children(vec![
                                    q_1, 
                                    Text::new(lbl).into(),
                                    q_2,
                                    ])
                                    .into()
        },
        IpgMenuSeparatorType::Line => {
            return Quad {
                ..separator()
            }.into()
        },
    }
}

fn default_quad(quad_type: IpgMenuSeparatorType, 
                quad_color: Color, 
                bg_color: Color,
                width: Length,
                height: Length,) 
-> Quad
{
    match quad_type {
        IpgMenuSeparatorType::Circle => {
            let radius = 10.0;
            Quad {
                quad_color: quad_color.into(),
                inner_bounds: InnerBounds::Square(radius * 2.0),
                quad_border: Border {
                    radius: [radius; 4].into(),
                    ..Default::default()
                },
                width: width,
                height: height,
                ..Default::default()
            }
        },
        IpgMenuSeparatorType::Dot => {
            Quad {
                quad_color: Background::Color(quad_color),
                inner_bounds: InnerBounds::Square(4.0),
                ..separator()
            }
        },
        IpgMenuSeparatorType::Label => {
            Quad {
                width: width,
                height: height,
                ..separator()
            }
        },
        IpgMenuSeparatorType::Line => {
            Quad {
                ..separator()
            }
        },
    }
}
