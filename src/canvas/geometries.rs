//! geometries

use std::collections::HashMap;

use iced::{alignment, keyboard::Key, widget::{image, text::{LineHeight, Shaping}}, Color, Font, Pixels, Point, Radians, Rectangle, Size, Vector};
use pyo3::pyclass;
use serde::{Deserialize, Serialize};

use super::{canvas_helpers::{build_polygon, get_angle_of_vectors, 
    get_horizontal_angle_of_vector, get_line_from_slope_intercept, 
    get_linear_regression, get_mid_point, rotate_geometry, to_degrees, 
    to_radians, translate_geometry}, 
    draw_canvas::{IpgDrawMode, IpgDrawStatus, IpgWidget}};



#[derive(Debug, Clone)]
pub struct IpgArc {
    pub id: usize,
    pub points: Vec<Point>,
    pub mid_point: Point,
    pub radius: f32,
    pub color: Color,
    pub fill_color: Option<Color>,
    pub width: f32,
    pub stroke_dash_offset: Option<usize>,
    pub stroke_dash_segments: Option<Vec<f32>>,
    pub start_angle: Radians,
    pub end_angle: Radians,
    pub draw_mode: IpgDrawMode,
    pub status: IpgDrawStatus,
}

#[derive(Debug, Clone)]
pub struct IpgBezier {
    pub id: usize,
    pub points: Vec<Point>,
    pub mid_point: Point,
    pub color: Color,
    pub fill_color: Option<Color>,
    pub width: f32,
    pub stroke_dash_offset: Option<usize>,
    pub stroke_dash_segments: Option<Vec<f32>>,
    pub rotation: f32,
    pub draw_mode: IpgDrawMode,
    pub status: IpgDrawStatus,
}

#[derive(Debug, Clone)]
pub struct IpgCircle {
    pub id: usize,
    pub center: Point,
    pub circle_point: Point,
    pub radius: f32,
    pub color: Color,
    pub fill_color: Option<Color>,
    pub width: f32,
    pub stroke_dash_offset: usize,
    pub stroke_dash_segments: Option<Vec<f32>>,
    pub draw_mode: IpgDrawMode,
    pub status: IpgDrawStatus,
}

#[derive(Debug, Clone)]
pub struct IpgEllipse {
    pub id: usize,
    pub points: Vec<Point>,
    pub center: Point,
    pub radii: Vector,
    pub rotation: Radians,
    pub color: Color,
    pub fill_color: Option<Color>,
    pub width: f32,
    pub stroke_dash_offset: Option<usize>,
    pub stroke_dash_segments: Option<Vec<f32>>,
    pub draw_mode: IpgDrawMode,
    pub status: IpgDrawStatus,
}

#[derive(Debug, Clone)]
pub struct IpgCanvasImage {
    pub id: usize,
    pub path: image::Handle,
    pub position: Point,
    pub bounds: Rectangle,
    pub width: f32,
    pub height: f32,
    pub rotation: f32,
    pub draw_mode: IpgDrawMode,
    pub status: IpgDrawStatus,
}

#[derive(Debug, Clone)]
pub struct IpgLine {
    pub id: usize,
    pub points: Vec<Point>,
    pub mid_point: Point,
    pub color: Color,
    pub width: f32,
    pub stroke_dash_offset: Option<usize>,
    pub stroke_dash_segments: Option<Vec<f32>>,
    pub rotation: f32,
    pub draw_mode: IpgDrawMode,
    pub status: IpgDrawStatus,
}

#[derive(Debug, Clone)]
pub struct IpgPolyLine {
    pub id: usize,
    pub points: Vec<Point>,
    pub poly_points: usize,
    pub mid_point: Point,
    pub pl_point: Point,
    pub color: Color,
    pub width: f32,
    pub stroke_dash_offset: Option<usize>,
    pub stroke_dash_segments: Option<Vec<f32>>,
    pub rotation: f32,
    pub draw_mode: IpgDrawMode,
    pub status: IpgDrawStatus,
}

#[derive(Debug, Clone)]
pub struct IpgPolygon {
    pub id: usize,
    pub points: Vec<Point>,
    pub poly_points: usize,
    pub mid_point: Point,
    pub pg_point: Point,
    pub color: Color,
    pub fill_color: Option<Color>,
    pub width: f32,
    pub stroke_dash_offset: Option<usize>,
    pub stroke_dash_segments: Option<Vec<f32>>,
    pub rotation: f32,
    pub draw_mode: IpgDrawMode,
    pub status: IpgDrawStatus,
}

#[derive(Debug, Clone)]
pub struct IpgRectangle {
    pub id: usize,
    pub top_left: Point,
    pub size: Size,
    pub mid_point: Point,
    pub color: Color,
    pub fill_color: Option<Color>,
    pub width: f32,
    pub stroke_dash_offset: Option<usize>,
    pub stroke_dash_segments: Option<Vec<f32>>,
    pub rotation: f32,
    pub draw_mode: IpgDrawMode,
    pub status: IpgDrawStatus,
}

#[derive(Debug, Clone)]
pub struct IpgRightTriangle {
    pub id: usize,
    pub points: Vec<Point>,
    pub mid_point: Point,
    pub tr_point: Point,
    pub color: Color,
    pub fill_color: Option<Color>,
    pub width: f32,
    pub stroke_dash_offset: Option<usize>,
    pub stroke_dash_segments: Option<Vec<f32>>,
    pub rotation: f32,
    pub draw_mode: IpgDrawMode,
    pub status: IpgDrawStatus,
}

#[derive(Debug, Clone)]
pub struct IpgText {
    pub id: usize,
    pub content: String,
    pub position: Point,
    pub color: Color,
    pub size: Pixels,
    pub line_height: LineHeight,
    pub font: Font,
    pub horizontal_alignment: alignment::Horizontal,
    pub vertical_alignment: alignment::Vertical,
    pub shaping: Shaping,
    pub rotation: f32,
    pub draw_mode: IpgDrawMode,
    pub status: IpgDrawStatus,
}

#[derive(Debug, Clone)]
pub struct IpgFreeHand {
    pub id: usize,
    pub points: Vec<Point>,
     pub color: Color,
    pub width: f32,
    pub stroke_dash_offset: Option<usize>,
    pub stroke_dash_segments: Option<Vec<f32>>,
    pub draw_mode: IpgDrawMode,
    pub status: IpgDrawStatus,
    pub completed: bool,
}

