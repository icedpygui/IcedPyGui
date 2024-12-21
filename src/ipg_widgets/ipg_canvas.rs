//! ipg_canvas

#![allow(dead_code)]
use std::fs;
use std::path::Path;

use iced::widget::container;
use iced::{Color, Element, Length};

use crate::app::Message;
use crate::canvas::draw_canvas::{IpgCanvasState, IpgCanvasWidget, IpgDrawMode, IpgDrawStatus};
use crate::canvas::geometries::{get_draw_mode_and_status, get_widget_id, set_widget_mode_or_status, Widget};
use crate::canvas::import_export::{convert_to_export, import_widgets, save};


#[derive(Debug, Clone)]
pub struct IpgCanvas {
    pub id: usize,
    pub width: Length,
    pub height: Length,
    pub show: bool,
}

impl IpgCanvas {
    pub fn new(
        id: usize,
        width: Length,
        height: Length,
        show: bool,
    ) -> Self {
        Self {
            id,
            width,
            height,
            show, 
        }
    }
}

pub fn construct_canvas(canvas_state: &IpgCanvasState) -> Element<Message>{
    let draw: Element<CanvasMessage> =  
            container(canvas_state.view(&canvas_state.curves)
                .map(CanvasMessage::WidgetDraw)).into();
    draw.map(move |message| Message::Canvas(message))
}


#[derive(Debug, Clone)]
pub enum CanvasMessage {
    WidgetDraw(IpgCanvasWidget),
    Clear,
    ModeSelected(String),
    RadioSelected(Widget),
    Load,
    Save,
    PolyInput(String),
    WidthInput(String),
    Tick,
    SelectDrawColor,
    SubmitDrawColor(Color),
    CancelDrawColor,
    SelectCanvasColor,
    SubmitCanvasColor(Color),
    CancelCanvasColor,
}

