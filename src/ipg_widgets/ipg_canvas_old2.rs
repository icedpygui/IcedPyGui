use iced::keyboard::key;
use iced::widget::container;
use iced::{keyboard, mouse, Length, Size};
use iced::widget::canvas::event::{self, Event};
use iced::widget::canvas::{self, Canvas, Frame, Geometry, Path, Stroke};
use iced::{Element, Fill, Point, Rectangle, Renderer, Theme};

use pyo3::{pyclass, PyObject, Python};
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;
use serde_json;
use std::fs::File;
use std::fs;
use std::io::{BufWriter, Write};

use crate::app::Message;
use crate::graphics::colors::IpgColor;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[pyclass]
pub enum IpgCanvasMode {
    #[default]
    New,
    Edit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum CurveStatus {
    #[default]
    New,
    InProgress,
    Complete,
    Edit,
}

#[derive(Debug, Clone)]
pub struct IpgCanvas {
    pub id: usize,
    pub width: Length,
    pub height: Length,
    pub show: bool,
}

impl IpgCanvas {
    pub fn new(
        id: usize,
        width: Length,
        height: Length,
        show: bool,
    ) -> Self {
        Self {
            id,
            width,
            height,
            show, 
        }
    }
}

#[derive(Debug, Clone)]
pub enum IpgGeometry {
    None,
    IpgArc(IpgArc),
    IpgBezier(IpgBezier),
    IpgCircle(IpgCircle),
    IpgEllipse(IpgEllipse),
    IpgLine(IpgLine),
    IpgPolygon(IpgPolygon),
    IpgRectangle(IpgRectangle),
    IpgTriangle(IpgTriangle),
    IpgRightTriangle(IpgRightTriangle),
}

pub fn construct_canvas<'a>(state: &'a IpgBuildCanvas) -> Element<'a, Message>{
    let draw: Element<CanvasMessage> =  
            container(state.view(&state.curves)
                .map(CanvasMessage::CanvasNew)).into();
    draw.map(move |message| Message::Canvas(message))
}

#[derive(Default)]
pub struct IpgBuildCanvas {
    cache: canvas::Cache,
    pub curves: Vec<DrawCurve>,
    pub mode: IpgCanvasMode,
    pub selection: IpgCanvasWidget,
    pub escape_pressed: bool,
    pub curve_index_to_edit: Option<usize>,
    pub polygon_number: i8,
    pub curve_editing: Option<DrawCurve>,
    pub status: CurveStatus,
}

impl IpgBuildCanvas {
    pub fn view<'a>(&'a self, curves: &'a [DrawCurve]) -> Element<'a, DrawCurve> {
        Canvas::new(DrawOending {
            state: self,
            curves,
        })
        .width(Fill)
        .height(Fill)
        .into()
    }

    pub fn request_redraw(&mut self) {
        self.cache.clear();
    }
}

pub struct DrawOending<'a> {
    state: &'a IpgBuildCanvas,
    curves: &'a [DrawCurve],
}

impl<'a> canvas::Program<DrawCurve> for DrawOending<'a> {
    type State = ();

