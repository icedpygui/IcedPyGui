//! Main IPG app.
#![allow(unused)]
use std::collections::HashMap;


use iced::window::Position;
use iced::{font, window, Size};
use iced::event::{Event, Status};
use iced::{Element, Point, Subscription, Task, Theme};
use iced::executor;
use iced::widget::{focus_next, horizontal_space, Canvas, Column};
use iced::time;
use iced::Color;
use once_cell::sync::Lazy;


use crate::{access_update_items, access_window_actions, ipg_widgets, match_container, match_widget, IpgState};
use ipg_widgets::ipg_button::{BTNMessage, construct_button, button_callback};
use ipg_widgets::ipg_canvas::{canvas_callback, construct_canvas, CanvasMessage, IpgBuildCanvas};
use ipg_widgets::ipg_card::{CardMessage, construct_card, card_callback};
use ipg_widgets::ipg_checkbox::{CHKMessage, construct_checkbox, checkbox_callback};
use ipg_widgets::ipg_column::construct_column;
use ipg_widgets::ipg_container::construct_container;
use ipg_widgets::ipg_date_picker::{DPMessage, construct_date_picker, date_picker_update};
use ipg_widgets::ipg_enums::{IpgContainers, IpgWidgets};
use ipg_widgets::ipg_events::{handle_window_closing, process_keyboard_events, process_mouse_events, process_touch_events, process_window_event};
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

use iced::widget::{scrollable, Space};

#[derive(Debug, Clone)]
pub enum Message {
    Button(usize, BTNMessage),
    Canvas,
    Card(usize, CardMessage),
    CheckBox(usize, CHKMessage),
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
    Tick,
    Timer(usize, TIMMessage),
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

#[derive(Default)]
pub struct Flags {
    pub keyboard_event_enabled: (usize, bool),
    pub mouse_event_enabled: (usize, bool),
    pub timer_event_enabled: (usize, bool),
    pub window_event_enabled: (usize, bool),
    pub touch_event_enabled: (usize, bool),
    pub timer_duration: u64,
}


pub struct App {
    state: IpgState,
    timer_duration: u64,
    keyboard_event_enabled: (usize, bool),
    mouse_event_enabled: (usize, bool),
    timer_event_enabled: (usize, bool),
    window_event_enabled: (usize, bool),
    touch_event_enabled: (usize, bool),

    counter: i32, 
}


impl App {
    
    pub fn new(flags: Flags) -> (Self, Task<Message>) {
        let mut state = IpgState::new();
        clone_state(&mut state);
        
        let mut open = add_windows(&mut state);
        open.push(font::load(include_bytes!("./graphics/fonts/bootstrap-icons.ttf").as_slice()).map(Message::FontLoaded));

        (
            Self {
                state,
                timer_duration: flags.timer_duration,
                keyboard_event_enabled: flags.keyboard_event_enabled,
                mouse_event_enabled: flags.mouse_event_enabled,
                timer_event_enabled: flags.timer_event_enabled,
                window_event_enabled: flags.window_event_enabled,
                touch_event_enabled: flags.touch_event_enabled,

                counter: 0,
            },
            
            Task::batch(open),
        )
    }