#[pyclass]
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq,)]
pub enum IpgCanvasWidget {
    None,
    Arc,
    Bezier,
    Circle,
    Ellipse,
    Line,
    PolyLine,
    Polygon,
    Rectangle,
    RightTriangle,
    Text,
    FreeHand,
}

pub fn check_if_text_widget(canvas_widget: &IpgWidget) -> bool {
    matches!(canvas_widget, IpgWidget::Text(_))
}

pub fn add_new_widget(widget: IpgCanvasWidget, 
                    poly_points: usize, 
                    color: Color,
                    fill_color: Option<Color>,
                    width: f32,
                    draw_mode: IpgDrawMode,
		            h_alignment: alignment::Horizontal,
                    v_alignment: alignment::Vertical,
                    ) 
                    -> IpgWidget {
    match widget {
        IpgCanvasWidget::None => {
            IpgWidget::None
        },
        IpgCanvasWidget::Arc => {
            IpgWidget::Arc(
                IpgArc {
                    id: 0,
                    points: vec![],
                    mid_point: Point::default(),
                    radius: 0.0,
                    color,
                    fill_color,
                    width,
                    stroke_dash_offset: None,
                    stroke_dash_segments: None,
                    start_angle: Radians::PI,
                    end_angle: Radians::PI,
                    draw_mode,
                    status: IpgDrawStatus::Inprogress,
                })
        
        },
        IpgCanvasWidget::Bezier => {
            IpgWidget::Bezier(
                IpgBezier { 
                    id: 0,
                    points: vec![],
                    mid_point: Point::default(),
                    color,
                    fill_color,
                    width,
                    stroke_dash_offset: None,
                    stroke_dash_segments: None, 
                    rotation: 0.0, 
                    draw_mode,
                    status: IpgDrawStatus::Inprogress,
                }
            )
        },
        IpgCanvasWidget::Circle => {
            IpgWidget::Circle(
                IpgCircle {
                    id: 0,
                    center: Point::default(),
                    circle_point: Point::default(),
                    radius: 0.0,
                    color,
                    fill_color,
                    width,
                    stroke_dash_offset: 0,
                    stroke_dash_segments: None,
                    draw_mode,
                    status: IpgDrawStatus::Inprogress,
                }
            )
        },
        IpgCanvasWidget::Ellipse => {
            IpgWidget::Ellipse(
                IpgEllipse {
                    id: 0,
                    points: vec![],
                    center: Point::default(),
                    radii: Vector{x: 0.0, y: 0.0},
                    rotation: Radians(0.0),
                    color,
                    fill_color,
                    width,
                    stroke_dash_offset: None,
                    stroke_dash_segments: None,
                    draw_mode,
                    status: IpgDrawStatus::Inprogress,
                }
            )
        },
        IpgCanvasWidget::Line => {
            IpgWidget::Line(
                IpgLine {
                    id: 0,
                    points: vec![],
                    mid_point: Point::default(),
                    color,
                    width,
                    stroke_dash_offset: None,
                    stroke_dash_segments: None,
                    rotation: 0.0,
                    draw_mode,
                    status: IpgDrawStatus::Inprogress,
                }
            )
        },
        IpgCanvasWidget::PolyLine => {
            IpgWidget::PolyLine(
                IpgPolyLine {
                    id: 0,
                    points: vec![],
                    poly_points,
                    mid_point: Point::default(),
                    pl_point: Point::default(),
                    color,
                    width,
                    stroke_dash_offset: None,
                    stroke_dash_segments: None,
                    rotation: 0.0,
                    draw_mode,
                    status: IpgDrawStatus::Inprogress,
                }
            )
        },
        IpgCanvasWidget::Polygon => {
            IpgWidget::Polygon(
                IpgPolygon {
                    id: 0,
                    points: vec![],
                    poly_points,
                    mid_point: Point::default(),
                    pg_point: Point::default(),
                    color,
                    fill_color,
                    width,
                    stroke_dash_offset: None,
                    stroke_dash_segments: None,
                    rotation: 0.0,
                    draw_mode,
                    status: IpgDrawStatus::Inprogress,
                }
            )
        },
        IpgCanvasWidget::Rectangle => {
            IpgWidget::Rectangle(
                IpgRectangle{ 
                    id: 0, 
                    top_left: Point::default(), 
                    size: Size::default(), 
                    mid_point: Point::default(), 
                    color, 
                    fill_color, 
                    width,
                    stroke_dash_offset: None,
                    stroke_dash_segments: None, 
                    rotation: 0.0, 
                    draw_mode, 
                    status: IpgDrawStatus::Inprogress,
                }
            )
        },
        IpgCanvasWidget::RightTriangle => {
            IpgWidget::RightTriangle(
                IpgRightTriangle {
                    id: 0,
                    points: vec![],
                    mid_point: Point::default(),
                    tr_point: Point::default(),
                    color,
                    fill_color,
                    width,
                    stroke_dash_offset: None,
                    stroke_dash_segments: None,
                    rotation: 0.0,
                    draw_mode,
                    status: IpgDrawStatus::Inprogress,
                }
            )
        },
        IpgCanvasWidget::FreeHand => {
            IpgWidget::FreeHand(
                IpgFreeHand {
                    id: 0,
                    points: vec![],
                    color,
                    width,
                    stroke_dash_offset: None,
                    stroke_dash_segments: None,
                    draw_mode,
                    status: IpgDrawStatus::Inprogress,
                    completed: false,
                }
            )
        }
        IpgCanvasWidget::Text => {
            IpgWidget::Text(
                IpgText {
                    id: 0,
                    content: String::new(),
                    position: Point::default(),
                    color,
                    size: Pixels(16.0),
                    line_height: LineHeight::Relative(1.2),
                    font: Default::default(),
                    horizontal_alignment: h_alignment,
                    vertical_alignment: v_alignment,
                    shaping: Shaping::Basic,
                    rotation: 0.0,
                    draw_mode,
                    status: IpgDrawStatus::Inprogress,
                }
            )
        },
    }
}

