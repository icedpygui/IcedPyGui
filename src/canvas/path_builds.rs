//! path_builds

use std::f32::consts::PI;

use iced::{widget::canvas::{self, path::arc::Elliptical, Path}, Point, Radians, Vector};
use crate::canvas::geometries::{IpgArc, IpgBezier, IpgCircle, IpgEllipse, IpgFreeHand, 
    IpgLine, IpgPolyLine, IpgPolygon, IpgRightTriangle, IpgText, IpgCanvasWidget};
use crate::{canvas::draw_canvas::IpgDrawMode, 
canvas::canvas_helpers::{build_polygon, get_angle_of_vectors, get_blink_position, 
    get_horizontal_angle_of_vector, get_mid_point, rotate_geometry, to_degrees, 
    translate_geometry}};

use super::geometries::get_mid_geometry;

pub fn build_arc_path(arc: &IpgArc, 
                    draw_mode: IpgDrawMode, 
                    pending_cursor: Option<Point>,
                    edit_point_index: Option<usize>, 
                    edit_mid_point: bool,
                    ) -> (Path, Point, Radians, Radians, Option<f32>, Option<f32>) {

    let mut pts = arc.points.clone();
    let mut mid_point = arc.mid_point;
    let mut start_angle = arc.start_angle;
    let mut end_angle = arc.end_angle;
    let mut radius = arc.radius;
    let mut degrees_left = None;
    let mut degrees_center = None;

    let path = Path::new(|p| {
        match draw_mode {
            IpgDrawMode::DrawAll => {
                let new_arc = 
                    canvas::path::Arc{ 
                        center: arc.mid_point, 
                        radius: arc.radius, 
                        start_angle: arc.start_angle, 
                        end_angle: arc.end_angle, 
                    };
                p.arc(new_arc);
            },
            IpgDrawMode::Edit => {
                pts = arc.points.clone();
                let cursor = pending_cursor.unwrap();

                if edit_mid_point {
                    pts = translate_geometry(
                        &pts, 
                        cursor,
                        mid_point, 
                        );
                    mid_point = cursor;
                } 
                if edit_point_index.is_some() && edit_point_index != Some(0) {
                    pts[edit_point_index.unwrap()] = cursor;
                    if edit_point_index == Some(1) {
                        radius = mid_point.distance(cursor);

                        start_angle = get_angle_of_vectors(
                            mid_point, 
                            Point::new(-mid_point.x, mid_point.y), 
                            cursor) + Radians::PI;
                    
                        end_angle = 
                            get_angle_of_vectors(
                                mid_point, 
                                cursor, 
                                pts[2]) + start_angle;
                        degrees_left = 
                            Some(get_horizontal_angle_of_vector(
                                mid_point, 
                                cursor, 
                            ));
                    } else if edit_point_index == Some(2) {
                        end_angle = 
                            get_angle_of_vectors(
                                mid_point, 
                                pts[1], 
                                cursor) + start_angle;
                        degrees_center = Some(to_degrees(&(end_angle.0-start_angle.0)));
                    }

                    // calc the end_angle point        
                    let r = radius;
                    let b = end_angle.0;
                    let point_b = Point::new(r*b.cos(), r*b.sin());
                    pts[2] = translate_geometry(&vec![point_b], mid_point, Point::default())[0];
                    
                }
                p.move_to(pts[0]);
                p.line_to(pts[1]);
                
                p.circle(mid_point, 3.0);
                p.circle(pts[1], 3.0);
                p.circle(pts[2], 3.0);
                
                let edit_arc = canvas::path::Arc{ 
                                            center: mid_point, 
                                            radius, 
                                            start_angle, 
                                            end_angle, 
                                        };
                p.arc(edit_arc);
            },
            IpgDrawMode::New => {
                let cursor = pending_cursor.unwrap();
                let pts_len = arc.points.len();
                
                if pts_len == 1 {
                    p.move_to(arc.points[0]);
                    p.line_to(cursor);
                    start_angle = get_angle_of_vectors(
                            arc.points[0], 
                            Point::new(-arc.points[0].x, arc.points[0].y), 
                            cursor);
                    degrees_left = 
                        Some(get_horizontal_angle_of_vector(
                        mid_point, 
                        cursor, 
                    ));
                }
                if pts_len == 2 {
                    p.move_to(arc.points[0]);
                    p.line_to(arc.points[1]);
                    
                    end_angle = 
                        get_angle_of_vectors(
                            arc.points[0], 
                            arc.points[1], 
                            cursor) + arc.start_angle;

                    let radius = arc.points[0].distance(arc.points[1]);
                    let new_arc = canvas::path::Arc{ 
                                            center: arc.points[0], 
                                            radius, 
                                            start_angle: arc.start_angle, 
                                            end_angle, 
                                        };
                    p.arc(new_arc);

                    degrees_left = 
                        Some(get_horizontal_angle_of_vector(
                        mid_point, 
                        pts[1], 
                    ));
                    degrees_center = Some(to_degrees(&(end_angle.0-start_angle.0)));
                };
            },
            IpgDrawMode::Rotate => {
                let rotated_arc = 
                    canvas::path::Arc{ 
                        center: arc.points[0], 
                        radius, 
                        start_angle: arc.start_angle, 
                        end_angle, 
                    };

                degrees_left = 
                    Some(get_horizontal_angle_of_vector(
                        mid_point, 
                        pts[1], 
                    ));

                p.arc(rotated_arc);
                p.move_to(arc.points[0]);
                p.line_to(arc.points[1]);
                p.move_to(arc.points[0]);
                p.line_to(arc.points[2]);
                p.circle(arc.mid_point, 3.0);
            },
        }
    });
    
    (path, mid_point, start_angle, end_angle, degrees_left, degrees_center)

}

