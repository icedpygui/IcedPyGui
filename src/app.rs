//! Main IPG app.
#![allow(unused)]
#![allow(clippy::map_clone)]
use std::collections::HashMap;


use iced::widget::container::Id;
use iced::window::Position;
use iced::{font, window, Size};
use iced::event::{Event, Status};
use iced::{Element, Point, Subscription, Task, Theme};
use iced::widget::{scrollable, Space};
use iced::executor;
use iced::widget::{focus_next, horizontal_space, Canvas, Column};
use iced::time;
use iced::Color;
use once_cell::sync::Lazy;


use crate::canvas::draw_canvas::IpgCanvasState;
use crate::ipg_widgets::ipg_canvas::match_canvas_widget;
use crate::ipg_widgets::ipg_color_picker::{color_picker_callback, construct_color_picker, ColPikMessage};
use crate::ipg_widgets::ipg_timer_canvas::{canvas_tick_callback, canvas_timer_callback, construct_canvas_timer, CanvasTimerMessage};
use crate::{access_canvas_state, access_canvas_update_items, access_update_items, access_window_actions, ipg_widgets, match_container, match_widget, IpgState};
use ipg_widgets::ipg_button::{BTNMessage, construct_button, button_callback};
use ipg_widgets::ipg_canvas::{canvas_callback, construct_canvas, CanvasMessage};
use ipg_widgets::ipg_card::{CardMessage, construct_card, card_callback};
use ipg_widgets::ipg_checkbox::{CHKMessage, construct_checkbox, checkbox_callback};
use ipg_widgets::ipg_column::construct_column;
use ipg_widgets::ipg_container::construct_container;
use ipg_widgets::ipg_date_picker::{DPMessage, construct_date_picker, date_picker_update};
use ipg_widgets::ipg_enums::{IpgContainers, IpgWidgets};
use ipg_widgets::ipg_events::{IpgKeyBoardEvent, handle_window_closing, process_keyboard_events, 
    process_mouse_events, process_touch_events, process_window_event};
use ipg_widgets::helpers::find_key_for_value;
use ipg_widgets::ipg_image::{ImageMessage, construct_image, image_callback};
use ipg_widgets::ipg_menu::{MenuMessage, construct_menu, menu_callback};
use ipg_widgets::ipg_modal::{construct_modal, modal_callback, ModalMessage};
use ipg_widgets::ipg_mousearea::{mousearea_callback, mousearea_callback_point, construct_mousearea};
use ipg_widgets::ipg_opaque::{construct_opaque, opaque_callback};
use ipg_widgets::ipg_pick_list::{PLMessage, construct_picklist, pick_list_callback};
use ipg_widgets::ipg_progress_bar::construct_progress_bar;
use ipg_widgets::ipg_radio::{RDMessage, construct_radio, radio_callback};
use ipg_widgets::ipg_row::construct_row;
use ipg_widgets::ipg_rule::construct_rule;
use ipg_widgets::ipg_scrollable::{construct_scrollable, scrollable_callback};
use ipg_widgets::ipg_selectable_text::{SLTXTMessage, construct_selectable_text, selectable_text_callback};
use ipg_widgets::ipg_slider::{SLMessage, construct_slider, slider_callback};
use ipg_widgets::ipg_space::construct_space;
use ipg_widgets::ipg_stack::construct_stack;
use ipg_widgets::ipg_svg::{SvgMessage, construct_svg, svg_callback};
use ipg_widgets::ipg_table::{construct_table, TableMessage, table_callback};
use ipg_widgets::ipg_text::construct_text;
use ipg_widgets::ipg_text_input::{TIMessage, construct_text_input, text_input_callback};
use ipg_widgets::ipg_timer::{construct_timer, timer_callback, TIMMessage, tick_callback};
use ipg_widgets::ipg_toggle::{construct_toggler, toggle_callback, TOGMessage};
use ipg_widgets::ipg_tool_tip::construct_tool_tip;
use ipg_widgets::ipg_window::{WndMessage, IpgWindow, add_windows, construct_window};
use ipg_widgets::ipg_window::IpgWindowMode;
use crate::{access_state, IpgIds};



#[derive(Debug, Clone)]
pub enum Message {
    Button(usize, BTNMessage),
    Canvas(CanvasMessage),
    Card(usize, CardMessage),
    CheckBox(usize, CHKMessage),
    ColorPicker(usize, ColPikMessage),
    DatePicker(usize, DPMessage),
    EventKeyboard(Event),
    EventMouse(Event),
    EventWindow((window::Id, Event)),
    EventTouch(Event),
    Image(usize, ImageMessage),
    Menu(usize, MenuMessage),
    Modal(usize, ModalMessage),
    PickList(usize, PLMessage),
    Radio(usize, RDMessage),
    Scrolled(scrollable::Viewport, usize),
    SelectableText(usize, SLTXTMessage),
    Slider(usize, SLMessage),
    Svg(usize, SvgMessage),
    Table(usize, TableMessage),
    TextInput(usize, TIMessage),
    Toggler(usize, TOGMessage),
    CanvasTextBlink,
    Tick,
    CanvasTick,
    Timer(usize, TIMMessage),
    CanvasTimer(usize, CanvasTimerMessage),
    FontLoaded(Result<(), font::Error>),
    WindowOpened(window::Id, Option<Point>, Size),

    MouseAreaOnPress(usize),
    MouseAreaOnRelease(usize),
    MouseAreaOnRightPress(usize),
    MouseAreaOnRightRelease(usize),
    MouseAreaOnMiddlePress(usize),
    MouseAreaOnMiddleRelease(usize),
    MouseAreaOnEnter(usize),
    MouseAreaOnMove(Point, usize),
    MouseAreaOnExit(usize),
    OpaqueOnPress(usize),
}


