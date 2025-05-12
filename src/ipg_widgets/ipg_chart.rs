//! ipg_chart

use std::rc::Rc;

use iced::widget::container;
use iced::widget::text::{LineHeight, Shaping};
use iced::{alignment, Color};
use pyo3::{pyclass, PyObject, Python};
use charts_rs::*;

use crate::app::Message;
use crate::chart::draw_chart::{IpgChartState, IpgDrawMode, 
    IpgDrawStatus, ChartWidget};
use crate::chart::geometries::{
    ChartCircle, ChartLine, ChartRectangle, ChartText, IpgChartWidget
};

use super::helpers::{
    get_horizontal_alignment, get_vertical_alignment, try_extract_f64, try_extract_ipg_horizontal_alignment,
    try_extract_ipg_vertical_alignment, try_extract_rgba_color, try_extract_string,
};

#[derive(Debug, Clone)]
pub struct IpgChart {
    pub id: usize,
}

impl IpgChart {
    pub fn new(id: usize) -> Self {
        Self { id }
    }
}

pub fn construct_chart(chart_state: &IpgChartState) -> iced::Element<Message> {
    let draw: iced::Element<ChartMessage> = container(
        chart_state
            .view(
                &chart_state.curves,
                &chart_state.text_curves,
                &chart_state.image_curves,
            )
            .map(ChartMessage::WidgetDraw),
    )
    .into();
    draw.map(move |message| Message::Chart(message))
}

#[derive(Debug, Clone)]
pub enum ChartMessage {
    WidgetDraw(ChartWidget),
}

// pub fn chart_callback(chart_message: ChartMessage, app_state: &mut IpgState, chart_state: &mut IpgChartState) {
//     match chart_message {
//         ChartMessage::WidgetDraw(mut widget) => {
//             match widget {
//                 ChartWidget::Text(_) => {
//                     let (draw_mode, draw_status) = get_draw_mode_and_status(&widget);
//                     let id = get_widget_id(&widget);
//                     match draw_status {
//                         IpgDrawStatus::Completed => {
//                             widget = set_widget_mode_or_status_or_id(widget, Some(IpgDrawMode::Display), None, None);
//                             chart_state.text_curves.entry(id).and_modify(|k| *k= widget.clone());
//                             chart_state.timer_event_enabled = false;
//                             chart_state.draw_mode = IpgDrawMode::Display;
//                         },
//                         IpgDrawStatus::Delete => {
//                             chart_state.text_curves.remove(&id);
//                             chart_state.timer_event_enabled = false;
//                         },
//                         IpgDrawStatus::Inprogress => {
//                             // Since the text always returns a new curve or updated curve,
//                             // a check for the first return is need to see if a text is present. 
//                             let present = chart_state.text_curves.get(&id);
//                             if present.is_none() {
//                                 chart_state.text_curves.insert(id, widget.clone());
//                             } else {
//                                 chart_state.text_curves.entry(id).and_modify(|k| *k= widget.clone());
//                             }
//                         },
//                     }
//                     match draw_mode {
//                         IpgDrawMode::Edit => {
//                             let id = get_widget_id(&widget);
//                             chart_state.edit_widget_id = Some(id);
//                             chart_state.text_curves.entry(id).and_modify(|k| *k= widget);
//                         },
//                         _ => (),
//                     }
//                     chart_state.request_text_redraw();
//                 },
//                 _ => {
//                     let (draw_mode, draw_status) = get_draw_mode_and_status(&widget);
//                     match draw_status {
//                         IpgDrawStatus::Completed => {
//                             widget = set_widget_mode_or_status_or_id(widget, Some(IpgDrawMode::Display), None, None);
//                         },
//                         IpgDrawStatus::Delete => {
//                             let id = get_widget_id(&widget);
//                             chart_state.curves.remove(&id);
//                         },  
//                         _ => (),
//                     }
//                     if draw_mode == IpgDrawMode::New {
//                         app_state.last_id += 1;
//                         let id = app_state.last_id;
//                         let widget = set_widget_mode_or_status_or_id(widget.clone(), 
//                                                                                 Some(IpgDrawMode::Display), 
//                                                                                 Some(IpgDrawStatus::Completed), 
//                                                                                 Some(id));
//                         chart_state.curves.insert(id, widget);
//                     } else {
//                         // if not new must be in edit or rotate mode so modify.
//                         let id = get_widget_id(&widget);
//                         chart_state.edit_widget_id = Some(id);
//                         chart_state.curves.entry(id).and_modify(|k| *k= widget);
//                     }
                    
