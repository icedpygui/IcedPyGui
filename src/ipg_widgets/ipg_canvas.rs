//! ipg_canvas
use iced::advanced::graphics::geometry::{Cache, Frame};
use iced::widget::canvas::path::arc::Elliptical;
use iced::widget::canvas::path::Arc;
use iced::widget::Canvas;
use iced::{mouse, Element, Length, Point, Radians, Size, Vector};
use iced::widget::canvas::{
    self, Fill, Geometry, Path, Stroke, Style
};
use iced::{Rectangle, Renderer, Theme};
use iced::widget::canvas::event::{self, Event};
use pyo3::pyclass;
use crate::{access_canvas_state, app, get_id};
use crate::app::Message;
use crate::graphics::colors::{match_ipg_color, IpgColor};

use serde::{Deserialize, Serialize};
use std::f64::consts::PI;


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

#[derive(Debug, Clone)]
pub enum CanvasMessage {
    AddCurve,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[pyclass]
pub enum IpgCanvasMode {
    New,
    Edit,
    Freehand,
    PicknPlace,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Mode {
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

#[derive(Debug, Default)]
pub struct IpgBuildCanvas {
    id: usize,
    cache: canvas::Cache,
}

pub fn construct_canvas(can: IpgCanvas) -> Element<'static, Message> {

    let canvas: Element<'static, Message>  = Canvas::new(IpgBuildCanvas {id: can.id, cache: Cache::default() })
        .width(can.width)
        .height(can.height)
        .into();

    canvas.map(move |_message| app::Message::Canvas)
}

impl canvas::Program<app::Message> for IpgBuildCanvas {
    type State = ();

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
    
