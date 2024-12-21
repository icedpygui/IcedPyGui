//! geometries

use std::collections::HashMap;

use iced::{alignment, keyboard::Key, widget::{container::Id, text::{LineHeight, Shaping}}, Color, Font, Pixels, Point, Radians, Vector};
use serde::{Deserialize, Serialize};

use super::{canvas_helpers::{build_polygon, get_angle_of_vectors, get_horizontal_angle_of_vector, get_line_from_slope_intercept, get_linear_regression, get_mid_point, rotate_geometry, to_degrees, to_radians, translate_geometry}, draw_canvas::{IpgCanvasWidget, IpgDrawMode, IpgDrawStatus}};



#[derive(Debug, Clone)]
pub struct IpgArc {
    pub id: Id,
    pub points: Vec<Point>,
    pub mid_point: Point,
    pub radius: f32,
    pub color: Color,
    pub width: f32,
    pub start_angle: Radians,
    pub end_angle: Radians,
    pub draw_mode: IpgDrawMode,
    pub status: IpgDrawStatus,
}

#[derive(Debug, Clone)]
pub struct IpgBezier {
    pub id: Id,
    pub points: Vec<Point>,
    pub mid_point: Point,
    pub color: Color,
    pub width: f32,
    pub degrees: f32,
    pub draw_mode: IpgDrawMode,
    pub status: IpgDrawStatus,
}

#[derive(Debug, Clone)]
pub struct IpgCircle {
    pub id: Id,
    pub center: Point,
    pub circle_point: Point,
    pub radius: f32,
    pub color: Color,
    pub width: f32,
    pub draw_mode: IpgDrawMode,
    pub status: IpgDrawStatus,
}

#[derive(Debug, Clone)]
pub struct IpgEllipse {
    pub id: Id,
    pub points: Vec<Point>,
    pub center: Point,
    pub radii: Vector,
    pub rotation: Radians,
    pub color: Color,
    pub width: f32,
    pub draw_mode: IpgDrawMode,
    pub status: IpgDrawStatus,
}

#[derive(Debug, Clone)]
pub struct IpgLine {
    pub id: Id,
    pub points: Vec<Point>,
    pub mid_point: Point,
    pub color: Color,
    pub width: f32,
    pub degrees: f32,
    pub draw_mode: IpgDrawMode,
    pub status: IpgDrawStatus,
}

#[derive(Debug, Clone)]
pub struct IpgPolyLine {
    pub id: Id,
    pub points: Vec<Point>,
    pub poly_points: usize,
    pub mid_point: Point,
    pub pl_point: Point,
    pub color: Color,
    pub width: f32,
    pub degrees: f32,
    pub draw_mode: IpgDrawMode,
    pub status: IpgDrawStatus,
}

#[derive(Debug, Clone)]
pub struct IpgPolygon {
    pub id: Id,
    pub points: Vec<Point>,
    pub poly_points: usize,
    pub mid_point: Point,
    pub pg_point: Point,
    pub color: Color,
    pub width: f32,
    pub degrees: f32,
    pub draw_mode: IpgDrawMode,
    pub status: IpgDrawStatus,
}

#[derive(Debug, Clone)]
pub struct IpgRightTriangle {
    pub id: Id,
    pub points: Vec<Point>,
    pub mid_point: Point,
    pub tr_point: Point,
    pub color: Color,
    pub width: f32,
    pub degrees: f32,
    pub draw_mode: IpgDrawMode,
    pub status: IpgDrawStatus,
}

#[derive(Debug, Clone)]
pub struct IpgText {
    pub id: Id,
    pub content: String,
    pub position: Point,
    pub color: Color,
    pub size: Pixels,
    pub line_height: LineHeight,
    pub font: Font,
    pub horizontal_alignment: alignment::Horizontal,
    pub vertical_alignment: alignment::Vertical,
    pub shaping: Shaping,
    pub degrees: f32,
    pub draw_mode: IpgDrawMode,
    pub status: IpgDrawStatus,
    pub blink_position: usize,
}

#[derive(Debug, Clone)]
pub struct IpgFreeHand {
    pub id: Id,
    pub points: Vec<Point>,
     pub color: Color,
    pub width: f32,
    pub draw_mode: IpgDrawMode,
    pub status: IpgDrawStatus,
    pub completed: bool,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq,)]
pub enum Widget {
    None,
    Arc,
    Bezier,
    Circle,
    Ellipse,
    Line,
    PolyLine,
    Polygon,
    RightTriangle,
    Text,
    FreeHand,
}