pub fn canvas_callback(canvas_message: CanvasMessage, 
                        mut canvas_state: &mut IpgCanvasState,
                        ) {
    match canvas_message {
            CanvasMessage::WidgetDraw(mut widget) => {
                let (draw_mode, draw_status) = get_draw_mode_and_status(&widget);
                // Since the text widget may have a blinking cursor, the only way to use a timer
                // is to use the main subscription one at this time, canvas lacks a time event.
                // Therefore, the pending has to return the curve also at each change so that
                // the curves can be updated.  The subscrition clears the cache at each tick.
                // A bit more efficient way would be to just have a text cache and just clear it.
                // Probable incorporated in a near future revision.
                match draw_status {
                    IpgDrawStatus::Completed => {
                        widget = set_widget_mode_or_status(widget, Some(IpgDrawMode::DrawAll), None);
                    },
                    IpgDrawStatus::Delete => {
                        let id = get_widget_id(&widget);
                        canvas_state.curves.remove(&id);
                    },
                    IpgDrawStatus::TextInProgress => {
                        // Since the text always returns a new curve or updated curve,
                        // a check for the first return is need to see if a text is present. 
                        let id = get_widget_id(&widget);
                        let present = canvas_state.curves.get(&id);
                        if present.is_none() {
                            canvas_state.curves.insert(id, widget.clone());
                        } else {
                            canvas_state.curves.entry(id).and_modify(|k| *k= widget.clone());
                        }
                    },
                    _ => (),
                }
                if draw_status != IpgDrawStatus::TextInProgress {
                    if draw_mode == IpgDrawMode::New {
                        let id = get_widget_id(&widget);
                        let widget = set_widget_mode_or_status(widget.clone(), Some(IpgDrawMode::DrawAll), Some(IpgDrawStatus::Completed));
                        canvas_state.curves.insert(id, widget);
                    } else {
                        // if not new must be in edit or rotate mode so modify.
                        let id = get_widget_id(&widget);
                        canvas_state.edit_widget_id = Some(id.clone());
                        canvas_state.curves.entry(id).and_modify(|k| *k= widget);
                    }
                }

                canvas_state.request_redraw();
            }
            CanvasMessage::Clear => {
                canvas_state.curves.clear();
                canvas_state = IpgCanvasState::default();
            }
            CanvasMessage::ModeSelected(mode) => {
                let mode = IpgDrawMode::to_enum(mode.clone());
                match mode {
                    IpgDrawMode::DrawAll => {
                        canvas_state.draw_mode = IpgDrawMode::DrawAll;
                    },
                    IpgDrawMode::Edit => {
                        if canvas_state.curves.is_empty() {
                            return
                        }
                        canvas_state.draw_mode = IpgDrawMode::Edit;
                    },
                    IpgDrawMode::New => {
                        canvas_state.draw_mode = IpgDrawMode::New;
                    },
                    IpgDrawMode::Rotate => {
                        canvas_state.draw_mode = IpgDrawMode::Rotate;
                    },
                }
                canvas_state.request_redraw();
            },
            CanvasMessage::RadioSelected(choice) => {
                // Have to  make sure and only use the timer event during
                // the text only.
                canvas_state.timer_event_enabled = false;
                match choice {
                    Widget::Arc => {
                        canvas_state.selected_radio_widget = Some(Widget::Arc);
                    },
                    Widget::Bezier => {
                        canvas_state.selected_radio_widget = Some(Widget::Bezier);
                    },
                    Widget::Circle => {
                        canvas_state.selected_radio_widget = Some(Widget::Circle);
                    },
                    Widget::Ellipse => {
                        canvas_state.selected_radio_widget = Some(Widget::Ellipse);
                    },
                    Widget::Line => {
                        canvas_state.selected_radio_widget = Some(Widget::Line);
                    },
                    Widget::PolyLine => {
                        canvas_state.selected_radio_widget = Some(Widget::PolyLine);
                    },
                    Widget::Polygon => {
                        canvas_state.selected_radio_widget = Some(Widget::Polygon);
                    },
                    Widget::RightTriangle => {
                        canvas_state.selected_radio_widget = Some(Widget::RightTriangle);
                    },
                    Widget::FreeHand => {
                        canvas_state.selected_radio_widget = Some(Widget::FreeHand);
                    }
                    Widget::Text => {
                        canvas_state.selected_radio_widget = Some(Widget::Text);
                        canvas_state.timer_event_enabled = true;
                    }
                    Widget::None => (),
                } 
            },
            CanvasMessage::Tick => {
                canvas_state.elapsed_time += canvas_state.timer_duration;
                canvas_state.blink = !canvas_state.blink;
                canvas_state.request_redraw();
            },
            CanvasMessage::Load => {
                let path = Path::new("../../icedpygui/resources/canvas_data.json");
                let data = fs::read_to_string(path).expect("Unable to read file");
                let widgets = serde_json::from_str(&data).expect("Unable to parse");
                canvas_state.curves = import_widgets(widgets);
                canvas_state.request_redraw();
            },
            CanvasMessage::Save => {
                let path = Path::new("../../icedpygui/resources/canvas_data.json");
                let widgets = convert_to_export(&canvas_state.curves);
                let _ = save(path, &widgets);
            },
            CanvasMessage::PolyInput(input) => {
                // little error checking
                canvas_state.selected_poly_points_str = input.clone();
                if !input.is_empty() {
                    canvas_state.selected_poly_points = input.parse().unwrap();
                } else {
                    canvas_state.selected_poly_points = 4; //default
                }
            },
            CanvasMessage::WidthInput(input) => {
                // little error checking
                canvas_state.selected_width_str = input.clone();
                if !input.is_empty() {
                    canvas_state.selected_width = input.parse().unwrap();
                } else {
                    canvas_state.selected_width = 2.0; //default
                }
            },
            CanvasMessage::SelectDrawColor => {
                canvas_state.show_draw_color_picker = true;
            },
            CanvasMessage::SubmitDrawColor(color) => {
                canvas_state.selected_draw_color = color;
                canvas_state.show_draw_color_picker = false;
            },
            CanvasMessage::CancelDrawColor => {
                canvas_state.show_draw_color_picker = false;
            },
            CanvasMessage::SelectCanvasColor => {
                canvas_state.show_canvas_color_picker = true;
            },
            CanvasMessage::SubmitCanvasColor(color) => {
                canvas_state.selected_canvas_color = color;
                canvas_state.show_canvas_color_picker = false;
                canvas_state.request_redraw();
            },
            CanvasMessage::CancelCanvasColor => {
                canvas_state.show_canvas_color_picker = false;
            },
        }
}