    fn update(
        &self,
        _state: &mut Self::State,
        event: Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (event::Status, Option<DrawCurve>) {
        let Some(mut cursor_position) = cursor.position_in(bounds) else {
            return (event::Status::Ignored, None);
        };

        match event {
            Event::Mouse(mouse_event) => {
                let mut return_curve = DrawCurve::default();
                match mouse_event {
                    mouse::Event::CursorMoved { position } => {
                        if self.state.mode == IpgCanvasMode::Edit && self.state.curve_editing.is_some() {
                            return_curve = self.state.curve_editing.as_ref().unwrap().clone();
                            let point_index = return_curve.edit_point_index.unwrap();
                            if return_curve.curve_type != IpgCanvasWidget::Polygon {
                                match point_index {
                                    0 => return_curve.from = Some(position),
                                    1 => return_curve.to = Some(position),
                                    2 => return_curve.control = Some(position),
                                    _ => (),
                                }
                            } else if return_curve.curve_type != IpgCanvasWidget::Polygon {
                                return_curve.points[point_index] = position;
                            }
                        }
                    }
                    mouse::Event::ButtonPressed(mouse::Button::Left) => {
                        match self.state.status {
                            CurveStatus::New => {
                               dbg!("button press new");
                            },
                            CurveStatus::InProgress => {
                                dbg!("button press in progress", &self.state.curves.last());
                                let curve = self.state.curves.last();
                                if curve.is_some() {
                                    return_curve = curve.unwrap().clone();
                                }
                            },
                            CurveStatus::Complete => (),
                            CurveStatus::Edit => {
                                for (index, curve) in self.state.curves.iter().enumerate() {
                                    if curve.from.unwrap().distance(cursor_position) < 5.0 {
                                        return_curve = curve.clone();
                                        return_curve.status = CurveStatus::Edit;
                                        return_curve.edit_point_index = Some(0);
                                        return_curve.edit_index = Some(index);
                                    } else if curve.to.unwrap().distance(cursor_position) < 5.0 {
                                        return_curve = curve.clone();
                                        return_curve.status = CurveStatus::Edit;
                                        return_curve.edit_point_index = Some(1);
                                        return_curve.edit_index = Some(index);
                                    } else if curve.to.unwrap().distance(cursor_position) < 5.0 {
                                        return_curve = curve.clone();
                                        return_curve.status = CurveStatus::Edit;
                                        return_curve.edit_point_index = Some(2);
                                        return_curve.edit_index = Some(index);
                                    } else if curve.curve_type == IpgCanvasWidget::Polygon {
                                        for (pt_index, point) in curve.points.iter().enumerate() {
                                            if point.distance(cursor_position) < 5.0 {
                                                return_curve = curve.clone();
                                                return_curve.status = CurveStatus::Edit;
                                                return_curve.edit_point_index = Some(pt_index);
                                                return_curve.edit_index = Some(index);
                                            }
                                        }
                                    }
                                };
                            },
                            
                            
                        }
                        
                        match self.state.selection {
                            IpgCanvasWidget::None => (),
                            IpgCanvasWidget::Bezier => {
                                if return_curve.from.is_none() {
                                    return_curve.curve_type = IpgCanvasWidget::Bezier;
                                    return_curve.status = CurveStatus::InProgress;
                                    return_curve.from = Some(cursor_position);
                                } else if return_curve.to.is_none() {
                                    return_curve.to = Some(cursor_position);
                                } else if return_curve.control.is_none() {
                                    return_curve.status = CurveStatus::Complete;
                                    return_curve.control = Some(cursor_position);
                                }
                            },
                            IpgCanvasWidget::Circle => {
                                if return_curve.from.is_none() {
                                    return_curve.curve_type = IpgCanvasWidget::Circle;
                                    return_curve.status = CurveStatus::InProgress;
                                    return_curve.from = Some(cursor_position);
                                } else if return_curve.to.is_none() {
                                    return_curve.to = Some(cursor_position);
                                    return_curve.status = CurveStatus::Complete;
                                }
                            },
                            IpgCanvasWidget::Line => {
                                if return_curve.from.is_none() {
                                    dbg!("from");
                                    return_curve.curve_type = IpgCanvasWidget::Line;
                                    return_curve.status = CurveStatus::InProgress;
                                    return_curve.from = Some(cursor_position);
                                } else if return_curve.to.is_none() {
                                    dbg!("to");
                                    return_curve.to = Some(cursor_position);
                                    return_curve.status = CurveStatus::Complete;
                                }
                            },
                            IpgCanvasWidget::Polygon => {
                                if return_curve.from.is_none() {
                                    return_curve.curve_type = IpgCanvasWidget::Polygon;
                                    return_curve.status = CurveStatus::InProgress;
                                    return_curve.from = Some(cursor_position);
                                } else if return_curve.to.is_none() {
                                    return_curve.to = Some(cursor_position);
                                    return_curve.status = CurveStatus::Complete;
                                
                                    let n = self.state.polygon_number;
                                    let angle = 0.0-PI/n as f64;
                                    let center = return_curve.from.unwrap();
                                    let to = return_curve.to.unwrap();
                                    let radius = center.distance(to) as f64;
                                    let mut points = vec![];
                                    let pi_2_n = 2.0*PI/n as f64;
                                    for i in 0..n {
                                        let x = center.x as f64 + radius * (pi_2_n * i as f64 - angle).sin();
                                        let y = center.y as f64 + radius * (pi_2_n * i as f64 - angle).cos();
                                        points.push(Point { x: x as f32, y: y as f32 });
                                    }
                                    
                                    return_curve.points = points.clone();                
                                }                
                            }
                            IpgCanvasWidget::Rectangle => {
                                if return_curve.from.is_none() {
                                    return_curve.curve_type = IpgCanvasWidget::Rectangle;
                                    return_curve.status = CurveStatus::InProgress;
                                    return_curve.from = Some(cursor_position);
                                } else if return_curve.to.is_none() {
                                    return_curve.to = Some(cursor_position);
                                    return_curve.status = CurveStatus::Complete;
                                }
                            },
                            IpgCanvasWidget::RightTriangle => {
                                if return_curve.from.is_none() {
                                    return_curve.curve_type = IpgCanvasWidget::RightTriangle;
                                    return_curve.status = CurveStatus::InProgress;
                                    return_curve.from = Some(cursor_position);
                                } else if return_curve.to.is_none() {
                                    cursor_position.x = return_curve.from.unwrap().x;
                                    return_curve.to = Some(cursor_position);
                                } else if return_curve.control.is_none() {
                                    return_curve.status = CurveStatus::Complete;
                                    cursor_position.y = return_curve.to.unwrap().y;
                                    return_curve.control= Some(cursor_position);
                                }
                            },
                            IpgCanvasWidget::Triangle => {
                                if return_curve.from.is_none() {
                                    return_curve.curve_type = IpgCanvasWidget::Triangle;
                                    return_curve.status = CurveStatus::InProgress;
                                    return_curve.from = Some(cursor_position);
                                } else if return_curve.to.is_none() {
                                    return_curve.to = Some(cursor_position);
                                } else if return_curve.control.is_none() {
                                    return_curve.status = CurveStatus::Complete;
                                    return_curve.control = Some(cursor_position);
                                }
                            },
                            IpgCanvasWidget::Ellipse => todo!(),
                        }
                    }
                    _ => (),
                };

                if return_curve.from.is_none() {
                    (event::Status::Captured, None)
                } else {
                    (event::Status::Captured, Some(return_curve))
                }
                
            }
            _ => (event::Status::Captured, None),
        }
    }

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        // dbg!("draw_all", &self.curves);
        let content =
            self.state.cache.draw(renderer, bounds.size(), |frame| {
                DrawCurve::draw_all(&self.state.curves, frame, theme, self.state.mode);
                frame.stroke(
                    &Path::rectangle(Point::ORIGIN, frame.size()),
                    Stroke::default()
                        .with_width(2.0)
                        .with_color(theme.palette().text),
                );
            });

        
            vec![content]
        
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        if cursor.is_over(bounds) {
            mouse::Interaction::Crosshair
        } else {
            mouse::Interaction::default()
        }
    }
}