pub fn add_new_widget(widget: Widget, 
                    poly_points: usize, 
                    color: Color,
                    width: f32,
                    draw_mode: IpgDrawMode) 
                    -> IpgCanvasWidget {
    match widget {
        Widget::None => {
            IpgCanvasWidget::None
        },
        Widget::Arc => {
            IpgCanvasWidget::Arc(
                IpgArc {
                    id: Id::unique(),
                    points: vec![],
                    mid_point: Point::default(),
                    radius: 0.0,
                    color,
                    width,
                    start_angle: Radians::PI,
                    end_angle: Radians::PI,
                    draw_mode,
                    status: IpgDrawStatus::Inprogress,
                })
        
        },
        Widget::Bezier => {
            IpgCanvasWidget::Bezier(
                IpgBezier { 
                    id: Id::unique(),
                    points: vec![],
                    mid_point: Point::default(),
                    color, 
                    width, 
                    degrees: 0.0, 
                    draw_mode,
                    status: IpgDrawStatus::Inprogress,
                }
            )
        },
        Widget::Circle => {
            IpgCanvasWidget::Circle(
                IpgCircle {
                    id: Id::unique(),
                    center: Point::default(),
                    circle_point: Point::default(),
                    radius: 0.0,
                    color,
                    width,
                    draw_mode,
                    status: IpgDrawStatus::Inprogress,
                }
            )
        },
        Widget::Ellipse => {
            IpgCanvasWidget::Ellipse(
                IpgEllipse {
                    id: Id::unique(),
                    points: vec![],
                    center: Point::default(),
                    radii: Vector{x: 0.0, y: 0.0},
                    rotation: Radians(0.0),
                    color,
                    width,
                    draw_mode,
                    status: IpgDrawStatus::Inprogress,
                }
            )
        },
        Widget::Line => {
            IpgCanvasWidget::Line(
                IpgLine {
                    id: Id::unique(),
                    points: vec![],
                    mid_point: Point::default(),
                    color,
                    width,
                    degrees: 0.0,
                    draw_mode,
                    status: IpgDrawStatus::Inprogress,
                }
            )
        },
        Widget::PolyLine => {
            IpgCanvasWidget::PolyLine(
                IpgPolyLine {
                    id: Id::unique(),
                    points: vec![],
                    poly_points,
                    mid_point: Point::default(),
                    pl_point: Point::default(),
                    color,
                    width,
                    degrees: 0.0,
                    draw_mode,
                    status: IpgDrawStatus::Inprogress,
                }
            )
        },
        Widget::Polygon => {
            IpgCanvasWidget::Polygon(
                IpgPolygon {
                    id: Id::unique(),
                    points: vec![],
                    poly_points,
                    mid_point: Point::default(),
                    pg_point: Point::default(),
                    color,
                    width,
                    degrees: 0.0,
                    draw_mode,
                    status: IpgDrawStatus::Inprogress,
                }
            )
        },
        Widget::RightTriangle => {
            IpgCanvasWidget::RightTriangle(
                IpgRightTriangle {
                    id: Id::unique(),
                    points: vec![],
                    mid_point: Point::default(),
                    tr_point: Point::default(),
                    color,
                    width,
                    degrees: 0.0,
                    draw_mode,
                    status: IpgDrawStatus::Inprogress,
                }
            )
        },
        Widget::FreeHand => {
            IpgCanvasWidget::FreeHand(
                IpgFreeHand {
                    id: Id::unique(),
                    points: vec![],
                    color,
                    width,
                    draw_mode,
                    status: IpgDrawStatus::Inprogress,
                    completed: false,
                }
            )
        }
        Widget::Text => {
            IpgCanvasWidget::Text(
                IpgText {
                    id: Id::unique(),
                    content: String::new(),
                    position: Point::default(),
                    color,
                    size: Pixels(16.0),
                    line_height: LineHeight::Relative(1.2),
                    font: Default::default(),
                    horizontal_alignment: alignment::Horizontal::Left,
                    vertical_alignment: alignment::Vertical::Top,
                    shaping: Shaping::Basic,
                    degrees: 0.0,
                    draw_mode,
                    status: IpgDrawStatus::TextInProgress,
                    blink_position: 0,
                }
            )
        },
    }
}

