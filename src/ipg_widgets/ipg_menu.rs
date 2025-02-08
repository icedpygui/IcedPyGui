//! ipg_menu
// #![allow(dead_code, unused_variables)]
#![allow(clippy::enum_variant_names)]
use std::collections::BTreeMap;

use iced::border::Radius;
use iced::widget::button;
use iced::widget::{row, Button, Checkbox, Container, Row, Text, Toggler};
use iced::{alignment, Background, Border, Color, Element, Length, Padding, Renderer, Theme, Vector};

use crate::graphics::colors::get_color;
use crate::iced_aw_widgets::menu::menu_tree::{Item, Menu};
use crate::iced_aw_widgets::menu::menu_bar::MenuBar;
use crate::iced_aw_widgets::menu::common::DrawPath;
use crate::iced_aw_widgets::menu::quad::{InnerBounds, Quad};
use crate::iced_aw_widgets::menu::style_status::Status;
use crate::iced_aw_widgets::menu::menu_bar_style::{primary, Style};

use crate::style::styling::{is_dark, IpgStyleStandard};
use crate::{access_callbacks, access_state, app, IpgState};
use super::callbacks::{set_or_get_widget_callback_data, 
    WidgetCallbackIn, WidgetCallbackOut};
use super::helpers::{get_height, get_padding_f64, get_radius, 
    try_extract_array_2, try_extract_boolean, try_extract_f64, 
    try_extract_ipg_color, try_extract_rgba_color, 
    try_extract_vec_f32, try_extract_vec_f64};
use super::ipg_button::get_btn_style;
use super::ipg_checkbox::get_chk_style;
use super::ipg_enums::IpgWidgets;
use super::ipg_toggle::get_toggler_style;
use super::{ipg_button, ipg_checkbox, ipg_toggle};

use pyo3::{pyclass, PyObject, Python};

#[derive(Debug, Clone)]
pub struct IpgMenu {
    pub id: usize,
    pub items: Vec<Vec<(Option<String>, IpgMenuType, Option<usize>)>>,
    pub bar_widths: Vec<f32>,
    pub item_widths: Vec<f32>,
    pub bar_spacing: f32,
    pub bar_padding: Padding,
    pub bar_height: Length,
    pub check_bounds_width: f32,
    pub item_spacings: Option<Vec<f32>>,
    pub item_offsets: Option<Vec<f32>>,
    pub menu_bar_style_id: Option<usize>, // style_id of add_menu_bar_style()
    pub menu_style_id: Option<usize>, // style_id of add_menu_style()
    pub theme: Theme,
    pub show: bool,
    pub user_data: Option<PyObject>,
    new_menu: bool, 
    updating_separators: bool,
    pub is_checked: bool,
    pub is_toggled: bool,
}

