//! ipg_table
#![allow(clippy::unit_arg)]
use crate::app::Message;
use crate::graphics::colors::get_color;
use crate::table;
use crate::IpgState;
use crate::table::table::{body_container, dummy_container, 
    single_row_container};

use iced::advanced::graphics::core::Element;
use iced::widget::scrollable::{Anchor, Scrollbar};
use iced::Color;
use iced::{Length, Padding, Renderer, Theme};
use iced::widget::{column, row, scrollable, text};

use polars::frame::DataFrame;
use pyo3::{pyclass, PyObject, Python};
use pyo3_polars::PyDataFrame;

use super::callbacks::{set_or_get_widget_callback_data, 
    WidgetCallbackIn};
use super::helpers::try_extract_ipg_color;
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
        pub width: Length,
        pub header_enabled: bool,
        pub header_custom_enabled: bool,
        pub footer_enabled: bool,
        pub control_columns: Vec<usize>,
        pub hide_columns: Vec<usize>,
        pub row_highlight: Option<IpgTableRowHighLight>,
        pub highlight_amount: f32,
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
        header_id: scrollable::Id,
        body_id: scrollable::Id,
        footer_id: scrollable::Id,
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
        footer_enabled: bool,
        control_columns: Vec<usize>,
        hide_columns: Vec<usize>,
        row_highlight: Option<IpgTableRowHighLight>,
        highlight_amount: f32,
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
            footer_enabled,
            control_columns,
            hide_columns,
            row_highlight,
            highlight_amount,
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
            header_id: scrollable::Id::unique(),
            body_id: scrollable::Id::unique(),
            footer_id: scrollable::Id::unique(),
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
                            mut content: Vec<Element<'a, Message, Theme, Renderer>>,
                            _style_opt: Option<&IpgWidgets>, 
                            ) 
                            -> Element<'a, Message, Theme, Renderer> {

    let columns = 
        if tbl.header_enabled {
            let df_columns = tbl.df.get_columns().to_vec();
            let mut columns: Vec<Element<'a, Message, Theme, Renderer>> = vec![];
            for col in df_columns.iter() {
                if tbl.header_custom_enabled {
                    columns.push(content.remove(0));
                } else {    
                    columns.push(text(col.name().to_string()).into());        
                }
            }
            columns
        } else {
            let mut columns: Vec<Element<'a, Message, Theme, Renderer>> = vec![];
            for _ in 0..tbl.column_widths.len() {
                columns.push(content.remove(0));
            }
            columns
        };

    // remove the footer from content, if enabled
    let mut footers = vec![];
    if tbl.footer_enabled {
        for _ in 0..tbl.column_widths.len() {
            footers.push(content.remove(0));
        }
    }

    let column_widths: Vec<f32> = tbl.column_widths.iter().map(|width|width+tbl.divider_width).collect();
    let min_column_width = tbl.min_column_width.unwrap_or(0.0);
    let cell_padding = Padding::from(tbl.cell_padding);
    let scrollbar = get_scrollbar(
                                    Anchor::Start, 
                                    tbl.scroller_bar_width, 
                                    tbl.scroller_margin, 
                                    tbl.scroller_width);

    // if table_width_fixed then column resizing doesn't change the table width
    let mut min_width = 0.0;
    if tbl.table_width_fixed {
        min_width = tbl.table_width;
    }

    let header = if tbl.header_enabled {
        Some(add_header(
            tbl.id, 
            tbl.header_id, 
            columns, 
            column_widths.clone(), 
            min_width, 
            tbl.resize_offset.clone(), 
            min_column_width, 
            tbl.divider_width, 
            cell_padding))
    } else {
        None
    };

    let mut rows: Vec<Element<'a, Message, Theme, Renderer>> = vec![];
    for idx in 0..tbl.df.height() {
        let row_items = tbl.df.get_row(idx).unwrap();
        for (idx, item) in row_items.0.iter().enumerate() {
            if tbl.control_columns.contains(&idx) {
                rows.push(content.remove(0));
            } else {
                let item = item.to_string().trim_matches('"').to_string();
                rows.push(text(item).into());
            }
        }
    }
 
    let row_num_vec = vec![0; tbl.df.height()];

    let body: Element<'a, Message, Theme, Renderer> = 
        scrollable(column(row_num_vec.iter().enumerate()
        .map(|(index, _width)| {
            table::style::wrapper::row(
                iced::widget::row(column_widths
                    .iter()
                    .enumerate()
                    .map(|(idx, width)| {
                        body_container(
                            rows.remove(0),
                            *width,
                            tbl.resize_offset[idx],
                            min_column_width,
                            tbl.divider_width,
                            cell_padding,
                            tbl.row_max_height,
                        )
                    })
                    .chain(dummy_container(
                        column_widths.clone(),
                        tbl.resize_offset.clone(), 
                        min_width, 
                        min_column_width))),
                        Default::default(),
                index,
            )   
        })))
        .id(tbl.body_id)
        .on_scroll(move |viewport| {
            let offset = viewport.absolute_offset();
            (Message::TableSyncHeader)(scrollable::AbsoluteOffset { x: offset.x, y: offset.y })
        })
        .direction(scrollable::Direction::Both {
            horizontal: scrollbar,
            vertical: scrollbar,
        })
        .height(Length::Fixed(tbl.height-100.0))
        .into();
    
        let footer = if tbl.footer_enabled {
        Some(add_footer(
            tbl.id, 
            tbl.footer_id, 
            footers, 
            column_widths, 
            min_width, 
            tbl.resize_offset, 
            min_column_width, 
            tbl.divider_width, 
            cell_padding))
    } else {
        None
    };

    let col = if header.is_some() && footer.is_some() {
        column![header.unwrap(), body, footer.unwrap()]
    } else if header.is_none() && footer.is_some() {
        column![body, footer.unwrap()]
    } else if header.is_some() && footer.is_none() {
        column![header.unwrap(), body]
    } else {
        column![body]
    };
    
    if tbl.table_width_fixed {
            col.width(tbl.table_width).into()
        } else {
            col.into()
        }
    
}

