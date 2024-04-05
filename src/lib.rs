#![allow(non_snake_case)]

use ipg_widgets::ipg_rule::IpgRule;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyList, PyModule};
use pyo3::PyObject;

use iced::multi_window::Application;
use iced::window::{self, Position};
use iced::{Color, Font, Length, Point, Settings, Size};
use iced::widget::text::{self, LineHeight};

use core::panic;
use std::iter::Iterator;
use std::collections::HashMap;

mod app;
use app::{App, Flags};

mod ipg_widgets;
mod iced_widgets;

use crate::iced_widgets::scrollable::Direction;

use ipg_widgets::ipg_button::{button_item_update, IpgButton, IpgButtonArrows, IpgButtonStyles, IpgButtonParams};
use ipg_widgets::ipg_card::{card_item_update, IpgCard, IpgCardStyles, IpgCardParams};
use ipg_widgets::ipg_checkbox::{checkbox_item_update, IpgCheckBox, IpgCheckboxParams};
use ipg_widgets::ipg_color_picker::{IpgColorPicker, color_picker_item_update};
use ipg_widgets::ipg_column::IpgColumn;
use ipg_widgets::ipg_container::IpgContainer;
use ipg_widgets::ipg_date_picker::{date_picker_item_update, IpgDatePicker, IpgDatePickerParams};
use ipg_widgets::ipg_events::{IpgEventCallbacks, IpgEvents, IpgKeyBoardEvent, 
                                IpgMouseEvent, IpgWindowEvent};
use ipg_widgets::ipg_image::{image_item_update, IpgImage, IpgImageParams};
use ipg_widgets::ipg_menu::{menu_item_update, IpgMenu, IpgMenuParams, IpgMenuSepTypes};
use ipg_widgets::ipg_pane_grid::{IpgPane, IpgPaneGrid};
use ipg_widgets::ipg_pick_list::{pick_list_item_update, IpgPickList, IpgPickListParams};
use ipg_widgets::ipg_progress_bar::{progress_bar_item_update, IpgProgressBar, IpgProgressBarParams};
use ipg_widgets::ipg_radio::{radio_item_update, IpgRadio, IpgRadioDirection, IpgRadioParams};
use ipg_widgets::ipg_row::IpgRow;
use ipg_widgets::ipg_scrollable::IpgScrollable;
use ipg_widgets::ipg_selectable_text::IpgSelectableText;
use ipg_widgets::ipg_slider::IpgSlider;
use ipg_widgets::ipg_space::IpgSpace;
use ipg_widgets::ipg_table::IpgTable;
use ipg_widgets::ipg_text::{text_item_update, IpgText, IpgTextParams};
use ipg_widgets::ipg_text_editor::IpgTextEditor;
use ipg_widgets::ipg_text_input::IpgTextInput;
use ipg_widgets::ipg_toggle::{IpgToggler, IpgTogglerParams};
use ipg_widgets::ipg_tool_tip::IpgToolTip;
use ipg_widgets::ipg_window::{IpgWindow, IpgWindowThemes};
use ipg_widgets::ipg_enums::{IpgContainers, IpgWidgets};