pub fn complete_new_widget(widget: IpgCanvasWidget, cursor: Point) -> Option<IpgCanvasWidget> {
    match widget {
        IpgCanvasWidget::None => {
            None
        },
        IpgCanvasWidget::Arc(arc) => {
            Some(IpgCanvasWidget::Arc(arc))
        },
        IpgCanvasWidget::Bezier(mut bz) => {
            bz.mid_point = 
                get_mid_point(
                    bz.points[0], 
                    bz.points[1]
                );
            Some(IpgCanvasWidget::Bezier(bz))
        },
        IpgCanvasWidget::Circle(cir) => { 
            Some(IpgCanvasWidget::Circle(cir))
        },
        IpgCanvasWidget::Ellipse(mut ell) => {
            ell.center = ell.points[0];
            let vx = ell.points[1].distance(ell.center);
            let vy = cursor.distance(ell.center);
            ell.radii = Vector{ x: vx, y: vy };
            Some(IpgCanvasWidget::Ellipse(ell))
        },
        IpgCanvasWidget::Line(mut ln) => {
            // degree is angle rotation around mid point 
            let degrees = 
                get_horizontal_angle_of_vector(
                    ln.points[0],
                    ln.points[1], 
                );
            ln.degrees = degrees;

            Some(IpgCanvasWidget::Line(ln))
        },
        IpgCanvasWidget::Polygon(mut pg) => {
            pg.pg_point = cursor;
            let degrees = 
                get_horizontal_angle_of_vector(
                    pg.mid_point, 
                    cursor, 
                    );

            pg.degrees = degrees;
            pg.points = 
                build_polygon(
                    pg.mid_point, 
                    pg.pg_point, 
                    pg.poly_points,
                    pg.degrees,
                );
            
            Some(IpgCanvasWidget::Polygon(pg))
        },
        IpgCanvasWidget::PolyLine(mut pl) => {
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
            pl.degrees = 
                get_horizontal_angle_of_vector(
                    pl.mid_point,
                    pl.pl_point,
                );
            
            Some(IpgCanvasWidget::PolyLine(pl))
        },
        IpgCanvasWidget::RightTriangle(mut tr) => {
            tr.mid_point = tr.points[1];
            let trans_pts = translate_geometry(&tr.points, Point::default(), tr.points[1]);
            let opp = Point::new(-trans_pts[2].x, -trans_pts[2].y);
            tr.tr_point = Point::new(opp.x+tr.points[1].x, opp.y+tr.points[1].y);
            if tr.points[1].x > tr.points[2].x {
                tr.degrees = 180.0;
            } else {
                tr.degrees = 0.0;
            }
            
            Some(IpgCanvasWidget::RightTriangle(tr))
        },
        IpgCanvasWidget::FreeHand(mut fh) => {
            fh.points.push(cursor);
            Some(IpgCanvasWidget::FreeHand(fh))
        }
        IpgCanvasWidget::Text(mut txt) => {
            txt.degrees = 0.0;
            txt.status = IpgDrawStatus::TextCompleted;
            Some(IpgCanvasWidget::Text(txt))
        }
    }
}

pub fn update_edited_widget(widget: IpgCanvasWidget,
                        cursor: Point, 
                        index: Option<usize>, 
                        mid_point: bool,
                        other_point: bool,
                        status: IpgDrawStatus,
                    ) -> IpgCanvasWidget {
    match widget {
        IpgCanvasWidget::None => {
            IpgCanvasWidget::None
        },
        IpgCanvasWidget::Arc(mut arc) => {
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
                arc.points[2] = translate_geometry(&vec![point_b], arc.mid_point, Point::default())[0];

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
            IpgCanvasWidget::Arc(arc)
        },
        IpgCanvasWidget::Bezier(mut bz) => {
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
            bz.degrees = degrees;
            bz.status = status;
            IpgCanvasWidget::Bezier(bz)
        },
        IpgCanvasWidget::Circle(mut cir) => {
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
            IpgCanvasWidget::Circle(cir)
        },
        IpgCanvasWidget::Ellipse(mut ell) => {
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
            IpgCanvasWidget::Ellipse(ell)
        },
        IpgCanvasWidget::Line(mut line) => {
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
            line.degrees = degrees;
            line.status = status;
            IpgCanvasWidget::Line(line)
        },
        IpgCanvasWidget::Polygon(mut pg) => {
            if other_point {
                pg.pg_point = cursor;
                pg.degrees = get_horizontal_angle_of_vector(pg.mid_point, cursor);
                pg.points = 
                    build_polygon(
                        pg.mid_point, 
                        pg.pg_point, 
                        pg.poly_points,
                        pg.degrees,
                );
            } else if mid_point {
                let trans_pts = 
                    translate_geometry(
                        &vec![pg.pg_point], 
                        cursor,
                        pg.mid_point, 
                    );
                pg.points = 
                    build_polygon(
                        cursor, 
                        trans_pts[0], 
                        pg.poly_points,
                        pg.degrees,
                    );
                pg.mid_point = cursor;
                pg.pg_point = trans_pts[0];
            }
            pg.status = status;
            IpgCanvasWidget::Polygon(pg)
        },
        IpgCanvasWidget::PolyLine(mut pl) => {
            if index.is_some() {
                pl.points[index.unwrap()] = cursor;
                let mid_point = 
                    get_mid_geometry(
                        &pl.points, 
                        Widget::PolyLine
                    );
                pl.pl_point = 
                    translate_geometry(
                        &vec![pl.pl_point], 
                        mid_point, 
                        pl.mid_point
                    )[0];
                pl.mid_point = mid_point;
                pl.degrees = 
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
                let step_degrees = degrees-pl.degrees;
                pl.points = rotate_geometry(&pl.points, &pl.mid_point, &step_degrees, Widget::PolyLine);
                pl.pl_point = cursor;
                pl.degrees = degrees;
            }
            pl.status = status;
            IpgCanvasWidget::PolyLine(pl)
        },
        IpgCanvasWidget::RightTriangle(mut tr) => {
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
                let step_degrees = degrees-tr.degrees;
                tr.points = rotate_geometry(&tr.points, &tr.mid_point, &step_degrees, Widget::RightTriangle);
                tr.tr_point = cursor;
                tr.degrees = degrees;
            }
            tr.status = status;
            IpgCanvasWidget::RightTriangle(tr)
        },
        IpgCanvasWidget::FreeHand(mut fh) => {
            if index.is_some() {
                fh.points[index.unwrap()] = cursor;
            }
            fh.status = status;
            IpgCanvasWidget::FreeHand(fh)
        },
        IpgCanvasWidget::Text(txt) => {
            IpgCanvasWidget::Text(txt)
        }
    }
}

