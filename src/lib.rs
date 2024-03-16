#![allow(non_snake_case)]
#![recursion_limit="2048"]
#![allow(unused)]

use pyo3::prelude::*;
use pyo3::types::{PyAny, PyList, PyModule, PyTuple};
use pyo3::PyObject;

use iced::multi_window::Application;
use iced::window::{self, Position};
use iced::{Color, Font, Length, Point, Settings, Size};
use iced::widget::text::{self, LineHeight};

use iced_aw::CardStyles;

use core::panic;
use std::iter::Iterator;
use std::collections::HashMap;

mod app;
use app::{App, Flags};

mod ipg_widgets;
mod iced_widgets;

use crate::iced_widgets::scrollable::Direction;

use ipg_widgets::ipg_button::{IpgButton, button_item_update};
use ipg_widgets::ipg_card::{IpgCard, card_item_update};
use ipg_widgets::ipg_checkbox::{IpgCheckBox, checkbox_item_update};
use ipg_widgets::ipg_color_picker::{IpgColorPicker, color_picker_item_update};
use ipg_widgets::ipg_column::IpgColumn;
use ipg_widgets::ipg_container::IpgContainer;
use ipg_widgets::ipg_date_picker::{IpgDatePicker, date_picker_item_update};
use ipg_widgets::ipg_events::{IpgEventCallbacks, IpgEvents, IpgKeyBoardEvent, 
                                IpgMouseEvent, IpgWindowEvent};
use ipg_widgets::ipg_image::IpgImage;
use ipg_widgets::ipg_menu::{IpgMenuBar, IpgMenuItem};
use ipg_widgets::ipg_pane_grid::{IpgPane, IpgPaneGrid};
use ipg_widgets::ipg_pick_list::IpgPickList;
use ipg_widgets::ipg_progress_bar::{IpgProgressBar, progress_bar_item_update};
use ipg_widgets::ipg_radio::{RadioDirection, IpgRadio};
use ipg_widgets::ipg_row::IpgRow;
use ipg_widgets::ipg_scrollable::IpgScrollable;
use ipg_widgets::ipg_selectable_text::IpgSelectableText;
use ipg_widgets::ipg_slider::IpgSlider;
use ipg_widgets::ipg_space::IpgSpace;
use ipg_widgets::ipg_table::IpgTable;
use ipg_widgets::ipg_text::IpgText;
use ipg_widgets::ipg_text_editor::IpgTextEditor;
use ipg_widgets::ipg_text_input::IpgTextInput;
use ipg_widgets::ipg_tool_tip::IpgToolTip;
use ipg_widgets::ipg_window::IpgWindow;
use ipg_widgets::ipg_enums::{IpgContainers, IpgWidgets};

use ipg_widgets::helpers::{check_for_dup_container_ids, 
                            check_for_dup_user_ids, 
                            get_width, get_height,
                            get_alignment,
                            get_horizontal_alignment,
                            get_vertical_alignment,
                            get_line_height,
                            get_padding,
                            get_scroll_direction,
                            get_shaping,};

const DEFAULT_PADDING: [f64; 1] = [10.0];
const ICON_FONT_BOOT: Font = Font::with_name("bootstrap-icons");
const ICON_FONT: Font = Font::with_name("icons");
const RADIO_SIZE: usize = 26;

use std::sync::{Mutex, MutexGuard};
use once_cell::sync::Lazy;


#[derive(Debug, Clone)]
pub struct CallBack {
    id: usize,
    cb: Option<PyObject>,
    name: Option<String>,
    
}

#[derive(Debug, Clone)]
pub struct CallBackEvent {
    id: usize,
    cb: PyObject,
    name: IpgEventCallbacks,
    
}

#[derive(Debug, Clone)]
pub struct Callbacks {
    callbacks: Vec<CallBack>,
    cb_events: Vec<CallBackEvent>,
    user_data: Vec<(usize, Option<PyObject>)>,
    radios: Vec<(String, Vec<usize>)>,
}

pub static CALLBACKS: Mutex<Callbacks> = Mutex::new(Callbacks {
    callbacks: vec![],
    cb_events: vec![],
    user_data: vec![],
    radios: vec![],
});

pub fn access_callbacks() -> MutexGuard<'static, Callbacks> {
    CALLBACKS.lock().unwrap()
}

pub struct State {
    pub containers: Lazy<HashMap<usize, IpgContainers>>,
    pub container_ids: Lazy<HashMap<usize, Vec<usize>>>,  // <window_id=usize, vec<container_id=usize>>
    pub widgets: Lazy<HashMap<usize, IpgWidgets>>,
    pub ids: Lazy<HashMap<usize, Vec<IpgIds>>>,  // <window_id=usize, Vec<IpgIds=structure>>
    pub pane_ids: Vec<usize>,
    pub windows: Vec<IpgWindow>,
    pub windows_str_ids: Lazy<HashMap<String, usize>>,  // <window_id=str, window_id=usize>
    pub container_wnd_str_ids: Lazy<HashMap<String, String>>, // <container_id=string, window_id-usize>
    pub window_debug: Lazy<HashMap<window::Id, bool>>,
    pub events: Vec<IpgEvents>,
    pub kb_modifiers: Lazy<HashMap<String, bool>>,
}

pub static STATE: Mutex<State> = Mutex::new(
    State {
        containers: Lazy::new(||HashMap::new()),
        container_ids: Lazy::new(||HashMap::new()),
        widgets: Lazy::new(||HashMap::new()),
        ids: Lazy::new(||HashMap::new()),
        pane_ids: vec![],
        windows: vec![],
        windows_str_ids: Lazy::new(||HashMap::new()),
        container_wnd_str_ids: Lazy::new(||HashMap::new()),
        window_debug: Lazy::new(||HashMap::new()),
        events: vec![],
        kb_modifiers: Lazy::new(||HashMap::from([
                                                ("shift".to_string(), false),
                                                ("control".to_string(), false),
                                                ("alt".to_string(), false),
                                                ("logo".to_string(), false),
                                            ])),
    }
);

pub fn access_state() -> MutexGuard<'static, State> {
    STATE.lock().unwrap()
}


#[derive(Debug, Clone)]
pub struct IpgIds {
    pub id: usize,  // id of widget or container
    pub parent_uid: usize,  // parent_uid == id of a container
    pub container_id: Option<String>, //required for all containers, optional  since this struct used for widgets too
    pub parent_id: String,  //user created parent_id == container_id
    pub user_id: Option<String>,  // user created convenience if wanted
    pub is_container: bool, // if container to get all ids of parents of containers
}


#[pyclass]
pub struct IPG {
    id: usize,
    window_id: usize,
    card_style: Option<String>,
}

#[pymethods]
impl IPG {
    #[new]
    fn new() -> IPG {
        IPG {
            id: 0,
            window_id: 0,
            card_style: None,
        }
    }

    #[pyo3(signature = ())]
    fn start_session(&self) {

        let state = access_state();
        let size = Size::new(state.windows[0].width, state.windows[0].height);
        let position = state.windows[0].position;
        let visible = state.windows[0].visible;
        let resizable = state.windows[0].resizable;
        let exit_on_close_request = state.windows[0].exit_on_close_request;
        let mut flags = Flags {keyboard_event_enabled: (0, false),
                                        mouse_event_enabled: (0, false), 
                                        timer_event_enabled: (0, false), 
                                        window_event_enabled: (0, false),
                                        timer_duration: 0,
                                    };

        for events in state.events.iter() {
            match events {
                IpgEvents::Keyboard(kb)=> {
                    flags.keyboard_event_enabled = (kb.id, kb.enabled)
                },
                IpgEvents::Mouse(mouse)=> {
                    flags.mouse_event_enabled = (mouse.id, mouse.enabled)
                },
                IpgEvents::Timer(timer) => {
                    flags.timer_event_enabled = (timer.id, timer.enabled);
                    flags.timer_duration = timer.duration;
                },
                IpgEvents::Window(wnd) => {
                    flags.window_event_enabled = (wnd.id, wnd.enabled);
                }, 
            }
        }

        drop(state);

        let _ = App::run(Settings {
            window: window::Settings {
                size,
                position,
                visible,
                resizable,
                exit_on_close_request,
                ..Default::default()
            },
            flags,
            ..Default::default()
        });
    }