#[derive(Debug, Clone)]
pub struct DrawCurve {
    pub curve_type: IpgCanvasWidget,
    pub mode: IpgCanvasMode,
    pub status: CurveStatus,
    pub from: Option<Point>,
    pub to: Option<Point>,
    pub control: Option<Point>,
    pub points: Vec<Point>,
    pub closed: bool,
    pub color: Option<IpgColor>,
    pub fill: bool,
    pub width: f32,
    pub edit_point_index: Option<usize>,
    pub edit_index: Option<usize>,
}
    

impl Default for DrawCurve {
    fn default() -> Self {
        DrawCurve {
            curve_type: IpgCanvasWidget::None,
            mode: IpgCanvasMode::New,
            status: CurveStatus::InProgress,
            from: None,
            to: None,
            control: None,
            points: vec![],
            closed: false,
            color: None,
            fill: false,
            width: 2.0,
            edit_point_index: None,
            edit_index: None,
        }
    }
}

impl DrawCurve {
    fn draw_all(curves: &Vec<DrawCurve>, frame: &mut Frame, theme: &Theme, mode: IpgCanvasMode) {
        let curves = Path::new(|p| {
            for (_index, curve) in curves.iter().enumerate() {
                if curve.from.is_some() && curve.to.is_none() && curve.control.is_none() {
                    p.circle(curve.from.unwrap(), 2.0);
                } else {
                    let from = curve.from.unwrap();
                    match curve.curve_type {
                        IpgCanvasWidget::None => p.move_to(Point::ORIGIN),
                        IpgCanvasWidget::Bezier => {
                            if curve.from.is_some() && curve.control.is_some() {
                                if mode == IpgCanvasMode::Edit {
                                    p.circle(from, 2.0);
                                    p.circle(curve.to.unwrap(), 2.0);
                                    p.circle(curve.control.unwrap(), 2.0);
                                }
                                p.move_to(from);
                                p.quadratic_curve_to(curve.control.unwrap(), curve.to.unwrap());
                            } else {
                                // just draw a line if not complete
                                p.move_to(curve.from.unwrap());
                                p.line_to(curve.to.unwrap());
                            }
                        },
                        IpgCanvasWidget::Circle => {
                            if mode == IpgCanvasMode::Edit {
                                    p.circle(from, 2.0);
                                    p.circle(curve.to.unwrap(), 2.0);
                            }
                            let radius = from.distance(curve.to.unwrap());
                            p.circle(from, radius);
                        },
                        IpgCanvasWidget::Line => {
                            if mode == IpgCanvasMode::Edit {
                                    p.circle(from, 2.0);
                                    p.circle(curve.to.unwrap(), 2.0);
                            }
                            p.move_to(from);
                            p.line_to(curve.to.unwrap());
                        },
                        IpgCanvasWidget::Polygon => {
                            if mode == IpgCanvasMode::Edit {
                                    p.circle(from, 2.0);
                                    p.circle(curve.to.unwrap(), 2.0);
                            }
                            p.move_to(curve.points[0]);
                            for point in curve.points.iter() {
                                p.line_to(point.clone());
                            }
                            p.line_to(curve.points[0]);
                        },
                        IpgCanvasWidget::Rectangle => {
                            if mode == IpgCanvasMode::Edit {
                                    p.circle(from, 2.0);
                                    p.circle(curve.to.unwrap(), 2.0);
                            }
                            let to = curve.to.unwrap();
                            let width = (to.x-from.x).abs();
                            let height = (to.y-from.y).abs();
                            let size = Size{ width, height };

                            let top_left = if from.x < to.x && from.y > to.y {
                                // top right
                                Point{ x: from.x, y: from.y-height }
                            } else if from.x > to.x && from.y > to.y {
                                // top_left
                                Point{x: from.x-width, y: to.y}
                            } else if from.x > to.x  && from.y < to.y {
                                // bottom left
                                Point{ x: to.x, y: from.y }
                            } else if from.x < to.x  && from.y < to.y {
                                // bottom right
                                from
                            } else {
                                to
                            };
                            p.rectangle(top_left, size);
                        },
                        IpgCanvasWidget::Triangle => {
                            if mode == IpgCanvasMode::Edit {
                                    p.circle(from, 2.0);
                                    p.circle(curve.to.unwrap(), 2.0);
                                    p.circle(curve.control.unwrap(), 2.0);
                            }
                            p.move_to(from);
                            p.line_to(curve.to.unwrap());
                            if curve.control.is_some() {
                                let point = curve.control.unwrap();
                                p.line_to(point);
                                p.line_to(from);
                            }
                        },
                        IpgCanvasWidget::RightTriangle => {
                            if mode == IpgCanvasMode::Edit {
                                    p.circle(from, 2.0);
                                    p.circle(curve.to.unwrap(), 2.0);
                                    p.circle(curve.control.unwrap(), 2.0);
                            }
                            p.move_to(from);
                            p.line_to(curve.to.unwrap());
                            if curve.control.is_some() {
                                let point = curve.control.unwrap();
                                p.line_to(point);
                                p.line_to(from);
                            }
                        },
                        IpgCanvasWidget::Ellipse => todo!(),
                    }
                }
            }
        });

        frame.stroke(
            &curves,
            Stroke::default()
                .with_width(2.0)
                .with_color(theme.palette().text),
        );
    }
}

