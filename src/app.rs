#![allow(dead_code)]

use iced::multi_window;
use iced::{font, window};
use iced::event::Event;
use iced::{Command, Element, Subscription, Theme};
use iced::executor;
use iced::widget::Column;
use iced::time;
use iced::Color;
use std::collections::HashMap;

use crate::ipg_widgets;
use ipg_widgets::ipg_button::{BTNMessage, construct_button, button_update};
use ipg_widgets::ipg_card::{CardMessage, construct_card, card_update};
use ipg_widgets::ipg_checkbox::{CHKMessage, construct_checkbox, checkbox_update};
use ipg_widgets::ipg_color_picker::{ColPikMessage, construct_color_picker, color_picker_update};
use ipg_widgets::ipg_column::construct_column;
use ipg_widgets::ipg_container::construct_container;
use ipg_widgets::ipg_date_picker::{DPMessage, construct_date_picker, date_picker_update};
use ipg_widgets::ipg_enums::{IpgContainers, IpgWidgets};
use ipg_widgets::ipg_events::process_events;
use ipg_widgets::ipg_menu::{MenuMessage, construct_menu_bar, menu_bar_update, 
                                         construct_menu_item, menu_item_update};
use ipg_widgets::ipg_pane_grid::{PGMessage, construct_pane_grid, pane_grid_update, 
                                 construct_pane, pane_update};
use ipg_widgets::ipg_pick_list::{PLMessage, construct_picklist, pick_list_update};
use ipg_widgets::ipg_progress_bar::construct_progress_bar;
use ipg_widgets::ipg_radio::{RDMessage, construct_radio, radio_update};
use ipg_widgets::ipg_row::construct_row;
use ipg_widgets::ipg_scrollable::{construct_scrollable, scrollable_update};
use ipg_widgets::ipg_selectable_text::{SLTXTMessage, construct_selectable_text, selectable_text_update};
use ipg_widgets::ipg_slider::{SLMessage, construct_slider, slider_update};
use ipg_widgets::ipg_space::construct_space;
use ipg_widgets::ipg_table::contruct_table;
use ipg_widgets::ipg_text::construct_text;
use ipg_widgets::ipg_text_editor::TEMessage;
use ipg_widgets::ipg_text_input::{TIMessage, construct_text_input, text_input_update};
use ipg_widgets::ipg_tool_tip::construct_tool_tip;
use ipg_widgets::ipg_window::{WndMessage, IpgWindow, add_windows, construct_window, window_update};


use ipg_widgets::helpers::get_usize_of_id;
use crate::{access_state, IpgIds};

use crate::iced_widgets::scrollable;

#[derive(Debug, Clone)]
pub enum Message {
    EventOccurred(Event),

    Button(usize, BTNMessage),
    Card(usize, CardMessage),
    CheckBox(usize, CHKMessage),
    ColorPicker(usize, ColPikMessage),
    DatePicker(usize, DPMessage),
    MenuBar(MenuMessage),
    MenuItem(MenuMessage),
    Pane(PGMessage),
    PaneGrid(PGMessage),
    PickList(usize, PLMessage),
    Radio(usize, RDMessage),
    Scrolled(usize, scrollable::Viewport),
    SelectableText(usize, SLTXTMessage),
    Slider(usize, SLMessage),
    TextEditor(TEMessage),
    TextInput(usize, TIMessage),
    Tick,
    FontLoaded(Result<(), font::Error>),
    UpdateText,
    Window(WndMessage),
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


impl multi_window::Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = Flags;


    fn new(flags: Flags) -> (Self, Command<Message>) {
        
        let (windows, mut spawn) = add_windows();
        spawn.push(font::load(include_bytes!("./graphics/fonts/bootstrap-icons.ttf").as_slice())
                                            .map(Message::FontLoaded));
        // spawn.push(font::load(iced_aw::BOOTSTRAP_FONT_BYTES).map(Message::FontLoaded));

        (
            Self {
                windows,
                timer_duration: flags.timer_duration,
                keyboard_event_enabled: flags.keyboard_event_enabled,
                mouse_event_enabled: flags.mouse_event_enabled,
                timer_event_enabled: flags.timer_event_enabled,
                window_event_enabled: flags.window_event_enabled,
            },
            
            Command::batch(spawn),
        )
    }