    pub fn title(&self, iced_window_id: window::Id) -> String {
        
        let ipg_window_id = match self.state.windows_iced_ipg_ids.get(&iced_window_id) {
            Some(id) => id.clone(),
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
                process_updates(&mut self.state);
                get_tasks(&mut self.state)
            },
            Message::Canvas => {
                canvas_callback();
                get_tasks(&mut self.state)
            },
            Message::Card(id, message) => {
                card_callback(&mut self.state, id, message);
                process_updates(&mut self.state);
                 Task::none()
            },
            Message::CheckBox(id, message) => {
                checkbox_callback(&mut self.state, id, message);
                process_updates(&mut self.state);
                get_tasks(&mut self.state)
            },
            Message::DatePicker(id, message) => {
                date_picker_update(&mut self.state, id, message);
                process_updates(&mut self.state);
                Task::none()
            },
            Message::EventKeyboard(event) => {
                process_keyboard_events(event, self.keyboard_event_enabled.0);
                Task::none()
            },
            Message::EventMouse(event) => {
                process_mouse_events(event, self.mouse_event_enabled.0);
                Task::none()
            },
            Message::EventWindow((window_id, event)) => {
                // if all windows are closed, returns a true else false.
                let is_empty = process_window_event(&mut self.state, event, 
                    self.window_event_enabled.0, 
                    self.window_event_enabled.1, 
                    window_id);

                if is_empty {
                    iced::exit()
                } else {
                    // check for any other window changes
                    get_tasks(&mut self.state)
                }
            },
            Message::EventTouch(event) => {
                process_touch_events(event, self.touch_event_enabled.0);
                Task::none()
            },
            Message::Image(id, message) => {
                image_callback(&mut self.state, id, message);
                process_updates(&mut self.state);
                Task::none()
            },
            Message::Menu(id, message) => {
                menu_callback(id, message);
                process_updates(&mut self.state);
                Task::none()
            },
            Message::Modal(id, message) => {
                modal_callback(id, message);
                process_updates(&mut self.state);
                focus_next()
            },
            Message::MouseAreaOnPress(id) => {
                mousearea_callback(&mut self.state, id, "on_press".to_string());
                Task::none()
            },
            Message::MouseAreaOnRelease(id) => {
                mousearea_callback(&mut self.state, id, "on_release".to_string());
                Task::none()
            },
            Message::MouseAreaOnRightPress(id) => {
                mousearea_callback(&mut self.state, id, "on_right_press".to_string());
                Task::none()
            },
            Message::MouseAreaOnRightRelease(id) => {
                mousearea_callback(&mut self.state, id, "on_right_release".to_string());
                Task::none()
            },
            Message::MouseAreaOnMiddlePress(id) => {
                mousearea_callback(&mut self.state, id, "on_middle_press".to_string());
                Task::none()
            },
            Message::MouseAreaOnMiddleRelease(id) => {
                mousearea_callback(&mut self.state, id, "on_middle_release".to_string());
                Task::none()
            },
            Message::MouseAreaOnEnter(id) => {
                mousearea_callback(&mut self.state, id, "on_enter".to_string());
                Task::none()
            },
            Message::MouseAreaOnMove(point, id) => {
                mousearea_callback_point(&mut self.state, id, point, "on_move".to_string());
                Task::none()
            },
            Message::MouseAreaOnExit(id) => {
                mousearea_callback(&mut self.state, id, "on_exit".to_string());
                Task::none()
            },
            Message::OpaqueOnPress(id) => {
                opaque_callback(id, "on_press".to_string());
                Task::none()
            },
            Message::PickList(id, message) => {
                pick_list_callback(&mut self.state, id, message);
                process_updates(&mut self.state);
                Task::none()
            },
            Message::Radio(id, message) => {
                radio_callback(id, message);
                process_updates(&mut self.state);
                Task::none()
            },
            Message::Scrolled(vp, id) => {
                scrollable_callback(&mut self.state, id, vp);
                process_updates(&mut self.state);
                Task::none()
            },
            Message::SelectableText(id, message) => {
                selectable_text_callback(id, message);
                process_updates(&mut self.state);
                Task::none()
            },
            Message::Slider(id, message) => {
                slider_callback(id, message);
                process_updates(&mut self.state);
                Task::none()
            },
            Message::Svg(id, message) => {
                svg_callback(&mut self.state, id, message);
                process_updates(&mut self.state);
                Task::none()
            },
            Message::Table(id, message) => {
                table_callback(id, message);
                process_updates(&mut self.state);
                Task::none()
            },
            Message::TextInput(id, message) => {
                text_input_callback(id, message);
                process_updates(&mut self.state);
                Task::none()
            },
            Message::Tick => {
                tick_callback(&mut self.state, self.timer_event_enabled.0);
                Task::none()
            }
            Message::Timer(id, message) => {
                match message {
                    TIMMessage::OnStart => {
                        self.timer_event_enabled.0 = id;
                        self.timer_event_enabled.1 = true;
                    },
                    TIMMessage::OnStop => self.timer_event_enabled.1 = false,
                }
                self.timer_duration = timer_callback(&mut self.state, id, message);
                Task::none()
            },
            Message::Toggler(id, message) => {
                toggle_callback(id, message);
                process_updates(&mut self.state);
                get_tasks(&mut self.state)
            },
            Message::WindowOpened(_id, _position, size) => {
                Task::none()
            },
        }
        
    }

