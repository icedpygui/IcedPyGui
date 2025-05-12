//! path_builds

use std::f32::consts::PI;

use iced::{widget::canvas::{self, path::arc::Elliptical, Path}, Point, Radians, Vector};
use crate::chart::geometries::{IpgCircle, IpgEllipse, 
    ChartLine, IpgPolyLine, IpgPolygon, IpgText, IpgChartWidget};
use crate::{chart::draw_chart::IpgDrawMode, 
chart::chart_helpers::{build_polygon, 
    get_horizontal_angle_of_vector, get_mid_point, rotate_geometry, 
    translate_geometry}};

use super::geometries::get_mid_geometry;

pub fn build_circle_path(cir: &IpgCircle, 
                    draw_mode: IpgDrawMode, 
                    pending_cursor: Option<Point>,
                    edit_point_index: Option<usize>, 
                    edit_mid_point: bool,
                ) -> Path {
    Path::new(|p| {
        match draw_mode {
            IpgDrawMode::Display => {
                p.circle(cir.center, cir.radius);
            },
            IpgDrawMode::Edit => {
                let mut center = cir.center;
                let mut cir_point = cir.circle_point;
                let mut radius = cir.radius;

                if edit_mid_point {
                    cir_point = translate_geometry(
                        &[cir_point], 
                        pending_cursor.unwrap(),
                        center,
                    )[0];
                    center = pending_cursor.unwrap();
                }

                if edit_point_index.is_some() {
                    cir_point = pending_cursor.unwrap();
                    radius = center.distance(cir_point);
                }

                p.circle(center, radius);
                p.circle(center, 3.0);
                p.circle(cir_point, 3.0);
            },
            IpgDrawMode::New => {
                let circle_point = pending_cursor.unwrap();
                let radius = cir.center.distance(circle_point);
                p.move_to(cir.center);
                p.line_to(circle_point);
                p.circle(cir.center, radius);
                
            },
        }
    })

}

pub fn build_ellipse_path(ell: &IpgEllipse, 
                        draw_mode: IpgDrawMode, 
                        pending_cursor: Option<Point>,
                        edit_point_index: Option<usize>, 
                        edit_mid_point: bool,
                    ) -> Path {
    Path::new(|p| {
        match draw_mode {
            IpgDrawMode::Display => {
                p.ellipse(Elliptical{ 
                    center: ell.center, 
                    radii: ell.radii, 
                    rotation: ell.rotation, 
                    start_angle: Radians(0.0), 
                    end_angle: Radians(2.0*PI) 
                });
            },
            IpgDrawMode::Edit => {
                let mut center = ell.center;
                let mut radii = ell.radii;
                let mut p1 = ell.points[1];
                let mut p2 = ell.points[2];

                if edit_mid_point {
                    let points = translate_geometry(
                        &ell.points, 
                        pending_cursor.unwrap(),
                        center,
                    );
                    center = pending_cursor.unwrap();
                    p1 = points[1];
                    p2 = points[2];
                }

                if edit_point_index.is_some() {
                    let cursor = pending_cursor.unwrap();
                    if edit_point_index == Some(1) {
                        let vx = cursor.distance(center);
                        let vy = ell.points[2].distance(center);
                        p1 = Point::new(cursor.x, center.y);
                        radii = Vector{x: vx, y: vy};
                    } else {
                        let vx = ell.points[1].distance(center);
                        let vy = cursor.distance(center);
                        p2 = Point::new(center.x, cursor.y);
                        radii = Vector{x: vx, y: vy};
                    }
                }

                p.circle(center, 3.0);
                p.circle(p1, 3.0);
                p.circle(p2, 3.0);
                p.ellipse(Elliptical{ 
                    center, 
                    radii, 
                    rotation: ell.rotation, 
                    start_angle: Radians(0.0), 
                    end_angle: Radians(2.0*PI) 
                });
            },
            IpgDrawMode::New => {
                let cursor = pending_cursor.unwrap();
                if !ell.points.is_empty(){
                    p.move_to(ell.points[0]);

                }
                if ell.points.is_empty() {
                    p.circle(cursor, 3.0);
                } else if ell.points.len() == 1 {
                    let p1 = Point::new(cursor.x, ell.points[0].y);
                    p.line_to(p1);
                    let radius = p1.distance(ell.points[0]);
                    p.circle(ell.points[0], radius);
                } else if ell.points.len() == 2 {
                    p.line_to(ell.points[1]);
                    let p2 = Point::new(ell.points[0].x, cursor.y);
                    p.line_to(p2);
                    let vx = ell.points[1].distance(ell.points[0]);
                    let vy = Point::new(ell.points[0].x, cursor.y).distance(ell.points[0]);
                    p.ellipse(Elliptical{ 
                        center: ell.points[0], 
                        radii: Vector{x: vx, y: vy}, 
                        rotation: Radians(0.0), 
                        start_angle: Radians(0.0), 
                        end_angle: Radians(2.0*PI) 
                    });
                }
                
            },
        }
    })
}