pub fn build_bezier_path(bz: &IpgBezier, 
                    draw_mode: IpgDrawMode, 
                    pending_cursor: Option<Point>,
                    edit_point_index: Option<usize>, 
                    edit_mid_point: bool,
                    degrees: Option<f32>,
                    ) -> (Path, f32, Point) {

    let mut degrees = match degrees {
        Some(d) => d,
        None => bz.degrees,
    };
    let mut mid_point = bz.mid_point;

    let path = Path::new(|p| {
        match draw_mode {
            IpgDrawMode::DrawAll => {
                p.move_to(bz.points[0]);
                p.quadratic_curve_to(bz.points[2], bz.points[1]);
            },
            IpgDrawMode::Edit => {
                let mut pts = bz.points.clone();

                if edit_mid_point {
                    pts = translate_geometry(
                        &pts, 
                        pending_cursor.unwrap(),
                        mid_point, 
                        );
                    mid_point = pending_cursor.unwrap();
                } 
                if edit_point_index.is_some() {
                    pts[edit_point_index.unwrap()] = pending_cursor.unwrap();
                    mid_point = get_mid_point(pts[0], pts[1]);
                    
                    degrees = 
                        get_horizontal_angle_of_vector(
                            mid_point, 
                            pts[1], 
                        );
                }

                p.move_to(pts[0]);
                p.quadratic_curve_to(pts[2], pts[1]);
                
                for pt in pts {
                    p.circle(pt, 3.0);
                }

                p.circle(mid_point, 3.0);
            },
            IpgDrawMode::New => {
                if bz.points.len() == 1 {
                    mid_point = 
                        get_mid_point(
                            bz.points[0], 
                            pending_cursor.unwrap()
                        );
                    degrees = 
                        get_horizontal_angle_of_vector(
                            bz.points[0],  
                            pending_cursor.unwrap(),
                        );
                    p.move_to(bz.points[0]);
                    p.line_to(pending_cursor.unwrap());
                }
                if bz.points.len() == 2 {
                    p.move_to(bz.points[0]);
                    p.quadratic_curve_to(pending_cursor.unwrap(), bz.points[1]);
                }
            },
            IpgDrawMode::Rotate => {
                p.move_to(bz.points[0]);
                p.quadratic_curve_to(bz.points[2], bz.points[1]);
                p.move_to(bz.points[0]);
                p.line_to(bz.points[1]);
                p.circle(bz.mid_point, 3.0);
            },
        }
    });

    (path, degrees, mid_point)

}