    #[pyo3(signature = (window_id, title, width, height, pos_x=None, pos_y=None,
                        pos_centered=false, resizable=true, 
                        theme="dark".to_string(), exit_on_close=true, on_resize=None, show=true, 
                        scroll=false, scroll_width=None, scroll_height=None, 
                        scroll_direction=None, on_scroll=None, debug=false,
                        user_data=None))]
    fn add_window(&mut self,
                        window_id: String, 
                        title: String, 
                        width: f32, 
                        height: f32, 
                        pos_x: Option<f32>,
                        pos_y: Option<f32>,
                        pos_centered: bool,
                        resizable: bool,
                        theme: String,
                        exit_on_close: bool,
                        on_resize: Option<PyObject>,
                        show: bool,
                        scroll: bool,
                        scroll_width: Option<f32>,
                        scroll_height: Option<f32>,
                        scroll_direction: Option<String>,
                        on_scroll: Option<bool>,
                        debug: bool,
                        user_data: Option<PyObject>,
                    ) -> PyResult<usize>
    {
        self.id += 1;

        let _on_scroll = on_scroll;

        let scroll_width = get_width(scroll_width, false);
        let scroll_height = get_height(scroll_height, false);

        let scroll_direction = get_scroll_direction(scroll_direction);
        
        let mut window_theme = iced::Theme::Dark;

        if theme == "light" {
            window_theme = iced::Theme::Light;
        }

        let mut window_position = Position::Default;

        if pos_x.is_some() && pos_y.is_some() {
            let pos_x = match pos_x {
                Some(x) => x,
                None => 0.0,
            };
            let pos_y = match pos_y {
                Some(y) => y,
                None => 0.0,
            };
            window_position = Position::Specific(Point { x: pos_x, y: pos_y })
        }

        if pos_centered {
            window_position = Position::Centered;
        }

        let visible = show;

        let mut state = access_state();

        if state.windows_str_ids.get(&window_id).is_some() {
            panic!("Window id {} is not unique", window_id)
        };

        let mut cb_name: Option<String> = None;

        if on_resize.is_some() {
            cb_name = Some("window_on_resize".to_string());
            let cb = CallBack{id: self.id, cb: on_resize, name: cb_name.clone()};

            add_callback_to_mutex(cb);
        }
        
        state.windows_str_ids.insert(window_id.clone(), self.window_id);

        state.ids.insert(self.window_id, vec![IpgIds{id: self.id, parent_uid: 0, container_id: Some(window_id.clone()),
                                                parent_id: "".to_string(), user_id: None, is_container: true}]);

        state.container_ids.insert(self.window_id, vec![self.id]);

        state.containers.insert(self.id, IpgContainers::IpgWindow(IpgWindow::new(
                                            self.id,
                                            self.window_id,
                                            window_id.clone(),
                                            title.clone(), 
                                            width, 
                                            height, 
                                            window_position,
                                            exit_on_close,
                                            window_theme.clone(), 
                                            resizable,
                                            visible,
                                            scroll,
                                            scroll_width,
                                            scroll_height,
                                            scroll_direction,
                                            debug,
                                            user_data.clone(),
                                            cb_name.clone(),
                                            )));

        state.windows.push(IpgWindow::new(
                                        self.id,
                                        self.window_id,
                                        window_id.clone(),
                                        title, 
                                        width, 
                                        height, 
                                        window_position,
                                        exit_on_close,
                                        window_theme, 
                                        resizable,
                                        show,
                                        scroll,
                                        scroll_width,
                                        scroll_height,
                                        scroll_direction,
                                        debug,
                                        user_data,
                                        cb_name,
                                        ));
        drop(state);

        self.window_id += 1;

        Ok(self.id)

    }

    #[pyo3(signature = (window_id, container_id, parent_id=None, 
                        width=None, height=None, width_fill=false, height_fill=false, 
                        max_height=f32::INFINITY, max_width=f32::INFINITY,
                        align_x="left", align_y="top",
                        padding=DEFAULT_PADDING.to_vec(), show=true,
                        user_id=None))]
    fn add_container(&mut self,
                        window_id: String,
                        container_id: String,
                        // **above reuired
                        parent_id: Option<String>,
                        width: Option<f32>,
                        height: Option<f32>,
                        width_fill: bool,
                        height_fill: bool,
                        max_height: f32,
                        max_width: f32,
                        align_x: &str,
                        align_y: &str, 
                        padding: Vec<f64>, 
                        show: bool,
                        user_id: Option<String>, 
                        ) -> PyResult<usize>
    {
        self.id += 1;
        
        let align_x = get_horizontal_alignment(align_x); 
        let align_y = get_vertical_alignment(align_y);
        
        let width = get_width(width, width_fill);
        let height = get_height(height, height_fill);
        let padding = get_padding(padding);
        
        let prt_id = match parent_id {
            Some(id) => id,
            None => window_id.clone(),
        };

        set_state_of_container(self.id, window_id, Some(container_id), prt_id, user_id);

        let mut state = access_state();

        state.containers.insert(self.id, IpgContainers::IpgContainer(IpgContainer::new(
                                                self.id,
                                                show,
                                                padding,
                                                width,
                                                height,
                                                max_width,
                                                max_height,
                                                align_x,
                                                align_y,
                                            )));

        drop(state);
        Ok(self.id)

    }


    #[pyo3(signature = (window_id, container_id, parent_id=None, 
                        align_items="start", width=None, height=None,
                        width_fill=false, height_fill=false,
                        max_width=f32::INFINITY, padding=DEFAULT_PADDING.to_vec(), 
                        spacing=20.0, show=true, user_id= None, 
                        ))]
    fn add_column(&mut self,
                        window_id: String,
                        container_id: String,
                        // **above required
                        parent_id: Option<String>,
                        align_items: &str,
                        width: Option<f32>,
                        height: Option<f32>,
                        width_fill: bool,
                        height_fill: bool,
                        max_width: f32,
                        padding: Vec<f64>,
                        spacing: f32,
                        show: bool,
                        user_id: Option<String>,
                        ) -> PyResult<usize> 
    {

        self.id += 1;
        
        let align_items = get_alignment(align_items);

        let width = get_width(width, width_fill);
        let height = get_height(height, height_fill);

        let padding = get_padding(padding);

        let prt_id = match parent_id {
            Some(id) => id,
            None => window_id.clone(),
        };

        set_state_of_container(self.id, window_id, Some(container_id), prt_id, user_id);

        let mut state = access_state();

        state.containers.insert(self.id, IpgContainers::IpgColumn(IpgColumn::new(
                                        self.id,  
                                        show, 
                                        spacing, 
                                        padding, 
                                        width, 
                                        height, 
                                        max_width, 
                                        align_items,
                                    )));
    
    Ok(self.id)

    }

    #[pyo3(signature = (window_id, container_id, add_direction, ratio, parent_id=None, user_id=None))]
    fn add_pane(&mut self,
                    window_id: String, 
                    container_id: String,
                    add_direction: String,
                    ratio: f32,
                    parent_id: Option<String>, 
                    user_id: Option<String>
                    ) -> PyResult<usize> 
    {
        self.id += 1;

        if !["first", "right", "below"].contains(&add_direction.as_str()) {
            panic!("add_direction must be one of the following strings 'first', 'right', or 'below'");
        }

        let prt_id = match parent_id {
            Some(id) => id,
            None => window_id.clone(),
        };

        set_state_of_container(self.id, window_id, Some(container_id), prt_id, user_id);

        let mut state = access_state();

        state.containers.insert(self.id, IpgContainers::IpgPane(IpgPane::new(
                                                            self.id,
                                                            add_direction,
                                                            ratio,
                                                            )));

        Ok(self.id)
    }

    #[pyo3(signature = (window_id, container_id, parent_id=None, 
                        width=None, height=None, width_fill=false, height_fill=false,
                        spacing=10.0, padding=DEFAULT_PADDING.to_vec(), 
                        show=true, user_id=None, 
                        ))]
    fn add_pane_grid(&mut self,
                        window_id: String,
                        container_id: String,

                        parent_id: Option<String>,
                        width: Option<f32>,
                        height: Option<f32>,
                        width_fill: bool,
                        height_fill: bool,
                        spacing: f32,
                        padding: Vec<f64>,
                        show: bool,
                        user_id: Option<String>,
                        ) -> PyResult<usize> 
    {

        self.id += 1;

        let width = get_width(width, width_fill);
        let height = get_height(height, height_fill);

        let padding = get_padding(padding);

        let prt_id = match parent_id {
            Some(id) => id,
            None => window_id.clone(),
        };

        set_state_of_container(self.id, window_id, Some(container_id), prt_id, user_id);

        let mut state = access_state();

        state.containers.insert(self.id, IpgContainers::IpgPaneGrid(IpgPaneGrid::new(
                                        self.id,
                                        width, 
                                        height,
                                        spacing, 
                                        padding, 
                                        show,
                                    )));

    Ok(self.id)

    }
    
    #[pyo3(signature = (window_id, container_id, parent_id=None, 
                        align_items="start", width=None, height=None, 
                        width_fill=false, height_fill=false,
                        padding=DEFAULT_PADDING.to_vec(), spacing=20.0, 
                        show=true, user_id=None))]
    fn add_row(&mut self,
                    window_id: String,
                    container_id: String,
                    // required above
                    parent_id: Option<String>,
                    align_items: &str,
                    width: Option<f32>,
                    height: Option<f32>,
                    width_fill: bool,
                    height_fill: bool,
                    padding: Vec<f64>,
                    spacing: f32,
                    show: bool,
                    user_id: Option<String>,
                    ) -> PyResult<usize> 
    {

        self.id += 1;

        let align_items = get_alignment(align_items);

        let width = get_width(width, width_fill);
        let height = get_height(height, height_fill);

        let padding = get_padding(padding);

        let prt_id = match parent_id {
            Some(id) => id,
            None => window_id.clone(),
        };

        set_state_of_container(self.id, window_id, Some(container_id), prt_id, user_id);

        let mut state = access_state();

        state.containers.insert(self.id, IpgContainers::IpgRow(IpgRow::new(
                                    self.id,  
                                    show, 
                                    spacing, 
                                    padding, 
                                    width, 
                                    height, 
                                    align_items,
                                )));
                                
        Ok(self.id)

    }

    #[pyo3(signature = (window_id, container_id, parent_id=None, 
                        width=None, height=None, width_fill=false, height_fill=false, 
                        direction="vertical".to_string(), on_scroll=None, 
                        user_data=None, user_id=None))]
    fn add_scrollable(&mut self,
                            window_id: String,
                            container_id: String,

                            parent_id: Option<String>,
                            width: Option<f32>,
                            height: Option<f32>,
                            width_fill: bool,
                            height_fill: bool,
                            direction: String,
                            on_scroll: Option<PyObject>,
                            user_data: Option<PyObject>,
                            user_id: Option<String>,
                            ) -> PyResult<usize>
    {
        self.id += 1;

        let mut cb_name: Option<String> = None;

        if on_scroll.is_some() {
            cb_name = Some("on_scroll".to_string());
            let cb = CallBack{id: self.id, cb: on_scroll, name: cb_name.clone()};

            add_callback_to_mutex(cb);
        }

        let width = get_width(width, width_fill);
        let height = get_height(height, height_fill);

        let direction: Direction = get_scroll_direction(Some(direction));

        let prt_id = match parent_id {
            Some(id) => id,
            None => window_id.clone(),
        };

        set_state_of_container(self.id, window_id, Some(container_id), prt_id, user_id);

        let mut state = access_state();
      
        state.containers.insert(self.id, IpgContainers::IpgScrollable(IpgScrollable::new( 
                                                    self.id,
                                                    width,
                                                    height,
                                                    direction,
                                                    user_data,
                                                    // style,
                                                    cb_name,
                                                    )));

        Ok(self.id)

    }

    #[pyo3(signature = (window_id, container_id, position, text_to_display, parent_id=None, 
                        gap=10, padding=0.0, snap_within_viewport=true, style="box".to_string()))]
    fn add_tool_tip(&mut self,
                            window_id: String,
                            container_id: String,
                            position: String,
                            text_to_display: String,
                            // **above required
                            parent_id: Option<String>,
                            gap: u16,
                            padding: f32,
                            snap_within_viewport: bool,
                            style: String,
                            ) -> PyResult<usize>
    {

        self.id += 1;

        let prt_id = match parent_id {
            Some(id) => id,
            None => window_id.clone(),
        };

        set_state_of_container(self.id, window_id, Some(container_id), prt_id, None);

        let mut state = access_state();

        state.containers.insert(self.id, IpgContainers::IpgToolTip(IpgToolTip::new( 
                                                            self.id,
                                                            position,
                                                            text_to_display,
                                                            gap,
                                                            padding,
                                                            snap_within_viewport,
                                                            style,
                                                            )));
        Ok(self.id)

    }
    
    #[pyo3(signature = (parent_id, label, on_press=None, width=None,
                        height=None, width_fill=false, height_fill=false,
                        padding=vec![10.0], corner_radius=15.0, 
                        style="primary".to_string(), user_data=None, show=true, user_id=None, 
                        ))]
    fn add_button(&mut self,
                        parent_id: String,
                        label: String,
                        // ** above required
                        on_press: Option<PyObject>,
                        width: Option<f32>,
                        height: Option<f32>,
                        width_fill: bool,
                        height_fill: bool,
                        padding: Vec<f64>,
                        corner_radius: f32,
                        style:String,
                        user_data: Option<PyObject>,
                        show: bool,
                        user_id: Option<String>,
                        ) -> PyResult<usize> 
    {
        self.id += 1;

        let mut cb_name: Option<String> = None;

        if on_press.is_some() {
            cb_name = Some("button".to_string());
            let cb = CallBack{id: self.id, cb: on_press, name: cb_name.clone()};

            add_callback_to_mutex(cb);
        }

        let width = get_width(width, width_fill);
        let height = get_height(height, height_fill);

        let padding = get_padding(padding);
        
        set_state_of_widget(self.id, parent_id, user_id);

        let mut state = access_state();

        state.widgets.insert(self.id, IpgWidgets::IpgButton(IpgButton::new(
                                                self.id,
                                                show,
                                                user_data,
                                                label,
                                                width,
                                                height,
                                                padding,
                                                corner_radius,
                                                style,
                                                cb_name,                              
                                                )));
        
        Ok(self.id)
    
    }

    #[pyo3(signature = (parent_id, head, body, foot=None, close_size=0.0, on_close=None, 
                        width=None, height=None, width_fill=false, height_fill=false, 
                        max_width=f32::INFINITY, max_height=f32::INFINITY, 
                        padding_head=vec![5.0], padding_body=vec![5.0], padding_foot=vec![5.0],
                        show=true, style=None, user_data=None, user_id=None))]
    fn add_card(&mut self,
                parent_id: String, 
                head: String,
                body: String,
                // above required
                foot: Option<String>,
                close_size: f32,
                on_close: Option<PyObject>,
                width: Option<f32>,
                height: Option<f32>,
                width_fill: bool,
                height_fill: bool,
                max_width: f32,
                max_height: f32,
                padding_head: Vec<f64>,
                padding_body: Vec<f64>,
                padding_foot: Vec<f64>,
                show: bool,
                style: Option<String>,
                user_data: Option<PyObject>, 
                user_id: Option<String>,
                ) -> PyResult<usize> 
    {
        self.id += 1;

        let style = self.card_style.clone();

        self.card_style = None;
        
        let mut cb_name: Option<String> = None;

        if on_close.is_some() {
            cb_name = Some("card".to_string());
            let cb = CallBack{id: self.id, cb: on_close, name: cb_name.clone()};

            add_callback_to_mutex(cb);
        }

        let width = get_width(width, width_fill);
        let height = get_height(height, height_fill);

        let padding_head = get_padding(padding_head);
        let padding_body = get_padding(padding_body);
        let padding_foot = get_padding(padding_foot);

        set_state_of_widget(self.id, parent_id, user_id);

         let mut state = access_state();

        state.widgets.insert(self.id, IpgWidgets::IpgCard(IpgCard::new(
                                                    self.id,
                                                    show,
                                                    user_data,
                                                    width,
                                                    height,
                                                    max_width,
                                                    max_height,
                                                    padding_head,
                                                    padding_body,
                                                    padding_foot,
                                                    close_size,
                                                    head,
                                                    body,
                                                    foot,
                                                    style,
                                                    cb_name,
                                                )));

        Ok(self.id)

    }

    #[pyo3(signature = (primary=None, secondary=None, success=None, danger=None, 
                        warning=None, info=None, light=None, dark=None, white=None,
                        default=None))]
    fn card_style(&mut self,
                    primary: Option<i32>,
                    secondary: Option<i32>,
                    success: Option<i32>, 
                    danger: Option<i32>, 
                    warning: Option<i32>,
                    info: Option<i32>, 
                    light: Option<i32>,
                    dark: Option<i32>,
                    white: Option<i32>,
                    default: Option<i32>,
                    ) 
    {
        if primary.is_some() {
            self.card_style = Some("primary".to_string());
            return
        }
        if secondary.is_some() {
            self.card_style = Some("secondary".to_string());
        }
        if success.is_some() {
            self.card_style = Some("success".to_string());
            return
        }
        if danger.is_some() {
            self.card_style = Some("danger".to_string());
            return
        }
        if warning.is_some() {
            self.card_style = Some("warning".to_string());
            return
        }
        if info.is_some() {
            self.card_style = Some("info".to_string());
            return
        }
        if light.is_some() {
            self.card_style = Some("light".to_string());
            return
        }
        if dark.is_some() {
            self.card_style = Some("dark".to_string());
            return
        }
        if white.is_some() {
            self.card_style = Some("white".to_string());
            return
        }
        if default.is_some() {
            self.card_style = Some("default".to_string());
            return
        }
    }

    #[pyo3(signature = (parent_id, on_checked=None, is_checked=false, label="".to_string(), 
                        width=None, width_fill=false, size=16.0, spacing=20.0, 
                        text_line_height=1.3, text_shaping="basic".to_string(),
                        text_size=16.0, user_data=None, show=true, user_id=None))] 
    fn add_checkbox(&mut self,
                        parent_id: String,
                        // ** above required
                        on_checked: Option<PyObject>,
                        is_checked: bool,
                        label: String,
                        width: Option<f32>,
                        width_fill: bool,
                        size: f32,
                        spacing: f32,
                        text_line_height: f32,
                        text_shaping: String,
                        text_size: f32,
                        user_data: Option<PyObject>,
                        show: bool,
                        user_id: Option<String>,
                        ) -> PyResult<usize> 
    {

        self.id += 1;

        let mut cb_name: Option<String> = None;

        if on_checked.is_some() {
            cb_name = Some("checkbox".to_string());
            let cb = CallBack{id: self.id, cb: on_checked, name: cb_name.clone()};

            add_callback_to_mutex(cb);
        }
       
        let text_shaping = get_shaping(text_shaping);

        let text_line_height = text::LineHeight::Relative(text_line_height);

        let width = get_width(width, width_fill);
        
        set_state_of_widget(self.id, parent_id, user_id);

        let mut state = access_state();

        state.widgets.insert(self.id, IpgWidgets::IpgCheckBox(IpgCheckBox::new(
                                                    self.id,
                                                    show,
                                                    user_data,
                                                    is_checked,
                                                    label,
                                                    width,
                                                    size,
                                                    spacing,
                                                    text_size,
                                                    text_line_height,
                                                    text_shaping,
                                                    cb_name,
                                                    )));

        Ok(self.id)

    }

    #[pyo3(signature = (parent_id, label="Set Color".to_string(), 
                        on_submit=None, show=false, 
                        start_up_color=[0.5, 0.2, 0.7, 1.0], 
                        width=None, height=None, width_fill=false, height_fill=false,
                        padding=vec![10.0], corner_radius=0.0, 
                        style="primary".to_string(), user_data=None, user_id=None, 
                        ))]
    fn add_color_picker(
                    &mut self,
                    parent_id: String,
                    // ** above required
                    label: String,
                    on_submit: Option<PyObject>,
                    show: bool,
                    start_up_color: [f32; 4],
                    width: Option<f32>,
                    height: Option<f32>,
                    width_fill: bool,
                    height_fill: bool,
                    padding: Vec<f64>,
                    corner_radius: f32,
                    style:String,
                    user_data: Option<PyObject>,
                    user_id: Option<String>,
                    ) -> PyResult<usize> 
    {
        self.id += 1;

        let mut cb_name: Option<String> = None;

        if on_submit.is_some() {
            cb_name = Some("color_on_submit".to_string());
            let cb = CallBack{id: self.id, cb: on_submit, name: cb_name.clone()};

            add_callback_to_mutex(cb);
        }

        let color = Color::from_rgba(start_up_color[0], 
                                            start_up_color[1], 
                                            start_up_color[2], 
                                            start_up_color[3]);

        let width = get_width(width, width_fill);
        let height = get_height(height, height_fill);

        let padding = get_padding(padding);

        set_state_of_widget(self.id, parent_id, user_id);

        let mut state = access_state();

        state.widgets.insert(self.id, IpgWidgets::IpgColorPicker(
                            IpgColorPicker::new(
                                            self.id,
                                            show,
                                            color,
                                            user_data,
                                            label,
                                            width,
                                            height,
                                            padding,
                                            corner_radius,
                                            style,
                                            cb_name,                              
                                        )));

        Ok(self.id)

    }

    #[pyo3(signature = (parent_id, label="Calendar".to_string(), size_factor=1.0, 
                        padding=vec![5.0], on_select=None, 
                        user_data=None, show=false, user_id=None))]
    fn add_date_picker(&mut self,
                        parent_id: String,
                        // ** above required
                        label: String,
                        size_factor: f32,
                        padding: Vec<f64>,
                        on_select: Option<PyObject>,
                        user_data: Option<PyObject>,
                        show: bool,
                        user_id: Option<String>,
                        ) -> PyResult<usize> 
    {
        self.id += 1;

        let mut cb_name: Option<String> = None;

        if size_factor < 1.0 {
            panic!("Size factor for date picker must be > 1.0")
        }

        if on_select.is_some() {
            cb_name = Some("date_picker".to_string());
            let cb = CallBack{id: self.id, cb: on_select, name: cb_name.clone()};

            add_callback_to_mutex(cb);
        }

        let padding = get_padding(padding);

        set_state_of_widget(self.id, parent_id, user_id);

        let mut state = access_state();

        state.widgets.insert(self.id, IpgWidgets::IpgDatePicker(IpgDatePicker::new(
                                            self.id,
                                            label,
                                            size_factor,
                                            padding,
                                            show,
                                            user_data,
                                            cb_name,                            
                                        )));
        Ok(self.id)
    }

    #[pyo3(signature = (parent_id, image_path, width=None, width_fill=false, 
                        height=None, height_fill=false, 
                        padding=vec![5.0], on_press=None, on_release=None,
                        on_right_press=None, on_right_release=None,
                        on_middle_press=None, on_middle_release=None, 
                        user_data=None, show=false, user_id=None))]