    fn update(
            &self,
            _state: &mut Self::State,
            event: canvas::Event,
            bounds: Rectangle,
            cursor: mouse::Cursor,
        ) -> (iced::event::Status, Option<Message>) {
            let Some(mut cursor_position) = cursor.position_in(bounds) else {
                return (event::Status::Ignored, None);
            };

            let mut return_curve = DrawCurve::default();
            let mut canvas_state = access_canvas_state();

            match event {
                Event::Mouse(mouse_event) => {
                    match mouse_event {
                        mouse::Event::CursorMoved { position: _ } => {
                            return (event::Status::Ignored, None);
                            // match canvas_state.mode {
                            //     IpgCanvasMode::New => {
                                    
                            //     },
                            //     IpgCanvasMode::Edit => {
                            //         if canvas_state.curve_editing.is_some() {
                            //             return_curve = canvas_state.curve_editing.as_ref().unwrap().clone();
                            //             let point_index = return_curve.edit_point_index.unwrap();
                            //             if return_curve.curve_type != IpgCanvasWidget::Polygon {
                            //                 match point_index {
                            //                     0 => return_curve.from = Some(position),
                            //                     1 => return_curve.to = Some(position),
                            //                     2 => return_curve.control = Some(position),
                            //                     _ => (),
                            //                 }
                            //             } else if return_curve.curve_type != IpgCanvasWidget::Polygon {
                            //                 return_curve.points[point_index] = position;
                            //             }
                            //         }
                            //     },
                            //     IpgCanvasMode::Freehand => todo!(),
                            //     IpgCanvasMode::PicknPlace => todo!(),
                            // }
                            
                        }
                        mouse::Event::ButtonPressed(mouse::Button::Left) => {
                            match canvas_state.mode {
                                IpgCanvasMode::New => {
                                    if canvas_state.return_curve.status == CurveStatus::New {
                                        canvas_state.return_curve = DrawCurve::default();
                                        canvas_state.return_curve.curve_type = canvas_state.selection;
                                        canvas_state.return_curve.mode = IpgCanvasMode::New;
                                        canvas_state.return_curve.status = CurveStatus::InProgress;
                                    }
                                    
                                },
                                IpgCanvasMode::Edit => {
                                    let canvas_id = canvas_state.canvas_id.clone().unwrap();
                                    let curves: &Vec<IpgGeometry> = canvas_state.geometries.get(&canvas_id).unwrap();
                                    for (index, curve) in curves.iter().enumerate() {
                                        match curve {
                                            IpgGeometry::None => {

                                            },
                                            IpgGeometry::IpgArc(_arc) => {

                                            },
                                            IpgGeometry::IpgBezier(_bz) => {

                                            },
                                            IpgGeometry::IpgCircle(circle) => {
                                                let to = Point::new(circle.center.x+circle.radius, circle.center.y);
                                                let point_index = 
                                                    if circle.center.distance(cursor_position) < 5.0 {
                                                        Some(0)
                                                    } else if to.distance(cursor_position) < 5.0 {
                                                        Some(1)
                                                    } else {
                                                        None
                                                    };
                                                if point_index.is_some() {    
                                                    return_curve.from = Some(circle.center);
                                                    return_curve.to = Some(to);
                                                    return_curve.color = circle.color.clone();
                                                    return_curve.status = CurveStatus::Edit;
                                                    return_curve.edit_point_index = point_index;
                                                    return_curve.edit_index = Some(index);
                                                }
                                            },
                                            IpgGeometry::IpgEllipse(_ell) => {

                                            },
                                            IpgGeometry::IpgLine(line) => {
                                                let point_index = 
                                                    if line.points[0].distance(cursor_position) < 5.0 {
                                                        Some(0)
                                                    } else if line.points[1].distance(cursor_position) < 5.0 {
                                                        Some(1)
                                                    } else {
                                                        None
                                                    };
                                                if point_index.is_some() {    
                                                    return_curve.from = Some(line.points[0]);
                                                    return_curve.to = Some(line.points[1]);
                                                    return_curve.color = line.color.clone();
                                                    return_curve.status = CurveStatus::Edit;
                                                    return_curve.edit_point_index = point_index;
                                                    return_curve.edit_index = Some(index);
                                                }
                                            },
                                            IpgGeometry::IpgPolygon(_poly) => {

                                            },
                                            IpgGeometry::IpgRectangle(_rect) => {

                                            },
                                            IpgGeometry::IpgTriangle(_tri) => {

                                            },
                                            IpgGeometry::IpgRightTriangle(_r_tri) => {

                                            },
                                        }
                                        
                                    };
                                },
                                IpgCanvasMode::Freehand => {
                                    dbg!("Freehand");
                                },
                                IpgCanvasMode::PicknPlace => {
                                    dbg!("PicknPlace");
                                },
                            }
                            match canvas_state.return_curve.curve_type {
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
                                        // finalize the curve
                                        return_curve.status = CurveStatus::Complete;
                                    }
                                },
                                IpgCanvasWidget::Circle => {
                                    if return_curve.from.is_none() {
                                        return_curve.curve_type = IpgCanvasWidget::Circle;
                                        return_curve.status = CurveStatus::InProgress;
                                        return_curve.from = Some(cursor_position);
                                    } else if return_curve.to.is_none() {
                                        return_curve.to = Some(cursor_position);
                                        // finalize the curve
                                        return_curve.status = CurveStatus::Complete;
                                    }
                                },
                                IpgCanvasWidget::Ellipse => {
                                    
                                }
                                IpgCanvasWidget::Line => {
                                    dbg!("line");
                                    if canvas_state.return_curve.from.is_none() {
                                        canvas_state.return_curve.from = Some(cursor_position);
                                    } else if canvas_state.return_curve.to.is_none() {
                                        canvas_state.return_curve.to = Some(cursor_position);
                                        // finalize the curve
                                        canvas_state.return_curve.status = CurveStatus::Complete;
                                    }
                                    if canvas_state.return_curve.status != CurveStatus::Complete {
                                        drop(canvas_state);
                                        return (event::Status::Captured, Some(app::Message::Canvas))
                                    }
                                },
                                IpgCanvasWidget::Polygon => {
                                    if return_curve.from.is_none() {
                                        return_curve.curve_type = IpgCanvasWidget::Polygon;
                                        return_curve.status = CurveStatus::InProgress;
                                        return_curve.from = Some(cursor_position);
                                    } else if return_curve.to.is_none() {
                                        return_curve.to = Some(cursor_position);
                                    
                                        let n = canvas_state.polygon_number;
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
                                        // finalize the curve
                                        return_curve.status = CurveStatus::Complete;            
                                    }                
                                }
                                IpgCanvasWidget::Rectangle => {
                                    if return_curve.from.is_none() {
                                        return_curve.curve_type = IpgCanvasWidget::Rectangle;
                                        return_curve.status = CurveStatus::InProgress;
                                        return_curve.from = Some(cursor_position);
                                    } else if return_curve.to.is_none() {
                                        return_curve.to = Some(cursor_position);
                                        // finalize the curve
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
                                        cursor_position.y = return_curve.to.unwrap().y;
                                        return_curve.control= Some(cursor_position);
                                        // finalize the curve
                                        return_curve.status = CurveStatus::Complete;
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
                                        return_curve.control = Some(cursor_position);
                                        // finalize the curve
                                        return_curve.status = CurveStatus::Complete;
                                    }
                                },
                            }
                        }
                        _ => return (event::Status::Ignored, None),
                    };
                    if canvas_state.return_curve.status == CurveStatus::Complete {
                        let geometry: IpgGeometry = match return_curve.curve_type {
                            IpgCanvasWidget::None => IpgGeometry::None,
                            IpgCanvasWidget::Bezier => {
                                let mut bz = IpgBezier::default();
                                bz.id = get_id();
                                bz.canvas_id = canvas_state.canvas_id.clone().unwrap();
                                bz.points = (return_curve.from.unwrap(), return_curve.to.unwrap(), return_curve.control.unwrap());
                                bz.stroke_width = return_curve.width;
                                bz.color = return_curve.color;
                                bz.fill = return_curve.fill;
                                IpgGeometry::IpgBezier(bz)
                            },
                            IpgCanvasWidget::Circle => {
                                let mut circle = IpgCircle::default();
                                circle.id = get_id();
                                circle.canvas_id = canvas_state.canvas_id.clone().unwrap();
                                circle.center = return_curve.from.unwrap();
                                circle.radius = return_curve.from.unwrap().distance(return_curve.to.unwrap());
                                circle.stroke_width = return_curve.width;
                                circle.color = return_curve.color;
                                circle.fill = return_curve.fill;
                                IpgGeometry::IpgCircle(circle)
                            },
                            IpgCanvasWidget::Ellipse => {
                                let mut ell = IpgEllipse::default();
                                ell.id = get_id();
                                ell.canvas_id = canvas_state.canvas_id.clone().unwrap();
                                
                                IpgGeometry::IpgEllipse(ell)
                            }
                            IpgCanvasWidget::Line => {
                                let mut ln = IpgLine::default();
                                ln.id = get_id();
                                ln.canvas_id = canvas_state.canvas_id.clone().unwrap();
                                ln.points = [return_curve.from.unwrap(), return_curve.to.unwrap()];
                                ln.stroke_width = return_curve.width;
                                ln.color = return_curve.color;
                                IpgGeometry::IpgLine(ln)
                            },
                            IpgCanvasWidget::Polygon => {
                                let mut poly = IpgPolygon::default();
                                poly.id = get_id();
                                poly.canvas_id = canvas_state.canvas_id.clone().unwrap();
                                poly.points = return_curve.points;
                                poly.stroke_width = return_curve.width;
                                poly.color = return_curve.color;
                                poly.fill = return_curve.fill;
                                IpgGeometry::IpgPolygon(poly)
                            },
                            IpgCanvasWidget::Rectangle => {
                                let mut rect = IpgRectangle::default();
                                rect.id = get_id();
                                rect.canvas_id = canvas_state.canvas_id.clone().unwrap();
                                
                                let to = return_curve.to.unwrap();
                                let from = return_curve.from.unwrap();
                                
                                rect.width = (to.x-from.x).abs();
                                rect.height = (to.y-from.y).abs();

                                rect.top_left = if from.x < to.x && from.y > to.y {
                                    // top right
                                    Point{ x: from.x, y: from.y-rect.height }
                                } else if from.x > to.x && from.y > to.y {
                                    // top_left
                                    Point{x: from.x-rect.width, y: to.y}
                                } else if from.x > to.x  && from.y < to.y {
                                    // bottom left
                                    Point{ x: to.x, y: from.y }
                                } else if from.x < to.x  && from.y < to.y {
                                    // bottom right
                                    from
                                } else {
                                    to
                                };
                                rect.stroke_width = return_curve.width;
                                rect.color = return_curve.color;
                                rect.fill = return_curve.fill;
                                IpgGeometry::IpgRectangle(rect)
                            },
                            IpgCanvasWidget::RightTriangle => {
                                let mut r_tri = IpgRightTriangle::default();
                                r_tri.id = get_id();
                                r_tri.canvas_id = canvas_state.canvas_id.clone().unwrap();
                                r_tri.points = [return_curve.from.unwrap(), return_curve.to.unwrap(), return_curve.control.unwrap()];
                                r_tri.stroke_width = return_curve.width;
                                r_tri.color = return_curve.color;
                                r_tri.fill = return_curve.fill;
                                IpgGeometry::IpgRightTriangle(r_tri)
                            },
                            IpgCanvasWidget::Triangle => {
                                let mut tri = IpgTriangle::default();
                                tri.id = get_id();
                                tri.canvas_id = canvas_state.canvas_id.clone().unwrap();
                                tri.points = [return_curve.from.unwrap(), return_curve.to.unwrap(), return_curve.control.unwrap()];
                                tri.stroke_width = return_curve.width;
                                tri.color = return_curve.color;
                                tri.fill = return_curve.fill;
                                IpgGeometry::IpgTriangle(tri)
                            },
                        };
                        let canvas_id = canvas_state.canvas_id.clone().unwrap();
                        let geo_vec_opt = canvas_state.geometries.get_mut(&canvas_id);

                        if geo_vec_opt.is_some() {
                            let geo_vec = geo_vec_opt.unwrap();
                            geo_vec.push(geometry);
                        } else {
                            canvas_state.geometries.insert(canvas_id, vec![geometry]);
                        }
                        canvas_state.return_curve = DrawCurve::default();
                    }
                    
                    drop(canvas_state);

                    (event::Status::Captured, Some(app::Message::Canvas))
                    
                }
                _ => (event::Status::Ignored, None),
            }
    }    

