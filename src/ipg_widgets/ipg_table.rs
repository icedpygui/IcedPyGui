//! ipg_table
#![allow(clippy::unit_arg)]

use crate::app::Message;
use crate::{access_callbacks, access_user_data1, IpgState};

use iced::widget::scrollable::Scrollbar;
use iced::{alignment, Border};
use iced::Length::Fill;
use iced::{Element, Length, Renderer, Theme};
use iced::widget::{column, container, Space, row, scrollable, stack, text};

use polars::frame::DataFrame;
use pyo3::{pyclass, PyObject, Python};
use pyo3_polars::PyDataFrame;

use super::callbacks::{set_or_get_widget_callback_data, WidgetCallbackIn};
use super::divider::{self, divider_horizontal};
use super::helpers::{get_width, try_extract_boolean, try_extract_f32, try_extract_f64, try_extract_usize, try_extract_vec_f32, try_extract_vec_usize};
use super::ipg_enums::IpgWidgets;


#[derive(Debug, Clone)]
pub struct IpgTable {
        pub id: usize,
        pub df: DataFrame,
        pub column_widths: Vec<f32>,
        pub height: f32,
        // above required
        pub width: Length,
        pub resizer_width: f32,
        pub header_enabled: bool,
        pub header_row_height: f32,
        pub header_scrollbar_height: f32,
        pub header_scrollbar_margin: f32,
        pub header_scroller_height: f32,
        pub header_scrollbar_spacing: f32,
        pub header_row_spacing: f32,
        pub footer_height: f32,
        pub footer_scrollbar_height: f32,
        pub footer_scrollbar_margin: f32,
        pub footer_scroller_height: f32,
        pub footer_scrollbar_spacing: f32,
        pub footer_spacing: f32,
        pub body_scrollbar_width: f32,
        pub body_scrollbar_margin: f32,
        pub body_scroller_width: f32,
        pub body_scrollbar_spacing: f32,
        pub custom_header_rows: usize,
        pub custom_footer_rows: usize,
        pub control_columns: Vec<usize>,
        pub column_proportional_resize: bool,
        pub row_spacing: f32,
        pub row_height: f32,
        pub header_body_spacing: f32,
        pub body_footer_spacing: f32,
        pub resize_columns_enabled: bool,
        pub min_column_width: Option<f32>,
        pub text_size: f32,
        pub show: bool,
        pub table_width_fixed: bool,
        pub style_id: Option<usize>,
        pub released: bool,
        pub header_scroller_id: scrollable::Id,
        pub body_scroller_id: scrollable::Id,
        pub footer_scroller_id: scrollable::Id,
        
        
        
}