//                     chart_state.request_redraw();
//                 },
//             }
//         }
//     }
// }

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
    Widget,
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
        IpgChartParam::Widget => {
            let selected_widget = Some(try_extract_widget(value));
            chart_state.selected_widget = selected_widget;
            chart_state.timer_event_enabled = selected_widget == Some(IpgChartWidget::Text);
            None
        }
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

fn try_extract_mode(update_obj: &PyObject) -> IpgDrawMode {
    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgDrawMode>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Chart mode update extraction failed"),
        }
    })
}

fn try_extract_widget(update_obj: &PyObject) -> IpgChartWidget {
    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgChartWidget>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Chart widget update extraction failed"),
        }
    })
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgChartGeometryParam {
    Position,
    Rotation,
}

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

pub fn construct_bar_chart() -> String {
    let mut bar_chart = BarChart::new_with_theme(
        vec![
            ("Evaporation", vec![2.0, 4.9, 7.0, 23.2, 25.6, 76.7, 135.6]).into(),
            (
                "Precipitation",
                vec![2.6, 5.9, 9.0, 26.4, 28.7, 70.7, 175.6],
            )
                .into(),
            ("Temperature", vec![2.0, 2.2, 3.3, 4.5, 6.3, 10.2, 20.3]).into(),
        ],
        vec![
            "Mon".to_string(),
            "Tue".to_string(),
            "Wed".to_string(),
            "Thu".to_string(),
            "Fri".to_string(),
            "Sat".to_string(),
            "Sun".to_string(),
        ],
        THEME_GRAFANA,
    );
    bar_chart.title_text = "Mixed Line and Bar".to_string();
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

    bar_chart.svg().unwrap()
    
}

use svg_simple_parser::parse;
use regex::Regex;
pub fn parse_svg(svg: String) -> (Vec<ChartWidget>, Vec<ChartWidget>) {

    let mut text_values = vec![];
    let re = Regex::new(r"(?i)<text[^>]*?>([\s\S]*?)<\/text>").unwrap();
    for cap in re.captures_iter(&svg) {
            text_values.push(cap[1].trim().to_string());
        }

    let title = text_values.remove(0);

    let (_, root) = parse(&svg).unwrap();

    let mut bar_elements: Vec<ChartWidget> = vec![];
    let mut text_elements: Vec<ChartWidget> = vec![];

    for child in root.children.borrow().iter() {
        if child.ele_type == "g" {
            for elem in  child.children.borrow().iter() {
                match elem.ele_type {
                    "line" => {
                        let line = get_line(elem, None);
                        bar_elements.push(ChartWidget::Line(line));
                    },
                    "circle" => {
                        let cir = get_circle(elem);
                       bar_elements.push(ChartWidget::Circle(cir));
                    },
                    "rect" => {
                        
                    }
                    "text" => {

                        let txt = get_text(elem, text_values.remove(0));
                        text_elements.push(ChartWidget::Text(txt));
                    },
                    "g" => {
                        let stroke = elem.attributes.borrow().get("stroke").map(|v| &**v);
                        for child in  elem.children.borrow().iter() {
                            match child.ele_type {  
                                "line" => {
                                    let line = get_line(child, stroke);
                                    bar_elements.push(ChartWidget::Line(line));
                                },
                                _ => println!("g - not found"),
                            }
                        }
                    }
                    _ => {
                       
                        // dbg!(elem_type);
                    }
                }
            }
        } else {

            match child.ele_type {
                "rect" => {
                    let x = child.attributes.borrow().get("x").unwrap().parse::<f32>().unwrap();
                    let y = child.attributes.borrow().get("y").unwrap().parse::<f32>().unwrap();
                    let width = child.attributes.borrow().get("width").unwrap().parse::<f32>().unwrap();
                    let height = child.attributes.borrow().get("height").unwrap().parse::<f32>().unwrap();
                    let fill = child.attributes.borrow().get("fill")
                                        .map(|v| &**v).unwrap();
                    let stroke = child.attributes.borrow().get("fill")
                                        .map(|v| &**v).unwrap();
                    let stroke_width = child.attributes.borrow().get("stroke-width")
                        .unwrap_or(&"0.0").parse::<f32>().unwrap();
                    let rect = 
                        ChartRectangle{
                            id: 0, 
                            top_left: iced::Point::new(x, y), 
                            size: iced::Size::new(width, height), 
                            stroke: iced::Color::parse(stroke).unwrap(),
                            stroke_width,
                            fill_color: iced::Color::parse(fill),
                            stroke_dash_offset: None,
                            stroke_dash_segments: None,
                            draw_mode: IpgDrawMode::Display,
                            status: IpgDrawStatus::Completed,
                            };

                    bar_elements.push(ChartWidget::Rectangle(rect));
                },
                _ => {
                    // dbg!(child.ele_type);
                }         
             
            }
            
        }
    }
     (bar_elements, text_elements)
}

