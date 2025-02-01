//! ipg_canvas

// #![allow(dead_code)]
use std::fs;
use std::path::Path;

use iced::widget::container;
use iced::{Color, Element, Point};
use pyo3::{pyclass, PyObject, Python};

use crate::app::Message;
use crate::canvas::draw_canvas::{IpgCanvasState, IpgDrawMode, 
    IpgDrawStatus, IpgWidget};
use crate::canvas::geometries::{get_draw_mode_and_status, 
    get_widget_id, set_widget_mode_or_status, IpgCanvasWidget};
use crate::canvas::import_export::{convert_to_export, import_widgets, save};
use crate::IpgState;

use super::helpers::{get_horizontal_alignment, get_vertical_alignment, try_extract_f64, try_extract_ipg_horizontal_alignment, try_extract_ipg_vertical_alignment, try_extract_point, try_extract_rgba_color, try_extract_string};


#[derive(Debug, Clone)]
pub struct IpgCanvas {
    pub id: usize,
}

impl IpgCanvas {
    pub fn new(
        id: usize,
    ) -> Self {
        Self {
            id,
        }
    }
}

pub fn construct_canvas(canvas_state: &IpgCanvasState) -> Element<Message>{
    let draw: Element<CanvasMessage> =  
            container(canvas_state.view(&canvas_state.curves, 
                                &canvas_state.text_curves,
                                &canvas_state.image_curves,
                            )
                .map(CanvasMessage::WidgetDraw)).into();
    draw.map(move |message| Message::Canvas(message))
}


#[derive(Debug, Clone)]
pub enum CanvasMessage {
    WidgetDraw(IpgWidget),
}

pub fn canvas_callback(canvas_message: CanvasMessage,
                        app_state: &mut IpgState,
                        canvas_state: &mut IpgCanvasState,
                        ) {
    match canvas_message {
            CanvasMessage::WidgetDraw(mut widget) => {
                // Since the text widget has a blinking cursor, the only way to use a timer
                // is to use the main subscription one at this time, canvas lacks a time event.
                // Therefore, the pending has to return the curve at each change so that
                // the curves can be updated.  The subscription clears the text cache at each tick.
                match widget {
                    IpgWidget::Text(ref txt) => {
                        match txt.status {
                            IpgDrawStatus::Completed => {
                                let id = if txt.id == 0 {
                                    app_state.last_id +=1;
                                    app_state.last_id
                                } else {
                                    txt.id
                                };
                                canvas_state.text_curves
                                    .entry(id)
                                    .and_modify(|k| *k= widget.clone());
                                canvas_state.timer_event_enabled = false;
                            },
                            IpgDrawStatus::Delete => {
                                canvas_state.text_curves.remove(&txt.id);
                                canvas_state.timer_event_enabled = false;
                            },
                            IpgDrawStatus::Inprogress => {
                                // Since the text always returns a new curve or updated curve,
                                // a check for the first return is need to see if a text is present. 
                                if txt.id != 0 {
                                    canvas_state.text_curves.insert(txt.id, widget.clone());
                                } else {
                                    app_state.last_id +=1;
                                    canvas_state.text_curves
                                        .entry(app_state.last_id)
                                        .and_modify(|k| *k= widget.clone());
                                }
                            },
                        }
                        canvas_state.request_text_redraw();
                    },
                    _ => {
                        let (draw_mode, draw_status) = get_draw_mode_and_status(&widget);
                        match draw_status {
                            IpgDrawStatus::Completed => {
                                widget = set_widget_mode_or_status(widget, Some(IpgDrawMode::Display), None);
                            },
                            IpgDrawStatus::Delete => {
                                let id = get_widget_id(&widget);
                                canvas_state.curves.remove(&id);
                            },  
                            _ => (),
                        }
                        if draw_mode == IpgDrawMode::New {
                            app_state.last_id +=1;
                            let widget = set_widget_mode_or_status(widget.clone(), Some(IpgDrawMode::Display), Some(IpgDrawStatus::Completed));
                            canvas_state.curves.insert(app_state.last_id, widget);
                        } else {
                            // if not new must be in edit or rotate mode so modify.
                            let id = get_widget_id(&widget);
                            canvas_state.edit_widget_id = Some(id);
                            canvas_state.curves.entry(id).and_modify(|k| *k= widget);
                        }
                        
                        canvas_state.request_redraw();
                    },
                }
            },
        }
}

#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgCanvasParam {
    Clear,
    CanvasColor,
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