pub struct App {
    state: IpgState,
    canvas_state: IpgCanvasState,
}

impl App {
    
    pub fn new() -> (Self, Task<Message>) {
        let mut state = IpgState::new();
        clone_state(&mut state);

        let mut canvas_state = IpgCanvasState::default();
        clone_canvas_state(&mut canvas_state, state.last_id);
        
        let mut open = add_windows(&mut state);
        open.push(font::load(include_bytes!("./graphics/fonts/bootstrap-icons.ttf").as_slice()).map(Message::FontLoaded));

        (
            Self {
                state,
                canvas_state,
            },
            
            Task::batch(open),
        )
    }

    pub fn title(&self, iced_window_id: window::Id) -> String {
        
        let ipg_window_id = match self.state.windows_iced_ipg_ids.get(&iced_window_id) {
            Some(id) => *id,
            None => panic!("App: title, Unable to find ipg_window_id based on iced_window_id {:?}.", iced_window_id)
        };
        
        let window_opt = self.state.containers.get(&ipg_window_id);
        let ipg_window = get_window_container(window_opt);

        ipg_window.title.clone()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::FontLoaded(_) => {
                Task::none()
            },
            Message::Button(id, message) => {
                button_callback(&mut self.state, id, message);
                process_updates(&mut self.state, &mut self.canvas_state);
                get_tasks(&mut self.state)
            },
            Message::Canvas(canvas_message) => {
                canvas_callback(canvas_message, &mut self.state, &mut self.canvas_state);
                self.canvas_state.last_id = self.state.last_id;
                process_updates(&mut self.state, &mut self.canvas_state);
                self.state.last_id = self.canvas_state.last_id;
                get_tasks(&mut self.state)
            },
            Message::Card(id, message) => {
                card_callback(&mut self.state, id, message);
                process_updates(&mut self.state, &mut self.canvas_state);
                Task::none()
            },
            Message::CheckBox(id, message) => {
                checkbox_callback(&mut self.state, id, message);
                process_updates(&mut self.state, &mut self.canvas_state);
                get_tasks(&mut self.state)
            },
            Message::ColorPicker(id, message ) => {
                color_picker_callback(&mut self.state, id, message);
                process_updates(&mut self.state, &mut self.canvas_state);
                Task::none()
            }
            Message::DatePicker(id, message) => {
                date_picker_update(&mut self.state, id, message);
                process_updates(&mut self.state, &mut self.canvas_state);
                Task::none()
            },
            Message::EventKeyboard(event) => {
                process_keyboard_events(event, self.state.keyboard_event_id_enabled.0);
                process_updates(&mut self.state, &mut self.canvas_state);
                Task::none()
            },
            Message::EventMouse(event) => {
                process_mouse_events(event, self.state.mouse_event_id_enabled.0);
                process_updates(&mut self.state, &mut self.canvas_state);
                Task::none()
            },
            Message::EventWindow((window_id, event)) => {
                process_window_event(&mut self.state, event, window_id);
                if self.state.windows_opened == 0 {
                    iced::exit()
                } else {
                    
                    // check for any other window changes
                    get_tasks(&mut self.state)
                }
            },
            Message::EventTouch(event) => {
                process_touch_events(event, self.state.touch_event_id_enabled.0);
                process_updates(&mut self.state, &mut self.canvas_state);
                Task::none()
            },
            Message::Image(id, message) => {
                image_callback(&mut self.state, id, message);
                process_updates(&mut self.state, &mut self.canvas_state);
                Task::none()
            },
            Message::Menu(id, message) => {
                menu_callback(&mut self.state, id, message);
                process_updates(&mut self.state, &mut self.canvas_state);
                Task::none()
            },
            Message::Modal(id, message) => {
                modal_callback(&mut self.state, id, message);
                process_updates(&mut self.state, &mut self.canvas_state);
                focus_next()
            },
            Message::MouseAreaOnPress(id) => {
                mousearea_callback(&mut self.state, id, "on_press".to_string());
                process_updates(&mut self.state, &mut self.canvas_state);
                Task::none()
            },
            Message::MouseAreaOnRelease(id) => {
                mousearea_callback(&mut self.state, id, "on_release".to_string());
                process_updates(&mut self.state, &mut self.canvas_state);
                Task::none()
            },
            Message::MouseAreaOnRightPress(id) => {
                mousearea_callback(&mut self.state, id, "on_right_press".to_string());
                process_updates(&mut self.state, &mut self.canvas_state);
                Task::none()
            },
            Message::MouseAreaOnRightRelease(id) => {
                mousearea_callback(&mut self.state, id, "on_right_release".to_string());
                process_updates(&mut self.state, &mut self.canvas_state);
                Task::none()
            },
            Message::MouseAreaOnMiddlePress(id) => {
                mousearea_callback(&mut self.state, id, "on_middle_press".to_string());
                process_updates(&mut self.state, &mut self.canvas_state);
                Task::none()
            },
            Message::MouseAreaOnMiddleRelease(id) => {
                mousearea_callback(&mut self.state, id, "on_middle_release".to_string());
                process_updates(&mut self.state, &mut self.canvas_state);
                Task::none()
            },
            Message::MouseAreaOnEnter(id) => {
                mousearea_callback(&mut self.state, id, "on_enter".to_string());
                process_updates(&mut self.state, &mut self.canvas_state);
                Task::none()
            },
            Message::MouseAreaOnMove(point, id) => {
                mousearea_callback_point(&mut self.state, id, point, "on_move".to_string());
                process_updates(&mut self.state, &mut self.canvas_state);
                Task::none()
            },
            Message::MouseAreaOnExit(id) => {
                mousearea_callback(&mut self.state, id, "on_exit".to_string());
                process_updates(&mut self.state, &mut self.canvas_state);
                Task::none()
            },
            Message::OpaqueOnPress(id) => {
                opaque_callback(&mut self.state, id, "on_press".to_string());
                process_updates(&mut self.state, &mut self.canvas_state);
                Task::none()
            },
            Message::PickList(id, message) => {
                pick_list_callback(&mut self.state, id, message);
                process_updates(&mut self.state, &mut self.canvas_state);
                Task::none()
            },
            Message::Radio(id, message) => {
                radio_callback(&mut self.state, id, message);
                process_updates(&mut self.state, &mut self.canvas_state);
                Task::none()
            },
            Message::Scrolled(vp, id) => {
                scrollable_callback(&mut self.state, id, vp);
                process_updates(&mut self.state, &mut self.canvas_state);
                Task::none()
            },
            Message::SelectableText(id, message) => {
                selectable_text_callback(&mut self.state, id, message);
                process_updates(&mut self.state, &mut self.canvas_state);
                Task::none()
            },
            Message::Slider(id, message) => {
                slider_callback(&mut self.state, id, message);
                process_updates(&mut self.state, &mut self.canvas_state);
                Task::none()
            },
            Message::Svg(id, message) => {
                svg_callback(&mut self.state, id, message);
                process_updates(&mut self.state, &mut self.canvas_state);
                Task::none()
            },
            Message::Table(id, message) => {
                table_callback(&mut self.state, id, message);
                process_updates(&mut self.state, &mut self.canvas_state);
                Task::none()
            },
            Message::TextInput(id, message) => {
                text_input_callback(&mut self.state, id, message);
                process_updates(&mut self.state, &mut self.canvas_state);
                Task::none()
            },
            Message::CanvasTextBlink => {
                self.canvas_state.elapsed_time += self.canvas_state.timer_duration;
                self.canvas_state.blink = !self.canvas_state.blink;
                self.canvas_state.request_text_redraw();
                Task::none()
            },
            Message::Tick => {
                tick_callback(&mut self.state);
                process_updates(&mut self.state, &mut self.canvas_state);
                Task::none()
            },
            Message::CanvasTick => {
                canvas_tick_callback(&mut self.state);
                process_canvas_updates(&mut self.canvas_state);
                process_updates(&mut self.state, &mut self.canvas_state); 
                self.canvas_state.request_image_redraw();
                Task::none()
            },
            Message::Timer(id, _) => {
                self.state.timer_event_id_enabled.1 = !self.state.timer_event_id_enabled.1;
                self.state.timer_event_id_enabled.0 = id;
                let started = self.state.timer_event_id_enabled.1;
                self.state.timer_duration = timer_callback(&mut self.state, id, started);
                process_updates(&mut self.state, &mut self.canvas_state);    
                Task::none()
            },
            Message::CanvasTimer(id, message) => {
                self.state.canvas_timer_event_id_enabled.1 = !self.state.canvas_timer_event_id_enabled.1;
                self.state.canvas_timer_event_id_enabled.0 = id;
                let started = self.state.canvas_timer_event_id_enabled.1;
                self.state.canvas_timer_duration = canvas_timer_callback(&mut self.state, id, started);
                process_updates(&mut self.state, &mut self.canvas_state);    
                Task::none()
            },
            Message::Toggler(id, message) => {
                toggle_callback(&mut self.state, id, message);
                process_updates(&mut self.state, &mut self.canvas_state);
                get_tasks(&mut self.state)
            },
            Message::WindowOpened(_id, _position, size) => {
                self.state.windows_opened += 1;
                process_updates(&mut self.state, &mut self.canvas_state);
                Task::none()
            },
        }
        
    }

    pub fn view(&self, window_id: window::Id) -> Element<self::Message> {

        let (_visible, debug, theme) = get_window_values(window_id, &self.state);
 
        let content = 
            create_content(window_id, &self.state, &self.canvas_state);
        
        if debug {
            let color = match_theme_with_debug_color(theme);
                content.explain(color)  
        } else {
            content
        }

    }

    pub fn subscription(&self) -> Subscription<Message> {

        let mut subscriptions = vec![];
        
        if self.state.timer_event_id_enabled.1 {
            subscriptions
            .push(time::every(iced::time::Duration::from_millis(
                self.state.timer_duration)).map(|_| Message::Tick));
        }
        if self.state.canvas_timer_event_id_enabled.1 {
            subscriptions
            .push(time::every(iced::time::Duration::from_millis(
                self.state.canvas_timer_duration)).map(|_| Message::CanvasTick));
        }
        
        if self.state.keyboard_event_id_enabled.1 {
            subscriptions.push(iced::event::listen().map(Message::EventKeyboard));
        }

        if self.state.mouse_event_id_enabled.1 {
            subscriptions.push(iced::event::listen().map(Message::EventMouse));
        }
        if self.canvas_state.timer_event_enabled {
            subscriptions.push(time::every(
                iced::time::Duration::from_millis(
                    self.canvas_state.timer_duration))
                    .map(|_| Message::CanvasTextBlink));
        }
        // window event is always enabled, since we are using iced::daemon, the windows
        // closing need to be followed and iced exited when the last window is closed.
        // The closing is the only event monitored unless the user enables the window events.
        let w_event = window::events()
            .map(|(id, event)| Message::EventWindow((id, iced::Event::Window(event))));

        subscriptions.push(w_event);

        if !subscriptions.is_empty() {
            Subscription::batch(subscriptions)
        }
        else {
            Subscription::none()
        }
    }

    pub fn theme(&self, iced_window_id: window::Id) -> Theme {

        let ipg_window_id_opt = self.state.windows_iced_ipg_ids.get(&iced_window_id);
        let ipg_window_id = match ipg_window_id_opt {
            Some(id) => *id,
            None => panic!("App: theme, Unable to find ipg_window_id based on iced_window_id {:?}.", iced_window_id)
        };
        
        let window_opt = self.state.containers.get(&ipg_window_id);
        let ipg_window = get_window_container(window_opt);

        ipg_window.theme.clone()
    }

    pub fn scale_factor(&self, iced_window_id: window::Id) -> f64 {

        let ipg_window_id_opt = self.state.windows_iced_ipg_ids.get(&iced_window_id);
        let ipg_window_id = match ipg_window_id_opt {
            Some(id) => *id,
            None => panic!("App: title, Unable to find ipg_window_id based on iced_window_id {:?}.", iced_window_id)
        };
        
        let window_opt = self.state.containers.get(&ipg_window_id);
        let ipg_window = get_window_container(window_opt);

        ipg_window.scale_factor
    }

}