impl IpgTable {
    pub fn new( 
        id: usize,
        df: DataFrame,
        column_widths: Vec<f32>,
        height: f32,
        width: Length,
        resizer_width: f32,
        header_enabled: bool,
        header_row_height: f32,
        header_scrollbar_height: f32,
        header_scrollbar_margin: f32,
        header_scroller_height: f32,
        header_scrollbar_spacing: f32,
        header_row_spacing: f32,
        footer_height: f32,
        footer_scrollbar_height: f32,
        footer_scrollbar_margin: f32,
        footer_scroller_height: f32,
        footer_scrollbar_spacing: f32,
        footer_spacing: f32,
        body_scrollbar_width: f32,
        body_scrollbar_margin: f32,
        body_scroller_width: f32,
        body_scrollbar_spacing: f32,
        custom_header_rows: usize,
        custom_footer_rows: usize,
        control_columns: Vec<usize>,
        column_proportional_resize: bool,
        row_spacing: f32,
        row_height: f32,
        header_body_spacing: f32,
        body_footer_spacing: f32,
        resize_columns_enabled: bool,
        min_column_width: Option<f32>,
        text_size: f32,
        show: bool,
        table_width_fixed: bool,
        style_id: Option<usize>,
        released: bool,
        ) -> Self {
        Self {
            id,
            df,
            column_widths,
            height,
            width,
            resizer_width,
            header_enabled,
            header_row_height,
            header_scrollbar_height,
            header_scrollbar_margin,
            header_scroller_height,
            header_scrollbar_spacing,
            header_row_spacing,
            footer_height,
            footer_scrollbar_height,
            footer_scrollbar_margin,
            footer_scroller_height,
            footer_scrollbar_spacing,
            footer_spacing,
            body_scrollbar_width,
            body_scrollbar_margin,
            body_scroller_width,
            body_scrollbar_spacing,
            custom_header_rows,
            custom_footer_rows,
            control_columns,
            column_proportional_resize,
            row_spacing,
            row_height,
            header_body_spacing,
            body_footer_spacing,
            resize_columns_enabled,
            min_column_width,
            text_size,
            show,
            table_width_fixed,
            style_id,
            released,
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

pub fn construct_table<'a>(tbl: IpgTable, 
                            mut content: Vec<Element<'a, Message, Theme, Renderer>>,
                            style_opt: Option<&IpgWidgets>, 
                            ) 
                            -> Element<'a, Message, Theme, Renderer> {

    let _style = get_table_style(style_opt);

    // let body_row_height = tbl.row_max_height.map_or(tbl.row_height, |max_height| {
    //     if tbl.row_height > max_height {
    //         max_height
    //     } else {
    //         tbl.row_height
    //     }
    // });
    
    let mut body_rows = vec![];
        for idx in 0..tbl.df.height() {
            if let Ok(df_row) = tbl.df.get_row(idx) {
                let mut rw = vec![];
                for (i, item) in df_row.0.iter().enumerate() {
                    if !tbl.control_columns.contains(&i) {
                        let txt = 
                            text(item.to_string())
                                .size(tbl.text_size)
                                .align_x(alignment::Horizontal::Center)
                                .align_y(alignment::Vertical::Center)
                                .width(tbl.column_widths[i]);
                        rw.push(Element::from(txt));
                    } else {
                        rw.push(Element::from(container(content.remove(0))
                                                .width(tbl.column_widths[i])
                                                .center_x(tbl.column_widths[i])));
                    }
                }
            body_rows.push(container(row(rw)
                                                .height(Fill))
                            .height(tbl.row_height)
                            .style(move|theme|bordered_box(theme, idx))
                            .into());
            }
        }

        let body_column = column(body_rows)
                                                .spacing(tbl.row_spacing);

        let body: Element<Message> = scrollable(body_column)
                                        .height(tbl.height)
                                        .width(tbl.width)
                                        .id(tbl.body_scroller_id)
                                        .on_scroll(move|vp|Message::TableSync(
                                                        vp.absolute_offset(), tbl.id))
                                        .direction({
                                            let scrollbar = Scrollbar::new()
                                                .scroller_width(tbl.body_scroller_width)
                                                .width(tbl.body_scrollbar_width)
                                                .margin(tbl.body_scrollbar_margin);
                                            scrollable::Direction::Both {
                                                horizontal: scrollbar,
                                                vertical: scrollbar,
                                            }
                                        })
                                        
                                        .into();
        
        let header_height = if tbl.header_enabled {
            tbl.header_row_height
        } else {
            0.0
        };

        let custom_header_height = if tbl.custom_header_rows > 0 {
            tbl.header_row_height
        } else {
            0.0
        };

        let mut header_column = vec![];

        // add the header if enabled
        if tbl.header_enabled {
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
                        .height(header_height)
                        .style(move|theme|container::bordered_box(theme))
                        ));
            }
            header_column.push(Element::from(row(rw)));
        }
               
        // add any custom header rows
        if tbl.custom_header_rows > 0 {
            for _ in 0..tbl.custom_header_rows {
                let mut custom_rw = vec![];
                for i in 0..tbl.df.width() {
                    custom_rw.push(Element::from(
                        container(content.remove(0))
                            .width(tbl.column_widths[i])
                            .height(custom_header_height)
                            .center_x(tbl.column_widths[i])
                            .style(move|theme|container::bordered_box(theme))
                            ));
                }
                header_column.push(Element::from(row(custom_rw)));
            }
        }

        let header = if header_column.len() > 0 {
            let hd_col = column(header_column)
                                                .spacing(tbl.header_row_spacing);
            Some(Element::from(
                scrollable(hd_col)
                    .width(tbl.width)
                    .id(tbl.header_scroller_id)
                    .direction({
                        let scrollbar = scrollable::Scrollbar::new()
                            .scroller_width(tbl.header_scroller_height)
                            .width(tbl.header_scrollbar_height)
                            .margin(tbl.header_scrollbar_margin)
                            .spacing(tbl.header_scrollbar_spacing);
                        scrollable::Direction::Horizontal(scrollbar)
                        })
                    .on_scroll(move|vp| Message::TableSync(
                                        vp.absolute_offset(), tbl.id))
                    ))
        } else {
            None
        };

        let footer = if tbl.custom_footer_rows > 0 {
            let mut footer_column= vec![];
            for _ in 0..tbl.custom_footer_rows {
                let mut rw = vec![];
                for i in 0..tbl.df.width() {
                    rw.push(Element::from(
                        container(content.remove(0))
                            .width(tbl.column_widths[i])
                            .height(tbl.footer_height)
                            .center_x(tbl.column_widths[i])
                            .style(move|theme|container::bordered_box(theme))
                            ));
                }
                footer_column.push(Element::from(row(rw)));
            }
            let ft_col = column(footer_column)
                                                .spacing(tbl.footer_spacing);
            Some(Element::from(
                scrollable(ft_col)
                    .width(tbl.width)
                    .id(tbl.footer_scroller_id)
                    .direction({
                        let scrollbar = scrollable::Scrollbar::new()
                            .scroller_width(tbl.footer_scroller_height)
                            .width(tbl.footer_scrollbar_height)
                            .margin(tbl.footer_scrollbar_margin)
                            .spacing(tbl.footer_scrollbar_spacing);
                        scrollable::Direction::Horizontal(scrollbar)
                        })
                    .on_scroll(move|vp| Message::TableSync(
                                        vp.absolute_offset(), tbl.id))
                    ))
        } else {
            None
        };

        let div_header = 
            divider_horizontal(
                tbl.id,
                tbl.column_widths.clone(),
                tbl.resizer_width,
                header_height + tbl.custom_header_rows as f32 * tbl.header_row_height,
                Message::TableDividerChanged,
            ).on_release(Message::TableDividerReleased(tbl.id));

        let div_footer = 
            divider_horizontal(
                tbl.id,
                tbl.column_widths.clone(),
                tbl.resizer_width,
                tbl.custom_footer_rows as f32 * tbl.footer_height,
                Message::TableDividerChanged,
            ).on_release(Message::TableDividerReleased(tbl.id));

        let mut main_col = vec![];

        if header.is_some() && tbl.resize_columns_enabled {
            let stk = stack([header.unwrap(), div_header.into()]).into();
            main_col.push(stk);
            main_col.push(Space::new(5.0, tbl.header_body_spacing).into());
        } else if header.is_some() && !tbl.resize_columns_enabled {
            main_col.push(header.unwrap());
            main_col.push(Space::new(5.0, tbl.header_body_spacing).into());
        }
        
        main_col.push(body.into());

        if footer.is_some() {
            main_col.push(Space::new(5.0, tbl.body_footer_spacing).into());
        }

        if footer.is_some() && tbl.resize_columns_enabled {
            let stk = stack([footer.unwrap().into(), div_footer.into()]).into();
            main_col.push(stk);
        } else if footer.is_some() && !tbl.resize_columns_enabled {
            main_col.push(footer.unwrap());
        }

        container(container(column(main_col)))
            .width(Fill)
            .height(Fill)
            .padding(20)
            .center_x(Fill)
            .center_y(Fill)
            .into()
    
}

