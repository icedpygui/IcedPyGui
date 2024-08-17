//! Main IPG app.
#![allow(unused)]
use std::collections::HashMap;


use iced::{font, window};
use iced::event::Event;
use iced::{Element, Point, Subscription, Task, Theme};
use iced::executor;
use iced::widget::{focus_next, Column};
use iced::time;
use iced::Color;


use crate::ipg_widgets;
use crate::ipg_widgets::ipg_modal::{construct_modal, modal_callback, ModalMessage};
use ipg_widgets::ipg_button::{BTNMessage, construct_button, button_callback};
use ipg_widgets::ipg_card::{CardMessage, construct_card, card_callback};
use ipg_widgets::ipg_checkbox::{CHKMessage, construct_checkbox, checkbox_callback};
use ipg_widgets::ipg_column::construct_column;
use ipg_widgets::ipg_container::construct_container;
use ipg_widgets::ipg_date_picker::{DPMessage, construct_date_picker, date_picker_update};
use ipg_widgets::ipg_enums::{IpgContainers, IpgWidgets};
use ipg_widgets::ipg_events::process_events;
use ipg_widgets::ipg_image::{ImageMessage, construct_image, image_callback};
use ipg_widgets::ipg_menu::{MenuMessage, construct_menu, menu_callback};
use ipg_widgets::ipg_mousearea::{mousearea_callback, mousearea_callback_point, construct_mousearea};
use ipg_widgets::ipg_pick_list::{PLMessage, construct_picklist, pick_list_callback};
use ipg_widgets::ipg_progress_bar::construct_progress_bar;
use ipg_widgets::ipg_radio::{RDMessage, construct_radio, radio_callback};
use ipg_widgets::ipg_row::construct_row;
use ipg_widgets::ipg_rule::construct_rule;
use ipg_widgets::ipg_scrollable::{construct_scrollable, scrollable_callback};
use ipg_widgets::ipg_selectable_text::{SLTXTMessage, construct_selectable_text, selectable_text_callback};
use ipg_widgets::ipg_slider::{SLMessage, construct_slider, slider_callback};
use ipg_widgets::ipg_space::construct_space;
use ipg_widgets::ipg_svg::{SvgMessage, construct_svg, svg_callback};
use ipg_widgets::ipg_table::{contruct_table, TableMessage, table_callback};
use ipg_widgets::ipg_text::construct_text;
use ipg_widgets::ipg_text_input::{TIMessage, construct_text_input, text_input_callback};
use ipg_widgets::ipg_timer::{construct_timer, timer_callback, TIMMessage, tick_callback};
use ipg_widgets::ipg_toggle::{construct_toggler, toggle_callback, TOGMessage};
use ipg_widgets::ipg_tool_tip::construct_tool_tip;
use ipg_widgets::ipg_window::{WndMessage, IpgWindow, add_windows, construct_window, window_callback};
use ipg_widgets::helpers::get_usize_of_id;
use crate::{access_state, IpgIds};


use iced::widget::{scrollable, Space};

#[derive(Debug, Clone)]
pub enum Message {
    EventOccurred(Event),

    Button(usize, BTNMessage),
    Card(usize, CardMessage),
    CheckBox(usize, CHKMessage),
    DatePicker(usize, DPMessage),
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
    Window(WndMessage),
    WindowOpened(window::Id),
    WindowClosed(window::Id),

    MouseAreaOnPress(usize),
    MouseAreaOnRelease(usize),
    MouseAreaOnRightPress(usize),
    MouseAreaOnRightRelease(usize),
    MouseAreaOnMiddlePress(usize),
    MouseAreaOnMiddleRelease(usize),
    MouseAreaOnEnter(usize),
    MouseAreaOnMove(Point, usize),
    MouseAreaOnExit(usize),
}

#[derive(Default)]
pub struct Flags {
    pub keyboard_event_enabled: (usize, bool),
    pub mouse_event_enabled: (usize, bool),
    pub timer_event_enabled: (usize, bool),
    pub window_event_enabled: (usize, bool),
    pub timer_duration: u64,
}