pub fn update_rotated_widget(widget: &mut IpgCanvasWidget, 
                        step_degrees: f32,
                        status: Option<IpgDrawStatus>,
                    ) -> (IpgCanvasWidget, f32) {
    match widget {
        IpgCanvasWidget::None => (IpgCanvasWidget::None, 0.0),
        IpgCanvasWidget::Arc(arc) => {
            arc.points = rotate_geometry(&arc.points, &arc.mid_point, &step_degrees, Widget::Arc);
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

            arc.points[2] = translate_geometry(&vec![point_b], arc.mid_point, Point::default())[0];
            
            if status.is_some() {
                arc.status = status.unwrap();
            }
            (IpgCanvasWidget::Arc(arc.clone()), Radians::into(arc.start_angle))
        },
        IpgCanvasWidget::Bezier(bz) => {
            bz.points = rotate_geometry(&bz.points, &bz.mid_point, &step_degrees, Widget::Bezier);
            bz.degrees = get_horizontal_angle_of_vector(bz.mid_point, bz.points[1]);
            if status.is_some() {
                bz.status = status.unwrap();
            }
            (IpgCanvasWidget::Bezier(bz.clone()), bz.degrees)
        },
        IpgCanvasWidget::Circle(cir) => {
            (IpgCanvasWidget::Circle(cir.clone()), 0.0)
        },
        IpgCanvasWidget::Ellipse(ell) => {
            let rads = to_radians(&step_degrees) + ell.rotation.0;
            ell.rotation = Radians(rads);
            if status.is_some() {
                ell.status = status.unwrap();
            }
            (IpgCanvasWidget::Ellipse(ell.clone()), to_degrees(&rads))
        },
        IpgCanvasWidget::Line(ln) => {
            ln.points = rotate_geometry(&ln.points, &ln.mid_point, &step_degrees, Widget::Line);
            ln.degrees = get_horizontal_angle_of_vector(ln.mid_point, ln.points[1]);
            if status.is_some() {
                ln.status = status.unwrap();
            }
            (IpgCanvasWidget::Line(ln.clone()), ln.degrees)
        },
        IpgCanvasWidget::Polygon(pg) => {
            pg.points = rotate_geometry(&pg.points, &pg.mid_point, &step_degrees, Widget::Polygon);
            pg.pg_point = rotate_geometry(&[pg.pg_point], &pg.mid_point, &step_degrees, Widget::Line)[0];
            pg.degrees = get_horizontal_angle_of_vector(pg.mid_point, pg.pg_point);
            if status.is_some() {
                pg.status = status.unwrap();
            }
            (IpgCanvasWidget::Polygon(pg.clone()), pg.degrees)
        },
        IpgCanvasWidget::PolyLine(pl) => {
            let mut pts = pl.points.clone();
            pts.push(pl.pl_point);
            pts = rotate_geometry(&pts, &pl.mid_point, &step_degrees, Widget::PolyLine);
            pl.pl_point = pts.pop().unwrap();
            pl.points = pts;
            pl.degrees = get_horizontal_angle_of_vector(pl.mid_point, pl.pl_point);
            if status.is_some() {
                pl.status = status.unwrap();
            }
            (IpgCanvasWidget::PolyLine(pl.clone()), pl.degrees)
        },
        IpgCanvasWidget::RightTriangle(tr) => {
            let mut pts = tr.points.clone();
            pts.push(tr.tr_point);
            pts = rotate_geometry(&pts, &tr.mid_point, &step_degrees, Widget::RightTriangle);
            tr.tr_point = pts.pop().unwrap();
            tr.points = pts;
            tr.degrees = get_horizontal_angle_of_vector(tr.mid_point, tr.tr_point);
            if status.is_some() {
                tr.status = status.unwrap();
            }
            (IpgCanvasWidget::RightTriangle(tr.clone()), tr.degrees)
        },
        IpgCanvasWidget::FreeHand(fh) => {
            (IpgCanvasWidget::FreeHand(fh.clone()), 0.0)
        },
        IpgCanvasWidget::Text(txt) => {
            (IpgCanvasWidget::Text(txt.clone()), 0.0)
        }
    }
}