fn get_window_values(iced_window_id: window::Id, state: &IpgState) -> (bool, bool, Theme) {

    let ipg_window_id_opt = state.windows_iced_ipg_ids.get(&iced_window_id);
    let ipg_window_id = match ipg_window_id_opt {
        Some(id) => *id,
        None => panic!("App: get_window_values, Unable to find ipg_window_id based on iced_window_id {:?}.", iced_window_id)
    };
    
    let window_opt = state.containers.get(&ipg_window_id);
    let ipg_window = get_window_container(window_opt);

    let vis = match ipg_window.mode {
        ipg_widgets::ipg_window::IpgWindowMode::Windowed => true,
        ipg_widgets::ipg_window::IpgWindowMode::FullScreen => true,
        ipg_widgets::ipg_window::IpgWindowMode::Closed => false,
    };
    let debug = ipg_window.debug;
    let theme = ipg_window.theme.clone();

    (vis, debug, theme)
}

fn get_tasks(ipg_state: &mut IpgState) -> Task<Message> {
    
    let mut state = access_window_actions();

    let mut actions = vec![];

    for (ipg_id, mode) in state.mode.iter() {
        let iced_id = find_key_for_value(ipg_state.windows_iced_ipg_ids.clone(), *ipg_id);
        actions.push(window::change_mode(iced_id, *mode));
        let is_empty = handle_window_closing(ipg_state, iced_id, *mode);
        if is_empty {
            actions.push(iced::exit());
        }
    }
    state.mode = vec![];

    for id in state.decorations.iter() {
        let iced_id = find_key_for_value(ipg_state.windows_iced_ipg_ids.clone(), *id);
        actions.push(window::toggle_decorations(iced_id))
    }
    state.decorations = vec![];

    for (id, width, height) in state.resize.iter() {
        let iced_id = find_key_for_value(ipg_state.windows_iced_ipg_ids.clone(), *id);
        let size = Size::new(*width, *height);
        actions.push(window::resize(iced_id, size))
    }
    state.resize = vec![];

    for (id, x, y) in state.position.iter() {
        let iced_id = find_key_for_value(ipg_state.windows_iced_ipg_ids.clone(), *id);
        let point = Point::new(*x, *y);
        actions.push(window::move_to(iced_id, point))
    }
    state.position = vec![];

    for (ipg_id, level) in state.level.iter() {
        let iced_id = find_key_for_value(ipg_state.windows_iced_ipg_ids.clone(), *ipg_id);
        actions.push(window::change_level(iced_id, *level));
    }
    state.level = vec![];

    drop(state);

    if actions.is_empty() {
        actions.push(Task::none());
    }
    Task::batch(actions)
}