use ipg_widgets::helpers::{check_for_dup_container_ids,  
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

use std::sync::{Mutex, MutexGuard};
use once_cell::sync::Lazy;



#[derive(Debug, Clone)]
pub struct CallBackEvent {
    id: usize,
    cb: PyObject,
    name: IpgEventCallbacks,
}

#[derive(Debug)]
pub struct Callbacks {
    callbacks: Lazy<HashMap<(usize, String), Option<PyObject>>>,
    cb_events: Vec<CallBackEvent>,
    user_data: Vec<(usize, Option<PyObject>)>,
}

pub static CALLBACKS: Mutex<Callbacks> = Mutex::new(Callbacks {
    callbacks: Lazy::new(||HashMap::new()),
    cb_events: vec![],
    user_data: vec![],
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
    pub text_buffer: Lazy<[u8; 1_000_000]>,
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
        text_buffer: Lazy::new(||[0_u8; 1_000_000]),
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
    pub is_container: bool, // if container to get all ids of parents of containers
}


#[pyclass]
pub struct IPG {
    id: usize,
    window_id: usize,
    gen_ids: Vec<usize>,
    group_index: usize,
}

#[pymethods]
impl IPG {
    #[new]
    fn new() -> IPG {
        IPG {
            id: 0,
            window_id: 0,
            gen_ids: vec![],
            group_index: 0,
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

    #[pyo3(signature = ())]
    fn generate_id(&mut self) -> PyResult<usize>
    {
        self.id += 1;
        self.gen_ids.push(self.id);
        Ok(self.id)
    }

    #[pyo3(signature = (window_id, title, width, height, pos_x=None, pos_y=None,
                        pos_centered=false, resizable=true, 
                        theme=None, exit_on_close=true, on_resize=None, 
                        show=true, debug=false, user_data=None))]
    fn add_window(&mut self,
                        window_id: String, 
                        title: String, 
                        width: f32, 
                        height: f32, 
                        pos_x: Option<f32>,
                        pos_y: Option<f32>,
                        pos_centered: bool,
                        resizable: bool,
                        theme: Option<PyObject>,
                        exit_on_close: bool,
                        on_resize: Option<PyObject>,
                        show: bool,
                        debug: bool,
                        user_data: Option<PyObject>,
                    ) -> PyResult<usize>
    {
        self.id += 1;

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

        if on_resize.is_some() {
            add_callback_to_mutex(self.id, "on_resize".to_string(), on_resize);
        }

        state.windows_str_ids.insert(window_id.clone(), self.window_id);

        state.ids.insert(self.window_id, vec![IpgIds{id: self.id, parent_uid: 0, container_id: Some(window_id.clone()),
                                                parent_id: "".to_string(), is_container: true}]);

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
                                            theme.clone(), 
                                            resizable,
                                            visible,
                                            debug,
                                            user_data.clone(),
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
                                        theme, 
                                        resizable,
                                        show,
                                        debug,
                                        user_data,
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
                       ))]
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

        set_state_of_container(self.id, window_id, Some(container_id), prt_id);

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
                        spacing=20.0, show=true,
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

        set_state_of_container(self.id, window_id, Some(container_id), prt_id);

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

    #[pyo3(signature = (window_id, container_id, add_direction, ratio, parent_id=None))]
    fn add_pane(&mut self,
                    window_id: String, 
                    container_id: String,
                    add_direction: String,
                    ratio: f32,
                    parent_id: Option<String>, 
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

        set_state_of_container(self.id, window_id, Some(container_id), prt_id);

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
                        show=true,
                        ))]
    fn add_pane_grid(&mut self,
                        window_id: String,
                        container_id: String,
                        // above required
                        parent_id: Option<String>,
                        width: Option<f32>,
                        height: Option<f32>,
                        width_fill: bool,
                        height_fill: bool,
                        spacing: f32,
                        padding: Vec<f64>,
                        show: bool,
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

        set_state_of_container(self.id, window_id, Some(container_id), prt_id);

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
                        show=true,
                        ))]
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

        set_state_of_container(self.id, window_id, Some(container_id), prt_id);

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
                        user_data=None,
                        ))]
    fn add_scrollable(&mut self,
                            window_id: String,
                            container_id: String,
                            // above required
                            parent_id: Option<String>,
                            mut width: Option<f32>,
                            mut height: Option<f32>,
                            width_fill: bool,
                            height_fill: bool,
                            direction: String,
                            on_scroll: Option<PyObject>,
                            user_data: Option<PyObject>,
                            ) -> PyResult<usize>
    {
        self.id += 1;

        if on_scroll.is_some() {
            add_callback_to_mutex(self.id, "on_scroll".to_string(), on_scroll);
        }
        // For scrollable the fill doesn't work well so as long as the fixed is
        // larger than the window, it will fill wahtever space is left.
        if width_fill {width = Some(10_000.0)}
        if height_fill {height = Some(10_000.0)}
        let width = get_width(width, false);
        let height = get_height(height, false);

        let direction: Direction = get_scroll_direction(Some(direction));

        let prt_id = match parent_id {
            Some(id) => id,
            None => window_id.clone(),
        };

        set_state_of_container(self.id, window_id, Some(container_id), prt_id);

        let mut state = access_state();
      
        state.containers.insert(self.id, IpgContainers::IpgScrollable(IpgScrollable::new( 
                                                    self.id,
                                                    width,
                                                    height,
                                                    direction,
                                                    user_data,
                                                    // style,
                                                    )));

        Ok(self.id)

    }

    #[pyo3(signature = (window_id, container_id, position, text_to_display, 
                        parent_id=None, gap=10, padding=0.0, 
                        snap_within_viewport=true, 
                        style="box".to_string()
                        ))]
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

        set_state_of_container(self.id, window_id, Some(container_id), prt_id);

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
    
    #[pyo3(signature = (parent_id, label, id=None, on_press=None, 
                        width=None, height=None, width_fill=false, 
                        height_fill=false, padding=vec![10.0], corner_radius=15.0, 
                        style=None, arrow_style=None, user_data=None, 
                        show=true, 
                        ))]
    fn add_button(&mut self,
                        parent_id: String,
                        label: String,
                        // ** above required
                        id: Option<usize>,
                        on_press: Option<PyObject>,
                        width: Option<f32>,
                        height: Option<f32>,
                        width_fill: bool,
                        height_fill: bool,
                        padding: Vec<f64>,
                        corner_radius: f32,
                        style: Option<PyObject>,
                        arrow_style: Option<PyObject>,
                        user_data: Option<PyObject>,
                        show: bool,
                        ) -> PyResult<usize> 
    {
        let id = self.get_id(id);

        if on_press.is_some() {
            add_callback_to_mutex(id, "on_press".to_string(), on_press);
        }

        let width = get_width(width, width_fill);
        let height = get_height(height, height_fill);

        let padding = get_padding(padding);

        // // iced button style has to be converted to a string
        // // because in can't be put into the mutex.
        // let style_opt = match style {
        //     Some(st) => try_extract_button_style(st),
        //     None => None,
        // };

        set_state_of_widget(id, parent_id);

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgButton(IpgButton::new(
                                                id,
                                                show,
                                                user_data,
                                                label,
                                                width,
                                                height,
                                                padding,
                                                corner_radius,
                                                style,
                                                arrow_style,                              
                                                )));
        
        Ok(id)
    
    }

    #[pyo3(signature = (parent_id, head, body, is_open=true, minmax_id=None, foot=None, 
                        id=None, close_size=20.0, on_close=None, 
                        width=None, height=None, width_fill=false, height_fill=false, 
                        max_width=f32::INFINITY, max_height=f32::INFINITY, 
                        padding_head=vec![5.0], padding_body=vec![5.0], padding_foot=vec![5.0],
                        style=None, user_data=None))]
    fn add_card(&mut self,
                parent_id: String, 
                head: String,
                body: String,
                // above required
                is_open: bool,
                minmax_id: Option<usize>,
                foot: Option<String>,
                id: Option<usize>,
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
                style: Option<PyObject>,
                user_data: Option<PyObject>, 
                ) -> PyResult<usize> 
    {
        let id = self.get_id(id);

        if on_close.is_some() {
            add_callback_to_mutex(id, "on_close".to_string(), on_close);
        }

        let width = get_width(width, width_fill);
        let height = get_height(height, height_fill);

        let padding_head = get_padding(padding_head);
        let padding_body = get_padding(padding_body);
        let padding_foot = get_padding(padding_foot);

        set_state_of_widget(id, parent_id);

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgCard(IpgCard::new(
                                                    id,
                                                    is_open,
                                                    user_data,
                                                    minmax_id,
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
                                                )));

        Ok(id)

    }

    #[pyo3(signature = (parent_id, id=None, on_toggle=None, is_checked=false, 
                        label="".to_string(), width=None, width_fill=false, 
                        size=16.0, spacing=20.0, text_line_height=1.3, 
                        text_shaping="basic".to_string(),text_size=16.0, icon_x=false, 
                        icon_size=25.0, user_data=None, show=true, style=None,
                        ))] 
    fn add_checkbox(&mut self,
                        parent_id: String,
                        // ** above required
                        id: Option<usize>,
                        on_toggle: Option<PyObject>,
                        is_checked: bool,
                        label: String,
                        width: Option<f32>,
                        width_fill: bool,
                        size: f32,
                        spacing: f32,
                        text_line_height: f32,
                        text_shaping: String,
                        text_size: f32,
                        icon_x: bool,
                        icon_size: f32,
                        user_data: Option<PyObject>,
                        show: bool,
                        style: Option<PyObject>,
                        ) -> PyResult<usize> 
    {
        let id = self.get_id(id);

        if on_toggle.is_some() {
            add_callback_to_mutex(id, "on_toggle".to_string(), on_toggle);
        }
       
        let text_shaping = get_shaping(text_shaping);

        let text_line_height = text::LineHeight::Relative(text_line_height);

        let width = get_width(width, width_fill);
        
        set_state_of_widget(id, parent_id);

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgCheckBox(IpgCheckBox::new(
                                                    id,
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
                                                    icon_x,
                                                    icon_size,
                                                    style,
                                                    )));

        Ok(id)

    }

    #[pyo3(signature = (parent_id, label="Set Color".to_string(), 
                        id=None, on_submit=None, show=true, 
                        start_up_color=[0.5, 0.2, 0.7, 1.0], 
                        width=None, height=None, width_fill=false, height_fill=false,
                        padding=vec![10.0], corner_radius=0.0, 
                        style=None, user_data=None, 
                        ))]
    fn add_color_picker(
                    &mut self,
                    parent_id: String,
                    // ** above required
                    label: String,
                    id: Option<usize>,
                    on_submit: Option<PyObject>,
                    show: bool,
                    start_up_color: [f32; 4],
                    width: Option<f32>,
                    height: Option<f32>,
                    width_fill: bool,
                    height_fill: bool,
                    padding: Vec<f64>,
                    corner_radius: f32,
                    style: Option<PyObject>,
                    user_data: Option<PyObject>,
                    ) -> PyResult<usize> 
    {
        let id = self.get_id(id);

        if on_submit.is_some() {
            add_callback_to_mutex(id, "on_submit".to_string(), on_submit);
        }

        let color = Color::from_rgba(start_up_color[0], 
                                            start_up_color[1], 
                                            start_up_color[2], 
                                            start_up_color[3]);

        let width = get_width(width, width_fill);
        let height = get_height(height, height_fill);

        let padding = get_padding(padding);

        set_state_of_widget(id, parent_id);

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgColorPicker(
                            IpgColorPicker::new(
                                            id,
                                            show,
                                            color,
                                            user_data,
                                            label,
                                            width,
                                            height,
                                            padding,
                                            corner_radius,
                                            style,                              
                                        )));

        Ok(id)

    }

    #[pyo3(signature = (parent_id, label="Calendar".to_string(), id=None,
                        size_factor=1.0, padding=vec![5.0], on_submit=None, 
                        user_data=None, show=false,
                        ))]
    fn add_date_picker(&mut self,
                        parent_id: String,
                        // ** above required
                        label: String,
                        id: Option<usize>,
                        size_factor: f32,
                        padding: Vec<f64>,
                        on_submit: Option<PyObject>,
                        user_data: Option<PyObject>,
                        show: bool,
                        ) -> PyResult<usize> 
    {
        let id = self.get_id(id);

        if size_factor < 1.0 {
            panic!("Size factor for date picker must be > 1.0")
        }

        if on_submit.is_some() {
            add_callback_to_mutex(id, "on_submit".to_string(), on_submit);
        }

        let padding = get_padding(padding);

        set_state_of_widget(id, parent_id);

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgDatePicker(IpgDatePicker::new(
                                            id,
                                            label,
                                            size_factor,
                                            padding,
                                            show,
                                            user_data,                            
                                        )));
        Ok(id)
    }

    #[pyo3(signature = (parent_id, image_path, id=None, 
                        width=None, width_fill=false, 
                        height=None, height_fill=false, 
                        padding=vec![5.0], on_press=None, on_release=None,
                        on_right_press=None, on_right_release=None,
                        on_middle_press=None, on_middle_release=None,
                        on_enter=None, on_move=None, on_exit=None, 
                        user_data=None, show=false,
                        ))]
