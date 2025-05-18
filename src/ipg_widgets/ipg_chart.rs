//! ipg_chart
use iced::widget::container;

use iced::{Color, Element};
use pyo3::{pyclass, PyObject, Python};
use charts_rs_mod::{BarChart, SeriesCategory, Box, IcedComponent};


use crate::{access_chart_state, IpgState};
use crate::app::Message;
use crate::chart::draw_chart::{IpgChartState, ChartDrawMode};
use crate::chart::themes::IpgChartTheme;

use super::helpers::{
    get_horizontal_alignment, get_vertical_alignment, try_extract_f64, try_extract_ipg_horizontal_alignment,
    try_extract_ipg_vertical_alignment, try_extract_rgba_color, try_extract_string,
};
use super::ipg_enums::IpgHorizontalAlignment;

#[derive(Debug, Clone)]
pub struct IpgChartId {
    pub id: usize,
}

impl IpgChartId {
    pub fn new(id: usize) -> Self {
        Self { 
            id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IpgChart {
    pub id: usize,
    pub series: Vec<(String, Vec<f32>)>,
    pub x_axis_labels: Vec<String>,
    pub width: f32,
    pub height: f32,
    pub position_xy: Option<[f32; 2]>,
    pub theme: Option<IpgChartTheme>,
    pub margin: Option<[f32; 4]>,
    pub font_family: String,
    pub background_color: Option<Color>,
    pub is_light: bool,
    pub grid_stroke_color: Option<Color>,
    pub grid_stroke_width: f32,
    pub radius: Option<f32>,
}

impl IpgChart {
    pub fn new(
        id: usize,
        series: Vec<(String, Vec<f32>)>,
        x_axis_labels: Vec<String>,
        width: f32,
        height: f32,
        position_xy: Option<[f32; 2]>,
        theme: Option<IpgChartTheme>,
        margin: Option<[f32; 4]>,
        font_family: String,
        background_color: Option<Color>,
        is_light: bool,
        grid_stroke_color: Option<Color>,
        grid_stroke_width: f32,
        radius: Option<f32>,) -> Self {
        Self { 
            id,
            series,
            x_axis_labels,
            width,
            height,
            position_xy,
            theme,
            margin,
            font_family,
            background_color,
            is_light,
            grid_stroke_color,
            grid_stroke_width,
            radius,
         }
    }
}

pub fn display_chart<'a>(_chart: &'a IpgChartId,
                            mut cs: &'a IpgChartState,
                            ) 
                            -> Element<'a, Message> {

    
    let draw: iced::Element<ChartMessage> = container(
        cs
            .view(
                &cs.curves,
                &cs.text_curves,
                &cs.image_curves,
            )
            .map(ChartMessage::WidgetDraw),
    )
    .into();
    draw.map(move |message| Message::Chart(message))
}

pub fn construct_chart(
    chart_ids: Vec<String>,
    ) {
        let mut cs = access_chart_state();
        let id = match cs.chart_ids.get(&chart_ids[0]) {
            Some(id) => id,
            None => panic!("Construct Chart: Chart id {} not found", chart_ids[0]),
        };

        let chart = cs.charts.get(id).unwrap();

        let mut series: Vec<charts_rs_mod::Series> = vec![];
        for (s, v) in chart.series.iter() {
            series.push((&s[..], v.clone()).into());
        }
        let mut bar_chart = BarChart::new_with_theme(
        series,
        chart.x_axis_labels.clone(),
        charts_rs_mod::THEME_GRAFANA,
        );
        bar_chart.title_text = "BarChart".to_string();
        bar_chart.legend_margin = Some(Box {
            top: bar_chart.title_height,
            bottom: 5.0,
            ..Default::default()
        });
        bar_chart.series_list[2].category = Some(SeriesCategory::Line);
        bar_chart.series_list[2].y_axis_index = 1;
        bar_chart.series_list[2].label_show = true;

        bar_chart
            .y_axis_configs
            .push(bar_chart.y_axis_configs[0].clone());
        bar_chart.y_axis_configs[0].axis_formatter = Some("{c} ml".to_string());
        bar_chart.y_axis_configs[1].axis_formatter = Some("{c} °C".to_string());

        let bc = bar_chart.iced();
        for comp in bc {
            if get_type(&comp) {
                cs.text_curves.push(comp);
            } else {
                cs.curves.push(comp);
            }
        }
        
}

fn get_type(ic: &IcedComponent) -> bool {
    match ic {
        IcedComponent::Text(_) => true,
        _ => false,
    }
}

#[derive(Debug, Clone)]
pub struct IpgChartTitle {
    pub id: usize,
    pub chart_id: String,
    pub title_text: Option<String>,
    pub title_font_size: f32,
    pub title_font_color: Option<Color>,
    pub title_font_weight: Option<String>,
    pub title_margin: Option<[f32; 4]>,
    pub title_align: IpgHorizontalAlignment,
    pub title_height: f32,

