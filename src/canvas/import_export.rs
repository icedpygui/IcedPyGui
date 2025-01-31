//! import_export

use std::{collections::HashMap, fs::File, io::{BufWriter, Write}, path::Path};

use iced::{alignment, widget::{container::Id, text::{LineHeight, Shaping}}, Color, Font, Pixels, Point, Radians, Vector};
use serde::{Deserialize, Serialize};

use super::{draw_canvas::{IpgWidget, IpgDrawMode, IpgDrawStatus}, geometries::{IpgArc, IpgBezier, IpgCircle, IpgEllipse, IpgFreeHand, IpgLine, IpgPolyLine, IpgPolygon, IpgRightTriangle, IpgText, IpgCanvasWidget}};


pub fn save(path: impl AsRef<Path>, data: &impl Serialize) -> std::io::Result<()> {
    let mut w = BufWriter::new(File::create(path).expect("unable to create file"));
    serde_json::to_writer_pretty(&mut w, data).expect("unable to format data");
    w.write_all(b"\n").expect("unable to append to buffer");
    w.flush().expect("unable to flush buffer");
    Ok(())
}

// iced Point does not derive any serialization 
// so had to use own version for saving data.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ExportPoint{
    x: f32,
    y: f32,
}

impl ExportPoint {
    fn convert(point: &Point) -> Self {
        ExportPoint {x: point.x, y: point.y}
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct ExportColor {
    /// Red component, 0.0 - 1.0
    pub r: f32,
    /// Green component, 0.0 - 1.0
    pub g: f32,
    /// Blue component, 0.0 - 1.0
    pub b: f32,
    /// Transparency, 0.0 - 1.0
    pub a: f32,
}

impl ExportColor {
    pub const fn from_rgba(color: &Color) -> ExportColor {
        ExportColor { r: color.r, g: color.g, b: color.b, a: color.a }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ExportHorizontal {
   Left,
   Center,
   Right,
   None,
}


pub fn convert_to_export_horizontal(h: alignment::Horizontal) -> ExportHorizontal {
    match h {
        alignment::Horizontal::Left => ExportHorizontal::Left,
        alignment::Horizontal::Center => ExportHorizontal::Center,
        alignment::Horizontal::Right => ExportHorizontal::Right,
    }
}

pub fn convert_to_iced_horizontal(h: ExportHorizontal) -> alignment::Horizontal {
    match h {
        ExportHorizontal::Left => alignment::Horizontal::Left,
        ExportHorizontal::Center => alignment::Horizontal::Center,
        ExportHorizontal::Right => alignment::Horizontal::Right,
        ExportHorizontal::None => panic!("no matching iced alingmnet::Horizontal"),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ExportVertical {
   Top,
   Center,
   Bottom,
   None,
}

pub fn convert_to_export_vertical(v: alignment::Vertical) -> ExportVertical {
    match v {
        alignment::Vertical::Top => ExportVertical::Top,
        alignment::Vertical::Center => ExportVertical::Center,
        alignment::Vertical::Bottom => ExportVertical::Bottom,
    }
}

pub fn convert_to_iced_vertical(v: ExportVertical) -> alignment::Vertical {
    match v {
        ExportVertical::Top => alignment::Vertical::Top,
        ExportVertical::Center => alignment::Vertical::Center,
        ExportVertical::Bottom => alignment::Vertical::Bottom,
        ExportVertical::None => panic!("no matching iced alingmnet::Vertical"),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportWidget {
    pub name: IpgCanvasWidget,
    pub content: String,
    pub points: Vec<ExportPoint>,
    pub poly_points: usize,
    pub mid_point: ExportPoint,
    pub other_point: ExportPoint,
    pub rotation: f32,
    pub radius: f32,
    pub color: ExportColor,
    pub fill_color: ExportColor,
    pub width: f32,
    pub horizontal_alignment: ExportHorizontal,
    pub vertical_alignment: ExportVertical,
}

#[allow(clippy::redundant_closure)]
pub fn import_widgets(widgets: Vec<ExportWidget>) 
                        -> (HashMap<usize, IpgWidget>, HashMap<usize, IpgWidget>) {
    
    let mut curves: HashMap<usize, IpgWidget> = HashMap::new();
    let mut text_curves: HashMap<usize, IpgWidget> = HashMap::new();

    for widget in widgets.iter() {
        let points: Vec<Point> = widget.points.iter().map(|p| convert_to_point(p)).collect();
        let other_point = convert_to_point(&widget.other_point);
        let color = convert_to_color(&widget.color);
        let width = widget.width;
        let draw_mode = IpgDrawMode::Display;
        let mid_point = convert_to_point(&widget.mid_point);
        let status = IpgDrawStatus::Completed;
        let no_color = ExportColor{r: 0.0, g: 0.0, b: 0.0, a: 0.0};
        let fill_color = 
            if widget.fill_color == no_color {
                    None
                } else {
                    Some(convert_to_color(&widget.fill_color))
                };
        
        match widget.name {
            IpgCanvasWidget::None => {
            },
            IpgCanvasWidget::Arc => {
                let id = Id::unique();
                let arc = IpgArc {
                    id: id.clone(),
                    points,
                    mid_point,
                    radius: widget.radius,
                    color,
                    fill_color,
                    width,
                    start_angle: Radians(other_point.x),
                    end_angle: Radians(other_point.y),
                    draw_mode,
                    status,
                };
                
                curves.insert(id, IpgWidget::Arc(arc));
            },
            IpgCanvasWidget::Bezier => {
                let id = Id::unique();
                let bz = IpgBezier {
                    id: id.clone(),
                    points,
                    mid_point,
                    color,
                    fill_color,
                    width,
                    rotation: widget.rotation,
                    draw_mode,
                    status
                };
                
                curves.insert(id, IpgWidget::Bezier(bz));
            },
            IpgCanvasWidget::Circle => {
                let id = Id::unique();
                let cir = IpgCircle {
                    id: id.clone(),
                    center: mid_point,
                    circle_point: convert_to_point(&widget.points[0]),
                    radius: widget.radius,
                    color,
                    fill_color,
                    width,
                    draw_mode,
                    status: IpgDrawStatus::Completed,
                };
                
                curves.insert(id, IpgWidget::Circle(cir));
            },
            IpgCanvasWidget::Ellipse => {
                let id = Id::unique();
                let vx = points[1].distance(points[0]);
                let vy = points[2].distance(points[0]);
                let ell = IpgEllipse {
                    id: id.clone(),
                    points,
                    center: convert_to_point(&widget.points[0]),
                    radii: Vector { x: vx, y: vy },
                    rotation: Radians(widget.rotation),
                    color,
                    fill_color,
                    width,
                    draw_mode,
                    status: IpgDrawStatus::Completed,
                };
                
                curves.insert(id, IpgWidget::Ellipse(ell));
            },
            IpgCanvasWidget::Line => {
                let id = Id::unique();
                let ln = IpgLine {
                    id: id.clone(),
                    points,
                    mid_point,
                    color,
                    width,
                    rotation: widget.rotation,
                    draw_mode,
                    status: IpgDrawStatus::Completed,
                };
                curves.insert(id, IpgWidget::Line(ln));
            },
            IpgCanvasWidget::Polygon => {
                let id = Id::unique();
                let pg = IpgPolygon {
                    id: id.clone(),
                    points,
                    poly_points: widget.poly_points,
                    mid_point,
                    pg_point: other_point,
                    color,
                    fill_color,
                    width,
                    rotation: widget.rotation,
                    draw_mode,
                    status: IpgDrawStatus::Completed,
                };
                curves.insert(id, IpgWidget::Polygon(pg));
            },
            IpgCanvasWidget::PolyLine => {
                let id = Id::unique();
                let pl = IpgPolyLine {
                    id: id.clone(),
                    points,
                    poly_points: widget.poly_points,
                    mid_point,
                    pl_point: other_point,
                    color,
                    width,
                    rotation: widget.rotation,
                    draw_mode,
                    status: IpgDrawStatus::Completed,
                };
                curves.insert(id, IpgWidget::PolyLine(pl));
            },
            IpgCanvasWidget::RightTriangle => {
                let id = Id::unique();
                let tr = IpgRightTriangle {
                    id: id.clone(),
                    points,
                    mid_point,
                    tr_point: other_point,
                    color,
                    fill_color,
                    width,
                    rotation: widget.rotation,
                    draw_mode,
                    status: IpgDrawStatus::Completed,
                };
                curves.insert(id, IpgWidget::RightTriangle(tr));
            },
            IpgCanvasWidget::FreeHand => {
                let id = Id::unique();
                let fh = IpgFreeHand {
                    id: id.clone(),
                    points,
                    color,
                    width,
                    draw_mode,
                    status: IpgDrawStatus::Completed,
                    completed: true,
                };
                curves.insert(id, IpgWidget::FreeHand(fh));
            }
            IpgCanvasWidget::Text => {
                let id = Id::unique();
                let txt = IpgText {
                    id: id.clone(),
                    content: widget.content.clone(),
                    position: other_point,
                    color,
                    size: Pixels(16.0),
                    line_height: LineHeight::Relative(1.2),
                    font: Font::default(),
                    horizontal_alignment: convert_to_iced_horizontal(widget.horizontal_alignment),
                    vertical_alignment: convert_to_iced_vertical(widget.vertical_alignment),
                    shaping: Shaping::Basic,
                    rotation: widget.rotation,
                    draw_mode,
                    status: IpgDrawStatus::Completed,
                };
                text_curves.insert(id, IpgWidget::Text(txt));
            }
        }
    }

   (curves, text_curves)

}

pub fn convert_to_export(widgets: &HashMap<Id, IpgWidget>, 
                        text: &HashMap<Id, IpgWidget>) 
                        -> Vec<ExportWidget> {

    let mut curves = widgets.clone();
    for (k, v) in text.iter() {
        curves.insert(k.clone(), v.clone());
    }

    let mut export = vec![];

    for (_id, widget) in curves.iter() {

        let (name, 
            points, 
            mid_point,
            other_point, 
            poly_points, 
            rotation,
            radius,
            color,
            fill_color, 
            width,
            content ,
            horizontal_alignment,
            vertical_alignment,
            ) = 
            match widget {
                IpgWidget::Arc(arc) => {
                    let other_point = Point{ x: arc.start_angle.0, y: arc.end_angle.0 };
                    (IpgCanvasWidget::Arc, &arc.points, arc.mid_point, other_point, 0, 0.0, arc.radius, 
                        arc.color, arc.fill_color, arc.width, String::new(), ExportHorizontal::None, ExportVertical::None)
                },
                IpgWidget::Bezier(bz) => {
                    (IpgCanvasWidget::Bezier, &bz.points, bz.mid_point, Point::default(), 0, bz.rotation, 0.0, 
                    bz.color, bz.fill_color, bz.width, String::new(), ExportHorizontal::None, ExportVertical::None)
                },
                IpgWidget::Circle(cir) => {
                    (IpgCanvasWidget::Circle, &vec![cir.circle_point], cir.center, cir.circle_point, 0, 0.0, cir.radius, 
                        cir.color, cir.fill_color, cir.width, String::new(), ExportHorizontal::None, ExportVertical::None)
                },
                IpgWidget::Ellipse(ell) => {
                    (IpgCanvasWidget::Ellipse, &ell.points, ell.center, Point::default(), 0, ell.rotation.0, 0.0, 
                    ell.color, ell.fill_color, ell.width, String::new(), ExportHorizontal::None, ExportVertical::None)
                },
                IpgWidget::Line(ln) => {
                    (IpgCanvasWidget::Line, &ln.points, ln.mid_point, Point::default(), 0, ln.rotation, 0.0, 
                    ln.color, Some(Color::TRANSPARENT), ln.width, String::new(), ExportHorizontal::None, ExportVertical::None)
                },
                IpgWidget::Polygon(pg) => {
                    (IpgCanvasWidget::Polygon, &pg.points, pg.mid_point, pg.pg_point, pg.poly_points, pg.rotation, 0.0, 
                        pg.color, pg.fill_color, pg.width, String::new(), ExportHorizontal::None, ExportVertical::None)
                },
                IpgWidget::PolyLine(pl) => {
                    (IpgCanvasWidget::PolyLine, &pl.points, pl.mid_point, pl.pl_point, pl.poly_points, pl.rotation, 0.0, 
                        pl.color, Some(Color::TRANSPARENT), pl.width, String::new(), ExportHorizontal::None, ExportVertical::None)
                },
                IpgWidget::RightTriangle(tr) => {
                    (IpgCanvasWidget::RightTriangle, &tr.points, tr.mid_point, tr.tr_point, 3, tr.rotation, 0.0, 
                        tr.color, tr.fill_color, tr.width, String::new(), ExportHorizontal::None, ExportVertical::None)
                },
                IpgWidget::FreeHand(fh) => {
                    (IpgCanvasWidget::FreeHand, &fh.points, Point::default(), Point::default(), 0, 0.0, 0.0, 
                    fh.color, Some(Color::TRANSPARENT), fh.width, String::new(), ExportHorizontal::None, ExportVertical::None)
                }
                IpgWidget::Text(txt) => {
                    (IpgCanvasWidget::Text, &vec![], Point::default(), txt.position, 0, txt.rotation, 0.0, 
                    txt.color, Some(Color::TRANSPARENT), 0.0, txt.content.clone(), 
                    convert_to_export_horizontal(txt.horizontal_alignment), convert_to_export_vertical(txt.vertical_alignment))
                },
                _ => {
                    (IpgCanvasWidget::None, &vec![], Point::default(), Point::default(), 0, 0.0, 0.0, 
                    Color::TRANSPARENT, Some(Color::TRANSPARENT), 0.0, String::new(), ExportHorizontal::None, ExportVertical::None)
                },
        };

        let x_color = ExportColor::from_rgba(&color);
        let x_fill_color = ExportColor::from_rgba(&fill_color.unwrap());
        let x_mid_pt = ExportPoint::convert(&mid_point);
        let x_other_point = ExportPoint::convert(&other_point);
        let mut x_points = vec![];
        for point in points.iter() {
            x_points.push(ExportPoint::convert(point));
        }
        
        export.push(
            ExportWidget{
                name,
                content,
                points: x_points,
                poly_points, 
                mid_point: x_mid_pt,
                other_point: x_other_point,
                rotation,
                radius, 
                color: x_color,
                fill_color: x_fill_color, 
                width,
                horizontal_alignment,
                vertical_alignment,  
            })
    }
    
    export

}

fn convert_to_point(point: &ExportPoint) -> Point {
    Point { x: point.x, y: point.y }
}

fn convert_to_color(color: &ExportColor) -> Color {
    Color::from_rgba(color.r, color.g, color.b, color.a)
}