fn add_image(&mut self,
                    parent_id: String,
                    image_path: String,
                    // above required
                    id: Option<usize>,
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
                    on_enter: Option<PyObject>,
                    on_move: Option<PyObject>,
                    on_exit: Option<PyObject>,
                    user_data: Option<PyObject>,
                    show: bool,
                    ) -> PyResult<usize>
{
    let id = self.get_id(id);

    if on_press.is_some() {
        add_callback_to_mutex(id, "on_press".to_string(), on_press);
    }
    
    if on_release.is_some() {
        add_callback_to_mutex(id, "event_name".to_string(), on_release);
    }
    
    if on_right_press.is_some() {
        add_callback_to_mutex(id, "on_right_press".to_string(), on_right_press);
    }
    
    if on_right_release.is_some() {
        add_callback_to_mutex(id, "on_right_release".to_string(), on_right_release);
    }
    
    if on_middle_press.is_some() {
        add_callback_to_mutex(id, "on_middle_press".to_string(), on_middle_press);
    }
    
    if on_middle_release.is_some() {
        add_callback_to_mutex(id, "on_middle_release".to_string(), on_middle_release);
    }
    
    if on_enter.is_some() {
        add_callback_to_mutex(id, "on_enter".to_string(), on_enter);
    }
    
    if on_move.is_some() {
        add_callback_to_mutex(id, "on_move".to_string(), on_move);
    }
    
    if on_exit.is_some() {
        add_callback_to_mutex(id, "on_exit".to_string(), on_exit);
    }
    
    let width = get_width(width, width_fill);
    let height = get_height(height, height_fill);

    let padding = get_padding(padding);

    set_state_of_widget(id, parent_id);

    let mut state = access_state();

    state.widgets.insert(id, IpgWidgets::IpgImage(IpgImage::new(
                                                id,
                                                image_path,
                                                width,
                                                height,
                                                padding,
                                                show,
                                                user_data,
                                            )));

        Ok(id)
    }

    #[pyo3(signature = (parent_id, items, widths, spacing, 
                        on_select=None, separators=None, sep_types=None, 
                        sep_label_names=None, user_data=None, id=None))]
    fn add_menu(&mut self, 
                    parent_id: String, 
                    items: PyObject,
                    widths: Vec<f32>,
                    spacing: Vec<f32>,
                    on_select: Option<PyObject>,
                    separators: Option<Vec<(usize, usize, IpgMenuSepTypes)>>,
                    sep_types: Option<Vec<IpgMenuSepTypes>>,
                    sep_label_names: Option<Vec<String>>,
                    user_data: Option<PyObject>,
                    id: Option<usize>,
                ) -> PyResult<usize> 
    {
        let id = self.get_id(id);

        if on_select.is_some() {
            add_callback_to_mutex(id, "on_select".to_string(), on_select);
        }

        set_state_of_widget(id, parent_id);

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgMenu(IpgMenu::new(
                                                                id,
                                                                items,
                                                                widths,
                                                                spacing,
                                                                separators,
                                                                sep_types,
                                                                sep_label_names,
                                                                user_data,
                                                                )));

        Ok(id)
    }

    #[pyo3(signature = (parent_id, options, id=None, on_select=None, 
                        width=None, width_fill=false, padding=vec![5.0],  
                        placeholder=None, selected=None, text_size=None, 
                        text_line_height=1.3, text_shaping="basic".to_string(), 
                        user_data=None, show=true,
                        ))]
    fn add_pick_list(&mut self,
                        parent_id: String,
                        options: PyObject,
                        // **above required
                        id: Option<usize>,
                        on_select: Option<PyObject>,
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
                    ) -> PyResult<usize>
    {

        let id = self.get_id(id);

        if on_select.is_some() {
            add_callback_to_mutex(id, "on_select".to_string(), on_select);
        }

        let padding = get_padding(padding);

        let text_line_height = text::LineHeight::Relative(text_line_height);
        
        let text_shaping = get_shaping(text_shaping);

        let width = get_width(width, width_fill);

        set_state_of_widget(id, parent_id);

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgPickList(IpgPickList::new(  
                                                        id,
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
                                                    )));

        Ok(id)
    }

    #[pyo3(signature = (parent_id, min, max, value,
                        id=None, width=None, height=Some(16.0), 
                        width_fill=true, height_fill=false,
                        show=true, 
                        ))]
    fn add_progress_bar(&mut self,
                            parent_id: String,
                            min: f32,
                            max: f32,
                            value: f32,
                            // **above required
                            id: Option<usize>,
                            width: Option<f32>,
                            height: Option<f32>,
                            width_fill: bool,
                            height_fill: bool,
                            show: bool,
                            ) -> PyResult<usize> 
    {

        let id = self.get_id(id);

        let width = get_width(width, width_fill);
        let height: Length = get_height(height, height_fill);

        set_state_of_widget(id, parent_id);

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgProgressBar(IpgProgressBar::new(   
                                                id,
                                                show,
                                                min,
                                                max,
                                                value,
                                                width,
                                                height,
                                            )));

        Ok(id)

    }

    #[pyo3(signature = (parent_id, labels, id=None,
                        direction=IpgRadioDirection::Vertical, 
                        spacing= 10.0, padding=DEFAULT_PADDING.to_vec(), 
                        width=None, width_fill=false, height=None, height_fill=false,
                        on_select=None, selected_index=None, 
                        size=20.0, text_spacing=15.0, text_size=16.0,
                        text_line_height=1.3, text_shaping="basic".to_string(), 
                        user_data=None, show=true, 
                        ))]
    fn add_radio(&mut self,
                    parent_id: String,
                    labels: Vec<String>,
                    //**above required
                    id: Option<usize>,
                    direction: IpgRadioDirection,
                    spacing: f32,
                    padding: Vec<f64>,
                    width: Option<f32>,
                    width_fill: bool,
                    height: Option<f32>,
                    height_fill: bool,
                    on_select: Option<PyObject>,
                    selected_index: Option<usize>,
                    size: f32,
                    text_spacing: f32,
                    text_size: f32,
                    text_line_height: f32,
                    text_shaping: String,
                    user_data: Option<PyObject>,
                    show: bool,
                    ) -> PyResult<usize>
    {

        let id = self.get_id(id);

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
            add_callback_to_mutex(id, "on_select".to_string(), on_select);
        }

        let text_line_height = text::LineHeight::Relative(text_line_height);
        
        let text_shaping = get_shaping(text_shaping);

        let width = get_width(width, width_fill);
        let height = get_height(height, height_fill);

        set_state_of_widget(id, parent_id);

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgRadio(IpgRadio::new( 
                                        id,
                                        labels,
                                        direction,
                                        spacing,
                                        padding,
                                        show,
                                        user_data,
                                        is_selected,
                                        width,
                                        height,
                                        size,
                                        text_spacing,
                                        text_size,
                                        text_line_height,
                                        text_shaping,
                                        self.group_index,
                                    )));
        self.group_index += 1;                                      
        Ok(id)

    }

    #[pyo3(signature = (parent_id, width=None, width_fill=true))]
    fn add_rule_horizontal(&mut self, 
                            parent_id: String,
                            width: Option<f32>,
                            width_fill: bool, 
                            ) -> PyResult<usize> 
    {
        let id: Option<usize> = None;
        let id = self.get_id(id);

        let width = get_width(width, width_fill);
        let height: Length = get_height(None, false);  // not used
        let rule_type = "h".to_string();

        set_state_of_widget(id, parent_id);

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgRule(IpgRule::new(
                                                        id,
                                                        width,
                                                        height,
                                                        rule_type,
                                                        )));

        Ok(id)
    }

    #[pyo3(signature = (parent_id, height=None, height_fill=true))]
    fn add_rule_vertical(&mut self, 
                            parent_id: String, 
                            height: Option<f32>,
                            height_fill: bool,
                            ) -> PyResult<usize> 
    {
        let id: Option<usize> = None;
        let id = self.get_id(id);

        let width = get_width(None, false);  //Not used
        let height: Length = get_height(height, height_fill);
        let rule_type = "v".to_string();

        set_state_of_widget(id, parent_id);

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgRule(IpgRule::new(
                                                        id,
                                                        width,
                                                        height,
                                                        rule_type,
                                                        )));

        Ok(id)
    }

    #[pyo3(signature = (parent_id, text, id=None, on_press=None, on_release=None, 
                        on_right_press=None, on_right_release=None, on_middle_press=None, 
                        on_middle_release=None, on_move=None, on_enter=None, on_exit=None, 
                        width=None, height=None, width_fill=false, height_fill=false, 
                        h_align="left".to_string(), v_align="top".to_string(), 
                        line_height=1.3, size=16.0, show=true, 
                        shaping="basic".to_string(), user_data=None
                        ))]
    fn add_selectable_text(&mut self,
                            parent_id: String,
                            text: String,
                            // ** above required
                            id: Option<usize>,
                            on_press: Option<PyObject>,
                            on_release: Option<PyObject>,
                            on_right_press: Option<PyObject>,
                            on_right_release: Option<PyObject>,
                            on_middle_press: Option<PyObject>,
                            on_middle_release: Option<PyObject>,
                            on_move: Option<PyObject>,
                            on_enter: Option<PyObject>,
                            on_exit: Option<PyObject>,
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
                            user_data: Option<PyObject>,
                            ) -> PyResult<usize> 
    {
    
        let id = self.get_id(id);

        let content = text.clone();

        if on_press.is_some() {
            add_callback_to_mutex(id, "on_press".to_string(), on_press);
        }
        
        if on_release.is_some() {
            add_callback_to_mutex(id, "on_release".to_string(), on_release);
        }
        
        if on_right_press.is_some() {
            add_callback_to_mutex(id, "on_right_press".to_string(), on_right_press);
        }
        
        if on_right_release.is_some() {
            add_callback_to_mutex(id, "on_right_release".to_string(), on_right_release);
        }
        
        if on_middle_press.is_some() {
            add_callback_to_mutex(id, "on_middle_press".to_string(), on_middle_press);
        }
        
        if on_middle_release.is_some() {
            add_callback_to_mutex(id, "on_middle_release".to_string(), on_middle_release);
        }
        
        if on_enter.is_some() {
            add_callback_to_mutex(id, "on_enter".to_string(), on_enter);
        }
        
        if on_move.is_some() {
            add_callback_to_mutex(id, "on_move".to_string(), on_move);
        }
        
        if on_exit.is_some() {
            add_callback_to_mutex(id, "on_exit".to_string(), on_exit);
        }
        
        let width = get_width(width, width_fill);
        let height = get_height(height, height_fill);

        let horizontal_alignment = get_horizontal_alignment(&h_align);

        let line_height = LineHeight::Relative(line_height);

        let shaping = get_shaping(shaping);

        let vertical_alignment = get_vertical_alignment(&v_align);

        set_state_of_widget(id, parent_id);

        let mut state = access_state();
        
        state.widgets.insert(id, IpgWidgets::IpgSelectableText(IpgSelectableText::new(
                                        id,
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
                                    )));
            
        Ok(id)

    }

    #[pyo3(signature = (parent_id, min, max, step, value, 
                        id=None, width=None, height=None, 
                        width_fill=false, on_change=None, 
                        on_release=None, user_data=None, show=true, 
                        ))]
    fn add_slider(&mut self,
                        parent_id: String,
                        min: f32,
                        max: f32,
                        step: f32,
                        value: f32,
                        id: Option<usize>,
                        width: Option<f32>,
                        height: Option<f32>,
                        width_fill: bool,
                        on_change: Option<PyObject>,
                        on_release: Option<PyObject>,
                        user_data: Option<PyObject>,
                        show: bool,
                        ) -> PyResult<usize> 
        {

        let id = self.get_id(id);

        if on_change.is_some() {
            add_callback_to_mutex(id, "on_change".to_string(), on_change);
        }
        if on_release.is_some() {
            add_callback_to_mutex(id, "on_release".to_string(), on_release);
        }
        
        let width = get_width(width, width_fill);
        let height = match height {
            Some(ht) => ht,
            None => 16.0,
        };

        set_state_of_widget(id, parent_id);

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgSlider(IpgSlider::new( 
                                            id,
                                            show,
                                            user_data,
                                            min,
                                            max,
                                            step,
                                            value,
                                            width,
                                            height,
                                        )));

        Ok(id)
    }

    #[pyo3(signature = (parent_id, id=None, width=None, height=None, 
                        width_fill=false, height_fill=false))]
    fn add_space(&mut self,
                        parent_id: String,
                        id: Option<usize>,
                        width: Option<f32>, 
                        height: Option<f32>,
                        width_fill: bool,
                        height_fill: bool,
                    ) -> PyResult<usize>
    {

        let id = self.get_id(id);

        let width = get_width(width, width_fill);
        let height = get_height(height, height_fill);

        set_state_of_widget(id, parent_id);

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgSpace(IpgSpace::new( 
                                                    id,
                                                    width,
                                                    height,
                                                    )));

        Ok(id)
    } 

    #[pyo3(signature = (parent_id, title, data, width, height,
                        id=None, callback=None, column_widths=vec![], 
                        show=true, user_data=None))]
    fn add_table(&mut self,
                    parent_id: String,
                    title: String,
                    data: Vec<PyObject>,
                    width: f32,
                    height: f32,
                    // **above required
                    id: Option<usize>,
                    callback: Option<PyObject>,
                    column_widths: Vec<f32>,
                    show: bool,
                    user_data: Option<PyObject>,
                ) -> PyResult<usize> 
    {

        let id = self.get_id(id);

        if callback.is_some() {
            add_callback_to_mutex(id, "table".to_string(), callback);
        }

        set_state_of_widget(id, parent_id);

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgTable(IpgTable::new( 
                                                    id,
                                                    title,
                                                    data,
                                                    width,
                                                    height,
                                                    column_widths,
                                                    show,
                                                    user_data,
                                                    )));

        Ok(id)

    }

    #[pyo3(signature = (parent_id, content, id=None, width=None, 
                        height=None, width_fill=false, height_fill=false, 
                        h_align="left".to_string(), v_align="top".to_string(),
                        line_height=1.3, size=16.0, 
                        shaping="basic".to_string(), show=true,
                        ))]
    fn add_text(&mut self,
                    parent_id: String,
                    content: String,
                    // ** above required
                    id: Option<usize>,
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
                    ) -> PyResult<usize> 
    {
    
        let id = self.get_id(id);

        let width = get_width(width, width_fill);
        let height = get_height(height, height_fill);

        let horizontal_alignment = get_horizontal_alignment(&h_align);

        let line_height = LineHeight::Relative(line_height);

        let shaping = get_shaping(shaping);

        let vertical_alignment = get_vertical_alignment(&v_align);

        set_state_of_widget(id, parent_id);

        let mut state = access_state();
        
        state.widgets.insert(id, IpgWidgets::IpgText(IpgText::new(
                                        id,
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
            
        Ok(id)

    }

    #[pyo3(signature = (parent_id, file_name, id=None))]
    fn add_text_editor(&mut self,
                            parent_id: String,
                            file_name: String,
                            id: Option<usize>,
                        )  -> PyResult<usize>
    {
        let id = self.get_id(id);

        set_state_of_widget(id, parent_id);

        let mut state = access_state();
        
        state.widgets.insert(id, IpgWidgets::IpgTextEditor(IpgTextEditor::new(
                                                        id,
                                                        file_name,
                                                        )));

        Ok(id)
    }

    #[pyo3(signature = (parent_id, placeholder, id=None,
                        on_input=None, on_submit=None, 
                        on_paste=None, width=None, width_fill=false, 
                        padding=DEFAULT_PADDING.to_vec(), 
                        size=20.0, line_height=("default".to_string(), 0.0), 
                        user_data=None, is_secure=false, show=true,
                        ))]
    fn add_text_input(&mut self,
                            parent_id: String,
                            placeholder: String,
                            // **above required
                            id: Option<usize>,
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
                        ) -> PyResult<usize> 
    {

        let id = self.get_id(id);

        if on_input.is_some() {
            add_callback_to_mutex(id, "on_input".to_string(), on_input);
        }
        if on_submit.is_some() {
            add_callback_to_mutex(id, "on_submit".to_string(), on_submit);
        }

        if on_paste.is_some() {
            add_callback_to_mutex(id, "on_paste".to_string(), on_paste);
        }
        
        let padding = get_padding(padding);

        let width = get_width(width, width_fill);

        let line_height = get_line_height(line_height);

        set_state_of_widget(id, parent_id);

        let mut state = access_state();
        
        state.widgets.insert(id, IpgWidgets::IpgTextInput(IpgTextInput::new( 
                                                                id,
                                                                placeholder,
                                                                is_secure,
                                                                // font,
                                                                width,
                                                                padding,
                                                                size,
                                                                line_height,
                                                                user_data,
                                                                show,
                                                                )));

        Ok(id)
    }

    #[pyo3(signature = (parent_id, label=None, id=None, toggled=None, 
                        width=None, width_fill=false, 
                        user_data=None, show=true, 
                        ))]
    fn add_toggler(&mut self,
                        parent_id: String,
                        // ** above required
                        label: Option<String>,
                        id: Option<usize>,
                        toggled: Option<PyObject>,
                        width: Option<f32>,
                        width_fill: bool,
                        user_data: Option<PyObject>,
                        show: bool,
                        ) -> PyResult<usize> 
    {
        let id = self.get_id(id);

        if toggled.is_some() {
            add_callback_to_mutex(id, "toggled".to_string(), toggled);
        }

        let width = get_width(width, width_fill);

        set_state_of_widget(id, parent_id);

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgToggler(IpgToggler::new(
                                                id,
                                                show,
                                                user_data,
                                                label,
                                                width,                           
                                                )));
        
        Ok(id)
    
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
                let cb = CallBackEvent{id: self.id, cb:kp, name: IpgEventCallbacks::OnKeyPress};
                add_callback_event_to_mutex(cb);
            },
            None => (),
        }
        match on_key_release {
            Some(kr) => {
                let cb = CallBackEvent{id: self.id, cb:kr, name: IpgEventCallbacks::OnKeyRelease};
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
    fn update_item(&self, id: usize, item: PyObject, value: PyObject) {
        
        let mut state = access_state();

        let widget_opt = state.widgets.get_mut(&id);

        let widget = match widget_opt {
            Some(w) => w,
            None => panic!("Widget with id {id} could not be updated"),
        };
        
        match widget {
            IpgWidgets::IpgButton(btn) => {
                button_item_update(btn, item, value);
                drop(state);
            },
            IpgWidgets::IpgCard(crd) => {
                card_item_update(crd, item, value);
                drop(state);
            },
            IpgWidgets::IpgCheckBox(chk) => {
                checkbox_item_update(chk, item, value);
                drop(state);
            },
            IpgWidgets::IpgColorPicker(cp) => {
                color_picker_item_update(cp, item, value);
                drop(state);
            },
            IpgWidgets::IpgDatePicker(dp) => {
                date_picker_item_update(dp, item, value);
                drop(state);
            },
            IpgWidgets::IpgImage(img) => {
                image_item_update(img, item, value);
                drop(state);
            },
            IpgWidgets::IpgMenu(menu) => {
                menu_item_update(menu, item, value);
                drop(state);
            },
            IpgWidgets::IpgPickList(pl) => {
                pick_list_item_update(pl, item, value);
                drop(state);
            },
            IpgWidgets::IpgProgressBar(pb) => {
                progress_bar_item_update(pb, item, value);
                drop(state);
            },
            IpgWidgets::IpgRadio(rd) => {
                radio_item_update(rd, item, value);
                drop(state);
            },
            IpgWidgets::IpgRule(_) => (),
            IpgWidgets::IpgSelectableText(_wid) => (),
            IpgWidgets::IpgSlider(_wid) => (),
            IpgWidgets::IpgSpace(_wid) => (),
            IpgWidgets::IpgTable(_wid) => (),
            IpgWidgets::IpgText(txt) => {
                text_item_update(txt, item, value);
                drop(state);
            },
            IpgWidgets::IpgTextEditor(_wid) => (),
            IpgWidgets::IpgTextInput(_wid) => (),
            IpgWidgets::IpgToggler(_wid) => (),
        }

    }

    #[pyo3(signature = (id, title=None, headers=None, data=None, user_id=None, on_update=None))]
    fn update_table(&self, 
                            id: Option<usize>,
                            title: Option<String>,
                            headers: Option<Vec<String>>,
                            data: Option<&PyList>,
                            user_id: Option<String>,
                            on_update: Option<PyObject>,
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

        let _title = match title {
            Some(title) => title,
            None => "".to_string(),
        };

        let _headers = match headers {
            Some(hd) => hd,
            None => vec![],
        };
        
        let _data = py_extract_list(data);
        
        if on_update.is_some() {
            add_callback_to_mutex(id, "on_update".to_string(), on_update);
        }
        
    }

    fn get_id(&mut self, gen_id: Option<usize>) -> usize
    {
        // When an id is generated, it is put into the self.gen_ids.
        // The below checks that if the user is using the id field for the
        // widget, then it must be in the gen_id vec otherwise, the user must 
        // have used his own integer for the id parameter.
        match gen_id {
            Some(id) => {
                if self.gen_ids.contains(&id) {
                    id
                } else {
                    panic!("The id parameter for widgets must use a generate id.  This id {id} was not found in the gen_id list.")
                }
            }
            None => {
                self.id += 1;
                self.id
                },
        }
    }
}


#[pymodule]
fn icedpygui(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<IPG>()?;
    m.add_class::<IpgButtonStyles>()?;
    m.add_class::<IpgButtonArrows>()?;
    m.add_class::<IpgButtonParams>()?;
    m.add_class::<IpgCardStyles>()?;
    m.add_class::<IpgCardParams>()?;
    m.add_class::<IpgCheckboxParams>()?;
    m.add_class::<IpgDatePickerParams>()?;
    m.add_class::<IpgImageParams>()?;
    m.add_class::<IpgMenuParams>()?;
    m.add_class::<IpgMenuSepTypes>()?;
    m.add_class::<IpgPickListParams>()?;
    m.add_class::<IpgProgressBarParams>()?;
    m.add_class::<IpgRadioDirection>()?;
    m.add_class::<IpgRadioParams>()?;
    m.add_class::<IpgTextParams>()?;
    m.add_class::<IpgTogglerParams>()?;
    m.add_class::<IpgWindowThemes>()?;
    Ok(())
}


fn set_state_of_container(
                            id: usize, 
                            window_id: String, 
                            container_id: Option<String>, 
                            parent_id: String,
                            ) 
{

    let state = access_state();

    let wnd_id_usize = match state.windows_str_ids.get(&window_id) {
        Some(id) => id.clone(),
        None => panic!("The main window id could not be found using window_id {}", window_id)
    };
    drop(state);

    check_for_dup_container_ids(wnd_id_usize, container_id.clone());
    
    let mut state = access_state();

    match container_id.clone() {
        Some(container_id_str) => state.container_wnd_str_ids.insert(container_id_str, window_id),
        None => None,
    };
    
    let parent_uid = find_parent_uid(state.ids.get(&wnd_id_usize).unwrap(), parent_id.clone());
    
    state.ids.get_mut(&wnd_id_usize).unwrap().push(IpgIds{id, parent_uid, container_id,
                                                        parent_id, is_container: true});

    state.container_ids.get_mut(&wnd_id_usize).unwrap().push(id);

    drop(state);

}

fn set_state_of_widget(
                        id: usize,  
                        parent_id: String,
                        )
{
    let state = access_state();

    let wnd_id_str = match state.container_wnd_str_ids.get(&parent_id) {
        Some(id) => id.clone(),
        None => panic!("The main window id could not be found using parent_id {}, check that your parent_id matches a container", parent_id)
    };

    let wnd_id_usize = match state.windows_str_ids.get(&wnd_id_str) {
        Some(id) => id.clone(),
        None => panic!("window id {} could not be found in set_state_of_widget", wnd_id_str),
    };

    drop(state);
   
    let mut state = access_state();

    let parent_uid = find_parent_uid(state.ids.get(&wnd_id_usize).unwrap(), parent_id.clone());
    
    state.ids.get_mut(&wnd_id_usize).unwrap().push(IpgIds{id, parent_uid, container_id: None,
                                                        parent_id, is_container: false});

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


fn add_callback_to_mutex(id: usize, event_name: String, py_obj: Option<PyObject>) {
    let mut app_cbs = access_callbacks();

        app_cbs.callbacks.insert((id, event_name), py_obj);

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


fn find_parent_uid(ipg_ids: &Vec<IpgIds>, parent_id: String) -> usize {

    for id in ipg_ids.iter() {
        if id.container_id == Some(parent_id.clone()) {
            return id.id
        }
    }
    panic!("Parent id {:?} not found in function find_parent_uid()", parent_id)
}

pub fn delete_item(_id: usize) {

}