pub fn add_keypress(widget: &mut IpgCanvasWidget, modified: Key) -> (Option<IpgCanvasWidget>, bool) {
    let mut escape = false;
    match widget {
        IpgCanvasWidget::Text(txt) => {
            match modified.as_ref() {
                Key::Named(named) => {
                    match named {
                        iced::keyboard::key::Named::Enter => {
                            txt.content.push_str("\r");
                            txt.blink_position += 1;
                        },
                        iced::keyboard::key::Named::Tab => {
                            txt.content.push_str("    ");
                            txt.blink_position += 4;
                        },
                        iced::keyboard::key::Named::Space => {
                            txt.content.push_str(" ");
                            txt.blink_position += 1;
                        },
                        iced::keyboard::key::Named::Delete => {
                            if txt.blink_position < txt.content.len() {
                                txt.content.remove(txt.blink_position);
                            }
                        },
                        iced::keyboard::key::Named::Escape => escape = true,
                        iced::keyboard::key::Named::Backspace => {
                            if !txt.content.is_empty() && txt.blink_position != 0 {
                                txt.content.remove(txt.blink_position-1);
                                txt.blink_position -= 1;
                            }
                        } 
                        iced::keyboard::key::Named::ArrowLeft => {
                            if txt.blink_position > 0 {
                                txt.blink_position -= 1;
                            }
                        },
                        iced::keyboard::key::Named::ArrowRight => {
                            if txt.blink_position < txt.content.len() {
                                txt.blink_position += 1;
                            }
                        }
                        _ => ()
                    }
                },
                Key::Character(c) => {
                    if txt.content.is_empty() {
                        txt.content.push_str(c);
                    } else if txt.blink_position < txt.content.len() {
                        let c_char = c.chars().next().expect("string is empty");
                        txt.content.insert(txt.blink_position, c_char );
                    } else if txt.blink_position == txt.content.len() {
                            txt.content.push_str(c);
                    }
                    txt.blink_position += 1;
                },
                Key::Unidentified => (),
            }
            if escape {
                (None, false)
            } else {
                (Some(IpgCanvasWidget::Text(txt.clone())), false)
            }
        },
        IpgCanvasWidget::FreeHand(fh) => {
            match modified.as_ref() {
                Key::Named(named) => {
                    match named {
                        iced::keyboard::key::Named::Enter => {
                            fh.completed = true;
                        },
                        _ => ()
                    }
                },
                _ => (),
            }
            
           (Some(IpgCanvasWidget::FreeHand(fh.clone())), fh.completed)
            
        }
        _ => (None, false)
    }
}

pub fn get_del_key(modified: Key) -> bool {
    match modified.as_ref() {
        Key::Named(named) => {
            match named {
                iced::keyboard::key::Named::Delete => true,
                _ => false,
            }
        },
        _ => false,
    }
}

