#![allow(dead_code)]

use iced::widget::canvas::path::arc::Elliptical;
use iced::widget::canvas::path::Arc;
use iced::widget::Canvas;
use iced::{mouse, Element, Length, Point, Radians, Vector};
use iced::widget::canvas::{
    self, Fill, Frame, Geometry, Path, Stroke, Style
};
use iced::{Rectangle, Renderer, Theme};

use crate::access_state;
use crate::app::Message;
use crate::graphics::colors::{match_ipg_color, IpgColor};


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

pub enum IpgGeometry {
    IpgArc(IpgArc),
    IpgBezier(IpgBezier),
    IpgCircle(IpgCircle),
    IpgEllipse(IpgEllipse),
    IpgLine(IpgLine),
    IpgRectangle(IpgRectangle),
}

pub fn construct_canvas(can: IpgCanvas) -> Element<'static, Message> {

    return Canvas::new(IpgBuildCanvas {id: can.id})
        .width(can.width)
        .height(can.height)
        .into()
}


pub struct IpgBuildCanvas {
    id: usize,
}

impl<Message> canvas::Program<Message> for IpgBuildCanvas {
    type State = ();

    fn draw(&self, _state: &(), renderer: &Renderer, theme: &Theme, bounds: Rectangle, _cursor: mouse::Cursor) -> Vec<Geometry> {

        let mut frame_vec: Vec<Geometry> = vec![];

        let state = access_state();

        let palette_color = theme.palette().text;

        let geometries_opt = state.geometries.get(&self.id);

        let geometries = if geometries_opt.is_some() {
            geometries_opt.unwrap()
        } else {
            panic!("IpgBuildCanvas: Unable to find the geometries using id {}", self.id )
        };

        for geometry in geometries.iter() {

            match geometry {
                IpgGeometry::IpgArc(ac) => {
                    let mut frame = Frame::new(renderer, bounds.size());
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
                    
                    frame_vec.push(frame.into_geometry());
                },
                IpgGeometry::IpgBezier(bz) => {
                    let mut frame = Frame::new(renderer, bounds.size());
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
                    
                    frame_vec.push(frame.into_geometry());
                },
                IpgGeometry::IpgCircle(cir) => {
                    let mut frame = Frame::new(renderer, bounds.size());
                    let point = Point::from(cir.center_xy);
                    
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
                    
                    frame_vec.push(frame.into_geometry());
                },
                IpgGeometry::IpgEllipse(el) => {
                    let mut frame = Frame::new(renderer, bounds.size());
                    let center = Point::from(el.center);
                    let radii = Vector::from([el.radii.0, el.radii.1]);
                    let rotation = Radians(el.rotation);
                    let start_angle = Radians(el.start_angle);
                    let end_angle = Radians(el.end_angle);
                    
                    let ellipse = Path::new(|p| {
                        p.ellipse(Elliptical{ center, radii, rotation, start_angle, end_angle });
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
                    
                    frame_vec.push(frame.into_geometry());
                },
                IpgGeometry::IpgLine(ln) => {
                    let mut frame = Frame::new(renderer, bounds.size());
                    let start = Point::from(ln.points[0]);
                    let end = Point::from(ln.points[1]);
                    let line = Path::line(start, end);

                    let color = if ln.color.is_some() {
                        match_ipg_color(ln.color.clone().unwrap())
                    } else {
                        palette_color
                    };

                    let style = Style::Solid(color);

                    let stroke = Stroke{ style, width: ln.stroke_width, ..Default::default()};
                    
                    frame.stroke(&line, stroke);
                    
                    frame_vec.push(frame.into_geometry());
                },
                IpgGeometry::IpgRectangle(rect) => {
                    let mut frame = Frame::new(renderer, bounds.size());
                    let top_left = Point { x: rect.top_left_xy.0, y: rect.top_left_xy.1 };
                    let size = iced::Size { width: rect.width, height: rect.height };
                    let rectangle = Path::rectangle(top_left, size);

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
                    
                    frame_vec.push(frame.into_geometry());
                },
            }
        }
        drop(state);
        frame_vec
        
    }
}


#[derive(Debug, Clone)]
pub struct IpgArc {
    pub id: usize,
    pub canvas_id: String,
    pub center: (f32, f32),
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
        center: (f32, f32),
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

#[derive(Debug, Clone)]
pub struct IpgBezier {
    pub id: usize,
    pub canvas_id: String,
    pub points: ((f32, f32), (f32, f32), (f32, f32)),
    pub radius: f32,
    pub stroke_width: f32,
    pub color: Option<IpgColor>,
    pub fill: bool,
    pub show: bool,
}

impl IpgBezier {
    pub fn new(
        id: usize,
        canvas_id: String,
        points: ((f32, f32), (f32, f32), (f32, f32)),
        radius: f32,
        stroke_width: f32,
        color: Option<IpgColor>,
        fill: bool,
        show: bool,
    ) -> Self {
        Self {
            id,
            canvas_id,
            points,
            radius,
            stroke_width,
            color,
            fill,
            show, 
        }
    }
}

#[derive(Debug, Clone)]
pub struct IpgCircle {
    pub id: usize,
    pub canvas_id: String,
    pub center_xy: (f32, f32),
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
        center_xy: (f32, f32),
        radius: f32,
        stroke_width: f32,
        color: Option<IpgColor>,
        fill: bool,
        show: bool,
    ) -> Self {
        Self {
            id,
            canvas_id,
            center_xy,
            radius,
            stroke_width,
            color,
            fill,
            show, 
        }
    }
}

#[derive(Debug, Clone)]
pub struct IpgEllipse {
    pub id: usize,
    pub canvas_id: String,
    pub center: (f32, f32),
    pub radii: (f32, f32),
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
        center: (f32, f32),
        radii: (f32, f32),
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

#[derive(Debug, Clone)]
pub struct IpgLine {
    pub id: usize,
    pub canvas_id: String,
    pub points: Vec<(f32, f32)>,
    pub stroke_width: f32,
    pub color: Option<IpgColor>,
    pub show: bool,
}

impl IpgLine {
    pub fn new(
        id: usize,
        canvas_id: String,
        points: Vec<(f32, f32)>,
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

#[derive(Debug, Clone)]
pub struct IpgRectangle {
    pub id: usize,
    pub canvas_id: String,
    pub top_left_xy: (f32, f32),
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
        top_left_xy: (f32, f32),
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
            top_left_xy,
            width,
            height,
            stroke_width,
            color,
            fill,
            show, 
        }
    }
}