// Central method to get the structures stored in the mutex and then the children 
fn create_content<'a>(iced_id: window::Id, state: &'a IpgState, canvas_state: &'a IpgCanvasState) 
                -> Element<'a, Message> {
    
    let ipg_window_id_opt = state.windows_iced_ipg_ids.get(&iced_id);

    let ipg_window_id = match ipg_window_id_opt {
        Some(id) => id,
        None => panic!("App::create_content: Unable to find ipg_window_id with iced_id {:?}.", iced_id),
    };

    // First we find the unique containers in the window
    let unique_parent_ids = get_unique_parents(state.container_ids.get(ipg_window_id));

    // The unique parent containers are combined with all the children ids held in a vec.
    let all_parent_ids = get_combine_parents_and_children(
                            &unique_parent_ids, state.ids.get(ipg_window_id));

    let content = get_children(&all_parent_ids,
                                                                &0, 
                                                                &unique_parent_ids,
                                                                state,
                                                                canvas_state,);
    content
}

fn get_unique_parents(ids: Option<&Vec<usize>>) -> Vec<usize> {
    // The unique ids are the ids sorted and duplicates removed
    let mut unique_ids: Vec<usize> = match ids {
        Some(ids) => ids.to_vec(),
        None => panic!("Container ids in unique_container_ids not found")
    }; 

    unique_ids.sort();
    unique_ids.dedup();

    unique_ids
}

#[derive(Debug, Clone, PartialEq)]
struct ParentChildIds {
    parent_id: usize,
    child_ids: Vec<usize>,
}