// update only the canvas, not the propterties of the canvas widgets.
// see canvas_geometry_update
pub fn canvas_item_update(canvas_state: &mut IpgCanvasState,
                            item: PyObject,
                            value: PyObject,
                            )
{
    let update = try_extract_canvas_update(item);

    match update {
        IpgCanvasParam::Clear => {
            canvas_state.clear_curves();
        },
        IpgCanvasParam::CanvasColor => {
            let rgba = try_extract_rgba_color(value);
            canvas_state.selected_canvas_color = Some(Color::from(rgba));
            canvas_state.request_redraw();
            canvas_state.request_text_redraw();
        },
        IpgCanvasParam::DrawColor => {
            let rgba = try_extract_rgba_color(value);
            canvas_state.selected_draw_color = Color::from(rgba);
        },
        IpgCanvasParam::FilePath => {
            canvas_state.file_path = try_extract_string(value);
        },
        IpgCanvasParam::FillColor => {
            let rgba = try_extract_rgba_color(value);
            canvas_state.selected_fill_color = Some(Color::from(rgba));
        },
        IpgCanvasParam::DrawWidth => {
            let width = try_extract_f64(value) as f32;
            canvas_state.selected_width = width;
        },
        IpgCanvasParam::Mode => {
            canvas_state.draw_mode = try_extract_mode(value);
        },
        IpgCanvasParam::PolyPoints => {
            let input = try_extract_string(value);
            canvas_state.selected_poly_points = match input.parse::<usize>() {
                Ok(int) => int,
                Err(e) => panic!("PolyPoint input must be an integer, {}", e),
            }
        },
        IpgCanvasParam::Load => {
            let path = Path::new(&canvas_state.file_path);
            let data = fs::read_to_string(path).expect("Unable to read file");
            let widgets = serde_json::from_str(&data).expect("Unable to parse file");
            canvas_state.clear_curves();
            (canvas_state.curves, canvas_state.text_curves, canvas_state.last_id) = import_widgets(widgets, canvas_state.last_id);
            canvas_state.request_redraw();
            canvas_state.request_text_redraw();
        },
        IpgCanvasParam::Save => {
            let path = Path::new(&canvas_state.file_path);
            let widgets = convert_to_export(&canvas_state.curves, &canvas_state.text_curves);
            let result = save(path, &widgets);
            match result {
                Ok(_) => (),
                Err(e) => println!("Unable to save file, {}", e),
            }
        },
        IpgCanvasParam::TextAlignment => {
            let align = try_extract_ipg_horizontal_alignment(value.clone());
            if align.is_some() {
                canvas_state.selected_h_text_alignment = get_horizontal_alignment(align)
            }
            let align = try_extract_ipg_vertical_alignment(value.clone());
            if align.is_some() {
                canvas_state.selected_v_text_alignment = get_vertical_alignment(align);
            }
        },
        IpgCanvasParam::Widget => {
            let selected_widget = Some(try_extract_widget(value));
            canvas_state.selected_widget = selected_widget;
            canvas_state.timer_event_enabled = selected_widget == Some(IpgCanvasWidget::Text);
        },
    }
}

pub fn try_extract_canvas_update(update_obj: PyObject) -> IpgCanvasParam {
    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgCanvasParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Canvas update extraction failed"),
        }
    })
}

fn try_extract_mode(update_obj: PyObject) -> IpgDrawMode {
    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgDrawMode>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Canvas mode update extraction failed"),
        }
    })
}

fn try_extract_widget(update_obj: PyObject) -> IpgCanvasWidget {
    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgCanvasWidget>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Canvas widget update extraction failed"),
        }
    })
}


#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgCanvasGeometryParam {
    Position,
    Rotation,
}

pub fn match_canvas_widget(widget: &mut IpgWidget, item: PyObject, value: PyObject) {

    let update_item = try_extract_geometry_update(item);

    match widget {
        IpgWidget::None => (),
        IpgWidget::Arc(arc) => {

        },
        IpgWidget::Bezier(bz) => {

        },
        IpgWidget::Circle(cir) => {
            match update_item {
                IpgCanvasGeometryParam::Position => {
                    let val = try_extract_point(value);
                    cir.center = Point::from(val);
                },
                IpgCanvasGeometryParam::Rotation => {
                    panic!("Circle update has not rotation property")
                },
            }
        },
        IpgWidget::Ellipse(ell) => {

        }
        IpgWidget::Image(img) => {
            match update_item {
                IpgCanvasGeometryParam::Position => {
                    let val = try_extract_point(value);
                    img.position = Point::from(val);
                },
                IpgCanvasGeometryParam::Rotation => {
                    let val = try_extract_f64(value) as f32;
                    img.rotation = val;
                },
            }
        },
        IpgWidget::Line(line) => {

        }
        IpgWidget::PolyLine(pl) => {

        },
        IpgWidget::Polygon(pg) => {

        },
        IpgWidget::Rectangle(ect) => {

        },
        IpgWidget::RightTriangle(tr) => {

        },
        IpgWidget::Text(txt) => {

        },
        IpgWidget::FreeHand(fh) => {

        },
    }
}




pub fn try_extract_geometry_update(update_obj: PyObject) -> IpgCanvasGeometryParam {
    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgCanvasGeometryParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Canvas update extraction failed"),
        }
    })
}