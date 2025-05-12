//! draw_chart
// #![allow(clippy::unnecessary_unwrap)]
use std::collections::HashMap;

use iced::{alignment, mouse, Color, Length, Vector};
use iced::widget::canvas::event::{self, Event};
use iced::widget::canvas::{self, stroke, Canvas, Frame, Geometry, LineDash, Path, Stroke};
use iced::{Element, Point, Renderer, Theme};
use pyo3::pyclass;

use crate::chart::path_builds::{build_circle_path, 
    build_ellipse_path, build_line_path, 
    build_polygon_path, build_polyline_path, build_text_path};

use super::chart_helpers::to_radians;
use super::geometries::{add_keypress, add_new_widget, check_if_text_widget, complete_new_widget, find_closest_point_index, find_closest_widget, get_del_key, set_widget_mode_or_status_or_id, set_widget_point, update_edited_widget, IpgChartImage, IpgChartWidget, IpgCircle, IpgEllipse, ChartLine, IpgPolyLine, IpgPolygon, IpgRectangle, IpgText};


#[derive(Debug, Clone, PartialEq, Default)]
pub enum ChartWidget {
    #[default]
    None,
    Circle(IpgCircle),
    Ellipse(IpgEllipse),
    Image(IpgChartImage),
    Line(ChartLine),
    PolyLine(IpgPolyLine),
    Polygon(IpgPolygon),
    Rectangle(IpgRectangle),
    Text(IpgText),
}


#[derive(Clone, Copy, Debug, Default, PartialEq, Eq,)]
#[pyclass(eq, eq_int)]
pub enum IpgDrawMode {
    #[default]
    Display,
    Edit,
    New,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq,)]
pub enum IpgDrawStatus {
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
    pub curves: HashMap<usize, ChartWidget>,
    pub text_curves: HashMap<usize, ChartWidget>,
    pub image_curves: HashMap<usize, ChartWidget>,
    pub draw_mode: IpgDrawMode,
    pub width: Length,
    pub height: Length,
    pub border_color: Option<Color>,
    pub border_width: Option<f32>,
    pub edit_widget_id: Option<usize>,
    pub escape_pressed: bool,
    pub selected_widget: Option<IpgChartWidget>,
    pub selected_chart_color: Option<Color>,
    pub selected_draw_color: Color,
    pub selected_fill_color: Option<Color>,
    pub selected_poly_points: usize,
    pub selected_width: f32,
    pub selected_h_text_alignment: alignment::Horizontal,
    pub selected_v_text_alignment: alignment::Vertical,
    pub timer_event_enabled: bool,
    pub blink: bool,
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
            curves: HashMap::new(),
            text_curves: HashMap::new(),
            image_curves: HashMap::new(),
            draw_mode: IpgDrawMode::Display,
            width: Length::Fill,
            height: Length::Fill,
            border_color: None,
            border_width: None,
            edit_widget_id: None,
            escape_pressed: false,
            selected_widget: None,
            selected_chart_color: None,
            selected_draw_color: Color::from_rgb(0.961, 0.871, 0.702),
            selected_fill_color: None,
            selected_poly_points: 3,
            selected_width: 2.0,
            selected_h_text_alignment: alignment::Horizontal::Center,
            selected_v_text_alignment:alignment::Vertical::Center,
            timer_event_enabled: false,
            blink: false,
            file_path: String::new(),
        }
    }
}

impl IpgChartState {
    pub fn view<'a>(&'a self, 
                    curves: &'a HashMap<usize, ChartWidget>, 
                    text_curves: &'a HashMap<usize, ChartWidget>,
                    image_curves: &'a HashMap<usize, ChartWidget>,
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
    curves: &'a HashMap<usize, ChartWidget>,
    text_curves: &'a HashMap<usize, ChartWidget>,
    image_curves: &'a HashMap<usize, ChartWidget>,
}