fn get_combine_parents_and_children(parent_ids: &Vec<usize>, ids_opt: Option<&Vec<IpgIds>>) -> Vec<ParentChildIds> {

    let mut parent_child_ids: Vec<ParentChildIds> = vec![];

    let ids = match ids_opt {
        Some(ids) => ids,
        None => panic!("ids in get_and_combine_parents_and_children not found")
    };

    for par_id in parent_ids {

        let mut child_ids: Vec<usize> = vec![];

        for ids in ids {
            if par_id == &ids.parent_uid {
                child_ids.push(ids.id);
            }  
        }
        
        parent_child_ids.push(ParentChildIds { parent_id: *par_id, child_ids })
    }

    parent_child_ids
}

fn get_children<'a>(parents: &Vec<ParentChildIds>, 
                index: &usize, 
                parent_ids: &Vec<usize>, 
                state: &IpgState,
                canvas_state: &'a IpgCanvasState,
                ) -> Element<'a, Message> 
{

    let mut content= vec![];

    for child in parents[*index].child_ids.iter() {
        if parent_ids.contains(child) {
            let index = parents.iter().position(|r| &r.parent_id == child).unwrap();
            content.push(get_children(parents, &index, parent_ids, state, canvas_state));
        } else {
            if get_widget(state, child).is_some() {
                content.push(get_widget(state, child).unwrap());
            }
        }
    }
    let id = &parents[*index].parent_id;

    if id != &0 {
        get_container(state, id, content, canvas_state)
    } else {
        Column::with_children(content).into()  // the final container
    }
}


fn get_container<'a>(state: &IpgState, 
                id: &usize, 
                content: Vec<Element<'a, Message>>,
                canvas_state: &'a IpgCanvasState,
                ) -> Element<'a, Message> {

    let container_opt: Option<&IpgContainers> = state.containers.get(id);

    match container_opt 
    {
        Some(container) => 
            match container {
                IpgContainers::IpgCanvas(canvas) => {
                    construct_canvas(canvas_state)
                },
                IpgContainers::IpgColumn(col) => {
                    construct_column(col, content) 
                },
                IpgContainers::IpgContainer(con) => {
                    if content.len() > 1 {
                        panic!("A container can have only one widget, place your multiple widgets into a column or row")
                    }
                    let style = 
                        match con.style_id.clone() {
                            Some(id) => {
                                state.container_style.get(&id).map(|st| st.clone())
                            },
                            None => None,
                        };

                    construct_container(con.clone(), content, style)
                },
                IpgContainers::IpgModal(modal) => {
                    construct_modal(modal.clone(), content)
                }
                IpgContainers::IpgMouseArea(m_area) => {
                    construct_mousearea(m_area.clone(), content)
                },
                IpgContainers::IpgOpaque(op) => {
                    let style = 
                        match op.style_id.clone() {
                            Some(id) => {
                                state.opaque_style.get(&id).map(|st| st.clone())
                            },
                            None => None,
                        };

                    construct_opaque(op.clone(), content, style)
                }
                IpgContainers::IpgTable(table) => {
                    let button_fill_style = 
                        match table.button_fill_style_id.clone() {
                            Some(id) => {
                                state.button_style.get(&id).map(|st| st.clone())
                            },
                            None => None,
                        };
                    let checkbox_fill_style = 
                        match table.checkbox_fill_style_id.clone() {
                            Some(id) => {
                                state.checkbox_style.get(&id).map(|st| st.clone())
                            },
                            None => None,
                        };
                    let toggler_fill_style = 
                        match table.toggler_fill_style_id.clone() {
                            Some(id) => {
                                state.toggler_style.get(&id).map(|st| st.clone())
                            },
                            None => None,
                        };

                    construct_table(table.clone(), 
                                    content, 
                                    button_fill_style,
                                    checkbox_fill_style,
                                    toggler_fill_style,)
                },
                IpgContainers::IpgRow(row) => {
                    construct_row(row, content)
                },
                IpgContainers::IpgScrollable(scroll) => {
                    let style_opt = match scroll.style_id.clone() {
                        Some(id) => {
                            state.scrollable_style.get(&id).map(|st| st.clone())
                        },
                        None => None,
                    };
                    
                    construct_scrollable(scroll.clone(), content, style_opt)
                },
                IpgContainers::IpgStack(stk) => {
                    construct_stack(stk.clone(), content)
                }
                IpgContainers::IpgToolTip(tool) => {
                    construct_tool_tip(tool, content)
                },
                IpgContainers::IpgWindow(_wnd) => {
                    construct_window(content)
                }
            },
        
        None => panic!("Container not found in fn get_container id={}", id),        
    }
    
}