    fn draw(&self, _state: &(), renderer: &Renderer, theme: &Theme, bounds: Rectangle, _cursor: mouse::Cursor) -> Vec<Geometry> {
        
        let canvas_state = access_canvas_state();
        // if !canvas_state.curves_update {
        //     drop(canvas_state);
        //     return vec![]
        // }
        
        // let mut frame_vec: Vec<Geometry> = vec![];
        let palette_color = theme.palette().text;

        let content =
                self.cache.draw(renderer, bounds.size(), |frame| { 

        for (_canvas, geometries) in canvas_state.geometries.iter() {
            for geometry in geometries {
                match geometry {
                    IpgGeometry::IpgArc(ac) => {
                        // let mut frame = Frame::new(renderer, bounds.size());
                        let center = Point::from(ac.center);

                        let arc = Path::new(|p| {
                            p.arc(Arc { center, radius: ac.radius, start_angle: Radians(ac.start_angle), end_angle: Radians(ac.end_angle) })
                        });

                        let color = if ac.color.is_some() {
                            match_ipg_color(ac.color.clone().unwrap())
                        } else {
                            palette_color
                        };

                        let style = Style::Solid(color);

                        let stroke = Stroke{ style, width: ac.stroke_width, ..Default::default()};

                        if ac.fill {
                            frame.fill(&arc, Fill {
                                style,
                                ..Fill::default()
                            });
                        } else {
                            frame.stroke(&arc, stroke);
                        }
                        
                        // frame_vec.push(frame.into_geometry());
                    },
                    IpgGeometry::IpgBezier(bz) => {
                        // let mut frame = Frame::new(renderer, bounds.size());
                        let start = Point::from(bz.points.0);
                        let cp = Point::from(bz.points.1);
                        let end = Point::from(bz.points.2);
                        
                        let bezier = Path::new(|p| {
                            p.move_to(start);
                            p.quadratic_curve_to(cp, end);
                        });
                        
                        let color = if bz.color.is_some() {
                            match_ipg_color(bz.color.clone().unwrap())
                        } else {
                            palette_color
                        };

                        let style = Style::Solid(color);

                        let stroke = Stroke{ style, width: bz.stroke_width, ..Default::default()};

                        if bz.fill {
                            frame.fill(&bezier, Fill {
                                style,
                                ..Fill::default()
                            });
                        } else {
                            frame.stroke(&bezier, stroke);
                        }
                        
                        // frame_vec.push(frame.into_geometry());
                    },
                    IpgGeometry::IpgCircle(cir) => {
                        // let mut frame = Frame::new(renderer, bounds.size());
                        let point = Point::from(cir.center);
                        
                        let circle = Path::circle(point, cir.radius);

                        let color = if cir.color.is_some() {
                            match_ipg_color(cir.color.clone().unwrap())
                        } else {
                            palette_color
                        };

                        let style = Style::Solid(color);

                        let stroke = Stroke{ style, width: cir.stroke_width, ..Default::default()};

                        if cir.fill {
                            frame.fill(&circle, Fill {
                                style,
                                ..Fill::default()
                            });
                        } else {
                            frame.stroke(&circle, stroke);
                        }
                        
                        // frame_vec.push(frame.into_geometry());
                    },
                    IpgGeometry::IpgEllipse(el) => {
                        // let mut frame = Frame::new(renderer, bounds.size());
                       
                        let radii = Vector::from([el.radii.x, el.radii.y]);
                        let rotation = Radians(el.rotation);
                        let start_angle = Radians(el.start_angle);
                        let end_angle = Radians(el.end_angle);
                        
                        let ellipse = Path::new(|p| {
                            p.ellipse(Elliptical{ center: el.center, radii, rotation, start_angle, end_angle });
                        });
                        
                        let color = if el.color.is_some() {
                            match_ipg_color(el.color.clone().unwrap())
                        } else {
                            palette_color
                        };

                        let style = Style::Solid(color);

                        let stroke = Stroke{ style, width: el.stroke_width, ..Default::default()};

                        if el.fill {
                            frame.fill(&ellipse, Fill {
                                style,
                                ..Fill::default()
                            });
                        } else {
                            frame.stroke(&ellipse, stroke);
                        }
                        
                        // frame_vec.push(frame.into_geometry());
                    },
                    IpgGeometry::IpgLine(ln) => {
                        // let mut frame = Frame::new(renderer, bounds.size());
                        let line = Path::line(ln.points[0].clone(), ln.points[1].clone());

                        let color = if ln.color.is_some() {
                            match_ipg_color(ln.color.clone().unwrap())
                        } else {
                            palette_color
                        };

                        let style = Style::Solid(color);

                        let stroke = Stroke{ style, width: ln.stroke_width, ..Default::default()};
                        
                        frame.stroke(&line, stroke);
                        
                        // frame_vec.push(frame.into_geometry());
                    },
                    IpgGeometry::IpgPolygon(_poly) => todo!(),
                    IpgGeometry::IpgRectangle(rect) => {
                        // let mut frame = Frame::new(renderer, bounds.size());
                        let size = iced::Size { width: rect.width, height: rect.height };
                        let rectangle = Path::rectangle(rect.top_left, size);

                        let color = if rect.color.is_some() {
                            match_ipg_color(rect.color.clone().unwrap())
                        } else {
                            palette_color
                        };

                        let style = Style::Solid(color);

                        let stroke = Stroke{ style, width: rect.stroke_width, ..Default::default()};
                        
                        if rect.fill {
                            frame.fill(&rectangle, Fill {
                                style,
                                ..Fill::default()
                            });
                        } else {
                            frame.stroke(&rectangle, stroke);
                        }
                        
                        // frame_vec.push(frame.into_geometry());
                    },
                    IpgGeometry::None => (),
                    
                    IpgGeometry::IpgTriangle(_tri) => {

                    },
                    IpgGeometry::IpgRightTriangle(_r_tri) => {

                    },
                }
            }
            frame.stroke(
                &Path::rectangle(Point::ORIGIN, frame.size()),
                Stroke::default()
                    .with_width(2.0)
                    .with_color(theme.palette().text),
                );
        }
        // canvas_state.curves_update = false;
        drop(canvas_state);
        });
        
        let mut canvas_state = access_canvas_state();
        let curve = canvas_state.return_curve.clone();
        canvas_state.curves_update = false;
        let pending = match curve.curve_type {
            IpgCanvasWidget::None => None,
            IpgCanvasWidget::Bezier => {
                None
            },
            IpgCanvasWidget::Circle => {
                let mut frame = Frame::new(renderer, bounds.size());
                let mut circle = Path::circle(curve.from.unwrap(), 2.0);
                    
                if curve.from.is_some() && curve.to.is_some() {
                    let radius = curve.from.unwrap().distance(curve.to.unwrap());
                    circle = Path::circle(curve.from.unwrap(), radius);
                }
                
                let color = 
                    if curve.color.is_some() {
                        match_ipg_color(curve.color.clone().unwrap())
                    } else {
                        palette_color
                    };

                let style = Style::Solid(color);
                let stroke = Stroke{ style, width: curve.width, ..Default::default()};

                if curve.fill {
                    frame.fill(&circle, Fill {
                        style,
                        ..Fill::default()
                    });
                } else {
                    frame.stroke(&circle, stroke);
                }
                
                Some(frame)
            },
            IpgCanvasWidget::Ellipse => {
                None
            },
            IpgCanvasWidget::Line => {
                let mut frame = Frame::new(renderer, bounds.size());
                let mut line = Path::circle(curve.from.unwrap(), 2.0);
                if curve.from.is_some() && curve.to.is_some() {
                    line = Path::line(curve.from.unwrap(), curve.to.unwrap());
                };

                let color = 
                    if curve.color.is_some() {
                        match_ipg_color(curve.color.clone().unwrap())
                    } else {
                        palette_color
                    };

                let style = Style::Solid(color);

                let stroke = Stroke{ style, width: curve.width, ..Default::default()};
                
                frame.stroke(&line, stroke);
                
                Some(frame)
            },
            IpgCanvasWidget::Polygon => None,
            IpgCanvasWidget::Rectangle => {
                let mut frame = Frame::new(renderer, bounds.size());
                let mut rect = Path::circle(curve.from.unwrap(), 2.0);

                if curve.from.is_some() && curve.to.is_some() {
                    let from = curve.from.unwrap();
                    let to = curve.to.unwrap();
                    
                    let width = (to.x-from.x).abs();
                    let height = (to.y-from.y).abs();
                    let size = Size{ width, height };
                    
                    let top_left = 
                        if from.x < to.x && from.y > to.y {
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
                    
                    rect = Path::rectangle(top_left, size);
                };
                
                let color = 
                    if curve.color.is_some() {
                        match_ipg_color(curve.color.clone().unwrap())
                    } else {
                        palette_color
                    };

                let style = Style::Solid(color);

                let stroke = Stroke{ style, width: curve.width, ..Default::default()};
                
                if curve.fill {
                    frame.fill(&rect, Fill {
                        style,
                        ..Fill::default()
                    });
                } else {
                    frame.stroke(&rect, stroke);
                }
                
                Some(frame)
            },
            IpgCanvasWidget::RightTriangle => {
                None
            },
            IpgCanvasWidget::Triangle => {
                let mut frame = Frame::new(renderer, bounds.size());
                let mut triangle = Path::circle(curve.from.unwrap(), 2.0);
                
                if curve.from.is_some() && curve.to.is_some() && curve.control.is_none() {
                    triangle = Path::line(curve.from.unwrap(), curve.to.unwrap());
                }

                if curve.from.is_some() && curve.to.is_some() && curve.control.is_some() {
                    triangle = Path::new(|p| {
                        p.move_to(curve.from.unwrap());
                        p.line_to(curve.to.unwrap());
                        p.line_to(curve.control.unwrap());
                        p.line_to(curve.from.unwrap());
                    });
                }

                let color = 
                    if curve.color.is_some() {
                        match_ipg_color(curve.color.clone().unwrap())
                    } else {
                        palette_color
                    };

                let style = Style::Solid(color);

                let stroke = Stroke{ style, width: curve.width, ..Default::default()};
                
                if curve.fill {
                    frame.fill(&triangle, Fill {
                        style,
                        ..Fill::default()
                    });
                } else {
                    frame.stroke(&triangle, stroke);
                }
                
                Some(frame)
                
            },
        };
        // let mut frame = Frame::new(renderer, bounds.size());
        
        // frame_vec.push(frame.into_geometry());

        drop(canvas_state);
       
    if pending.is_some() {
        vec![content, pending.unwrap().into_geometry()]
    } else {
        vec![content]
    }

    }

}

// This callback comes from the mouse actions
pub fn canvas_callback() {
    let mut canvas_state = access_canvas_state();
    canvas_state.curves_update = true;
    // match canvas_state.mode {
    //     IpgCanvasMode::New => {
    //         canvas_state.return_curve.mode = canvas_state.mode.clone();
    //         canvas_state.return_curve.curve_type = canvas_state.selection;
            // let last = canvas_state.curves.last();
            // if last.is_none() {
            //     canvas_state.curves.push(return_curve.clone());
            // } else if last.unwrap().status == CurveStatus::InProgress {
            //     //replace last curve
            //     let final_length = canvas_state.curves.len().saturating_sub(1);
            //     canvas_state.curves.truncate(final_length);
            //     canvas_state.curves.push(return_curve.clone());
            //     dbg!(canvas_state.curves.len());
            // } else {
            //     canvas_state.curves.push(return_curve.clone());
            // }
        // }
        // IpgCanvasMode::Edit => {
        // canvas_state.curves[return_curve.edit_index.unwrap()] = return_curve.clone();
        // canvas_state.curve_editing = Some(return_curve.clone());
        // }
        // IpgCanvasMode::Freehand => todo!(),
        // IpgCanvasMode::PicknPlace => todo!(),
    // }   
    drop(canvas_state);
    // self.state.request_redraw();
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