fn add_image(&mut self,
                    parent_id: String,
                    image_path: String,
                    width: Option<f32>,
                    width_fill: bool,
                    height: Option<f32>,
                    height_fill: bool,
                    padding: Vec<f64>,
                    on_press: Option<PyObject>,
                    on_release: Option<PyObject>,
                    on_right_press: Option<PyObject>,
                    on_right_release: Option<PyObject>,
                    on_middle_press: Option<PyObject>,
                    on_middle_release: Option<PyObject>,
                    user_data: Option<PyObject>,
                    show: bool,
                    user_id: Option<String>,
) -> PyResult<usize>
{
    self.id += 1;

    let mut callback_made = false;

    let mut cb_on_press: Option<String> = None;

    if on_press.is_some() {
        cb_on_press = Some("cb_on_press".to_string());
        callback_made = true;
    }
    add_callback_to_mutex(CallBack{id: self.id, cb: on_press, name: cb_on_press.clone()});

    let mut cb_on_release: Option<String> = None;

    if on_release.is_some() {
        cb_on_release = Some("cb_on_release".to_string());
        callback_made = true;
    }
    add_callback_to_mutex(CallBack{id: self.id, cb: on_release, name: cb_on_release.clone()});

    let mut cb_on_right_press: Option<String> = None;

    if on_right_press.is_some() {
        cb_on_right_press = Some("cb_on_right_press".to_string());
        callback_made = true;
    }
    add_callback_to_mutex(CallBack{id: self.id, cb: on_right_press, name: cb_on_right_press.clone()});

    let mut cb_on_right_release: Option<String> = None;

    if on_right_release.is_some() {
        cb_on_right_release = Some("cb_on_right_release".to_string());
        callback_made = true;
    }
    add_callback_to_mutex(CallBack{id: self.id, cb: on_right_release, name: cb_on_right_release.clone()});

    let mut cb_on_middle_press: Option<String> = None;

    if on_middle_press.is_some() {
        cb_on_middle_press = Some("cb_on_middle_press".to_string());
        callback_made = true;
    }
    add_callback_to_mutex(CallBack{id: self.id, cb: on_middle_press, name: cb_on_middle_press.clone()});

    let mut cb_on_middle_release: Option<String> = None;

    if on_middle_release.is_some() {
        cb_on_middle_release = Some("cb_on_middle_release".to_string());
        callback_made = true;
    }

    add_callback_to_mutex(CallBack{id: self.id, cb: on_middle_release, name: cb_on_middle_release.clone()});

    let width = get_width(width, width_fill);
    let height = get_height(height, height_fill);

    set_state_of_widget(self.id, parent_id, user_id);

    let mut state = access_state();

    state.widgets.insert(self.id, IpgWidgets::IpgImage(IpgImage::new(
                                                self.id,
                                                image_path,
                                                width,
                                                height,
                                                show,
                                                user_data,
                                                cb_on_press,
                                                cb_on_release,
                                                cb_on_right_press,
                                                cb_on_right_release,
                                                cb_on_middle_press,
                                                cb_on_middle_release,
                                                callback_made,
                                            )));

    Ok(self.id)
}

    #[pyo3(signature = (parent_id, items, user_id=None))]
    fn add_menu_bar(&mut self, 
                            parent_id: String, 
                            items: Vec<String>,
                            user_id: Option<String> 
                        ) -> PyResult<usize> 
    {
        self.id += 1;

        set_state_of_widget(self.id, parent_id, user_id);

        let mut state = access_state();

        state.widgets.insert(self.id, IpgWidgets::IpgMenuBar(IpgMenuBar::new(
                                                                self.id,
                                                                items,
                                                                )));

        Ok(self.id)
    }

    #[pyo3(signature = (parent_id, item, user_id=None))]
    fn add_menu_item(&mut self,
                            parent_id: String, 
                            item: String, 
                            user_id: Option<String>
                        ) -> PyResult<usize> 
    {
        self.id += 1;

        set_state_of_widget(self.id, parent_id, user_id);

        let mut state = access_state();

        state.widgets.insert(self.id, IpgWidgets::IpgMenuItem(IpgMenuItem::new(
                                                                self.id,
                                                                item,
                                                                )));

        Ok(self.id)
    }

    #[pyo3(signature = (parent_id, options, callback=None, width=None,
                        width_fill=false, padding=vec![5.0],  
                        placeholder=None, selected=None, text_size=None, 
                        text_line_height=1.3, text_shaping="basic".to_string(), 
                        user_data=None, show=true, user_id=None))]
    fn add_pick_list(&mut self,
                        parent_id: String,
                        options: Vec<String>,
                        // **above required
                        callback: Option<PyObject>,
                        width: Option<f32>,
                        width_fill: bool,
                        padding: Vec<f64>,
                        placeholder: Option<String>,
                        selected: Option<String>,
                        text_size: Option<f32>,
                        text_line_height: f32,
                        text_shaping: String,
                        user_data: Option<PyObject>,
                        show: bool,
                        user_id: Option<String>,
                    ) -> PyResult<usize>
    {

        self.id += 1;

        let mut cb_name: Option<String> = None;

        if callback.is_some() {
            cb_name = Some("picklist".to_string());
            let cb = CallBack{id: self.id, cb: callback, name: cb_name.clone()};

            add_callback_to_mutex(cb);
        }

        let padding = get_padding(padding);

        let text_line_height = text::LineHeight::Relative(text_line_height);
        
        let text_shaping = get_shaping(text_shaping);

        let width = get_width(width, width_fill);

        set_state_of_widget(self.id, parent_id, user_id);

        let mut state = access_state();

        state.widgets.insert(self.id, IpgWidgets::IpgPickList(IpgPickList::new(  
                                                        self.id,
                                                        show,
                                                        user_data,
                                                        options,
                                                        placeholder,
                                                        selected,
                                                        width,
                                                        padding,
                                                        text_size,
                                                        text_line_height,
                                                        text_shaping,
                                                        cb_name,
                                                    )));

        Ok(self.id)
    }

    #[pyo3(signature = (parent_id, min, max, value,
                        width=None, height=Some(16.0),
                        width_fill=true, height_fill=false,
                        show=true, user_id=None, 
                        ))]
    fn add_progress_bar(&mut self,
                            parent_id: String,
                            min: f32,
                            max: f32,
                            value: f32,
                            // **above required
                            width: Option<f32>,
                            height: Option<f32>,
                            width_fill: bool,
                            height_fill: bool,
                            show: bool,
                            user_id: Option<String>,
                            ) -> PyResult<usize> 
    {

        self.id += 1;

        let width = get_width(width, width_fill);
        let height: Length = get_height(height, height_fill);

        set_state_of_widget(self.id, parent_id, user_id);

        let mut state = access_state();

        state.widgets.insert(self.id, IpgWidgets::IpgProgressBar(IpgProgressBar::new(   
                                                self.id,
                                                show,
                                                min,
                                                max,
                                                value,
                                                width,
                                                height,
                                            )));

        Ok(self.id)

    }

    #[pyo3(signature = (parent_id, labels, direction="vertical".to_string(), 
                        spacing= 10.0, padding=DEFAULT_PADDING.to_vec(), 
                        width=None, width_fill=false,
                        on_select=None, selected_index=None, 
                        size=20.0, text_spacing=15.0, text_size=16.0,
                        text_line_height=1.3, text_shaping="basic".to_string(), 
                        user_data=None, show=true, user_id=None, 
                        ))]
    fn add_radio(&mut self,
                    parent_id: String,
                    labels: Vec<String>,
                    //**above required
                    direction: String,
                    spacing: f32,
                    padding: Vec<f64>,
                    width: Option<f32>,
                    width_fill: bool,
                    on_select: Option<PyObject>,
                    selected_index: Option<usize>,
                    size: f32,
                    text_spacing: f32,
                    text_size: f32,
                    text_line_height: f32,
                    text_shaping: String,
                    user_data: Option<PyObject>,
                    show: bool,
                    user_id: Option<String>,
                    ) -> PyResult<usize>
    {

        self.id += 1;

        let mut cb_name: Option<String> = None;

        let direction = match direction.as_str() {
            "horizontal" => RadioDirection::Horizontal,
            "vertical" => RadioDirection::Vertical,
            _ => panic!("Radio direction must be either 'horizontal' or 'vertical'.")
        };

        let is_selected: Option<usize> = match selected_index {
            Some(index) => {
                if index > labels.len()-1 {
                    panic!("Radio selected_index is greater than the size of the labels")
                } else {
                    Some(index)
                }
            },
            None => None,
        };

        let padding = get_padding(padding);

        if on_select.is_some() {
            cb_name = Some("radio".to_string());
            let cb = CallBack{id: self.id, cb: on_select, name: cb_name.clone()};

            add_callback_to_mutex(cb);
        }

        let text_line_height = text::LineHeight::Relative(text_line_height);
        
        let text_shaping = get_shaping(text_shaping);

        let width = get_width(width, width_fill);

        set_state_of_widget(self.id, parent_id, user_id);

        let mut state = access_state();

        state.widgets.insert(self.id, IpgWidgets::IpgRadio(IpgRadio::new( 
                                        self.id,
                                        labels,
                                        direction,
                                        spacing,
                                        padding,
                                        show,
                                        user_data,
                                        is_selected,
                                        width,
                                        size,
                                        text_spacing,
                                        text_size,
                                        text_line_height,
                                        text_shaping,
                                        cb_name,
                                    )));
                                              
        Ok(self.id)

    }

    #[pyo3(signature = (parent_id, text, on_press=None, on_release=None, on_right_press=None, 
                        on_right_release=None, on_middle_press=None, on_middle_release=None, 
                        width=None, height=None, width_fill=false, height_fill=false, 
                        h_align="left".to_string(), v_align="top".to_string(), 
                        line_height=1.3, size=16.0, show=true, 
                        shaping="basic".to_string(), user_id=None, user_data=None))]
    fn add_selectable_text(&mut self,
                            parent_id: String,
                            text: String,
                            // ** above required
                            on_press: Option<PyObject>,
                            on_release: Option<PyObject>,
                            on_right_press: Option<PyObject>,
                            on_right_release: Option<PyObject>,
                            on_middle_press: Option<PyObject>,
                            on_middle_release: Option<PyObject>,
                            width: Option<f32>,
                            height: Option<f32>,
                            width_fill: bool,
                            height_fill: bool,
                            h_align: String,
                            v_align: String,
                            line_height: f32,
                            size: f32,
                            show: bool,
                            shaping: String,
                            user_id: Option<String>,
                            user_data: Option<PyObject>,
                            ) -> PyResult<usize> 
    {
    
        self.id += 1;

        let content = text.clone();

        let mut cb_on_press: Option<String> = None;

        if on_press.is_some() {
            cb_on_press = Some("cb_on_press".to_string());
        }
        add_callback_to_mutex(CallBack{id: self.id, cb: on_press, name: cb_on_press.clone()});
        
        let mut cb_on_release: Option<String> = None;

        if on_release.is_some() {
            cb_on_release = Some("cb_on_release".to_string());
        }
        add_callback_to_mutex(CallBack{id: self.id, cb: on_release, name: cb_on_release.clone()});

        let mut cb_on_right_press: Option<String> = None;

        if on_right_press.is_some() {
            cb_on_right_press = Some("cb_on_right_press".to_string());
        }
        add_callback_to_mutex(CallBack{id: self.id, cb: on_right_press, name: cb_on_right_press.clone()});

        let mut cb_on_right_release: Option<String> = None;

        if on_right_release.is_some() {
            cb_on_right_release = Some("cb_on_right_release".to_string());
        }
        add_callback_to_mutex(CallBack{id: self.id, cb: on_right_release, name: cb_on_right_release.clone()});
        
        let mut cb_on_middle_press: Option<String> = None;

        if on_middle_press.is_some() {
            cb_on_middle_press = Some("cb_on_middle_press".to_string());
        }
        add_callback_to_mutex(CallBack{id: self.id, cb: on_middle_press, name: cb_on_middle_press.clone()});
        
        let mut cb_on_middle_release: Option<String> = None;

        if on_middle_release.is_some() {
            cb_on_middle_release = Some("cb_on_middle_release".to_string());
        }
        add_callback_to_mutex(CallBack{id: self.id, cb: on_middle_release, name: cb_on_middle_release.clone()});
        
        let width = get_width(width, width_fill);
        let height = get_height(height, height_fill);

        let horizontal_alignment = get_horizontal_alignment(&h_align);

        let line_height = LineHeight::Relative(line_height);

        let shaping = get_shaping(shaping);

        let vertical_alignment = get_vertical_alignment(&v_align);

        set_state_of_widget(self.id, parent_id, user_id);

        let mut state = access_state();
        
        state.widgets.insert(self.id, IpgWidgets::IpgSelectableText(IpgSelectableText::new(
                                        self.id,
                                        content,
                                        width,
                                        height,
                                        horizontal_alignment,
                                        vertical_alignment,
                                        line_height,
                                        size,
                                        show,
                                        shaping,
                                        user_data,
                                        cb_on_press,
                                        cb_on_release,
                                        cb_on_right_press,
                                        cb_on_right_release,
                                        cb_on_middle_press,
                                        cb_on_middle_release,
                                    )));
            
        Ok(self.id)

    }

    #[pyo3(signature = (parent_id, min, max, step, value, 
                        width=None, height=None, 
                        width_fill=false, on_change=None, 
                        on_release=None, user_data=None, show=true, user_id=None, 
                        ))]
    fn add_slider(&mut self,
                        parent_id: String,
                        min: f32,
                        max: f32,
                        step: f32,
                        value: f32,
                        width: Option<f32>,
                        height: Option<f32>,
                        width_fill: bool,
                        on_change: Option<PyObject>,
                        on_release: Option<PyObject>,
                        user_data: Option<PyObject>,
                        show: bool,
                        user_id: Option<String>,
                        ) -> PyResult<usize> 
        {

        self.id += 1;

        let mut cb_name_change: Option<String> = None;
        let mut cb_name_release: Option<String> = None;

        if on_change.is_some() {
            cb_name_change = Some("on_change".to_string());
            let cb = CallBack{id: self.id, cb: on_change, name: cb_name_change.clone()};
            add_callback_to_mutex(cb);
        }
        if on_release.is_some() {
            cb_name_release = Some("on_release".to_string());
            let cb = CallBack{id: self.id, cb: on_release, name: cb_name_release.clone()};
            add_callback_to_mutex(cb);
        }
        
        let width = get_width(width, width_fill);
        let height = match height {
            Some(ht) => ht,
            None => 16.0,
        };

        set_state_of_widget(self.id, parent_id, user_id);

        let mut state = access_state();

        state.widgets.insert(self.id, IpgWidgets::IpgSlider(IpgSlider::new( 
                                            self.id,
                                            show,
                                            user_data,
                                            min,
                                            max,
                                            step,
                                            value,
                                            width,
                                            height,
                                            cb_name_change,
                                            cb_name_release,
                                        )));

        Ok(self.id)
    }

    #[pyo3(signature = (parent_id, width=None, height=None, 
                        width_fill=false, height_fill=false))]
    fn add_space(&mut self,
                        parent_id: String,
                        width: Option<f32>, 
                        height: Option<f32>,
                        width_fill: bool,
                        height_fill: bool,
                    ) -> PyResult<usize>
    {

        self.id += 1;

        let width = get_width(width, width_fill);
        let height = get_height(height, height_fill);

        let user_id: Option<String> = None;

        set_state_of_widget(self.id, parent_id, user_id);

        let mut state = access_state();

        state.widgets.insert(self.id, IpgWidgets::IpgSpace(IpgSpace::new( 
                                                    self.id,
                                                    width,
                                                    height,
                                                    )));

        Ok(self.id)
    } 

    #[pyo3(signature = (parent_id, title, data, width, height,
                        callback=None, column_widths=vec![], 
                        show=true, user_data=None, user_id=None))]
    fn add_table(&mut self,
                    parent_id: String,
                    title: String,
                    data: Vec<PyObject>,
                    width: f32,
                    height: f32,
                    // **above required
                    callback: Option<PyObject>,
                    column_widths: Vec<f32>,
                    show: bool,
                    user_data: Option<PyObject>,
                    user_id: Option<String>,
                ) -> PyResult<usize> 
    {

        self.id += 1;

        let mut cb_name: Option<String> = None;

        if callback.is_some() {
            cb_name = Some("table".to_string());
            let cb = CallBack{id: self.id, cb: callback, name: cb_name.clone()};

            add_callback_to_mutex(cb);
        }

        set_state_of_widget(self.id, parent_id, user_id);

        let mut state = access_state();

        state.widgets.insert(self.id, IpgWidgets::IpgTable(IpgTable::new( 
                                                    self.id,
                                                    title,
                                                    data,
                                                    width,
                                                    height,
                                                    column_widths,
                                                    show,
                                                    user_data,
                                                    cb_name,
                                                    )));

        Ok(self.id)

    }

    #[pyo3(signature = (parent_id, content, width=None, height=None,
                        width_fill=false, height_fill=false, 
                        h_align="left".to_string(), v_align="top".to_string(),
                        line_height=1.3, size=16.0, 
                        shaping="basic".to_string(), show=true, user_id=None 
                        ))]
    fn add_text(&mut self,
                    parent_id: String,
                    content: String,
                    // ** above required
                    width: Option<f32>,
                    height: Option<f32>,
                    width_fill: bool,
                    height_fill: bool,
                    h_align: String,
                    v_align: String,
                    line_height: f32,
                    size: f32,
                    shaping: String,
                    show: bool,
                    user_id: Option<String>,
                    ) -> PyResult<usize> 
    {
    
        self.id += 1;

        let width = get_width(width, width_fill);
        let height = get_height(height, height_fill);

        let horizontal_alignment = get_horizontal_alignment(&h_align);

        let line_height = LineHeight::Relative(line_height);

        let shaping = get_shaping(shaping);

        let vertical_alignment = get_vertical_alignment(&v_align);

        set_state_of_widget(self.id, parent_id, user_id);

        let mut state = access_state();
        
        state.widgets.insert(self.id, IpgWidgets::IpgText(IpgText::new(
                                        self.id,
                                        content,
                                        size,
                                        line_height,
                                        width,
                                        height,
                                        horizontal_alignment,
                                        vertical_alignment,
                                        // font: Font,
                                        shaping,
                                        show,
                                        // style: Style,
                                    )));
            
        Ok(self.id)

    }

    #[pyo3(signature = (parent_id, file_name, user_id=None))]
    fn add_text_editor(&mut self,
                            parent_id: String,
                            file_name: String,
                            user_id: Option<String>,
                        )  -> PyResult<usize>
    {
        self.id += 1;

        set_state_of_widget(self.id, parent_id, user_id);

        let mut state = access_state();
        
        state.widgets.insert(self.id, IpgWidgets::IpgTextEditor(IpgTextEditor::new(
                                                        self.id,
                                                        file_name,
                                                    )));

        Ok(self.id)
    }

    #[pyo3(signature = (parent_id, placeholder, on_input=None, on_submit=None, 
                        on_paste=None, width=None, width_fill=false, 
                        padding=DEFAULT_PADDING.to_vec(), 
                        size=20.0, line_height=("default".to_string(), 0.0), 
                        user_data=None, is_secure=false, show=true, user_id=None))]
    fn add_text_input(&mut self,
                            parent_id: String,
                            placeholder: String,
                            // **above required
                            on_input: Option<PyObject>,
                            on_submit: Option<PyObject>,
                            on_paste: Option<PyObject>,
                            width: Option<f32>,
                            width_fill: bool,
                            padding: Vec<f64>,
                            size: f32,
                            line_height: (String, f32),
                            user_data: Option<PyObject>,
                            is_secure: bool,
                            show: bool,
                            user_id: Option<String>,
                        ) -> PyResult<usize> 
    {

        self.id += 1;

        let mut cb_name_input: Option<String> = None;
        let mut cb_name_submit: Option<String> = None;
        let mut cb_name_paste: Option<String> = None;

        if on_input.is_some() {
            cb_name_input = Some("on_input".to_string());
            let cb = CallBack{id: self.id, cb: on_input, name: cb_name_input.clone()};
            add_callback_to_mutex(cb);
        }
        if on_submit.is_some() {
            cb_name_submit = Some("on_submit".to_string());
            let cb = CallBack{id: self.id, cb: on_submit, name: cb_name_submit.clone()};
            add_callback_to_mutex(cb);
        }

        if on_paste.is_some() {
            cb_name_paste = Some("on_paste".to_string());
            let cb = CallBack{id: self.id, cb: on_paste, name: cb_name_paste.clone()};
            add_callback_to_mutex(cb);
        }
        
        let padding = get_padding(padding);

        let width = get_width(width, width_fill);

        let line_height = get_line_height(line_height);

        set_state_of_widget(self.id, parent_id, user_id);

        let mut state = access_state();
        
        state.widgets.insert(self.id, IpgWidgets::IpgTextInput(IpgTextInput::new( 
                                                                self.id,
                                                                placeholder,
                                                                is_secure,
                                                                // font,
                                                                width,
                                                                padding,
                                                                size,
                                                                line_height,
                                                                user_data,
                                                                cb_name_input,
                                                                cb_name_submit,
                                                                cb_name_paste,
                                                                show,
                                                                )));

        Ok(self.id)
    }

    #[pyo3(signature = (enabled=false, on_key_press=None, on_key_release=None,
                        user_data=None))]
    fn add_event_keyboard(&mut self, 
                            enabled: bool,
                            on_key_press: Option<PyObject>,
                            on_key_release: Option<PyObject>,
                            user_data: Option<PyObject>,
                            )  -> PyResult<usize>
    {
        self.id += 1;

        match on_key_press {
            Some(kp) => {
                let cb = CallBackEvent{id: self.id, cb: kp, name: IpgEventCallbacks::OnKeyPress};
                add_callback_event_to_mutex(cb);
            },
            None => (),
        }
        match on_key_release {
            Some(kr) => {
                let cb = CallBackEvent{id: self.id, cb: kr, name: IpgEventCallbacks::OnKeyRelease};
                add_callback_event_to_mutex(cb);
            },
            None => (),
        }

        add_user_data_to_mutex(self.id, user_data);
        
        let mut state = access_state();

        state.events.push(IpgEvents::Keyboard(IpgKeyBoardEvent::new(
                                                                    self.id,
                                                                    enabled, 
                                                                    )));

        Ok(self.id)
    }

    #[pyo3(signature = (enabled=false, on_move=None, on_enter_window=None, 
                        on_exit_window=None, on_left_press=None, on_left_release=None,
                        on_middle_press=None, on_middle_release=None,
                        on_right_press=None, on_right_release=None,
                        on_middle_scroll=None,
                        user_data=None))]
    fn add_event_mouse(&mut self, 
                        enabled: bool,
                        on_move: Option<PyObject>,
                        on_enter_window: Option<PyObject>,
                        on_exit_window: Option<PyObject>,
                        on_left_press: Option<PyObject>,
                        on_left_release: Option<PyObject>,
                        on_middle_press: Option<PyObject>,
                        on_middle_release: Option<PyObject>,
                        on_right_press: Option<PyObject>,
                        on_right_release: Option<PyObject>,
                        on_middle_scroll: Option<PyObject>,
                        user_data: Option<PyObject>,
                        ) -> PyResult<usize>
    {
        self.id += 1;

        match on_move {
            Some(om) => {
                let cb = CallBackEvent{id: self.id, cb: om, name: IpgEventCallbacks::OnMove};
                add_callback_event_to_mutex(cb);
            },
            None => (),
        }
        match on_enter_window {
            Some(ew) => {
                let cb = CallBackEvent{id: self.id, cb: ew, name: IpgEventCallbacks::OnEnterWindow};
                add_callback_event_to_mutex(cb);
            },
            None => (),
        }
        match on_exit_window {
            Some(ew) => {
                let cb = CallBackEvent{id: self.id, cb: ew, name: IpgEventCallbacks::OnExitWindow};
                add_callback_event_to_mutex(cb);
            },
            None => (),
        }
        match on_left_press {
            Some(lp) => {
                let cb = CallBackEvent{id: self.id, cb: lp, name: IpgEventCallbacks::OnLeftPress};
                add_callback_event_to_mutex(cb);
            },
            None => (),
        }
        match on_left_release {
            Some(lr) => {
                let cb = CallBackEvent{id: self.id, cb: lr, name: IpgEventCallbacks::OnLeftRelease};
                add_callback_event_to_mutex(cb);
            },
            None => (),
        }
        match on_middle_press {
            Some(mp) => {
                let cb = CallBackEvent{id: self.id, cb: mp, name: IpgEventCallbacks::OnMiddlePress};
                add_callback_event_to_mutex(cb);
            },
            None => (),
        }
        match on_middle_release {
            Some(mr) => {
                let cb = CallBackEvent{id: self.id, cb: mr, name: IpgEventCallbacks::OnMiddleRelease};
                add_callback_event_to_mutex(cb);
            },
            None => (),
        }
        match on_right_press {
            Some(rp) => {
                let cb = CallBackEvent{id: self.id, cb: rp, name: IpgEventCallbacks::OnRightPress};
                add_callback_event_to_mutex(cb);
            },
            None => (),
        }
        match on_right_release {
            Some(rr) => {
                let cb = CallBackEvent{id: self.id, cb: rr, name: IpgEventCallbacks::OnRightRelease};
                add_callback_event_to_mutex(cb);
            },
            None => (),
        }
        match on_middle_scroll {
            Some(ms) => {
                let cb = CallBackEvent{id: self.id, cb: ms, name: IpgEventCallbacks::OnMiddleScrollLine};
                add_callback_event_to_mutex(cb);
            },
            None => (),
        }

        add_user_data_to_mutex(self.id, user_data);
        
        let mut state = access_state();

        state.events.push(IpgEvents::Mouse(IpgMouseEvent::new(
                                                            self.id,
                                                            enabled, 
                                                            )));

        Ok(self.id)
    }

    #[pyo3(signature = (enabled=false, on_open=None, on_close=None, 
                        on_moved=None, on_resized=None, user_data=None))]
    fn add_event_window(&mut self,
                        enabled: bool,
                        on_open: Option<PyObject>,
                        on_close: Option<PyObject>,
                        on_moved: Option<PyObject>,
                        on_resized: Option<PyObject>,
                        user_data: Option<PyObject>,
                        ) -> PyResult<usize>
    {
        self.id += 1;

        match on_open {
            Some(oo) => {
                let cb = CallBackEvent{id: self.id, cb: oo, name: IpgEventCallbacks::WindowOnOpened};
                add_callback_event_to_mutex(cb);
            },
            None => (),
        }
        match on_close {
            Some(oc) => {
                let cb = CallBackEvent{id: self.id, cb: oc, name: IpgEventCallbacks::WindowOnClosed};
                add_callback_event_to_mutex(cb);
            },
            None => (),
        }
        match on_moved {
            Some(om) => {
                let cb = CallBackEvent{id: self.id, cb: om, name: IpgEventCallbacks::WindowOnMoved};
                add_callback_event_to_mutex(cb);
            },
            None => (),
        }
        match on_resized {
            Some(or) => {
                let cb = CallBackEvent{id: self.id, cb: or, name: IpgEventCallbacks::WindowOnResized};
                add_callback_event_to_mutex(cb);
            },
            None => (),
        }

        add_user_data_to_mutex(self.id, user_data);
        
        let mut state = access_state();

        state.events.push(IpgEvents::Window(IpgWindowEvent::new(
                                                            self.id,
                                                            enabled, 
                                                            )));

        Ok(self.id)
    }

    #[pyo3(signature = (window_id, id))]
    fn delete_item(&self, window_id: String, id: usize) 
    {

        let mut state = access_state();

        let iced_id = match state.windows_str_ids.get(&window_id) {
            Some(id) => id.clone(),
            None => panic!("Window_id {} not found in delete_item", window_id)
        };

        let ipg_ids = match state.ids.get_mut(&iced_id) {
            Some(ids) => ids,
            None => panic!("Ids not found for window_id {} in delete_item", window_id)
        };

        let mut index: i32 = -1;

        for (i, ipg_id) in ipg_ids.iter().enumerate() {
            if ipg_id.id == id {
                index = i as i32;
                break;
            }
        }

        if index == -1 {
            panic!("item with id {id} could not be found to delete")
        }

        ipg_ids.remove(index as usize);

        state.widgets.remove(&id);

    }


    #[pyo3(signature = (id, item, value))]
    fn update_item(&self, id: usize, item: String, value: PyObject) {

        let mut value_str: Option<String> = None;
        let mut value_bool: Option<bool> = None;
        let mut value_i64: Option<i64> = None;
        let mut value_f64: Option<f64> = None;
        let mut value_tup_str_i64: Option<(String, i64)> = None;
        let mut value_tup_str_f64: Option<(String, f64)> = None;
        let mut value_vec_f64: Option<Vec<f64>> = None;
        
        Python::with_gil(|py| {

            let res = value.extract::<String>(py);
            if !res.is_err() {
                value_str = match res {
                    Ok(res) => Some(res),
                    Err(_) => None,
                } 
            }
            
            let res = value.extract::<bool>(py);
            if !res.is_err() {
                value_bool = match res {
                    Ok(res) => Some(res),
                    Err(_) => None,
                } 
            }

            let res = value.extract::<i64>(py);
            if !res.is_err() { 
                value_i64 = match res {
                    Ok(res) => Some(res),
                    Err(_) => None,
                } 
            } 
            
            let res = value.extract::<f64>(py);
            if !res.is_err() { 
                value_f64 = match res {
                    Ok(res) => Some(res),
                    Err(_) => None,
                } 
            }

            let res = value.extract::<(String, i64)>(py);
            if !res.is_err() { 
                value_tup_str_i64 = match res {
                    Ok(res) => Some(res),
                    Err(_) => None,
                } 
            }

            let res = value.extract::<(String, f64)>(py);
            if !res.is_err() { 
                value_tup_str_f64 = match res {
                    Ok(res) => Some(res),
                    Err(_) => None,
                } 
            }

            let res = value.extract::<Vec<f64>>(py);
            if !res.is_err() { 
                value_vec_f64 = match res {
                    Ok(res) => Some(res),
                    Err(_) => None,
                } 
            }

        });

        let mut state = access_state();

        let widget_opt = state.widgets.get_mut(&id);

        let widget = match widget_opt {
            Some(w) => w,
            None => panic!("Widget with id {id} could not be updated"),
        };
        
        match widget {
            IpgWidgets::IpgButton(btn) => {
                button_item_update(btn,
                                    item,
                                    value_str,
                                    value_bool,
                                    value_i64,
                                    value_f64,
                                    value_tup_str_i64,
                                    value_tup_str_f64,
                                    value_vec_f64,
                                    );
                drop(state);
                return
            },
            IpgWidgets::IpgCard(crd) => {
                card_item_update(
                                    crd,
                                    item,
                                    value_str,
                                    value_bool,
                                    value_i64,
                                    value_f64,
                                    value_tup_str_i64,
                                    value_tup_str_f64,
                                    value_vec_f64,
                                );
drop(state);
return
            },
            IpgWidgets::IpgCheckBox(chk) => {
                checkbox_item_update(chk,
                                    item,
                                    value_str,
                                    value_bool,
                                    value_i64,
                                    value_f64,
                                    value_tup_str_i64,
                                    value_tup_str_f64,
                                    value_vec_f64,
                                    );
                drop(state);
                return
            },
            IpgWidgets::IpgColorPicker(cp) => {
                color_picker_item_update(cp,
                                    item,
                                    value_str,
                                    value_bool,
                                    value_i64,
                                    value_f64,
                                    value_tup_str_i64,
                                    value_tup_str_f64,
                                    value_vec_f64,
                                    );
                drop(state);
                return
            },
            IpgWidgets::IpgDatePicker(dp) => {
                date_picker_item_update(dp,
                                    item,
                                    value_str,
                                    value_bool,
                                    value_i64,
                                    value_f64,
                                    value_tup_str_i64,
                                    value_tup_str_f64,
                                    value_vec_f64,
                                    );
                drop(state);
                return
            },
            IpgWidgets::IpgImage(img) => (),
            IpgWidgets::IpgMenuBar(_wid) => (),
            IpgWidgets::IpgMenuItem(_wid) => (),
            IpgWidgets::IpgPickList(_wid) => (),
            IpgWidgets::IpgProgressBar(pb) => {
                progress_bar_item_update(pb,
                                    item,
                                    value_str,
                                    value_bool,
                                    value_i64,
                                    value_f64,
                                    value_tup_str_i64,
                                    value_tup_str_f64,
                                    value_vec_f64,
                                    );
                drop(state);
                return
            },
            IpgWidgets::IpgRadio(_wid) => (),
            IpgWidgets::IpgSelectableText(_wid) => (),
            IpgWidgets::IpgSlider(_wid) => (),
            IpgWidgets::IpgSpace(_wid) => (),
            IpgWidgets::IpgTable(_wid) => (),
            IpgWidgets::IpgText(wid) => {
                if item == "content".to_string() {
                    wid.content = match value_str {
                        Some(str) => str,
                        None => panic!("A string value is needed to update text"),
                    };
                } 
                if item == "show".to_string() {
                    wid.show = match value_bool {
                        Some(bl) => bl,
                        None => panic!("The show parameter must be of type bool")
                    };
                }
                drop(state);
                return
            },
            IpgWidgets::IpgTextEditor(_wid) => (),
            IpgWidgets::IpgTextInput(_wid) => (),
        }

    }

    #[pyo3(signature = (id, title=None, headers=None, data=None, user_id=None, callback=None))]
    fn update_table(&self, 
                            id: Option<usize>,
                            title: Option<String>,
                            headers: Option<Vec<String>>,
                            data: Option<&PyList>,
                            user_id: Option<String>,
                            callback: Option<PyObject>,
                    ) 
    {
        
        let id: usize = match id {
            Some(id) => id,
            None => 0
        };

        let user_id = match user_id {
            Some(id) => id,
            None => "".to_string()
        };
        
        if &id == &0 && &user_id == &"".to_string() {
            panic!("You must supply either an id or user_id to update the table.")
        }

        let title = match title {
            Some(title) => title,
            None => "".to_string(),
        };

        let headers = match headers {
            Some(hd) => hd,
            None => vec![],
        };
        
        let data = py_extract_list(data);
        
        // let mut app_temp = access_temp();
        // let input = data.last().unwrap();
        // app_temp.my_text = Some(input[1].clone());
        // drop(app_temp);

        let mut cb_name: Option<String> = None;
        if callback.is_some() {
            cb_name = Some("update_table".to_string());
            let cb = CallBack{id, cb: callback, name: cb_name.clone()};

            add_callback_to_mutex(cb);
        }
        
    }

}

