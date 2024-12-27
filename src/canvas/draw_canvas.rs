//! draw_canvas

use std::collections::HashMap;

use iced::widget::container::Id;
use iced::{alignment, mouse, Color};
use iced::widget::canvas::event::{self, Event};
use iced::widget::canvas::{self, Canvas, Frame, Geometry, Path, Stroke};
use iced::{Element, Fill, Point, Renderer, Theme};
use pyo3::pyclass;

use crate::canvas::canvas_helpers::to_degrees;
use crate::canvas::path_builds::{build_arc_path, build_bezier_path, build_circle_path, 
    build_ellipse_path, build_free_hand_path, build_line_path, 
    build_polygon_path, build_polyline_path, build_right_triangle_path, build_text_path};

use super::geometries::{add_keypress, add_new_widget, complete_new_widget, find_closest_point_index, find_closest_widget, get_del_key, get_widget_degrees, set_widget_mode_or_status, set_widget_point, update_edited_widget, update_rotated_widget, IpgArc, IpgBezier, IpgCircle, IpgEllipse, IpgFreeHand, IpgLine, IpgPolyLine, IpgPolygon, IpgRightTriangle, IpgText, IpgCanvasWidget};


#[derive(Debug, Clone, Default)]
pub enum IpgWidget {
    #[default]
    None,
    Arc(IpgArc),
    Bezier(IpgBezier),
    Circle(IpgCircle),
    Ellipse(IpgEllipse),
    Line(IpgLine),
    PolyLine(IpgPolyLine),
    Polygon(IpgPolygon),
    RightTriangle(IpgRightTriangle),
    Text(IpgText),
    FreeHand(IpgFreeHand),
}

#[pyclass]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq,)]
pub enum IpgDrawMode {
    #[default]
    DrawAll,
    Edit,
    New,
    Rotate,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq,)]
pub enum IpgDrawStatus {
    Inprogress,
    Completed,
    Delete,
    TextInProgress,
    TextCompleted,
}

// used to display text widget
impl IpgDrawMode {
    pub fn string(&self) -> String {
        match &self {
            IpgDrawMode::DrawAll => "DrawAll".to_string(),
            IpgDrawMode::New => "New".to_string(),
            IpgDrawMode::Edit => "Edit".to_string(),
            IpgDrawMode::Rotate => "Rotate".to_string(),
        }
    }

    pub fn to_enum(s: String) -> Self {
        match s.as_str() {
            "DrawAll" | "drawall" | "Drawall" => IpgDrawMode::DrawAll,
            "Edit" | "edit" => IpgDrawMode::Edit,
            "New" | "new" => IpgDrawMode::New,
            "Rotate" | "rotate" => IpgDrawMode::Rotate,
            _ => IpgDrawMode::DrawAll,
        }
    }
}

#[derive(Debug)]
pub struct IpgCanvasState {
    cache: canvas::Cache,
    pub curves: HashMap<Id, IpgWidget>,
    pub draw_mode: IpgDrawMode,
    pub edit_widget_id: Option<Id>,
    pub escape_pressed: bool,
    pub selected_widget: Option<IpgCanvasWidget>,
    pub selected_draw_color: Color,
    pub selected_canvas_color: Color,
    pub selected_poly_points: usize,
    pub selected_poly_points_str: String,
    pub selected_step_degrees: f32,
    pub selected_width: f32,
    pub selected_width_str: String,
    pub timer_event_enabled: bool,
    pub timer_duration: u64,
    pub elapsed_time: u64,
    pub blink: bool,
    pub show_draw_color_picker: bool,
    pub show_canvas_color_picker: bool,
}

impl Default for IpgCanvasState {
    fn default() -> Self {
        Self { 
            cache: canvas::Cache::default(),
            curves: HashMap::new(),
            draw_mode: IpgDrawMode::DrawAll,
            edit_widget_id: None,
            escape_pressed: false,
            selected_widget: None,
            selected_draw_color: Color::from_rgb(0.961, 0.871, 0.702),
            selected_canvas_color: Color::from_rgb(0.0, 0.502, 0.502),
            selected_poly_points: 3,
            selected_poly_points_str: String::new(),
            selected_step_degrees: 6.0,
            selected_width: 2.0,
            selected_width_str: String::new(),
            timer_event_enabled: false,
            timer_duration: 750,
            elapsed_time: 0,
            blink: false,
            show_draw_color_picker: false,
            show_canvas_color_picker: false,
        }
    }
}