impl IpgMenu {
    pub fn new(
        id: usize,
        items: Vec<Vec<(Option<String>, IpgMenuType, Option<usize>)>>,
        bar_widths: Vec<f32>,
        item_widths: Vec<f32>,
        bar_spacing: f32,
        bar_padding: Padding,
        bar_height: Length,
        check_bounds_width: f32,
        item_spacings: Option<Vec<f32>>,
        item_offsets: Option<Vec<f32>>,
        menu_bar_style_id: Option<usize>,
        menu_style_id: Option<usize>,
        theme: Theme,
        show: bool,
        user_data: Option<PyObject>,
    ) -> Self {
        Self {
            id,
            items,
            bar_widths,
            item_widths,
            bar_spacing,
            bar_padding,
            bar_height,
            check_bounds_width,
            item_spacings,
            item_offsets,
            menu_bar_style_id,
            menu_style_id,
            theme,
            show,
            user_data,
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
    pub base_color: Option<Color>, // background
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
        base_color: Option<Color>,
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
            base_color,
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
    pub base_color: Option<Color>, // background
    pub border_color: Option<Color>,
    pub border_radius: Option<Vec<f32>>,
    pub border_width: Option<f32>,
    pub shadow_color: Option<Color>,
    pub shadow_offset_x: Option<f32>,
    pub shadow_offset_y: Option<f32>,
    pub shadow_blur_radius: Option<f32>,
    pub path_base_color: Option<Color>,
    pub path_border_color: Option<Color>,
    pub path_border_radius: Option<Vec<f32>>,
    pub path_border_width: Option<f32>,
}

impl IpgMenuStyle {
    pub fn new(
        id: usize,
        base_color: Option<Color>,
        border_color: Option<Color>,
        border_radius: Option<Vec<f32>>,
        border_width: Option<f32>,
        shadow_color: Option<Color>,
        shadow_offset_x: Option<f32>,
        shadow_offset_y: Option<f32>,
        shadow_blur_radius: Option<f32>,
        path_base_color: Option<Color>,
        path_border_color: Option<Color>,
        path_border_radius: Option<Vec<f32>>,
        path_border_width: Option<f32>,
    ) -> Self {
        Self {
            id,
            base_color,
            border_color,
            border_radius,
            border_width,
            shadow_color,
            shadow_offset_x,
            shadow_offset_y,
            shadow_blur_radius,
            path_base_color,
            path_border_color,
            path_border_radius,
            path_border_width,
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

pub fn construct_menu(mut mn: IpgMenu, 
                        state: &IpgState)
                        -> Element<'static, app::Message, Theme, Renderer> {

    let menu_style_opt = match mn.menu_style_id.clone() {
                        Some(id) => {
                            state.widgets.get(&id).map(|st|st.clone())
                        },
                        None => None,
                    };
                    
    let bar_style_opt = match mn.menu_bar_style_id.clone() {
        Some(id) => {
            state.widgets.get(&id).map(|st|st.clone())
        },
        None => None,
    };
    
    let column_count = mn.items[0].len();
;
    // default the spacing and widths if new menu.
    // Occurs during menu updating.
    if mn.new_menu {
        mn.item_spacings = Some(vec![0.0; column_count]);
        mn.item_widths = vec![0.0; column_count];
        mn.item_offsets = Some(vec![0.0; column_count]);
        mn.new_menu = false;
    }

    let mut item_spacings = vec![0.0; column_count];
    if mn.item_spacings.is_some() {
        let spacings = mn.item_spacings.unwrap();
        if spacings.len() == 1 {
            item_spacings = vec![spacings[0]; column_count]
        } else if spacings.len() != column_count {
            panic!("Menu spacings: The number of spacings {} must be 1 or match the number of bar items {}.", spacings.len(), column_count)
        } else {
            item_spacings = spacings;
        }
    }

    let item_widths = if mn.item_widths.len() == 1 {
        vec![mn.item_widths[0]; column_count]
    } else if mn.item_widths.len() != column_count {
        panic!("Menu item widths: The number of widths {} must be 1 or match the number of bar items {}.", mn.item_widths.len(), column_count)
    } else {
        mn.item_widths
    };
    
    let bar_widths = if mn.bar_widths.len() == 1 {
        vec![mn.bar_widths[0]; column_count]
    } else if mn.bar_widths.len() != column_count {
        panic!("Menu bar_widths: The number of widths {} must be 1 or match the number of bar items {}.", mn.bar_widths.len(), column_count)
    } else {
        mn.bar_widths
    };

    let mut item_offsets = vec![0.0; column_count];
    if mn.item_offsets.is_some() {
        let offsets = mn.item_offsets.unwrap();
        if offsets.len() == 1 {
            item_offsets = vec![offsets[0]; column_count]
        } else if offsets.len() != column_count {
            panic!("Menu offsets: The number of offsets {} must be 1 or match the number of bar items {}.", item_offsets.len(), column_count)
        } else {
            item_offsets = offsets;
        }
    }

    let menu_bar: Vec<Element<'static, MenuMessage, Theme, Renderer>> = vec![];

    let mut bar_items: Vec<Item<MenuMessage, Theme, Renderer>> = vec![];

    for (bar_index, (bar_label, bar_item_type, bar_item_style_id)) 
        in mn.items[0].iter().enumerate() {
        
        let mut items: Vec<Item<'static, MenuMessage, Theme, Renderer>> = vec![];
        
        for item_index in 1..mn.items[bar_index].len() {
            items.push(get_menu_item(mn.items[bar_index][item_index].0.clone(),
                                    mn.items[bar_index][item_index].1.clone(),
                                    mn.items[bar_index][item_index].2.clone(),
                                    bar_index,
                                    item_index,
                                    state,
                                    ));
        }
        
        let menu_tpl = 
        |items| Menu::new(items)
            .max_width(100.0) // Don't see any effect
            .spacing(item_spacings[bar_index])
            .width(item_widths[bar_index])
            .offset(item_offsets[bar_index])
            ;

        let bar_item = Item::with_menu(menu_bar_button(
                                                        bar_label.clone(),
                                                        item_widths[bar_index],
                                                        bar_index,
                                                        *bar_item_style_id), 
                                                        menu_tpl(items)
                                                        );

        bar_items.push(bar_item); 
    }
    let bar_style = get_menu_bar_style(bar_style_opt);
    let menu_style = get_menu_style(menu_style_opt);

    let mb = MenuBar::new(bar_items)
                .draw_path(DrawPath::Backdrop)
                .style(move|theme:&iced::Theme, status: Status | 
                    get_mb_styling(theme, status, 
                        bar_style.clone(), 
                        menu_style.clone()
                    )
                )
                .spacing(mn.bar_spacing)
                .padding(mn.bar_padding)
                // .width(bar_widths[bar_index])
                .height(mn.bar_height)
                .check_bounds_width(mn.check_bounds_width);

    let ipg_menu: Element<MenuMessage, Theme, Renderer> = Container::new(mb).into();

    ipg_menu.map(move |message| app::Message::Menu(mn.id, message))

}

fn get_mb_styling(theme: &Theme, 
                    status: Status,
                    br_style: Option<IpgMenuBarStyle>,
                    mn_style: Option<IpgMenuStyle>,
                ) -> Style {

    let mut menu_style = primary(theme, status);

    if br_style.is_none() && mn_style.is_none() {
        return menu_style
    }

    if br_style.is_some() {

        let b_style = br_style.unwrap();

        if b_style.base_color.is_some() {
            menu_style.bar_background = b_style.base_color.unwrap().into();
        }

        if b_style.border_width.is_some() {
            menu_style.bar_border.width = b_style.border_width.unwrap();
        }

        if b_style.border_color.is_some() {
            // just in case the user forget to set width, then something shows
            if menu_style.bar_border.width == 0.0 {
                menu_style.bar_border.width = 1.0;
            }
            menu_style.bar_border.color = b_style.border_color.unwrap();
        }

        if b_style.border_radius.is_some() {
            menu_style.bar_border.radius = get_radius(b_style.border_radius.clone().unwrap(),
                                                "Menu".to_string());
        }

        if b_style.shadow_color.is_some() {
            menu_style.bar_shadow.color = b_style.shadow_color.unwrap();
        }

        if b_style.shadow_offset_x.is_some() {
            let v = menu_style.bar_shadow.offset;
            menu_style.bar_shadow.offset = 
                Vector{ x: b_style.shadow_offset_x.unwrap(), y: v.y };
        }

        if b_style.shadow_offset_y.is_some() {
            let v = menu_style.bar_shadow.offset;
            menu_style.bar_shadow.offset = 
                Vector{ x: v.x , y: b_style.shadow_offset_y.unwrap() };
        }

        if b_style.shadow_blur_radius.is_some() {
            menu_style.bar_shadow.blur_radius = b_style.shadow_blur_radius.unwrap();
        }
    }

    if mn_style.is_some() {

        let m_style = mn_style.unwrap();

        if m_style.base_color.is_some() {
            menu_style.menu_background = m_style.base_color.unwrap().into();
        }

        if m_style.border_width.is_some() {
            menu_style.menu_border.width = m_style.border_width.unwrap();
        }

        if m_style.border_color.is_some() {
            // just in case the user forget to set width, then something shows
            if menu_style.menu_border.width == 0.0 {
                menu_style.menu_border.width = 1.0;
            }
            menu_style.menu_border.color = m_style.border_color.unwrap();
        }

        if m_style.border_radius.is_some() {
            menu_style.menu_border.radius = 
                get_radius(
                    m_style.border_radius.clone().unwrap(),
                    "Menu".to_string()
                );
        }

        if m_style.shadow_color.is_some() {
            menu_style.menu_shadow.color = m_style.shadow_color.unwrap();
        }

        if m_style.shadow_offset_x.is_some() {
            let v = menu_style.menu_shadow.offset;
            menu_style.menu_shadow.offset = 
                Vector{ x: m_style.shadow_offset_x.unwrap(), y: v.y };
        }

        if m_style.shadow_offset_y.is_some() {
            let v = menu_style.menu_shadow.offset;
            menu_style.menu_shadow.offset = 
                Vector{ x: v.x , y: m_style.shadow_offset_y.unwrap() };
        }

        if m_style.shadow_blur_radius.is_some() {
            menu_style.menu_shadow.blur_radius = 
                m_style.shadow_blur_radius.unwrap();
        }

        if m_style.path_base_color.is_some() {
            menu_style.path = m_style.path_base_color.unwrap().into();
        }

        if m_style.path_border_width.is_some() {
            menu_style.path_border.width = m_style.path_border_width.unwrap();
        }

        if m_style.path_border_color.is_some() {
            // just in case the user forget to set width, then something shows
            if menu_style.path_border.width == 0.0 {
                menu_style.path_border.width = 1.0;
            }
            menu_style.path_border.color = m_style.path_border_color.unwrap();
        }

        if m_style.path_border_radius.is_some() {
            menu_style.path_border.radius = 
                get_radius(
                    m_style.path_border_radius.clone().unwrap(),
                    "Menu".to_string()
                );
        }
    }

    menu_style

}


pub fn menu_callback(state: &mut IpgState, id: usize, message: MenuMessage) {
    let mut wci = WidgetCallbackIn{id, ..Default::default()};
    
    match message {
        MenuMessage::ItemPressed((bar_index, menu_index)) => {
            let mut wco: WidgetCallbackOut = set_or_get_widget_callback_data(state, wci);
            wco.id = id;
            wco.bar_index = Some(bar_index);
            wco.menu_index = Some(menu_index);
            wco.event_name = "on_select".to_string();
            process_callback(wco);
        }
        MenuMessage::ItemCheckToggled(is_checked, (bar_index, menu_index)) => {
            wci.is_checked = Some(is_checked);
            let mut wco: WidgetCallbackOut = set_or_get_widget_callback_data(state, wci);
            wco.id = id;
            wco.bar_index = Some(bar_index);
            wco.menu_index = Some(menu_index);
            wco.event_name = "on_select".to_string();
            process_callback(wco);
        },
        MenuMessage::ItemTogToggled(togged, (bar_index, menu_index)) => {
            wci.on_toggle = Some(togged)
;            let mut wco: WidgetCallbackOut = set_or_get_widget_callback_data(state, wci);
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
                                                                    wco.id,
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
                                                                    wco.id,
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
    BarWidths,
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
        IpgMenuParam::BarWidths => {
            mn.bar_widths = try_extract_vec_f32(value);
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
                item_style_id: Option<usize>,
                bar_index: usize,
                item_index: usize,
                state: &IpgState,
                ) -> Item<'static, MenuMessage, Theme, Renderer> {
    

    let style_opt = match item_style_id {
        Some(id) => {
            state.widgets.get(&id).map(|st|st.clone())
        },
        None => None,
    };
  
    match_menu_item(
        item_type, 
        style_opt, 
        bar_index, item_index, 
        false, false, 
        label)
            
}

fn match_menu_item(item_type: IpgMenuType,
                    style: Option<IpgWidgets>,
                    bar_index: usize, item_index: usize, 
                    is_checked: bool, is_toggled: bool,
                    label: Option<String>,
                    ) -> Item<'static, MenuMessage, Theme, Renderer> 
{
    let mut lbl = "".to_string();
    if label.is_some() && item_type!= IpgMenuType::Line {
        lbl = label.clone().unwrap();
    }

    match item_type {
        IpgMenuType::Button => {
            let label_txt: Element<MenuMessage, Theme, Renderer> = Text::new(lbl).into();

            let style_opt = get_btn_style(style);
            let style_standard = if style_opt.is_none() {
                Some(IpgStyleStandard::Primary)
            } else {
                None
            };

            let btn: Element<MenuMessage, Theme, Renderer> = 
                            Button::new(label_txt)
                                    .on_press(MenuMessage::ItemPressed((bar_index, item_index)))
                                    .width(Length::Fill)
                                    .style(move|theme: &Theme, status| {
                                        ipg_button::get_styling(theme, status, 
                                                                style_opt,
                                                                style_standard) 
                                        })
                                    .into();
            Item::new(btn)
        },
        IpgMenuType::Checkbox => {
            let style_opt = get_chk_style(style);
            let style_standard = if style_opt.is_none() {
                Some(IpgStyleStandard::Primary)
            } else {
                None
            };
            let chkbx: Element<MenuMessage, Theme, Renderer> = 
                        Checkbox::new(lbl, 
                            is_checked)
                            .on_toggle(move|b| MenuMessage::ItemCheckToggled(b, (bar_index, item_index)))
                            .style(move|theme: &Theme, status| {
                                ipg_checkbox::get_styling(theme, status, 
                                                        style_opt,
                                                        style_standard,
                                                        is_checked) 
                            })
                            .into();
            Item::new(chkbx)
        },
        IpgMenuType::Toggler => {
            let style_opt = get_toggler_style(style);
            let style_standard = if style_opt.is_none() {
                Some(IpgStyleStandard::Primary)
            } else {
                None
            };
            let tog: Element<MenuMessage, Theme, Renderer> = 
                        Toggler::new(is_toggled)
                            .on_toggle(move|b| 
                                MenuMessage::ItemTogToggled(b, (bar_index, item_index)))
                            .label(lbl)
                            .style(move|theme: &Theme, status| {     
                                ipg_toggle::get_styling(theme, status, 
                                                        None) 
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
                    bar_style_id: Option<usize>,
                ) -> Element<'static, MenuMessage, Theme, Renderer> {

    let mut style = None;
    let mut style_standard = Noe;                
    if item_style_id.is_some() {
        style = Some(state.widgets.get(&item_style_id.unwrap()).map(|st|st.clone()));
    } else {       
        style_standard = Some(IpgStyleStandard::Primary);
    }

    let label_txt: Element<'static, MenuMessage, Theme, Renderer> = 
                                Text::new(label.clone())
                                    .align_x(alignment::Horizontal::Center)
                                    .align_y(alignment::Vertical::Center)
                                    .into();

    let btn: Element<'static, MenuMessage, Theme, Renderer> = 
                                Button::new(label_txt)
                                    .on_press(MenuMessage::ItemPressed((bar_index, 999)))
                                    .width(width)
                                    .style(move|theme: &Theme, status| {
                                        if style_standard.is_none() && style.is_none() {
                                            button::text(theme, status)
                                        } else {
                                            ipg_button::get_styling(theme, status, 
                                                None, style_standard.clone())
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
            radius: Radius::new(4.0),
            ..Default::default()
        },
        inner_bounds: InnerBounds::Ratio(0.98, 0.2),
        height: Length::Fixed(20.0),
        ..Default::default()
    }
}

fn try_extract_dict(items: PyObject) -> BTreeMap<String, Vec<(Option<String>, IpgMenuType, usize)>> {
    Python::with_gil(|py| {

        let res = items.extract::<BTreeMap<String, Vec<(Option<String>, IpgMenuType, usize)>>>(py);
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

    let quad_color = if is_dark(bg_color) {
        Color::WHITE
    } else {
        Color::BLACK
    };

    if style_id.is_none() {
            return default_separator(sep_type.clone(), quad_color, bg_color, label)
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
        quad.quad_border.radius = get_radius(sep_style.separator_border_radius.clone().unwrap(),
                                    "Menu".to_string());
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
        quad.bg_border.radius = get_radius(sep_style.background_border_radius.clone().unwrap(),
                                    "Menu".to_string());
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
            Quad {
                height: Length::Fixed(20.0),
                quad_color: Color::from([0.5; 3]).into(),
                inner_bounds: InnerBounds::Square(radius * 2.0),
                quad_border: Border {
                    radius: Radius::new(4.0),
                    ..Default::default()
                },
                
                ..Default::default()
            }.into()
        },
        IpgMenuSeparatorType::Dot => {
            return row((0..20).map(|_| {
                Quad {
                    quad_color: Background::Color(Color::from([0.5; 3])),
                    quad_border: Border {
                        radius: Radius::new(4.0),
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
            Quad {
                ..separator()
            }.into()
        },
    }
}

fn default_quad(quad_type: IpgMenuSeparatorType, 
                quad_color: Color, 
                _bg_color: Color,
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
                    radius: Radius::new(radius),
                    ..Default::default()
                },
                width,
                height,
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
                width,
                height,
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

#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgMenuBarStyleParam {
    BaseIpgColor,
    BaseRgbaColor,
    BorderIpgColor,
    BorderRgbaColor,
    BorderRadius,
    BorderWidth,
    ShadowIpgColor,
    ShadowRgbaColor,
    ShadowOffsetX,
    ShadowOffsetY,
    ShadowBlurRadius,
}

pub fn menu_bar_style_update_item(style: &mut IpgMenuBarStyle,
                            item: PyObject,
                            value: PyObject,) 
{

    let update = try_extract_menu_bar_style_update(item);
    match update {
        IpgMenuBarStyleParam::BaseIpgColor => {
            let color = try_extract_ipg_color(value);
            style.base_color = get_color(None, Some(color), 1.0, false);
        },
        IpgMenuBarStyleParam::BaseRgbaColor => {
            style.base_color = Some(Color::from(try_extract_rgba_color(value)));
        },
        IpgMenuBarStyleParam::BorderIpgColor => {
            let color = try_extract_ipg_color(value);
            style.border_color = get_color(None, Some(color), 1.0, false);
        },
        IpgMenuBarStyleParam::BorderRgbaColor => {
            style.border_color = Some(Color::from(try_extract_rgba_color(value)));
        },
        IpgMenuBarStyleParam::BorderRadius => {
            style.border_radius = Some(try_extract_vec_f32(value));
        },
        IpgMenuBarStyleParam::BorderWidth => {
            style.border_width = Some(try_extract_f64(value) as f32);
        },
        IpgMenuBarStyleParam::ShadowIpgColor => {
            let color = try_extract_ipg_color(value);
            style.shadow_color = get_color(None, Some(color), 1.0, false);
        },
        IpgMenuBarStyleParam::ShadowRgbaColor => {
            style.shadow_color = Some(Color::from(try_extract_rgba_color(value)));
        },
        IpgMenuBarStyleParam::ShadowOffsetX => {
            style.shadow_offset_x = Some(try_extract_f64(value) as f32);
        },
        IpgMenuBarStyleParam::ShadowOffsetY => {
            style.shadow_offset_y = Some(try_extract_f64(value) as f32);
        },
        IpgMenuBarStyleParam::ShadowBlurRadius => {
            style.shadow_blur_radius = Some(try_extract_f64(value) as f32);
        },
    }
}

fn get_menu_bar_style(style: Option<IpgWidgets>) -> Option<IpgMenuBarStyle>{
    match style {
        Some(st) => {
            match st {
                IpgWidgets::IpgMenuBarStyle(style) => {
                    Some(style)
                }
                _ => None,
            }
        },
        None => None,
    }
}

pub fn try_extract_menu_bar_style_update(update_obj: PyObject) -> IpgMenuBarStyleParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgMenuBarStyleParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Menu Bar style parameter update extraction failed"),
        }
    })
}

#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgMenuStyleParam {
    BaseIpgColor,
    BaseRgbaColor,
    BorderIpgColor,
    BorderRgbaColor,
    BorderRadius,
    BorderWidth,
    ShadowIpgColor,
    ShadowRgbaColor,
    ShadowOffsetX,
    ShadowOffsetY,
    ShadowBlurRadius,
    PathBaseIpgColor,
    PathBaseRgbaColor,
    PathBorderIpgColor,
    PathBorderRgbaColor,
    PathBorderRadius,
    PathBorderWidth,
}

pub fn menu_style_update_item(style: &mut IpgMenuStyle,
                            item: PyObject,
                            value: PyObject,) 
{
    let update = try_extract_menu_style_update(item);
    match update {
        IpgMenuStyleParam::BaseIpgColor => {
            let color = try_extract_ipg_color(value);
            style.base_color = get_color(None, Some(color), 1.0, false);
        },
        IpgMenuStyleParam::BaseRgbaColor => {
            style.base_color = Some(Color::from(try_extract_rgba_color(value)));
        },
        IpgMenuStyleParam::BorderIpgColor => {
            let color = try_extract_ipg_color(value);
            style.border_color = get_color(None, Some(color), 1.0, false);
        },
        IpgMenuStyleParam::BorderRgbaColor => {
            style.border_color = Some(Color::from(try_extract_rgba_color(value)));
        },
        IpgMenuStyleParam::BorderRadius => {
            style.border_radius = Some(try_extract_vec_f32(value));
        },
        IpgMenuStyleParam::BorderWidth => {
            style.border_width = Some(try_extract_f64(value) as f32);
        },
        IpgMenuStyleParam::ShadowIpgColor => {
            let color = try_extract_ipg_color(value);
            style.shadow_color = get_color(None, Some(color), 1.0, false);
        },
        IpgMenuStyleParam::ShadowRgbaColor => {
            style.shadow_color = Some(Color::from(try_extract_rgba_color(value)));
        },
        IpgMenuStyleParam::ShadowOffsetX => {
            style.shadow_offset_x = Some(try_extract_f64(value) as f32);
        },
        IpgMenuStyleParam::ShadowOffsetY => {
            style.shadow_offset_y = Some(try_extract_f64(value) as f32);
        },
        IpgMenuStyleParam::ShadowBlurRadius => {
            style.shadow_blur_radius = Some(try_extract_f64(value) as f32);
        },
        IpgMenuStyleParam::PathBaseIpgColor => {
            let color = try_extract_ipg_color(value);
            style.path_base_color = get_color(None, Some(color), 1.0, false);
        },
        IpgMenuStyleParam::PathBaseRgbaColor => {
            style.path_base_color = Some(Color::from(try_extract_rgba_color(value)));
        },
        IpgMenuStyleParam::PathBorderIpgColor => {
            let color = try_extract_ipg_color(value);
            style.path_border_color = get_color(None, Some(color), 1.0, false);
        },
        IpgMenuStyleParam::PathBorderRgbaColor => {
            style.path_border_color = Some(Color::from(try_extract_rgba_color(value)));
        },
        IpgMenuStyleParam::PathBorderRadius => {
            style.path_border_radius = Some(try_extract_vec_f32(value));
        },
        IpgMenuStyleParam::PathBorderWidth => {
            style.path_border_width = Some(try_extract_f64(value) as f32);
        },
    }
}

fn get_menu_style(style: Option<IpgWidgets>) -> Option<IpgMenuStyle>{
    match style {
        Some(st) => {
            match st {
                IpgWidgets::IpgMenuStyle(style) => {
                    Some(style)
                }
                _ => None,
            }
        },
        None => None,
    }
}

pub fn try_extract_menu_style_update(update_obj: PyObject) -> IpgMenuStyleParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgMenuStyleParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Menu style parameter update extraction failed"),
        }
    })
}

#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgMenuSeparatorStyleParam {
    SeparatorType,
    Width,
    WidthFill,
    Height,
    HeightFill,
    QuadRatios,
    SeparatorIpgColor,
    SeparatorRgbaColor,
    SeparatorBorderIpgColor,
    SeparatorBorderRgbaColor,
    SeparatorBorderRadius,
    SeparatorBorderWidth,
    SeparatorShadowIpgColor,
    SeparatorShadowRgbaColor,
    SeparatorShadowOffset,
    SeparatorShadowBlurRadius,
    BackgroundIpgColor,
    BackgroundRgbaColor,
    BackgroundBorderIpgColor,
    BackgroundBorderRbgaColor,
    BackgroundBorderWidth,
    BackgroundBorderRadius,
    BackgroundShadowIpgColor,
    BackgroundShadowRbgaColor,
    BackgroundShadowOffset,
    BackgroundShadowBlurRadius,
}

pub fn menu_separator_style_update_item(style: &mut IpgMenuSeparatorStyle,
                            item: PyObject,
                            value: PyObject,) 
{

    let update = try_extract_menu_separator_style_update(item);
    match update {
        IpgMenuSeparatorStyleParam::SeparatorType => {
            style.separator_type = try_extract_menu_separator_type(value)
        },
        IpgMenuSeparatorStyleParam::Width => {
            style.width = Length::Fixed(try_extract_f64(value) as f32);
        },
        IpgMenuSeparatorStyleParam::WidthFill => {
            let width = try_extract_boolean(value);
            if width {
                style.width = Length::Fill;
            } else {
                style.width = Length::Shrink;
            }
        },
        IpgMenuSeparatorStyleParam::Height => {
            style.height = Length::Fixed(try_extract_f64(value) as f32);
        },
        IpgMenuSeparatorStyleParam::HeightFill => {
            let height = try_extract_boolean(value);
            if height {
                style.height = Length::Fill;
            } else {
                style.height = Length::Shrink;
            }
        },
        IpgMenuSeparatorStyleParam::QuadRatios => {
            style.quad_ratios = Some(try_extract_array_2(value));
        },
        IpgMenuSeparatorStyleParam::SeparatorIpgColor => {
            let color = try_extract_ipg_color(value);
            style.separator_color = get_color(None, Some(color), 1.0, false);
        },
        IpgMenuSeparatorStyleParam::SeparatorRgbaColor => {
            style.separator_color = Some(Color::from(try_extract_rgba_color(value)));
        },
        IpgMenuSeparatorStyleParam::SeparatorBorderIpgColor => {
            let color = try_extract_ipg_color(value);
            style.separator_border_color = get_color(None, Some(color), 1.0, false);
        },
        IpgMenuSeparatorStyleParam::SeparatorBorderRgbaColor => {
            style.separator_border_color = Some(Color::from(try_extract_rgba_color(value)));
        },
        IpgMenuSeparatorStyleParam::SeparatorBorderRadius => {
            style.separator_border_radius = Some(try_extract_vec_f32(value));
        },
        IpgMenuSeparatorStyleParam::SeparatorBorderWidth => {
            style.separator_border_width = Some(try_extract_f64(value) as f32);
        },
        IpgMenuSeparatorStyleParam::SeparatorShadowIpgColor => {
            let color = try_extract_ipg_color(value);
            style.separator_shadow_color = get_color(None, Some(color), 1.0, false);
        },
        IpgMenuSeparatorStyleParam::SeparatorShadowRgbaColor => {
            style.separator_shadow_color = Some(Color::from(try_extract_rgba_color(value)));
        },
        IpgMenuSeparatorStyleParam::SeparatorShadowOffset => {
            style.separator_shadow_offset = Some(try_extract_array_2(value));
        },
        IpgMenuSeparatorStyleParam::SeparatorShadowBlurRadius => {
            style.separator_shadow_blur_radius = Some(try_extract_f64(value) as f32);
        },
        IpgMenuSeparatorStyleParam::BackgroundIpgColor => {
            let color = try_extract_ipg_color(value);
            style.background_color = get_color(None, Some(color), 1.0, false);
        },
        IpgMenuSeparatorStyleParam::BackgroundRgbaColor => {
            style.background_color = Some(Color::from(try_extract_rgba_color(value)));
        },
        IpgMenuSeparatorStyleParam::BackgroundBorderIpgColor => {
            let color = try_extract_ipg_color(value);
            style.background_border_color = get_color(None, Some(color), 1.0, false);
        },
        IpgMenuSeparatorStyleParam::BackgroundBorderRbgaColor => {
            style.background_border_color = Some(Color::from(try_extract_rgba_color(value)));
        },
        IpgMenuSeparatorStyleParam::BackgroundBorderWidth => {
            style.background_border_width = Some(try_extract_f64(value) as f32);
        },
        IpgMenuSeparatorStyleParam::BackgroundBorderRadius => {
            style.background_border_radius = Some(try_extract_vec_f32(value));
        },
        IpgMenuSeparatorStyleParam::BackgroundShadowIpgColor => {
            let color = try_extract_ipg_color(value);
            style.background_shadow_color = get_color(None, Some(color), 1.0, false);
        },
        IpgMenuSeparatorStyleParam::BackgroundShadowRbgaColor => {
            style.background_shadow_color = Some(Color::from(try_extract_rgba_color(value)));
        },
        IpgMenuSeparatorStyleParam::BackgroundShadowOffset => {
            style.background_shadow_offset = Some(try_extract_array_2(value));
        },
        IpgMenuSeparatorStyleParam::BackgroundShadowBlurRadius => {
            style.background_shadow_blur_radius = Some(try_extract_f64(value) as f32);
        },
    }
}

fn get_menu_separator_style(style: Option<IpgWidgets>) -> Option<IpgMenuSeparatorStyle>{
    match style {
        Some(st) => {
            match st {
                IpgWidgets::IpgMenuSeparatorStyle(style) => {
                    Some(style)
                }
                _ => None,
            }
        },
        None => None,
    }
}

pub fn try_extract_menu_separator_style_update(update_obj: PyObject) -> IpgMenuSeparatorStyleParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgMenuSeparatorStyleParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Menu separator style parameter update extraction failed"),
        }
    })
}
pub fn try_extract_menu_separator_type(value_obj: PyObject) -> IpgMenuSeparatorType {

    Python::with_gil(|py| {
        let res = value_obj.extract::<IpgMenuSeparatorType>(py);
        match res {
            Ok(m_type) => m_type,
            Err(_) => panic!("Menu separator type parameter value extraction failed"),
        }
    })
}
