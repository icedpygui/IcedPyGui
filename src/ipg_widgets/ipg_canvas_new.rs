use iced::keyboard::key;
use iced::widget::container;
use iced::{keyboard, mouse, Length, Size};
use iced::widget::canvas::event::{self, Event};
use iced::widget::canvas::{self, Canvas, Frame, Geometry, Path, Stroke};
use iced::{Element, Fill, Point, Rectangle, Renderer, Theme};

use serde::{Deserialize, Serialize};
use std::f64::consts::PI;
use serde_json;
use std::fs::File;
use std::fs;
use std::io::{BufWriter, Write};

use crate::app::Message;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Choice {
    #[default]
    None,
    Bezier,
    Circle,
    Line,
    Polygon,
    Rectangle,
    RightTriangle,
    Triangle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Mode {
    #[default]
    New,
    Edit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum CurveStatus {
    #[default]
    InProgress,
    Complete,
    Edit,
}

#[derive(Debug, Clone)]
pub struct IpgCanvasNew {
    pub id: usize,
    pub width: Length,
    pub height: Length,
    pub show: bool,
}

impl IpgCanvasNew {
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

pub fn construct_canvas_new<'a>(state: &'a IpgBuildCanvasNew) -> Element<'a, Message>{
    let draw: Element<CanvasMessageNew> =  
            container(state.view(&state.curves)
                .map(CanvasMessageNew::CanvasNew)).into();
    draw.map(move |message| Message::CanvasNew(message))

}

#[derive(Default)]
pub struct IpgBuildCanvasNew {
    cache: canvas::Cache,
    pub curves: Vec<DrawCurveNew>,
    pub mode: Mode,
    pub selection: Choice,
    pub escape_pressed: bool,
    pub curve_index_to_edit: Option<usize>,
    pub polygon_number: i8,
    pub curve_editing: Option<DrawCurveNew>,
}

impl IpgBuildCanvasNew {
    pub fn view<'a>(&'a self, curves: &'a [DrawCurveNew]) -> Element<'a, DrawCurveNew> {
        Canvas::new(Draw {
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

   pub struct Draw<'a> {
        state: &'a IpgBuildCanvasNew,
        curves: &'a [DrawCurveNew],
    }

    impl<'a> canvas::Program<DrawCurveNew> for Draw<'a> {
        type State = ();

        fn update(
            &self,
            _state: &mut Self::State,
            event: Event,
            bounds: Rectangle,
            cursor: mouse::Cursor,
        ) -> (event::Status, Option<DrawCurveNew>) {
            let Some(mut cursor_position) = cursor.position_in(bounds) else {
                return (event::Status::Ignored, None);
            };

            match event {
                Event::Mouse(mouse_event) => {
                    let mut return_curve = DrawCurveNew::default();
                    match mouse_event {
                        mouse::Event::CursorMoved { position } => {
                            if self.state.mode == Mode::Edit && self.state.curve_editing.is_some() {
                                return_curve = self.state.curve_editing.as_ref().unwrap().clone();
                                let point_index = return_curve.edit_point_index.unwrap();
                                if return_curve.curve_type != Choice::Polygon {
                                    match point_index {
                                        0 => return_curve.from = Some(position),
                                        1 => return_curve.to = Some(position),
                                        2 => return_curve.control = Some(position),
                                        _ => (),
                                    }
                                } else if return_curve.curve_type != Choice::Polygon {
                                    return_curve.points[point_index] = position;
                                }
                            }
                        }
                        mouse::Event::ButtonPressed(mouse::Button::Left) => {
                            if self.state.mode == Mode::Edit {
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
                                    } else if curve.curve_type == Choice::Polygon {
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
                            }
                            if self.state.mode == Mode::New {
                               let last = self.curves.last();
                                match last {
                                    Some(curve) => {
                                        if curve.status == CurveStatus::InProgress {
                                            return_curve = curve.clone()
                                        }
                                    },
                                    None => ()
                                } 
                            }
                            
                            match self.state.selection {
                                Choice::None => (),
                                Choice::Bezier => {
                                    if return_curve.from.is_none() {
                                        return_curve.curve_type = Choice::Bezier;
                                        return_curve.status = CurveStatus::InProgress;
                                        return_curve.from = Some(cursor_position);
                                    } else if return_curve.to.is_none() {
                                        return_curve.to = Some(cursor_position);
                                    } else if return_curve.control.is_none() {
                                        return_curve.status = CurveStatus::Complete;
                                        return_curve.control = Some(cursor_position);
                                    }
                                },
                                Choice::Circle => {
                                    if return_curve.from.is_none() {
                                        return_curve.curve_type = Choice::Circle;
                                        return_curve.status = CurveStatus::InProgress;
                                        return_curve.from = Some(cursor_position);
                                    } else if return_curve.to.is_none() {
                                        return_curve.to = Some(cursor_position);
                                        return_curve.status = CurveStatus::Complete;
                                    }
                                },
                                Choice::Line => {
                                    dbg!("line");
                                    if return_curve.from.is_none() {
                                        return_curve.curve_type = Choice::Line;
                                        return_curve.status = CurveStatus::InProgress;
                                        return_curve.from = Some(cursor_position);
                                    } else if return_curve.to.is_none() {
                                        return_curve.to = Some(cursor_position);
                                        return_curve.status = CurveStatus::Complete;
                                    }
                                },
                                Choice::Polygon => {
                                    if return_curve.from.is_none() {
                                        return_curve.curve_type = Choice::Polygon;
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
                                Choice::Rectangle => {
                                    if return_curve.from.is_none() {
                                        return_curve.curve_type = Choice::Rectangle;
                                        return_curve.status = CurveStatus::InProgress;
                                        return_curve.from = Some(cursor_position);
                                    } else if return_curve.to.is_none() {
                                        return_curve.to = Some(cursor_position);
                                        return_curve.status = CurveStatus::Complete;
                                    }
                                },
                                Choice::RightTriangle => {
                                    if return_curve.from.is_none() {
                                        return_curve.curve_type = Choice::RightTriangle;
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
                                Choice::Triangle => {
                                    if return_curve.from.is_none() {
                                        return_curve.curve_type = Choice::Triangle;
                                        return_curve.status = CurveStatus::InProgress;
                                        return_curve.from = Some(cursor_position);
                                    } else if return_curve.to.is_none() {
                                        return_curve.to = Some(cursor_position);
                                    } else if return_curve.control.is_none() {
                                        return_curve.status = CurveStatus::Complete;
                                        return_curve.control = Some(cursor_position);
                                    }
                                },
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
                    DrawCurveNew::draw_all(self.curves, frame, theme, self.state.mode);
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
    pub struct DrawCurveNew {
        pub curve_type: Choice,
        pub status: CurveStatus,
        pub from: Option<Point>,
        pub to: Option<Point>,
        pub control: Option<Point>,
        pub points: Vec<Point>,
        pub edit_point_index: Option<usize>,
        pub edit_index: Option<usize>,
    }

    impl Default for DrawCurveNew {
        fn default() -> Self {
            DrawCurveNew {
                curve_type: Choice::None,
                status: CurveStatus::InProgress,
                from: None,
                to: None,
                control: None,
                points: vec![],
                edit_point_index: None,
                edit_index: None,
            }
        }
    }

    impl DrawCurveNew {
        fn draw_all(curves: &[DrawCurveNew], frame: &mut Frame, theme: &Theme, mode: Mode) {
            let curves = Path::new(|p| {
                for (_index, curve) in curves.iter().enumerate() {
                    if curve.from.is_some() && curve.to.is_none() && curve.control.is_none() {
                        p.circle(curve.from.unwrap(), 2.0);
                    } else {
                        let from = curve.from.unwrap();
                        match curve.curve_type {
                            Choice::None => p.move_to(Point::ORIGIN),
                            Choice::Bezier => {
                                if curve.from.is_some() && curve.control.is_some() {
                                    if mode == Mode::Edit {
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
                            Choice::Circle => {
                                if mode == Mode::Edit {
                                        p.circle(from, 2.0);
                                        p.circle(curve.to.unwrap(), 2.0);
                                }
                                let radius = from.distance(curve.to.unwrap());
                                p.circle(from, radius);
                            },
                            Choice::Line => {
                                if mode == Mode::Edit {
                                        p.circle(from, 2.0);
                                        p.circle(curve.to.unwrap(), 2.0);
                                }
                                p.move_to(from);
                                p.line_to(curve.to.unwrap());
                            },
                            Choice::Polygon => {
                                if mode == Mode::Edit {
                                        p.circle(from, 2.0);
                                        p.circle(curve.to.unwrap(), 2.0);
                                }
                                p.move_to(curve.points[0]);
                                for point in curve.points.iter() {
                                    p.line_to(point.clone());
                                }
                                p.line_to(curve.points[0]);
                            },
                            Choice::Rectangle => {
                                if mode == Mode::Edit {
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
                            Choice::Triangle => {
                                if mode == Mode::Edit {
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
                            Choice::RightTriangle => {
                                if mode == Mode::Edit {
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
pub enum CanvasMessageNew {
    CanvasNew(DrawCurveNew),
    Clear,
    DeleteLast,
    Edit,
    PolygonSides(i8),
    RadioSelected(Choice),
    Event(Event),
    Load,
    Save,
}

pub fn canvas_new_callback(canvas_message: CanvasMessageNew, canvas_state: &mut IpgBuildCanvasNew) {
    match canvas_message {
            CanvasMessageNew::CanvasNew(curve) => {
                match canvas_state.mode {
                    Mode::New => {
                        let last = canvas_state.curves.last();
                        if last.is_none() {
                            canvas_state.curves.push(curve);
                        } else if last.unwrap().status == CurveStatus::InProgress {
                            //replace last curve
                            let final_length = canvas_state.curves.len().saturating_sub(1);
                            canvas_state.curves.truncate(final_length);
                            canvas_state.curves.push(curve);
                        } else {
                            canvas_state.curves.push(curve);
                        }
                    }
                   Mode::Edit => {
                    canvas_state.curves[curve.edit_index.unwrap()] = curve.clone();
                    canvas_state.curve_editing = Some(curve);
                   }
                }   
                
                canvas_state.request_redraw();
            }
            CanvasMessageNew::Clear => {
                // canvas_state = IpgBuildCanvasNew::default();
                canvas_state.curves.clear();
            }
            CanvasMessageNew::DeleteLast => {
                if canvas_state.curves.is_empty() {
                    return
                }
                canvas_state.curves.remove(canvas_state.curves.len()-1);
                canvas_state.request_redraw();
            }
            CanvasMessageNew::Edit => {
                if canvas_state.curves.is_empty() {
                    return
                }
                
                canvas_state.mode = Mode::Edit;
                
                canvas_state.request_redraw();
            }
            CanvasMessageNew::PolygonSides(sides) => {
                canvas_state.polygon_number = sides;
            },
            CanvasMessageNew::RadioSelected(choice) => {
                canvas_state.selection = choice; 
            },
            CanvasMessageNew::Event(Event::Keyboard(keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(key::Named::Escape),
                ..
            })) => { 
                canvas_state.escape_pressed = true;
            },
            CanvasMessageNew::Event(Event::Keyboard(keyboard::Event::KeyReleased {
                key: keyboard::Key::Named(key::Named::Escape),
                ..
            })) => { 
                canvas_state.escape_pressed = false;
            },
            CanvasMessageNew::Event(_) => (),
            CanvasMessageNew::Load => {
                let path = std::path::Path::new("./resources/data.json");
                let data = fs::read_to_string(path).expect("Unable to read file");
                let curves = serde_json::from_str(&data).expect("Unable to parse");
                canvas_state.curves = convert_to_iced_point(curves);
                canvas_state.request_redraw();
            }
            CanvasMessageNew::Save => {
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

// iced Point does not derive any serialization 
// so had to use own version for saving data.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct IpgPoint{
    x: f32,
    y: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpgDrawCurve {
    pub curve_type: Choice,
    pub from: IpgPoint,
    pub to: IpgPoint,
    pub control: Option<IpgPoint>,
    pub points: Vec<IpgPoint>,
}

fn convert_to_iced_point(curves: Vec<IpgDrawCurve>) -> Vec<DrawCurveNew> {
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

        iced_curves.push(DrawCurveNew { curve_type: curve.curve_type, 
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

fn convert_to_ipg_point(curves: &Vec<DrawCurveNew>) -> Vec<IpgDrawCurve> {
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

