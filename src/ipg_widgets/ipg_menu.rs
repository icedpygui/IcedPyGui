//! ipg_menu
// #![allow(dead_code, unused_variables)]
#![allow(clippy::enum_variant_names)]

use iced::{Color, Element, Length, Padding, Renderer, Theme, Vector};

use crate::graphics::colors::get_color;
use crate::iced_aw_widgets::menu::menu_tree::{Item, Menu};
use crate::iced_aw_widgets::menu::menu_bar::MenuBar;
use crate::iced_aw_widgets::menu::common::DrawPath;
use crate::iced_aw_widgets::menu::style_status::Status;
use crate::iced_aw_widgets::menu::menu_bar_style::{primary, Style};


use crate::app;
use super::helpers::{get_height, get_padding_f64, get_radius, get_width, try_extract_array_2, try_extract_boolean, try_extract_f64, try_extract_ipg_color, try_extract_rgba_color, try_extract_vec_f32, try_extract_vec_f64};
use super::ipg_enums::IpgWidgets;


use pyo3::{pyclass, PyObject, Python};

#[derive(Debug, Clone)]
pub struct IpgMenu {
    pub id: usize,
    pub bar_items: usize,
    pub menu_items: Vec<usize>,
    pub bar_width: Length,
    pub item_widths: Vec<Length>,
    pub bar_spacing: f32,
    pub bar_padding: Padding,
    pub bar_height: Length,
    pub check_bounds_width: f32,
    pub item_spacing: Option<Vec<f32>>,
    pub item_offset: Option<Vec<f32>>,
    pub menu_bar_style_id: Option<usize>, // style_id of add_menu_bar_style()
    pub menu_style_id: Option<usize>, // style_id of add_menu_style()
    pub theme: Theme,
    pub show: bool,
    pub user_data: Option<PyObject>,
    pub is_checked: bool,
    pub is_toggled: bool,
}

impl IpgMenu {
    pub fn new(
        id: usize,
        bar_items: usize,
        menu_items: Vec<usize>,
        bar_width: Length,
        item_widths: Vec<Length>,
        bar_spacing: f32,
        bar_padding: Padding,
        bar_height: Length,
        check_bounds_width: f32,
        item_spacing: Option<Vec<f32>>,
        item_offset: Option<Vec<f32>>,
        menu_bar_style_id: Option<usize>,
        menu_style_id: Option<usize>,
        theme: Theme,
        show: bool,
        user_data: Option<PyObject>,
    ) -> Self {
        Self {
            id,
            bar_items,
            menu_items,
            bar_width,
            item_widths,
            bar_spacing,
            bar_padding,
            bar_height,
            check_bounds_width,
            item_spacing,
            item_offset,
            menu_bar_style_id,
            menu_style_id,
            theme,
            show,
            user_data,
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
    pub shadow_offset_xy: Option<[f32; 2]>,
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
        shadow_offset_xy: Option<[f32; 2]>,
        shadow_blur_radius: Option<f32>,
    ) -> Self {
        Self {
            id,
            base_color,
            border_color,
            border_radius,
            border_width,
            shadow_color,
            shadow_offset_xy,
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
    pub shadow_offset_xy: Option<[f32; 2]>,
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
        shadow_offset_xy: Option<[f32; 2]>,
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
            shadow_offset_xy,
            shadow_blur_radius,
            path_base_color,
            path_border_color,
            path_border_radius,
            path_border_width,
        }
    }
}


pub fn construct_menu<'a>(mn: IpgMenu, 
                        mut content: Vec<Element<'a, app::Message>>,
                        bar_style_opt: Option<IpgWidgets>,
                        menu_style_opt: Option<IpgWidgets>)
                        -> Element<'a, app::Message, Theme, Renderer> {
    
    let mut item_spacings = vec![0.0; mn.bar_items];
    if mn.item_spacing.is_some() {
        let spacings = mn.item_spacing.unwrap();
        if spacings.len() == 1 {
            item_spacings = vec![spacings[0]; mn.bar_items]
        } else if spacings.len() != mn.bar_items {
            panic!("Menu spacings: The number of spacings {} must be 1 or match the number of bar items {}.", spacings.len(), mn.bar_items)
        } else {
            item_spacings = spacings;
        }
    }

    let item_widths = if mn.item_widths.len() == 1 {
        vec![mn.item_widths[0]; mn.bar_items]
    } else if mn.item_widths.len() != mn.bar_items {
        panic!("Menu item widths: The number of widths {} must be 1 or match the number of bar items {}.", mn.item_widths.len(), mn.bar_items)
    } else {
        mn.item_widths
    };
    
    let mut item_offsets = vec![0.0; mn.bar_items];
    if mn.item_offset.is_some() {
        let offsets = mn.item_offset.unwrap();
        if offsets.len() == 1 {
            item_offsets = vec![offsets[0]; mn.bar_items]
        } else if offsets.len() != mn.bar_items {
            panic!("Menu offsets: The number of offsets {} must be 1 or match the number of bar items {}.", item_offsets.len(), mn.bar_items)
        } else {
            item_offsets = offsets;
        }
    }

    

    let mut bar_items: Vec<Item<app::Message, Theme, Renderer>> = vec![];

    for bar_index in 0..mn.bar_items {

        let menu_bar = content.remove(0);

        let mut items = vec![];
        
        for _ in 0..mn.menu_items[bar_index] {
            items.push(Item::new(content.remove(0)));
        }
        
        let menu_tpl = 
        |items| Menu::new(items)
            .max_width(100.0) // Don't see any effect
            .spacing(item_spacings[bar_index])
            .width(item_widths[bar_index])
            .offset(item_offsets[bar_index])
            ;

        let bar_item = Item::with_menu(
                                                        menu_bar, 
                                                        menu_tpl(items)
                                                        );

        bar_items.push(bar_item); 
    }
    let bar_style = get_menu_bar_style(bar_style_opt);
    let menu_style: Option<IpgMenuStyle> = get_menu_style(menu_style_opt);

    let mb: MenuBar<'a, app::Message, Theme, Renderer> = MenuBar::new(bar_items)
                .draw_path(DrawPath::Backdrop)
                .style(move|theme:&iced::Theme, status: Status | 
                    get_mb_styling(theme, status, 
                        bar_style.clone(), 
                        menu_style.clone()
                    )
                )
                .spacing(mn.bar_spacing)
                .padding(mn.bar_padding)
                .width(mn.bar_width)
                .height(mn.bar_height)
                .check_bounds_width(mn.check_bounds_width);

    mb.into()
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

        if b_style.shadow_offset_xy.is_some() {
            let offset = b_style.shadow_offset_xy.unwrap();
            menu_style.bar_shadow.offset = 
                Vector{ x: offset[0], y: offset[1] };
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

        if m_style.shadow_offset_xy.is_some() {
            let offset = m_style.shadow_offset_xy.unwrap();
            menu_style.menu_shadow.offset = 
                Vector{ x: offset[0], y:offset[1] };
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

#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgMenuParam {
    BarHeight,
    BarHeightFill,
    BarPadding,
    BarSpacing,
    BarWidth,
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
            let width = get_width(Some(try_extract_f64(value) as f32), false);
            mn.bar_width = width;
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
    ShadowOffsetXY,
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
        IpgMenuBarStyleParam::ShadowOffsetXY => {
            style.shadow_offset_xy = Some(try_extract_array_2(value));
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
    ShadowOffsetXY,
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
        IpgMenuStyleParam::ShadowOffsetXY => {
            style.shadow_offset_xy = Some(try_extract_array_2(value));
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