impl canvas::Program<ChartWidget> for DrawPending<'_> {
    type State = Option<Pending>;

    fn update(
        &self,
        program_state: &mut Self::State,
        event: Event,
        bounds: iced::Rectangle,
        cursor: mouse::Cursor,
    ) -> (event::Status, Option<ChartWidget>) {
        let Some(cursor_position) = cursor.position_in(bounds) else {
            return (event::Status::Ignored, None);
        };
        
        match event {
            Event::Mouse(mouse_event) => {
                if self.state.escape_pressed {
                    *program_state = None;
                    return (event::Status::Ignored, None)
                }
                
                let message = match mouse_event {
                    mouse::Event::ButtonPressed(mouse::Button::Left) => {
                        match self.state.draw_mode {
                            IpgDrawMode::Display => {
                                None
                            },
                            IpgDrawMode::Edit => {
                                match program_state {
                                    // edit consists of 3 clicks
                                    // 1 - find closest widget
                                    // 2 - find closest point
                                    // 3 - finish
                                    None => {
                                        let widget_opt = 
                                            find_closest_widget(
                                                self.curves, 
                                                self.text_curves, 
                                                cursor_position,
                                            );
                                        
                                        let selected_widget = 
                                            match widget_opt {
                                                Some(w) => w,
                                                None => return (event::Status::Ignored, None),
                                            };

                                        // set draw_mode to indicate being edited
                                        let widget = 
                                            set_widget_mode_or_status_or_id(
                                                selected_widget, 
                                                Some(IpgDrawMode::Edit),
                                                Some(IpgDrawStatus::Inprogress),
                                                None,
                                            );
                                        *program_state = Some(Pending::EditSecond {
                                            widget: widget.clone(),
                                        });
                                        // returning DrawCurve so that the curve
                                        // being editied will not show after the refresh
                                        // The pending process will show the curve
                                        // until its finsihed.

                                        Some(widget)
                                    },
                                    // The second click is a Some() since it was created above
                                    // The pending is carrying the previous info
                                    // This second click will find the point
                                    // and replace with cursor
                                    Some(Pending::EditSecond { 
                                        widget,
                                    }) => {
                                        // Find for closest point to edit in selected widget
                                        // which might be either a mid point(translate) or 
                                        // curve point (move point).
                                        let (point_index, mid_point, other_point) = 
                                            find_closest_point_index(widget, cursor_position);
                                        let widget = widget.clone();
                                        *program_state = Some(Pending::EditThird {
                                            widget: widget.clone(),
                                            edit_point_index: point_index,
                                            edit_mid_point: mid_point,
                                            edit_other_point: other_point,
                                        });
                                        // If a text widget, need to send back the curve so that the
                                        // cursor can be seen.  No access to the time event during pending.
                                        if self.state.selected_widget == Some(IpgChartWidget::Text) {
                                            Some(widget.clone())
                                        } else {
                                            None
                                        }
                                    },
                                    // The third click will send back the DrawCurve
                                    // with the finally updated curve
                                    Some(Pending::EditThird { 
                                        widget,
                                        edit_point_index,
                                        edit_mid_point,
                                        edit_other_point, 
                                    }) => {

                                        let edited_widget: ChartWidget = 
                                                update_edited_widget(
                                                    widget.clone(), 
                                                    cursor_position, 
                                                    *edit_point_index, 
                                                    *edit_mid_point,
                                                    *edit_other_point,
                                                    IpgDrawStatus::Completed,
                                                );
                                        
                                        *program_state = None;
                                        Some(edited_widget)
                                    },
                                    _ => None,
                                }
                            },
                            IpgDrawMode::New => {
                                match program_state {
                                    // First mouse click sets the state of the first Pending point
                                    // return a none since no Curve yet
                                    None => {
                                        if self.state.selected_widget.is_none() {
                                            return (event::Status::Ignored, None)
                                        }
                                        // in case the poly points, color, and width have changed since 
                                        // the widget selected, we set them
                                        let selected_widget = 
                                            add_new_widget(
                                                self.state.selected_widget.unwrap(), 
                                                self.state.selected_poly_points,
                                                self.state.selected_draw_color,
                                                self.state.selected_fill_color,
                                                self.state.selected_width,
                                                self.state.draw_mode,
                                                self.state.selected_h_text_alignment,
                                                self.state.selected_v_text_alignment,
                                            );
                                        
                                        let (widget, _) = 
                                            set_widget_point(
                                                &selected_widget, 
                                                cursor_position,
                                            );
                                        *program_state = Some(Pending::New {
                                            widget: widget.clone(),
                                        });

                                        // If a text widget, need to send back the curve so that the
                                        // cursor can be seen.  No access to the time event during pending.
                                        if self.state.selected_widget == Some(IpgChartWidget::Text) {
                                            Some(widget)
                                        } else {
                                            None
                                        }
                                    },
                                    // The second click is a Some() since it was created above
                                    // The pending is carrying the previous info
                                    Some(Pending::New { 
                                            widget, 
                                    }) => {

                                        let (widget, completed) = 
                                            set_widget_point(widget, cursor_position);
                                        
                                        // if completed, we return the ChartWidget and set the state to none
                                        // if not, then this is repeated until completed.
                                        if completed {
                                            *program_state = None;
                                            complete_new_widget(widget, cursor_position)
                                        } else {
                                            *program_state = Some(Pending::New {
                                                widget: widget.clone(),
                                            });
                                            
                                            if check_if_text_widget(&widget) {
                                                Some(widget)
                                            } else {
                                                None
                                            }
                                        }
                                    },
                                    _ => None,
                                }
                            },
                        }
                    },
                    mouse::Event::WheelScrolled { delta: _} => {
                        match self.state.draw_mode {
                            _ => None,
                        }
                    },
                    _ => None,
                };
                (event::Status::Captured, message)
            },
            Event::Keyboard(key_event) => {
                let message = match key_event {
                    iced::keyboard::Event::KeyPressed { 
                        key:_, 
                        modified_key, 
                        physical_key:_, 
                        location:_, 
                        modifiers:_, 
                        text:_ } => {
                            match program_state {
                                None => None,
                                Some(Pending::New { 
                                    widget }) => {
                                        let (widget, completed) = 
                                            add_keypress(widget, modified_key);
                                        match widget {
                                            Some(widget) => {
                                                // if not completed, keep doing the pending
                                                // and since text only for now, 
                                                // return the curve too.
                                                // if completed for freehand, quit pending 
                                                // and return the curve.
                                                if !completed {
                                                    *program_state = Some(Pending::New { 
                                                        widget: widget.clone(), 
                                                    });
                                                    Some(widget)
                                                } else {
                                                    *program_state = None;
                                                    Some(widget)
                                                }
                                            },
                                            None => {
                                                *program_state = None;
                                                None
                                            }
                                        }
                                    },
                                    Some(Pending::EditSecond { 
                                        widget }) => {
                                            let del_key = get_del_key(modified_key);
                                            let del_widget = if del_key {
                                                set_widget_mode_or_status_or_id(
                                                    widget.clone(), 
                                                    None, 
                                                    Some(IpgDrawStatus::Delete),
                                                    None,
                                                )
                                            } else {
                                                widget.clone()
                                            };
                                                
                                            *program_state = None;
                                            Some(del_widget)
                                    },
                                    _ => None,
                            }
                        },
                    iced::keyboard::Event::KeyReleased {key: _, location:_, modifiers:_ } => None,
                    iced::keyboard::Event::ModifiersChanged(_) => None,
                };

                (event::Status::Captured, message)
            },
            _ => (event::Status::Ignored, None),
        }
    }

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: iced::Rectangle,
        cursor: mouse::Cursor,
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
        for (i, (_id, text_curve)) in self.text_curves.iter().enumerate() {
            text_content.push(self.state.text_cache[i].draw(renderer, bounds.size(), |frame| {
                DrawCurve::draw_text(text_curve, self.state.blink, frame, theme);
            }));
        }

        let mut image_content = vec![];
        for (i, (_id, image_curve)) in self.image_curves.iter().enumerate() {
            image_content.push(self.state.image_cache[i].draw(renderer, bounds.size(), |frame| {
                DrawCurve::draw_image(image_curve, frame, theme);
            }));
        }
            
        if let Some(pending) = state {
            let mut content = 
            vec![background, content, pending.draw(renderer, theme, bounds, cursor)];
            content.append(&mut text_content);
            content.append(&mut image_content);
            content
        } else {
            let mut content = vec![background, content];
            content.append(&mut text_content);
            content.append(&mut image_content);
            content
        }

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
    fn draw_all(curves: &HashMap<usize, ChartWidget>, frame: &mut Frame, _theme: &Theme) {
        // This draw only occurs at the completion of the 
        // widget(update occurs) and cache is cleared
        for (_id, widget) in curves.iter() {
            // if first click, skip the curve to be edited so that it 
            // will not be seen until the second click.  Otherwise is shows
            // during editing because there is no way to refresh
            // The pending routine will diplay the curve

            let (path, 
                color, 
                width, 
                offset,
                line_dash,
                ) = 
                match &widget {
                    ChartWidget::Circle(cir) => {
                        // skip if being editied or rotated
                        if cir.status== IpgDrawStatus::Inprogress {
                            (None, None, None, 0, None)
                        } else {
                            let path = 
                                build_circle_path(
                                    cir,
                                    cir.draw_mode,
                                    None, 
                                    None, 
                                    false
                                );
                            if cir.fill_color.is_some() {
                                frame.fill(&path, cir.fill_color.unwrap());
                            }
                            (Some(path), Some(cir.color), Some(cir.width), cir.stroke_dash_offset, cir.stroke_dash_segments.clone())
                        }
                    },
                    ChartWidget::Ellipse(ell) => {
                        // skip if being editied or rotated
                        if ell.status == IpgDrawStatus::Inprogress {
                            (None, None, None, 0, None)
                        } else {
                            let path = 
                                build_ellipse_path(
                                    ell, 
                                    ell.draw_mode,
                                    None, 
                                    None, 
                                    false,
                                );
                            if ell.fill_color.is_some() {
                                frame.fill(&path, ell.fill_color.unwrap());
                            }
                            (Some(path), Some(ell.color), Some(ell.width), 0, None)
                        }
                    },
                    ChartWidget::Line(line) => {
                        dbg!("at line");
                        // skip if being editied or rotated
                        if line.status == IpgDrawStatus::Inprogress {
                            (None, None, None, 0, None)
                        } else {
                            let (path, _, _) = 
                                build_line_path(
                                    line, 
                                    line.draw_mode, 
                                    None, 
                                    None, 
                                    false,
                                    None,
                                    );

                            (Some(path), Some(line.color), Some(line.width), 0, None)
                        }
                    },
                    ChartWidget::PolyLine(pl) => {
                        // skip if being editied or rotated
                        if pl.status == IpgDrawStatus::Inprogress {
                            (None, None, None, 0, None)
                        } else {
                            let (path, _, _) = 
                                build_polyline_path(
                                    pl, 
                                    pl.draw_mode, 
                                    None, 
                                    None, 
                                    false,
                                    false,
                                    None,
                                );
                            (Some(path), Some(pl.color), Some(pl.width), 0, None)
                        }
                    },
                    ChartWidget::Polygon(pg) => {
                        // skip if being editied or rotated
                        if pg.status == IpgDrawStatus::Inprogress {
                            (None, None, None, 0, None)
                        } else {
                            let (path, _, _) = 
                                build_polygon_path(
                                    pg, 
                                    pg.draw_mode, 
                                    None,  
                                    false,
                                    false,
                                    None,
                                );
                            if pg.fill_color.is_some() {
                                frame.fill(&path, pg.fill_color.unwrap());
                            }    
                            (Some(path), Some(pg.color), Some(pg.width), 0, None)
                        }
                    },
                    ChartWidget::Rectangle(rect) => {
                        let path =  Path::new(|p| {
                            p.rectangle(rect.top_left, rect.size)});
                            if rect.fill_color.is_some() {
                                frame.fill(&path, rect.fill_color.unwrap());
                            }    
                            (Some(path), Some(rect.color), Some(rect.width), 0, None)
                    },
                    _ => (None, None, None, 0, None),
                };

                let stroke = match (line_dash, color, width) {
                    (Some(line_dash), Some(color), Some(width)) => Stroke {
                        style: stroke::Style::Solid(color),
                        width,
                        line_dash: LineDash {
                            offset,
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

    fn draw_text(text_curve: &ChartWidget, mut blink: bool, frame: &mut Frame, _theme: &Theme) {
        let (path, color, width) = 
            match &text_curve {
                ChartWidget::Text(txt) => {
                    // During edit or rotate, pending draws the text,
                    // so skip drawing here.  If completed, always draw here.
                    if txt.draw_mode == IpgDrawMode::Display || 
                        txt.draw_mode == IpgDrawMode::New {
                        if txt.draw_mode == IpgDrawMode::Display {
                            blink = false;
                        }
                        frame.translate(Vector::new(txt.position.x, txt.position.y));
                        let (text, path) = 
                            build_text_path (
                                txt,
                                txt.draw_mode,
                                blink,
                            );
                        frame.rotate(to_radians(&txt.rotation));
                        frame.fill_text(text);
                        
                        (path, Some(txt.color), Some(1.0))
                    } else {
                        (None, None, None)
                    }
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

    fn draw_image(image_curve: &ChartWidget, frame: &mut Frame, _theme: &Theme) {
        if let ChartWidget::Image(img) = &image_curve {
             frame.translate(Vector::new(img.position.x, img.position.y));
             frame.rotate(to_radians(&img.rotation));
             frame.draw_image(
                         img.bounds,
                        &img.path,
             );
         };
    }

}



#[allow(dead_code)]
#[derive(Debug, Clone)]
enum Pending {
    New {
        widget: ChartWidget, 
    },
    EditSecond {
        widget: ChartWidget, 
        },
    EditThird {
        widget: ChartWidget, 
        edit_point_index: Option<usize>,
        edit_mid_point: bool,
        edit_other_point: bool,
        },
}

impl Pending {
    fn draw(
        &self,
        renderer: &Renderer,
        theme: &Theme,
        bounds: iced::Rectangle,
        cursor: mouse::Cursor,
    ) -> Geometry {
        let _ = theme;
        let mut frame = Frame::new(renderer, bounds.size());

        if let Some(cursor) = cursor.position_in(bounds) {
            // This draw happens when the mouse is moved and the state is none.
            match self {
                Pending::New { 
                    widget, 
                } => {
                    let (path, 
                        color, 
                        width,
                        ) = 
                    match widget {
                        ChartWidget::Circle(cir) => {
                            let path = 
                                build_circle_path(
                                    cir, 
                                    IpgDrawMode::New, 
                                    Some(cursor),
                                    None,
                                    false,
                                );
                            if cir.fill_color.is_some() {
                                frame.fill(&path, cir.fill_color.unwrap());
                            }
                            (path, cir.color, cir.width)
                        },
                        ChartWidget::Ellipse(ell) => {
                            let path = 
                                build_ellipse_path(
                                    ell, 
                                    IpgDrawMode::New, 
                                    Some(cursor),
                                    None,
                                    false,
                                );
                            if ell.fill_color.is_some() {
                                frame.fill(&path, ell.fill_color.unwrap());
                            }
                            (path, ell.color, ell.width)
                        }
                        ChartWidget::Line(line) => {
                            let (path, _, _) = 
                                build_line_path(
                                    line, 
                                    IpgDrawMode::New, 
                                    Some(cursor),
                                    None,
                                    false,
                                    None,
                                );
                            (path, line.color, line.width)
                        },
                        ChartWidget::Polygon(pg) => {
                            let (path, _, _) = 
                                build_polygon_path(
                                    pg,
                                    IpgDrawMode::New, 
                                    Some(cursor),
                                    false,
                                    false,
                                    None,
                                );
                            if pg.fill_color.is_some() {
                                frame.fill(&path, pg.fill_color.unwrap());
                            }
                            (path, pg.color, pg.width)
                        },
                        // return points as they are set
                        ChartWidget::PolyLine(pl) => {
                            let (path, _, _) = 
                                build_polyline_path(
                                    pl, 
                                    IpgDrawMode::New, 
                                    Some(cursor),
                                    None,
                                    false,
                                    false,
                                    None,
                                );
                            (path, pl.color, pl.width)
                        },
                        _ => {
                            (Path::new(|_| {}), Color::TRANSPARENT, 0.0)  
                        }
                    };

                    frame.stroke(
                        &path,
                        Stroke::default()
                            .with_width(width)
                            .with_color(color),
                    );
                },
                Pending::EditSecond{
                    widget, 
                } => {
                    let (path, color, width) = 
                        match widget {
                            ChartWidget::Circle(cir) => {
                                let path = 
                                build_circle_path(
                                    cir,
                                    IpgDrawMode::Edit, 
                                    Some(cursor),
                                    None, 
                                    false,
                                );
                                (path, cir.color, cir.width)
                            },
                            ChartWidget::Ellipse(ell) => {
                                let path = 
                                build_ellipse_path(
                                    ell, 
                                    IpgDrawMode::Edit, 
                                    Some(cursor),
                                    None, 
                                    false,
                                );
                                (path, ell.color, ell.width)
                            },
                            ChartWidget::Line(line) => {
                                let (path, _, _) = 
                                build_line_path(
                                    line, 
                                    IpgDrawMode::Edit, 
                                    Some(cursor),
                                    None, 
                                    false,
                                    None,
                                );
                            
                                (path, line.color, line.width)
                            },
                            ChartWidget::Polygon(pg) => {
                                let (path, _, _) = 
                                build_polygon_path(
                                    pg, 
                                    IpgDrawMode::Edit, 
                                    Some(cursor), 
                                    false,
                                    false,
                                    None,
                                );
                                (path, pg.color, pg.width)
                            },
                            ChartWidget::PolyLine(pl) => {
                                let (path, _, _) = 
                                    build_polyline_path(
                                        pl, 
                                        IpgDrawMode::Edit, 
                                        Some(cursor),
                                        None, 
                                        false,
                                        false,
                                        None,
                                    );
                                (path, pl.color, pl.width)
                            },
                            ChartWidget::Text(txt) => {
				                frame.translate(Vector::new(txt.position.x, txt.position.y));
                                let (text, path) = build_text_path (
                                        txt,
                                        IpgDrawMode::Edit,
                                        false,
                                    );
                                    
                                frame.rotate(to_radians(&txt.rotation));
                                frame.fill_text(text);
                                (path.unwrap(), txt.color, 2.0)
                            },
                            _ => {
                                (Path::new(|_| {}), Color::TRANSPARENT, 0.0)
                            },
                        };

                    frame.stroke(
                    &path,
                    Stroke::default()
                        .with_width(width)
                        .with_color(color),
                    );
                },
                Pending::EditThird { 
                    widget,
                    edit_point_index, 
                    edit_mid_point, 
                    edit_other_point, 
                } => {

                    let (path, 
                        color, 
                        width, 
                        ) = match widget {
                        ChartWidget::Circle(cir) => {
                            let path = 
                                build_circle_path(
                                    cir,
                                    IpgDrawMode::Edit, 
                                    Some(cursor),
                                    *edit_point_index, 
                                    *edit_mid_point,
                                );
                            (path, cir.color, cir.width)
                        },
                        ChartWidget::Ellipse(ell) => {
                            let path = 
                                build_ellipse_path(
                                    ell, 
                                    IpgDrawMode::Edit, 
                                    Some(cursor),
                                    *edit_point_index, 
                                    *edit_mid_point,
                                );
                            (path, ell.color, ell.width)
                        },
                        ChartWidget::Line(line) => {
                            let (path, _, _) = 
                                build_line_path(
                                    line, 
                                    IpgDrawMode::Edit, 
                                    Some(cursor),
                                    *edit_point_index, 
                                    *edit_mid_point,
                                    None,
                                );
                            
                            (path, line.color, line.width)
                        },
                        ChartWidget::Polygon(pg) => {
                            let (path, _, _) = 
                                build_polygon_path(
                                    pg, 
                                    IpgDrawMode::Edit, 
                                    Some(cursor), 
                                    *edit_mid_point,
                                    *edit_other_point,
                                    None,
                                );
                            (path, pg.color, pg.width)
                        },
                        ChartWidget::PolyLine(pl) => {
                            let (path, _, _) = 
                                build_polyline_path(
                                    pl, 
                                    IpgDrawMode::Edit, 
                                    Some(cursor),
                                    *edit_point_index, 
                                    *edit_mid_point,
                                    *edit_other_point,
                                    None,
                                );
                            (path, pl.color, pl.width)
                        },
                        ChartWidget::Text(txt) => {
                            frame.translate(Vector::new(cursor.x, cursor.y));
                            let (text, path) = build_text_path (
                                        txt,
                                        IpgDrawMode::Edit,
                                        false,
                                    );

                            frame.rotate(to_radians(&txt.rotation));
                            frame.fill_text(text);
                            (path.unwrap(), Color::TRANSPARENT, 0.0)
                        },
                        _ => {
                            (Path::new(|_| {}), Color::TRANSPARENT, 0.0)
                        },
                    };

                    frame.stroke(
                        &path,
                        Stroke::default()
                            .with_width(width)
                            .with_color(color),
                    );
                },
                
            }
        }
        
        frame.into_geometry()
    }
}