    fn title(&self, window: window::Id) -> String {
        self.windows
            .get(&window)
            .map(|window| window.title.clone())
            .unwrap_or("IcePyGui".to_string())
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        
        match message {
            Message::FontLoaded(_) => {
                Command::none()
            },
            Message::EventOccurred(ipg_event) => {
                process_events(ipg_event, self.keyboard_event_enabled,
                                            self.mouse_event_enabled,
                                            self.window_event_enabled,
                                            self.timer_event_enabled);
                Command::none()
            },
            Message::Tick => {
                Command::none()
            },
            Message::Button(id, message) => {
                button_update(id, message);
                Command::none()
            },
            Message::Card(id, message) => {
                card_update(id, message);
                 Command::none()
            },
            Message::CheckBox(id, message) => {
                checkbox_update(id, message);
                Command::none()
            },
            Message::ColorPicker(id, message) => {
                color_picker_update(id, message);
                Command::none()
            },
            Message::DatePicker(id, message) => {
                date_picker_update(id, message);
                Command::none()
            },
            Message::MenuBar(mn) => {
                menu_bar_update(mn);
                Command::none()
            },
            Message::MenuItem(mi) => {
                menu_item_update(mi);
                Command::none()
            },
            Message::Pane(pn) => {
                pane_update(pn);
                Command::none()
            },
            Message::PaneGrid(pg) => {
                pane_grid_update(pg);
                Command::none()
            },
            Message::PickList(id, message) => {
                pick_list_update(id, message);
                Command::none()
            },
            Message::Radio(id, message) => {
                radio_update(id, message);
                Command::none()
            },
            Message::Scrolled(id, vp) => {
                scrollable_update(id, vp);
                Command::none()
            },
            Message::SelectableText(id, message) => {
                selectable_text_update(id, message);
                Command::none()
            },
            Message::Slider(id, message) => {
                slider_update(id, message);
                Command::none()
            },
            Message::TextEditor(_message) => {
                // text_editor_update(message);
                Command::none()
            },
            Message::TextInput(id, message) => {
                text_input_update(id, message);
                Command::none()
            },
            Message::UpdateText => {
                Command::none()
            },
            Message::Window(message) => {
                window_update(message)
            },
        }
        
    }

    fn view(&self, window: window::Id) -> Element<'_, self::Message> {

        let content = create_content(window);

        let debug = self.windows
                            .get(&window)
                            .map(|window| window.debug)
                            .unwrap_or(false);

        let scroll = self.windows
                                .get(&window)
                                .map(|window|window.scroll)
                                .unwrap_or(false);
        
        if scroll {
        //     let width = self.windows
        //                                 .get(&window)
        //                                 .map(|window|window.scroll_width)
        //                                 .unwrap();
        //     let height = self.windows
        //                                 .get(&window)
        //                                 .map(|window|window.scroll_height)
        //                                 .unwrap();
        //     let direction = self.windows
        //                                 .get(&window)
        //                                 .map(|window|window.scroll_direction)
        //                                 .unwrap();
        //     let on_scroll_opt = self.windows
        //                                 .get(&window)
        //                                 .map(|window|window.on_scroll)
        //                                 .unwrap();
            
        //     match on_scroll_opt {
        //         Some(on_scroll) => {
        //             content = Scrollable::new(content)
        //                                     .width(width)
        //                                     .height(height)
        //                                     .direction(direction)
        //                                     .on_scroll(on_scroll)
        //                                     .into();
                    
        //         },
        //         None => {
        //             content = Scrollable::new(content)
        //                                     .width(width)
        //                                     .height(height)
        //                                     .direction(direction)
        //                                     .into();
        //         }
        //     }
        };

        let theme = self.windows
                                .get(&window)
                                .map(|window|window.theme.clone())
                                .unwrap();