#[derive(Debug, Clone)]
pub enum CanvasMessage {
    CanvasNew(DrawCurve),
    Clear,
    DeleteLast,
    Edit,
    PolygonSides(i8),
    RadioSelected(IpgCanvasWidget),
    Event(Event),
    Load,
    Save,
}

pub fn canvas_callback(canvas_message: CanvasMessage, canvas_state: &mut IpgBuildCanvas) {
    dbg!(&canvas_message);
    match canvas_message {
            CanvasMessage::CanvasNew(curve) => {
                match canvas_state.status {
                    CurveStatus::New => {
                        canvas_state.curves.push(curve);
                        let last = canvas_state.curves.last_mut().unwrap();
                        last.status = CurveStatus::InProgress;
                        dbg!(&canvas_state.curves);
                    }
                    CurveStatus::Edit => {
                        // add new curve
                        canvas_state.curves[curve.edit_index.unwrap()] = curve.clone();
                        canvas_state.curve_editing = Some(curve);
                    }
                    CurveStatus::InProgress => {
                        // replace last curve
                        let last = canvas_state.curves.last_mut().unwrap();
                        *last = curve;
                    },
                    CurveStatus::Complete => todo!(),
                }   
                canvas_state.request_redraw();
            }
            CanvasMessage::Clear => {
                // canvas_state = IpgBuildCanvasNew::default();
                canvas_state.curves.clear();
            }
            CanvasMessage::DeleteLast => {
                if canvas_state.curves.is_empty() {
                    return
                }
                canvas_state.curves.remove(canvas_state.curves.len()-1);
                canvas_state.request_redraw();
            }
            CanvasMessage::Edit => {
                if canvas_state.curves.is_empty() {
                    return
                }
                
                canvas_state.mode = IpgCanvasMode::Edit;
                
                canvas_state.request_redraw();
            }
            CanvasMessage::PolygonSides(sides) => {
                canvas_state.polygon_number = sides;
            },
            CanvasMessage::RadioSelected(choice) => {
                canvas_state.selection = choice; 
            },
            CanvasMessage::Event(Event::Keyboard(keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(key::Named::Escape),
                ..
            })) => { 
                canvas_state.escape_pressed = true;
            },
            CanvasMessage::Event(Event::Keyboard(keyboard::Event::KeyReleased {
                key: keyboard::Key::Named(key::Named::Escape),
                ..
            })) => { 
                canvas_state.escape_pressed = false;
            },
            CanvasMessage::Event(_) => (),
            CanvasMessage::Load => {
                let path = std::path::Path::new("./resources/data.json");
                let data = fs::read_to_string(path).expect("Unable to read file");
                let curves = serde_json::from_str(&data).expect("Unable to parse");
                canvas_state.curves = convert_to_iced_point(curves);
                canvas_state.request_redraw();
            }
            CanvasMessage::Save => {
                let path = std::path::Path::new("./resources/data.json");
                let curves = convert_to_ipg_point(&canvas_state.curves);
                let _ = save(path, &curves);
            }
        }
}