fn get_widget(state: &IpgState, id: &usize) -> Option<Element<'static, Message>> {

    let widget_opt: Option<&IpgWidgets> = state.widgets.get(id);

    match widget_opt 
    {
        Some(widget) => 
            match widget {      
                IpgWidgets::IpgButton(btn) => {
                    let style_opt = match btn.style_id.clone() {
                        Some(id) => {
                            state.widgets.get(&id).map(|st|st.clone())
                        },
                        None => None,
                    };
                    
                    construct_button(btn.clone(), style_opt)
                },
                IpgWidgets::IpgCard(crd) => {
                    Some(construct_card(crd.clone()))
                },
                IpgWidgets::IpgCheckBox(chk) => {
                    let style_opt = match chk.style_id.clone() {
                        Some(id) => {
                            state.widgets.get(&id).map(|st|st.clone())
                        },
                        None => None,
                    };
                    construct_checkbox(chk.clone(), style_opt)
                },
                IpgWidgets::IpgColorPicker(cp) => {
                    let style_opt = match cp.style_id.clone() {
                        Some(id) => {
                            state.widgets.get(&id).map(|st|st.clone())
                        },
                        None => None,
                    };
                    construct_color_picker(cp.clone(), style_opt)
                }
                IpgWidgets::IpgImage(img) => {
                    let image = img.clone();
                    Some(construct_image(image))
                },
                IpgWidgets::IpgMenu(menu) => {
                    let menu_style = match menu.menu_style_id.clone() {
                        Some(id) => {
                            state.menu_style.get(&id).map(|st|st.clone())
                        },
                        None => None,
                    };
                    let bar_style = match menu.menu_bar_style_id.clone() {
                        Some(id) => {
                            state.menu_bar_style.get(&id).map(|st|st.clone())
                        },
                        None => None,
                    };
                    let sep_style = match menu.separator_item_style_all.clone() {
                        Some(id) => {
                            state.menu_separator_style.get(&id).map(|st|st.clone())
                        },
                        None => None,
                     };
                    Some(construct_menu(menu.clone(), menu_style, bar_style, sep_style))
                },
                IpgWidgets::IpgDatePicker(dp) => {
                    let style_opt = match dp.button_style_id.clone() {
                        Some(id) => {
                            state.button_style.get(&id).map(|st|st.clone())
                        },
                        None => None,
                    };
                    Some(construct_date_picker(dp.clone(), style_opt))
                },
                IpgWidgets::IpgPickList(pick) => {
                    let style_opt = match pick.style_id.clone() {
                        Some(id) => {
                            state.widgets.get(&id).map(|st|st.clone())
                        },
                        None => None,
                    };
                    construct_picklist(pick.clone(), style_opt)
                },
                IpgWidgets::IpgProgressBar(bar) => {
                    let style_opt = match bar.style_id.clone() {
                        Some(id) => {
                            state.widgets.get(&id).map(|st|st.clone())
                        },
                        None => None,
                    };
                    construct_progress_bar(bar.clone(), style_opt)
                },
                IpgWidgets::IpgRadio(radio) => {
                    let style_opt = match radio.style_id.clone() {
                        Some(id) => {
                            state.widgets.get(&id).map(|st|st.clone())
                        },
                        None => None,
                    };
                    construct_radio(radio.clone(), style_opt)
                },
                IpgWidgets::IpgRule(rule) => {
                    let style_opt = match rule.style_id.clone() {
                        Some(id) => {
                            state.widgets.get(&id).map(|st|st.clone())
                        },
                        None => None,
                    };
                    construct_rule(rule.clone(), style_opt)
                },
                IpgWidgets::IpgSelectableText(sltxt) => {
                    Some(construct_selectable_text(sltxt.clone()))
                },
                IpgWidgets::IpgSlider(slider) => {
                    let style_opt = match slider.style_id.clone() {
                        Some(id) => {
                            state.slider_style.get(&id).cloned()
                        },
                        None => None,
                    };
                    Some(construct_slider(slider.clone(), style_opt))
                },
                IpgWidgets::IpgSpace(sp) => {
                    Some(construct_space(sp))
                },
                IpgWidgets::IpgSvg(i_svg) => {
                    let svg = i_svg.clone();
                    Some(construct_svg(svg))
                },
                IpgWidgets::IpgText(text) => {
                    let txt = text.clone();
                    Some(construct_text(txt))
                },
                IpgWidgets::IpgTextInput(input) => {
                    let style_opt = match input.style_id.clone() {
                        Some(id) => {
                            state.text_input_style.get(&id).cloned()
                        },
                        None => None,
                    };
                    Some(construct_text_input(input.clone(), style_opt))       
                },
                IpgWidgets::IpgTimer(timer) => {
                    Some(construct_timer(timer.clone()))
                },
                IpgWidgets::IpgCanvasTimer(ctimer) => {
                    Some(construct_canvas_timer(ctimer.clone()))
                },
                IpgWidgets::IpgToggler(tog) => {
                    let style_opt = match tog.style_id.clone() {
                        Some(id) => {
                            state.toggler_style.get(&id).cloned()
                        },
                        None => None,
                    };
                    Some(construct_toggler(tog.clone(), style_opt))    
                },
                _ => None,

            },
        None => panic!("App: Widgets not found in fn get_widget id={}", id)
    }
}

fn match_theme_with_debug_color(theme: Theme) -> Color {

    match theme {
        Theme::Light => Color::BLACK,
        Theme::Dark => Color::WHITE,
        Theme::Dracula => Color::WHITE,
        Theme::Ferra => Color::WHITE,
        Theme::Nord => Color::WHITE,
        Theme::SolarizedLight => Color::BLACK,
        Theme::SolarizedDark => Color::WHITE,
        Theme::GruvboxLight => Color::BLACK,
        Theme::GruvboxDark => Color::WHITE,
        Theme::CatppuccinLatte => Color::WHITE,
        Theme::CatppuccinFrappe => Color::WHITE,
        Theme::CatppuccinMacchiato => Color::WHITE,
        Theme::CatppuccinMocha => Color::WHITE,
        Theme::TokyoNight => Color::WHITE,
        Theme::TokyoNightStorm => Color::WHITE,
        Theme::TokyoNightLight => Color::BLACK,
        Theme::KanagawaWave => Color::WHITE,
        Theme::KanagawaDragon => Color::WHITE,
        Theme::KanagawaLotus => Color::BLACK,
        Theme::Moonfly => Color::WHITE,
        Theme::Nightfly => Color::WHITE,
        Theme::Oxocarbon => Color::WHITE,
        Theme::Custom(_) => Color::WHITE,
    }
}

