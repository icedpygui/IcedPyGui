//! ipg_chart

// #![allow(dead_code)]
use std::fs;
use std::path::Path;

use iced::widget::container;
use iced::{Color, Element, Point, Radians};
use pyo3::{pyclass, PyObject, Python};

use crate::app::Message;
use crate::chart::draw_chart::{IpgChartState, IpgDrawMode, 
    IpgDrawStatus, IpgWidget};
use crate::chart::geometries::{
    get_draw_mode_and_status, get_widget_id,
    set_widget_mode_or_status_or_id, IpgChartWidget
};
use crate::chart::import_export::{convert_to_export, import_widgets, save};
use crate::IpgState;

use super::helpers::{
    get_horizontal_alignment, get_vertical_alignment, try_extract_f64, try_extract_ipg_horizontal_alignment,
    try_extract_ipg_vertical_alignment, try_extract_point, try_extract_rgba_color, try_extract_string,
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

pub fn construct_chart(chart_state: &IpgChartState) -> Element<Message> {
    let draw: Element<ChartMessage> = container(
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
    WidgetDraw(IpgWidget),
}

pub fn chart_callback(chart_message: ChartMessage, app_state: &mut IpgState, chart_state: &mut IpgChartState) {
    match chart_message {
        ChartMessage::WidgetDraw(mut widget) => {
            // Since the text widget may have a blinking cursor, the only way to use a timer
            // is to use the main subscription one at this time, chart lacks a time event.
            // Therefore, the pending has to return the curve also at each change so that
            // the curves can be updated.  The subscription clears the text cache at each tick.
            match widget {
                IpgWidget::Text(_) => {
                    let (draw_mode, draw_status) = get_draw_mode_and_status(&widget);
                    let id = get_widget_id(&widget);
                    match draw_status {
                        IpgDrawStatus::Completed => {
                            widget = set_widget_mode_or_status_or_id(widget, Some(IpgDrawMode::Display), None, None);
                            chart_state.text_curves.entry(id).and_modify(|k| *k= widget.clone());
                            chart_state.timer_event_enabled = false;
                            chart_state.draw_mode = IpgDrawMode::Display;
                        },
                        IpgDrawStatus::Delete => {
                            chart_state.text_curves.remove(&id);
                            chart_state.timer_event_enabled = false;
                        },
                        IpgDrawStatus::Inprogress => {
                            // Since the text always returns a new curve or updated curve,
                            // a check for the first return is need to see if a text is present. 
                            let present = chart_state.text_curves.get(&id);
                            if present.is_none() {
                                chart_state.text_curves.insert(id, widget.clone());
                            } else {
                                chart_state.text_curves.entry(id).and_modify(|k| *k= widget.clone());
                            }
                        },
                    }
                    match draw_mode {
                        IpgDrawMode::Edit | IpgDrawMode::Rotate => {
                            let id = get_widget_id(&widget);
                            chart_state.edit_widget_id = Some(id);
                            chart_state.text_curves.entry(id).and_modify(|k| *k= widget);
                        },
                        _ => (),
                    }
                    chart_state.request_text_redraw();
                },
                _ => {
                    let (draw_mode, draw_status) = get_draw_mode_and_status(&widget);
                    match draw_status {
                        IpgDrawStatus::Completed => {
                            widget = set_widget_mode_or_status_or_id(widget, Some(IpgDrawMode::Display), None, None);
                        },
                        IpgDrawStatus::Delete => {
                            let id = get_widget_id(&widget);
                            chart_state.curves.remove(&id);
                        },  
                        _ => (),
                    }
                    if draw_mode == IpgDrawMode::New {
                        app_state.last_id += 1;
                        let id = app_state.last_id;
                        let widget = set_widget_mode_or_status_or_id(widget.clone(), 
                                                                                Some(IpgDrawMode::Display), 
                                                                                Some(IpgDrawStatus::Completed), 
                                                                                Some(id));
                        chart_state.curves.insert(id, widget);
                    } else {
                        // if not new must be in edit or rotate mode so modify.
                        let id = get_widget_id(&widget);
                        chart_state.edit_widget_id = Some(id);
                        chart_state.curves.entry(id).and_modify(|k| *k= widget);
                    }
                    
                    chart_state.request_redraw();
                },
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
    Widget,
    Load,
    Save,
    TextAlignment,
}

// update only the chart, not the propterties of the chart widgets.
// see chart_geometry_update
pub fn chart_item_update(chart_state: &mut IpgChartState, 
                            item: &PyObject, 
                            value: &PyObject,
                            mut last_id: usize,) 
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
        IpgChartParam::Load => {
            let path = Path::new(&chart_state.file_path);
            let data = fs::read_to_string(path).expect("Unable to read file");
            let widgets = serde_json::from_str(&data).expect("Unable to parse file");
            chart_state.clear_curves();
            (chart_state.curves, chart_state.text_curves, last_id) =
                import_widgets(widgets, last_id);
            chart_state.request_redraw();
            chart_state.request_text_redraw();
            Some(last_id)
        }
        IpgChartParam::Save => {
            let path = Path::new(&chart_state.file_path);
            let widgets = convert_to_export(&chart_state.curves, &chart_state.text_curves);
            let result = save(path, &widgets);
            match result {
                Ok(_) => (),
                Err(e) => println!("Unable to save file, {}", e),
            }
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