    pub fn view(&self, window_id: window::Id) -> Element<self::Message> {

        let (_visible, debug, theme) = get_window_values(window_id, &self.state);
        
        // if !visible { 
        //     return horizontal_space().into();
        // }
 
        let content = create_content(window_id, &self.state);
        
        if debug {
            let color = match_theme_with_color(theme);
                content.explain(color)  
        } else {
            content
        }

    }

    pub fn subscription(&self) -> Subscription<Message> {

        let mut subscriptions = vec![];
        
        if self.timer_event_enabled.1 {
            subscriptions.push(time::every(iced::time::Duration::from_millis(self.timer_duration)).map(|_| Message::Tick));
        } 
        
        if self.keyboard_event_enabled.1 {
            subscriptions.push(iced::event::listen().map(Message::EventKeyboard));
        }

        if self.mouse_event_enabled.1 {
            subscriptions.push(iced::event::listen().map(Message::EventMouse));
        }
        // window event is always enabled, since we are using iced::daemon, the windows
        // closing need to be followed and iced exited when the last window is closed.
        // The closing is the only event monitored unless the user enables the window events.
        let w_event = window::events()
            .map(|(id, event)| Message::EventWindow((id, iced::Event::Window(event))));

        subscriptions.push(w_event);
        
        
        if subscriptions.len() > 0 {
            Subscription::batch(subscriptions)
        }
        else {
            Subscription::none()
        }
    }

    pub fn theme(&self, iced_window_id: window::Id) -> Theme {

        let ipg_window_id_opt = self.state.windows_iced_ipg_ids.get(&iced_window_id);
        let ipg_window_id = match ipg_window_id_opt {
            Some(id) => id.clone(),
            None => panic!("App: theme, Unable to find ipg_window_id based on iced_window_id {:?}.", iced_window_id)
        };
        
        let window_opt = self.state.containers.get(&ipg_window_id);
        let ipg_window = get_window_container(window_opt);

        ipg_window.theme.clone()
    }