fn add_header(id: usize,
                    header_id: scrollable::Id,
                    mut columns: Vec<Element<'_, Message, Theme, Renderer>>, 
                    column_widths: Vec<f32>,
                    min_width: f32,
                    resize_offset: Vec<Option<f32>>,
                    min_column_width: f32,
                    divider_width: f32,
                    cell_padding: Padding) 
                -> Element<'_, Message, Theme, Renderer> {

    scrollable(table::style::wrapper::header(
        row(column_widths
            .iter()
            .enumerate()
            .map(|(index, column_width)| {
                single_row_container(
                    id,
                    index,
                    columns.remove(0),
                    *column_width,
                    resize_offset[index],
                    Some(Message::TableResizing),
                    Some(Message::TableResized(id)),
                    min_column_width,
                    divider_width,
                    cell_padding,
                    Default::default(),
                )
            })
            .chain(dummy_container(column_widths.clone(),
                                    resize_offset.clone(),
                                    min_width, 
                                    min_column_width))),
            Default::default(),
    ))
    .id(header_id)
    .direction(scrollable::Direction::Both {
        vertical: scrollable::Scrollbar::new()
            .width(0)
            .margin(0)
            .scroller_width(0),
        horizontal: scrollable::Scrollbar::new()
            .width(0)
            .margin(0)
            .scroller_width(0),
    })
    .into()
}

fn add_footer(
            id: usize,
            footer_id: scrollable::Id,
            mut footers: Vec<Element<'_, Message, Theme, Renderer>>, 
            column_widths: Vec<f32>,
            min_width: f32,
            resize_offset: Vec<Option<f32>>,
            min_column_width: f32,
            divider_width: f32,
            cell_padding: Padding) -> Element<'_, Message, Theme, Renderer>{
    
    scrollable(table::style::wrapper::footer(
        row(column_widths
            .iter()
            .enumerate()
            .map(|(index, column_width)| {
                single_row_container(
                    id,
                    index,
                    footers.remove(0),
                    *column_width,
                    resize_offset[index],
                    Some(Message::TableResizing),
                    Some(Message::TableResized(id)),
                    min_column_width,
                    divider_width,
                    cell_padding,
                    Default::default(),
                )
            })
            .chain(dummy_container(column_widths.clone(),
                                    resize_offset.clone(),
                                    min_width, 
                                    min_column_width))),
            Default::default(),
    ))
    .id(footer_id)
    .direction(scrollable::Direction::Both {
        vertical: scrollable::Scrollbar::new()
            .width(0)
            .margin(0)
            .scroller_width(0),
        horizontal: scrollable::Scrollbar::new()
            .width(0)
            .margin(0)
            .scroller_width(0),
    })
    .into()
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
    FooterEnabled,
    RowHighlight,
    HighlightAmount,
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
        IpgTableParam::RowHighlight => {
            table.row_highlight = Some(try_extract_row_highlight(value));
        },
        IpgTableParam::HighlightAmount => {
            table.highlight_amount = try_extract_f64(value, name) as f32;
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

fn try_extract_row_highlight(value: &PyObject) -> IpgTableRowHighLight {
    Python::with_gil(|py| {
        let res = value.extract::<IpgTableRowHighLight>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Table update extraction of IpgTableRowHighLight failed"),
        }
    })
}