pub fn complete_new_widget(widget: IpgWidget, cursor: Point) -> Option<IpgWidget> {
    match widget {
        IpgWidget::Arc(arc) => {
            Some(IpgWidget::Arc(arc))
        },
        IpgWidget::Bezier(mut bz) => {
            bz.mid_point = 
                get_mid_point(
                    bz.points[0], 
                    bz.points[1]
                );
            Some(IpgWidget::Bezier(bz))
        },
        IpgWidget::Circle(cir) => { 
            Some(IpgWidget::Circle(cir))
        },
        IpgWidget::Ellipse(mut ell) => {
            ell.center = ell.points[0];
            let vx = ell.points[1].distance(ell.center);
            let vy = cursor.distance(ell.center);
            ell.radii = Vector{ x: vx, y: vy };
            Some(IpgWidget::Ellipse(ell))
        },
        IpgWidget::Line(mut ln) => {
            // degree is angle rotation around mid point 
            let degrees = 
                get_horizontal_angle_of_vector(
                    ln.points[0],
                    ln.points[1], 
                );
            ln.rotation = degrees;

            Some(IpgWidget::Line(ln))
        },
        IpgWidget::Polygon(mut pg) => {
            pg.pg_point = cursor;
            let degrees = 
                get_horizontal_angle_of_vector(
                    pg.mid_point, 
                    cursor, 
                    );

            pg.rotation = degrees;
            pg.points = 
                build_polygon(
                    pg.mid_point, 
                    pg.pg_point, 
                    pg.poly_points,
                    pg.rotation,
                );
            
            Some(IpgWidget::Polygon(pg))
        },
        IpgWidget::PolyLine(mut pl) => {
            let (slope, intercept) =
                get_linear_regression(&pl.points);
            
            let line = 
                get_line_from_slope_intercept(
                    &pl.points, 
                    slope, 
                    intercept
                );
            pl.mid_point = 
                get_mid_point(
                    line.0, 
                    line.1);
            pl.pl_point = 
                Point::new(
                    pl.mid_point.x + 100.0, 
                    pl.mid_point.y
                );
            pl.rotation = 
                get_horizontal_angle_of_vector(
                    pl.mid_point,
                    pl.pl_point,
                );
            
            Some(IpgWidget::PolyLine(pl))
        },
        IpgWidget::RightTriangle(mut tr) => {
            tr.mid_point = tr.points[1];
            let trans_pts = translate_geometry(&tr.points, Point::default(), tr.points[1]);
            let opp = Point::new(-trans_pts[2].x, -trans_pts[2].y);
            tr.tr_point = Point::new(opp.x+tr.points[1].x, opp.y+tr.points[1].y);
            if tr.points[1].x > tr.points[2].x {
                tr.rotation = 180.0;
            } else {
                tr.rotation = 0.0;
            }
            
            Some(IpgWidget::RightTriangle(tr))
        },
        IpgWidget::FreeHand(mut fh) => {
            fh.points.push(cursor);
            Some(IpgWidget::FreeHand(fh))
        }
        IpgWidget::Text(mut txt) => {
            txt.rotation = 0.0;
            txt.status = IpgDrawStatus::Completed;
            Some(IpgWidget::Text(txt))
        },
        _ => {
            None
        },
    }
}