fn get_window_container(container_opt: Option<&IpgContainers>) -> &IpgWindow {
    
    let container = match container_opt {
        Some(cnt) => cnt,
        None => panic!("App: get_window_container: Cannot find IpgContainer"),
    };

    match container {
        IpgContainers::IpgWindow(wnd) => {
            wnd
        },
        _ => panic!("get_window: Not a Window")
    }
}

fn process_updates(state: &mut IpgState, canvas_state: &mut IpgCanvasState) {
    
    let mut all_updates = access_update_items();

    let deletes = all_updates.deletes.clone();
    for (window_id, wid) in deletes.iter() {
        let iced_id = match state.windows_str_ids.get(window_id) {
            Some(id) => *id,
            None => panic!("Window_id {} not found in delete_item", window_id)
        };

        let ipg_ids = match state.ids.get_mut(&iced_id) {
            Some(ids) => ids,
            None => panic!("Ids not found for window_id {} in delete_item", window_id)
        };

        let mut index: i32 = -1;

        for (i, ipg_id) in ipg_ids.iter().enumerate() {
            if ipg_id.id == *wid {
                index = i as i32;
                break;
            }
        }

        if index == -1 {
            panic!("item with id {wid} could not be found to delete")
        }

        ipg_ids.remove(index as usize);

        state.widgets.remove(wid);   
    }
    all_updates.deletes = vec![];

    let moves = all_updates.moves.clone();
    for (window_id, 
        widget_id, 
        target_container_str_id, 
        move_after, 
        move_before) in moves.iter() {

        let container_str_id_opt = state.container_str_ids.get(target_container_str_id);

        let container_usize_id = match container_str_id_opt {
            Some(id) => *id,
            None => panic!("move_widget: unable to find the target container id based on the id {}", target_container_str_id)
        };

        let window_id_usize_opt = state.windows_str_ids.get(window_id);

        let window_id_usize = match window_id_usize_opt {
            Some(id) => *id,
            None => panic!("move_widget: unable to find the window_id using the id {}", window_id)
        };

        let window_widget_ids_opt = state.ids.get_mut(&window_id_usize);

        let window_widget_ids = match window_widget_ids_opt {
            Some(ids) => ids,
            None => panic!("move_widget: unable to find widget using window_id {}", window_id)    
        };

        let mut before = false;
        let pos_id = if move_after.is_some() {
            move_after.unwrap()
        } else if move_before.is_some() { 
            before = true;
            move_before.unwrap()
        } else {
            1_000_000
        };

        //  set some large numbers to break early
        let mut found_index = 1_000_000;
        let mut target_index: usize = 1_000_000;

        for (i, ids) in window_widget_ids.iter_mut().enumerate() {
            if ids.id == *widget_id {
                ids.parent_uid = container_usize_id;
                ids.parent_id = target_container_str_id.clone();
                found_index = i;
            }
            if ids.id == pos_id {
                target_index = i
            }
            if found_index != 1_000_000 && (target_index != 1_000_000 || pos_id == 1_000_000) {
                break;
            }
        }
        
        let move_ids = window_widget_ids.remove(found_index);
        
        if pos_id == 1_000_000 {
            window_widget_ids.push(move_ids);
        } else if before {
            window_widget_ids.insert(target_index-1, move_ids);
        } else {
            window_widget_ids.insert(target_index, move_ids);
        }
    }  
    all_updates.moves = vec![];

    let mut updates = all_updates.updates.clone();
    for ((wid, item, value)) in updates.iter() {
        let widget = state.widgets.get_mut(wid);
        if let Some(w) = widget {
            match_widget(w, item.clone(), value.clone());
        } else {
            match state.containers.get_mut(wid) {
                Some(cnt) => {
                    match_container(cnt, item.clone(), value.clone(), canvas_state);
                },
                None => panic!("Item_update: Widget, Container, or Window with id {wid} not found.")
            }
        }  
    }
    all_updates.updates = vec![];
    
}

fn process_canvas_updates(cs: &mut IpgCanvasState) {
    let mut canvas_items = access_canvas_update_items();

    for ((wid, item, value)) in canvas_items.updates.iter() {
        let mut canvas_widget = if cs.curves.get_mut(wid).is_some(){
            cs.curves.get_mut(wid).unwrap()
        } else if cs.image_curves.get_mut(wid).is_some() {
            cs.image_curves.get_mut(wid).unwrap()
        } else if cs.text_curves.get_mut(wid).is_some() {
            cs.text_curves.get_mut(wid).unwrap()
        } else {
           panic!("canvas_item_update: canvas item with id, {} not found", wid);
        };
        match_canvas_widget(canvas_widget, item.clone(), value.clone());
    }
    canvas_items.updates = vec![];

}