pub struct App {
    windows: HashMap<window::Id, IpgWindow>,
    timer_duration: u64,
    keyboard_event_enabled: (usize, bool),
    mouse_event_enabled: (usize, bool),
    timer_event_enabled: (usize, bool),
    window_event_enabled: (usize, bool), 
}


impl App {
    
    pub fn new(flags: Flags) -> (Self, Task<Message>) {
        
        let (windows, mut open) = add_windows();
        open.push(font::load(include_bytes!("./graphics/fonts/bootstrap-icons.ttf").as_slice()).map(Message::FontLoaded));

        (
            Self {
                windows,
                timer_duration: flags.timer_duration,
                keyboard_event_enabled: flags.keyboard_event_enabled,
                mouse_event_enabled: flags.mouse_event_enabled,
                timer_event_enabled: flags.timer_event_enabled,
                window_event_enabled: flags.window_event_enabled,
            },
            
            Task::batch(open),
        )
    }

    pub fn title(&self, iced_window_id: window::Id) -> String {
        let state = access_state();

        let ipg_window_id_opt = state.windows_iced_ipg_ids.get(&iced_window_id);
        let ipg_window_id = match ipg_window_id_opt {
            Some(id) => id.clone(),
            None => panic!("App: title, Unable to find ipg_window_id based on iced_window_id {:?}.", iced_window_id)
        };
        
        let window_opt = state.containers.get(&ipg_window_id);
        let ipg_window = get_window_container(window_opt);

        let title = ipg_window.title.clone();
        drop(state);
        title
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        
        match message {
            Message::FontLoaded(_) => {
                Task::none()
            },
            Message::EventOccurred(ipg_event) => {
                process_events(ipg_event, self.keyboard_event_enabled,
                                            self.mouse_event_enabled,
                                            self.window_event_enabled,
                                            self.timer_event_enabled);
                Task::none()
            },
            Message::Button(id, message) => {
                button_callback(id, message);
                Task::none()
            },
            Message::Card(id, message) => {
                card_callback(id, message);
                 Task::none()
            },
            Message::CheckBox(id, message) => {
                checkbox_callback(id, message);
                Task::none()
            },
            Message::DatePicker(id, message) => {
                date_picker_update(id, message);
                Task::none()
            },
            Message::Image(id, message) => {
                image_callback(id, message);
                Task::none()
            },
            Message::Menu(id, message) => {
                menu_callback(id, message);
                Task::none()
            },
            Message::Modal(id, message) => {
                modal_callback(id, message);
                focus_next()
            },
            Message::MouseAreaOnPress(id) => {
                mousearea_callback(id, "on_press".to_string());
                Task::none()
            },
            Message::MouseAreaOnRelease(id) => {
                mousearea_callback(id, "on_release".to_string());
                Task::none()
            },
            Message::MouseAreaOnRightPress(id) => {
                mousearea_callback(id, "on_right_press".to_string());
                Task::none()
            },
            Message::MouseAreaOnRightRelease(id) => {
                mousearea_callback(id, "on_right_release".to_string());
                Task::none()
            },
            Message::MouseAreaOnMiddlePress(id) => {
                mousearea_callback(id, "on_middle_press".to_string());
                Task::none()
            },
            Message::MouseAreaOnMiddleRelease(id) => {
                mousearea_callback(id, "on_middle_release".to_string());
                Task::none()
            },
            Message::MouseAreaOnEnter(id) => {
                mousearea_callback(id, "on_enter".to_string());
                Task::none()
            },
            Message::MouseAreaOnMove(point, id) => {
                mousearea_callback_point(id, point, "on_move".to_string());
                Task::none()
            },
            Message::MouseAreaOnExit(id) => {
                mousearea_callback(id, "on_exit".to_string());
                Task::none()
            },
            Message::PickList(id, message) => {
                pick_list_callback(id, message);
                Task::none()
            },
            Message::Radio(id, message) => {
                radio_callback(id, message);
                Task::none()
            },
            Message::Scrolled(vp, id) => {
                scrollable_callback(id, vp);
                Task::none()
            },
            Message::SelectableText(id, message) => {
                selectable_text_callback(id, message);
                Task::none()
            },
            Message::Slider(id, message) => {
                slider_callback(id, message);
                Task::none()
            },
            Message::Svg(id, message) => {
                svg_callback(id, message);
                Task::none()
            },
            Message::Table(id, message) => {
                table_callback(id, message);
                Task::none()
            },
            Message::TextInput(id, message) => {
                text_input_callback(id, message);
                Task::none()
            },
            Message::Tick => {
                tick_callback(self.timer_event_enabled.0);
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
                self.timer_duration = timer_callback(id, message);
                Task::none()
            },
            Message::Toggler(id, message) => {
                toggle_callback(id, message);
                Task::none()
            },
            Message::Window(message) => {
                window_callback(message)
            },
            Message::WindowOpened(_id) => {
                Task::none()
            },
            Message::WindowClosed(id) => {
                self.windows.remove(&id);

                if self.windows.is_empty() {
                    iced::exit()
                } else {
                    Task::none()
                }
            },
        }
        
    }

