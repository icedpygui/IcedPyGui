//! ipg_table

use crate::app::Message;
use crate::table;
use crate::IpgState;
use crate::table::table::{body_container, dummy_container, 
    single_row_container};

use iced::advanced::graphics::core::Element;
use iced::widget::scrollable::{Anchor, Scrollbar};
use iced::{Length, Padding, Renderer, Theme};
use iced::widget::{column, row, scrollable, text};

use polars::frame::DataFrame;
use pyo3::{pyclass, PyObject, Python};
use pyo3_polars::PyDataFrame;

use super::callbacks::{set_or_get_widget_callback_data, 
    WidgetCallbackIn};
use super::helpers::{get_width, try_extract_boolean, 
    try_extract_f64, try_extract_string, try_extract_vec_f32};


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
        pub add_data_row_wise: bool,
        pub add_data_column_wise: bool,
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
        header_id: scrollable::Id,
        body_id: scrollable::Id,
        footer_id: Option<scrollable::Id>,
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
        add_data_row_wise: bool,
        add_data_column_wise: bool,
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
            add_data_row_wise,
            add_data_column_wise,
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
            header_id: scrollable::Id::unique(),
            body_id: scrollable::Id::unique(),
            footer_id: Some(scrollable::Id::unique()),
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
                            ) 
                            -> Element<'a, Message, Theme, Renderer> {
    dbg!(&tbl.df.height());
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
            footers.insert(0, content.remove(content.len()-1))
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
                rows.push(text(item.to_string()).into());
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
            .into()
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
        .height(Length::Fixed(tbl.height))
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

fn add_header <'a>(id: usize,
                    header_id: scrollable::Id,
                    mut columns: Vec<Element<'a, Message, Theme, Renderer>>, 
                    column_widths: Vec<f32>,
                    min_width: f32,
                    resize_offset: Vec<Option<f32>>,
                    min_column_width: f32,
                    divider_width: f32,
                    cell_padding: Padding) 
                -> Element<'a, Message, Theme, Renderer> {

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

fn add_footer <'a>(
            id: usize,
            footer_id: Option<scrollable::Id>,
            mut footers: Vec<Element<'a, Message, Theme, Renderer>>, 
            column_widths: Vec<f32>,
            min_width: f32,
            resize_offset: Vec<Option<f32>>,
            min_column_width: f32,
            divider_width: f32,
            cell_padding: Padding) -> Element<'a, Message, Theme, Renderer>{
    
    footer_id.map(|footer| {
        scrollable(table::style::wrapper::footer(
            row(column_widths
                .iter()
                .enumerate()
                .map(|(index, width)| {
                    single_row_container(
                        id,
                        index,
                        footers.remove(0),
                        *width,
                        resize_offset[index],
                        Some(Message::TableResizing),
                        Some(Message::TableResized(id)),
                        min_column_width,
                        divider_width,
                        cell_padding,
                        Default::default(),
                    )
                })
                .chain(dummy_container(
                                column_widths.clone(),
                                resize_offset.clone(), 
                                min_width, 
                                min_column_width))),
                Default::default(),
        ))
        .id(footer)
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
    }).unwrap().into()
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
                value_float_32: Some(offset),
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
    match update {
        IpgTableParam::PolarsDf => {
            let df = Into::<DataFrame>::into(value.clone());
            table.df= df;
        },
        _ => ()
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


