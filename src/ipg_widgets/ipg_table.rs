//! ipg_table
#![allow(clippy::unit_arg)]

use crate::app::Message;

use crate::IpgState;


use iced::advanced::graphics::core::Element;
use iced::widget::scrollable::{Anchor, Scrollbar};
use iced::Alignment;
use iced::{Length, Renderer, Theme};
use iced::widget::{column, container, row, text};

use polars::frame::DataFrame;
use pyo3::{pyclass, PyObject, Python};
use pyo3_polars::PyDataFrame;

use super::callbacks::{set_or_get_widget_callback_data, 
    WidgetCallbackIn};
use super::divider;
use super::helpers::{get_radius, try_extract_f32, try_extract_ipg_color};
use super::helpers::try_extract_rgba_color;
use super::helpers::{get_width, try_extract_boolean, 
    try_extract_f64, try_extract_string, try_extract_vec_f32};
use super::ipg_enums::IpgWidgets;


#[derive(Debug, Clone)]
pub struct IpgTable {
        pub id: usize,
        pub title: String,
        pub df: DataFrame,
        pub column_widths: Vec<f32>,
        pub height: f32,
        // above required
        pub width: Length,
        pub header_enabled: bool,
        pub header_custom_enabled: bool,
        pub footer: Option<Vec<String>>,
        pub control_columns: Vec<usize>,
        pub hide_columns: Vec<usize>,
        pub column_spacing: f32,
        pub row_spacing: f32,
        pub row_max_height: Option<f32>,
        pub divider_width: f32,
        pub resize_columns_enabled: bool,
        pub min_column_width: Option<f32>,
        pub cell_padding: f32,
        pub show: bool,
        pub resize_offset: Vec<Option<f32>>,
        pub table_width_fixed: bool,
        pub table_width: f32,
        pub scroller_width: f32,
        pub scroller_bar_width: f32,
        pub scroller_margin: f32,
        pub style_id: Option<usize>,
}