pub fn update_edited_widget(widget: IpgWidget,
                        cursor: Point, 
                        index: Option<usize>, 
                        mid_point: bool,
                        other_point: bool,
                        status: IpgDrawStatus,
                    ) -> IpgWidget {
    match widget {
        IpgWidget::Arc(mut arc) => {
            if index.is_some() {
                arc.points[index.unwrap()] = cursor;
                if index.unwrap() == 1 {
                    arc.radius = arc.mid_point.distance(cursor);
                    arc.start_angle = get_angle_of_vectors(
                            arc.mid_point, 
                            Point::new(-arc.mid_point.x, arc.mid_point.y), 
                            cursor) + Radians::PI;
                    arc.end_angle = 
                            get_angle_of_vectors(
                                arc.mid_point, 
                                cursor, 
                                arc.points[2]) + arc.start_angle;
                }
                if index.unwrap() == 2 {
                    arc.end_angle = 
                        get_angle_of_vectors(
                            arc.points[0], 
                            arc.points[1], 
                            cursor
                        ) + arc.start_angle;
                }
                // calc the end_angle point        
                let r = arc.radius;
                let b = arc.end_angle.0;
                let point_b = Point::new(r*b.cos(), r*b.sin());
                arc.points[2] = translate_geometry(&[point_b], arc.mid_point, Point::default())[0];

            } else if mid_point {
                arc.points = 
                    translate_geometry(
                        &arc.points, 
                        cursor,
                        arc.mid_point, 
                        );
                arc.mid_point = cursor;
            }
            arc.status = status;
            IpgWidget::Arc(arc)
        },
        IpgWidget::Bezier(mut bz) => {
            if index.is_some() {
                bz.points[index.unwrap()] = cursor;
                bz.mid_point = get_mid_point(bz.points[0], bz.points[1]);
            } else if mid_point {
                bz.points = 
                    translate_geometry(
                        &bz.points, 
                        cursor,
                        bz.mid_point, 
                        );
                bz.mid_point = cursor;
            }
            let degrees = 
                get_horizontal_angle_of_vector(
                    bz.points[0],
                    bz.points[1], 
                );
            bz.rotation = degrees;
            bz.status = status;
            IpgWidget::Bezier(bz)
        },
        IpgWidget::Circle(mut cir) => {
            if index.is_some() {
                cir.circle_point = cursor;
                cir.radius = cir.center.distance(cursor);
            } else if mid_point {
                let mut points = vec![cir.circle_point];
                points = 
                    translate_geometry(
                        &points, 
                        cursor,
                        cir.center,
                    );
                cir.center = cursor;
                cir.circle_point = points[0];
            }
            cir.status = status;
            IpgWidget::Circle(cir)
        },
        IpgWidget::Ellipse(mut ell) => {
           if mid_point {
                let points = 
                    translate_geometry(
                        &ell.points, 
                        cursor,
                        ell.center,
                    );
                ell.center = cursor;
                ell.points = points;
            }
            if index == Some(1) {
                let p1 = Point::new(cursor.x, ell.center.y);
                let vx = p1.distance(ell.center);
                let vy = ell.points[2].distance(ell.center);
                ell.points[1] = p1;
                ell.radii = Vector{ x: vx, y: vy };
            } else if index == Some(2) {
                let p2 = Point::new(ell.center.x, cursor.y);
                let vx = ell.points[1].distance(ell.center);
                let vy = p2.distance(ell.center);
                ell.points[2] = p2;
                ell.radii = Vector{ x: vx, y: vy };
            }

            ell.status = status;
            IpgWidget::Ellipse(ell)
        },
        IpgWidget::Line(mut line) => {
            if index.is_some() {
                line.points[index.unwrap()] = cursor;
                line.mid_point = get_mid_point(line.points[0], line.points[1]);
            } else if mid_point {
                line.points = 
                    translate_geometry(
                        &line.points, 
                        cursor,
                        line.mid_point, 
                        );
                line.mid_point = cursor;
            }

            let degrees = 
                get_horizontal_angle_of_vector(
                    line.points[0],  
                    line.points[1], 
                );
            line.rotation = degrees;
            line.status = status;
            IpgWidget::Line(line)
        },
        IpgWidget::Polygon(mut pg) => {
            if other_point {
                pg.pg_point = cursor;
                pg.rotation = get_horizontal_angle_of_vector(pg.mid_point, cursor);
                pg.points = 
                    build_polygon(
                        pg.mid_point, 
                        pg.pg_point, 
                        pg.poly_points,
                        pg.rotation,
                );
            } else if mid_point {
                let trans_pts = 
                    translate_geometry(
                        &[pg.pg_point], 
                        cursor,
                        pg.mid_point, 
                    );
                pg.points = 
                    build_polygon(
                        cursor, 
                        trans_pts[0], 
                        pg.poly_points,
                        pg.rotation,
                    );
                pg.mid_point = cursor;
                pg.pg_point = trans_pts[0];
            }
            pg.status = status;
            IpgWidget::Polygon(pg)
        },
        IpgWidget::PolyLine(mut pl) => {
            if index.is_some() {
                pl.points[index.unwrap()] = cursor;
                let mid_point = 
                    get_mid_geometry(
                        &pl.points, 
                        IpgCanvasWidget::PolyLine
                    );
                pl.pl_point = 
                    translate_geometry(
                        &[pl.pl_point], 
                        mid_point, 
                        pl.mid_point
                    )[0];
                pl.mid_point = mid_point;
                pl.rotation = 
                    get_horizontal_angle_of_vector(
                        pl.mid_point, 
                        pl.pl_point
                    );
            }  else if mid_point {
                let mut pts = pl.points.clone();
                pts.push(pl.pl_point);
                pts = 
                    translate_geometry(
                        &pts, 
                        cursor,
                        pl.mid_point, 
                    );
                pl.mid_point = cursor;
                pl.pl_point = pts.pop().unwrap();
                pl.points = pts;
            } else if other_point {
                let degrees = get_horizontal_angle_of_vector(pl.mid_point, cursor);
                let step_degrees = degrees-pl.rotation;
                pl.points = rotate_geometry(&pl.points, &pl.mid_point, &step_degrees, IpgCanvasWidget::PolyLine);
                pl.pl_point = cursor;
                pl.rotation = degrees;
            }
            pl.status = status;
            IpgWidget::PolyLine(pl)
        },
        IpgWidget::RightTriangle(mut tr) => {
            if index.is_some() {
                let index = index.unwrap();
                if index == 0 {
                    tr.points[index].y = cursor.y;
                }
                if index == 1 {
                    tr.points[1].y = cursor.y;
                    tr.points[2].y = cursor.y;
                }
                if index == 2 {
                    tr.points[2].x = cursor.x;
                }
                let mid = get_mid_point(tr.points[1], tr.points[2]);
                let dist_b_mid = Point::new(mid.x-tr.points[2].x, mid.y-tr.points[2].y);
                tr.tr_point = Point::new(tr.points[2].x+dist_b_mid.x, tr.points[2].y+dist_b_mid.y);
            } else if mid_point {
                let mut pts = tr.points.clone();
                pts.push(tr.tr_point);
                pts = 
                    translate_geometry(
                        &pts, 
                        cursor,
                        tr.mid_point, 
                    );
                tr.mid_point = cursor;
                tr.tr_point = pts.pop().unwrap();
                tr.points = pts;
            } else if other_point {
                let degrees = get_horizontal_angle_of_vector(tr.mid_point, cursor);
                let step_degrees = degrees-tr.rotation;
                tr.points = rotate_geometry(&tr.points, &tr.mid_point, &step_degrees, IpgCanvasWidget::RightTriangle);
                tr.tr_point = cursor;
                tr.rotation = degrees;
            }
            tr.status = status;
            IpgWidget::RightTriangle(tr)
        },
        IpgWidget::FreeHand(mut fh) => {
            if index.is_some() {
                fh.points[index.unwrap()] = cursor;
            }
            fh.status = status;
            IpgWidget::FreeHand(fh)
        },
        IpgWidget::Text(mut txt) => {
            txt.position = cursor;
            txt.status = status;
            IpgWidget::Text(txt)
        },
        _ => {
            IpgWidget::None
        },
    }
}