    pub fn view(&self, window: window::Id) -> Element<'_, self::Message> {
 
        let content = create_content(window);

        let debug = self.windows
                            .get(&window)
                            .map(|window| window.debug.clone())
                            .unwrap();
        
        let theme = self.windows
                                .get(&window)
                                .map(|window| window.theme.clone())
                                .unwrap();
      
        if debug {
            let color = match_theme_with_color(theme);
                content.explain(color)  
        } else {
            content
        }

    }

    pub fn subscription(&self) -> Subscription<Message> {

        let mut subscriptions = vec![];

        subscriptions.push(window::close_events().map(Message::WindowClosed));

        if self.timer_event_enabled.1 {
            subscriptions.push(time::every(iced::time::Duration::from_millis(self.timer_duration)).map(|_| Message::Tick));
        } 
        
        if self.keyboard_event_enabled.1 || self.mouse_event_enabled.1 || self.window_event_enabled.1 {

            subscriptions.push(iced::event::listen().map(Message::EventOccurred));

        }
        
        if subscriptions.len() > 0 {
            Subscription::batch(subscriptions)
        }
        else {
            Subscription::none()
        }
    }

    pub fn theme(&self, window: window::Id) -> Theme {

        self.windows
            .get(&window)
            .map(|window| window.theme.clone())
            .unwrap()
    }

    pub fn scale_factor(&self, iced_window_id: window::Id) -> f64 {

        let state = access_state();

        let ipg_window_id_opt = state.windows_iced_ipg_ids.get(&iced_window_id);
        let ipg_window_id = match ipg_window_id_opt {
            Some(id) => id.clone(),
            None => panic!("App: title, Unable to find ipg_window_id based on iced_window_id {:?}.", iced_window_id)
        };
        
        let window_opt = state.containers.get(&ipg_window_id);
        let ipg_window = get_window_container(window_opt);

        let factor = ipg_window.scale_factor;
        drop(state);
        factor
    
    }

}


// Central method to get the structures stored in the mutex and then the children 
fn create_content(iced_id: window::Id) -> Element<'static, Message> {
    
    let state = access_state();

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
    
    drop(state);

    let content = get_children(&all_parent_ids,
                                                                    &0, 
                                                                    &unique_parent_ids);
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
                ) -> Element<'static, Message> 
{

    let mut content: Vec<Element<'static, Message>> = vec![];

    for child in &parents[*index].child_ids {
        if parent_ids.contains(&child) {
            let index = parents.iter().position(|r| &r.parent_id == child).unwrap();
            content.push(get_children(&parents, &index, parent_ids));
        } else {
            content.push(get_widget(child));
        }
    }
    let id = &parents[*index].parent_id;

    if id != &0 {
        get_container(id, content)
    } else {
        Column::with_children(content).into()  // the final container
    }
}