pub fn build_circle_path(cir: &IpgCircle, 
                    draw_mode: IpgDrawMode, 
                    pending_cursor: Option<Point>,
                    edit_point_index: Option<usize>, 
                    edit_mid_point: bool,
                ) -> Path {
    Path::new(|p| {
        match draw_mode {
            IpgDrawMode::DrawAll => {
                p.circle(cir.center, cir.radius);
            },
            IpgDrawMode::Edit => {
                let mut center = cir.center;
                let mut cir_point = cir.circle_point;
                let mut radius = cir.radius;

                if edit_mid_point {
                    cir_point = translate_geometry(
                        &vec![cir_point], 
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
            IpgDrawMode::Rotate => {
                p.circle(cir.center, cir.radius);
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
            IpgDrawMode::DrawAll => {
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
                        let vy = Point::new(ell.points[0].x, cursor.y).distance(ell.points[0]);
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
                if ell.points.len() > 0 {
                    p.move_to(ell.points[0]);

                }
                if ell.points.len() == 0 {
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
                    let vy = p2.distance(ell.points[0]);
                    p.ellipse(Elliptical{ 
                        center: ell.points[0], 
                        radii: Vector{x: vx, y: vy}, 
                        rotation: Radians(0.0), 
                        start_angle: Radians(0.0), 
                        end_angle: Radians(2.0*PI) 
                    });
                }
                
            },
            IpgDrawMode::Rotate => {
                let vx = ell.points[1].distance(ell.center);
                let vy = ell.points[2].distance(ell.center);
                p.ellipse(Elliptical{ 
                        center: ell.center, 
                        radii: Vector{x: vx, y: vy}, 
                        rotation: ell.rotation, 
                        start_angle: Radians(0.0), 
                        end_angle: Radians(2.0*PI) 
                    });
                p.circle(ell.center, 3.0);
            },
        }
    })
}

pub fn build_line_path(line: &IpgLine, 
                    draw_mode: IpgDrawMode, 
                    pending_cursor: Option<Point>,
                    edit_point_index: Option<usize>, 
                    edit_mid_point: bool,
                    degrees: Option<f32>,
                    ) -> (Path, f32, Point) {

    let mut degrees = match degrees {
        Some(d) => d,
        None => line.degrees,
    };
    let mut mid_point = line.mid_point;

    let path = Path::new(|p| {
        match draw_mode {
            IpgDrawMode::DrawAll => {
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
            IpgDrawMode::Rotate => {
                p.move_to(line.points[0]);
                p.line_to(line.points[1]);
                p.circle(mid_point, 3.0);
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
        None => pg.degrees,
    };
    let mut mid_point = pg.mid_point;
    let mut pg_point = pg.pg_point;

    let path = Path::new(|p| {
        match draw_mode {
            IpgDrawMode::DrawAll => {
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
                        &vec![pg.pg_point], 
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
            IpgDrawMode::Rotate => {
                for (index, point) in pg.points.iter().enumerate() {
                    if index == 0 {
                        p.move_to(*point);
                    } else {
                        p.line_to(*point);
                    }
                }
                p.move_to(pg.mid_point);
                p.line_to(pg.pg_point);
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
        None => pl.degrees,
    };
    let mut pts = pl.points.clone();
    let mut mid_point = pl.mid_point;
    let mut pl_point = pl.pl_point;

    let path = Path::new(|p| {
        match draw_mode {
            IpgDrawMode::DrawAll => {
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
                    mid_point = get_mid_geometry(&pts, IpgCanvasWidget::PolyLine);
                    pl_point = translate_geometry(
                                    &vec![pl_point], 
                                    mid_point, 
                                    pl.mid_point,
                                )[0];
                }
                if edit_other_point {
                    degrees = get_horizontal_angle_of_vector(pl.mid_point, pending_cursor.unwrap());
                    let step_degrees = degrees-pl.degrees;
                    pts = rotate_geometry(&pts, &mid_point, &step_degrees, IpgCanvasWidget::PolyLine);
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
            IpgDrawMode::Rotate => {
                for (index, point) in pl.points.iter().enumerate() {
                    if index == 0 {
                        p.move_to(*point);
                    } else {
                        p.line_to(*point);
                    }
                }
                // let (slope, intercept) = get_linear_regression(&pl.points);
                // let(p1, p2) = get_line_from_slope_intercept(&pl.points, slope, intercept);
                // mid_point = get_mid_point(p1, p2);
                // degrees = get_vertical_angle_of_vector(mid_point, p2);
                p.move_to(pl.mid_point);
                p.line_to(pl.pl_point);
                p.circle(mid_point, 3.0);
            },
        }
    });

    (path, degrees, mid_point)

}

pub fn build_right_triangle_path(tr: &IpgRightTriangle, 
                            draw_mode: IpgDrawMode, 
                            pending_cursor: Option<Point>,
                            edit_point_index: Option<usize>, 
                            edit_mid_point: bool,
                            edit_other_point: bool,
                            degrees: Option<f32>,
                        ) -> (Path, f32, Point, Point) {

    let mut pts = tr.points.clone();
    let mut mid_point = tr.mid_point;
    let mut tr_point = tr.tr_point;
    let mut degrees = match degrees {
        Some(d) => d,
        None => tr.degrees,
    };

    let path = Path::new(|p| {
        match draw_mode {
            IpgDrawMode::DrawAll => {
                p.move_to(tr.points[0]);
                p.line_to(tr.points[1]);
                p.line_to(tr.points[2]);
                p.line_to(tr.points[0]);
            },
            IpgDrawMode::Edit => {
                pts.push(tr_point);
                if edit_mid_point {
                    pts = translate_geometry(
                        &pts, 
                        pending_cursor.unwrap(),
                        mid_point, 
                    );
                    tr_point = pts.pop().unwrap();
                    mid_point = pending_cursor.unwrap();
                } 
                if edit_point_index.is_some() {
                    let index = edit_point_index.unwrap();
                    let cursor = pending_cursor.unwrap();
                    if index == 0 {
                        pts[0].y = cursor.y
                    }
                    if index == 1 {
                        pts[1].y = cursor.y;
                        pts[2].y = cursor.y;
                    }
                    if index == 2 {
                        pts[2].x = cursor.x;
                }
                    let mid = get_mid_point(pts[1], pts[2]);
                    let dist_b_mid = Point::new(mid.x-pts[2].x, mid.y-pts[2].y);
                    tr_point = Point::new(
                                    tr.points[2].x+dist_b_mid.x, 
                                    tr.points[2].y+dist_b_mid.y);
                }
                if edit_other_point {
                    degrees = 
                        get_horizontal_angle_of_vector(
                            tr.mid_point, 
                            pending_cursor.unwrap()
                        );
                    let step_degrees = degrees-tr.degrees;
                    pts = rotate_geometry(
                            &pts, 
                            &mid_point, 
                            &step_degrees, 
                            IpgCanvasWidget::RightTriangle
                        );
                    tr_point = pending_cursor.unwrap();
                }

                p.move_to(pts[0]);
                p.line_to(pts[1]);
                p.line_to(pts[2]);
                p.line_to(pts[0]);
                p.move_to(mid_point);
                p.line_to(tr_point);

                p.circle(pts[0], 3.0);
                p.circle(pts[1], 3.0);
                p.circle(pts[2], 3.0);
                p.circle(mid_point, 3.0);
                p.circle(tr_point, 3.0);
            },
            IpgDrawMode::New => {
                let mut cursor = pending_cursor.unwrap();
                p.move_to(tr.points[0]);
                if tr.points.len() == 1 {
                    cursor.x = tr.points[0].x;
                    p.line_to(cursor);
                } else if tr.points.len() == 2 {
                    cursor.y = tr.points[1].y;
                    p.line_to(tr.points[1]);
                    p.line_to(cursor);
                }
            },
            IpgDrawMode::Rotate => {
                p.move_to(tr.points[0]);
                p.line_to(tr.points[1]);
                p.line_to(tr.points[2]);
                p.line_to(tr.points[0]);

                p.move_to(tr.mid_point);
                p.line_to(tr.tr_point);

                p.circle(tr.mid_point, 3.0);
                p.circle(tr.tr_point, 3.0);
            },
        }
    });

    (path, degrees, mid_point, tr_point)

}


pub fn build_free_hand_path(fh: &IpgFreeHand, 
                        draw_mode: IpgDrawMode, 
                        pending_cursor: Option<Point>,
                        edit_point_index: Option<usize>,
                        ) -> Path {

    let mut pts = fh.points.clone();

    let path = Path::new(|p| {
        match draw_mode {
            IpgDrawMode::DrawAll => {
                for (index, point) in fh.points.iter().enumerate() {
                    if index == 0 {
                        p.move_to(*point);
                    } else {
                        p.line_to(*point);
                    }
                }
            },
            IpgDrawMode::Edit => {
                if edit_point_index.is_some() {
                    pts[edit_point_index.unwrap()] = pending_cursor.unwrap();
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
            },
            IpgDrawMode::New => {
                for (index, point) in fh.points.iter().enumerate() {
                    if index == 0 {
                        p.move_to(*point);
                    } else {
                        p.line_to(*point);
                    }
                }
                p.line_to(pending_cursor.unwrap());
            },
            IpgDrawMode::Rotate => {
                p.move_to(Point::new(0.0,0.0));
            },
        }
    });

    path
    
}

pub fn build_text_path (txt: &IpgText, 
                    draw_mode: IpgDrawMode, 
                    _pending_cursor: Option<Point>,
                    _degrees: f32,
                    blink: bool,
                    ) -> (canvas::Text, Option<Path>) {

        match draw_mode {
            IpgDrawMode::DrawAll => {
                let text = canvas::Text {
                    content: txt.content.clone(),
                    position: txt.position,
                    color: txt.color,
                    size: txt.size,
                    line_height: txt.line_height,
                    font: txt.font,
                    horizontal_alignment: txt.horizontal_alignment,
                    vertical_alignment: txt.vertical_alignment,
                    shaping: txt.shaping,
                };
                (text, None)
            },
            IpgDrawMode::Edit => {
                let text = canvas::Text {
                    content: txt.content.clone(),
                    position: txt.position,
                    color: txt.color,
                    size: txt.size,
                    line_height: txt.line_height,
                    font: txt.font,
                    horizontal_alignment: txt.horizontal_alignment,
                    vertical_alignment: txt.vertical_alignment,
                    shaping: txt.shaping,
                };
                let path = Some(Path::new(|p| {
                    p.circle(txt.position, 3.0);
                }));
                (text, path)
            },
            IpgDrawMode::New => {
                let text = canvas::Text {
                    content: txt.content.clone(),
                    position: txt.position,
                    color: txt.color,
                    size: txt.size,
                    line_height: txt.line_height,
                    font: txt.font,
                    horizontal_alignment: txt.horizontal_alignment,
                    vertical_alignment: txt.vertical_alignment,
                    shaping: txt.shaping,
                };
                let path: Option<Path> = if blink {
                    Some(Path::new(|p| {
                        let (from, to) = 
                            get_blink_position( 
                                &txt.content,
                                txt.position, 
                                txt.blink_position,
                            );
                        p.move_to(from);
                        p.line_to(to);
                    }))
                } else {
                    None
                };
                (text, path)
            },
            IpgDrawMode::Rotate => {
                let text = canvas::Text {
                    content: txt.content.clone(),
                    position: txt.position,
                    color: txt.color,
                    size: txt.size,
                    line_height: txt.line_height,
                    font: txt.font,
                    horizontal_alignment: txt.horizontal_alignment,
                    vertical_alignment: txt.vertical_alignment,
                    shaping: txt.shaping,
                };
                (text, None)
            },
        }

                  
}