fn set_state_of_container(
                            id: usize, 
                            window_id: String, 
                            container_id: Option<String>, 
                            parent_id: String,
                            user_id: Option<String>) 
{

    let state = access_state();

    let wnd_id_usize = match state.windows_str_ids.get(&window_id) {
        Some(id) => id.clone(),
        None => panic!("The main window id could not be found using window_id {}", window_id)
    };
    drop(state);

    check_for_dup_container_ids(wnd_id_usize, container_id.clone());
    
    check_for_dup_user_ids(wnd_id_usize, &user_id);

    let mut state = access_state();

    match container_id.clone() {
        Some(container_id_str) => state.container_wnd_str_ids.insert(container_id_str, window_id),
        None => None,
    };
    
    let parent_uid = find_parent_uid(state.ids.get(&wnd_id_usize).unwrap(), parent_id.clone());
    
    state.ids.get_mut(&wnd_id_usize).unwrap().push(IpgIds{id, parent_uid, container_id,
                                                        parent_id, user_id, is_container: true});

    state.container_ids.get_mut(&wnd_id_usize).unwrap().push(id);

    drop(state);

}

fn set_state_of_widget(
                        id: usize,  
                        parent_id: String,
                        user_id: Option<String>)
{
    let state = access_state();

    let wnd_id_str = match state.container_wnd_str_ids.get(&parent_id) {
        Some(id) => id.clone(),
        None => panic!("The main window id could not be found using parent_id {}, check that your parent_id matches a contaier", parent_id)
    };

    let wnd_id_usize = match state.windows_str_ids.get(&wnd_id_str) {
        Some(id) => id.clone(),
        None => panic!("window id {} could not be found in set_state_of_widget", wnd_id_str),
    };

    drop(state);
   
    check_for_dup_user_ids(wnd_id_usize, &user_id);

    let mut state = access_state();

    let parent_uid = find_parent_uid(state.ids.get(&wnd_id_usize).unwrap(), parent_id.clone());
    
    state.ids.get_mut(&wnd_id_usize).unwrap().push(IpgIds{id, parent_uid, container_id: None,
                                                        parent_id, user_id, is_container: false});

    drop(state);
}