fn get_container(id: &usize, content: Vec<Element<'static, Message>>) -> Element<'static, Message> {

    let state = access_state();

    let container_opt: Option<&IpgContainers> = state.containers.get(id);

    match container_opt 
    {
        Some(container) => 
            match container {
                IpgContainers::IpgColumn(col) => {
                    return construct_column(col, content) 
                },
                IpgContainers::IpgContainer(con) => {
                    if content.len() > 1 {
                        panic!("A container can have only one widget, place your multiple widgets into  a column or row")
                    }
                    return construct_container(con.clone(), content)
                },
                IpgContainers::IpgModal(modal) => {
                    return construct_modal(modal.clone(), content)
                }
                IpgContainers::IpgMouseArea(m_area) => {
                    return construct_mousearea(m_area.clone(), content)
                },
                IpgContainers::IpgTable(table) => {
                    let tbl = table.clone();
                    drop(state);
                    return contruct_table(tbl, content)
                },
                IpgContainers::IpgRow(row) => {
                    return construct_row(row, content)
                },
                IpgContainers::IpgScrollable(scroll) => {
                    return construct_scrollable(scroll.clone(), content)
                },
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

fn get_widget(id: &usize) -> Element<'static, Message> {

    let state = access_state();

    let widget_opt: Option<&IpgWidgets> = state.widgets.get(id);

    match widget_opt 
    {
        Some(widget) => 
            match widget {      
                IpgWidgets::IpgButton(btn) => {
                    let button = btn.clone();
                    drop(state);
                    return construct_button(button)
                },
                IpgWidgets::IpgCard(crd) => {
                    let card = crd.clone();
                    drop(state);
                  return construct_card(card)
                },
                IpgWidgets::IpgCheckBox(chk) => {
                    let c_box = chk.clone();
                    drop(state);
                    return construct_checkbox(c_box)
                },
                IpgWidgets::IpgImage(img) => {
                    let image = img.clone();
                    drop(state);
                    return construct_image(image)
                },
                IpgWidgets::IpgMenu(mn) => {
                    let menu = mn.clone();
                    drop(state);
;                    return construct_menu(menu)
                },
                IpgWidgets::IpgDatePicker(dp) => {
                    let d_picker = dp.clone();
                    drop(state);
                    return construct_date_picker(d_picker)
                },
                IpgWidgets::IpgPickList(pick) => {
                    let pck = pick.clone();
                    drop(state);
                    return construct_picklist(pck)
                },
                IpgWidgets::IpgProgressBar(bar) => {
                    let br = bar.clone();
                    drop(state);
                    return construct_progress_bar(br)
                }
                IpgWidgets::IpgSelectableText(sltxt) => {
                    let s_txt = sltxt.clone();
                    drop(state);
                    return construct_selectable_text(s_txt)
                },
                IpgWidgets::IpgRadio(radio) => {
                    let rad = radio.clone();
                    drop(state);
                    return construct_radio(rad) 
                },
                IpgWidgets::IpgRule(rule) => {
                    let rul = rule.clone();
                    drop(state);
                    return construct_rule(rul) 
                },
                IpgWidgets::IpgSlider(slider) => {
                    let sld = slider.clone();
                    drop(state);
                    return construct_slider(sld)
                },
                IpgWidgets::IpgSpace(sp) => {
                    return construct_space(sp)
                },
                IpgWidgets::IpgSvg(isvg) => {
                    let svg = isvg.clone();
                    drop(state);
                    return construct_svg(svg)
                },
                IpgWidgets::IpgText(text) => {
                    let txt = text.clone();
                    drop(state);
                    return construct_text(txt)
                },
                IpgWidgets::IpgTextInput(input) => {
                    let t_input = input.clone();
                    drop(state);
                    return construct_text_input(t_input)           
                },
                IpgWidgets::IpgTimer(tim) => {
                    let tm = tim.clone();
                    drop(state);
                    return construct_timer(tm);
                },
                IpgWidgets::IpgToggler(tog) => {
                    let tg = tog.clone();
                    drop(state);
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
        IpgContainers::IpgColumn(_) => panic!("Wrong container"),
        IpgContainers::IpgContainer(_) => panic!("Wrong container"),
        IpgContainers::IpgModal(_) => panic!("Wrong container"),
        IpgContainers::IpgMouseArea(_) => panic!("Wrong container"),
        IpgContainers::IpgTable(_) => panic!("Wrong container"),
        IpgContainers::IpgRow(_) => panic!("Wrong container"),
        IpgContainers::IpgScrollable(_) => panic!("Wrong container"),
        IpgContainers::IpgToolTip(_) => panic!("Wrong container"),
        IpgContainers::IpgWindow(wnd) => {
            wnd
        },
    }
}