pub fn build_line_path(line: &ChartLine, 
                    draw_mode: IpgDrawMode, 
                    pending_cursor: Option<Point>,
                    edit_point_index: Option<usize>, 
                    edit_mid_point: bool,
                    degrees: Option<f32>,
                    ) -> (Path, f32, Point) {

    let mut degrees = match degrees {
        Some(d) => d,
        None => line.rotation,
    };
    let mut mid_point = line.mid_point;

    let path = Path::new(|p| {
        match draw_mode {
            IpgDrawMode::Display => {
                p.move_to(line.points[0]);
                p.line_to(line.points[1]);
            },
            IpgDrawMode::Edit => {
                let mut pts = line.points.clone();

                if edit_mid_point {
                    pts = translate_geometry(
                        &pts, 
                        pending_cursor.unwrap(),
                        mid_point,
                    );
                    mid_point = pending_cursor.unwrap();
                };

                if edit_point_index.is_some() {
                    pts[edit_point_index.unwrap()] = pending_cursor.unwrap();
                    mid_point = get_mid_point(pts[0], pts[1])
                }

                degrees = 
                    get_horizontal_angle_of_vector(
                        pts[0],  
                        pts[1], 
                    );

                p.move_to(pts[0]);
                p.line_to(pts[1]);
                p.circle(pts[0], 3.0);
                p.circle(pts[1], 3.0);
                p.circle(mid_point, 3.0);
            },
            IpgDrawMode::New => {
                p.move_to(line.points[0]);
                p.line_to(pending_cursor.unwrap());

                degrees = 
                    get_horizontal_angle_of_vector(
                        line.points[0], 
                        pending_cursor.unwrap(), 
                    );
            },
        }
    });

    (path, degrees, mid_point)

}

pub fn build_polygon_path(pg: &IpgPolygon, 
                        draw_mode: IpgDrawMode, 
                        pending_cursor: Option<Point>,
                        edit_mid_point: bool,
                        edit_other_point: bool,
                        degrees: Option<f32>,
                        ) -> (Path, f32, Point) {

    let mut degrees = match degrees {
        Some(d) => d,
        None => pg.rotation,
    };
    let mut mid_point = pg.mid_point;
    let mut pg_point = pg.pg_point;

    let path = Path::new(|p| {
        match draw_mode {
            IpgDrawMode::Display => {
                let points = &pg.points;
                for (index, point) in points.iter().enumerate() {
                    if index == 0 {
                        p.move_to(*point);
                    } else {
                        p.line_to(*point);
                    }
                }
                p.line_to(points[0]);
            },
            IpgDrawMode::Edit => {
                if edit_mid_point {
                    pg_point = translate_geometry(
                        &[pg.pg_point], 
                        pending_cursor.unwrap(),
                        pg.mid_point, 
                    )[0];
                    mid_point = pending_cursor.unwrap();
                } 
                if edit_other_point {
                    pg_point = pending_cursor.unwrap();
                    degrees = get_horizontal_angle_of_vector(pg.mid_point, pending_cursor.unwrap());
                }
                
                let pts = 
                    build_polygon(
                        mid_point, 
                        pg_point, 
                        pg.poly_points,
                        degrees
                    );
                
                for (index, pt) in pts.iter().enumerate() {
                    if index == 0 {
                        p.move_to(*pt);
                    } else {
                        p.line_to(*pt);
                    }
                }
                p.line_to(pts[0]);
                p.move_to(mid_point);
                p.line_to(pg_point);
                p.circle(mid_point, 3.0);
                p.circle(pg_point, 3.0);
            },
            IpgDrawMode::New => {
                degrees = get_horizontal_angle_of_vector(pg.mid_point, pending_cursor.unwrap());

                let points = 
                    build_polygon(
                        pg.mid_point, 
                        pending_cursor.unwrap(), 
                        pg.poly_points,
                        degrees,
                    );
                
                for (index, point) in points.iter().enumerate() {
                    if index == 0 {
                        p.move_to(*point);
                    } else {
                        p.line_to(*point);
                    }
                }
                p.move_to(pg.mid_point);
                p.line_to(pending_cursor.unwrap());
                p.circle(points[0], 3.0);
            },
        }
    });

    (path, degrees, mid_point)

}