impl IpgTable {
    pub fn new( 
        id: usize,
        title: String,
        df: DataFrame,
        column_widths: Vec<f32>,
        height: f32,
        width: Length,
        header_enabled: bool,
        header_custom_enabled: bool,
        footer: Option<Vec<String>>,
        control_columns: Vec<usize>,
        hide_columns: Vec<usize>,
        column_spacing: f32,
        row_spacing: f32,
        row_max_height: Option<f32>,
        divider_width: f32,
        resize_columns_enabled: bool,
        min_column_width: Option<f32>,
        cell_padding: f32,
        show: bool,
        resize_offset: Vec<Option<f32>>,
        table_width_fixed: bool,
        table_width: f32,
        scroller_width: f32,
        scroller_bar_width: f32,
        scroller_margin: f32,
        style_id: Option<usize>,
        ) -> Self {
        Self {
            id,
            title,
            df,
            column_widths,
            height,
            width,
            header_enabled,
            header_custom_enabled,
            footer,
            control_columns,
            hide_columns,
            column_spacing,
            row_spacing,
            row_max_height,
            divider_width,
            resize_columns_enabled,
            min_column_width,
            cell_padding,
            show,
            resize_offset,
            table_width_fixed,
            table_width,
            scroller_width,
            scroller_bar_width,
            scroller_margin,
            style_id,
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgTableRowHighLight {
    Darker,
    Lighter,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum IpgTableMouse {
    #[default]
    None,
    Resizing,
    Resized,
}

pub fn construct_table<'a>(tbl: IpgTable, 
                            content: Vec<Element<'a, Message, Theme, Renderer>>,
                            style_opt: Option<&IpgWidgets>, 
                            ) 
                            -> Element<'a, Message, Theme, Renderer> {

    let style = get_table_style(style_opt);
    

    // get the header
    let header = if tbl.header_enabled {
        let names: Vec<String> = tbl.df.get_column_names_str().iter().map(|s| s.to_string()).collect();
        let mut header: Vec<Element<Message, Theme, Renderer>> = vec![];
        for (i, name) in names.iter().enumerate() {
            header.push(text(name.clone())
                .align_x(Alignment::Center)
                .width(tbl.column_widths[i]).into());
        }
        let cont = 
            container(row(header))
            .style(move|theme| {
                if style.is_some() {
                    style.unwrap().header_style
                } else {
                    container::bordered_box(theme)
                }});

        Some(Element::from(cont))
    } else {
        None
    };
    

    let mut rows = vec![];
    for idx in 0..tbl.df.height() {
        let mut tbl_row = vec![];
        if let Ok(row_val) = tbl.df.get_row(idx) {
            let row_items: Vec<String> = row_val.0.iter().map(|item| item.to_string()).collect();
            for (i, item) in row_items.iter().enumerate() {
                if i != 0 {
                    let txt = 
                        text(item.clone())
                            .align_x(Alignment::Center)
                            .width(tbl.column_widths[i]);
                    tbl_row.push(txt.into());
                }
            }
            
            let cont =
                container(row(tbl_row))
                .style(move|theme| {
                    if style.is_some() {
                        style.unwrap().body_style
                    } else {
                        container::bordered_box(theme)
                    }});
            
            rows.push(cont.into());
        }
    }

    let footer = if let Some(footer_values) = &tbl.footer {
        let mut footer = vec![];
        for (i, f) in footer_values.iter().enumerate() {
            let txt = 
                text(f.clone())
                    .align_x(Alignment::Center)
                    .width(tbl.column_widths[i]);
            footer.push(txt.into());
        }
        let cont = 
            container(row(footer))
            .style(move|theme| {
                if style.is_some() {
                    style.unwrap().footer_style
                } else {
                    container::bordered_box(theme)
                }});
        Some(Element::from(cont))
    } else {
        None
    };
    
    let col = if tbl.header_enabled && footer.is_some() {
        column([header.unwrap()]).extend(rows).extend([footer.unwrap()])
    } else if tbl.header_enabled {
        column([header.unwrap()]).extend(rows)
    } else if footer.is_some(){
        column(rows).extend([footer.unwrap()])
    } else {
        column(rows)
    };

    col.into()
    
}

fn get_scrollbar(alignment: Anchor, width: f32, margin: f32, scroller_width: f32) -> Scrollbar {
    Scrollbar::new()
        .anchor(alignment)
        .width(width)
        .margin(margin)
        .scroller_width(scroller_width)
}

pub fn table_callback(state: &mut IpgState, message: Message) {

    match message {
        Message::TableResizing((id, index), offset) => {
            let wci = WidgetCallbackIn{
                id,
                index: Some(index),
                value_f32: Some(offset),
                table_mouse: IpgTableMouse::Resizing,
                ..Default::default()};
            let _ = set_or_get_widget_callback_data(state, wci);
        },
        Message::TableResized(id) => {
            let wci = WidgetCallbackIn{
                id,
                table_mouse: IpgTableMouse::Resized,
                ..Default::default()};
            let _ = set_or_get_widget_callback_data(state, wci);
        },
       _ => ()
    }
}


// pub fn process_callback(wco: WidgetCallbackOut) 
// {
//     let app_cbs = access_callbacks();

//     let callback_present = app_cbs.callbacks.get(&(wco.id, wco.event_name.clone()));

//     let callback_opt = match callback_present {
//         Some(cb) => cb,
//         None => return,
//     };

//     let callback = match callback_opt {
//         Some(cb) => cb,
//         None => panic!("Table callback could not be found with id {}", wco.id),
//     };

//     let table_index: (usize, usize) = match wco.index_table {
//         Some(ti) => ti,
//         None => panic!("Table: Unable to find table index for callback.")
//     };
    
//     Python::with_gil(|py| {
        
//         if wco.user_data.is_some() {
//             let user_data = wco.user_data.unwrap();
//             let res = 
//                 if wco.event_name == "on_button" {
//                     callback.call1(py, (
//                                 wco.id,
//                                 table_index, 
//                                 user_data
//                                 ))
//                     } else if wco.event_name == "on_checkbox" || wco.event_name == "on_toggler" {
//                         callback.call1(py, (
//                             wco.id,
//                             table_index,
//                             wco.on_toggle,  
//                             user_data
//                             ))
//                     } else if wco.event_name == "on_scroll" {
//                         callback.call1(py, (
//                             wco.id,
//                             wco.scroll_pos,  
//                             user_data
//                             ))
//                     } else {
//                         panic!("Table callback: Event name {} could not be found", wco.event_name)
//                     };
                    
//             match res {
//                 Ok(_) => (),
//                 Err(er) => panic!("Table: 4 parameters (id, widget_index, on_toggle, user_data) are required or a python error in this function. {er}"),
//             }
//         } else {
            
//             let res = 
//                 if wco.event_name == "on_button" {
//                     callback.call1(py, (
//                                 wco.id,
//                                 table_index, 
//                                 ))
//                     } else if wco.event_name == "on_checkbox" || wco.event_name == "on_toggler" {
//                         callback.call1(py, (
//                             wco.id,
//                             table_index,
//                             wco.on_toggle,  
//                             ))
//                     } else if wco.event_name == "on_scroll" {
//                         callback.call1(py, (
//                             wco.id,
//                             wco.scroll_pos,  
//                             ))
//                     } else {
//                         panic!("Table callback: Event name {} could not be found", wco.event_name)
//                 };
//             match res {
//                 Ok(_) => (),
//                 Err(er) => panic!("Table: if on_scroll, 2 parameters (id, scroll_pos), else 3 parameter (id, widget_index, on_toggle) are required or possibly a python error in this function. {er}"),
//             }
//         }
//     });
 
//     drop(app_cbs);
         
// }

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgTableParam {
    Title,
    PolarsDf,
    ColumnWidths,
    Height,
    Width,
    HeaderEnabled,
    HeaderCustomEnabled,
    Footer,
    ColumnSpacing,
    RowSpacing,
    DividerWidth,
    ResizeColumnsEnabled,
    MinColumnWidth,
    CellPadding,
    Show,
    TableWidthFixed,
    TableWidth,
    ScrollerWidth,
    ScrollerBarWidth,
    ScrollerMargin,
}

pub fn table_item_update( 
                    table: &mut IpgTable,
                    item: &PyObject,
                    value: &PyObject,
                    ) 
{
    let update = try_extract_table_update(item);
    let name = "Table".to_string();
    match update {
        IpgTableParam::Title => {
            table.title = try_extract_string(value, name);
        },
        IpgTableParam::ColumnWidths => {
            table.column_widths = try_extract_vec_f32(value, name);
        },
        IpgTableParam::Width => {
            let width = Some(try_extract_f64(value, name) as f32);
            table.width = get_width(width, false);
        },
        IpgTableParam::Height => {
            table.height = try_extract_f64(value, name) as f32;
        },
        IpgTableParam::ColumnSpacing => {
            table.column_spacing = try_extract_f64(value, name) as f32;
        },
        IpgTableParam::RowSpacing => {
            table.row_spacing = try_extract_f64(value, name) as f32;
        },
        IpgTableParam::DividerWidth => {
            table.divider_width = try_extract_f64(value, name) as f32;
        },
        IpgTableParam::ResizeColumnsEnabled => {
            table.resize_columns_enabled = try_extract_boolean(value, name);
        },
        IpgTableParam::MinColumnWidth => {
            table.min_column_width = Some(try_extract_f64(value, name) as f32);
        },
        IpgTableParam::CellPadding => {
            table.cell_padding = try_extract_f64(value, name) as f32;
        },
        IpgTableParam::Show => {
            table.show = try_extract_boolean(value, name);
        },
        IpgTableParam::TableWidthFixed => {
            table.table_width_fixed = try_extract_boolean(value, name);
        },
        IpgTableParam::TableWidth => {
            table.table_width = try_extract_f64(value, name) as f32;
        },
        IpgTableParam::ScrollerWidth => {
            table.scroller_width = try_extract_f64(value, name) as f32;
        },
        IpgTableParam::ScrollerBarWidth => {
            table.scroller_bar_width = try_extract_f64(value, name) as f32;
        },
        IpgTableParam::ScrollerMargin => {
            table.scroller_margin = try_extract_f64(value, name) as f32;
        },
        _ => ()
    }
}

pub fn table_dataframe_update( 
                    table: &mut IpgTable,
                    item: &PyObject,
                    value: &PyDataFrame,
                    ) 
{
    let update = try_extract_table_update(item);
    if update == IpgTableParam::PolarsDf {
         let df = Into::<DataFrame>::into(value.clone());
         table.df= df;
     }
}
pub fn try_extract_table_update(update_obj: &PyObject) -> IpgTableParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgTableParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Table update extraction failed"),
        }
    })
}