pub fn save(path: impl AsRef<std::path::Path>, data: &impl Serialize) -> std::io::Result<()> {
    let mut w = BufWriter::new(File::create(path).expect("Unable to create file"));
    serde_json::to_writer_pretty(&mut w, data).expect("serde_json failed");
    w.write(b"\n").expect_err("Unable to append \n on buffer");
    w.flush().expect_err("Unable to flush buffer");
    Ok(())
}

#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgCanvasParam {
    Mode,
    Widget,
}

pub fn canvas_item_update(canvas_state: &mut IpgBuildCanvas,
                            item: PyObject,
                            value: PyObject,
                            )
{
    let update = try_extract_canvas_update(item);

    match update {
        IpgCanvasParam::Mode => {
            canvas_state.mode = try_extract_mode(value);
        },
        IpgCanvasParam::Widget => {
            canvas_state.selection = try_extract_widget(value);
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

fn try_extract_mode(update_obj: PyObject) -> IpgCanvasMode {
    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgCanvasMode>(py);
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


// iced Point does not derive any serialization 
// so had to use own version for saving data.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct IpgPoint{
    x: f32,
    y: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpgDrawCurve {
    pub curve_type: IpgCanvasWidget,
    pub from: IpgPoint,
    pub to: IpgPoint,
    pub control: Option<IpgPoint>,
    pub points: Vec<IpgPoint>,
}

fn convert_to_iced_point(curves: Vec<IpgDrawCurve>) -> Vec<DrawCurve> {
    let mut iced_curves = vec![];
    for curve in curves {
        let from = to_point(curve.from);
        let to = to_point(curve.to);
        let control: Option<Point> = 
            match curve.control {
                Some(ctrl) => Some(to_point(ctrl)),
                None => None,
            };

        let mut points: Vec<Point> = vec![];
        for point in curve.points {
            points.push(to_point(point));
        }

        iced_curves.push(DrawCurve { curve_type: curve.curve_type, 
                                    status: CurveStatus::Complete, 
                                    from: Some(from), 
                                    to: Some(to), 
                                    points, 
                                    control, 
                                    ..Default::default() 
                                });
    }
    iced_curves
}

fn convert_to_ipg_point(curves: &Vec<DrawCurve>) -> Vec<IpgDrawCurve> {
    let mut ipg_curves = vec![];
    for curve in curves {
        let from = to_ipg_point(curve.from.unwrap());
        let to = to_ipg_point(curve.to.unwrap());
        let control: Option<IpgPoint> = 
            match curve.control {
                Some(ctrl) => Some(to_ipg_point(ctrl)),
                None => None,
            };

        let mut points = vec![];
        for point in curve.points.clone() {
            points.push(to_ipg_point(point));
        }
        
        ipg_curves.push(IpgDrawCurve { curve_type: curve.curve_type, from, to, control, points });
    }
    ipg_curves
}

fn to_point(ipg_point: IpgPoint) -> Point {
    Point { x: ipg_point.x, y: ipg_point.y }
}
fn to_ipg_point(point: Point) -> IpgPoint {
    IpgPoint { x: point.x, y: point.y }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[pyclass]
pub enum IpgCanvasWidget {
    #[default]
    None,
    Bezier,
    Circle,
    Ellipse,
    Line,
    Polygon,
    Rectangle,
    RightTriangle,
    Triangle,
}

#[derive(Debug, Clone)]
pub struct IpgArc {
    pub id: usize,
    pub canvas_id: String,
    pub center: Point,
    pub radius: f32,
    pub start_angle: f32,
    pub end_angle: f32,
    pub stroke_width: f32,
    pub color: Option<IpgColor>,
    pub fill: bool,
    pub show: bool,
}

impl IpgArc {
    pub fn new(
        id: usize,
        canvas_id: String,
        center: Point,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        stroke_width: f32,
        color: Option<IpgColor>,
        fill: bool,
        show: bool,
    ) -> Self {
        Self {
            id,
            canvas_id,
            center,
            radius,
            start_angle,
            end_angle,
            stroke_width,
            color,
            fill,
            show, 
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct IpgBezier {
    pub id: usize,
    pub canvas_id: String,
    pub points: (Point, Point, Point),
    pub stroke_width: f32,
    pub color: Option<IpgColor>,
    pub fill: bool,
    pub show: bool,
}

impl IpgBezier {
    pub fn new(
        id: usize,
        canvas_id: String,
        points: (Point, Point, Point),
        stroke_width: f32,
        color: Option<IpgColor>,
        fill: bool,
        show: bool,
    ) -> Self {
        Self {
            id,
            canvas_id,
            points,
            stroke_width,
            color,
            fill,
            show, 
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct IpgCircle {
    pub id: usize,
    pub canvas_id: String,
    pub center: Point,
    pub radius: f32,
    pub stroke_width: f32,
    pub color: Option<IpgColor>,
    pub fill: bool,
    pub show: bool,
}

impl IpgCircle {
    pub fn new(
        id: usize,
        canvas_id: String,
        center: Point,
        radius: f32,
        stroke_width: f32,
        color: Option<IpgColor>,
        fill: bool,
        show: bool,
    ) -> Self {
        Self {
            id,
            canvas_id,
            center,
            radius,
            stroke_width,
            color,
            fill,
            show, 
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct IpgEllipse {
    pub id: usize,
    pub canvas_id: String,
    pub center: Point,
    pub radii: Point,
    pub rotation: f32,
    pub start_angle: f32,
    pub end_angle: f32,
    pub stroke_width: f32,
    pub color: Option<IpgColor>,
    pub fill: bool,
    pub show: bool,
}

impl IpgEllipse {
    pub fn new(
        id: usize,
        canvas_id: String,
        center: Point,
        radii: Point,
        rotation: f32,
        start_angle: f32,
        end_angle: f32,
        stroke_width: f32,
        color: Option<IpgColor>,
        fill: bool,
        show: bool,
    ) -> Self {
        Self {
            id,
            canvas_id,
            center,
            radii,
            rotation,
            start_angle,
            end_angle,
            stroke_width,
            color,
            fill,
            show, 
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct IpgLine {
    pub id: usize,
    pub canvas_id: String,
    pub points: [Point; 2],
    pub stroke_width: f32,
    pub color: Option<IpgColor>,
    pub show: bool,
}

impl IpgLine {
    pub fn new(
        id: usize,
        canvas_id: String,
        points: [Point; 2],
        stroke_width: f32,
        color: Option<IpgColor>,
        show: bool,
    ) -> Self {
        Self {
            id,
            canvas_id,
            points,
            stroke_width,
            color,
            show, 
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct IpgPolygon {
    pub id: usize,
    pub canvas_id: String,
    pub points: Vec<Point>,
    pub stroke_width: f32,
    pub color: Option<IpgColor>,
    pub fill: bool,
    pub show: bool,
}

impl IpgPolygon {
    pub fn new(
        id: usize,
        canvas_id: String,
        points: Vec<Point>,
        stroke_width: f32,
        color: Option<IpgColor>,
        fill: bool,
        show: bool,
    ) -> Self {
        Self {
            id,
            canvas_id,
            points,
            stroke_width,
            color,
            fill,
            show,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct IpgRectangle {
    pub id: usize,
    pub canvas_id: String,
    pub top_left: Point,
    pub width: f32,
    pub height: f32,
    pub stroke_width: f32,
    pub color: Option<IpgColor>,
    pub fill: bool,
    pub show: bool,
}

impl IpgRectangle {
    pub fn new(
        id: usize,
        canvas_id: String,
        top_left: Point,
        width: f32,
        height: f32,
        stroke_width: f32,
        color: Option<IpgColor>,
        fill: bool,
        show: bool,
    ) -> Self {
        Self {
            id,
            canvas_id,
            top_left,
            width,
            height,
            stroke_width,
            color,
            fill,
            show, 
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct IpgTriangle {
    pub id: usize,
    pub canvas_id: String,
    pub points: [Point; 3],
    pub stroke_width: f32,
    pub color: Option<IpgColor>,
    pub fill: bool,
    pub show: bool,
}

impl IpgTriangle {
    pub fn new(
        id: usize,
        canvas_id: String,
        points: [Point; 3],
        stroke_width: f32,
        color: Option<IpgColor>,
        fill: bool,
        show: bool,
    ) -> Self {
        Self {
            id,
            canvas_id,
            points,
            stroke_width,
            color,
            fill,
            show,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct IpgRightTriangle {
    pub id: usize,
    pub canvas_id: String,
    pub points: [Point; 3],
    pub stroke_width: f32,
    pub color: Option<IpgColor>,
    pub fill: bool,
    pub show: bool,
}

impl IpgRightTriangle {
    pub fn new(
        id: usize,
        canvas_id: String,
        points: [Point; 3],
        stroke_width: f32,
        color: Option<IpgColor>,
        fill: bool,
        show: bool,
    ) -> Self {
        Self {
            id,
            canvas_id,
            points,
            stroke_width,
            color,
            fill,
            show,
        }
    }
}