pub fn py_extract_list(list_opt: Option<&PyList>) -> Vec<Vec<String>> {

    match list_opt {
        Some(list) => {
            
            let mut data_str: Vec<Vec<String>> = vec![];  
            pyo3::prepare_freethreaded_python();
            let _ = Python::with_gil(|_py| -> PyResult<()> {
                
                for lst in list {
                    let dat: PyResult<Vec<String>> = lst
                        .iter()?
                        .map(|i| i.and_then(PyAny::extract::<String>))
                        .collect();
                    
                    let data = match dat {
                        Ok(dat) => dat,
                        Err(_E) => panic!("Could not extract list")
                    };
                    data_str.push(data);
                }
                Ok(())
            });
            return data_str
            },

            None => return vec![vec![]],
        };
}


#[pymodule]
fn icedpygui(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<IPG>()?;
    Ok(())
}



fn add_callback_to_mutex(callback: CallBack) {
    let mut app_cbs = access_callbacks();

        app_cbs.callbacks.push(callback);

        drop(app_cbs);
}

fn add_callback_event_to_mutex(callback: CallBackEvent) {
    let mut app_cbs = access_callbacks();

        app_cbs.cb_events.push(callback);

        drop(app_cbs);
}

fn add_user_data_to_mutex(id: usize, user_data: Option<PyObject>) {
    let mut cb = access_callbacks();

    cb.user_data.push((id, user_data));

    drop(cb);
}