pub fn update_rotated_widget(widget: &mut IpgWidget, 
                        step_degrees: f32,
                        status: Option<IpgDrawStatus>,
                    ) -> (IpgWidget, f32) {
    match widget {
        IpgWidget::Arc(arc) => {
            arc.points = rotate_geometry(&arc.points, &arc.mid_point, &step_degrees, IpgCanvasWidget::Arc);
            arc.start_angle = 
                get_angle_of_vectors(
                    arc.points[0], 
                    Point::new(-arc.points[0].x, arc.points[0].y), 
                    arc.points[1]) + Radians::PI;
            arc.end_angle = 
                get_angle_of_vectors(
                    arc.points[0], 
                    arc.points[1], 
                    arc.points[2]) + arc.start_angle;

            // calc the end_angle point        
            let r = arc.radius;
            let b = arc.end_angle.0;
            let point_b = Point::new(r*b.cos(), r*b.sin());

            arc.points[2] = translate_geometry(&[point_b], arc.mid_point, Point::default())[0];
            
            if status.is_some() {
                arc.status = status.unwrap();
            }
            (IpgWidget::Arc(arc.clone()), Radians::into(arc.start_angle))
        },
        IpgWidget::Bezier(bz) => {
            bz.points = rotate_geometry(&bz.points, &bz.mid_point, &step_degrees, IpgCanvasWidget::Bezier);
            bz.rotation = get_horizontal_angle_of_vector(bz.mid_point, bz.points[1]);
            if status.is_some() {
                bz.status = status.unwrap();
            }
            (IpgWidget::Bezier(bz.clone()), bz.rotation)
        },
        IpgWidget::Circle(cir) => {
            (IpgWidget::Circle(cir.clone()), 0.0)
        },
        IpgWidget::Ellipse(ell) => {
            let rads = to_radians(&step_degrees) + ell.rotation.0;
            ell.rotation = Radians(rads);
            if status.is_some() {
                ell.status = status.unwrap();
            }
            (IpgWidget::Ellipse(ell.clone()), to_degrees(&rads))
        },
        IpgWidget::Line(ln) => {
            ln.points = rotate_geometry(&ln.points, &ln.mid_point, &step_degrees, IpgCanvasWidget::Line);
            ln.rotation = get_horizontal_angle_of_vector(ln.mid_point, ln.points[1]);
            if status.is_some() {
                ln.status = status.unwrap();
            }
            (IpgWidget::Line(ln.clone()), ln.rotation)
        },
        IpgWidget::Polygon(pg) => {
            pg.points = rotate_geometry(&pg.points, &pg.mid_point, &step_degrees, IpgCanvasWidget::Polygon);
            pg.pg_point = rotate_geometry(&[pg.pg_point], &pg.mid_point, &step_degrees, IpgCanvasWidget::Line)[0];
            pg.rotation = get_horizontal_angle_of_vector(pg.mid_point, pg.pg_point);
            if status.is_some() {
                pg.status = status.unwrap();
            }
            (IpgWidget::Polygon(pg.clone()), pg.rotation)
        },
        IpgWidget::PolyLine(pl) => {
            let mut pts = pl.points.clone();
            pts.push(pl.pl_point);
            pts = rotate_geometry(&pts, &pl.mid_point, &step_degrees, IpgCanvasWidget::PolyLine);
            pl.pl_point = pts.pop().unwrap();
            pl.points = pts;
            pl.rotation = get_horizontal_angle_of_vector(pl.mid_point, pl.pl_point);
            if status.is_some() {
                pl.status = status.unwrap();
            }
            (IpgWidget::PolyLine(pl.clone()), pl.rotation)
        },
        IpgWidget::RightTriangle(tr) => {
            let mut pts = tr.points.clone();
            pts.push(tr.tr_point);
            pts = rotate_geometry(&pts, &tr.mid_point, &step_degrees, IpgCanvasWidget::RightTriangle);
            tr.tr_point = pts.pop().unwrap();
            tr.points = pts;
            tr.rotation = get_horizontal_angle_of_vector(tr.mid_point, tr.tr_point);
            if status.is_some() {
                tr.status = status.unwrap();
            }
            (IpgWidget::RightTriangle(tr.clone()), tr.rotation)
        },
        IpgWidget::FreeHand(fh) => {
            (IpgWidget::FreeHand(fh.clone()), 0.0)
        },
        IpgWidget::Text(txt) => {
            txt.rotation += step_degrees;
            if status.is_some() {
                txt.status = status.unwrap();
            }
            (IpgWidget::Text(txt.clone()), txt.rotation)
        },
        _ => (IpgWidget::None, 0.0),
    }
}

pub fn add_keypress(widget: &mut IpgWidget, modified: Key) -> (Option<IpgWidget>, bool) {
    let mut escape = false;
    match widget {
        IpgWidget::Text(txt) => {
            match modified.as_ref() {
                Key::Named(named) => {
                    match named {
                        iced::keyboard::key::Named::Enter => {
                            txt.content.push('\r');
                        },
                        iced::keyboard::key::Named::Tab => {
                            txt.content.push_str("    ");
                        },
                        iced::keyboard::key::Named::Space => {
                            txt.content.push(' ');
                        },
                        iced::keyboard::key::Named::Escape => escape = true,
                        iced::keyboard::key::Named::Backspace => {
                            if !txt.content.is_empty() {
                                txt.content.pop();
                            }
                        } 
                        _ => ()
                    }
                },
                Key::Character(c) => {
                    txt.content.push_str(c);
                },
                Key::Unidentified => (),
            }
            if escape {
                (None, false)
            } else {
                (Some(IpgWidget::Text(txt.clone())), false)
            }
        },
        IpgWidget::FreeHand(fh) => {
            if let Key::Named(named) = modified.as_ref() {
                if named == iced::keyboard::key::Named::Enter {
                    fh.completed = true;
                }
            }
            
           (Some(IpgWidget::FreeHand(fh.clone())), fh.completed)
            
        }
        _ => (None, false)
    }
}

pub fn get_del_key(modified: Key) -> bool {
    match modified.as_ref() {
        Key::Named(named) => {
            matches!(named, iced::keyboard::key::Named::Delete)
        },
        _ => false,
    }
}

