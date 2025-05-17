//! draw_chart
// #![allow(clippy::unnecessary_unwrap)]
use std::f32::consts::PI;

use charts_rs_mod::IcedComponent;
use iced::widget::canvas::path::arc::Elliptical;
use iced::{alignment, mouse, Color, Font, Radians, Vector};
use iced::widget::canvas::event::{self, Event};
use iced::widget::canvas::{self, stroke, Canvas, Frame, Geometry, LineDash, Path, Stroke};
use iced::{Element, Point, Renderer, Theme};
use pyo3::pyclass;

use super::geometries::{ChartImage, IpgChartWidget,
    ChartCircle, ChartEllipse, ChartLine, ChartPolyLine, ChartPolygon, ChartRectangle, ChartText};


#[derive(Debug, Clone, PartialEq, Default)]
pub enum ChartWidget {
    #[default]
    None,
    Circle(ChartCircle),
    Ellipse(ChartEllipse),
    Image(ChartImage),
    Line(ChartLine),
    PolyLine(ChartPolyLine),
    Polygon(ChartPolygon),
    Rectangle(ChartRectangle),
    Text(ChartText),
}


#[derive(Clone, Copy, Debug, Default, PartialEq, Eq,)]
#[pyclass(eq, eq_int)]
pub enum ChartDrawMode {
    #[default]
    Display,
    Edit,
    New,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq,)]
pub enum ChartDrawStatus {
    Inprogress,
    Completed,
    Delete,
}


#[derive(Debug)]
pub struct IpgChartState {
    cache: canvas::Cache,
    background_cache: canvas::Cache,
    text_cache: Vec<canvas::Cache>,
    image_cache: Vec<canvas::Cache>,
    pub curves: Vec<IcedComponent>,
    pub text_curves: Vec<IcedComponent>,
    pub image_curves: Vec<IcedComponent>,
    pub draw_mode: ChartDrawMode,
    pub width: f32,
    pub height: f32,
    pub border_color: Option<Color>,
    pub border_width: Option<f32>,
    pub selected_widget: Option<IpgChartWidget>,
    pub selected_chart_color: Option<Color>,
    pub selected_draw_color: Color,
    pub selected_fill_color: Option<Color>,
    pub selected_poly_points: usize,
    pub selected_width: f32,
    pub selected_h_text_alignment: alignment::Horizontal,
    pub selected_v_text_alignment: alignment::Vertical,
    pub timer_event_enabled: bool,
    pub file_path: String,
}

impl Default for IpgChartState {
    fn default() -> Self {
        let mut text_cache = vec![];
        for _ in 0..100 {
            text_cache.push(canvas::Cache::new());
        }
        let mut image_cache = vec![];
        for _ in 0..100 {
            image_cache.push(canvas::Cache::new());
        }
        Self { 
            cache: canvas::Cache::new(),
            background_cache: canvas::Cache::new(),
            text_cache,
            image_cache,
            curves: vec![],
            text_curves: vec![],
            image_curves: vec![],
            draw_mode: ChartDrawMode::Display,
            width: 300.0,
            height: 300.0,
            border_color: None,
            border_width: None,
            selected_widget: None,
            selected_chart_color: None,
            selected_draw_color: Color::from_rgb(0.961, 0.871, 0.702),
            selected_fill_color: None,
            selected_poly_points: 3,
            selected_width: 2.0,
            selected_h_text_alignment: alignment::Horizontal::Center,
            selected_v_text_alignment:alignment::Vertical::Center,
            timer_event_enabled: false,
            file_path: String::new(),
        }
    }
}