        if debug {
            let color = match_theme(theme);
                content.explain(color)  
        } else {
            content
        }

    }

    fn subscription(&self) -> Subscription<Message> {
        
        if self.timer_event_enabled.1 && (self.keyboard_event_enabled.1 || 
                                        self.mouse_event_enabled.1 || 
                                        self.window_event_enabled.1) {

            let event = iced::event::listen().map(Message::EventOccurred);
            let timer = time::every(iced::time::Duration::from_millis(1000))
                                                                    .map(|_| Message::Tick);
            Subscription::batch(vec![event, timer])
            
        } else if self.timer_event_enabled.1 {

            time::every(iced::time::Duration::from_millis(1000)).map(|_| Message::Tick)

        } else if self.keyboard_event_enabled.1 || self.mouse_event_enabled.1 || self.window_event_enabled.1 {

            iced::event::listen().map(Message::EventOccurred)

        } else {

            Subscription::none()
        }
    }

    fn theme(&self, window: window::Id) -> Theme {
        self.windows.get(&window).unwrap().theme.clone()
    }
}



fn usize_to_bool(v: usize)->bool{
    match v{
       0 => false,
       1 => true,
       _ => panic!("Invalid bool in usize {}", v),
    }
}

fn create_content(iced_id: window::Id) -> Element<'static, Message> {
    
    let id_usize = get_usize_of_id(iced_id);

    let state = access_state();

    let unique_parent_ids = get_unique_parents(state.container_ids.get(&id_usize));

    let all_parent_ids = get_combine_parents_and_children(
                            &unique_parent_ids, state.ids.get(&id_usize));
    
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

fn get_combine_parents_and_children(unique_ids: &Vec<usize>, ids_opt: Option<&Vec<IpgIds>>) -> Vec<ParentChildIds> {

    let mut parent_child_ids: Vec<ParentChildIds> = vec![];

    let ids = match ids_opt {
        Some(ids) => ids,
        None => panic!("ids in get _and_combine_parents_and_children not found")
    };

    for id in unique_ids {

        let mut child_ids: Vec<usize> = vec![];

        for ids in ids {
            if id == &ids.parent_uid {
                child_ids.push(ids.id);
            }  
        }
        
        parent_child_ids.push(ParentChildIds { parent_id: id.clone(), child_ids })
    }

    parent_child_ids
}

fn get_children(parents: &Vec<ParentChildIds>, 
                index: &usize, 
                parent_ids: &Vec<usize>, 
                ) -> Element<'static, Message> {

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
                    return construct_container(con, content)
                },
                IpgContainers::IpgPane(pane) => {
                    return construct_pane(pane, content)
                },
                IpgContainers::IpgPaneGrid(pngd) => {
                    return construct_pane_grid(pngd, content)
                },
                IpgContainers::IpgRow(row) => {
                    return construct_row(row, content)
                },
                IpgContainers::IpgScrollable(scroll) => {
                    return construct_scrollable(scroll, content)
                },
                IpgContainers::IpgToolTip(tool) => {
                    return construct_tool_tip(tool, content)
                },
                IpgContainers::IpgWindow(wnd) => {
                    return construct_window(content)
                }
            },
        
        None => panic!("Container not found in fn get_container"),        
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
                    return construct_button(btn.clone())
                },
                IpgWidgets::IpgCard(crd) => {
                  return construct_card(crd.clone())
                },
                IpgWidgets::IpgCheckBox(chk) => {
                    return construct_checkbox(chk.clone())
                },
                IpgWidgets::IpgColorPicker(cp) => {
                    return construct_color_picker(cp.clone())
                },
                IpgWidgets::IpgMenuBar(mn) => {
                    return construct_menu_bar(mn)
                }
                IpgWidgets::IpgMenuItem(mi) => {
                    return construct_menu_item(mi)
                }
                IpgWidgets::IpgDatePicker(dp) => {
                    return construct_date_picker(dp.clone())
                },
                IpgWidgets::IpgPickList(pick) => {
                        return construct_picklist(pick.clone())
                },
                IpgWidgets::IpgProgressBar(bar) => {
                    return construct_progress_bar(bar)
                }
                IpgWidgets::IpgSelectableText(sltxt) => {
                    return construct_selectable_text(sltxt.clone())
                },
                IpgWidgets::IpgRadio(radio) => {
                    return construct_radio(radio.clone()) 
                },
                IpgWidgets::IpgSlider(slider) => {
                    return construct_slider(slider.clone())
                },
                IpgWidgets::IpgSpace(sp) => {
                    return construct_space(sp)
                },
                IpgWidgets::IpgTable(table) => {
                    return contruct_table(table.clone())
                },
                IpgWidgets::IpgText(text) => {
                    return construct_text(text)
                },
                IpgWidgets::IpgTextEditor(_te) => {
                    return Column::new().into()
                    // return construct_text_editor_controls(te) 
                },
                IpgWidgets::IpgTextInput(input) => {
                    return construct_text_input(input.clone())           
                },
            },
        None => panic!("Widget not found in fn get_widget")
    }
}

