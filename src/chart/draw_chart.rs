//! draw_chart
// #![allow(clippy::unnecessary_unwrap)]
use std::f32::consts::PI;

use charts_rs_mod::IcedComponent;
use iced::widget::text::{LineHeight, Shaping};
use iced::{alignment, mouse, Color, Font, Pixels, Size, Vector};
use iced::widget::canvas::event::{self, Event};
use iced::widget::canvas::{self, stroke, Canvas, Frame, Geometry, Path, Stroke};
use iced::{Element, Point, Renderer, Theme};
use pyo3::pyclass;

use super::geometries::{ChartImage, ChartCircle, ChartEllipse, 
    ChartLine, ChartPolyLine, ChartPolygon, ChartRectangle, ChartText};


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
    pub selected_widget: Option<IcedComponent>,
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
                    ) -> Element<'a, IcedComponent> {
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

        // let background =
            // self.state.background_cache.draw(renderer, bounds.size(), 
            //                 |frame| {

            //     let path = Path::rectangle(Point::ORIGIN, frame.size());
            //     if self.state.selected_chart_color.is_some() {
            //         frame.fill(&path, self.state.selected_chart_color.unwrap());
            //     }
                
            //     frame.stroke(
            //         &Path::rectangle(Point::ORIGIN, frame.size()),
            //         Stroke::default()
            //             .with_width(2.0)
            //             .with_color(theme.palette().text),
            //     );
            // });

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
        
        // let mut image_content = vec![];
        // for (i, image_curve) in self.image_curves.iter().enumerate() {
        //     image_content.push(self.state.image_cache[i].draw(renderer, bounds.size(), |frame| {
        //         DrawCurve::draw_image(image_curve, frame, theme);
        //     }));
        // }
            
        let mut content = vec![content];
        content.append(&mut text_content);
        // content.append(&mut image_content);
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
            dbg!(widget);
            let (path, 
                color, 
                width, 
                // _offset,
                // line_dash,
                ) = 
                match &widget {
                    IcedComponent::Circle(cir) => {
                        let fill_color: Option<Color> = match cir.fill_color {
                            Some(c) => Some(convert_color(c)),
                            None => None,
                        };
                        let stroke_color: Color = convert_color(cir.stroke_color);
                        
                        let path = Path::new(|p| { 
                            p.circle(Point::new(cir.center.0, cir.center.1), cir.radius);
                        });
                        if fill_color.is_some() {
                            frame.fill(&path, fill_color.unwrap());
                        }

                        (Some(path), Some(stroke_color), Some(cir.stroke_width))
                        //, cir.stroke_dash_offset, cir.stroke_dash_segments.clone())
                        
                    },
                    IcedComponent::Line(line) => {
                        let path = 
                            Path::new(|p| {
                                p.move_to(Point::new(line.move_to.0, line.line_to.1));
                                p.line_to(Point::new(line.line_to.0, line.line_to.1));
                            });
                        let stroke_color = convert_color(line.stroke_color);

                        (Some(path), Some(stroke_color), Some(line.stroke_width))
                    },
                    IcedComponent::Polyline(pl) => {
                        let path = 
                            Path::new(|p| {
                                for (index, (x, y)) in pl.points.iter().enumerate() {
                                    if index == 0 {
                                        p.move_to(Point::new(*x, *y));
                                    } else {
                                        p.line_to(Point::new(*x, *y));
                                    }
                                }
                            });
                        let stroke_color = convert_color(pl.stroke_color);
                        (Some(path), Some(stroke_color), Some(pl.stroke_width))
                    },
                    IcedComponent::Polygon(pg) => {
                        let path = 
                            Path::new(|p| {
                                for (index, (x, y)) in pg.points.iter().enumerate() {
                                    if index == 0 {
                                        p.move_to(Point::new(*x, *y));
                                    } else {
                                        p.line_to(Point::new(*x, *y));
                                    }
                                }
                        });
                        if pg.fill_color.is_some() {
                            frame.fill(&path, convert_color(pg.fill_color.unwrap()));
                        }

                        let stroke_color = convert_color(pg.stroke_color);
                        (Some(path), Some(stroke_color), Some(pg.stroke_width))
                    },
                    IcedComponent::Rect(rect) => {
                        let size = Size::new(rect.width, rect.height);
                        let fill_color = match rect.fill_color {
                            Some(c) => Color::from_rgba8(c.r, c.g, c.b, c.a),
                            None => Color::TRANSPARENT,
                        };
                        let stroke_color = convert_color(rect.stroke_color); 
                       
                        let path =  Path::new(|p| {
                            p.rectangle(Point::new(rect.top_left.0, rect.top_left.1), size)});
                            if rect.fill_color.is_some() {
                                frame.fill(&path, fill_color);
                            }
                              
                            (Some(path), Some(stroke_color), Some(rect.stroke_width))
                    },
                    _ => (None, None, None),
                };

                let color = match color {
                    Some(c) => c,
                    None => Color::TRANSPARENT,
                };
                let width = match width {
                    Some(w) => w,
                    None => 1.0,
                };

                let stroke = 
                    Stroke {
                        style: stroke::Style::Solid(color),
                        width,
                        ..Stroke::default()
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
                    let x = match txt.x {
                        Some(x) => x,
                        None => 0.0,
                    };
                    let y = match txt.y {
                        Some(y) => y,
                        None => 0.0,
                    };
                    let dx = match txt.dx {
                        Some(dx) => dx,
                        None => 0.0,
                    };
                    let dy = match txt.dy {
                        Some(dy) => dy,
                        None => 0.0,
                    };
                    let color = match txt.font_color {
                        Some(c) => convert_color(c),
                        None => Color::TRANSPARENT,
                    };
                    let size = match txt.font_size {
                        Some(s) => Pixels(s),
                        None => Pixels(16.0),
                    };
                    let line_height = match txt.line_height {
                        Some(l) => LineHeight::Relative(l),
                        None => LineHeight::Relative(1.2),
                    };
                    // let font = match txt.font_family.clone() {
                    //     Some(f) => f,
                    //     None => "Roboto".to_string(),
                    // };
                    let horizontal_alignment = 
                        if txt.text_anchor.is_some() {
                            match txt.text_anchor.clone().unwrap().as_str() {
                                "left" => alignment::Horizontal::Left,
                                "center" => alignment::Horizontal::Center,
                                "right" =>alignment::Horizontal::Right,
                                _ => alignment::Horizontal::Left,
                            }
                        } else {
                            alignment::Horizontal::Center                 
                        };
                    // let vertical_alignment = 
                    //     if txt.dominant_baseline.is_some() {
                    //         match txt.text_anchor.clone().unwrap().as_str() {
                    //             "hanging" => alignment::Vertical::Top,
                    //             "middle" => alignment::Vertical::Center,
                    //             "text-top" =>alignment::Vertical::Bottom,
                    //             _ => alignment::Vertical::Top,
                    //         }
                    //     } else {
                    //         alignment::Vertical::Top               
                    //     };
                    // vertical_alignment = 
                    //     if txt.alignment_baseline.is_some() {
                    //         match txt.text_anchor.clone().unwrap().as_str() {
                    //             "hanging" => alignment::Vertical::Top,
                    //             "middle" => alignment::Vertical::Center,
                    //             "text-top" =>alignment::Vertical::Bottom,
                    //             _ => alignment::Vertical::Top,
                    //         }
                    //     } else {
                    //         alignment::Vertical::Top               
                    //     };
                    frame.translate(Vector::new(x, y));
                    frame.fill_text(canvas::Text {
                        content: txt.text.clone(),
                        position: Point::new(dx, dy),
                        color,
                        size,
                        line_height,
                        font: Font::with_name(&"Roboto"),
                        horizontal_alignment,
                        vertical_alignment: alignment::Vertical::Top,
                        shaping: Shaping::Basic,
                    });
                    frame.rotate(&0.0 * PI/180.0);
                    
                    (None, Some(color), Some(1.0))
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

    // fn draw_image(image_curve: &IcedComponent, frame: &mut Frame, _theme: &Theme) {
    //     if let IcedComponent::Image(img) = &image_curve {
    //          frame.draw_image(
    //                      img.bounds,
    //                     &img.path,
    //          );
    //      };
    // }

}

fn convert_color(c: [u8; 4]) -> Color {
    Color::from_rgba8(c[0], c[1], c[2], c[3] as f32)
}