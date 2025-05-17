// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.


use iced::Point;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct QuadraticBezier {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}
impl QuadraticBezier {
    fn new(&self) -> ((f32, f32), f32, f32, (f32, f32), (f32, f32)){
        let m = (self.x1, self.y1);
        let x = (self.x1 + self.x1) / 2.0;
        let y = self.y1 + (self.y2 - self.y1) / 2.0;
        let q = (x, y);
        let end = (self.x2, self.y2);
    (m, x, y, q, end)
    }
}

#[derive(Clone, PartialEq, Debug, Default)]
struct ControlPoint {
    left: Option<Point>,
    right: Option<Point>,
}

// http://scaledinnovation.com/analytics/splines/aboutSplines.html
fn get_control_points(
    p: &Point,
    left: Option<&Point>,
    right: Option<&Point>,
    t: f32,
) -> ControlPoint {
    let x0 = left.unwrap_or(p).x;
    let y0 = left.unwrap_or(p).y;
    let x1 = p.x;
    let y1 = p.y;
    let x2 = right.unwrap_or(p).x;
    let y2 = right.unwrap_or(p).y;

    let d01 = ((x1 - x0).powf(2.0) + (y1 - y0).powf(2.0)).sqrt();
    let d12 = ((x2 - x1).powf(2.0) + (y2 - y1).powf(2.0)).sqrt();
    // scaling factor for triangle Ta
    let fa = t * d01 / (d01 + d12);
    // ditto for Tb, simplifies to fb=t-fa
    let fb = t * d12 / (d01 + d12);
    // x2-x0 is the width of triangle T
    let p1x = x1 - fa * (x2 - x0);
    // y2-y0 is the height of T
    let p1y = y1 - fa * (y2 - y0);
    let p2x = x1 + fb * (x2 - x0);
    let p2y = y1 + fb * (y2 - y0);

    let mut cpl = None;
    let mut cpr = None;
    if left.is_some() {
        cpl = Some((p1x, p1y).into());
    }
    if right.is_some() {
        cpr = Some((p2x, p2y).into());
    }
    ControlPoint {
        left: cpl,
        right: cpr,
    }
}


pub fn smooth_curve(
    points: Vec<Point>, 
    close: bool) 
    -> Vec<Point> {

    let tension = 0.25;

    let close = close;
    let count = points.len();
    let mut control_points = vec![];
    for (index, point) in points.iter().enumerate() {
        let mut left = None;
        let mut right = None;
        if index >= 1 {
            left = Some(&points[index - 1]);
        } else if close {
            // For the previous node of the first point, last
            left = points.last();
        }
        if index + 1 < count {
            right = Some(&points[index + 1]);
        } else if close {
            // The next node of the last point is first
            right = points.first()
        }
        control_points.push(get_control_points(point, left, right, tension));
    }

    let mut arr = vec![];
    for (index, point) in points.iter().enumerate() {
        if index == 0 {
            arr.push(point.clone());
        }
        let cp1 = control_points[index].right;
        let mut cp2 = None;
        if let Some(value) = control_points.get(index + 1) {
            cp2 = value.left;
        } else if close {
            // The most important point
            cp2 = control_points[0].left;
        }
        let mut next_point = points.get(index + 1);
        // If it is close, you need to process the last point
        // If not the last point
        if close && index == count - 1 {
            next_point = points.first();
        }
        if let Some(next_point_value) = next_point {
            let next_point = next_point_value;
            if let Some(cp1_value) = cp1 {
                if let Some(cp2_value) = cp2 {
                    let c1 = cp1_value.x;
                    let c2 = cp2_value;
                    arr.extend([c1, c2, next_point.clone()]);
                    continue;
                }
            }
            let q = cp1.unwrap_or(cp2.unwrap_or_default());

            arr.extend([q, next_point.clone()]);
        }
    }
    arr

}