fn match_theme(theme: Theme) -> Color {

    match theme {
        Theme::Light => Color::BLACK,
        Theme::Dark => Color::WHITE,
        Theme::Dracula => Color::WHITE,
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

// #[cfg(test)]
// mod tests {
//     use crate::{IpgIds, access_state};
//     use crate::app::{ get_combine_parents_and_children, ParentChildIds};

//     fn setup_ids() -> Vec<IpgIds> {
//         return vec![
//             IpgIds {
//                 id: 1,
//                 parent_uid: 0,
//                 container_id: Some("col0".to_string()),
//                 parent_id: None,
//                 user_id: None,
//                 is_container: true,
//             },
//             IpgIds {
//                 id: 2,
//                 parent_uid: 1,
//                 container_id: None,
//                 parent_id: Some("col0".to_string()),
//                 user_id: None,
//                 is_container: false,
//             },
//             IpgIds {
//                 id: 3,
//                 parent_uid: 1,
//                 container_id: Some("row0".to_string()),
//                 parent_id: Some("col0".to_string()),
//                 user_id: None,
//                 is_container: true,
//             },
//             IpgIds {
//                 id: 4,
//                 parent_uid: 3,
//                 container_id: Some("col1".to_string()),
//                 parent_id: Some("row0".to_string()),
//                 user_id: None,
//                 is_container: true,
//             },
//             IpgIds {
//                 id: 5,
//                 parent_uid: 4,
//                 container_id: None,
//                 parent_id: Some("col1".to_string()),
//                 user_id: None,
//                 is_container: false,
//             },
//             IpgIds {
//                 id: 6,
//                 parent_uid: 3,
//                 container_id: Some("row1".to_string()),
//                 parent_id: Some("row0".to_string()),
//                 user_id: None,
//                 is_container: true,
//             },
//             IpgIds {
//                 id: 7,
//                 parent_uid: 6,
//                 container_id: None,
//                 parent_id: Some("col1".to_string()),
//                 user_id: None,
//                 is_container: false,
//             },
//             IpgIds {
//                 id: 8,
//                 parent_uid: 6,
//                 container_id: None,
//                 parent_id: Some("row0".to_string()),
//                 user_id: None,
//                 is_container: false,
//             },
//         ];
//     }

//     fn setup_all_parent_ids() -> Vec<ParentChildIds> {
//         return vec![
//             ParentChildIds {
//                 parent_id: 0,
//                 child_ids: vec![1],
//             },
//             ParentChildIds {
//                 parent_id: 1,
//                 child_ids: vec![2, 3],
//             },
//             ParentChildIds {
//                 parent_id: 3,
//                 child_ids: vec![4, 6],
//             },
//             ParentChildIds {
//                 parent_id: 4,
//                 child_ids: vec![5],
//             },
//             ParentChildIds {
//                 parent_id: 6,
//                 child_ids: vec![7, 8],
//             },
//         ]
//     }

//     #[test]
//     fn test_get_unique_parents() {

//         // let ids = setup_ids();

//         // let expected_unique_parent_ids = vec![0, 1, 3, 4, 6];
//         // let unique_parent_ids = get_unique_parents(&ids);
//         // assert_eq!(expected_unique_parent_ids, unique_parent_ids);
//     }

//     #[test]
//     fn test_get_combine_parents_and_children() {

//         let unique_parent_ids = vec![0, 1, 3, 4, 6];
//         let state = access_state();
//         dbg!(&state.ids);
//         let all_parent_ids = get_combine_parents_and_children(
//             &unique_parent_ids, &state.ids);

//         let expect_all_parent_ids = setup_all_parent_ids();

//         assert_eq!(expect_all_parent_ids, all_parent_ids);
//     }

// }