#[derive(Debug, Clone)]
pub struct IpgTableStyle {
    pub id: usize,
    pub header_style: container::Style,
    pub body_style: container::Style,
    pub footer_style: container::Style,
    pub divider_style: divider::Style,
}

impl IpgTableStyle {
    pub fn new(
        id: usize,
        header_style: container::Style,
        body_style: container::Style,
        footer_style: container::Style,
        divider_style: divider::Style,
    ) -> Self {
        Self {
            id,
            header_style,
            body_style,
            footer_style,
            divider_style
        }
    }
}

fn style_default(id: usize, theme: &Theme) -> IpgTableStyle {
    IpgTableStyle {
        id,
        header_style: container::bordered_box(theme),
        body_style: container::bordered_box(theme),
        footer_style: container::bordered_box(theme),
        divider_style: divider::default(),
    }
}

pub fn get_table_style(style: Option<&IpgWidgets>) -> Option<IpgTableStyle> {
    match style {
        Some(IpgWidgets::IpgTableStyle(style)) => {
            Some(style.clone())
        }
        _ => None,
    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgTableStyleParam {
    HeaderBackgroundIpgColor,
    HeaderBackgroundRgbaColor,
    HeaderBorderIpgColor,
    HeaderBorderRgbaColor,
    HeaderBorderRadius,
    HeaderBorderWidth,
    HeaderTextIpgColor,
    HeaderTextRgbaColor,

    BodyBackgroundIpgColor,
    BodyBackgroundRgbaColor,
    BodyBorderIpgColor,
    BodyBorderRgbaColor,
    BodyBorderRadius,
    BodyBorderWidth,
    BodyTextIpgColor,
    BodyTextRgbaColor,

    FooterBackgroundIpgColor,
    FooterBackgroundRgbaColor,
    FooterBorderIpgColor,
    FooterBorderRgbaColor,
    FooterBorderRadius,
    FooterBorderWidth,
    FooterTextIpgColor,
    FooterTextRgbaColor,
}

pub fn table_style_update_item(_style: &mut IpgTableStyle,
                            _item: &PyObject,
                            _value: &PyObject,) 
{
//     let update = try_extract_table_style_update(item);
//     let name = "TableStyle".to_string();
//     match update {
//         IpgTableStyleParam::HeaderBackgroundIpgColor => {
//             let color = try_extract_ipg_color(value, name);
//             style.header_background_color = get_color(None, Some(color), 1.0, false);
//         },
//         IpgTableStyleParam::BodyBackgroundIpgColor => {
//             let color = try_extract_ipg_color(value, name);
//             style.body_background_color = get_color(None, Some(color), 1.0, false);
//         },
//         IpgTableStyleParam::FooterBackgroundIpgColor => {
//             let color = try_extract_ipg_color(value, name);
//             style.footer_background_color = get_color(None, Some(color), 1.0, false);
//         },
//         IpgTableStyleParam::HeaderBackgroundRgbaColor => {
//             style.header_background_color = Some(Color::from(try_extract_rgba_color(value, name)));
//         },
//         IpgTableStyleParam::BodyBackgroundRgbaColor => {
//             style.body_background_color = Some(Color::from(try_extract_rgba_color(value, name)));
//         },
//         IpgTableStyleParam::FooterBackgroundRgbaColor => {
//             style.footer_background_color = Some(Color::from(try_extract_rgba_color(value, name)));
//         },
//         IpgTableStyleParam::HeaderBorderIpgColor => {
//             let color = try_extract_ipg_color(value, name);
//             style.header_border_color = get_color(None, Some(color), 1.0, false);
//         },
//         IpgTableStyleParam::BodyBorderIpgColor => {
//             let color = try_extract_ipg_color(value, name);
//             style.body_border_color = get_color(None, Some(color), 1.0, false);
//         },
//         IpgTableStyleParam::FooterBorderIpgColor => {
//             let color = try_extract_ipg_color(value, name);
//             style.footer_border_color = get_color(None, Some(color), 1.0, false);
//         },
//         IpgTableStyleParam::HeaderBorderRgbaColor => {
//             style.header_border_color = Some(Color::from(try_extract_rgba_color(value, name)));
//         },
//         IpgTableStyleParam::BodyBorderRgbaColor => {
//             style.body_border_color = Some(Color::from(try_extract_rgba_color(value, name)));
//         },
//         IpgTableStyleParam::FooterBorderRgbaColor => {
//             style.footer_border_color = Some(Color::from(try_extract_rgba_color(value, name)));
//         },
//         IpgTableStyleParam::HeaderBorderRadius => {
//             style.header_border_radius = try_extract_f32(value, name);
//         },
//         IpgTableStyleParam::BodyBorderRadius => {
//             style.body_border_radius = try_extract_f32(value, name);
//         },
//         IpgTableStyleParam::FooterBorderRadius => {
//             style.footer_border_radius = try_extract_f32(value, name);
//         },
//         IpgTableStyleParam::HeaderBorderWidth => {
//             style.header_border_width = try_extract_f32(value, name);
//         },
//         IpgTableStyleParam::BodyBorderWidth => {
//             style.body_border_width = try_extract_f32(value, name);
//         },
//         IpgTableStyleParam::FooterBorderWidth => {
//             style.footer_border_width = try_extract_f32(value, name);
//         },
//         IpgTableStyleParam::HeaderTextIpgColor => {
//             let color = try_extract_ipg_color(value, name);
//             style.header_text_color = get_color(None, Some(color), 1.0, false);
//         },
//         IpgTableStyleParam::BodyTextIpgColor => {
//             let color = try_extract_ipg_color(value, name);
//             style.body_text_color = get_color(None, Some(color), 1.0, false);
//         },
//         IpgTableStyleParam::FooterTextIpgColor => {
//             let color = try_extract_ipg_color(value, name);
//             style.footer_text_color = get_color(None, Some(color), 1.0, false);
//         },
//         IpgTableStyleParam::HeaderTextRgbaColor => {
//             style.header_text_color = Some(Color::from(try_extract_rgba_color(value, name)));
//         },
//         IpgTableStyleParam::BodyTextRgbaColor => {
//             style.body_text_color = Some(Color::from(try_extract_rgba_color(value, name)));
//         },
//         IpgTableStyleParam::FooterTextRgbaColor => {
//             style.footer_text_color = Some(Color::from(try_extract_rgba_color(value, name)));
//         },
//     }

}

pub fn try_extract_table_style_update(update_obj: &PyObject) -> IpgTableStyleParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgTableStyleParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Table style parameter update extraction failed"),
        }
    })
}