#[derive(Clone, Debug, PartialEq)]
pub enum TableMessage {
    DivDragging((usize, f32)),
    DivOnRelease,
    SyncScrollables(usize),
}

pub fn table_callback(
        state: &mut IpgState,  
        id: usize,  
        message: TableMessage) 
        -> (Option<scrollable::Id>, Option<scrollable::Id>, Option<scrollable::Id>){

    let mut wci: WidgetCallbackIn = WidgetCallbackIn{id, ..Default::default()};

    match message {
        TableMessage::DivDragging((index, value)) => {
            wci.value_f32 = Some(value);
            wci.value_usize = Some(index);
            wci.value_str = Some("dragging".to_string());
            let wco = set_or_get_widget_callback_data(state, wci);
            process_callback1(
                id, 
                "dragging".to_string(), 
                index, 
                wco.vec_f32);
            return (None, None, None)
        },
        TableMessage::DivOnRelease=> {
            process_callback2(
                id, 
                "released".to_string()
            );
            return (None, None, None)
        },
        TableMessage::SyncScrollables(id) => {
            wci.id = id;
            wci.value_str = Some("sync".to_string());
            let wco = set_or_get_widget_callback_data(state, wci);

            return wco.scroller_ids.unwrap();
        }
    }
}

pub fn process_callback1(id: usize, event_name: String, index: usize, value: Vec<f32>) 
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