pub fn set_widget_mode_or_status_or_id(widget: IpgWidget, 
                    mode: Option<IpgDrawMode>,
                    status: Option<IpgDrawStatus>,
                    id: Option<usize>,
                    ) -> IpgWidget {
    match widget {
        IpgWidget::Arc(mut arc) => {
            if mode.is_some() {
                arc.draw_mode = mode.unwrap();
            }
            if status.is_some() {
                arc.status = status.unwrap();
            }
            if id.is_some() {
                arc.id = id.unwrap();
            }
            IpgWidget::Arc(arc)
        },
        IpgWidget::Bezier(mut bz) => {
            if mode.is_some() {
                bz.draw_mode = mode.unwrap();
            }
            if status.is_some() {
                bz.status = status.unwrap();
            }
            if id.is_some() {
                bz.id = id.unwrap();
            }
            IpgWidget::Bezier(bz)
        },
        IpgWidget::Circle(mut cir) => {
            if mode.is_some() {
                cir.draw_mode = mode.unwrap();
            }
            if status.is_some() {
                cir.status = status.unwrap();
            }
            if id.is_some() {
                cir.id = id.unwrap();
            }
            IpgWidget::Circle(cir)
        },
        IpgWidget::Ellipse(mut ell) => {
            if mode.is_some() {
                ell.draw_mode = mode.unwrap();
            }
            if status.is_some() {
                ell.status = status.unwrap();
            }
            if id.is_some() {
                ell.id = id.unwrap();
            }
            IpgWidget::Ellipse(ell)
        },
        IpgWidget::Image(mut img) => {
            if mode.is_some() {
                img.draw_mode = mode.unwrap();
            }
            if status.is_some() {
                img.status = status.unwrap();
            }
            if id.is_some() {
                img.id = id.unwrap();
            }
            IpgWidget::Image(img)
        },
        IpgWidget::Line(mut ln) => {
            if mode.is_some() {
                ln.draw_mode = mode.unwrap();
            }
            if status.is_some() {
                ln.status = status.unwrap();
            }
            if id.is_some() {
                ln.id = id.unwrap();
            }
            IpgWidget::Line(ln)
        },
        IpgWidget::PolyLine(mut pl) => {
            if mode.is_some() {
                pl.draw_mode = mode.unwrap();
            }
            if status.is_some() {
                pl.status = status.unwrap();
            }
            if id.is_some() {
                pl.id = id.unwrap();
            }
            IpgWidget::PolyLine(pl)
        },
        IpgWidget::Polygon(mut pg) => {
            if mode.is_some() {
                pg.draw_mode = mode.unwrap();
            }
            if status.is_some() {
                pg.status = status.unwrap();
            }
            if id.is_some() {
                pg.id = id.unwrap();
            }
            IpgWidget::Polygon(pg)
        },
        IpgWidget::Rectangle(mut rect) => {
            if mode.is_some() {
                rect.draw_mode = mode.unwrap();
            }
            if status.is_some() {
                rect.status = status.unwrap();
            }
            if id.is_some() {
                rect.id = id.unwrap();
            }
            IpgWidget::Rectangle(rect)
        },
        IpgWidget::RightTriangle(mut tr) => {
            if mode.is_some() {
                tr.draw_mode = mode.unwrap();
            }
            if status.is_some() {
                tr.status = status.unwrap();
            }
            if id.is_some() {
                tr.id = id.unwrap();
            }
            IpgWidget::RightTriangle(tr)
        },
        IpgWidget::FreeHand(mut fh) => {
            if mode.is_some() {
                fh.draw_mode = mode.unwrap();
            }
            if status.is_some() {
                fh.status = status.unwrap();
            }
            if id.is_some() {
                fh.id = id.unwrap();
            }
            IpgWidget::FreeHand(fh)
        },
        IpgWidget::Text(mut txt) => {
            if mode.is_some() {
                txt.draw_mode = mode.unwrap();
            }
            if status.is_some() {
                txt.status = status.unwrap();
            }
            if id.is_some() {
                txt.id = id.unwrap();
            }
            IpgWidget::Text(txt)
        },
        IpgWidget::None => {
            IpgWidget::None
        },
    }
}

// Adds a cursor position to the points then determines 
// if finish by returning the widget and the boolean
pub fn set_widget_point(widget: &IpgWidget, cursor: Point) -> (IpgWidget, bool) {
    match widget {
        IpgWidget::Arc(arc) => {
            let mut arc = arc.clone();
            arc.points.push(cursor);

            let finished = match arc.points.len() {
                1 => {
                    arc.mid_point = arc.points[0];
                    false
                },
                2 => {
                    arc.radius = arc.points[0].distance(arc.points[1]);
                    arc.start_angle = 
                        get_angle_of_vectors(
                            arc.points[0], 
                            Point::new(-arc.points[0].x, arc.points[0].y), 
                            arc.points[1]) + Radians::PI;
                    false
                },
                3 => {
                    arc.end_angle = 
                        get_angle_of_vectors(
                            arc.points[0], 
                            arc.points[1], 
                            cursor) + arc.start_angle;
                    // calc the end_angle point        
                    let r = arc.radius;
                    let b = arc.end_angle.0;
                    let point_b = Point::new(r*b.cos(), r*b.sin());
                    arc.points[2] = translate_geometry(&[point_b], arc.mid_point, Point::default())[0];
                    true
                },
                _ => false
            };

            (IpgWidget::Arc(arc), finished)
        },
        IpgWidget::Bezier(bezier) => {
            let mut bz = bezier.clone();
            let mut finished = false;
            bz.points.push(cursor);

            if bz.points.len() == 2 {
                bz.rotation = get_horizontal_angle_of_vector(bz.points[0], bz.points[1]);
            }
            if bz.points.len() == 3 {
                finished = true;
            }
            
            (IpgWidget::Bezier(bz), finished)
        },
        IpgWidget::Circle(circle) => {
            let mut cir = circle.clone();
            let finished = if cir.center == Point::default() {
                cir.center = cursor;
                false
            } else {
                cir.radius = cir.center.distance(cursor);
                cir.circle_point = cursor;
                true
            };
            
            (IpgWidget::Circle(cir), finished)
        },
        IpgWidget::Ellipse(ell) => {
            let mut ell = ell.clone();
            let finished = if ell.points.is_empty() {
                ell.points.push(cursor);
                false
            } else if ell.points.len() == 1 {
                let p1 = Point::new(cursor.x, ell.points[0].y);
                ell.points.push(p1);
                false
            } else if ell.points.len() == 2 {
                let p2 = Point::new(ell.points[0].x, cursor.y);
                ell.points.push(p2);
                true
            } else {
                false
            };
            
            (IpgWidget::Ellipse(ell), finished)
        },
        IpgWidget::Line(line) => {
            let mut ln = line.clone();
            ln.points.push(cursor);

            let finished = if ln.points.len() == 2 {
                ln.mid_point = get_mid_point(ln.points[0], ln.points[1]);
                true
            } else {
                false
            };
            
            (IpgWidget::Line(ln), finished)
        },
        IpgWidget::PolyLine(poly_line) => {
            let mut pl = poly_line.clone();
            pl.points.push(cursor);
            let finished = if pl.points.len() == pl.poly_points {
                pl.mid_point = get_mid_geometry(&pl.points, IpgCanvasWidget::PolyLine);
                true
            } else {
                false
            };
            
            (IpgWidget::PolyLine(pl), finished)
        },
        IpgWidget::Polygon(polygon) => {
            let mut pg = polygon.clone();
            let finished = if pg.mid_point == Point::default() {
                pg.mid_point = cursor;
                false
            } else {
                pg.pg_point = cursor;
                true
            };
            if finished {
                pg.rotation = get_horizontal_angle_of_vector(pg.mid_point, pg.pg_point)
            }
            (IpgWidget::Polygon(pg), finished)
        },
        IpgWidget::RightTriangle(right_triangle) => {
            let mut rt = right_triangle.clone();
            rt.points.push(cursor);
            if rt.points.len() > 1 {
            rt.points[1].x = rt.points[0].x;
            }
            if rt.points.len() > 2 {
                rt.points[2].y = rt.points[1].y;
            }
            let finished = if rt.points.len() == 3 {
                // close the triangle
                rt.points.push(right_triangle.points[0]);
                rt.mid_point = get_mid_geometry(&rt.points, IpgCanvasWidget::RightTriangle);
                true
            } else {
                false
            };
            
            (IpgWidget::RightTriangle(rt), finished)
        },
        IpgWidget::FreeHand(fh) => {
            let mut fh = fh.clone();
            fh.points.push(cursor);
            let finished = fh.completed;
            
            (IpgWidget::FreeHand(fh), finished)
        },
        IpgWidget::Text(text) => {
            let mut txt = text.clone();
            
            let finished = if txt.position == Point::default() {
                txt.position = cursor;
                false
            } else {
                txt.status = IpgDrawStatus::Completed;
                txt.draw_mode = IpgDrawMode::Display;
                true
            };
            
            (IpgWidget::Text(txt), finished)
        },
        _ => (IpgWidget::None, true),
    }
}