fn clone_state(state: &mut IpgState) {
    let mut mutex_state = access_state();
    state.ids = mutex_state.ids.to_owned();
    state.last_id = mutex_state.last_id.to_owned();
    state.containers = mutex_state.containers.to_owned();
    state.container_ids = mutex_state.container_ids.to_owned();
    state.container_wnd_str_ids = mutex_state.container_wnd_str_ids.to_owned();
    state.container_str_ids = mutex_state.container_str_ids.to_owned();
    state.container_window_usize_ids = mutex_state.container_window_usize_ids.to_owned();
    state.widgets = mutex_state.widgets.to_owned();
    state.widget_container_ids = mutex_state.widget_container_ids.to_owned();
    state.windows_iced_ipg_ids = mutex_state.windows_iced_ipg_ids.to_owned();
    state.windows_str_ids = mutex_state.windows_str_ids.to_owned();
    state.windows = mutex_state.windows.to_owned();
    state.window_debug = mutex_state.window_debug.to_owned();
    state.window_theme = mutex_state.window_theme.to_owned();
    state.window_mode = mutex_state.window_mode.to_owned();
    state.container_style = mutex_state.container_style.to_owned();
    state.button_style = mutex_state.button_style.to_owned();
    state.checkbox_style = mutex_state.checkbox_style.to_owned();
    state.color_picker_style = mutex_state.color_picker_style.to_owned();
    state.menu_bar_style = mutex_state.menu_bar_style.to_owned();
    state.menu_style = mutex_state.menu_style.to_owned();
    state.menu_separator_style = mutex_state.menu_separator_style.to_owned();
    state.opaque_style = mutex_state.opaque_style.to_owned();
    state.pick_list_style = mutex_state.pick_list_style.to_owned();
    state.progress_bar_style = mutex_state.progress_bar_style.to_owned();
    state.radio_style = mutex_state.radio_style.to_owned();
    state.rule_style = mutex_state.rule_style.to_owned();
    state.slider_style = mutex_state.slider_style.to_owned();
    state.text_input_style = mutex_state.text_input_style.to_owned();
    state.toggler_style = mutex_state.toggler_style.to_owned();
    state.scrollable_style = mutex_state.scrollable_style.to_owned();
    state.keyboard_event_id_enabled = mutex_state.keyboard_event_id_enabled.to_owned();
    state.mouse_event_id_enabled = mutex_state.mouse_event_id_enabled.to_owned();
    state.timer_event_id_enabled = mutex_state.timer_event_id_enabled.to_owned();
    state.canvas_timer_event_id_enabled = mutex_state.canvas_timer_event_id_enabled.to_owned();
    state.window_event_id_enabled = mutex_state.window_event_id_enabled.to_owned();
    state.touch_event_id_enabled = mutex_state.touch_event_id_enabled.to_owned();
    state.timer_duration = mutex_state.timer_duration.to_owned();
    state.canvas_timer_duration = mutex_state.canvas_timer_duration.to_owned();

    // zeroing out any vecs and hashmaps
    mutex_state.ids = Lazy::new(||HashMap::new());
    mutex_state.containers = Lazy::new(||HashMap::new());
    mutex_state.container_ids = Lazy::new(||HashMap::new());
    mutex_state.container_str_ids = Lazy::new(||HashMap::new());
    mutex_state.container_wnd_str_ids = Lazy::new(||HashMap::new());
    mutex_state.container_window_usize_ids = Lazy::new(||HashMap::new());
    mutex_state.widgets = Lazy::new(||HashMap::new());
    mutex_state.widget_container_ids = Lazy::new(||HashMap::new());
    mutex_state.windows = vec![];
    mutex_state.windows_iced_ipg_ids = Lazy::new(||HashMap::new());
    mutex_state.windows_str_ids = Lazy::new(||HashMap::new());
    mutex_state.window_debug = Lazy::new(||HashMap::new());
    mutex_state.window_theme = Lazy::new(||HashMap::new());
    mutex_state.window_mode = Lazy::new(||HashMap::new());
    mutex_state.container_style = Lazy::new(||HashMap::new());
    mutex_state.button_style = Lazy::new(||HashMap::new());
    mutex_state.checkbox_style = Lazy::new(||HashMap::new());
    mutex_state.color_picker_style = Lazy::new(||HashMap::new());
    mutex_state.menu_bar_style = Lazy::new(||HashMap::new());
    mutex_state.menu_style = Lazy::new(||HashMap::new());
    mutex_state.menu_separator_style = Lazy::new(||HashMap::new());
    mutex_state.opaque_style = Lazy::new(||HashMap::new());
    mutex_state.pick_list_style = Lazy::new(||HashMap::new());
    mutex_state.progress_bar_style = Lazy::new(||HashMap::new());
    mutex_state.radio_style = Lazy::new(||HashMap::new());
    mutex_state.rule_style = Lazy::new(||HashMap::new());
    mutex_state.slider_style = Lazy::new(||HashMap::new());
    mutex_state.text_input_style = Lazy::new(||HashMap::new());
    mutex_state.toggler_style = Lazy::new(||HashMap::new());
    mutex_state.scrollable_style = Lazy::new(||HashMap::new());

    drop(mutex_state);
}

fn clone_canvas_state(canvas_state: &mut IpgCanvasState, last_id: usize) {
    let mut mutex_cs = access_canvas_state();
    canvas_state.curves = mutex_cs.curves.to_owned();
    canvas_state.text_curves = mutex_cs.text_curves.to_owned();
    canvas_state.image_curves = mutex_cs.image_curves.to_owned();
    canvas_state.last_id = last_id;
    canvas_state.width = mutex_cs.width;
    canvas_state.height = mutex_cs.height;
    canvas_state.border_width = mutex_cs.border_width;
    canvas_state.border_color = mutex_cs.border_color;
    canvas_state.selected_canvas_color = mutex_cs.background;

    // zeroing out any vecs and hashmaps
    mutex_cs.curves = Lazy::new(||HashMap::new());
    mutex_cs.text_curves = Lazy::new(||HashMap::new());
    mutex_cs.image_curves = Lazy::new(||HashMap::new());
    drop(mutex_cs);
}