pub fn set_widget_mode_or_status(widget: IpgCanvasWidget, 
                    mode: Option<IpgDrawMode>,
                    status: Option<IpgDrawStatus>,
                    ) -> IpgCanvasWidget {
    match widget {
        IpgCanvasWidget::None => {
            IpgCanvasWidget::None
        },
        IpgCanvasWidget::Arc(mut arc) => {
            if mode.is_some() {
                arc.draw_mode = mode.unwrap();
            }
            if status.is_some() {
                arc.status = status.unwrap();
            }
            IpgCanvasWidget::Arc(arc)
        },
        IpgCanvasWidget::Bezier(mut bz) => {
            if mode.is_some() {
                bz.draw_mode = mode.unwrap();
            }
            if status.is_some() {
                bz.status = status.unwrap();
            }
            IpgCanvasWidget::Bezier(bz)
        },
        IpgCanvasWidget::Circle(mut cir) => {
            if mode.is_some() {
                cir.draw_mode = mode.unwrap();
            }
            if status.is_some() {
                cir.status = status.unwrap();
            }
            IpgCanvasWidget::Circle(cir)
        },
        IpgCanvasWidget::Ellipse(mut ell) => {
            if mode.is_some() {
                ell.draw_mode = mode.unwrap();
            }
            if status.is_some() {
                ell.status = status.unwrap();
            }
            IpgCanvasWidget::Ellipse(ell)
        },
        IpgCanvasWidget::Line(mut ln) => {
            if mode.is_some() {
                ln.draw_mode = mode.unwrap();
            }
            if status.is_some() {
                ln.status = status.unwrap();
            }
            IpgCanvasWidget::Line(ln)
        },
        IpgCanvasWidget::PolyLine(mut pl) => {
            if mode.is_some() {
                pl.draw_mode = mode.unwrap();
            }
            if status.is_some() {
                pl.status = status.unwrap();
            }
            IpgCanvasWidget::PolyLine(pl)
        },
        IpgCanvasWidget::Polygon(mut pg) => {
            if mode.is_some() {
                pg.draw_mode = mode.unwrap();
            }
            if status.is_some() {
                pg.status = status.unwrap();
            }
            IpgCanvasWidget::Polygon(pg)
        },
        IpgCanvasWidget::RightTriangle(mut tr) => {
            if mode.is_some() {
                tr.draw_mode = mode.unwrap();
            }
            if status.is_some() {
                tr.status = status.unwrap();
            }
            IpgCanvasWidget::RightTriangle(tr)
        },
        IpgCanvasWidget::FreeHand(mut fh) => {
            if mode.is_some() {
                fh.draw_mode = mode.unwrap();
            }
            if status.is_some() {
                fh.status = status.unwrap();
            }
            IpgCanvasWidget::FreeHand(fh)
        },
        IpgCanvasWidget::Text(mut txt) => {
            if mode.is_some() {
                txt.draw_mode = mode.unwrap();
            }
            if status.is_some() {
                txt.status = status.unwrap();
            }
            IpgCanvasWidget::Text(txt)
        }
    }
}

// Adds a cursor position to the points then determines 
// if finish by returning the widget and the boolean
pub fn set_widget_point(widget: &IpgCanvasWidget, cursor: Point) -> (IpgCanvasWidget, bool) {
    match widget {
        IpgCanvasWidget::None => (IpgCanvasWidget::None, true),
        IpgCanvasWidget::Arc(arc) => {
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
                    arc.points[2] = translate_geometry(&vec![point_b], arc.mid_point, Point::default())[0];
                    true
                },
                _ => false
            };

            (IpgCanvasWidget::Arc(arc), finished)
        },
        IpgCanvasWidget::Bezier(bezier) => {
            let mut bz = bezier.clone();
            bz.points.push(cursor);

            if bz.points.len() == 2 {
                bz.degrees = get_horizontal_angle_of_vector(bz.points[0], bz.points[1]);
            }
            let finished = if bz.points.len() == 3 {
                true
            } else {
                false
            };
            
            (IpgCanvasWidget::Bezier(bz), finished)
        },
        IpgCanvasWidget::Circle(circle) => {
            let mut cir = circle.clone();
            let finished = if cir.center == Point::default() {
                cir.center = cursor;
                false
            } else {
                cir.radius = cir.center.distance(cursor);
                cir.circle_point = cursor;
                true
            };
            
            (IpgCanvasWidget::Circle(cir), finished)
        },
        IpgCanvasWidget::Ellipse(ell) => {
            let mut ell = ell.clone();
            let finished = if ell.points.len() == 0 {
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
            
            (IpgCanvasWidget::Ellipse(ell), finished)
        },
        IpgCanvasWidget::Line(line) => {
            let mut ln = line.clone();
            ln.points.push(cursor);

            let finished = if ln.points.len() == 2 {
                ln.mid_point = get_mid_point(ln.points[0], ln.points[1]);
                true
            } else {
                false
            };
            
            (IpgCanvasWidget::Line(ln), finished)
        },
        IpgCanvasWidget::PolyLine(poly_line) => {
            let mut pl = poly_line.clone();
            pl.points.push(cursor);
            let finished = if pl.points.len() == pl.poly_points {
                pl.mid_point = get_mid_geometry(&pl.points, Widget::PolyLine);
                true
            } else {
                false
            };
            
            (IpgCanvasWidget::PolyLine(pl), finished)
        },
        IpgCanvasWidget::Polygon(polygon) => {
            let mut pg = polygon.clone();
            let finished = if pg.mid_point == Point::default() {
                pg.mid_point = cursor;
                false
            } else {
                pg.pg_point = cursor;
                true
            };
            if finished {
                pg.degrees = get_horizontal_angle_of_vector(pg.mid_point, pg.pg_point)
            }
            (IpgCanvasWidget::Polygon(pg), finished)
        },
        IpgCanvasWidget::RightTriangle(right_triangle) => {
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
                rt.mid_point = get_mid_geometry(&rt.points, Widget::RightTriangle);
                true
            } else {
                false
            };
            
            (IpgCanvasWidget::RightTriangle(rt), finished)
        },
        IpgCanvasWidget::FreeHand(fh) => {
            let mut fh = fh.clone();
            fh.points.push(cursor);
            let finished = if fh.completed {
                true
            } else {
                false
            };
            
            (IpgCanvasWidget::FreeHand(fh), finished)
        },
        IpgCanvasWidget::Text(text) => {
            let mut txt = text.clone();
            
            let finished = if txt.position == Point::default() {
                txt.position = cursor;
                false
            } else {
                true
            };
            
            (IpgCanvasWidget::Text(txt), finished)
        }
    }
}