impl IpgCanvasState {
    pub fn view<'a>(&'a self, curves: &'a HashMap<Id, IpgWidget>) -> Element<'a, IpgWidget> {
        Canvas::new(DrawPending {
            state: self,
            curves,
        })
        .width(Fill)
        .height(Fill)
        .into()
    }

    pub fn request_redraw(&mut self) {
        self.cache.clear();
    }
}

struct DrawPending<'a> {
    state: &'a IpgCanvasState,
    curves: &'a HashMap<Id, IpgWidget>,
}

impl<'a> canvas::Program<IpgWidget> for DrawPending<'a> {
    type State = Option<Pending>;

    fn update(
        &self,
        program_state: &mut Self::State,
        event: Event,
        bounds: iced::Rectangle,
        cursor: mouse::Cursor,
    ) -> (event::Status, Option<IpgWidget>) {
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
                            IpgDrawMode::DrawAll => {
                                None
                            },
                            IpgDrawMode::Edit => {
                                match program_state {
                                    // edit consists of 3 clicks
                                    // 1 - find closest widget
                                    // 2 - find closest point
                                    // 3 - finish
                                    None => {
                                        let widget_opt = find_closest_widget(self.curves, cursor_position);
                                        
                                        let selected_widget = 
                                            match widget_opt {
                                                Some(w) => w,
                                                None => return (event::Status::Ignored, None),
                                            };

                                        // set draw_mode to indicate being edited
                                        let widget = 
                                            set_widget_mode_or_status(
                                                selected_widget, 
                                                Some(IpgDrawMode::Edit),
                                                Some(IpgDrawStatus::Inprogress),
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
                                            find_closest_point_index(&widget, cursor_position);
                                        let widget = widget.clone();
                                        *program_state = Some(Pending::EditThird {
                                            widget: widget.clone(),
                                            edit_point_index: point_index,
                                            edit_mid_point: mid_point,
                                            edit_other_point: other_point,
                                        });
                                        // If a text widget, need to send back the curve so that the
                                        // cursor can be seen.  No access to the time event during pending.
                                        if self.state.selected_widget == Some(IpgCanvasWidget::Text) {
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

                                        let new_widget: IpgWidget = 
                                                update_edited_widget(
                                                    widget.clone(), 
                                                    cursor_position, 
                                                    *edit_point_index, 
                                                    *edit_mid_point,
                                                    *edit_other_point,
                                                    IpgDrawStatus::Completed,
                                                );
                                        
                                        *program_state = None;
                                        Some(new_widget)
                                    },
                                    _ => None,
                                }
                            },
                            IpgDrawMode::New => {
                                match program_state {
                                    // First mouse click sets the state of the first Pending point
                                    // return a none since no Curve yet
                                    None => {
                                        // in case the poly points, color, and width have changed since 
                                        // the widget selected
                                        if self.state.selected_widget.is_none() {
                                            return (event::Status::Ignored, None)
                                        }
                                        let selected_widget = 
                                            add_new_widget(
                                                self.state.selected_widget.unwrap(), 
                                                self.state.selected_poly_points,
                                                self.state.selected_draw_color,
                                                self.state.selected_width,
                                                self.state.draw_mode,
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
                                        if self.state.selected_widget == Some(IpgCanvasWidget::Text) {
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
                                        
                                        // if completed, we return the CanvasWidget and set the state to none
                                        // if not, then this is repeated until completed.
                                        if completed {
                                            *program_state = None;
                                            complete_new_widget(widget, cursor_position)
                                        } else {
                                            *program_state = Some(Pending::New {
                                                widget: widget.clone(),
                                            });
                                            
                                            let some_text = 
                                                if self.state.selected_widget.unwrap() == IpgCanvasWidget::Text {
                                                    Some(widget)
                                                } else {
                                                    None
                                                };
                                            some_text
                                        }
                                    },
                                    _ => None,
                                }
                            },
                            IpgDrawMode::Rotate => {
                                match program_state {
                                    // rotation consists of 2 clicks
                                    // 1 - find closest widget
                                    //  - move mouse wheel
                                    // 2 - click to finish
                                    None => {
                                        let widget_opt = find_closest_widget(self.curves, cursor_position);
                                        
                                        let selected_widget = 
                                            match widget_opt {
                                                Some(w) => w,
                                                None => return (event::Status::Ignored, None),
                                            };
                                        
                                        // The widget needs to be in DrawAll initially, 
                                        // in order to display it in pending
                                        // However, the below return of the draw curve 
                                        // the widget need to ne in the rotate method.
                                        let widget = 
                                            set_widget_mode_or_status(
                                                selected_widget, 
                                                Some(IpgDrawMode::Rotate),
                                                Some(IpgDrawStatus::Inprogress),
                                            );
                                        
                                        *program_state = Some(Pending::Rotate {
                                            widget: widget.clone(),
                                            step_degrees: self.state.selected_step_degrees,
                                            degrees: get_widget_degrees(&widget),
                                        });

                                        // returning CanvasWidget so that the curve
                                        // being editied will not show after the refresh
                                        // The pending process will show the curve
                                        // until its finsihed.
                                        Some(widget)
                                    },
                                    // After the final rotation completed
                                    Some(Pending::Rotate {
                                        widget,
                                        step_degrees: _,
                                        degrees: _,
                                    }) => {
                                        let (rotated_widget, _) = 
                                            update_rotated_widget(
                                                widget,
                                                0.0,
                                                Some(IpgDrawStatus::Completed),
                                            );

                                        *program_state = None;

                                        Some(rotated_widget)
                                    },
                                    _ => None,
                                }
                            },
                        }
                    },
                    mouse::Event::WheelScrolled { delta} => {
                        match self.state.draw_mode {
                            IpgDrawMode::Rotate => {
                                match program_state {
                                    None => None,
                                    Some(Pending::Rotate { 
                                        widget,
                                        step_degrees,
                                        degrees: _,  
                                    }) => {
                                        let delta = match delta {
                                            mouse::ScrollDelta::Lines { x:_, y } => y,
                                            mouse::ScrollDelta::Pixels { x:_, y } => y,
                                        };

                                        // Setting the widget draw_mode at each mouse wheel
                                        // since it was set to DrawAll initially.
                                        // Otherwise needed to have another pending type
                                        // and duplicate a lot of code.  Had to clone anyway.
                                        let (widget, degrees) = 
                                            update_rotated_widget(
                                                widget, 
                                                *step_degrees*delta,
                                                None, 
                                            );
                                        
                                        *program_state = Some(Pending::Rotate{
                                            widget,
                                            step_degrees: *step_degrees,
                                            degrees: Some(degrees),
                                        });
                                        None
                                    },
                                    _ => None,
                                }
                            },
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
                                                set_widget_mode_or_status(
                                                    widget.clone(), 
                                                    None, 
                                                    Some(IpgDrawStatus::Delete),
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
        let content =
            self.state.cache.draw(renderer, bounds.size(), 
                            |frame| {

                let background = Path::rectangle(Point::ORIGIN, frame.size());
                frame.fill(&background, self.state.selected_canvas_color);

                DrawCurve::draw_all(self.curves, self.state.blink, frame, theme);

                frame.stroke(
                    &Path::rectangle(Point::ORIGIN, frame.size()),
                    Stroke::default()
                        .with_width(2.0)
                        .with_color(theme.palette().text),
                );
            });

        if let Some(pending) = state {
            vec![content, pending.draw(renderer, theme, bounds, cursor)]
        } else {
            vec![content]
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
    fn draw_all(curves: &HashMap<Id, IpgWidget>, blink: bool, frame: &mut Frame, _theme: &Theme) {
        // This draw only occurs at the completion of the 
        // widget(update occurs) and cache is cleared
        for (_id, widget) in curves.iter() {
            // if first click, skip the curve to be edited so that it 
            // will not be seen until the second click.  Otherwise is shows
            // during editing because there is no way to refresh
            // The pending routine will diplay the curve

            let (path, color, width) = 
                match &widget {
                    IpgWidget::Arc(arc) => {
                        // skip if being editied or rotated
                        if arc.status == IpgDrawStatus::Inprogress {
                            (None, None, None)
                        } else {
                            let (path, _, _,_,_,_) = 
                                build_arc_path(
                                arc, 
                                arc.draw_mode, 
                                None, 
                                None, 
                                false,
                            );

                            (Some(path), Some(arc.color), Some(arc.width))
                        }
                    },
                    IpgWidget::Bezier(bz) => {
                        // skip if being editied or rotated
                        if bz.status == IpgDrawStatus::Inprogress {
                            (None, None, None)
                        } else {
                            let (path, _, _) = 
                                build_bezier_path(
                                bz, 
                                bz.draw_mode, 
                                None, 
                                None, 
                                false,
                                None,
                            );

                            (Some(path), Some(bz.color), Some(bz.width))
                        }
                    },
                    IpgWidget::Circle(cir) => {
                        // skip if being editied or rotated
                        if cir.status== IpgDrawStatus::Inprogress {
                            (None, None, None)
                        } else {
                            let path = 
                                build_circle_path(
                                    cir, 
                                    cir.draw_mode,
                                    None, 
                                    None, 
                                    false
                                );
                            (Some(path), Some(cir.color), Some(cir.width))
                        }
                    },
                    IpgWidget::Ellipse(ell) => {
                        // skip if being editied or rotated
                        if ell.status == IpgDrawStatus::Inprogress {
                            (None, None, None)
                        } else {
                            let path = 
                                build_ellipse_path(
                                    ell, 
                                    ell.draw_mode,
                                    None, 
                                    None, 
                                    false,
                                );
                            (Some(path), Some(ell.color), Some(ell.width))
                        }
                    },
                    IpgWidget::Line(line) => {
                        // skip if being editied or rotated
                        if line.status == IpgDrawStatus::Inprogress {
                            (None, None, None)
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

                            (Some(path), Some(line.color), Some(line.width))
                        }
                    },
                    IpgWidget::PolyLine(pl) => {
                        // skip if being editied or rotated
                        if pl.status == IpgDrawStatus::Inprogress {
                            (None, None, None)
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
                            (Some(path), Some(pl.color), Some(pl.width))
                        }
                    },
                    IpgWidget::Polygon(pg) => {
                        // skip if being editied or rotated
                        if pg.status == IpgDrawStatus::Inprogress {
                            (None, None, None)
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
                                
                            (Some(path), Some(pg.color), Some(pg.width))
                        }
                    }
                    IpgWidget::RightTriangle(tr) => {
                        // skip if being editied or rotated
                        if tr.status == IpgDrawStatus::Inprogress {
                            (None, None, None)
                        } else {
                            let (path, _, _, _) = 
                                build_right_triangle_path(
                                    tr, 
                                    tr.draw_mode, 
                                    None, 
                                    None, 
                                    false,
                                    false,
                                    None,
                                );
                                
                            (Some(path), Some(tr.color), Some(tr.width))
                        }
                    },
                    IpgWidget::FreeHand(fh) => {
                        // skip if being editied or rotated
                        if fh.status == IpgDrawStatus::Inprogress {
                            (None, None, None)
                        } else {
                            let path = 
                                build_free_hand_path(
                                    fh, 
                                    fh.draw_mode, 
                                    None, 
                                    None, 
                                );
                            (Some(path), Some(fh.color), Some(fh.width))
                        }
                    },
                    IpgWidget::Text(txt) => {
                        if txt.draw_mode != IpgDrawMode::Edit {
                            let frame_path = 
                                build_text_path (
                                    txt,
                                    txt.draw_mode,
                                    None,
                                    false,
                                    0.0,
                                    blink,
                                );
                            frame.fill_text(frame_path.0);
                            
                            (frame_path.1, Some(txt.color), Some(1.0))
                        } else {
                            (None, None, None)
                        }
                    }
                    IpgWidget::None => (None, None, None),
                };

                if let Some(path) = path { frame.stroke(
                    &path,
                    Stroke::default()
                    .with_width(width.unwrap())
                    .with_color(color.unwrap()),
                    ) }
        }

    }
}



#[allow(dead_code)]
#[derive(Debug, Clone)]
enum Pending {
    New {
        widget: IpgWidget, 
    },
    EditSecond {
        widget: IpgWidget, 
        },
    EditThird {
        widget: IpgWidget, 
        edit_point_index: Option<usize>,
        edit_mid_point: bool,
        edit_other_point: bool,
        },
    Rotate {
        widget: IpgWidget,
        step_degrees: f32,
        degrees: Option<f32>,
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
                        mid_point, 
                        degrees_left,
                        degrees_center,
                        ) = 
                    match widget {
                        IpgWidget::Arc(arc) => {
                            let (path, _, 
                                _, 
                                _, 
                                degrees_left,
                                degrees_center) = 
                                build_arc_path(
                                    arc, 
                                    IpgDrawMode::New, 
                                    Some(cursor),
                                    None,
                                    false,
                                );
                            (path, arc.color, arc.width, Some(arc.points[0]), degrees_left, degrees_center)
                        },
                        IpgWidget::Bezier(bz) => {
                            let (path, degrees, _) = 
                                build_bezier_path(
                                    bz, 
                                    IpgDrawMode::New, 
                                    Some(cursor),
                                    None,
                                    false,
                                    None,
                                );
                                
                            (path, bz.color, bz.width, Some(bz.points[0]), None, Some(degrees))
                        },
                        IpgWidget::Circle(cir) => {
                            let path = 
                                build_circle_path(
                                    cir, 
                                    IpgDrawMode::New, 
                                    Some(cursor),
                                    None,
                                    false,
                                );
                            (path, cir.color, cir.width, None, None, None)
                        },
                        IpgWidget::Ellipse(ell) => {
                            let path = 
                                build_ellipse_path(
                                    ell, 
                                    IpgDrawMode::New, 
                                    Some(cursor),
                                    None,
                                    false,
                                );
                            (path, ell.color, ell.width, Some(ell.points[0]), None, None)
                        }
                        IpgWidget::Line(line) => {
                            let (path, degrees, _) = 
                                build_line_path(
                                    line, 
                                    IpgDrawMode::New, 
                                    Some(cursor),
                                    None,
                                    false,
                                    None,
                                );
                            (path, line.color, line.width, Some(line.points[0]), Some(degrees), None)
                        },
                        IpgWidget::Polygon(pg) => {
                            let (path, degrees, mid_point) = 
                                build_polygon_path(
                                    pg,
                                    IpgDrawMode::New, 
                                    Some(cursor),
                                    false,
                                    false,
                                    None,
                                );
                            
                            (path, pg.color, pg.width, Some(mid_point), Some(degrees), None)
                        },
                        // return points as they are set
                        IpgWidget::PolyLine(pl) => {
                            let (path, degrees, mid_point) = 
                                build_polyline_path(
                                    pl, 
                                    IpgDrawMode::New, 
                                    Some(cursor),
                                    None,
                                    false,
                                    false,
                                    None,
                                );
                            (path, pl.color, pl.width, Some(mid_point), Some(degrees), None)
                        },
                        IpgWidget::RightTriangle(r_tr) => {
                            let (path, degrees, mid_point, _) = 
                                build_right_triangle_path(
                                    r_tr, 
                                    IpgDrawMode::New, 
                                    Some(cursor),
                                    None,
                                    false,
                                    false,
                                    None,
                                );
                            (path, r_tr.color, r_tr.width, Some(mid_point), Some(degrees), None)
                        },
                        IpgWidget::FreeHand(fh) => {
                            let path = 
                                build_free_hand_path(
                                    fh, 
                                    IpgDrawMode::New, 
                                    Some(cursor), 
                                    None,
                                );
                            (path, fh.color, fh.width, None, None, None)
                        }
                        IpgWidget::Text(_txt) => {
                            (Path::new(|_| {}), Color::TRANSPARENT, 0.0, None, None, None)  
                        }
                        IpgWidget::None => {
                            (Path::new(|_| {}), Color::TRANSPARENT, 0.0, None, None, None)
                        }
                    };

                    if degrees_center.is_some() {
                        let degrees = format!("{:.prec$}", degrees_center.unwrap(), prec = 1);
                        let mid_point = mid_point.unwrap();
                        let position_center = Point::new(mid_point.x-10.0, mid_point.y-20.0);
                        frame.fill_text(canvas::Text {
                            position: position_center,
                            color: Color::WHITE,
                            size: 10.0.into(),
                            content: degrees,
                            horizontal_alignment: alignment::Horizontal::Center,
                            vertical_alignment: alignment::Vertical::Center,
                            ..canvas::Text::default()
                        });
                    }
                    if degrees_left.is_some() {
                        let degrees = format!("{:.prec$}", degrees_left.unwrap(), prec = 1);
                        let mid_point = mid_point.unwrap();
                        let position_left = Point::new(mid_point.x-30.0, mid_point.y-10.0);
                        frame.fill_text(canvas::Text {
                            position: position_left,
                            color: Color::WHITE,
                            size: 10.0.into(),
                            content: degrees,
                            horizontal_alignment: alignment::Horizontal::Center,
                            vertical_alignment: alignment::Vertical::Center,
                            ..canvas::Text::default()
                        });
                    }
                    
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
                            IpgWidget::None => {
                                (Path::new(|_| {}), Color::TRANSPARENT, 0.0)
                            },
                            IpgWidget::Arc(arc) => {
                                let (path, _, _, _,_,_) = 
                                build_arc_path(
                                    arc, 
                                    IpgDrawMode::Edit, 
                                    Some(cursor),
                                    None,
                                    false,
                                );

                                (path, arc.color, arc.width)
                            },
                            IpgWidget::Bezier(bz) => {
                                let (path, _, _) = 
                                build_bezier_path(
                                    bz, 
                                    IpgDrawMode::Edit, 
                                    Some(cursor),
                                    None, 
                                    false,
                                    None,
                                );
                           
                                (path, bz.color, bz.width)
                            },
                            IpgWidget::Circle(cir) => {
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
                            IpgWidget::Ellipse(ell) => {
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
                            IpgWidget::Line(line) => {
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
                            IpgWidget::Polygon(pg) => {
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
                            IpgWidget::PolyLine(pl) => {
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
                            IpgWidget::RightTriangle(tr) => {
                                let (path, _, _, _) = 
                                build_right_triangle_path(
                                    tr, 
                                    IpgDrawMode::Edit, 
                                    Some(cursor),
                                    None, 
                                    false,
                                    false,
                                    None,
                                );
                                (path, tr.color, tr.width)
                            },
                            IpgWidget::FreeHand(fh) => {
                                let path = 
                                    build_free_hand_path(
                                        fh, 
                                        IpgDrawMode::Edit, 
                                        Some(cursor),
                                        None, 
                                    );
                                (path, fh.color, fh.width)
                            },
                            IpgWidget::Text(txt) => {
                                let (text, path) = build_text_path (
                                        txt,
                                        IpgDrawMode::Edit,
                                        Some(cursor),
                                        false,
                                        0.0,
                                        false,
                                    );
                                frame.fill_text(text);
                                (path.unwrap(), txt.color, 2.0)
                            }
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
                        mid_point, 
                        degrees_left,
                        degrees_center,
                        ) = match widget {

                        IpgWidget::None => {
                            (Path::new(|_| {}), Color::TRANSPARENT, 0.0, Point::default(), None, None)
                        },
                        IpgWidget::Arc(arc) => {
                            let (path, 
                                mid_point, 
                                _, 
                                _,
                                degrees_left,
                                degrees_center,
                                ) = 
                                build_arc_path(
                                    arc, 
                                    IpgDrawMode::Edit, 
                                    Some(cursor),
                                    *edit_point_index, 
                                    *edit_mid_point,
                                );

                            (path, arc.color, arc.width, mid_point, degrees_left, degrees_center)
                        },
                        IpgWidget::Bezier(bz) => {
                            let (path, degrees, mid_point) = 
                                build_bezier_path(
                                    bz, 
                                    IpgDrawMode::Edit, 
                                    Some(cursor),
                                    *edit_point_index, 
                                    *edit_mid_point,
                                    None,
                                );
                           
                            (path, bz.color, bz.width, mid_point, None, Some(degrees))
                        },
                        IpgWidget::Circle(cir) => {
                            let path = 
                                build_circle_path(
                                    cir, 
                                    IpgDrawMode::Edit, 
                                    Some(cursor),
                                    *edit_point_index, 
                                    *edit_mid_point,
                                );
                            (path, cir.color, cir.width, cir.center, None, None)
                        },
                        IpgWidget::Ellipse(ell) => {
                            let path = 
                                build_ellipse_path(
                                    ell, 
                                    IpgDrawMode::Edit, 
                                    Some(cursor),
                                    *edit_point_index, 
                                    *edit_mid_point,
                                );
                            (path, ell.color, ell.width, ell.center, None, None)
                        },
                        IpgWidget::Line(line) => {
                            let (path, degrees, mid_point) = 
                                build_line_path(
                                    line, 
                                    IpgDrawMode::Edit, 
                                    Some(cursor),
                                    *edit_point_index, 
                                    *edit_mid_point,
                                    None,
                                );
                            
                            (path, line.color, line.width, mid_point, None, Some(degrees))
                        },
                        IpgWidget::Polygon(pg) => {
                            let (path, degrees, mid_point) = 
                                build_polygon_path(
                                    pg, 
                                    IpgDrawMode::Edit, 
                                    Some(cursor), 
                                    *edit_mid_point,
                                    *edit_other_point,
                                    None,
                                );
                            (path, pg.color, pg.width, mid_point, None, Some(degrees))
                        },
                        IpgWidget::PolyLine(pl) => {
                            let (path, degrees, mid_point) = 
                                build_polyline_path(
                                    pl, 
                                    IpgDrawMode::Edit, 
                                    Some(cursor),
                                    *edit_point_index, 
                                    *edit_mid_point,
                                    *edit_other_point,
                                    None,
                                );
                            (path, pl.color, pl.width, mid_point, None, Some(degrees))
                        },
                        IpgWidget::RightTriangle(tr) => {
                            let (path, degrees, mid_point, _) = 
                                build_right_triangle_path(
                                    tr, 
                                    IpgDrawMode::Edit, 
                                    Some(cursor),
                                    *edit_point_index, 
                                    *edit_mid_point,
                                    *edit_other_point,
                                    None,
                                );
                            (path, tr.color, tr.width, mid_point, None, Some(degrees))
                        },
                        IpgWidget::FreeHand(fh) => {
                            let path= 
                                build_free_hand_path(
                                    fh, 
                                    IpgDrawMode::Edit, 
                                    Some(cursor),
                                    *edit_point_index, 
                                );
                            (path, fh.color, fh.width, Point::default(), None, None)
                        },
                        IpgWidget::Text(txt) => {
                            let (text, path) = build_text_path (
                                        txt,
                                        IpgDrawMode::Edit,
                                        Some(cursor),
                                        true,
                                        0.0,
                                        false,
                                    );

                            frame.fill_text(text);
                            (path.unwrap(), Color::TRANSPARENT, 0.0, Point::default(), None, None)
                        }
                    };

                    if degrees_left.is_some() {
                        let degrees = format!("{:.prec$}", degrees_left.unwrap(), prec = 1);
                        let position = Point::new(mid_point.x-30.0, mid_point.y-10.0);
                        frame.fill_text(canvas::Text {
                            position,
                            color: Color::WHITE,
                            size: 10.0.into(),
                            content: degrees,
                            horizontal_alignment: alignment::Horizontal::Center,
                            vertical_alignment: alignment::Vertical::Center,
                            ..canvas::Text::default()
                        });
                    }

                    if degrees_center.is_some() {
                        let degrees = format!("{:.prec$}", degrees_center.unwrap(), prec = 1);
                        let position = Point::new(mid_point.x-10.0, mid_point.y-20.0);
                        frame.fill_text(canvas::Text {
                            position,
                            color: Color::WHITE,
                            size: 10.0.into(),
                            content: degrees,
                            horizontal_alignment: alignment::Horizontal::Center,
                            vertical_alignment: alignment::Vertical::Center,
                            ..canvas::Text::default()
                        });
                    }

                    frame.stroke(
                        &path,
                        Stroke::default()
                            .with_width(width)
                            .with_color(color),
                    );
                },
                
                Pending::Rotate {
                    widget,
                    step_degrees: _,
                    degrees, 
                } => {
                    let (path, 
                        color, 
                        width, 
                        mid_point, 
                        degrees_left,
                        degrees_center,
                    ) = match widget {
                        IpgWidget::Arc(arc) => {
                            let (path, 
                                _, 
                                _, 
                                _, 
                                degrees_left,
                                degrees_center,) = 
                                build_arc_path(
                                    arc, 
                                    arc.draw_mode,
                                    None,
                                    None, 
                                    false,
                                );

                            (path, arc.color, arc.width, arc.mid_point, degrees_left, degrees_center)
                        },
                        IpgWidget::Bezier(bz) => {
                            let (path, pending_degrees, _) = 
                                build_bezier_path(
                                    bz, 
                                    bz.draw_mode,
                                    None,
                                    None, 
                                    false,
                                    *degrees,
                                );
                            (path, bz.color, bz.width, bz.mid_point, None, Some(pending_degrees))
                        },
                        IpgWidget::Circle(cir) => {
                        let path = 
                            build_circle_path(
                                cir, 
                                IpgDrawMode::Rotate, 
                                None,
                                None,
                                false,
                            );
                            (path, cir.color, cir.width, cir.center, None, None)
                        },
                        IpgWidget::Ellipse(ell) => {
                            let path = 
                                build_ellipse_path(
                                    ell, 
                                    IpgDrawMode::Rotate, 
                                    None,
                                    None,
                                    false,
                                );
                                (path, ell.color, ell.width, ell.center, None, Some(to_degrees(&ell.rotation.0)))
                            },
                        IpgWidget::Line(line) => {
                            let (path, pending_degrees, _) = 
                                build_line_path(
                                    line, 
                                    line.draw_mode, 
                                    None,
                                    None,
                                    false,
                                    *degrees,
                                );
                            (path, line.color, line.width, line.mid_point, None, Some(pending_degrees))
                        },
                        IpgWidget::Polygon(pg) => {
                            let (path, pending_degrees, _) = 
                                build_polygon_path(
                                    pg, 
                                    pg.draw_mode, 
                                    None,
                                    false,
                                    false,
                                    *degrees,
                                );
                            (path, pg.color, pg.width, pg.mid_point, None, Some(pending_degrees))
                        },
                        IpgWidget::PolyLine(pl) => {
                            let (path, pending_degrees, _) = 
                                build_polyline_path(
                                    pl, 
                                    IpgDrawMode::Rotate, 
                                    None,
                                    None,
                                    false,
                                    false,
                                    *degrees,
                                );
                            (path, pl.color, pl.width, pl.mid_point, None, Some(pending_degrees))
                        },
                        IpgWidget::RightTriangle(tr) => {
                            let (path, pending_degrees, _, _) = 
                                build_right_triangle_path(
                                    tr, 
                                    IpgDrawMode::Rotate, 
                                    None,
                                    None,
                                    false,
                                    false,
                                    *degrees,
                                );
                            (path, tr.color, tr.width, tr.mid_point, None, Some(pending_degrees))
                        },
                        IpgWidget::FreeHand(fh) => {
                            let path = 
                                build_free_hand_path(
                                    fh, 
                                    IpgDrawMode::Rotate, 
                                    None,
                                    None,
                                );
                            (path, fh.color, fh.width, Point::default(), None, None)
                        },
                        IpgWidget::Text(_txt) => {
                            (Path::new(|_| {}), Color::TRANSPARENT, 0.0, Point::default(), None, None)
                        }
                        IpgWidget::None => {
                            (Path::new(|_| {}), Color::TRANSPARENT, 0.0, Point::default(), None, None)
                        }
                    };

                    if degrees_left.is_some() {
                        let degrees = format!("{:.prec$}", degrees_left.unwrap(), prec = 1);
                        let position = Point::new(mid_point.x-30.0, mid_point.y-10.0);
                        frame.fill_text(canvas::Text {
                            position,
                            color: Color::WHITE,
                            size: 10.0.into(),
                            content: degrees,
                            horizontal_alignment: alignment::Horizontal::Center,
                            vertical_alignment: alignment::Vertical::Center,
                            ..canvas::Text::default()
                        });
                    }

                    if degrees_center.is_some() {
                        let degrees = format!("{:.prec$}", degrees_center.unwrap(), prec = 1);
                        let position = Point::new(mid_point.x-10.0, mid_point.y-20.0);
                        frame.fill_text(canvas::Text {
                            position,
                            color: Color::WHITE,
                            size: 10.0.into(),
                            content: degrees,
                            horizontal_alignment: alignment::Horizontal::Center,
                            vertical_alignment: alignment::Vertical::Center,
                            ..canvas::Text::default()
                        });
                    }

                    frame.stroke(
                        &path,
                        Stroke::default()
                            .with_width(width)
                            .with_color(color),
                    );
                },
            };
        }
        
        frame.into_geometry()
    }
}