pub fn build_polyline_path(pl: &IpgPolyLine, 
                        draw_mode: IpgDrawMode, 
                        pending_cursor: Option<Point>,
                        edit_point_index: Option<usize>, 
                        edit_mid_point: bool,
                        edit_other_point: bool,
                        degrees: Option<f32>,
                        ) -> (Path, f32, Point) {

    let mut degrees = match degrees {
        Some(d) => d,
        None => pl.rotation,
    };
    let mut pts = pl.points.clone();
    let mut mid_point = pl.mid_point;
    let mut pl_point = pl.pl_point;

    let path = Path::new(|p| {
        match draw_mode {
            IpgDrawMode::Display => {
                for (index, point) in pl.points.iter().enumerate() {
                    if index == 0 {
                        p.move_to(*point);
                    } else {
                        p.line_to(*point);
                    }
                }
            },
            IpgDrawMode::Edit => {
                if edit_mid_point {
                    pts.push(pl_point);
                    pts = translate_geometry(
                        &pts, 
                        pending_cursor.unwrap(),
                        mid_point, 
                    );
                    pl_point = pts.pop().unwrap();
                    mid_point = pending_cursor.unwrap();
                } 
                if edit_point_index.is_some() {
                    pts[edit_point_index.unwrap()] = pending_cursor.unwrap();
                    mid_point = get_mid_geometry(&pts, IpgChartWidget::PolyLine);
                    pl_point = translate_geometry(
                                    &[pl_point], 
                                    mid_point, 
                                    pl.mid_point,
                                )[0];
                }
                if edit_other_point {
                    degrees = get_horizontal_angle_of_vector(pl.mid_point, pending_cursor.unwrap());
                    let step_degrees = degrees-pl.rotation;
                    pts = rotate_geometry(&pts, &mid_point, &step_degrees, IpgChartWidget::PolyLine);
                    pl_point = pending_cursor.unwrap();

                }

                for (index, point) in pts.iter().enumerate() {
                    if index == 0 {
                        p.move_to(*point);
                    } else {
                        p.line_to(*point);
                    }
                }
                for pt in pts.iter() {
                    p.circle(*pt, 3.0);
                }
                p.circle(mid_point, 3.0);
                p.move_to(mid_point);
                p.line_to(pl_point);
                p.circle(pl_point, 3.0);
            },
            IpgDrawMode::New => {
                for (index, point) in pl.points.iter().enumerate() {
                    if index == 0 {
                        p.move_to(*point);
                    } else {
                        p.line_to(*point);
                    }
                }
                p.line_to(pending_cursor.unwrap());
            },
        }
    });

    (path, degrees, mid_point)

}

pub fn build_text_path (txt: &IpgText, 
                    draw_mode: IpgDrawMode, 
                    blink: bool,
                    ) -> (canvas::Text, Option<Path>) {

        let mut text = canvas::Text {
                    content: txt.content.clone(),
                    position: Point::ORIGIN,
                    color: txt.color,
                    size: txt.size,
                    line_height: txt.line_height,
                    font: txt.font,
                    horizontal_alignment: txt.horizontal_alignment,
                    vertical_alignment: txt.vertical_alignment,
                    shaping: txt.shaping,
                };

        match draw_mode {
            IpgDrawMode::Display => { 
                (text, None)
            },
            IpgDrawMode::Edit => {
                let path = Some(Path::new(|p| {
                    p.circle(Point::new(0.0, 0.0), 3.0);
                }));
                (text, path)
            },
            IpgDrawMode::New => {
                let mut text_cont = txt.content.clone();
                if blink {
                    text_cont.push('|');
                }
                text.content = text_cont;
                (text, None)
            },
        }

                  
}