use svg_simple_parser::Element;
fn get_line(child: &Rc<Element<'_>>, stroke_alt: Option<&str>) -> ChartLine {
    let attr = child.attributes.borrow();
    let start_x = attr.get("x1").unwrap().parse::<f32>().unwrap();
    let start_y = attr.get("y1").unwrap().parse::<f32>().unwrap();
    let end_x = attr.get("x2").unwrap().parse::<f32>().unwrap();
    let end_y = attr.get("y2").unwrap().parse::<f32>().unwrap();
    let stroke_opt = attr.get("stroke");
    
    let stroke = stroke_opt
    .map(|s| iced::Color::parse(s))
    .unwrap_or_else(|| stroke_alt.map(|s| iced::Color::parse(s)).unwrap_or(Some(iced::Color::WHITE)));

    let stroke_width = attr.get("stroke-width").unwrap().parse::<f32>().unwrap();

    ChartLine {
        id: 0,
        points: vec![iced::Point::new(start_x, start_y), iced::Point::new(end_x, end_y)],
        stroke: stroke.unwrap(),
        stroke_width,
        stroke_dash_offset: None,
        stroke_dash_segments: None,
        draw_mode: IpgDrawMode::Display,
        status: IpgDrawStatus::Completed,
    }

}

fn get_circle(child: &Rc<Element<'_>>) -> ChartCircle {
    let attr = child.attributes.borrow();
    let x = attr.get("cx").unwrap().parse::<f32>().unwrap();
    let y = attr.get("cy").unwrap().parse::<f32>().unwrap();
    let radius = attr.get("r").unwrap().parse::<f32>().unwrap();
    let stroke = attr.get("stroke").unwrap();
    let stroke_color = iced::Color::parse(stroke).unwrap();
    let stroke_width = attr.get("stroke-width").unwrap().parse::<f32>().unwrap();
    let fill = attr.get("fill").map(|v| &**v).unwrap();
    let fill_color = iced::Color::parse(fill);

    ChartCircle {
        id: 0, 
        center: iced::Point::new(x, y), 
        radius, 
        fill_color, 
        stroke: stroke_color, 
        stroke_width,
        stroke_dash_offset: None,
        stroke_dash_segments: None,
        draw_mode: IpgDrawMode::Display,
        status: IpgDrawStatus::Completed, 
    }

}

fn get_text(child: &Rc<Element<'_>>, value: String) -> ChartText {
    let attr = child.attributes.borrow();
    let x = attr.get("x").unwrap().parse::<f32>().unwrap();
    let y = attr.get("y").unwrap().parse::<f32>().unwrap(); 
    let fill = attr.get("fill").map(|v| &**v).unwrap();
    let fill_color = iced::Color::parse(fill).unwrap();
    let size = attr.get("font-size").unwrap().parse::<f32>().unwrap();

    ChartText { 
        id: 0, 
        content: value, 
        position: iced::Point::new(x, y), 
        color: fill_color, 
        size: size.into(), 
        line_height: LineHeight::default(), 
        font: iced::Font::DEFAULT, 
        horizontal_alignment: alignment::Horizontal::Center, 
        vertical_alignment: alignment::Vertical::Center, 
        shaping: Shaping::Basic, 
        rotation: 0.0, 
        draw_mode: IpgDrawMode::Display,
        status: IpgDrawStatus::Completed, 
    }
}


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
