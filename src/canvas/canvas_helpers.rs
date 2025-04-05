//! helpers

use std::f32::consts::PI;

use iced::{Point, Radians};

use super::geometries::IpgCanvasWidget;


pub fn build_polygon(mid_point: Point, pg_point: Point, poly_points: usize, mut degrees: f32) -> Vec<Point> {
    
    let angle = 2.0 * PI / poly_points as f32;
    let radius = mid_point.distance(pg_point);
    let mut points = vec![];
    for i in 0..poly_points {
        let x = mid_point.x + radius * (i as f32 * angle).sin();
        let y = mid_point.y + radius * (i as f32 * angle).cos();
        points.push(Point::new(x, y));
    }
    
    degrees += 180.0;
    let mut pts = rotate_geometry(&points, &mid_point, &degrees, IpgCanvasWidget::Polygon);
    pts.push(pts[0]);
    pts

}

pub fn get_mid_point(pt1: Point, pt2: Point) -> Point {
    Point {x: (pt1.x + pt2.x) / 2.0, y: (pt1.y + pt2.y) / 2.0 }
}

pub fn get_linear_regression(points: &[Point]) -> (f32, f32) {
    let mut sx: f64 = 0.0;
    let mut sy: f64 = 0.0;
    let mut sxx: f64 = 0.0;
    let mut sxy: f64 = 0.0;

    for point in points.iter() {
        sx += point.x as f64;
        sy += point.y as f64;
        sxx += point.x as f64 * point.x as f64;
        sxy += point.x as f64 * point.y as f64;
    }

    let n = points.len() as f64;
    let beta = (n*sxy-sx*sy) / (n*sxx - sx*sx);
    let alpha = (1.0/n * sy) - (beta*1.0/n*sx);

    (beta as f32, alpha as f32)

}

pub fn get_line_from_slope_intercept(points: &[Point], 
                                slope: f32, 
                                intercept: f32,
                                ) -> (Point, Point) {

    let mut small_x = 1_000_000_f32;
    let mut large_x = 0.0;
    let mut small_y = 1_000_000_f32;
    let mut large_y = 0.0;

    for point in points.iter() {
        if point.x < small_x {
            small_x = point.x;
        }
        if point.x > large_x {
            large_x = point.x;
        }
        if point.y < small_y {
            small_y = point.y;
        }
        if point.y > large_y {
            large_y = point.y;
        }
    }
 
    let ys = slope*small_x + intercept;
    let yl = slope*large_x + intercept; 
    
    (Point{x: small_x, y: ys}, Point{x: large_x, y: yl})  

}

pub fn translate_geometry(pts: &[Point], 
                        new_center: Point,
                        old_center: Point, 
                        ) 
                        -> Vec<Point> {
    let mut new_pts = vec![];
    let dist_x = new_center.x - old_center.x;
    let dist_y = new_center.y - old_center.y;
    for pt in pts.iter() {
        new_pts.push(Point{x: pt.x + dist_x, y: pt.y + dist_y})
    }

    new_pts
}

// The degrees are adjusted based on how degrees where calulated for each widget.
pub fn rotate_geometry(
                    points: &[Point], 
                    mid_point: &Point, 
                    step_degrees: &f32, 
                    widget: IpgCanvasWidget,
                    ) -> Vec<Point> {
    match widget {
        IpgCanvasWidget::None => vec![],
        _ => {
            let theta = to_radians(step_degrees);
            let mut new_points = vec![];
            for point in points.iter() {
                let x_new = (point.x - mid_point.x) * theta.cos() - 
                                (point.y - mid_point.y) * theta.sin() + 
                                mid_point.x;
                let y_new = (point.x - mid_point.x) * theta.sin() + 
                                (point.y - mid_point.y) * theta.cos() + 
                                mid_point.y;

                new_points.push(Point { x: x_new, y: y_new })
            }
            
            new_points
        }
    }
}

// The first point is used to create a horizontal vector
pub fn get_horizontal_angle_of_vector(center: Point, p2: Point) -> f32 {
    let p1 = Point::new(center.x-10.0, center.y);
    let pts = 
        translate_geometry(
            &[p1, p2], 
            Point::default(), 
            center,
        );

    let angle = ((pts[0].y).atan2(pts[0].x) -
                        (pts[1].y).atan2(pts[1].x)) * -1.0;

    // Since beyond pi, angle goes negative
    let new_angle = if angle < 0.0 {
        2.0 * PI + angle
    } else {
        angle
    };

    to_degrees(&new_angle)
}

pub fn get_angle_of_vectors(center: Point, p1: Point, p2: Point) -> Radians {

    let pts = 
        translate_geometry(
            &[p1, p2], 
            Point::default(), 
            center,
        );

    let angle = ((pts[0].y).atan2(pts[0].x) -
                        (pts[1].y).atan2(pts[1].x)) * -1.0;
    
    // Since beyond pi, angle goes negative
    let new_angle = if angle < 0.0 {
        2.0 * PI + angle
    } else {
        angle
    };

    Radians::from(new_angle)
}

pub fn to_degrees(radians_f32: &f32) -> f32 {
    radians_f32 * 180.0/PI
}

pub fn to_radians(degrees: &f32) -> f32 {
    degrees * PI/180.0
}


#[test]
fn test_get_linear_regression() {
    let points: Vec<Point>= 
    vec![
    Point::new(1.47, 52.21),
    Point::new(1.50, 53.12),
    Point::new(1.52, 54.48),
    Point::new(1.55, 55.84),
    Point::new(1.57, 57.20),
    Point::new(1.60, 58.57),
    Point::new(1.63, 59.93),
    Point::new(1.65, 61.29),
    Point::new(1.68, 63.11),
    Point::new(1.70, 64.47),
    Point::new(1.73, 66.28),
    Point::new(1.75, 68.10),
    Point::new(1.78, 69.92),
    Point::new(1.80, 72.19),
    Point::new(1.83, 74.46),
    ];

    let (slope, intercept) = get_linear_regression(&points);

    assert_eq!(61.27219, slope);
    assert_eq!(-39.06196, intercept);

}

#[test]
fn test_get_line_from_slope_intercept() {
    let points = vec![Point::new(0.0, 100.0), Point::new(30.0, 30.0), Point::new(25.0, 50.0)];
    let (slope, intercept) = get_linear_regression(&points);
    let line_points = get_line_from_slope_intercept(&points, slope, intercept);
    println!("{:?} {:?}, {:?}",slope, intercept, line_points );
}

#[test]
fn test_get_angle() {
    //  all 4 quadrants
    let center = Point::new(0.0, 0.0);
    let p2 = Point::new(0.0, 10.0);
    let degrees = get_horizontal_angle_of_vector(center, p2);
    dbg!(degrees);

    let p2 = Point::new(20.0, 10.0);
    let degrees = get_horizontal_angle_of_vector(center, p2);
    dbg!(degrees);

    let p2 = Point::new(0.0, -10.0);
    let degrees = get_horizontal_angle_of_vector(center, p2);
    dbg!(degrees);

    let p2 = Point::new(-20.0, 0.0);
    let degrees = get_horizontal_angle_of_vector(center, p2);
    dbg!(degrees);
}

#[test]
fn test_rotate_geometry() {
    let mut points= vec![Point::new(0.0, 0.0), Point::new(0.0, 20.0)];
    let mid_point = Point::new(0.0, 0.0);
    let degrees = &6.0;
    for _ in 0..2 {
        points = rotate_geometry(&points.clone(), &mid_point, degrees, IpgCanvasWidget::Line);
        dbg!(&points);
    }
}