    pub sub_title_text: Option<String>,
    pub sub_title_font_size: Option<f32>,
    pub sub_title_font_color: Option<Color>,
    pub sub_title_font_weight: Option<String>,
    pub sub_title_margin: Option<[f32; 4]>,
    pub sub_title_align: IpgHorizontalAlignment,
    pub sub_title_height: f32,
}

impl IpgChartTitle {
    pub fn new(
        id: usize,
        chart_id: String,
        title_text: Option<String>,
        title_font_size: f32,
        title_font_color: Option<Color>,
        title_font_weight: Option<String>,
        title_margin: Option<[f32; 4]>,
        title_align: IpgHorizontalAlignment,
        title_height: f32,
        
        sub_title_text: Option<String>,
        sub_title_font_size: Option<f32>,
        sub_title_font_color: Option<Color>,
        sub_title_font_weight: Option<String>,
        sub_title_margin: Option<[f32; 4]>,
        sub_title_align: IpgHorizontalAlignment,
        sub_title_height: f32,
    ) -> Self {
        Self { 
            id,
            chart_id,
            title_text,
            title_font_size,
            title_font_color,
            title_font_weight,
            title_margin,
            title_align,
            title_height,
            
            sub_title_text,
            sub_title_font_size,
            sub_title_font_color,
            sub_title_font_weight,
            sub_title_margin,
            sub_title_align,
            sub_title_height,
         }
    }
}

#[derive(Debug, Clone)]
pub struct IpgChartLegend {
    pub id: usize,
    pub chart_id: String,
    pub legend_font_size: f32,
    pub legend_font_ipgcolor: Option<Color>,
    pub legend_font_rgba: Option<[f32; 4]>,
    pub legend_font_weight: Option<String>,
    pub legend_align: IpgHorizontalAlignment,
    pub legend_margin: Option<[f32; 4]>,
    pub legend_category: IpgChartLegendCategory,
    pub legend_show: bool,
}

impl IpgChartLegend {
    pub fn new(
        id: usize,
        chart_id: String,
        legend_font_size: f32,
        legend_font_ipgcolor: Option<Color>,
        legend_font_rgba: Option<[f32; 4]>,
        legend_font_weight: Option<String>,
        legend_align: IpgHorizontalAlignment,
        legend_margin: Option<[f32; 4]>,
        legend_category: IpgChartLegendCategory,
        legend_show: bool,
    ) -> Self {
        Self { 
            id,
            chart_id,
            legend_font_size,
            legend_font_ipgcolor,
            legend_font_rgba,
            legend_font_weight,
            legend_align,
            legend_margin,
            legend_category,
            legend_show, 
        }
    }
}

#[derive(Debug, Clone)]
pub struct IpgChartXAxis {
    pub id: usize,
    pub chart_id: String,
    pub x_axis_data: Vec<String>,
    pub x_axis_height: f32,
    pub x_axis_stroke_color: Option<Color>,
    pub x_axis_font_size: f32,
    pub x_axis_font_color: Option<Color>,
    pub x_axis_font_weight: Option<String>,
    pub x_axis_name_gap: f32,
    pub x_axis_name_rotate: f32,
    pub x_axis_margin: Option<[f32; 4]>,
    pub x_axis_hidden: bool,
    pub x_boundary_gap: Option<bool>,
}

impl IpgChartXAxis {
    pub fn new(
        id: usize,
        chart_id: String,
        x_axis_data: Vec<String>,
        x_axis_height: f32,
        x_axis_stroke_color: Option<Color>,
        x_axis_font_size: f32,
        x_axis_font_color: Option<Color>,
        x_axis_font_weight: Option<String>,
        x_axis_name_gap: f32,
        x_axis_name_rotate: f32,
        x_axis_margin: Option<[f32; 4]>,
        x_axis_hidden: bool,
        x_boundary_gap: Option<bool>,
    ) -> Self {
        Self { 
            id,
            chart_id,
            x_axis_data,
            x_axis_height,
            x_axis_stroke_color,
            x_axis_font_size,
            x_axis_font_color,
            x_axis_font_weight,
            x_axis_name_gap,
            x_axis_name_rotate,
            x_axis_margin,
            x_axis_hidden,
            x_boundary_gap,
         }
    }
}

#[derive(Debug, Clone)]
pub struct IpgChartYAxis {
    pub id: usize,
    pub chart_id: String,
    pub y_axis_hidden: bool,
    pub y_axis_font_size: f32,
    pub y_axis_font_color: Option<Color>,
    pub y_axis_font_weight: Option<String>,
    pub y_axis_stroke_color: Option<Color>,
    pub y_axis_width: Option<f32>,
    pub y_axis_split_number: usize,
    pub y_axis_name_gap: f32,
    pub y_axis_name_align: Option<IpgHorizontalAlignment>,
    pub y_axis_margin: Option<[f32; 4]>,
    pub y_axis_formatter: Option<String>,
    pub y_axis_min: Option<f32>,
    pub y_axis_max: Option<f32>,
}

impl IpgChartYAxis {
    pub fn new(
        id: usize,
        chart_id: String,
        y_axis_hidden: bool,
        y_axis_font_size: f32,
        y_axis_font_color: Option<Color>,
        y_axis_font_weight: Option<String>,
        y_axis_stroke_color: Option<Color>,
        y_axis_width: Option<f32>,
        y_axis_split_number: usize,
        y_axis_name_gap: f32,
        y_axis_name_align: Option<IpgHorizontalAlignment>,
        y_axis_margin: Option<[f32; 4]>,
        y_axis_formatter: Option<String>,
        y_axis_min: Option<f32>,
        y_axis_max: Option<f32>,
    ) -> Self {
        Self { 
            id,
            chart_id,
            y_axis_hidden,
            y_axis_font_size,
            y_axis_font_color,
            y_axis_font_weight,
            y_axis_stroke_color,
            y_axis_width,
            y_axis_split_number,
            y_axis_name_gap,
            y_axis_name_align,
            y_axis_margin,
            y_axis_formatter,
            y_axis_min,
            y_axis_max,
         }
    }
}

#[derive(Debug, Clone)]
pub struct IpgChartSeries {
    pub id: usize,
    pub chart_id: String,
    pub series_stroke_width: f32,
    pub series_label_font_color: Option<Color>,
    pub series_label_font_size: f32,
    pub series_label_font_weight: Option<String>,
    pub series_label_formatter: Option<String>,
    pub series_colors: Vec<Color>,
    pub series_symbol: bool,
    pub symbol_color: Option<Color>,
    pub symbol_radius: Option<f32>,
    pub series_smooth: bool,
    pub series_fill: bool,
}

impl IpgChartSeries {
    pub fn new(
        id: usize,
        chart_id: String,
        series_stroke_width: f32,
        series_label_font_color: Option<Color>,
        series_label_font_size: f32,
        series_label_font_weight: Option<String>,
        series_label_formatter: Option<String>,
        series_colors: Vec<Color>,
        series_symbol: bool,
        symbol_color: Option<Color>,
        symbol_radius: Option<f32>,
        series_smooth: bool,
        series_fill: bool,
    ) -> Self {
        Self { 
            id,
            chart_id,
            series_stroke_width,
            series_label_font_color,
            series_label_font_size,
            series_label_font_weight,
            series_label_formatter,
            series_colors,
            series_symbol,
            symbol_color,
            symbol_radius,
            series_smooth,
            series_fill,
         }
    }
}

#[derive(Debug, Clone)]
pub enum ChartMessage {
    WidgetDraw(IcedComponent),
}


pub fn chart_callback(chart_message: ChartMessage, _app_state: &mut IpgState, _chart_state: &mut IpgChartState) {
    match chart_message {
        ChartMessage::WidgetDraw(widget) => {
            match widget {
                IcedComponent::Text(_) => {
                //     let (draw_mode, draw_status) = get_draw_mode_and_status(&widget);
                //     let id = get_widget_id(&widget);
                //     match draw_status {
                //         IpgDrawStatus::Completed => {
                //             widget = set_widget_mode_or_status_or_id(widget, Some(IpgDrawMode::Display), None, None);
                //             chart_state.text_curves.entry(id).and_modify(|k| *k= widget.clone());
                //             chart_state.timer_event_enabled = false;
                //             chart_state.draw_mode = IpgDrawMode::Display;
                //         },
                //         IpgDrawStatus::Delete => {
                //             chart_state.text_curves.remove(&id);
                //             chart_state.timer_event_enabled = false;
                //         },
                //         IpgDrawStatus::Inprogress => {
                //             // Since the text always returns a new curve or updated curve,
                //             // a check for the first return is need to see if a text is present. 
                //             let present = chart_state.text_curves.get(&id);
                //             if present.is_none() {
                //                 chart_state.text_curves.insert(id, widget.clone());
                //             } else {
                //                 chart_state.text_curves.entry(id).and_modify(|k| *k= widget.clone());
                //             }
                //         },
                //     }
                //     match draw_mode {
                //         IpgDrawMode::Edit => {
                //             let id = get_widget_id(&widget);
                //             chart_state.edit_widget_id = Some(id);
                //             chart_state.text_curves.entry(id).and_modify(|k| *k= widget);
                //         },
                //         _ => (),
                //     }
                //     chart_state.request_text_redraw();
                // },
                // _ => {
                //     let (draw_mode, draw_status) = get_draw_mode_and_status(&widget);
                //     match draw_status {
                //         IpgDrawStatus::Completed => {
                //             widget = set_widget_mode_or_status_or_id(widget, Some(IpgDrawMode::Display), None, None);
                //         },
                //         IpgDrawStatus::Delete => {
                //             let id = get_widget_id(&widget);
                //             chart_state.curves.remove(&id);
                //         },  
                //         _ => (),
                //     }
                //     if draw_mode == IpgDrawMode::New {
                //         app_state.last_id += 1;
                //         let id = app_state.last_id;
                //         let widget = set_widget_mode_or_status_or_id(widget.clone(), 
                //                                                                 Some(IpgDrawMode::Display), 
                //                                                                 Some(IpgDrawStatus::Completed), 
                //                                                                 Some(id));
                //         chart_state.curves.insert(id, widget);
                //     } else {
                //         // if not new must be in edit or rotate mode so modify.
                //         let id = get_widget_id(&widget);
                //         chart_state.edit_widget_id = Some(id);
                //         chart_state.curves.entry(id).and_modify(|k| *k= widget);
                //     }
                    
                //     chart_state.request_redraw();
                },
                _ => (),
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgChartParam {
    Clear,
    ChartColor,
    DrawColor,
    FillColor,
    DrawWidth,
    FilePath,
    Mode,
    PolyPoints,
    TextAlignment,
}

// update only the chart, not the propterties of the chart widgets.
// see chart_geometry_update
pub fn chart_item_update(chart_state: &mut IpgChartState, 
                            item: &PyObject, 
                            value: &PyObject,
                            _last_id: usize,) 
                            -> Option<usize> 
{
    let update = try_extract_chart_update(item);
    let name = "Chart".to_string();
    match update {
        IpgChartParam::Clear => {
            chart_state.clear_curves();
            None
        }
        IpgChartParam::ChartColor => {
            let rgba = try_extract_rgba_color(value, name);
            chart_state.selected_chart_color = Some(Color::from(rgba));
            chart_state.clear_background_cache();
            None
        }
        IpgChartParam::DrawColor => {
            let rgba = try_extract_rgba_color(value, name);
            chart_state.selected_draw_color = Color::from(rgba);
            None
        }
        IpgChartParam::FilePath => {
            chart_state.file_path = try_extract_string(value, name);
            None
        }
        IpgChartParam::FillColor => {
            let rgba = try_extract_rgba_color(value, name);
            chart_state.selected_fill_color = Some(Color::from(rgba));
            None
        }
        IpgChartParam::DrawWidth => {
            let width = try_extract_f64(value, name) as f32;
            chart_state.selected_width = width;
            None
        }
        IpgChartParam::Mode => {
            chart_state.draw_mode = try_extract_mode(value);
            None
        }
        IpgChartParam::PolyPoints => {
            let input = try_extract_string(value, name);
            chart_state.selected_poly_points = match input.parse::<usize>() {
                Ok(int) => int,
                Err(e) => panic!("PolyPoint input must be an integer, {}", e),
            };
            None
        }
        IpgChartParam::TextAlignment => {
            let align = try_extract_ipg_horizontal_alignment(value);
            if align.is_some() {
                chart_state.selected_h_text_alignment = get_horizontal_alignment(&align.unwrap())
            }
            let align = try_extract_ipg_vertical_alignment(value);
            if align.is_some() {
                chart_state.selected_v_text_alignment = get_vertical_alignment(&align.unwrap());
            }
            None
        }
        // IpgChartParam::Widget => {
        //     let selected_widget = Some(try_extract_widget(value));
        //     chart_state.selected_widget = selected_widget;
        //     chart_state.timer_event_enabled = selected_widget == Some(IpgChartWidget::Text);
        //     None
        // }
    }
}

pub fn try_extract_chart_update(update_obj: &PyObject) -> IpgChartParam {
    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgChartParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Chart update extraction failed"),
        }
    })
}

fn try_extract_mode(update_obj: &PyObject) -> ChartDrawMode {
    Python::with_gil(|py| {
        let res = update_obj.extract::<ChartDrawMode>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Chart mode update extraction failed"),
        }
    })
}

// fn try_extract_widget(update_obj: &PyObject) -> IcedComponent {
//     Python::with_gil(|py| {
//         let res = update_obj.extract::<IcedComponent>(py);
//         match res {
//             Ok(update) => update,
//             Err(_) => panic!("Chart IcedComponent update extraction failed"),
//         }
//     })
// }

#[derive(Debug, Clone, PartialEq, Default)]
pub struct BarChartElements {
    pub line: Vec<Line>,
    pub circle: Vec<Circle>,
    pub rect: Vec<Rect>,
    pub text: Vec<Text>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    pub start: iced::Point,
    pub end: iced::Point,
    pub stroke_width: f32,
    pub stroke: Option<iced::Color>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Circle {
    pub center: iced::Point,
    pub radius: f32,
    pub fill: Option<iced::Color>,
    pub stroke: Option<iced::Color>,
    pub stroke_width: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rect {
    pub top_left: iced::Point,
    pub size: iced::Size,
    pub fill: Option<iced::Color>,
    pub stroke: Option<iced::Color>,
    pub stroke_width: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Text {

}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
enum ChartTheme {
    ThemeGrafana,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgChartGeometryParam {
    Position,
    Rotation,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgChartLegendCategory {
    Normal,
    RoundRect,
    Circle,
    Rect,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgSeriesCategory {
    Line,
    Bar,
}

// pub fn construct_bar_chart(chart_ids: Vec<String>) -> (Vec<ChartWidget>, Vec<ChartWidget>) {
    
//     let cs = access_chart_state();
    
    
// }

// use svg_simple_parser::parse;
// use regex::Regex;
// pub fn parse_svg(svg: String) -> (Vec<ChartWidget>, Vec<ChartWidget>) {

//     let mut text_values = vec![];
//     let re = Regex::new(r"(?i)<text[^>]*?>([\s\S]*?)<\/text>").unwrap();
//     for cap in re.captures_iter(&svg) {
//             text_values.push(cap[1].trim().to_string());
//     }

//     let (_, root) = parse(&svg).unwrap();

//     let mut bar_elements: Vec<ChartWidget> = vec![];
//     let mut text_elements: Vec<ChartWidget> = vec![];

//     for child in root.children.borrow().iter() {
//         if child.ele_type == "g" {
//             for elem in  child.children.borrow().iter() {
//                 match elem.ele_type {
//                     "line" => {
//                         let line = get_line(elem, None);
//                         bar_elements.push(ChartWidget::Line(line));
//                     },
//                     "circle" => {
//                         let cir = get_circle(elem);
//                         bar_elements.push(ChartWidget::Circle(cir));
//                     },
//                     "rect" => {
                        
//                     }
//                     "text" => {
//                         let txt = get_text(elem, text_values.remove(0));
//                         text_elements.push(ChartWidget::Text(txt));
//                     },
//                     "g" => {
//                         let stroke = elem.attributes.borrow().get("stroke").map(|v| &**v);
//                         for child in  elem.children.borrow().iter() {
//                             match child.ele_type {  
//                                 "line" => {
//                                     let line = get_line(child, stroke);
//                                     bar_elements.push(ChartWidget::Line(line));
//                                 },
                                
//                                 _ => println!("g - not found"),
//                             }
//                         }
//                     },
//                     "path" => {
//                         let pline = get_polyline(elem);
//                         if pline.is_some() {
//                             bar_elements.push(ChartWidget::PolyLine(pline.unwrap()));
//                         }
//                     },
//                     _ => {
//                        dbg!("under g ",elem.ele_type);
//                     }
//                 }
//             }
//         } else {
//             match child.ele_type {
//                 "rect" => {
//                     let x = child.attributes.borrow().get("x").unwrap().parse::<f32>().unwrap();
//                     let y = child.attributes.borrow().get("y").unwrap().parse::<f32>().unwrap();
//                     let width = child.attributes.borrow().get("width").unwrap().parse::<f32>().unwrap();
//                     let height = child.attributes.borrow().get("height").unwrap().parse::<f32>().unwrap();
//                     let fill = child.attributes.borrow().get("fill")
//                                         .map(|v| &**v).unwrap();
//                     let stroke = child.attributes.borrow().get("fill")
//                                         .map(|v| &**v).unwrap();
//                     let stroke_width = child.attributes.borrow().get("stroke-width")
//                         .unwrap_or(&"0.0").parse::<f32>().unwrap();
//                     let rect = 
//                         ChartRectangle{
//                             id: 0, 
//                             top_left: iced::Point::new(x, y), 
//                             size: iced::Size::new(width, height), 
//                             stroke: iced::Color::parse(stroke).unwrap(),
//                             stroke_width,
//                             fill_color: iced::Color::parse(fill),
//                             stroke_dash_offset: None,
//                             stroke_dash_segments: None,
//                             draw_mode: ChartDrawMode::Display,
//                             status: ChartDrawStatus::Completed,
//                             };

//                     bar_elements.push(ChartWidget::Rectangle(rect));
//                 },
//                 "text" => {
//                     let txt = get_text(child, text_values.remove(0));
//                     text_elements.push(ChartWidget::Text(txt));
//                 }
//                 _ => {
//                     dbg!("others", child.ele_type);
//                 }         
//             }
            
//         }
//     }
//      (bar_elements, text_elements)
// }

// use svg_simple_parser::Element;
// fn get_line(child: &Rc<Element<'_>>, stroke_alt: Option<&str>) -> ChartLine {
//     let attr = child.attributes.borrow();
//     let start_x = attr.get("x1").unwrap().parse::<f32>().unwrap();
//     let start_y = attr.get("y1").unwrap().parse::<f32>().unwrap();
//     let end_x = attr.get("x2").unwrap().parse::<f32>().unwrap();
//     let end_y = attr.get("y2").unwrap().parse::<f32>().unwrap();
//     let stroke_opt = attr.get("stroke");
    
//     let stroke = stroke_opt
//     .map(|s| iced::Color::parse(s))
//     .unwrap_or_else(|| stroke_alt.map(|s| iced::Color::parse(s)).unwrap_or(Some(iced::Color::WHITE)));

//     let stroke_width = attr.get("stroke-width").unwrap().parse::<f32>().unwrap();

//     ChartLine {
//         id: 0,
//         points: vec![iced::Point::new(start_x, start_y), iced::Point::new(end_x, end_y)],
//         stroke: stroke.unwrap(),
//         stroke_width,
//         stroke_dash_offset: None,
//         stroke_dash_segments: None,
//         draw_mode: ChartDrawMode::Display,
//         status: ChartDrawStatus::Completed,
//     }

// }

// fn get_circle(child: &Rc<Element<'_>>) -> ChartCircle {
//     let attr = child.attributes.borrow();
//     let x = attr.get("cx").unwrap().parse::<f32>().unwrap();
//     let y = attr.get("cy").unwrap().parse::<f32>().unwrap();
//     let radius = attr.get("r").unwrap().parse::<f32>().unwrap();
//     let stroke = attr.get("stroke").unwrap();
//     let stroke_color = iced::Color::parse(stroke).unwrap();
//     let stroke_width = attr.get("stroke-width").unwrap().parse::<f32>().unwrap();
//     let fill = attr.get("fill").map(|v| &**v).unwrap();
//     let fill_color = iced::Color::parse(fill);

//     ChartCircle {
//         id: 0, 
//         center: iced::Point::new(x, y), 
//         radius, 
//         fill_color, 
//         stroke: stroke_color, 
//         stroke_width,
//         stroke_dash_offset: None,
//         stroke_dash_segments: None,
//         draw_mode: ChartDrawMode::Display,
//         status: ChartDrawStatus::Completed, 
//     }

// }

// fn get_text(child: &Rc<Element<'_>>, value: String) -> ChartText {
//     let attr = child.attributes.borrow();
//     let mut x = attr.get("x").unwrap().parse::<f32>().unwrap();
//     let dx = attr.get("dx");
//     if dx.is_some() {
//         x += dx.unwrap().parse::<f32>().unwrap();
//     }
//     let mut y = attr.get("y").unwrap().parse::<f32>().unwrap();
//     let dy = attr.get("dy");
//     if dy.is_some() {
//         y += dy.unwrap().parse::<f32>().unwrap();
//     }  
//     let fill = attr.get("fill").map(|v| &**v).unwrap();
//     let fill_color = iced::Color::parse(fill).unwrap();
//     let size = attr.get("font-size").unwrap().parse::<f32>().unwrap();

//     ChartText { 
//         id: 0, 
//         content: value, 
//         position: iced::Point::new(x, y), 
//         color: fill_color, 
//         size: size.into(), 
//         line_height: LineHeight::default(), 
//         font: "Roboto".to_string(), 
//         horizontal_alignment: alignment::Horizontal::Left, 
//         vertical_alignment: alignment::Vertical::Center, 
//         shaping: Shaping::Basic, 
//         rotation: 0.0, 
//         draw_mode: ChartDrawMode::Display,
//         status: ChartDrawStatus::Completed, 
//     }
// }

// fn get_polyline(child: &Rc<Element<'_>>) -> Option<ChartPolyLine> {
//     let attr = child.attributes.borrow();
//     let d = attr.get("d").map(|v| &**v);
    
//     if let Some(d) = d {
//         // Regex to match x, y points
//         let re = Regex::new(r"([0-9.]+)\s([0-9.]+)").unwrap();

//         // Extract points as tuples
//         let points = re
//             .captures_iter(d)
//             .filter_map(|cap| {
//                 let x = cap[1].parse::<f32>().ok();
//                 let y = cap[2].parse::<f32>().ok();
//                 match (x, y) {
//                     (Some(x), Some(y)) => Some(iced::Point::new(x, y)),
//                     _ => None,
//                 }
//             })
//             .collect();
        
//         let stroke = attr.get("stroke").unwrap();
//         let stroke_color = iced::Color::parse(stroke).unwrap();
//         let stroke_width = attr.get("stroke-width").unwrap().parse::<f32>().unwrap();

//         Some(ChartPolyLine {
//             id: 0,
//             points,
//             stroke: stroke_color,
//             stroke_width,
//             stroke_dash_offset: None,
//             stroke_dash_segments: None,
//             draw_mode: ChartDrawMode::Display,
//             status: ChartDrawStatus::Completed,
//         })
//     } else {
//         None
//     }
// }

// pub fn match_chart_widget(widget: &mut IpgWidget, item: &PyObject, value: &PyObject) {
//     let update_item = try_extract_geometry_update(item);
//     let name = "ChartGeometry".to_string();
//     match widget {
//         IpgWidget::None => (),
//         IpgWidget::Arc(arc) => match update_item {
//             IpgChartGeometryParam::Position => {
//                 let val = try_extract_point(value, name);
//                 arc.mid_point = Point::from(val);
//             }
//             IpgChartGeometryParam::Rotation => {
//                 panic!("Arc has no rotation property")
//             }
//         },
//         IpgWidget::Bezier(bz) => match update_item {
//             IpgChartGeometryParam::Position => {
//                 let val = try_extract_point(value, name);
//                 bz.mid_point = Point::from(val);
//             }
//             IpgChartGeometryParam::Rotation => {
//                 let val = try_extract_f64(value, name) as f32;
//                 bz.rotation = val;
//             }
//         },
//         IpgWidget::Circle(cir) => match update_item {
//             IpgChartGeometryParam::Position => {
//                 let val = try_extract_point(value, name);
//                 cir.center = Point::from(val);
//             }
//             IpgChartGeometryParam::Rotation => {
//                 panic!("Circle update has no rotation property")
//             }
//         },
//         IpgWidget::Ellipse(ell) => match update_item {
//             IpgChartGeometryParam::Position => {
//                 let val = try_extract_point(value, name);
//                 ell.center = Point::from(val);
//             }
//             IpgChartGeometryParam::Rotation => {
//                 let val = try_extract_f64(value, name) as f32;
//                 ell.rotation = Radians(val);
//             }
//         },
//         IpgWidget::Image(img) => match update_item {
//             IpgChartGeometryParam::Position => {
//                 let val = try_extract_point(value, name);
//                 img.position = Point::from(val);
//             }
//             IpgChartGeometryParam::Rotation => {
//                 let val = try_extract_f64(value, name) as f32;
//                 img.rotation = val;
//             }
//         },
//         IpgWidget::Line(line) => match update_item {
//             IpgChartGeometryParam::Position => {
//                 let val = try_extract_point(value, name);
//                 line.mid_point = Point::from(val);
//             }
//             IpgChartGeometryParam::Rotation => {
//                 let val = try_extract_f64(value, name) as f32;
//                 line.rotation = val;
//             }
//         },
//         IpgWidget::PolyLine(pl) => match update_item {
//             IpgChartGeometryParam::Position => {
//                 let val = try_extract_point(value, name);
//                 pl.mid_point = Point::from(val);
//             }
//             IpgChartGeometryParam::Rotation => {
//                 let val = try_extract_f64(value, name) as f32;
//                 pl.rotation = val;
//             }
//         },
//         IpgWidget::Polygon(pg) => match update_item {
//             IpgChartGeometryParam::Position => {
//                 let val = try_extract_point(value, name);
//                 pg.mid_point = Point::from(val);
//             }
//             IpgChartGeometryParam::Rotation => {
//                 let val = try_extract_f64(value, name) as f32;
//                 pg.rotation = val;
//             }
//         },
//         IpgWidget::Rectangle(rect) => match update_item {
//             IpgChartGeometryParam::Position => {
//                 let val = try_extract_point(value, name);
//                 rect.mid_point = Point::from(val);
//             }
//             IpgChartGeometryParam::Rotation => {
//                 panic!("Rectangle has no rotation property use polygon with 4 sides")
//             }
//         },
//         IpgWidget::RightTriangle(tr) => match update_item {
//             IpgChartGeometryParam::Position => {
//                 let val = try_extract_point(value, name);
//                 tr.mid_point = Point::from(val);
//             }
//             IpgChartGeometryParam::Rotation => {
//                 let val = try_extract_f64(value, name) as f32;
//                 tr.rotation = val;
//             }
//         },
//         IpgWidget::Text(txt) => match update_item {
//             IpgChartGeometryParam::Position => {
//                 let val = try_extract_point(value, name);
//                 txt.position = Point::from(val);
//             }
//             IpgChartGeometryParam::Rotation => {
//                 let val = try_extract_f64(value, name) as f32;
//                 txt.rotation = val;
//             }
//         },
//         IpgWidget::FreeHand(fh) => match update_item {
//             IpgChartGeometryParam::Position => {
//                 let val = try_extract_point(value, name);
//                 fh.points[0] = Point::from(val);
//             }
//             IpgChartGeometryParam::Rotation => {
//                 panic!("Freehand geometry has no rotation property")
//             }
//         },
//     }
// }

// pub fn try_extract_geometry_update(update_obj: &PyObject) -> IpgChartGeometryParam {
//     Python::with_gil(|py| {
//         let res = update_obj.extract::<IpgChartGeometryParam>(py);
//         match res {
//             Ok(update) => update,
//             Err(_) => panic!("Chart update extraction failed"),
//         }
//     })
// }