pub fn process_callback2(id: usize, event_name: String) 
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
                                                        user_data_opt,
                                                        ));
            match res {
                Ok(_) => (),
                Err(er) => panic!("Table Divider: 2 parameters (id, user_data) 
                                    are required or a python error in this function. {er}"),
            }
        } else {
            let res = cb.call1(py, (
                                                        id, 
                                                        ));
            match res {
                Ok(_) => (),
                Err(er) => panic!("Table Divider: 1 parameters (id) 
                                    are required or a python error in this function. {er}"),
            }
        }
    });

    drop(ud); 

}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgTableParam {
    PolarsDf,
    ColumnWidths,
    Height,
    Width,
    ResizerWidth,
    HeaderEnabled,
    HeaderHeight,
    HeaderScrollbarHeight,
    HeaderScrollbarMargin,
    HeaderScrollerHeight,
    HeaderScrollbarSpacing,
    HeaderRowSpacing,
    FooterHeight,
    FooterScrollbarHeight,
    FooterScrollbarMargin,
    FooterScrollerHeight,
    FooterScrollbarSpacing,
    FooterSpacing,
    BodyScrollbarWidth,
    BodyScrollbarMargin,
    BodyScrollerWidth,
    BodyScrollbarSpacing,
    CustomHeaderRows,
    CustomFooterRows,
    ControlColumns,
    ColumnProportionalResize,
    RowSpacing,
    RowHeight,
    HeaderBodySpacing,
    BodyFooterSpacing,
    ResizeColumnsEnabled,
    MinColumnWidth,
    TextSize,
    Show,
    TableWidthFixed,
    StyleId,
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
        IpgTableParam::ColumnWidths => {
            table.column_widths = try_extract_vec_f32(value, name);
        },
        IpgTableParam::Width => {
            let width = Some(try_extract_f32(value, name));
            table.width = get_width(width, false);
        },
        IpgTableParam::Height => {
            table.height = try_extract_f32(value, name);
        },
        IpgTableParam::HeaderEnabled => {
            table.header_enabled = try_extract_boolean(value, name);
        },
        IpgTableParam::HeaderHeight => {
            table.header_row_height = try_extract_f32(value, name);
        },
        IpgTableParam::HeaderScrollbarSpacing => {
            table.header_scrollbar_spacing = try_extract_f32(value, name);
        },
        IpgTableParam::HeaderRowSpacing => {
            table.header_row_spacing = try_extract_f32(value, name);
        },
        IpgTableParam::FooterHeight => {
            table.footer_height = try_extract_f32(value, name);
        },
        IpgTableParam::FooterScrollbarSpacing => {
            table.footer_scrollbar_spacing = try_extract_f32(value, name);
        },
        IpgTableParam::FooterSpacing => {
            table.footer_spacing = try_extract_f32(value, name);
        },
        IpgTableParam::BodyScrollbarSpacing => {
            table.body_scrollbar_spacing = try_extract_f32(value, name);
        },
        IpgTableParam::CustomHeaderRows => {
            table.custom_header_rows = try_extract_usize(value, name);
        },
        IpgTableParam::CustomFooterRows => {
            table.custom_footer_rows = try_extract_usize(value, name);
        },
        IpgTableParam::ControlColumns => {
            table.control_columns = try_extract_vec_usize(value, name);
        },
        IpgTableParam::RowHeight => {
            table.row_height = try_extract_f32(value, name);
        },
        IpgTableParam::HeaderBodySpacing => {
            table.header_body_spacing = try_extract_f32(value, name);
        },
        IpgTableParam::BodyFooterSpacing => {
            table.body_footer_spacing = try_extract_f32(value, name);
        },
        IpgTableParam::TextSize => {
            table.text_size = try_extract_f32(value, name);
        },
        IpgTableParam::StyleId => {
            table.style_id = Some(try_extract_usize(value, name));
        },
        IpgTableParam::ColumnProportionalResize => {
            table.column_proportional_resize = try_extract_boolean(value, name);
        },
        IpgTableParam::RowSpacing => {
            table.row_spacing = try_extract_f64(value, name) as f32;
        },
        IpgTableParam::ResizerWidth => {
            table.resizer_width = try_extract_f64(value, name) as f32;
        },
        IpgTableParam::ResizeColumnsEnabled => {
            table.resize_columns_enabled = try_extract_boolean(value, name);
        },
        IpgTableParam::MinColumnWidth => {
            table.min_column_width = Some(try_extract_f64(value, name) as f32);
        },
        IpgTableParam::Show => {
            table.show = try_extract_boolean(value, name);
        },
        IpgTableParam::TableWidthFixed => {
            table.table_width_fixed = try_extract_boolean(value, name);
        },
        IpgTableParam::HeaderScrollbarHeight => {
            table.header_scrollbar_height = try_extract_f32(value, name);
        },
        IpgTableParam::HeaderScrollbarMargin => {
            table.header_scrollbar_margin = try_extract_f32(value, name);
        },
        IpgTableParam::HeaderScrollerHeight => {
            table.header_scroller_height = try_extract_f32(value, name);
        },
        IpgTableParam::FooterScrollbarHeight => {
            table.footer_scrollbar_height = try_extract_f32(value, name);
        },
        IpgTableParam::FooterScrollbarMargin => {
            table.footer_scrollbar_margin = try_extract_f32(value, name);
        },
        IpgTableParam::FooterScrollerHeight => {
            table.footer_scroller_height = try_extract_f32(value, name);
        },
        IpgTableParam::BodyScrollbarWidth => {
            table.body_scrollbar_width = try_extract_f32(value, name);
        },
        IpgTableParam::BodyScrollbarMargin => {
            table.body_scrollbar_margin = try_extract_f32(value, name);
        },
        IpgTableParam::BodyScrollerWidth => {
            table.body_scroller_width = try_extract_f32(value, name);
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