impl IpgChartState {
    pub fn view<'a>(&'a self, 
                    curves: &'a Vec<IcedComponent>, 
                    text_curves: &'a Vec<IcedComponent>,
                    image_curves: &'a Vec<IcedComponent>,
                    ) -> Element<'a, ChartWidget> {
        Canvas::new(DrawPending {
            state: self,
            curves,
            text_curves,
            image_curves,
        })
        .width(self.width)
        .height(self.height)
        .into()
    }

    pub fn request_redraw(&mut self) {
        self.cache.clear();
    }

    pub fn request_text_redraw(&mut self) {
        for i in 0..100 {
            self.text_cache[i].clear();
        }
    }

    pub fn request_image_redraw(&mut self) {
        for i in 0..100 {
            self.image_cache[i].clear();
        }
    }

    pub fn clear_background_cache(&mut self) {
        self.background_cache.clear();
    }

    pub fn clear_curves(&mut self) {
        self.curves.clear();
        self.request_redraw();
        self.request_text_redraw();
        self.request_image_redraw();
        self.text_curves.clear();
        self.image_curves.clear();
    }
}

struct DrawPending<'a> {
    state: &'a IpgChartState,
    curves: &'a Vec<IcedComponent>,
    text_curves: &'a Vec<IcedComponent>,
    image_curves: &'a Vec<IcedComponent>,
}

impl canvas::Program<IcedComponent> for DrawPending<'_> {
    type State = ();

    fn update(
        &self,
        _program_state: &mut Self::State,
        _event: Event,
        _bounds: iced::Rectangle,
        _cursor: mouse::Cursor,
    ) -> (event::Status, Option<IcedComponent>) {
        (event::Status::Ignored, None)
    }

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: iced::Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {

        let background =
            self.state.background_cache.draw(renderer, bounds.size(), 
                            |frame| {

                let path = Path::rectangle(Point::ORIGIN, frame.size());
                if self.state.selected_chart_color.is_some() {
                    frame.fill(&path, self.state.selected_chart_color.unwrap());
                }
                
                frame.stroke(
                    &Path::rectangle(Point::ORIGIN, frame.size()),
                    Stroke::default()
                        .with_width(2.0)
                        .with_color(theme.palette().text),
                );
            });

        let content =
            self.state.cache.draw(renderer, bounds.size(), 
                |frame| {
                DrawCurve::draw_all(self.curves, frame, theme);
            });

        let mut text_content = vec![];
        for (i, text_curve) in self.text_curves.iter().enumerate() {
            text_content.push(self.state.text_cache[i].draw(renderer, bounds.size(), |frame| {
                DrawCurve::draw_text(text_curve, frame, theme);
            }));
        }

        let mut image_content = vec![];
        for (i, image_curve) in self.image_curves.iter().enumerate() {
            image_content.push(self.state.image_cache[i].draw(renderer, bounds.size(), |frame| {
                DrawCurve::draw_image(image_curve, frame, theme);
            }));
        }
            
        let mut content = vec![background, content];
        content.append(&mut text_content);
        content.append(&mut image_content);
        content
        

    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        bounds: iced::Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        if cursor.is_over(bounds) {
            mouse::Interaction::Crosshair
        } else {
            mouse::Interaction::default()
        }
    }
}


#[derive(Debug, Clone, Default)]
pub struct DrawCurve {
}