pub fn find_closest_widget(curves: &HashMap<usize, IpgWidget>, 
                            text_curves: &HashMap<usize, IpgWidget>,
                            cursor: Point) 
                            -> Option<IpgWidget> {
    let mut closest = f32::INFINITY;
    let mut closest_id = None;
    for (id, cw) in curves.iter() {
        let distance: f32 = get_distance_to_mid_point(cw, cursor);
        if distance < closest {
            closest = distance;
            closest_id = Some(id);
        }
    }

    let mut text_id = false;
    for(id, text) in text_curves.iter() {
        let distance: f32 = get_distance_to_mid_point(text, cursor);
        if distance < closest {
            closest = distance;
            closest_id = Some(id);
            text_id = true;
        }
    }

    let dc_opt = 
        if text_id {
            match closest_id {
                Some(id) => text_curves.get(id).cloned(),
                None => None,
            }
        } else {
            match closest_id {
                Some(id) => curves.get(id).cloned(),
                None => None,
            }
        };
        
    dc_opt
    
}

// returns a bool if mid_point and an optional usize 
// if a point in points.
pub fn find_closest_point_index(widget: &IpgWidget,
                            cursor: Point, 
                            ) -> (Option<usize>, bool, bool) {

    let mut point_dist: f32 = f32::INFINITY;
    let mut point_index = 0;

    match widget {
        IpgWidget::Arc(arc) => {
            for (idx, point) in arc.points.iter().enumerate() {
                // skip first point since its a mid_point too.
                if idx == 0 {
                    continue;
                } else {
                    let dist = cursor.distance(*point);
                    if  dist < point_dist {
                        point_index = idx;
                        point_dist = dist;
                    }
                }
            };
            
            let mid_dist = arc.mid_point.distance(cursor);

            if mid_dist < point_dist {
                (None, true, false)
            } else {
                (Some(point_index), false, false)
            }
        },
        IpgWidget::Bezier(bezier) => {
            for (idx, point) in bezier.points.iter().enumerate() {
                let dist = cursor.distance(*point);
                if  dist < point_dist {
                    point_index = idx;
                    point_dist = dist;
                }
            };
            
            let mid_dist = bezier.mid_point.distance(cursor);

            if mid_dist < point_dist {
                (None, true, false)
            } else {
                (Some(point_index), false, false)
            }
        },
        IpgWidget::Circle(cir) => {
            let center_dist = cursor.distance(cir.center);
            let point_dist = cursor.distance(cir.circle_point);
            if center_dist < point_dist {
                (None, true, false)
            } else {
                (Some(1), false, false)
            }
        }
        IpgWidget::Ellipse(ell) => {
            let center_dist = cursor.distance(ell.center);
            let point_1_dist = cursor.distance(ell.points[1]);
            let point_2_dist = cursor.distance(ell.points[2]);
            if center_dist < point_1_dist && center_dist < point_2_dist {
                (None, true, false)
            } else if point_1_dist < point_2_dist {
                (Some(1), false, false)
            } else {
                (Some(2), false, false)
            }
        }
        IpgWidget::Line(line) => {
            for (idx, point) in line.points.iter().enumerate() {
                let dist = cursor.distance(*point);
                if  dist < point_dist {
                    point_index = idx;
                    point_dist = dist;
                }
            };
            
            let mid_dist = cursor.distance(line.mid_point);

            if mid_dist < point_dist {
                (None, true, false)
            } else {
                (Some(point_index), false, false)
            }
        },
        IpgWidget::Polygon(pg) => {
            let pg_center = cursor.distance(pg.mid_point);
            let pg_point = cursor.distance(pg.pg_point);
            if pg_center <= pg_point {
                (None, true, false)
            } else {
                (None, false, true)
            }
        },
        IpgWidget::PolyLine(pl) => {
            for (idx, point) in pl.points.iter().enumerate() {
                let dist = cursor.distance(*point);
                if  dist < point_dist {
                    point_index = idx;
                    point_dist = dist;
                }
            };
            
            let mid_dist = pl.mid_point.distance(cursor);
            let pl_pt_dist = pl.pl_point.distance(cursor);

            if point_dist < mid_dist && point_dist < pl_pt_dist {
                (Some(point_index), false, false)
            } else if mid_dist < pl_pt_dist {
                (None, true, false)
            } else {
                (None, false, true)
            }
        },
        IpgWidget::RightTriangle(tr) => {
            for (idx, point) in tr.points.iter().enumerate() {
                let dist = cursor.distance(*point);
                if  dist < point_dist {
                    point_index = idx;
                    point_dist = dist;
                }
            };
            
            let mid_dist = tr.mid_point.distance(cursor);
            let tr_pt_dist = tr.tr_point.distance(cursor);

            if point_dist < mid_dist && point_dist < tr_pt_dist {
                (Some(point_index), false, false)
            } else if mid_dist < tr_pt_dist {
                (None, true, false)
            } else {
                (None, false, true)
            }
        },
        IpgWidget::FreeHand(fh) => {
            for (idx, point) in fh.points.iter().enumerate() {
                let dist = cursor.distance(*point);
                if  dist < point_dist {
                    point_index = idx;
                    point_dist = dist;
                }
            };
            (Some(point_index), false, false)
        },
        IpgWidget::Text(_) => {
            // just using the edit_other_point to indicate the position point
            (None, false, true)
        },
        _ => (None, false, false),
    }
    
}