#[derive(Debug, Clone)]
pub struct IpgTableStyle {
    pub id: usize,
    pub header_background_color: Option<Color>,
    pub header_border_color: Option<Color>,
    pub header_border_radius: Vec<f32>,
    pub header_border_width: f32,
    pub header_text_color: Option<Color>,
    pub body_background_color: Option<Color>,
    pub body_border_color: Option<Color>,
    pub body_border_radius: Vec<f32>,
    pub body_border_width: f32,
    pub body_text_color: Option<Color>,
    pub footer_background_color: Option<Color>,
    pub footer_border_color: Option<Color>,
    pub footer_border_radius: Vec<f32>,
    pub footer_border_width: f32,
    pub footer_text_color: Option<Color>,
    pub divider_hover_color: Option<Color>,
    pub divider_unhover_color: Option<Color>,
}

impl IpgTableStyle {
    pub fn new(
        id: usize,
        header_background_color: Option<Color>,
        header_border_color: Option<Color>,
        header_border_radius: Vec<f32>,
        header_border_width: f32,
        header_text_color: Option<Color>,
        body_background_color: Option<Color>,
        body_border_color: Option<Color>,
        body_border_radius: Vec<f32>,
        body_border_width: f32,
        body_text_color: Option<Color>,
        footer_background_color: Option<Color>,
        footer_border_color: Option<Color>,
        footer_border_radius: Vec<f32>,
        footer_border_width: f32,
        footer_text_color: Option<Color>,
        divider_hover_color: Option<Color>,
        divider_unhover_color: Option<Color>,
    ) -> Self {
        Self {
            id,
            header_background_color,
            header_border_color,
            header_border_radius,
            header_border_width,
            header_text_color,
            body_background_color,
            body_border_color,
            body_border_radius,
            body_border_width,
            body_text_color,
            footer_background_color,
            footer_border_color,
            footer_border_radius,
            footer_border_width,
            footer_text_color,
            divider_hover_color,
            divider_unhover_color,
        }
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

pub fn table_style_update_item(style: &mut IpgTableStyle,
                            item: &PyObject,
                            value: &PyObject,) 
{
    let update = try_extract_table_style_update(item);
    let name = "TableStyle".to_string();
    match update {
        IpgTableStyleParam::HeaderBackgroundIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.header_background_color = get_color(None, Some(color), 1.0, false);
        },
        IpgTableStyleParam::BodyBackgroundIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.body_background_color = get_color(None, Some(color), 1.0, false);
        },
        IpgTableStyleParam::FooterBackgroundIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.footer_background_color = get_color(None, Some(color), 1.0, false);
        },
        IpgTableStyleParam::HeaderBackgroundRgbaColor => {
            style.header_background_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgTableStyleParam::BodyBackgroundRgbaColor => {
            style.body_background_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgTableStyleParam::FooterBackgroundRgbaColor => {
            style.footer_background_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgTableStyleParam::HeaderBorderIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.header_border_color = get_color(None, Some(color), 1.0, false);
        },
        IpgTableStyleParam::BodyBorderIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.body_border_color = get_color(None, Some(color), 1.0, false);
        },
        IpgTableStyleParam::FooterBorderIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.footer_border_color = get_color(None, Some(color), 1.0, false);
        },
        IpgTableStyleParam::HeaderBorderRgbaColor => {
            style.header_border_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgTableStyleParam::BodyBorderRgbaColor => {
            style.body_border_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgTableStyleParam::FooterBorderRgbaColor => {
            style.footer_border_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgTableStyleParam::HeaderBorderRadius => {
            style.header_border_radius = try_extract_vec_f32(value, name);
        },
        IpgTableStyleParam::BodyBorderRadius => {
            style.body_border_radius = try_extract_vec_f32(value, name);
        },
        IpgTableStyleParam::FooterBorderRadius => {
            style.footer_border_radius = try_extract_vec_f32(value, name);
        },
        IpgTableStyleParam::HeaderBorderWidth => {
            style.header_border_width = try_extract_f64(value, name) as f32;
        },
        IpgTableStyleParam::BodyBorderWidth => {
            style.body_border_width = try_extract_f64(value, name) as f32;
        },
        IpgTableStyleParam::FooterBorderWidth => {
            style.footer_border_width = try_extract_f64(value, name) as f32;
        },
        IpgTableStyleParam::HeaderTextIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.header_text_color = get_color(None, Some(color), 1.0, false);
        },
        IpgTableStyleParam::BodyTextIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.body_text_color = get_color(None, Some(color), 1.0, false);
        },
        IpgTableStyleParam::FooterTextIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.footer_text_color = get_color(None, Some(color), 1.0, false);
        },
        IpgTableStyleParam::HeaderTextRgbaColor => {
            style.header_text_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgTableStyleParam::BodyTextRgbaColor => {
            style.body_text_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgTableStyleParam::FooterTextRgbaColor => {
            style.footer_text_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
    }

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