impl DrawCurve {
    fn draw_all(curves: &Vec<IcedComponent>, frame: &mut Frame, _theme: &Theme) {

        for widget in curves.iter() {
            
            let (path, 
                color, 
                width, 
                _offset,
                line_dash,
                ) = 
                match &widget {
                    IcedComponent::Circle(cir) => {
                        let path = Path::new(|p| { 
                            p.circle(Point::new(cir.center.0, cir.center.1), cir.radius);
                        });
                        if cir.fill_color.is_some() {
                            frame.fill(&path, cir.fill_color.unwrap());
                        }

                        (Some(path), Some(cir.stroke), Some(cir.stroke_width), cir.stroke_dash_offset, cir.stroke_dash_segments.clone())
                        
                    },
                    IcedComponent::Ellipse(ell) => {
                        let path = 
                            Path::new(|p| {
                                p.ellipse(Elliptical{ 
                                center: ell.center, 
                                radii: ell.radii, 
                                rotation: ell.rotation, 
                                start_angle: Radians(0.0), 
                                end_angle: Radians(2.0*PI)})
                            });
                        if ell.fill_color.is_some() {
                            frame.fill(&path, ell.fill_color.unwrap());
                        }
                        (Some(path), Some(ell.stroke), Some(ell.stroke_width), None, None)
                    },
                    IcedComponent::Line(line) => {
                        let path = 
                            Path::new(|p| {
                                p.move_to(line.points[0]);
                                p.line_to(line.points[1]);
                            });
                        (Some(path), Some(line.stroke), Some(line.stroke_width), None, None)
                    },
                    IcedComponent::PolyLine(pl) => {
                        let path = 
                            Path::new(|p| {
                                for (index, point) in pl.points.iter().enumerate() {
                                    if index == 0 {
                                        p.move_to(*point);
                                    } else {
                                        p.line_to(*point);
                                    }
                                }
                            });
                        (Some(path), Some(pl.stroke), Some(pl.stroke_width), None, None)
                    },
                    IcedComponent::Polygon(pg) => {
                        let path = 
                            Path::new(|p| {
                                for (index, point) in pg.points.iter().enumerate() {
                                    if index == 0 {
                                        p.move_to(*point);
                                    } else {
                                        p.line_to(*point);
                                    }
                                }
                                p.line_to(pg.points[0]);
                        });
                        if pg.fill_color.is_some() {
                            frame.fill(&path, pg.fill_color.unwrap());
                        }    
                        (Some(path), Some(pg.stroke), Some(pg.stroke_width), None, None)
                    },
                    IcedComponent::Rectangle(rect) => {
                        let path =  Path::new(|p| {
                            p.rectangle(rect.top_left, rect.size)});
                            if rect.fill_color.is_some() {
                                frame.fill(&path, rect.fill_color.unwrap());
                            }    
                            (Some(path), Some(rect.stroke), Some(rect.stroke_width), None, None)
                    },
                    _ => (None, None, None, None, None),
                };

                let stroke = match (line_dash, color, width) {
                    (Some(line_dash), Some(color), Some(width)) => Stroke {
                        style: stroke::Style::Solid(color),
                        width,
                        line_dash: LineDash {
                            offset: 0,
                            segments: &line_dash.clone(),
                        },
                        ..Stroke::default()
                    },
                    (None, Some(color), Some(width)) => Stroke {
                        style: stroke::Style::Solid(color),
                        width,
                        ..Stroke::default()
                    },
                    _ => Stroke::default(),
                };
                
                if let Some(path) = path { frame.stroke(
                    &path,
                    stroke,
                    ) }
        }

    }

    fn draw_text(text_curve: &IcedComponent, frame: &mut Frame, _theme: &Theme) {
        let (path, color, width) = 
            match &text_curve {
                IcedComponent::Text(txt) => {
                    frame.translate(Vector::new(txt.position.x, txt.position.y));
                    frame.fill_text(canvas::Text {
                        content: txt.content.clone(),
                        position: Point::new(0.0, -3.0),
                        color: txt.color,
                        size: txt.size,
                        line_height: txt.line_height,
                        font: Font::with_name("Roboto"),
                        horizontal_alignment: txt.horizontal_alignment,
                        vertical_alignment: txt.vertical_alignment,
                        shaping: txt.shaping,
                    });
                    frame.rotate(&txt.rotation * PI/180.0);
                    
                    (None, Some(txt.color), Some(1.0))
                },
                _ => (None, None, None)
            };

            if let Some(path) = path { frame.stroke(
                &path,
                Stroke::default()
                .with_width(width.unwrap())
                .with_color(color.unwrap()),
                ) }
        
    }

    fn draw_image(image_curve: &IcedComponent, frame: &mut Frame, _theme: &Theme) {
        if let IcedComponent::Image(img) = &image_curve {
             frame.draw_image(
                         img.bounds,
                        &img.path,
             );
         };
    }

}
