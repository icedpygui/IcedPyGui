
use core::fmt;

use charts_rs_mod::{Position, Symbol};
use iced::alignment::Horizontal;
use iced::{self, Color, Point};
use iced::widget::canvas::Path;

use super::path::smooth_curve;

pub enum Component {
    Arrow(ChartArrow),
    Bubble(ChartBubble),
    Line(ChartLine),
    Rect(ChartRect),
    Polyline(ChartPolyline),
    Circle(ChartCircle),
    Polygon(ChartPolygon),
    Text(ChartText),
    SmoothLine(SmoothLine),
    StraightLine(StraightLine),
    SmoothLineFill(SmoothLineFill),
    StraightLineFill(StraightLineFill),
    Grid(Grid),
    Axis(Axis),
    Legend(Legend),
    Pie(Pie),
}
#[derive(Clone, PartialEq, Debug)]

pub struct ChartLine {
    pub color: Option<iced::Color>,
    pub stroke_width: f32,
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    // dash array
    pub stroke_dash_array: Option<String>,
}

impl Default for ChartLine {
    fn default() -> Self {
        ChartLine {
            color: None,
            stroke_width: 1.0,
            left: 0.0,
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
            stroke_dash_array: None,
        }
    }
}

impl ChartLine {
    pub fn build_path(&self, path: Path) -> Path {
        Path::new(|p| {
            p.move_to(Point::new(self.left, self.top));
            p.line_to(Point::new(self.right, self.right));
        })
    }
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct ChartRect {
    pub color: Option<Color>,
    pub fill: Option<Color>,
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub height: f32,
    pub rx: Option<f32>,
    pub ry: Option<f32>,
}
impl ChartRect {
    pub fn build_path(&self) -> Path {
        Path::new(|p| {
        })
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct ChartPolyline {
    pub color: Option<Color>,
    pub stroke_width: f32,
    pub points: Vec<Point>,
}

impl Default for ChartPolyline {
    fn default() -> Self {
        ChartPolyline {
            color: None,
            stroke_width: 1.0,
            points: vec![],
        }
    }
}

impl ChartPolyline {
    pub fn build_path(&self) -> Path {
        Path::new(|p| {

        })
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct ChartCircle {
    pub stroke_color: Option<Color>,
    pub fill: Option<Color>,
    pub stroke_width: f32,
    pub cx: f32,
    pub cy: f32,
    pub r: f32,
}

impl Default for ChartCircle {
    fn default() -> Self {
        ChartCircle {
            stroke_color: None,
            fill: None,
            stroke_width: 1.0,
            cx: 0.0,
            cy: 0.0,
            r: 3.0,
        }
    }
}

impl ChartCircle {
    pub fn build_path(&self) -> Path {
        Path::new(|p| {

        })
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct ChartArrow {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub stroke_color: Color,
}
impl ChartArrow {
    pub fn default() -> Self {
        ChartArrow {
            x: 0.0,
            y: 0.0,
            width: 10.0,
            stroke_color: Color::default(),
        }
    }
    pub fn build_path(&self) -> Path {
        let x_offset = self.width / 2.0;
        let y_offset = self.width / 2.0;
        let points = vec![
            Point {
                x: self.x,
                y: self.y,
            },
            Point {
                x: self.x - x_offset,
                y: self.y - y_offset,
            },
            Point {
                x: self.x + self.width,
                y: self.y,
            },
            Point {
                x: self.x - x_offset,
                y: self.y + y_offset,
            },
        ];
        let st_line = StraightLine {
            color: Some(self.stroke_color),
            fill: Some(self.stroke_color),
            points,
            close: true,
            symbol: None,
            ..Default::default()
        };

        Path::new(|p| {

        })

    }
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct ChartPolygon {
    pub color: Option<Color>,
    pub fill: Option<Color>,
    pub points: Vec<Point>,
}

impl ChartPolygon {
    pub fn build_path(&self) -> Path {
        
        Path::new(|p| {
        
        })
        
    }
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct ChartBubble {
    pub r: f32,
    pub x: f32,
    pub y: f32,
    pub fill: Color,
}

impl ChartBubble {
    pub fn build_path(&self) -> Path {

        let first = get_pie_point(self.x, self.y, self.r, -140.0);
        let last = get_pie_point(self.x, self.y, self.r, 140.0);
        
        let path_list = vec![
            first,
            
            self.x + self.r,
            last.x,
            last.y,
            self.y + self.r * 1.5,
        ];

        Path::new(|p| {
            for (index, pt) in path_list.iter().enumerate() {
                if index == 0 {
                    p.move_to(*pt);
                } else {
                    p.line_to(*pt);
                }
            }
        })
    }
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct ChartText {
    pub text: String,
    pub font_family: Option<String>,
    pub font_size: Option<f32>,
    pub font_color: Option<Color>,
    pub line_height: Option<f32>,
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub dx: Option<f32>,
    pub dy: Option<f32>,
    pub font_weight: Option<String>,
    pub transform: Option<String>,
    pub dominant_baseline: Option<String>,
    pub text_anchor: Option<String>,
    pub alignment_baseline: Option<String>,
}

impl ChartText {
    pub fn build_path(&self) -> Path {
        
        Path::new(|p| {

        })
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Pie {
    pub fill: Color,
    pub stroke_color: Option<Color>,
    pub cx: f32,
    pub cy: f32,
    pub r: f32,
    pub ir: f32,
    pub start_angle: f32,
    pub delta: f32,
    pub border_radius: f32,
}

impl Default for Pie {
    fn default() -> Self {
        Pie {
            fill: Color::TRANSPARENT,
            stroke_color: None,
            cx: 0.0,
            cy: 0.0,
            r: 250.0,
            ir: 60.0,
            start_angle: 0.0,
            delta: 0.0,
            border_radius: 8.0,
        }
    }
}

impl Pie {
    pub fn build_path(&self) -> Path {
        
        let mut path_list = vec![];
        let mut border_radius = self.border_radius;
        if border_radius != 0.0 && self.r - self.ir < border_radius {
            border_radius = 2.0;
        }
    
        let border_angle = 2.0_f32;
        let start_angle = self.start_angle;
        let end_angle = start_angle + self.delta;

        // The first point in the lower left corner
        let point = get_pie_point(self.cx, self.cy, self.ir + border_radius, start_angle);
        path_list.push(point);
        
        // Left straight line
        let point = get_pie_point(self.cx, self.cy, self.r - border_radius, start_angle);
        path_list.push(point);

        // Top left rounded corner
        let point = get_pie_point(self.cx, self.cy, self.r, start_angle + border_angle);
        path_list.push(point);

        // large arc
        // If it is too big, cut it in half first.
        if self.delta > 180.0 {
            let point = get_pie_point(
                self.cx,
                self.cy,
                self.r,
                self.start_angle + 180.0 - border_angle,
            );
            path_list.push(point);
        }

        let point = get_pie_point(self.cx, self.cy, self.r, end_angle - border_angle);
        path_list.push(point);

        // Top right rounded corner
        let point = get_pie_point(self.cx, self.cy, self.r - border_radius, end_angle);
        path_list.push(point);

        // Right straight line
        let point = get_pie_point(self.cx, self.cy, self.ir + border_radius, end_angle);
        path_list.push(point);

        if self.ir > 0.0 {
            // Bottom right rounded corner
            let point = get_pie_point(self.cx, self.cy, self.ir, end_angle - border_angle);
            path_list.push(point);

            // small arc
            // If it is too big, cut it in half first.
            if self.delta > 180.0 {
                let point = get_pie_point(self.cx, self.cy, self.ir, end_angle - 180.0);
                path_list.push(point);
            }

            let point = get_pie_point(self.cx, self.cy, self.ir, start_angle + border_angle);
            path_list.push(point);

            // Bottom left rounded corner
            let point = get_pie_point(self.cx, self.cy, self.ir + border_radius, start_angle);
            path_list.push(point);
        }

        Path::new(|p| {
            for (index, pt) in path_list.iter().enumerate() {
                if index == 0 {
                    p.move_to(*pt);
                } else {
                    p.line_to(*pt);
                }
            }
        })
        
    }
}

struct BaseLine {
    pub color: Option<iced::Color>,
    pub fill: Option<iced::Color>,
    pub points: Vec<iced::Point>,
    pub stroke_width: f32,
    pub symbol: Option<Symbol>,
    pub is_smooth: bool,
    pub close: bool,
    pub stroke_dash_array: Option<String>,
}

impl BaseLine {
    pub fn build_path(&self) -> Path {

        let points = if self.is_smooth{
            smooth_curve(
                self.points.clone(),
                false)
        } else {
            self.points
        };

        Path::new(|p| {
            for (index, pt) in points.iter().enumerate() {
                if index == 0 {
                    p.move_to(*pt);
                } else {
                    p.line_to(*pt);
                }
            }
        })
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct SmoothLine {
    pub color: Option<Color>,
    pub points: Vec<Point>,
    pub stroke_width: f32,
    pub symbol: Option<Symbol>,
    pub stroke_dash_array: Option<String>,
}

impl Default for SmoothLine {
    fn default() -> Self {
        SmoothLine {
            color: None,
            points: vec![],
            stroke_width: 1.0,
            symbol: Some(Symbol::Circle(2.0, None)),
            stroke_dash_array: None,
        }
    }
}

impl SmoothLine {
    pub fn build_path(&self) -> String {
        let bs = BaseLine {
            color: self.color,
            fill: None,
            points: self.points.clone(),
            stroke_width: self.stroke_width,
            symbol: self.symbol.clone(),
            is_smooth: true,
            close: false,
            stroke_dash_array: self.stroke_dash_array.clone(),
        };

        Path::new(|p| {

        })
        
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct SmoothLineFill {
    pub fill: Color,
    pub points: Vec<Point>,
    pub bottom: f32,
}

impl Default for SmoothLineFill {
    fn default() -> Self {
        SmoothLineFill {
            fill: (255, 255, 255, 255).into(),
            points: vec![],
            bottom: 0.0,
        }
    }
}

impl SmoothLineFill {
    pub fn build_path(&self) -> Path {
        let mut path = SmoothCurve {
            points: self.points.clone(),
            ..Default::default()
        };

        let last = self.points[self.points.len() - 1];
        let first = self.points[0];
        let fill_path = [
            Point::new(last.x, last.y),
            Point::new(last.x, self.bottom),
            Point::new(first.x, self.bottom),
            Point::new(first.x, first.y),
        ];

        path.push(fill_path);

        Path::new(|p| {

        })
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct StraightLine {
    pub color: Option<Color>,
    pub fill: Option<Color>,
    pub points: Vec<Point>,
    pub stroke_width: f32,
    pub symbol: Option<Symbol>,
    pub close: bool,
    pub stroke_dash_array: Option<String>,
}

impl Default for StraightLine {
    fn default() -> Self {
        StraightLine {
            color: None,
            fill: None,
            points: vec![],
            stroke_width: 1.0,
            symbol: Some(Symbol::Circle(2.0, None)),
            close: false,
            stroke_dash_array: None,
        }
    }
}

impl StraightLine {
    pub fn build_path(&self) -> Path {
        let bl = BaseLine {
            color: self.color,
            fill: self.fill,
            points: self.points.clone(),
            stroke_width: self.stroke_width,
            symbol: self.symbol.clone(),
            is_smooth: false,
            close: self.close,
            stroke_dash_array: self.stroke_dash_array.clone(),
        };

        Path::new(|p| {

        })
        
    }
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct StraightLineFill {
    pub fill: Color,
    pub points: Vec<Point>,
    pub bottom: f32,
    pub close: bool,
}

impl StraightLineFill {
    pub fn svg(&self) -> Path {
        let mut points = self.points.clone();
        let last = points[self.points.len() - 1];
        let first = points[0];
        points.push((last.x, self.bottom).into());
        points.push((first.x, self.bottom).into());
        points.push(first);
        let mut arr = vec![];
        for (index, p) in points.iter().enumerate() {
            
            arr.push(p);
        }
        if self.close {
            arr.push(&points[0]);
        }
        Path::new(|p| {

        })
    }
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Grid {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub color: Option<Color>,
    pub stroke_width: f32,
    pub verticals: usize,
    pub hidden_verticals: Vec<usize>,
    pub horizontals: usize,
    pub hidden_horizontals: Vec<usize>,
}

impl Grid {
    pub fn build_path(&self) -> Path {
        if (self.verticals == 0 && self.horizontals == 0) || self.stroke_width <= 0.0 {
            return Path::new(|p| {})
        }
        let mut points = vec![];
        if self.verticals != 0 {
            let unit = (self.right - self.left) / (self.verticals) as f32;
            for index in 0..=self.verticals {
                if self.hidden_verticals.contains(&index) {
                    continue;
                }
                let x = self.left + unit * index as f32;
                points.push((x, self.top, x, self.bottom));
            }
        }
        if self.horizontals != 0 {
            let unit = (self.bottom - self.top) / (self.horizontals) as f32;
            for index in 0..=self.horizontals {
                if self.hidden_horizontals.contains(&index) {
                    continue;
                }
                let y = self.top + unit * index as f32;
                points.push((self.left, y, self.right, y));
            }
        }
        let mut data = vec![];
        for (left, top, right, bottom) in points.iter() {
            let cl = ChartLine {
                color: None,
                stroke_width: self.stroke_width,
                left: left.to_owned(),
                top: top.to_owned(),
                right: right.to_owned(),
                bottom: bottom.to_owned(),
                ..Default::default()
            };
            data.push(cl);
        }

        Path::new(|p| {

        })

    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Axis {
    pub position: Position,
    pub split_number: usize,
    pub font_size: f32,
    pub font_family: String,
    pub font_color: Option<Color>,
    pub font_weight: Option<String>,
    pub data: Vec<String>,
    pub formatter: Option<String>,
    pub name_gap: f32,
    pub name_align: Horizontal,
    pub name_rotate: f32,
    pub stroke_color: Option<Color>,
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub height: f32,
    pub tick_length: f32,
    pub tick_start: usize,
    pub tick_interval: usize,
    pub is_transparent: bool,
}
impl Default for Axis {
    fn default() -> Self {
        Axis {
            position: Position::Bottom,
            split_number: 0,
            font_size: 14.0,
            font_family: "Roboto".to_string(),
            data: vec![],
            formatter: None,
            font_color: None,
            font_weight: None,
            stroke_color: None,
            name_gap: 5.0,
            name_rotate: 0.0,
            name_align: Horizontal::Center,
            left: 0.0,
            top: 0.0,
            width: 0.0,
            height: 0.0,
            tick_length: 5.0,
            tick_start: 0,
            tick_interval: 0,
            is_transparent: false,
        }
    }
}

impl Axis {
    pub fn svg(&self) -> Result<String> {
        let left = self.left;
        let top = self.top;
        let width = self.width;
        let height = self.height;
        let tick_length = self.tick_length;

        let stroke_width = 1.0;

        let mut line_data = vec![];
        if !self.is_transparent {
            let values = match self.position {
                Position::Top => {
                    let y = top + height;
                    (left, y, left + width, y)
                }
                Position::Right => {
                    let y = top + height;
                    (left, top, left, y)
                }
                Position::Bottom => (left, top, left + width, top),
                _ => {
                    let x = left + width;
                    (x, top, x, top + height)
                }
            };

            line_data.push(
                ChartLine {
                    stroke_width,
                    left: values.0,
                    top: values.1,
                    right: values.2,
                    bottom: values.3,
                    ..Default::default()
                }
                .svg(),
            )
        }

        let is_horizontal = self.position == Position::Bottom || self.position == Position::Top;

        let axis_length = if is_horizontal {
            self.width
        } else {
            self.height
        };
        let font_size = self.font_size;
        let formatter = &self.formatter.clone().unwrap_or_default();

        let mut text_list = vec![];
        let mut text_unit_count: usize = 1;
        if font_size > 0.0 && !self.data.is_empty() {
            text_list = self
                .data
                .iter()
                .map(|item| charts_rs::format_string(item, formatter))
                .collect();
            if self.position == Position::Top || self.position == Position::Bottom {
                let f = charts_rs::get_font(&self.font_family).unwrap();
                let total_measure = 
                    charts_rs::measure_text_width_family(f, font_size, &text_list.join(" ")).unwrap();
                // Not enough space
                if total_measure.width() > axis_length {
                    text_unit_count += (total_measure.width() / axis_length).ceil() as usize;
                }
            }
        }

        let mut split_number = self.split_number;
        if split_number == 0 {
            split_number = self.data.len();
        }
        if !self.is_transparent {
            let unit = axis_length / split_number as f32;
            let tick_interval = self.tick_interval.max(text_unit_count);
            let tick_start = self.tick_start;
            for i in 0..=split_number {
                if i < tick_start {
                    continue;
                }
                let index = if i > tick_start { i - tick_start } else { i };
                if i != tick_start && (tick_interval != 0 && index % tick_interval != 0) {
                    continue;
                }

                let values = match self.position {
                    Position::Top => {
                        let x = left + unit * i as f32;
                        let y = top + height;
                        (x, y - tick_length, x, y)
                    }
                    Position::Right => {
                        let y = top + unit * i as f32;
                        (left, y, left + tick_length, y)
                    }
                    Position::Bottom => {
                        let x = left + unit * i as f32;
                        (x, top, x, top + tick_length)
                    }
                    _ => {
                        let y = top + unit * i as f32;
                        let x = left + width;
                        (x, y, x - tick_length, y)
                    }
                };

                line_data.push(
                    ChartLine {
                        stroke_width,
                        left: values.0,
                        top: values.1,
                        right: values.2,
                        bottom: values.3,
                        ..Default::default()
                    }
                );
            }
        }
        let mut text_data = vec![];
        let name_rotate = self.name_rotate / std::f32::consts::PI * 180.0;
        if !text_list.is_empty() {
            let name_gap = self.name_gap;
            let f = charts_rs::get_font(&self.font_family).unwrap();
            let mut data_len = self.data.len();
            let is_name_align_start = self.name_align == Horizontal::Left;
            if is_name_align_start {
                data_len -= 1;
            }
            let unit = axis_length / data_len as f32;

            for (index, text) in text_list.iter().enumerate() {
                if index % text_unit_count != 0 {
                    continue;
                }
                let b = charts_rs::measure_text_width_family(f, font_size, text);
                let mut unit_offset = unit * index as f32 + unit / 2.0;
                if is_name_align_start {
                    unit_offset -= unit / 2.0;
                }
                let text_width = b.width();

                let values = match self.position {
                    Position::Top => {
                        let y = top + height - name_gap;
                        let x = left + unit_offset - text_width / 2.0;
                        (x, y)
                    }
                    Position::Right => {
                        let x = left + name_gap;
                        let y = top + unit_offset + font_size / 2.0;
                        (x, y)
                    }
                    Position::Bottom => {
                        let y = top + font_size + name_gap;
                        let x = left + unit_offset - text_width / 2.0;
                        (x, y)
                    }
                    _ => {
                        let x = left + width - text_width - name_gap;
                        let y = top + unit_offset + font_size / 2.0 - 2.0;
                        (x, y)
                    }
                };
                let mut transform = None;
                let mut x = Some(values.0);
                let mut y = Some(values.1);
                let mut text_anchor = None;
                if name_rotate != 0.0 {
                    let w = self.name_rotate.sin().abs() * b.width();
                    let translate_x = (values.0 + b.width() / 2.0) as i32;
                    let translate_y = (values.1 + w / 2.0) as i32;
                    text_anchor = Some("middle".to_string());

                    let a = name_rotate as i32;
                    transform = Some(format!(
                        "translate({translate_x},{translate_y}) rotate({a})"
                    ));
                    x = None;
                    y = None;
                }

                text_data.push(
                    ChartText {
                        text: text.to_string(),
                        font_family: Some(self.font_family.clone()),
                        font_size: Some(self.font_size),
                        font_color: self.font_color,
                        font_weight: self.font_weight.clone(),
                        x,
                        y,
                        transform,
                        text_anchor,
                        ..Default::default()
                    }
                    .svg(),
                );
            }
        };
        Path::new(|p| {

        })
    }
}

pub(crate) static LEGEND_WIDTH: f32 = 25.0;
pub(crate) static LEGEND_HEIGHT: f32 = 20.0;
pub(crate) static LEGEND_TEXT_MARGIN: f32 = 3.0;
pub(crate) static LEGEND_MARGIN: f32 = 8.0;

pub(crate) fn measure_legends(
    font_family: &str,
    font_size: f32,
    legends: &[&str],
) -> (f32, Vec<f32>) {
    let widths: Vec<f32> = legends
        .iter()
        .map(|item| {
            let text_box = charts_rs::measure_text_width_family(font_family, font_size, item.to_owned())
                .unwrap_or_default();
            text_box.width() + LEGEND_WIDTH + LEGEND_TEXT_MARGIN
        })
        .collect();
    let width: f32 = widths.iter().sum();
    let margin = LEGEND_MARGIN * (legends.len() - 1) as f32;

    (width + margin, widths)
}

#[derive(Clone, PartialEq, Debug, Default)]
pub enum LegendCategory {
    #[default]
    Normal,
    RoundRect,
    Circle,
    Rect,
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Legend {
    pub text: String,
    pub font_size: f32,
    pub font_family: String,
    pub font_color: Option<Color>,
    pub font_weight: Option<String>,
    pub stroke_color: Option<Color>,
    pub fill: Option<Color>,
    pub left: f32,
    pub top: f32,
    pub category: LegendCategory,
}
impl Legend {
    pub fn build_path(&self) -> Path {
        let stroke_width = 2.0;
        let mut data: Vec<String> = vec![];
        match self.category {
            LegendCategory::Rect => {
                let height = 10.0_f32;
                data.push(
                    ChartRect {
                        color: self.stroke_color,
                        fill: self.stroke_color,
                        left: self.left,
                        top: self.top + (LEGEND_HEIGHT - height) / 2.0,
                        width: LEGEND_WIDTH,
                        height,
                        ..Default::default()
                    }
                    .svg(),
                );
            }
            LegendCategory::RoundRect => {
                let height = 10.0_f32;
                data.push(
                    ChartRect {
                        color: self.stroke_color,
                        fill: self.stroke_color,
                        left: self.left,
                        top: self.top + (LEGEND_HEIGHT - height) / 2.0,
                        width: LEGEND_WIDTH,
                        height,
                        rx: Some(2.0),
                        ry: Some(2.0),
                    }
                    .svg(),
                );
            }
            LegendCategory::Circle => {
                data.push(
                    ChartCircle {
                        stroke_width,
                        stroke_color: self.stroke_color,
                        fill: self.fill,
                        cx: self.left + LEGEND_WIDTH * 0.6,
                        cy: self.top + LEGEND_HEIGHT / 2.0,
                        r: 5.5,
                    }
                    .svg(),
                );
            }
            _ => {
                data.push(
                    ChartLine {
                        stroke_width,
                        color: self.stroke_color,
                        left: self.left,
                        top: self.top + LEGEND_HEIGHT / 2.0,
                        right: self.left + LEGEND_WIDTH,
                        bottom: self.top + LEGEND_HEIGHT / 2.0,
                        ..Default::default()
                    }
                    .svg(),
                );
                data.push(
                    ChartCircle {
                        stroke_width,
                        stroke_color: self.stroke_color,
                        fill: self.fill,
                        cx: self.left + LEGEND_WIDTH / 2.0,
                        cy: self.top + LEGEND_HEIGHT / 2.0,
                        r: 5.5,
                    }
                    .svg(),
                );
            }
        }
        data.push(
            ChartText {
                text: self.text.clone(),
                font_family: Some(self.font_family.clone()),
                font_color: self.font_color,
                font_size: Some(self.font_size),
                font_weight: self.font_weight.clone(),
                x: Some(self.left + LEGEND_WIDTH + LEGEND_TEXT_MARGIN),
                y: Some(self.top + self.font_size),
                ..Default::default()
            }
        );
        Path::new(|p| {

        })
    }
}


fn get_pie_point(cx: f32, cy: f32, r: f32, angle: f32) -> Point {
    let value = angle / 180.0 * std::f32::consts::PI;
    let x = cx + r * value.sin();
    let y = cy - r * value.cos();
    Point { x, y }
}

fn get_box_of_points(points: &[Point]) -> Box {
    let mut b = Box {
        left: f32::MAX,
        top: f32::MAX,
        ..Default::default()
    };
    for p in points.iter() {
        if p.x < b.left {
            b.left = p.x;
        }
        if p.x > b.right {
            b.right = p.x;
        }
        if p.y < b.top {
            b.top = p.y;
        }
        if p.y > b.bottom {
            b.bottom = p.y;
        }
    }
    b
}

#[derive(Clone, Debug, Default)]
pub struct Box {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}
impl Box {
    pub fn width(&self) -> f32 {
        self.right - self.left
    }
    pub fn height(&self) -> f32 {
        self.bottom - self.top
    }
    pub fn outer_width(&self) -> f32 {
        self.right
    }
    pub fn outer_height(&self) -> f32 {
        self.bottom
    }
}
impl fmt::Display for Box {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let m = format!(
            "({},{},{},{})",
            format_float(self.left),
            format_float(self.top),
            format_float(self.right),
            format_float(self.bottom)
        );
        write!(f, "{m}")
    }
}

impl From<f32> for Box {
    fn from(val: f32) -> Self {
        Box {
            left: val,
            top: val,
            right: val,
            bottom: val,
        }
    }
}
impl From<(f32, f32)> for Box {
    fn from(val: (f32, f32)) -> Self {
        Box {
            left: val.0,
            top: val.1,
            right: val.0,
            bottom: val.1,
        }
    }
}
impl From<(f32, f32, f32)> for Box {
    fn from(val: (f32, f32, f32)) -> Self {
        Box {
            left: val.0,
            top: val.1,
            right: val.2,
            bottom: val.1,
        }
    }
}
impl From<(f32, f32, f32, f32)> for Box {
    fn from(val: (f32, f32, f32, f32)) -> Self {
        Box {
            left: val.0,
            top: val.1,
            right: val.2,
            bottom: val.3,
        }
    }
}

pub(crate) fn format_float(value: f32) -> String {
    let str = format!("{:.1}", value);
    if str.ends_with(".0") {
        return str.substring(0, str.len() - 2).to_string();
    }
    str
}