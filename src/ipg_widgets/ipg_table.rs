//! ipg_table

use crate::app::Message;
use crate::table;
use crate::{access_callbacks, IpgState};
use crate::table::table::{body_container, dummy_container, footer_container, header_container};

use iced::advanced::graphics::core::Element;
use iced::widget::scrollable::{Anchor, Scrollbar};
use iced::{Length, Padding, Renderer, Theme};
use iced::widget::{column, row, scrollable};

use pyo3::{pyclass, PyObject, Python};
use pyo3::types::IntoPyDict;

use super::callbacks::{set_or_get_widget_callback_data, 
    WidgetCallbackIn, WidgetCallbackOut};
use super::helpers::{get_width, try_extract_boolean, 
    try_extract_f64, try_extract_string, try_extract_vec_f32};


#[derive(Debug, Clone)]
pub struct IpgTable {
        pub id: usize,
        pub title: String,
        pub column_widths: Vec<f32>,
        pub height: f32,
        pub width: Length,
        pub header_enabled: bool,
        pub control_row_enabled: bool,
        pub footer_enabled: bool,
        pub add_data_row_wise: bool,
        pub add_date_column_wise: bool,
        pub row_highlight: Option<IpgTableRowHighLight>,
        pub highlight_amount: f32,
        pub column_spacing: f32,
        pub row_spacing: f32,
        pub divider_width: f32,
        pub resize_columns_enabled: bool,
        pub min_column_width: Option<f32>,
        pub cell_padding: f32,
        pub show: bool,
        pub resize_offset: Vec<Option<f32>>,
        pub user_data: Option<PyObject>,
        header_id: scrollable::Id,
        body_id: scrollable::Id,
        footer_id: Option<scrollable::Id>,
}

impl IpgTable {
    pub fn new( 
        id: usize,
        title: String,
        column_widths: Vec<f32>,
        height: f32,
        width: Length,
        header_enabled: bool,
        control_row_enabled: bool,
        footer_enabled: bool,
        add_data_row_wise: bool,
        add_date_column_wise: bool,
        row_highlight: Option<IpgTableRowHighLight>,
        highlight_amount: f32,
        column_spacing: f32,
        row_spacing: f32,
        divider_width: f32,
        resize_columns_enabled: bool,
        min_column_width: Option<f32>,
        cell_padding: f32,
        show: bool,
        resize_offset: Vec<Option<f32>>,
        user_data: Option<PyObject>,
        ) -> Self {
        Self {
            id,
            title,
            column_widths,
            height,
            width,
            header_enabled,
            control_row_enabled,
            footer_enabled,
            add_data_row_wise,
            add_date_column_wise,
            row_highlight,
            highlight_amount,
            column_spacing,
            row_spacing,
            divider_width,
            resize_columns_enabled,
            min_column_width,
            cell_padding,
            show,
            resize_offset,
            user_data,
            header_id: scrollable::Id::unique(),
            body_id: scrollable::Id::unique(),
            footer_id: Some(scrollable::Id::unique()),
        }
    }
}

// #[derive(Debug, Clone)]
// pub enum TableMessage {
//     Scrolled(Viewport, usize),
//     SyncHeader(scrollable::AbsoluteOffset),
//     Resizing(usize, f32),
//     Resized,
// }

