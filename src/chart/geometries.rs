//! geometries

use iced::{alignment, widget::{image, 
    text::{LineHeight, Shaping}}, Color, 
    Pixels, Point, Radians, Rectangle, Size, Vector};
use pyo3::pyclass;
use serde::{Deserialize, Serialize};

use super::draw_chart::{ChartDrawMode, ChartDrawStatus};

#[derive(Debug, Clone, PartialEq)]
pub struct ChartCircle {
    pub id: usize,
    pub center: Point,
    pub radius: f32,
    pub stroke: Color,
    pub stroke_width: f32,
    pub fill_color: Option<Color>,
    pub stroke_dash_offset: Option<usize>,
    pub stroke_dash_segments: Option<Vec<f32>>,
    pub draw_mode: ChartDrawMode,
    pub status: ChartDrawStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChartEllipse {
    pub id: usize,
    pub points: Vec<Point>,
    pub center: Point,
    pub radii: Vector,
    pub rotation: Radians,
    pub stroke: Color,
    pub fill_color: Option<Color>,
    pub stroke_width: f32,
    pub stroke_dash_offset: Option<usize>,
    pub stroke_dash_segments: Option<Vec<f32>>,
    pub draw_mode: ChartDrawMode,
    pub status: ChartDrawStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChartImage {
    pub id: usize,
    pub path: image::Handle,
    pub bounds: Rectangle,
    pub width: f32,
    pub height: f32,
    pub draw_mode: ChartDrawMode,
    pub status: ChartDrawStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChartLine {
    pub id: usize,
    pub points: Vec<Point>,
    pub stroke: Color,
    pub stroke_width: f32,
    pub stroke_dash_offset: Option<usize>,
    pub stroke_dash_segments: Option<Vec<f32>>,
    pub draw_mode: ChartDrawMode,
    pub status: ChartDrawStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChartPolyLine {
    pub id: usize,
    pub points: Vec<Point>,
    pub stroke: Color,
    pub stroke_width: f32,
    pub stroke_dash_offset: Option<usize>,
    pub stroke_dash_segments: Option<Vec<f32>>,
    pub draw_mode: ChartDrawMode,
    pub status: ChartDrawStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChartPolygon {
    pub id: usize,
    pub points: Vec<Point>,
    pub stroke: Color,
    pub stroke_width: f32,
    pub fill_color: Option<Color>,
    pub stroke_dash_offset: Option<usize>,
    pub stroke_dash_segments: Option<Vec<f32>>,
    pub draw_mode: ChartDrawMode,
    pub status: ChartDrawStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChartRectangle {
    pub id: usize,
    pub top_left: Point,
    pub size: Size,
    pub stroke: Color,
    pub stroke_width: f32,
    pub fill_color: Option<Color>,
    pub stroke_dash_offset: Option<usize>,
    pub stroke_dash_segments: Option<Vec<f32>>,
    pub draw_mode: ChartDrawMode,
    pub status: ChartDrawStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChartText {
    pub id: usize,
    pub content: String,
    pub position: Point,
    pub color: Color,
    pub size: Pixels,
    pub line_height: LineHeight,
    pub font: String,
    pub horizontal_alignment: alignment::Horizontal,
    pub vertical_alignment: alignment::Vertical,
    pub shaping: Shaping,
    pub rotation: f32,
    pub draw_mode: ChartDrawMode,
    pub status: ChartDrawStatus,
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


// pub fn add_keypress(widget: &mut ChartWidget, modified: Key) -> (Option<ChartWidget>, bool) {
//     let mut escape = false;
//     match widget {
//         ChartWidget::Text(txt) => {
//             match modified.as_ref() {
//                 Key::Named(named) => {
//                     match named {
//                         iced::keyboard::key::Named::Enter => {
//                             txt.content.push('\r');
//                         },
//                         iced::keyboard::key::Named::Tab => {
//                             txt.content.push_str("    ");
//                         },
//                         iced::keyboard::key::Named::Space => {
//                             txt.content.push(' ');
//                         },
//                         iced::keyboard::key::Named::Escape => escape = true,
//                         iced::keyboard::key::Named::Backspace => {
//                             if !txt.content.is_empty() {
//                                 txt.content.pop();
//                             }
//                         } 
//                         _ => ()
//                     }
//                 },
//                 Key::Character(c) => {
//                     txt.content.push_str(c);
//                 },
//                 Key::Unidentified => (),
//             }
//             if escape {
//                 (None, false)
//             } else {
//                 (Some(ChartWidget::Text(txt.clone())), false)
//             }
//         },
//         _ => (None, false)
//     }
// }

// pub fn get_del_key(modified: Key) -> bool {
//     match modified.as_ref() {
//         Key::Named(named) => {
//             matches!(named, iced::keyboard::key::Named::Delete)
//         },
//         _ => false,
//     }
// }

// pub fn find_closest_widget(curves: &HashMap<usize, ChartWidget>, 
//                             text_curves: &HashMap<usize, ChartWidget>,
//                             cursor: Point,
//                             ) 
//                             -> Option<ChartWidget> {
//     let mut closest = f32::INFINITY;
//     let mut closest_id = None;
//     for (id, wid) in curves.iter() {
//         if match_ipg_widget(wid) == IpgChartWidget::Circle {
//             // do nothing
//         } else {
//             let distance: f32 = get_distance_to_mid_point(wid, cursor);
//             if distance < closest {
//                 closest = distance;
//                 closest_id = Some(id);
//             }
//         }
//     }

//     let mut text_id = false;
//     for(id, text) in text_curves.iter() {
//         let distance: f32 = get_distance_to_mid_point(text, cursor);
//         if distance < closest {
//             closest = distance;
//             closest_id = Some(id);
//             text_id = true;
//         }
//     }

//     let dc_opt = 
//         if text_id {
//             match closest_id {
//                 Some(id) => text_curves.get(id).cloned(),
//                 None => None,
//             }
//         } else {
//             match closest_id {
//                 Some(id) => curves.get(id).cloned(),
//                 None => None,
//             }
//         };
        
//     dc_opt
    
// }

// // returns a bool if mid_point and an optional usize 
// // if a point in points.
// pub fn find_closest_point_index(widget: &ChartWidget,
//                             cursor: Point, 
//                             ) -> (Option<usize>, bool, bool) {

//     let mut point_dist: f32 = f32::INFINITY;
//     let mut point_index = 0;

//     match widget {
//         ChartWidget::Circle(cir) => {
//             let center_dist = cursor.distance(cir.center);
//             let point_dist = cursor.distance(cir.circle_point);
//             if center_dist < point_dist {
//                 (None, true, false)
//             } else {
//                 (Some(1), false, false)
//             }
//         }
//         ChartWidget::Ellipse(ell) => {
//             let center_dist = cursor.distance(ell.center);
//             let point_1_dist = cursor.distance(ell.points[1]);
//             let point_2_dist = cursor.distance(ell.points[2]);
//             if center_dist < point_1_dist && center_dist < point_2_dist {
//                 (None, true, false)
//             } else if point_1_dist < point_2_dist {
//                 (Some(1), false, false)
//             } else {
//                 (Some(2), false, false)
//             }
//         }
//         ChartWidget::Line(line) => {
//             for (idx, point) in line.points.iter().enumerate() {
//                 let dist = cursor.distance(*point);
//                 if  dist < point_dist {
//                     point_index = idx;
//                     point_dist = dist;
//                 }
//             };
            
//             let mid_dist = cursor.distance(line.mid_point);

//             if mid_dist < point_dist {
//                 (None, true, false)
//             } else {
//                 (Some(point_index), false, false)
//             }
//         },
//         ChartWidget::Polygon(pg) => {
//             let pg_center = cursor.distance(pg.mid_point);
//             let pg_point = cursor.distance(pg.pg_point);
//             if pg_center <= pg_point {
//                 (None, true, false)
//             } else {
//                 (None, false, true)
//             }
//         },
//         ChartWidget::PolyLine(pl) => {
//             for (idx, point) in pl.points.iter().enumerate() {
//                 let dist = cursor.distance(*point);
//                 if  dist < point_dist {
//                     point_index = idx;
//                     point_dist = dist;
//                 }
//             };
            
//             let mid_dist = pl.mid_point.distance(cursor);
//             let pl_pt_dist = pl.pl_point.distance(cursor);

//             if point_dist < mid_dist && point_dist < pl_pt_dist {
//                 (Some(point_index), false, false)
//             } else if mid_dist < pl_pt_dist {
//                 (None, true, false)
//             } else {
//                 (None, false, true)
//             }
//         },
//         ChartWidget::Text(_) => {
//             // just using the edit_other_point to indicate the position point
//             (None, false, true)
//         },
//         _ => (None, false, false),
//     }
    
// }
