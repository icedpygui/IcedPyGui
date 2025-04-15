//! ipg_table
#![allow(clippy::unit_arg)]

use crate::app::Message;
use crate::{access_callbacks, access_user_data1, IpgState};

use iced::widget::scrollable::Scrollbar;
use iced::{alignment, Border};
use iced::Length::Fill;
use iced::{Element, Length, Renderer, Theme};
use iced::widget::{column, container, row, scrollable, stack, text};

use polars::frame::DataFrame;
use pyo3::{pyclass, PyObject, Python};
use pyo3_polars::PyDataFrame;

use super::callbacks::{set_or_get_widget_callback_data, 
    WidgetCallbackIn};
use super::divider::{self, divider_horizontal};
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
        pub text_size: f32,
        pub show: bool,
        pub resize_offset: Vec<Option<f32>>,
        pub table_width_fixed: bool,
        pub table_width: f32,
        pub scroller_width: f32,
        pub scroller_bar_width: f32,
        pub scroller_margin: f32,
        pub style_id: Option<usize>,
        pub header_scroller_id: scrollable::Id,
        pub body_scroller_id: scrollable::Id,
        pub footer_scroller_id: scrollable::Id,
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
        text_size: f32,
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
            text_size,
            show,
            resize_offset,
            table_width_fixed,
            table_width,
            scroller_width,
            scroller_bar_width,
            scroller_margin,
            style_id,
            header_scroller_id: scrollable::Id::unique(),
            body_scroller_id: scrollable::Id::unique(),
            footer_scroller_id: scrollable::Id::unique(),
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
                            _content: Vec<Element<'a, Message, Theme, Renderer>>,
                            style_opt: Option<&IpgWidgets>, 
                            ) 
                            -> Element<'a, Message, Theme, Renderer> {

    let _style = get_table_style(style_opt);
    

    let mut body_rows = vec![];
        for idx in 0..tbl.df.height() {
            if let Ok(df_row) = tbl.df.get_row(idx) {
                let mut rw = vec![];
                for (i, item) in df_row.0.iter().enumerate() {
                    let txt = 
                        text(item.to_string())
                            .size(tbl.text_size)
                            .align_x(alignment::Horizontal::Center)
                            .align_y(alignment::Vertical::Center)
                            .width(tbl.column_widths[i]);
                    rw.push(Element::from(txt));
                }
            body_rows.push(container(row(rw))
                            .style(move|theme|bordered_box(theme, idx))
                            .into());
            }
        }
        let body: Element<Message> = scrollable(column(body_rows))
                                        .height(400.0)
                                        .on_scroll(move|vp|Message::TableSync(vp.absolute_offset()))
                                        .direction({
                                            let scrollbar = Scrollbar::new();
                                            scrollable::Direction::Both {
                                                horizontal: scrollbar,
                                                vertical: scrollbar,
                                            }
                                        })
                                        .id(tbl.body_scroller_id.clone())
                                        .into();

        let header = if tbl.header_enabled {
            let column_names = tbl.df.get_column_names_owned();
            let header = column_names.iter().map(|s| s.to_string());
            let mut rw = vec![];
            for (i, hd) in header.into_iter().enumerate() {
                    let txt = 
                    text(hd)
                    .size(tbl.text_size)
                    .align_x(alignment::Horizontal::Center)
                    .align_y(alignment::Vertical::Center)
                    .width(Fill)
                    .height(Fill);
                rw.push(Element::from(
                    container(txt)
                        .width(tbl.column_widths[i])
                        .height(25.0)
                        .style(move|theme|container::bordered_box(theme))
                        ));
            }
            Some(Element::from(scrollable(row(rw))
                                .direction({
                                    let scrollbar = scrollable::Scrollbar::new();
                                    scrollable::Direction::Horizontal(scrollbar)
                                    })
                                .id(tbl.header_scroller_id.clone())
                                ))
        } else {
            None
        };

        let div = 
            divider_horizontal(
                tbl.id,
                tbl.column_widths.clone(),
                4.0,
                25.0,
                Message::TableDivider,
            );

        let mut col = vec![];
        if header.is_some() {
            let stk = stack([header.unwrap(), div.into()]).into();
            col.push(stk);
        }
        col.push(body.into());
        
        let main_col = column(col).spacing(5.0);
    
        container(container(main_col))
            .width(Fill)
            .height(Fill)
            .padding(20)
            .center_x(Fill)
            .center_y(Fill)
            .into()
    
}

#[derive(Clone, Debug, PartialEq)]
pub enum TableMessage {
    ColumnResizing((usize, f32)),
    DivOnRelease,
}

pub fn table_callback(
        state: &mut IpgState,  
        id: usize,  
        message: TableMessage) {

    let mut wci: WidgetCallbackIn = WidgetCallbackIn{id, ..Default::default()};
           
    match message {
        TableMessage::ColumnResizing((index, value)) => {
            process_callback(
                id, 
                "on_column_resize".to_string(), 
                index, 
                value);
        },
        TableMessage::DivOnRelease => {
            // to be consistent, returning values for both
            wci.value_str = Some("on_release".to_string());
            let wco = set_or_get_widget_callback_data(state, wci);
            process_callback(
                id, 
                "on_release".to_string(), 
                wco.value_usize.unwrap(), 
                wco.value_f32.unwrap());
        },
    }
}

pub fn process_callback(id: usize, event_name: String, index: usize, value: f32) 
{
    let ud = access_user_data1();
    let user_data_opt = ud.user_data.get(&id);

    let app_cbs = access_callbacks();

    let callback_present = 
        app_cbs.callbacks.get(&(id, event_name));
    
    let callback = match callback_present {
        Some(cb) => cb,
        None => return,
    };

    let cb = 
        Python::with_gil(|py| {
            callback.clone_ref(py)
        });

    drop(app_cbs);
                 
    Python::with_gil(|py| {
        if user_data_opt.is_some() {
            
            let res = cb.call1(py, (
                                                        id,
                                                        index, 
                                                        value, 
                                                        user_data_opt,
                                                        ));
            match res {
                Ok(_) => (),
                Err(er) => panic!("Table Divider: 4 parameters (id, value, user_data) 
                                    are required or a python error in this function. {er}"),
            }
        } else {
            let res = cb.call1(py, (
                                                        id,
                                                        index, 
                                                        value, 
                                                        ));
            match res {
                Ok(_) => (),
                Err(er) => panic!("Table Divider: 3 parameters (id, value) 
                                    are required or a python error in this function. {er}"),
            }
        }
    });

    drop(ud); 

}

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

fn bordered_box(theme: &Theme, index: usize) -> container::Style {
    let palette = theme.extended_palette();
    let background = if index%2 == 0 {
        Some(palette.background.strong.color.into())
    } else {
        Some(palette.background.weak.color.into())
    };
    container::Style {
        background,
        border: Border {
            width: 1.0,
            radius: 0.0.into(),
            color: palette.background.base.color,
        },
        ..container::Style::default()
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