pub fn get_widget_id(widget: &IpgWidget) -> usize {
    dbg!(&widget);
    match widget {
        IpgWidget::Arc(arc) => arc.id,
        IpgWidget::Bezier(bz) => bz.id,
        IpgWidget::Circle(cir) => cir.id,
        IpgWidget::Ellipse(ell) => ell.id,
        IpgWidget::Image(img) => img.id,
        IpgWidget::Line(line) => line.id,
        IpgWidget::PolyLine(pl) => pl.id,
        IpgWidget::Polygon(pg) => pg.id,
        IpgWidget::Rectangle(rect) => rect.id,
        IpgWidget::RightTriangle(tr) => tr.id,
        IpgWidget::FreeHand(fh) => fh.id,
        IpgWidget::Text(txt) => txt.id,
        IpgWidget::None => 0,
    }
}

pub fn get_widget_degrees(widget: &IpgWidget) -> Option<f32> {
    match widget {
        IpgWidget::None => Some(0.0),
        IpgWidget::Arc(arc) => Some(Radians::into(arc.start_angle)),
        IpgWidget::Bezier(bezier) => Some(bezier.rotation),
        IpgWidget::Circle(_circle) => Some(0.0),
        IpgWidget::Ellipse(_ell) => Some(0.0),
        IpgWidget::Image(image) => Some(image.rotation),
        IpgWidget::Line(line) => Some(line.rotation),
        IpgWidget::PolyLine(pl) => Some(pl.rotation),
        IpgWidget::Polygon(pg) => Some(pg.rotation),
        IpgWidget::Rectangle(rect) => Some(rect.rotation),
        IpgWidget::RightTriangle(tr) => Some(tr.rotation),
        IpgWidget::FreeHand(_) => None,
        IpgWidget::Text(txt) => Some(txt.rotation),
    }
}

pub fn get_draw_mode_and_status(widget: &IpgWidget) -> (IpgDrawMode, IpgDrawStatus) {
    match widget {
        IpgWidget::None => (IpgDrawMode::Display, IpgDrawStatus::Completed),
        IpgWidget::Arc(arc) => (arc.draw_mode, arc.status),
        IpgWidget::Bezier(bz) => (bz.draw_mode, bz.status),
        IpgWidget::Circle(cir) => (cir.draw_mode, cir.status),
        IpgWidget::Ellipse(ell) => (ell.draw_mode, ell.status),
        IpgWidget::Image(image) => (image.draw_mode, image.status),
        IpgWidget::Line(ln) => (ln.draw_mode, ln.status),
        IpgWidget::PolyLine(pl) => (pl.draw_mode, pl.status),
        IpgWidget::Polygon(pg) => (pg.draw_mode, pg.status),
        IpgWidget::Rectangle(rect) => (rect.draw_mode, rect.status),
        IpgWidget::RightTriangle(tr) => (tr.draw_mode, tr.status),
        IpgWidget::FreeHand(fh) => (fh.draw_mode, fh.status),
        IpgWidget::Text(txt) => (txt.draw_mode, txt.status),
    }
}

pub fn get_distance_to_mid_point(widget: &IpgWidget, cursor: Point) -> f32 {

        match &widget {
            IpgWidget::Arc(arc) => {
                cursor.distance(arc.mid_point)
            },
            IpgWidget::Bezier(bz) => {
                cursor.distance(bz.mid_point)
            },
            IpgWidget::Circle(cir) => {
                cursor.distance(cir.center)
            },
            IpgWidget::Ellipse(ell) => {
                cursor.distance(ell.center)
            },
            IpgWidget::Line(line) => {
                cursor.distance(line.mid_point)
            },
            IpgWidget::Polygon(pg) => {
                cursor.distance(pg.mid_point)
            },
            IpgWidget::PolyLine(pl) => {
                cursor.distance(pl.mid_point)
            },
            IpgWidget::RightTriangle(tr) => {
                cursor.distance(tr.mid_point)
            },
            IpgWidget::FreeHand(fh) => {
                cursor.distance(fh.points[0])
            },
            IpgWidget::Text(txt) => {
                cursor.distance(txt.position)
            },
            _ => f32::INFINITY,
        }

}

pub fn get_mid_geometry(pts: &[Point], curve_type: IpgCanvasWidget) -> Point {
    match curve_type {
        IpgCanvasWidget::Arc => {
            get_mid_point(pts[0], pts[1])
        }
        IpgCanvasWidget::Bezier => {
            get_mid_point(pts[0], pts[1])
        },
        IpgCanvasWidget::Circle => {
            // return the center point
            pts[0]
        },
        IpgCanvasWidget::Ellipse => {
            // return the center point
            pts[0]
        }
        IpgCanvasWidget::Line => {
            get_mid_point(pts[0], pts[1])
        },
        IpgCanvasWidget::PolyLine => {

            let (slope, intercept) = get_linear_regression(pts);

            let (p1, p2) = get_line_from_slope_intercept(pts, slope, intercept);

            get_mid_point(p1, p2)

        },
        IpgCanvasWidget::Polygon => {
            // return the center point
            pts[0]
        },
        IpgCanvasWidget::Rectangle => {
            pts[0]
        }
        IpgCanvasWidget::RightTriangle => {
            let x = (pts[0].x + pts[1].x + pts[2].x)/3.0;
            let y = (pts[0].y + pts[1].y + pts[2].y)/3.0;
            Point {x, y}
        },
        IpgCanvasWidget::FreeHand => {
            pts[0]
        }
        IpgCanvasWidget::Text => {
            pts[0]
        }
        IpgCanvasWidget::None => Point::default(),
    }
    
}