    pub fn scale_factor(&self, iced_window_id: window::Id) -> f64 {

        let ipg_window_id_opt = self.state.windows_iced_ipg_ids.get(&iced_window_id);
        let ipg_window_id = match ipg_window_id_opt {
            Some(id) => id.clone(),
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
        Some(id) => id.clone(),
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

fn get_tasks(state: &mut IpgState) -> Task<Message> {
    
    let mut actions = vec![];

    for (ipg_id, mode) in state.mode.iter() {
        let iced_id = find_key_for_value(*ipg_id);
        actions.push(window::change_mode(iced_id, *mode));
        let is_empty = handle_window_closing(iced_id, *mode);
        if is_empty {
            actions.push(iced::exit());
        }
    }
    state.mode = vec![];

    for id in state.decorations.iter() {
        let iced_id = find_key_for_value(*id);
        actions.push(window::toggle_decorations(iced_id))
    }
    state.decorations = vec![];

    for (id, width, height) in state.resize.iter() {
        let iced_id = find_key_for_value(*id);
        let size = Size::new(*width, *height);
        actions.push(window::resize(iced_id, size))
    }
    state.resize = vec![];

    for (id, x, y) in state.position.iter() {
        let iced_id = find_key_for_value(*id);
        let point = Point::new(*x, *y);
        actions.push(window::move_to(iced_id, point))
    }
    state.position = vec![];

    for (ipg_id, level) in state.level.iter() {
        let iced_id = find_key_for_value(*ipg_id);
        actions.push(window::change_level(iced_id, *level));
    }
    state.level = vec![];

    if actions.is_empty() {
        actions.push(Task::none());
    }
    Task::batch(actions)

}


// Central method to get the structures stored in the mutex and then the children 
fn create_content(iced_id: window::Id, state: &IpgState) -> Element<'static, Message> {
    
    let ipg_window_id_opt = state.windows_iced_ipg_ids.get(&iced_id);

    let ipg_window_id = match ipg_window_id_opt {
        Some(id) => id,
        None => panic!("App::create_content: Unable to find ipg_window_id with iced_id {:?}.", iced_id),
    };

    // First we find the unique containers in the window
    let unique_parent_ids = get_unique_parents(state.container_ids.get(&ipg_window_id));

    // The unique parent containers are combined with all the children ids held in a vec.
    let all_parent_ids = get_combine_parents_and_children(
                            &unique_parent_ids, state.ids.get(&ipg_window_id));

    let content = get_children(&all_parent_ids,
                                                                &0, 
                                                                &unique_parent_ids,
                                                                state);
    content.into()
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
        
        parent_child_ids.push(ParentChildIds { parent_id: par_id.clone(), child_ids })
    }

    parent_child_ids
}

fn get_children(parents: &Vec<ParentChildIds>, 
                index: &usize, 
                parent_ids: &Vec<usize>, 
                state: &IpgState,
                ) -> Element<'static, Message> 
{

    let mut content: Vec<Element<'static, Message>> = vec![];

    for child in &parents[*index].child_ids {
        if parent_ids.contains(&child) {
            let index = parents.iter().position(|r| &r.parent_id == child).unwrap();
            content.push(get_children(&parents, &index, parent_ids, state));
        } else {
            content.push(get_widget(state, child));
        }
    }
    let id = &parents[*index].parent_id;

    if id != &0 {
        get_container(state, id, content)
    } else {
        Column::with_children(content).into()  // the final container
    }
}


fn get_container(state: &IpgState, id: &usize, content: Vec<Element<'static, Message>>) -> Element<'static, Message> {

    let container_opt: Option<&IpgContainers> = state.containers.get(id);

    match container_opt 
    {
        Some(container) => 
            match container {
                IpgContainers::IpgCanvas(can) => {
                    let canvas = can.clone();
                    return construct_canvas(canvas)
                },
                IpgContainers::IpgColumn(col) => {
                    return construct_column(col, content) 
                },
                IpgContainers::IpgContainer(con) => {
                    if content.len() > 1 {
                        panic!("A container can have only one widget, place your multiple widgets into a column or row")
                    }
                    return construct_container(con.clone(), content)
                },
                IpgContainers::IpgModal(modal) => {
                    return construct_modal(modal.clone(), content)
                }
                IpgContainers::IpgMouseArea(m_area) => {
                    return construct_mousearea(m_area.clone(), content)
                },
                IpgContainers::IpgOpaque(op) => {
                    return construct_opaque(op.clone(), content)
                }
                IpgContainers::IpgTable(table) => {
                    let tbl = table.clone();
                    let style = match table.button_fill_style_id.clone() {
                        Some(id) => state.button_style.get(&id),
                        None => None,
                    };
                    if style.is_some() {
                        let st = style.unwrap().clone();
                        return construct_table(tbl, content, Some(st));
                    } else {
                        return construct_table(tbl, content, None);
                    }
                },
                IpgContainers::IpgRow(row) => {
                    return construct_row(row, content)
                },
                IpgContainers::IpgScrollable(scroll) => {
                    return construct_scrollable(scroll.clone(), content)
                },
                IpgContainers::IpgStack(stk) => {
                    return construct_stack(stk.clone(), content)
                }
                IpgContainers::IpgToolTip(tool) => {
                    return construct_tool_tip(tool, content)
                },
                IpgContainers::IpgWindow(_wnd) => {
                    return construct_window(content)
                }
            },
        
        None => panic!("Container not found in fn get_container id={}", id),        
    }
    
}

fn get_widget(state: &IpgState, id: &usize) -> Element<'static, Message> {

    let widget_opt: Option<&IpgWidgets> = state.widgets.get(id);

    match widget_opt 
    {
        Some(widget) => 
            match widget {      
                IpgWidgets::IpgButton(btn) => {
                    let style = match btn.style_id.clone() {
                        Some(id) => state.button_style.get(&id),
                        None => None,
                    };
                    if style.is_some() {
                        let st = style.unwrap().clone();
                        return construct_button(btn.clone(), Some(st));
                    } else {
                        return construct_button(btn.clone(), None);
                    }
                },
                IpgWidgets::IpgCard(crd) => {
                    let card = crd.clone();
                  return construct_card(card)
                },
                IpgWidgets::IpgCheckBox(chk) => {
                    let style = match chk.style_id.clone() {
                        Some(id) => state.checkbox_style.get(&id),
                        None => None,
                    };
                    if style.is_some() {
                        let st = style.unwrap().clone();
                        return construct_checkbox(chk.clone(), Some(st));
                    } else {
                        return construct_checkbox(chk.clone(), None);
                    }
                },
                IpgWidgets::IpgImage(img) => {
                    let image = img.clone();
                    return construct_image(image)
                },
                IpgWidgets::IpgMenu(mn) => {
                    let menu = mn.clone();
;                    return construct_menu(menu)
                },
                IpgWidgets::IpgDatePicker(dp) => {
                    let style = match dp.button_style_id.clone() {
                        Some(id) => state.button_style.get(&id),
                        None => None,
                    };
                    if style.is_some() {
                        let st = style.unwrap().clone();
                        return construct_date_picker(dp.clone(), Some(st));
                    } else {
                        return construct_date_picker(dp.clone(), None);
                    }
                },
                IpgWidgets::IpgPickList(pick) => {
                    let style = match pick.style_id.clone() {
                        Some(id) => state.pick_list_style.get(&id),
                        None => None,
                    };
                    if style.is_some() {
                        let st = style.unwrap().clone();
                        return construct_picklist(pick.clone(), Some(st));
                    } else {
                        return construct_picklist(pick.clone(), None);
                    }
                },
                IpgWidgets::IpgProgressBar(bar) => {
                    let style = match bar.style_id.clone() {
                        Some(id) => state.progress_bar_style.get(&id),
                        None => None,
                    };
                    if style.is_some() {
                        let st = style.unwrap().clone();
                        return construct_progress_bar(bar.clone(), Some(st));
                    } else {
                        return construct_progress_bar(bar.clone(), None);
                    }
                }
                IpgWidgets::IpgSelectableText(sltxt) => {
                    let s_txt = sltxt.clone();
                    return construct_selectable_text(s_txt)
                },
                IpgWidgets::IpgRadio(radio) => {
                    let rad = radio.clone();
                    return construct_radio(rad) 
                },
                IpgWidgets::IpgRule(rule) => {
                    let rul = rule.clone();
                    return construct_rule(rul) 
                },
                IpgWidgets::IpgSlider(slider) => {
                    let sld = slider.clone();
                    return construct_slider(sld)
                },
                IpgWidgets::IpgSpace(sp) => {
                    return construct_space(sp)
                },
                IpgWidgets::IpgSvg(i_svg) => {
                    let svg = i_svg.clone();
                    return construct_svg(svg)
                },
                IpgWidgets::IpgText(text) => {
                    let txt = text.clone();
                    return construct_text(txt)
                },
                IpgWidgets::IpgTextInput(input) => {
                    let t_input = input.clone();
                    return construct_text_input(t_input)           
                },
                IpgWidgets::IpgTimer(tim) => {
                    let tm = tim.clone();
                    return construct_timer(tm);
                },
                IpgWidgets::IpgToggler(tog) => {
                    let tg = tog.clone();
                    return construct_toggler(tg)           
                },
            },
        None => panic!("App: Widgets not found in fn get_widget id={}", id)
    }
}

fn match_theme_with_color(theme: Theme) -> Color {

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

fn process_updates(state: &mut IpgState) {
    
    let mut update = access_update_items();

    for (id, (item, value)) in update.items.clone().into_iter() {
        let widget = state.widgets.get_mut(&id);

        if widget.is_some() {
            match_widget(widget.unwrap(), item, value);
        } else {
            match state.containers.get_mut(&id) {
                Some(cnt) => {
                    match_container(cnt, item, value);
                    
                },
                None => panic!("Item_update: Widget, Container, or Window with id {id} not found.")
            }
        }
    }
    drop(update);
}

fn clone_state(state: &mut IpgState) {
    let mut mut_state = access_state();
    state.ids = mut_state.ids.to_owned();
    state.containers = mut_state.containers.to_owned();
    state.container_ids = mut_state.container_ids.to_owned();
    state.container_wnd_str_ids = mut_state.container_wnd_str_ids.to_owned();
    state.container_str_ids = mut_state.container_str_ids.to_owned();
    state.container_window_usize_ids = mut_state.container_window_usize_ids.to_owned();
    state.widgets = mut_state.widgets.to_owned();
    state.widget_container_ids = mut_state.widget_container_ids.to_owned();
    state.windows_iced_ipg_ids = mut_state.windows_iced_ipg_ids.to_owned();
    state.windows_str_ids = mut_state.windows_str_ids.to_owned();
    state.windows = mut_state.windows.to_owned();
    state.window_debug = mut_state.window_debug.to_owned();
    state.window_theme = mut_state.window_theme.to_owned();
    state.window_mode = mut_state.window_mode.to_owned();
    state.container_style = mut_state.container_style.to_owned();
    state.button_style = mut_state.button_style.to_owned();
    state.checkbox_style = mut_state.checkbox_style.to_owned();
    state.menu_bar_style = mut_state.menu_bar_style.to_owned();
    state.menu_style = mut_state.menu_style.to_owned();
    state.menu_separator_style = mut_state.menu_separator_style.to_owned();
    state.opaque_style = mut_state.opaque_style.to_owned();
    state.pick_list_style = mut_state.pick_list_style.to_owned();
    state.progress_bar_style = mut_state.progress_bar_style.to_owned();
    state.radio_style = mut_state.radio_style.to_owned();
    state.rule_style = mut_state.rule_style.to_owned();
    state.slider_style = mut_state.slider_style.to_owned();
    state.text_input_style = mut_state.text_input_style.to_owned();
    state.toggler_style = mut_state.toggler_style.to_owned();
    state.scrollable_style = mut_state.scrollable_style.to_owned();

    mut_state.ids = Lazy::new(||HashMap::new());
    mut_state.last_id = 0;
    mut_state.containers = Lazy::new(||HashMap::new());
    mut_state.container_ids = Lazy::new(||HashMap::new());
    mut_state.container_str_ids = Lazy::new(||HashMap::new());
    mut_state.container_wnd_str_ids = Lazy::new(||HashMap::new());
    mut_state.container_window_usize_ids = Lazy::new(||HashMap::new());
    mut_state.widgets = Lazy::new(||HashMap::new());
    mut_state.widget_container_ids = Lazy::new(||HashMap::new());
    mut_state.windows = vec![];
    mut_state.windows_iced_ipg_ids = Lazy::new(||HashMap::new());
    mut_state.windows_str_ids = Lazy::new(||HashMap::new());
    mut_state.window_debug = Lazy::new(||HashMap::new());
    mut_state.window_theme = Lazy::new(||HashMap::new());
    mut_state.window_mode = Lazy::new(||HashMap::new());
    mut_state.container_style = Lazy::new(||HashMap::new());
    mut_state.button_style = Lazy::new(||HashMap::new());
    mut_state.checkbox_style = Lazy::new(||HashMap::new());
    mut_state.menu_bar_style = Lazy::new(||HashMap::new());
    mut_state.menu_style = Lazy::new(||HashMap::new());
    mut_state.menu_separator_style = Lazy::new(||HashMap::new());
    mut_state.opaque_style = Lazy::new(||HashMap::new());
    mut_state.pick_list_style = Lazy::new(||HashMap::new());
    mut_state.progress_bar_style = Lazy::new(||HashMap::new());
    mut_state.radio_style = Lazy::new(||HashMap::new());
    mut_state.rule_style = Lazy::new(||HashMap::new());
    mut_state.slider_style = Lazy::new(||HashMap::new());
    mut_state.text_input_style = Lazy::new(||HashMap::new());
    mut_state.toggler_style = Lazy::new(||HashMap::new());
    mut_state.scrollable_style = Lazy::new(||HashMap::new());

    drop(mut_state);
}