fn add_radio_to_mutex(id: usize, group_id: String) -> usize {
    let mut cb = access_callbacks();

    let mut found = false;
    let mut index = 0;

    for radio in cb.radios.iter_mut() {
        if group_id == radio.0 {
            found = true;
            radio.1.push(id);
            index = radio.1.len();
        }
    }

    if !found {
        cb.radios.push((group_id, vec![id]));
        index = 1;
    }

    drop(cb);

    index -= 1;

    if index > RADIO_SIZE {
        panic!("The number of radio buttons in a group is currently limited to {RADIO_SIZE}, put in a request for additional ones")
    }

    index

}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn find_parent_uid(ipg_ids: &Vec<IpgIds>, parent_id: String) -> usize {

    for id in ipg_ids.iter() {
        if id.container_id == Some(parent_id.clone()) {
            return id.id
        }
    }
    panic!("Parent id {:?} not found in function find_parent_uid()", parent_id)
}

fn create_tips_columns(table_columns: &Vec<String>,
                        tooltip_columns: &Vec<String>, 
                        mut tooltips: Vec<Vec<String>>) 
                        -> Vec<Vec<String>> 
{

    tooltips.reverse();

    let mut tips: Vec<Vec<String>> = vec![];

    for _ in table_columns {
        tips.push(vec![]);
    }

    for (index, name) in table_columns.iter().enumerate() {
        if tooltip_columns.contains(name) {
            match tooltips.pop() {
                Some(tip) => tips[index] = tip,
                None => {},
            }
        }
    }

    tips
}