#[derive(Debug, Clone, Copy)]
#[pyclass]
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

    let num_of_columns = tbl.column_widths.len();

    // remove the headers from the content
    let mut columns  = vec![];

    for _ in 0..num_of_columns {
        columns.push(content.remove(0));
    }

    // remove the footer from content, if enabled
    let mut footers = vec![];
    if tbl.footer_enabled {
        for _ in 0..num_of_columns {
            footers.push(content.remove(content.len()-1))
        }
    }

    let column_widths: Vec<f32> = tbl.column_widths.iter().map(|width|width+tbl.divider_width).collect();
    let min_column_width = tbl.min_column_width.unwrap_or(0.0);
    let cell_padding = Padding::from(tbl.cell_padding);
    let scrollbar = get_scrollbar(Anchor::Start, 5.0, 0.0, 10.0);

    // TODO: need better understanding the effect of min_width
    let min_width = 0.0;

    let header: Element<'a, Message, Theme, Renderer> = 
        scrollable(table::style::wrapper::header(
            row(column_widths
                .iter()
                .enumerate()
                .map(|(index, column_width)| {
                    header_container(
                        index,
                        columns.remove(0),
                        *column_width,
                        tbl.resize_offset[index],
                        Some(Message::TableResizing),
                        Some(Message::TableResized),
                        min_column_width,
                        tbl.divider_width,
                        cell_padding,
                        Default::default(),
                    )
                })
                .chain(dummy_container(column_widths.clone(),
                                        tbl.resize_offset.clone(),
                                        min_width, 
                                        min_column_width))),
                Default::default(),
        ))
        .id(tbl.header_id)
        .direction(scrollable::Direction::Both {
            vertical: scrollable::Scrollbar::new()
                .width(0)
                .margin(0)
                .scroller_width(0),
            horizontal: scrollable::Scrollbar::new()
                .width(0)
                .margin(0)
                .scroller_width(0),
        }).into();
    
    let rows = vec![0; content.len()/tbl.column_widths.len()];
    let body: Element<'a, Message, Theme, Renderer> = 
        scrollable(column(rows.iter().enumerate()
        .map(|(index, _width)| {
            table::style::wrapper::row(
                iced::widget::row(column_widths
                    .iter()
                    .enumerate()
                    .map(|(idx, width)| {
                        body_container(
                            content.remove(0),
                            *width,
                            tbl.resize_offset[idx],
                            min_column_width,
                            tbl.divider_width,
                            cell_padding,
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
            (Message::TableSyncHeader)(scrollable::AbsoluteOffset { y: 0.0, ..offset })
        })
        .direction(scrollable::Direction::Both {
            horizontal: scrollbar,
            vertical: scrollbar,
        })
        .height(Length::Fixed(300.0))
        .into();
    
    if tbl.footer_enabled {
        let footer: Element<'a, Message, Theme, Renderer> = 
            tbl.footer_id.map(|footer| {
            scrollable(table::style::wrapper::footer(
                row(column_widths
                    .iter()
                    .enumerate()
                    .map(|(index, width)| {
                        footer_container(
                            index,
                            footers.remove(0),
                            *width,
                            tbl.resize_offset[index],
                            Some(Message::TableResizing),
                            Some(Message::TableResized),
                            min_column_width,
                            tbl.divider_width,
                            cell_padding,
                            Default::default(),
                        )
                    })
                    .chain(dummy_container(
                                    tbl.column_widths,
                                    tbl.resize_offset.clone(), 
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
        }).unwrap().into();
        column![header, body, footer].into()
    } else {
        column![header, body].into()
    }

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
        Message::TableResizing(index, offset) => {
            let wci = WidgetCallbackIn{id: 6,
                            index: Some(index),
                            value_float_32: Some(offset),
                            table_mouse: IpgTableMouse::Resizing,
                            ..Default::default()};
            let _ = set_or_get_widget_callback_data(state, wci);
        },
        Message::TableResized => {
            let wci = WidgetCallbackIn{
                id: 6,
                table_mouse: IpgTableMouse::Resized,
                ..Default::default()};
            let _ = set_or_get_widget_callback_data(state, wci);
        },
       _ => ()
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
        None => panic!("Table callback could not be found with id {}", wco.id),
    };

    let table_index: (usize, usize) = match wco.index_table {
        Some(ti) => ti,
        None => panic!("Table: Unable to find table index for callback.")
    };
    
    Python::with_gil(|py| {
        
        if wco.user_data.is_some() {
            let user_data = wco.user_data.unwrap();
            let res = 
                if wco.event_name == "on_button" {
                    callback.call1(py, (
                                wco.id,
                                table_index, 
                                user_data
                                ))
                    } else if wco.event_name == "on_checkbox" || wco.event_name == "on_toggler" {
                        callback.call1(py, (
                            wco.id,
                            table_index,
                            wco.on_toggle,  
                            user_data
                            ))
                    } else if wco.event_name == "on_scroll" {
                        callback.call1(py, (
                            wco.id,
                            wco.scroll_pos.into_py_dict_bound(py),  
                            user_data
                            ))
                    } else {
                        panic!("Table callback: Event name {} could not be found", wco.event_name)
                    };
                    
            match res {
                Ok(_) => (),
                Err(er) => panic!("Table: 4 parameters (id, widget_index, on_toggle, user_data) are required or a python error in this function. {er}"),
            }
        } else {
            
            let res = 
                if wco.event_name == "on_button" {
                    callback.call1(py, (
                                wco.id,
                                table_index, 
                                ))
                    } else if wco.event_name == "on_checkbox" || wco.event_name == "on_toggler" {
                        callback.call1(py, (
                            wco.id,
                            table_index,
                            wco.on_toggle,  
                            ))
                    } else if wco.event_name == "on_scroll" {
                        callback.call1(py, (
                            wco.id,
                            wco.scroll_pos.into_py_dict_bound(py),  
                            ))
                    } else {
                        panic!("Table callback: Event name {} could not be found", wco.event_name)
                };
            match res {
                Ok(_) => (),
                Err(er) => panic!("Table: if on_scroll, 2 parameters (id, scroll_pos), else 3 parameter (id, widget_index, on_toggle) are required or possibly a python error in this function. {er}"),
            }
        }
    });
 
    drop(app_cbs);
         
}

#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgTableParam {
    Title,
    Width,
    Height,
    RowHighlight,
    HighlightAmount,
    ColumnWidths,
    Show,
}

pub fn table_item_update( 
                    table: &mut IpgTable,
                    item: PyObject,
                    value: PyObject,
                    ) 
{
    
    let update = try_extract_table_update(item);
    
    match update {
        IpgTableParam::Title => {
            table.title = try_extract_string(value);
        },
        IpgTableParam::Width => {
            let width = Some(try_extract_f64(value) as f32);
            table.width = get_width(width, false);
        },
        IpgTableParam::Height => {
            table.height = try_extract_f64(value) as f32;
        },
        IpgTableParam::RowHighlight => {
            table.row_highlight = Some(try_extract_row_highlight(value));
        },
        IpgTableParam::HighlightAmount => {
            table.highlight_amount = try_extract_f64(value) as f32;
        },
        IpgTableParam::ColumnWidths => {
            table.column_widths = try_extract_vec_f32(value);
        },
        IpgTableParam::Show => {
            table.show = try_extract_boolean(value);
        },
    }
}

pub fn try_extract_table_update(update_obj: PyObject) -> IpgTableParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgTableParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Table update extraction failed"),
        }
    })
}

fn try_extract_row_highlight(value: PyObject) -> IpgTableRowHighLight {
    Python::with_gil(|py| {
        let res = value.extract::<IpgTableRowHighLight>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Table update extraction of IpgTableRowHighLight failed"),
        }
    })
}
