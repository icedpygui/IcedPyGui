//! geometries

use std::collections::HashMap;

use iced::{alignment, keyboard::Key, widget::{image, text::{LineHeight, Shaping}}, Color, Font, Pixels, Point, Radians, Rectangle, Size, Vector};
use pyo3::pyclass;
use serde::{Deserialize, Serialize};

use super::{chart_helpers::{build_polygon, 
    get_horizontal_angle_of_vector, get_line_from_slope_intercept, 
    get_linear_regression, get_mid_point, rotate_geometry, translate_geometry}, 
    draw_chart::{IpgDrawMode, IpgDrawStatus, ChartWidget}};



#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub struct IpgChartImage {
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

#[derive(Debug, Clone, PartialEq)]
pub struct ChartLine {
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

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
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


#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq,)]
#[pyclass(eq, eq_int)]
pub enum IpgChartWidget {
    None,
    Circle,
    Ellipse,
    Line,
    PolyLine,
    Polygon,
    Rectangle,
    Text,
}

pub fn check_if_text_widget(chart_widget: &ChartWidget) -> bool {
    matches!(chart_widget, ChartWidget::Text(_))
}

pub fn add_new_widget(widget: IpgChartWidget, 
                        poly_points: usize, 
                        color: Color,
                        fill_color: Option<Color>,
                        width: f32,
                        draw_mode: IpgDrawMode,
                        h_alignment: alignment::Horizontal,
                        v_alignment: alignment::Vertical,
                        ) 
                        -> ChartWidget {
    match widget {
        IpgChartWidget::Circle => {
            ChartWidget::Circle(
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
        IpgChartWidget::Ellipse => {
            ChartWidget::Ellipse(
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
        IpgChartWidget::Line => {
            ChartWidget::Line(
                ChartLine {
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
        IpgChartWidget::PolyLine => {
            ChartWidget::PolyLine(
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
        IpgChartWidget::Polygon => {
            ChartWidget::Polygon(
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
        IpgChartWidget::Rectangle => {
            ChartWidget::Rectangle(
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
        IpgChartWidget::Text => {
            ChartWidget::Text(
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
        _ => panic!("Chart Widget type not found")

    }
}

pub fn complete_new_widget(widget: ChartWidget, cursor: Point) -> Option<ChartWidget> {
    match widget {
        ChartWidget::Circle(cir) => { 
            Some(ChartWidget::Circle(cir))
        },
        ChartWidget::Ellipse(mut ell) => {
            ell.center = ell.points[0];
            let vx = ell.points[1].distance(ell.center);
            let new_pt = Point{ x: ell.center.x, y: cursor.y };
            let vy = new_pt.distance(ell.center);
            ell.radii = Vector{ x: vx, y: vy };
            Some(ChartWidget::Ellipse(ell))
        },
        ChartWidget::Line(mut ln) => {
            // degree is angle rotation around mid point 
            let degrees = 
                get_horizontal_angle_of_vector(
                    ln.points[0],
                    ln.points[1], 
                );
            ln.rotation = degrees;

            Some(ChartWidget::Line(ln))
        },
        ChartWidget::Polygon(mut pg) => {
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
            
            Some(ChartWidget::Polygon(pg))
        },
        ChartWidget::PolyLine(mut pl) => {
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
            
            Some(ChartWidget::PolyLine(pl))
        },
        ChartWidget::Text(mut txt) => {
            txt.rotation = 0.0;
            txt.status = IpgDrawStatus::Completed;
            Some(ChartWidget::Text(txt))
        },
        _ => {
            None
        },
    }
}

pub fn update_edited_widget(widget: ChartWidget,
                        cursor: Point, 
                        index: Option<usize>, 
                        mid_point: bool,
                        other_point: bool,
                        status: IpgDrawStatus,
                    ) -> ChartWidget {
    match widget {
        ChartWidget::Circle(mut cir) => {
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
            ChartWidget::Circle(cir)
        },
        ChartWidget::Ellipse(mut ell) => {
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
            ChartWidget::Ellipse(ell)
        },
        ChartWidget::Line(mut line) => {
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
            ChartWidget::Line(line)
        },
        ChartWidget::Polygon(mut pg) => {
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
            ChartWidget::Polygon(pg)
        },
        ChartWidget::PolyLine(mut pl) => {
            if index.is_some() {
                pl.points[index.unwrap()] = cursor;
                let mid_point = 
                    get_mid_geometry(
                        &pl.points, 
                        IpgChartWidget::PolyLine
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
                pl.points = rotate_geometry(&pl.points, &pl.mid_point, &step_degrees, IpgChartWidget::PolyLine);
                pl.pl_point = cursor;
                pl.rotation = degrees;
            }
            pl.status = status;
            ChartWidget::PolyLine(pl)
        },
        _ => {
            ChartWidget::None
        },
    }
}


pub fn add_keypress(widget: &mut ChartWidget, modified: Key) -> (Option<ChartWidget>, bool) {
    let mut escape = false;
    match widget {
        ChartWidget::Text(txt) => {
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
                (Some(ChartWidget::Text(txt.clone())), false)
            }
        },
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

pub fn set_widget_mode_or_status_or_id(widget: ChartWidget, 
                    mode: Option<IpgDrawMode>,
                    status: Option<IpgDrawStatus>,
                    id: Option<usize>,
                    ) -> ChartWidget {
    match widget {
        ChartWidget::Circle(mut cir) => {
            if mode.is_some() {
                cir.draw_mode = mode.unwrap();
            }
            if status.is_some() {
                cir.status = status.unwrap();
            }
            if id.is_some() {
                cir.id = id.unwrap();
            }
            ChartWidget::Circle(cir)
        },
        ChartWidget::Ellipse(mut ell) => {
            if mode.is_some() {
                ell.draw_mode = mode.unwrap();
            }
            if status.is_some() {
                ell.status = status.unwrap();
            }
            if id.is_some() {
                ell.id = id.unwrap();
            }
            ChartWidget::Ellipse(ell)
        },
        ChartWidget::Image(mut img) => {
            if mode.is_some() {
                img.draw_mode = mode.unwrap();
            }
            if status.is_some() {
                img.status = status.unwrap();
            }
            if id.is_some() {
                img.id = id.unwrap();
            }
            ChartWidget::Image(img)
        },
        ChartWidget::Line(mut ln) => {
            if mode.is_some() {
                ln.draw_mode = mode.unwrap();
            }
            if status.is_some() {
                ln.status = status.unwrap();
            }
            if id.is_some() {
                ln.id = id.unwrap();
            }
            ChartWidget::Line(ln)
        },
        ChartWidget::PolyLine(mut pl) => {
            if mode.is_some() {
                pl.draw_mode = mode.unwrap();
            }
            if status.is_some() {
                pl.status = status.unwrap();
            }
            if id.is_some() {
                pl.id = id.unwrap();
            }
            ChartWidget::PolyLine(pl)
        },
        ChartWidget::Polygon(mut pg) => {
            if mode.is_some() {
                pg.draw_mode = mode.unwrap();
            }
            if status.is_some() {
                pg.status = status.unwrap();
            }
            if id.is_some() {
                pg.id = id.unwrap();
            }
            ChartWidget::Polygon(pg)
        },
        ChartWidget::Rectangle(mut rect) => {
            if mode.is_some() {
                rect.draw_mode = mode.unwrap();
            }
            if status.is_some() {
                rect.status = status.unwrap();
            }
            if id.is_some() {
                rect.id = id.unwrap();
            }
            ChartWidget::Rectangle(rect)
        },
        ChartWidget::Text(mut txt) => {
            if mode.is_some() {
                txt.draw_mode = mode.unwrap();
            }
            if status.is_some() {
                txt.status = status.unwrap();
            }
            if id.is_some() {
                txt.id = id.unwrap();
            }
            ChartWidget::Text(txt)
        },
        ChartWidget::None => {
            ChartWidget::None
        },
    }
}

// Adds a cursor position to the points then determines 
// if finish by returning the widget and the boolean
pub fn set_widget_point(widget: &ChartWidget, cursor: Point) -> (ChartWidget, bool) {
    match widget {
        ChartWidget::Circle(circle) => {
            let mut cir = circle.clone();
            let finished = if cir.center == Point::default() {
                cir.center = cursor;
                false
            } else {
                cir.radius = cir.center.distance(cursor);
                cir.circle_point = cursor;
                true
            };
            
            (ChartWidget::Circle(cir), finished)
        },
        ChartWidget::Ellipse(ell) => {
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
            
            (ChartWidget::Ellipse(ell), finished)
        },
        ChartWidget::Line(line) => {
            let mut ln = line.clone();
            ln.points.push(cursor);

            let finished = if ln.points.len() == 2 {
                ln.mid_point = get_mid_point(ln.points[0], ln.points[1]);
                true
            } else {
                false
            };
            
            (ChartWidget::Line(ln), finished)
        },
        ChartWidget::PolyLine(poly_line) => {
            let mut pl = poly_line.clone();
            pl.points.push(cursor);
            let finished = if pl.points.len() == pl.poly_points {
                pl.mid_point = get_mid_geometry(&pl.points, IpgChartWidget::PolyLine);
                true
            } else {
                false
            };
            
            (ChartWidget::PolyLine(pl), finished)
        },
        ChartWidget::Polygon(polygon) => {
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
            (ChartWidget::Polygon(pg), finished)
        },
        ChartWidget::Text(text) => {
            let mut txt = text.clone();
            
            let finished = if txt.position == Point::default() {
                txt.position = cursor;
                false
            } else {
                txt.status = IpgDrawStatus::Completed;
                txt.draw_mode = IpgDrawMode::Display;
                true
            };
            
            (ChartWidget::Text(txt), finished)
        },
        _ => (ChartWidget::None, true),
    }
}

pub fn find_closest_widget(curves: &HashMap<usize, ChartWidget>, 
                            text_curves: &HashMap<usize, ChartWidget>,
                            cursor: Point,
                            ) 
                            -> Option<ChartWidget> {
    let mut closest = f32::INFINITY;
    let mut closest_id = None;
    for (id, wid) in curves.iter() {
        if match_ipg_widget(wid) == IpgChartWidget::Circle {
            // do nothing
        } else {
            let distance: f32 = get_distance_to_mid_point(wid, cursor);
            if distance < closest {
                closest = distance;
                closest_id = Some(id);
            }
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
pub fn find_closest_point_index(widget: &ChartWidget,
                            cursor: Point, 
                            ) -> (Option<usize>, bool, bool) {

    let mut point_dist: f32 = f32::INFINITY;
    let mut point_index = 0;

    match widget {
        ChartWidget::Circle(cir) => {
            let center_dist = cursor.distance(cir.center);
            let point_dist = cursor.distance(cir.circle_point);
            if center_dist < point_dist {
                (None, true, false)
            } else {
                (Some(1), false, false)
            }
        }
        ChartWidget::Ellipse(ell) => {
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
        ChartWidget::Line(line) => {
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
        ChartWidget::Polygon(pg) => {
            let pg_center = cursor.distance(pg.mid_point);
            let pg_point = cursor.distance(pg.pg_point);
            if pg_center <= pg_point {
                (None, true, false)
            } else {
                (None, false, true)
            }
        },
        ChartWidget::PolyLine(pl) => {
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
        ChartWidget::Text(_) => {
            // just using the edit_other_point to indicate the position point
            (None, false, true)
        },
        _ => (None, false, false),
    }
    
}


pub fn get_widget_id(widget: &ChartWidget) -> usize {
    match widget {
        ChartWidget::Circle(cir) => cir.id,
        ChartWidget::Ellipse(ell) => ell.id,
        ChartWidget::Image(img) => img.id,
        ChartWidget::Line(line) => line.id,
        ChartWidget::PolyLine(pl) => pl.id,
        ChartWidget::Polygon(pg) => pg.id,
        ChartWidget::Rectangle(rect) => rect.id,
        ChartWidget::Text(txt) => txt.id,
        ChartWidget::None => 0,
    }
}

pub fn get_draw_mode_and_status(widget: &ChartWidget) -> (IpgDrawMode, IpgDrawStatus) {
    match widget {
        ChartWidget::None => (IpgDrawMode::Display, IpgDrawStatus::Completed),
        ChartWidget::Circle(cir) => (cir.draw_mode, cir.status),
        ChartWidget::Ellipse(ell) => (ell.draw_mode, ell.status),
        ChartWidget::Image(image) => (image.draw_mode, image.status),
        ChartWidget::Line(ln) => (ln.draw_mode, ln.status),
        ChartWidget::PolyLine(pl) => (pl.draw_mode, pl.status),
        ChartWidget::Polygon(pg) => (pg.draw_mode, pg.status),
        ChartWidget::Rectangle(rect) => (rect.draw_mode, rect.status),
        ChartWidget::Text(txt) => (txt.draw_mode, txt.status),
    }
}

pub fn get_distance_to_mid_point(widget: &ChartWidget, cursor: Point) -> f32 {

        match &widget {
            ChartWidget::Circle(cir) => {
                cursor.distance(cir.center)
            },
            ChartWidget::Ellipse(ell) => {
                cursor.distance(ell.center)
            },
            ChartWidget::Line(line) => {
                cursor.distance(line.mid_point)
            },
            ChartWidget::Polygon(pg) => {
                cursor.distance(pg.mid_point)
            },
            ChartWidget::PolyLine(pl) => {
                cursor.distance(pl.mid_point)
            },
            ChartWidget::Text(txt) => {
                cursor.distance(txt.position)
            },
            _ => f32::INFINITY,
        }

}

pub fn get_mid_geometry(pts: &[Point], curve_type: IpgChartWidget) -> Point {
    match curve_type {
        IpgChartWidget::Circle => {
            // return the center point
            pts[0]
        },
        IpgChartWidget::Ellipse => {
            // return the center point
            pts[0]
        }
        IpgChartWidget::Line => {
            get_mid_point(pts[0], pts[1])
        },
        IpgChartWidget::PolyLine => {

            let (slope, intercept) = get_linear_regression(pts);

            let (p1, p2) = get_line_from_slope_intercept(pts, slope, intercept);

            get_mid_point(p1, p2)

        },
        IpgChartWidget::Polygon => {
            // return the center point
            pts[0]
        },
        IpgChartWidget::Rectangle => {
            pts[0]
        }
        IpgChartWidget::Text => {
            pts[0]
        }
        IpgChartWidget::None => {
            pts[0]
        }
    }
    
}

fn match_ipg_widget(widget: &ChartWidget) -> IpgChartWidget {
    match widget {
        ChartWidget::None => IpgChartWidget::None,
        ChartWidget::Circle(_) => IpgChartWidget::Circle,
        ChartWidget::Ellipse(_) => IpgChartWidget::Ellipse,
        ChartWidget::Image(_) => IpgChartWidget::None,
        ChartWidget::Line(_) => IpgChartWidget::Line,
        ChartWidget::PolyLine(_) => IpgChartWidget::PolyLine,
        ChartWidget::Polygon(_) => IpgChartWidget::Polygon,
        ChartWidget::Rectangle(_) => IpgChartWidget::Rectangle,
        ChartWidget::Text(_) => IpgChartWidget::Text,
    }
}