pub fn find_closest_widget(curves: &HashMap<Id, IpgCanvasWidget>, cursor: Point) -> Option<IpgCanvasWidget> {
    let mut closest = f32::INFINITY;
    let mut closest_id = None;
    for (id, cw) in curves.iter() {
        let distance: f32 = get_distance_to_mid_point(cw, cursor);
        if distance < closest {
            closest = distance;
            closest_id = Some(id);
        }
    }

    let dc_opt = 
        match closest_id {
            Some(id) => curves.get(id),
            None => None,
        };

    match dc_opt {
        Some(widget) => Some(widget.clone()),
        None => None,
    }
}

// returns a bool if mid_point and an optional usize 
// if a point in points.
pub fn find_closest_point_index(widget: &IpgCanvasWidget,
                            cursor: Point, 
                            ) -> (Option<usize>, bool, bool) {

    let mut point_dist: f32 = f32::INFINITY;
    let mut point_index = 0;

    match widget {
        IpgCanvasWidget::None => (None, false, false),
        IpgCanvasWidget::Arc(arc) => {
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
        IpgCanvasWidget::Bezier(bezier) => {
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
        IpgCanvasWidget::Circle(cir) => {
            let center_dist = cursor.distance(cir.center);
            let point_dist = cursor.distance(cir.circle_point);
            if center_dist < point_dist {
                (None, true, false)
            } else {
                (Some(1), false, false)
            }
        }
        IpgCanvasWidget::Ellipse(ell) => {
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
        IpgCanvasWidget::Line(line) => {
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
        IpgCanvasWidget::Polygon(pg) => {
            let pg_center = cursor.distance(pg.mid_point);
            let pg_point = cursor.distance(pg.pg_point);
            if pg_center <= pg_point {
                (None, true, false)
            } else {
                (None, false, true)
            }
        },
        IpgCanvasWidget::PolyLine(pl) => {
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
        IpgCanvasWidget::RightTriangle(tr) => {
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
        IpgCanvasWidget::FreeHand(fh) => {
            for (idx, point) in fh.points.iter().enumerate() {
                let dist = cursor.distance(*point);
                if  dist < point_dist {
                    point_index = idx;
                    point_dist = dist;
                }
            };
            (Some(point_index), false, false)
        },
        IpgCanvasWidget::Text(txt) => {
            // just putting cursor at the end for now
            // todo find closest point between letters
            (Some(txt.content.len()), false, false)
        }
    }
    
}


pub fn get_widget_id(widget: &IpgCanvasWidget) -> Id {
    match widget {
        IpgCanvasWidget::None => Id::new("None"),
        IpgCanvasWidget::Arc(arc) => arc.id.clone(),
        IpgCanvasWidget::Bezier(bz) => bz.id.clone(),
        IpgCanvasWidget::Circle(cir) => cir.id.clone(),
        IpgCanvasWidget::Ellipse(ell) => ell.id.clone(),
        IpgCanvasWidget::Line(line) => line.id.clone(),
        IpgCanvasWidget::PolyLine(pl) => pl.id.clone(),
        IpgCanvasWidget::Polygon(pg) => pg.id.clone(),
        IpgCanvasWidget::RightTriangle(tr) => tr.id.clone(),
        IpgCanvasWidget::FreeHand(fh) => fh.id.clone(),
        IpgCanvasWidget::Text(txt) => txt.id.clone(),
    }
}

pub fn get_widget_degrees(widget: &IpgCanvasWidget) -> Option<f32> {
    match widget {
        IpgCanvasWidget::None => Some(0.0),
        IpgCanvasWidget::Arc(arc) => Some(Radians::into(arc.start_angle)),
        IpgCanvasWidget::Bezier(bezier) => Some(bezier.degrees),
        IpgCanvasWidget::Circle(_circle) => Some(0.0),
        IpgCanvasWidget::Ellipse(_ell) => Some(0.0),
        IpgCanvasWidget::Line(line) => Some(line.degrees),
        IpgCanvasWidget::PolyLine(poly_line) => Some(poly_line.degrees),
        IpgCanvasWidget::Polygon(polygon) => Some(polygon.degrees),
        IpgCanvasWidget::RightTriangle(right_triangle) => Some(right_triangle.degrees),
        IpgCanvasWidget::FreeHand(_) => None,
        IpgCanvasWidget::Text(txt) => Some(txt.degrees),
    }
}

pub fn get_draw_mode_and_status(widget: &IpgCanvasWidget) -> (IpgDrawMode, IpgDrawStatus) {
    match widget {
        IpgCanvasWidget::None => (IpgDrawMode::DrawAll, IpgDrawStatus::Completed),
        IpgCanvasWidget::Arc(arc) => (arc.draw_mode, arc.status),
        IpgCanvasWidget::Bezier(bz) => (bz.draw_mode, bz.status),
        IpgCanvasWidget::Circle(cir) => (cir.draw_mode, cir.status),
        IpgCanvasWidget::Ellipse(ell) => (ell.draw_mode, ell.status),
        IpgCanvasWidget::Line(ln) => (ln.draw_mode, ln.status),
        IpgCanvasWidget::PolyLine(pl) => (pl.draw_mode, pl.status),
        IpgCanvasWidget::Polygon(pg) => (pg.draw_mode, pg.status),
        IpgCanvasWidget::RightTriangle(tr) => (tr.draw_mode, tr.status),
        IpgCanvasWidget::FreeHand(fh) => (fh.draw_mode, fh.status),
        IpgCanvasWidget::Text(txt) => (txt.draw_mode, txt.status),
    }
}

pub fn get_distance_to_mid_point(widget: &IpgCanvasWidget, cursor: Point) -> f32 {

        match &widget {
            IpgCanvasWidget::None => f32::INFINITY,
            IpgCanvasWidget::Arc(arc) => {
                cursor.distance(arc.mid_point)
            },
            IpgCanvasWidget::Bezier(bz) => {
                cursor.distance(bz.mid_point)
            },
            IpgCanvasWidget::Circle(cir) => {
                cursor.distance(cir.center)
            },
            IpgCanvasWidget::Ellipse(ell) => {
                cursor.distance(ell.center)
            },
            IpgCanvasWidget::Line(line) => {
                cursor.distance(line.mid_point)
            },
            IpgCanvasWidget::Polygon(pg) => {
                cursor.distance(pg.mid_point)
            },
            IpgCanvasWidget::PolyLine(pl) => {
                cursor.distance(pl.mid_point)
            },
            IpgCanvasWidget::RightTriangle(tr) => {
                cursor.distance(tr.mid_point)
            },
            IpgCanvasWidget::FreeHand(fh) => {
                cursor.distance(fh.points[0])
            }
            IpgCanvasWidget::Text(txt) => {
                cursor.distance(txt.position)
            }
        }

}

pub fn get_mid_geometry(pts: &[Point], curve_type: Widget) -> Point {
    match curve_type {
        Widget::Arc => {
            get_mid_point(pts[0], pts[1])
        }
        Widget::Bezier => {
            get_mid_point(pts[0], pts[1])
        },
        Widget::Circle => {
            // return the center point
            pts[0]
        },
        Widget::Ellipse => {
            // return the center point
            pts[0]
        }
        Widget::Line => {
            get_mid_point(pts[0], pts[1])
        },
        Widget::PolyLine => {

            let (slope, intercept) = get_linear_regression(pts);

            let (p1, p2) = get_line_from_slope_intercept(pts, slope, intercept);

            get_mid_point(p1, p2)

        },
        Widget::Polygon => {
            // return the center point
            pts[0]
        },
        Widget::RightTriangle => {
            let x = (pts[0].x + pts[1].x + pts[2].x)/3.0;
            let y = (pts[0].y + pts[1].y + pts[2].y)/3.0;
            Point {x, y}
        },
        Widget::FreeHand => {
            pts[0]
        }
        Widget::Text => {
            pts[0]
        }
        Widget::None => Point::default(),
    }
    
}
