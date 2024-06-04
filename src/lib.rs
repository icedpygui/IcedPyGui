//!lib for all of the python callable functions using pyo3
#![allow(non_snake_case)]

use iced::border::Radius;
use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::PyObject;

use iced::multi_window::Application;
use iced::window::{self, Position};
use iced::{Color, Font, Length, Point, Settings, Size, Theme};
use iced::widget::text::{self, LineHeight};

use core::panic;
use std::iter::Iterator;
use std::collections::HashMap;

mod app;
use app::{App, Flags};

mod ipg_widgets;
mod iced_widgets;
mod iced_aw_widgets;
mod graphics;
mod style;


use ipg_widgets::ipg_button::{button_item_update, IpgButton, IpgButtonArrows, IpgButtonParams, IpgButtonStyle};
use ipg_widgets::ipg_card::{card_item_update, IpgCard, IpgCardStyles, IpgCardParams};
use ipg_widgets::ipg_checkbox::{checkbox_item_update, IpgCheckBox, IpgCheckboxParams};
use ipg_widgets::ipg_column::{IpgColumn, IpgColumnAlignment};
use ipg_widgets::ipg_container::{IpgContainer, IpgContainerAlignment};
use ipg_widgets::ipg_date_picker::{date_picker_item_update, IpgDatePicker, IpgDatePickerParams};
use ipg_widgets::ipg_events::{IpgEventCallbacks, IpgEvents, IpgKeyBoardEvent, IpgMouseEvent, IpgWindowEvent};
use ipg_widgets::ipg_image::{image_item_update, IpgImage, IpgImageContentFit, IpgImageFilterMethod, IpgImageParams, IpgImageRotation};
use ipg_widgets::ipg_menu::{menu_item_update, IpgMenu, IpgMenuBarStyle, IpgMenuItemStyle, IpgMenuItemType, IpgMenuParams, IpgMenuSepTypes};
use ipg_widgets::ipg_mousearea::{mousearea_item_update, IpgMouseArea, IpgMouseAreaParams};
use ipg_widgets::ipg_pick_list::{pick_list_item_update, IpgPickList, IpgPickListParams};
use ipg_widgets::ipg_progress_bar::{progress_bar_item_update, IpgProgressBar, IpgProgressBarParams};
use ipg_widgets::ipg_radio::{radio_item_update, IpgRadio, IpgRadioDirection, IpgRadioParams};
use ipg_widgets::ipg_row::{IpgRow, IpgRowAlignment};
use ipg_widgets::ipg_rule::IpgRule;
use ipg_widgets::ipg_scrollable::{scrollable_item_update, IpgScrollable, IpgScrollableAlignment, 
                                    IpgScrollableDirection, IpgScrollableParams};
use ipg_widgets::ipg_selectable_text::{selectable_text_item_update, IpgSelectableText, 
    IpgSelectableTextHorAlign, IpgSelectableTextParams, IpgSelectableTextVertAlign};
use ipg_widgets::ipg_slider::{slider_item_update, IpgSlider, IpgSliderParams};
use ipg_widgets::ipg_space::IpgSpace;
use ipg_widgets::ipg_svg::{svg_item_update, IpgSvg, IpgSvgContentFit, IpgSvgParams, IpgSvgRotation};
use ipg_widgets::ipg_table::{IpgTable, TableRowHighLight, TableWidget};
use ipg_widgets::ipg_text::{text_item_update, IpgText, IpgTextParams};
use ipg_widgets::ipg_text_input::{text_input_item_update, IpgTextInput, IpgTextInputParams};
use ipg_widgets::ipg_timer::{timer_item_update, IpgTimer, IpgTimerParams};
use ipg_widgets::ipg_toggle::{toggler_item_update, IpgToggler, IpgTogglerParams};
use ipg_widgets::ipg_tool_tip::IpgToolTip;
use ipg_widgets::ipg_window::{get_iced_window_theme, window_item_update, IpgWindow, 
    IpgWindowParams, IpgWindowThemes};
use ipg_widgets::ipg_enums::{IpgContainers, IpgWidgets};

use ipg_widgets::helpers::{check_for_dup_container_ids, 
    get_height, get_horizontal_alignment, 
    get_line_height, get_padding, get_shaping, get_vertical_alignment, get_width};

use graphics::colors::{get_color, IpgColor};
use style::styling::{StyleBackground, StyleBorder, StyleIconColor, StyleShadow, StyleTextColor};

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
    pub ids: Lazy<HashMap<usize, Vec<IpgIds>>>,  // <window_id=usize, Vec<IpgIds=structure>>
    
    pub containers: Lazy<HashMap<usize, IpgContainers>>,
    pub container_ids: Lazy<HashMap<usize, Vec<usize>>>,  // <window_id=usize, vec<container_id=usize>>
    pub container_str_ids: Lazy<HashMap<String, usize>>, // get container usize id based on container string
    pub container_wnd_str_ids: Lazy<HashMap<String, String>>, // get window string id based on container string id
    pub container_window_usize_ids: Lazy<HashMap<usize, usize>>, //get window usize id based on container usize id

    pub widgets: Lazy<HashMap<usize, IpgWidgets>>,
    pub widget_container_ids: Lazy<HashMap<usize, String>>, //widget_id=usize, container_id=String
    
    pub windows: Vec<IpgWindow>,
    pub windows_str_ids: Lazy<HashMap<String, usize>>,  // <window_id=str, window_id=usize>
    pub window_debug: Lazy<HashMap<window::Id, (usize, bool)>>, // (usize, bool) = (wid, debug)
    pub window_theme: Lazy<HashMap<window::Id, (usize, Theme)>>, //(usize, Theme) = (wid, window Theme)
    
    pub events: Vec<IpgEvents>,
    
    pub styling_background: Lazy<HashMap<String, StyleBackground>>,
    pub styling_border: Lazy<HashMap<String, StyleBorder>>,
    pub styling_icon_color: Lazy<HashMap<String, StyleIconColor>>,
    pub styling_shadow: Lazy<HashMap<String, StyleShadow>>,
    pub styling_text_color: Lazy<HashMap<String, StyleTextColor>>,
    
}

pub static STATE: Mutex<State> = Mutex::new(
    State {
        ids: Lazy::new(||HashMap::new()),
        
        containers: Lazy::new(||HashMap::new()),
        container_ids: Lazy::new(||HashMap::new()),
        container_str_ids: Lazy::new(||HashMap::new()),
        container_wnd_str_ids: Lazy::new(||HashMap::new()),
        container_window_usize_ids: Lazy::new(||HashMap::new()),

        widgets: Lazy::new(||HashMap::new()),
        widget_container_ids: Lazy::new(||HashMap::new()),

        
        windows: vec![],
        windows_str_ids: Lazy::new(||HashMap::new()),
        window_debug: Lazy::new(||HashMap::new()),
        window_theme: Lazy::new(||HashMap::new()),
        
        events: vec![],
        
        styling_background: Lazy::new(||HashMap::new()),
        styling_border: Lazy::new(||HashMap::new()),
        styling_icon_color: Lazy::new(||HashMap::new()),
        styling_shadow: Lazy::new(||HashMap::new()),
        styling_text_color: Lazy::new(||HashMap::new()),
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
                        theme=IpgWindowThemes::Dark, exit_on_close=true, on_resize=None, 
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
                        theme: IpgWindowThemes,
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

        let iced_theme = get_iced_window_theme(theme);

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
        // TODO: A windows container is probably not needed but some suttle issues arise when not used.
        // Will need to work through it in the near future.  At the onset, used only one window then
        // iced made multiwindow so sort of patch it to work but need to revisit it.
        state.containers.insert(self.id, IpgContainers::IpgWindow(IpgWindow::new(
                                            self.id,
                                            self.window_id,
                                            window_id.clone(),
                                            title.clone(), 
                                            width, 
                                            height, 
                                            window_position,
                                            exit_on_close,
                                            iced_theme.clone(), 
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
                                        iced_theme, 
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
                        center_xy=false, clip=false, max_height=f32::INFINITY, max_width=f32::INFINITY,
                        align_x=IpgContainerAlignment::Start, align_y=IpgContainerAlignment::Start,
                        padding=DEFAULT_PADDING.to_vec(), show=true, style_background=None, 
                        style_border=None, style_shadow=None, style_text_color=None
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
                        center_xy: bool,
                        clip: bool,
                        max_height: f32,
                        max_width: f32,
                        align_x: IpgContainerAlignment,
                        align_y: IpgContainerAlignment, 
                        padding: Vec<f64>, 
                        show: bool,
                        style_background: Option<String>, 
                        style_border: Option<String>, 
                        style_shadow: Option<String>,
                        style_text_color: Option<String>
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

        set_state_of_container(self.id, window_id.clone(), Some(container_id.clone()), prt_id);

        let mut state = access_state();

        set_state_cont_wnd_ids(&mut state, &window_id, container_id, self.id, "add_container".to_string());

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
                                                center_xy,
                                                clip,
                                                style_background, 
                                                style_border, 
                                                style_shadow,
                                                style_text_color,
                                            )));

        drop(state);

        Ok(self.id)

    }


    #[pyo3(signature = (window_id, container_id, parent_id=None,
                        align_items=IpgColumnAlignment::Start, width=None, height=None,
                        width_fill=false, height_fill=false,
                        max_width=f32::INFINITY, padding=DEFAULT_PADDING.to_vec(), 
                        spacing=20.0, clip=false, show=true,
                        ))]
    fn add_column(&mut self,
                        window_id: String,
                        container_id: String,
                        // **above required
                        parent_id: Option<String>,
                        align_items: IpgColumnAlignment,
                        width: Option<f32>,
                        height: Option<f32>,
                        width_fill: bool,
                        height_fill: bool,
                        max_width: f32,
                        padding: Vec<f64>,
                        spacing: f32,
                        clip: bool,
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

        set_state_of_container(self.id, window_id.clone(), Some(container_id.clone()), prt_id);

        let mut state = access_state();

        set_state_cont_wnd_ids(&mut state, &window_id, container_id, self.id, "add_column".to_string());

        state.containers.insert(self.id, IpgContainers::IpgColumn(IpgColumn::new(
                                        self.id,  
                                        show, 
                                        spacing, 
                                        padding, 
                                        width, 
                                        height, 
                                        max_width, 
                                        align_items,
                                        clip,
                                    )));
    
    Ok(self.id)

    }

    #[pyo3(signature = (window_id, container_id, parent_id=None, 
                        gen_id=None, on_press=None, on_release=None,
                        on_right_press=None, on_right_release=None,
                        on_middle_press=None, on_middle_release=None,
                        on_enter=None, on_move=None, on_exit=None,
                        show=true, user_data=None))]
    fn add_mousearea(&mut self,
                        window_id: String,
                        container_id: String,
                        // required above
                        parent_id: Option<String>,
                        gen_id: Option<usize>,
                        on_press: Option<PyObject>,
                        on_release: Option<PyObject>,
                        on_right_press: Option<PyObject>,
                        on_right_release: Option<PyObject>,
                        on_middle_press: Option<PyObject>,
                        on_middle_release: Option<PyObject>,
                        on_enter: Option<PyObject>,
                        on_move: Option<PyObject>,
                        on_exit: Option<PyObject>,
                        show: bool,
                        user_data: Option<PyObject>,
                        ) -> PyResult<usize>
    {
        self.id += 1;

        let id = self.get_id(gen_id);

        let prt_id = match parent_id {
            Some(id) => id,
            None => window_id.clone(),
        };

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

        set_state_of_container(self.id, window_id.clone(), Some(container_id.clone()), prt_id);

        let mut state = access_state();

        set_state_cont_wnd_ids(&mut state, &window_id, container_id, self.id, "add_mousearea".to_string());

        state.containers.insert(self.id, IpgContainers::IpgMouseArea(IpgMouseArea::new(
                                    self.id,  
                                    show, 
                                    user_data
                                )));

        Ok(self.id)

    }

    #[pyo3(signature = (window_id, container_id, parent_id=None,
                        align_items=IpgRowAlignment::Start, width=None, height=None, 
                        width_fill=false, height_fill=false,
                        padding=DEFAULT_PADDING.to_vec(), spacing=20.0, clip=false,
                        show=true,
                        ))]
    fn add_row(&mut self,
                    window_id: String,
                    container_id: String,
                    // required above
                    parent_id: Option<String>,
                    align_items: IpgRowAlignment,
                    width: Option<f32>,
                    height: Option<f32>,
                    width_fill: bool,
                    height_fill: bool,
                    padding: Vec<f64>,
                    spacing: f32,
                    clip: bool,
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

        set_state_of_container(self.id, window_id.clone(), Some(container_id.clone()), prt_id);

        let mut state = access_state();

        set_state_cont_wnd_ids(&mut state, &window_id, container_id, self.id, "add_row".to_string());

        state.containers.insert(self.id, IpgContainers::IpgRow(IpgRow::new(
                                    self.id,  
                                    show, 
                                    spacing, 
                                    padding, 
                                    width, 
                                    height, 
                                    align_items,
                                    clip,
                                )));
                                
        Ok(self.id)

    }

    #[pyo3(signature = (window_id, container_id, parent_id=None,
                        width=None, height=None, width_fill=false, height_fill=false, 
                        direction=IpgScrollableDirection::Vertical, h_bar_width=10.0, 
                        h_bar_margin=0.0, h_scroller_width=10.0, 
                        h_bar_alignment=IpgScrollableAlignment::Start,
                        v_bar_width=10.0, v_bar_margin=0.0, v_scroller_width=10.0, 
                        v_bar_alignment=IpgScrollableAlignment::Start,
                        on_scroll=None, user_data=None,
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
                            direction: IpgScrollableDirection,
                            h_bar_width: f32,
                            h_bar_margin: f32,
                            h_scroller_width: f32,
                            h_bar_alignment: IpgScrollableAlignment,
                            v_bar_width: f32,
                            v_bar_margin: f32,
                            v_scroller_width: f32,
                            v_bar_alignment: IpgScrollableAlignment,
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

        let prt_id = match parent_id {
            Some(id) => id,
            None => window_id.clone(),
        };

        set_state_of_container(self.id, window_id.clone(), Some(container_id.clone()), prt_id);

        let mut state = access_state();

        set_state_cont_wnd_ids(&mut state, &window_id, container_id, self.id, "add_scrollable".to_string());
      
        state.containers.insert(self.id, IpgContainers::IpgScrollable(IpgScrollable::new( 
                                                    self.id,
                                                    width,
                                                    height,
                                                    direction,
                                                    h_bar_width,
                                                    h_bar_margin,
                                                    h_scroller_width,
                                                    h_bar_alignment,
                                                    v_bar_width,
                                                    v_bar_margin,
                                                    v_scroller_width,
                                                    v_bar_alignment,
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

        set_state_of_container(self.id, window_id.clone(), Some(container_id.clone()), prt_id);

        let mut state = access_state();

        set_state_cont_wnd_ids(&mut state, &window_id, container_id, self.id, "add_tool_tip".to_string());

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
    
    #[pyo3(signature = (parent_id, label, gen_id=None, on_press=None, 
                        width=None, height=None, width_fill=false, 
                        height_fill=false, padding=vec![10.0], clip=false,
                        style=Some(IpgButtonStyle::Primary), 
                        style_background=None, style_border=None, style_shadow=None, 
                        style_text_color=None, arrow_style=None, user_data=None, show=true, 
                        ))]
    fn add_button(&mut self,
                        parent_id: String,
                        label: String,
                        // ** above required
                        gen_id: Option<usize>,
                        on_press: Option<PyObject>,
                        width: Option<f32>,
                        height: Option<f32>,
                        width_fill: bool,
                        height_fill: bool,
                        padding: Vec<f64>,
                        clip: bool,
                        style: Option<IpgButtonStyle>,
                        style_background: Option<String>,
                        style_border: Option<String>,
                        style_shadow: Option<String>,
                        style_text_color: Option<String>,
                        arrow_style: Option<PyObject>,
                        user_data: Option<PyObject>,
                        show: bool,
                        ) -> PyResult<usize> 
    {
        let id = self.get_id(gen_id);

        if on_press.is_some() {
            add_callback_to_mutex(id, "on_press".to_string(), on_press);
        }

        let width = get_width(width, width_fill);
        let height = get_height(height, height_fill);

        let padding = get_padding(padding);

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
                                                clip,
                                                style,
                                                style_background,
                                                style_border,
                                                style_shadow,
                                                style_text_color,
                                                arrow_style,                              
                                                )));
        
        Ok(id)
    
    }

    #[pyo3(signature = (parent_id, head, body, is_open=true, minmax_id=None, foot=None, 
                        gen_id=None, close_size=15.0, on_close=None, 
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
                gen_id: Option<usize>,
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
        let id = self.get_id(gen_id);

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

    #[pyo3(signature = (parent_id, gen_id=None, on_toggle=None, is_checked=false, 
                        label="".to_string(), width=None, width_fill=false, 
                        size=16.0, spacing=10.0, text_line_height=1.3, 
                        text_shaping="basic".to_string(),text_size=16.0, icon_x=false, 
                        icon_size=25.0, user_data=None, show=true, style_background=None,
                        style_border=None,style_icon_color=None, style_text_color=None,
                        ))] 
    fn add_checkbox(&mut self,
                        parent_id: String,
                        // ** above required
                        gen_id: Option<usize>,
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
                        style_background: Option<String>,
                        style_border: Option<String>,
                        style_icon_color: Option<String>,
                        style_text_color: Option<String>,
                        ) -> PyResult<usize> 
    {
        let id = self.get_id(gen_id);

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
                                                    style_background,
                                                    style_border,
                                                    style_icon_color,
                                                    style_text_color,
                                                    )));

        Ok(id)

    }

    #[pyo3(signature = (parent_id, label="Calendar".to_string(), gen_id=None,
                        size_factor=1.0, padding=vec![5.0], on_submit=None, 
                        user_data=None, show=false,
                        ))]
    fn add_date_picker(&mut self,
                        parent_id: String,
                        // ** above required
                        label: String,
                        gen_id: Option<usize>,
                        size_factor: f32,
                        padding: Vec<f64>,
                        on_submit: Option<PyObject>,
                        user_data: Option<PyObject>,
                        show: bool,
                        ) -> PyResult<usize> 
    {
        let id = self.get_id(gen_id);

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

    #[pyo3(signature = (parent_id, image_path, gen_id=None, 
                        width=None, width_fill=false, 
                        height=None, height_fill=false, 
                        padding=vec![5.0], content_fit=IpgImageContentFit::Contain, 
                        filter_method=IpgImageFilterMethod::Linear,
                        rotation=IpgImageRotation::Floating,
                        rotation_radians=0.0, opacity=1.0,
                        on_press=None, on_release=None,
                        on_right_press=None, on_right_release=None,
                        on_middle_press=None, on_middle_release=None,
                        on_enter=None, on_move=None, on_exit=None, 
                        user_data=None, show=false,
                        ))]
    fn add_image(&mut self,
                    parent_id: String,
                    image_path: String,
                    // above required
                    gen_id: Option<usize>,
                    width: Option<f32>,
                    width_fill: bool,
                    height: Option<f32>,
                    height_fill: bool,
                    padding: Vec<f64>,
                    content_fit: IpgImageContentFit,
                    filter_method: IpgImageFilterMethod,
                    rotation: IpgImageRotation,
                    rotation_radians: f32,
                    opacity: f32,
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
    let id = self.get_id(gen_id);

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
                                                content_fit,
                                                filter_method,
                                                rotation,
                                                rotation_radians,
                                                opacity,
                                                show,
                                                user_data,
                                            )));

        Ok(id)
    }

    #[pyo3(signature = (parent_id, items, widths, spacing, 
                        on_select=None, bar_style=IpgMenuBarStyle::Text, 
                        item_type=vec![], item_style=vec![], 
                        separators=None, sep_types=None, 
                        sep_label_names=None, user_data=None, gen_id=None))]
    fn add_menu(&mut self, 
                    parent_id: String, 
                    items: PyObject,
                    widths: Vec<f32>,
                    spacing: Vec<f32>,
                    on_select: Option<PyObject>,
                    bar_style: IpgMenuBarStyle,
                    item_type: Vec<(usize, usize, IpgMenuItemType)>,
                    item_style: Vec<(usize, usize, IpgMenuItemStyle)>,
                    separators: Option<Vec<(usize, usize, IpgMenuSepTypes)>>,
                    sep_types: Option<Vec<IpgMenuSepTypes>>,
                    sep_label_names: Option<Vec<String>>,
                    user_data: Option<PyObject>,
                    gen_id: Option<usize>,
                ) -> PyResult<usize> 
    {
        let id = self.get_id(gen_id);

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
                                                                bar_style,
                                                                item_type,
                                                                item_style,
                                                                separators,
                                                                sep_types,
                                                                sep_label_names,
                                                                user_data,
                                                                )));

        Ok(id)
    }

    #[pyo3(signature = (parent_id, options, gen_id=None, on_select=None, 
                        width=None, width_fill=false, padding=vec![5.0],  
                        placeholder=None, selected=None, text_size=None, 
                        text_line_height=1.3, text_shaping="basic".to_string(), 
                        user_data=None, show=true,
                        ))]
    fn add_pick_list(&mut self,
                        parent_id: String,
                        options: PyObject,
                        // **above required
                        gen_id: Option<usize>,
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

        let id = self.get_id(gen_id);

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
                        gen_id=None, width=None, height=Some(16.0), 
                        width_fill=true, height_fill=false,
                        show=true, 
                        ))]
    fn add_progress_bar(&mut self,
                            parent_id: String,
                            min: f32,
                            max: f32,
                            value: f32,
                            // **above required
                            gen_id: Option<usize>,
                            width: Option<f32>,
                            height: Option<f32>,
                            width_fill: bool,
                            height_fill: bool,
                            show: bool,
                            ) -> PyResult<usize> 
    {

        let id = self.get_id(gen_id);

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

    #[pyo3(signature = (parent_id, labels, gen_id=None,
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
                    gen_id: Option<usize>,
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

        let id = self.get_id(gen_id);

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
        let gen_id: Option<usize> = None;
        let id = self.get_id(gen_id);

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
        let gen_id: Option<usize> = None;
        let id = self.get_id(gen_id);

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

    #[pyo3(signature = (parent_id, text, gen_id=None, on_press=None, on_release=None, 
                        on_right_press=None, on_right_release=None, on_middle_press=None, 
                        on_middle_release=None, on_move=None, on_enter=None, on_exit=None, 
                        width=None, height=None, width_fill=false, height_fill=false, 
                        h_align=IpgSelectableTextHorAlign::Left, v_align=IpgSelectableTextVertAlign::Top, 
                        line_height=1.3, size=16.0, show=true, 
                        shaping="basic".to_string(), user_data=None
                        ))]
    fn add_selectable_text(&mut self,
                            parent_id: String,
                            text: String,
                            // ** above required
                            gen_id: Option<usize>,
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
                            h_align: IpgSelectableTextHorAlign,
                            v_align: IpgSelectableTextVertAlign,
                            line_height: f32,
                            size: f32,
                            show: bool,
                            shaping: String,
                            user_data: Option<PyObject>,
                            ) -> PyResult<usize> 
    {
    
        let id = self.get_id(gen_id);

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

        let line_height = LineHeight::Relative(line_height);

        let shaping = get_shaping(shaping);

        set_state_of_widget(id, parent_id);

        let mut state = access_state();
        
        state.widgets.insert(id, IpgWidgets::IpgSelectableText(IpgSelectableText::new(
                                                    id,
                                                    content,
                                                    width,
                                                    height,
                                                    h_align,
                                                    v_align,
                                                    line_height,
                                                    size,
                                                    show,
                                                    shaping,
                                                    user_data,
                                                    )));
            
        Ok(id)

    }

    #[pyo3(signature = (parent_id, min, max, step, value, 
                        gen_id=None, width=None, height=None, 
                        width_fill=false, on_change=None, 
                        on_release=None, user_data=None, show=true, 
                        ))]
    fn add_slider(&mut self,
                        parent_id: String,
                        min: f32,
                        max: f32,
                        step: f32,
                        value: f32,
                        gen_id: Option<usize>,
                        width: Option<f32>,
                        height: Option<f32>,
                        width_fill: bool,
                        on_change: Option<PyObject>,
                        on_release: Option<PyObject>,
                        user_data: Option<PyObject>,
                        show: bool,
                        ) -> PyResult<usize> 
        {

        let id = self.get_id(gen_id);

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

    #[pyo3(signature = (parent_id, gen_id=None, width=None, height=None, 
                        width_fill=false, height_fill=false))]
    fn add_space(&mut self,
                        parent_id: String,
                        gen_id: Option<usize>,
                        width: Option<f32>, 
                        height: Option<f32>,
                        width_fill: bool,
                        height_fill: bool,
                    ) -> PyResult<usize>
    {

        let id = self.get_id(gen_id);

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

    #[pyo3(signature = (style_id, rgba=None, color=None, 
                        invert=false, scale_alpha=1.0, accent_amount=0.05, 
                        gen_id=None))]
    fn add_styling_background(&mut self,
                            style_id: String,
                            rgba: Option<[f32; 4]>,
                            color: Option<IpgColor>,
                            invert: bool,
                            scale_alpha: f32,
                            accent_amount: f32,
                            gen_id: Option<usize>,
                            ) -> PyResult<usize>
    {
        let id = self.get_id(gen_id);

        let color: Color = get_color(rgba, color, scale_alpha, invert);

        let mut state = access_state();
       
        state.styling_background.insert(style_id, StyleBackground::new( 
                                                    id,
                                                    color,
                                                    accent_amount,
                                                    ));
        
        drop(state);

        Ok(id)
    }

    #[pyo3(signature = (style_id, rgba=None, color=None, 
                        invert=false, scale_alpha=1.0, width=1.0, 
                        radius=vec![5.0], gen_id=None))]
    fn add_styling_border(&mut self,
                            style_id: String,
                            rgba: Option<[f32; 4]>,
                            color: Option<IpgColor>,
                            invert: bool,
                            scale_alpha: f32,
                            width: f32,
                            radius: Vec<f32>,
                            gen_id: Option<usize>,
                            ) -> PyResult<usize>
    {
        let id = self.get_id(gen_id);

        let color: Color = get_color(rgba, color, scale_alpha, invert);

        let radius: Radius = if radius.len() == 1 {
            Radius::from([radius[0]; 4])
        } else if radius.len() == 4 {
            Radius::from([radius[0], radius[1], radius[2], radius[3]])
        } else {
            panic!("Radius must have a type of list with either 1 or 4 items")
        };


        let mut state = access_state();
       
        state.styling_border.insert(style_id, StyleBorder::new( 
                                                id,
                                                color,
                                                radius,
                                                width,
                                                ));

        drop(state);

        Ok(id) 
    }

    #[pyo3(signature = (style_id, rgba=None, color=None, 
                        invert=false, scale_alpha=1.0, offset_x=0.0, 
                        offset_y=0.0, blur_radius=0.0, gen_id=None))]
    fn add_styling_shadow(&mut self,
                            style_id: String,
                            rgba: Option<[f32; 4]>,
                            color: Option<IpgColor>,
                            invert: bool,
                            scale_alpha: f32,
                            offset_x: f32,
                            offset_y: f32,
                            blur_radius: f32,
                            gen_id: Option<usize>,
                            ) -> PyResult<usize>
    {
        let id = self.get_id(gen_id);

        let color: Color = get_color(rgba, color, scale_alpha, invert);

        let mut state = access_state();
       
        state.styling_shadow.insert(style_id, StyleShadow::new( 
                                                id,
                                                color,
                                                offset_x,
                                                offset_y,
                                                blur_radius,
                                                ));

        drop(state);

        Ok(id)
    }

    #[pyo3(signature = (style_id, rgba=None, color=None, 
                        invert=false, alpha=1.0, gen_id=None))]
    fn add_styling_text_color(&mut self,
                                style_id: String,
                                rgba: Option<[f32; 4]>,
                                color: Option<IpgColor>,
                                invert: bool,
                                alpha: f32,
                                gen_id: Option<usize>,
                                ) -> PyResult<usize>
    {
        let id = self.get_id(gen_id);

        let color: Color = get_color(rgba, color, alpha, invert);

        let mut state = access_state();
       
        state.styling_text_color.insert(style_id, StyleTextColor::new( 
                                                id,
                                                color,
                                                ));

        drop(state);

        Ok(id)
    }

    #[pyo3(signature = (parent_id, svg_path, gen_id=None, 
                        width=None, width_fill=false, 
                        height=None, height_fill=false,
                        content_fit=IpgSvgContentFit::Contain,
                        rotation=IpgSvgRotation::Floating,
                        rotation_radians=0.0, opacity=1.0, 
                        on_press=None, on_release=None,
                        on_right_press=None, on_right_release=None,
                        on_middle_press=None, on_middle_release=None,
                        on_enter=None, on_move=None, on_exit=None, 
                        user_data=None, show=false,
                        ))]
    fn add_svg(&mut self,
                    parent_id: String,
                    svg_path: String,
                    // above required
                    gen_id: Option<usize>,
                    width: Option<f32>,
                    width_fill: bool,
                    height: Option<f32>,
                    height_fill: bool,
                    content_fit: IpgSvgContentFit,
                    rotation: IpgSvgRotation,
                    rotation_radians: f32,
                    opacity: f32,
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
    let id = self.get_id(gen_id);

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

    set_state_of_widget(id, parent_id);

    let mut state = access_state();

    state.widgets.insert(id, IpgWidgets::IpgSvg(IpgSvg::new(
                                                id,
                                                svg_path,
                                                width,
                                                height,
                                                content_fit,
                                                rotation,
                                                rotation_radians,
                                                opacity,
                                                show,
                                                user_data,
                                            )));

        Ok(id)
    }

    #[pyo3(signature = (parent_id, title, data, width, height,
                        row_highlight=None, highlight_amount=0.15,
                        column_widths=vec![], table_length=0, 
                        widgets_using_columns=None, gen_id=None, 
                        on_button=None, on_checkbox=None,
                        on_toggler=None, button_style=None,
                        show=true, user_data=None))]
    fn add_table(&mut self,
                    parent_id: String,
                    title: String,
                    data: Vec<PyObject>,
                    width: f32,
                    height: f32,
                    // **above required
                    row_highlight: Option<TableRowHighLight>,
                    highlight_amount: f32,
                    column_widths: Vec<f32>,
                    table_length: u32,
                    widgets_using_columns: Option<PyObject>,
                    gen_id: Option<usize>,
                    on_button: Option<PyObject>,
                    on_checkbox: Option<PyObject>,
                    on_toggler: Option<PyObject>,
                    button_style: Option<HashMap<usize, IpgButtonStyle>>,
                    show: bool,
                    user_data: Option<PyObject>,
                ) -> PyResult<usize> 
    {

        let id = self.get_id(gen_id);

        let mut column_widgets: Option<HashMap<usize, Vec<TableWidget>>> = None;
        
        if widgets_using_columns.is_some() {
            Python::with_gil(|py| {
                let wwc = widgets_using_columns.unwrap();
                let res = wwc.extract::<HashMap<usize, Vec<TableWidget>>>(py);
                column_widgets = match res {
                    Ok(val) => Some(val),
                    Err(_) => panic!("table: Unable to extract widgets_using_columns"),
                };
            });
        }

        // Need to generate the ids for the widgets and the boolean values
        // Keeping the ids organized in a hashmap for now, may need only a vec.
        let mut button_ids: Vec<(usize, usize, usize, bool)> = vec![]; // (id, row, col, bool)
        let mut check_ids: Vec<(usize, usize, usize, bool)> = vec![];
        let mut tog_ids: Vec<(usize, usize, usize, bool)> = vec![];
            
        if column_widgets.is_some() {
            for (col, table_widgets) in column_widgets.unwrap() {
                for (row, widget) in table_widgets.iter().enumerate() {
                    match widget {
                        TableWidget::Button => {
                            button_ids.push((self.get_id(None), row, col, false));
                        },
                        TableWidget::Checkbox => {
                            check_ids.push((self.get_id(None), row, col, false));
                        },
                        TableWidget::Toggler => {
                            tog_ids.push((self.get_id(None), row, col, false));
                        },
                    }

                }
            }
        }
        
        if on_button.is_some() {
            add_callback_to_mutex(id, "on_button".to_string(), on_button);
        }

        if on_checkbox.is_some() {
            add_callback_to_mutex(id, "on_checkbox".to_string(), on_checkbox);
        }

        if on_toggler.is_some() {
            add_callback_to_mutex(id, "on_toggler".to_string(), on_toggler);
        }

        set_state_of_widget(id, parent_id.clone());

        let mut state = access_state();

        let container_id_opt = state.container_str_ids.get(&parent_id);

        let container_id = match container_id_opt {
            Some(ci) => *ci,
            None => panic!("add_table: Unable to find container_id")
        };

        let window_id_opt = state.container_window_usize_ids.get(&container_id);

        let window_id = match window_id_opt {
            Some(wnd_id) => *wnd_id,
            None => panic!("add_table: Unable to find window_id")
        };

        state.widgets.insert(id, IpgWidgets::IpgTable(IpgTable::new( 
                                                    id,
                                                    title,
                                                    data,
                                                    width,
                                                    height,
                                                    row_highlight,
                                                    highlight_amount,
                                                    column_widths,
                                                    table_length,
                                                    button_style,
                                                    button_ids,
                                                    check_ids,
                                                    tog_ids,
                                                    show,
                                                    user_data,
                                                    container_id,
                                                    window_id,
                                                    )));

        Ok(id)

    }

    #[pyo3(signature = (parent_id, content, gen_id=None, width=None, 
                        height=None, width_fill=false, height_fill=false, 
                        h_align="left".to_string(), v_align="top".to_string(),
                        line_height=1.3, size=16.0, 
                        shaping="basic".to_string(), show=true,
                        ))]
    fn add_text(&mut self,
                    parent_id: String,
                    content: String,
                    // ** above required
                    gen_id: Option<usize>,
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
    
        let id = self.get_id(gen_id);

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

    #[pyo3(signature = (parent_id, placeholder, gen_id=None,
                        on_input=None, on_submit=None, 
                        on_paste=None, width=None, width_fill=false, 
                        padding=DEFAULT_PADDING.to_vec(), 
                        size=20.0, line_height=None, 
                        user_data=None, is_secure=false, show=true,
                        ))]
    fn add_text_input(&mut self,
                            parent_id: String,
                            placeholder: String,
                            // **above required
                            gen_id: Option<usize>,
                            on_input: Option<PyObject>,
                            on_submit: Option<PyObject>,
                            on_paste: Option<PyObject>,
                            width: Option<f32>,
                            width_fill: bool,
                            padding: Vec<f64>,
                            size: f32,
                            line_height: Option<f32>,
                            user_data: Option<PyObject>,
                            is_secure: bool,
                            show: bool,
                        ) -> PyResult<usize> 
    {

        let id = self.get_id(gen_id);

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

    #[pyo3(signature = (parent_id, duration_ms, on_start=None, on_tick=None, on_stop=None, 
                        start_label="Start Timer".to_string(), 
                        stop_label="Stop Timer".to_string(), width=None, height=None, 
                        width_fill=false, height_fill=false, padding=vec![10.0], 
                        style_background=None, style_border=None, style_shadow=None,
                        style_text_color=None, arrow_style=None, user_data=None))]
    fn add_timer(&mut self,
                        parent_id: String,
                        duration_ms: u64,
                        on_start: Option<PyObject>,
                        on_tick: Option<PyObject>,
                        on_stop: Option<PyObject>,
                        start_label: String,
                        stop_label: String,
                        width: Option<f32>,
                        height: Option<f32>,
                        width_fill: bool,
                        height_fill: bool,
                        padding: Vec<f64>,
                        style_background: Option<String>,
                        style_border: Option<String>,
                        style_shadow: Option<String>,
                        style_text_color: Option<String>,
                        arrow_style: Option<PyObject>,
                        user_data: Option<PyObject>
                    ) -> PyResult<usize>
    {
        self.id += 1;

        if on_start.is_some() {
            add_callback_to_mutex(self.id, "on_start".to_string(), on_start);
        }
        if on_tick.is_some() {
            add_callback_to_mutex(self.id, "on_tick".to_string(), on_tick);
        }

        if on_stop.is_some() {
            add_callback_to_mutex(self.id, "on_stop".to_string(), on_stop);
        }

        let width = get_width(width, width_fill);
        let height = get_height(height, height_fill);

        let padding = get_padding(padding);

        set_state_of_widget(self.id, parent_id);
        
        let mut state = access_state();

        state.widgets.insert(self.id, IpgWidgets::IpgTimer(IpgTimer::new(
                                                            self.id,
                                                            duration_ms,
                                                            start_label,
                                                            stop_label,
                                                            width,
                                                            height,
                                                            padding,
                                                            style_background,
                                                            style_border,
                                                            style_shadow,
                                                            style_text_color,
                                                            arrow_style,
                                                            user_data, 
                                                            )));

        Ok(self.id)
    }

    #[pyo3(signature = (parent_id, label=None, gen_id=None, toggled=None, 
                        width=None, width_fill=false, size=20.0, text_size=16.0,
                        text_line_height=1.3, text_alignment=IpgAlignment::Center, 
                        spacing=10.0, user_data=None, show=true, 
                        ))]
    fn add_toggler(&mut self,
                        parent_id: String,
                        // ** above required
                        label: Option<String>,
                        gen_id: Option<usize>,
                        toggled: Option<PyObject>,
                        width: Option<f32>,
                        width_fill: bool,
                        size: f32,
                        text_size: f32,
                        text_line_height: f32,
                        text_alignment: IpgAlignment,
                        spacing: f32,
                        user_data: Option<PyObject>,
                        show: bool,
                        ) -> PyResult<usize> 
    {
        let id = self.get_id(gen_id);

        if toggled.is_some() {
            add_callback_to_mutex(id, "toggled".to_string(), toggled);
        }

        let text_line_height = LineHeight::Relative(text_line_height);

        let width = get_width(width, width_fill);

        set_state_of_widget(id, parent_id);

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgToggler(IpgToggler::new(
                                                id,
                                                show,
                                                user_data,
                                                label,
                                                width,
                                                size,
                                                text_size,
                                                text_line_height,
                                                text_alignment,
                                                spacing,                           
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

    #[pyo3(signature = (window_id, wid))]
    fn delete_item(&self, window_id: String, wid: usize) 
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
            if ipg_id.id == wid {
                index = i as i32;
                break;
            }
        }

        if index == -1 {
            panic!("item with id {wid} could not be found to delete")
        }

        ipg_ids.remove(index as usize);

        state.widgets.remove(&wid);

    }


    #[pyo3(signature = (wid, item, value))]
    fn update_item(&self, wid: usize, item: PyObject, value: PyObject) {

        let mut state = access_state();

        let widget_opt = state.widgets.get_mut(&wid);

        match widget_opt {
            Some(w) => {
                match_widget(w, item, value);
                drop(state);
            },
            None => {
                match state.containers.get_mut(&wid) {
                    // Since multi windows implementation, the window container
                    // is no longer used except for obtaining the id (usize type)
                    // which in turn allows one to get the Id type.  The state needs 
                    // to be dropped here before calling the window_update since it 
                    // opened again.
                    // The window methods will be refactored later.
                    
                    Some(cnt) => {
                        let wnd_id = check_if_window(cnt);
                        if wnd_id != 0 {
                            drop(state);
                            window_item_update(wnd_id, item, value);
                        } else {
                            match_container(cnt, item.clone(), value.clone());
                            drop(state);
                        }
                    },
                    None => panic!("Item_update: Widget, Container, or Window with id {wid} not found.")
                }
            },
        };
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

fn match_widget(widget: &mut IpgWidgets, item: PyObject, value: PyObject) {
    match widget {
        IpgWidgets::IpgButton(btn) => {
            button_item_update(btn, item, value);
        },
        IpgWidgets::IpgCard(crd) => {
            card_item_update(crd, item, value);
        },
        IpgWidgets::IpgCheckBox(chk) => {
            checkbox_item_update(chk, item, value);
        },
        IpgWidgets::IpgDatePicker(dp) => {
            date_picker_item_update(dp, item, value);
        },
        IpgWidgets::IpgImage(img) => {
            image_item_update(img, item, value);
        },
        IpgWidgets::IpgMenu(menu) => {
            menu_item_update(menu, item, value);
        },
        IpgWidgets::IpgPickList(pl) => {
            pick_list_item_update(pl, item, value);
        },
        IpgWidgets::IpgProgressBar(pb) => {
            progress_bar_item_update(pb, item, value);
        },
        IpgWidgets::IpgRadio(rd) => {
            radio_item_update(rd, item, value);
        },
        IpgWidgets::IpgRule(_) => (),
        IpgWidgets::IpgSelectableText(st) => {
            selectable_text_item_update(st, item, value);
        },
        IpgWidgets::IpgSlider(sldr) => {
            slider_item_update(sldr, item, value)
        },
        IpgWidgets::IpgSpace(_) => (),
        IpgWidgets::IpgSvg(sg) => {
            svg_item_update(sg, item, value);
        },
        IpgWidgets::IpgTable(_) => (),
        IpgWidgets::IpgText(txt) => {
            text_item_update(txt, item, value);
        },
        IpgWidgets::IpgTextInput(ti) => {
            text_input_item_update(ti, item, value);
        },
        IpgWidgets::IpgTimer(tim) => {
            timer_item_update(tim, item, value);
        },
        IpgWidgets::IpgToggler(tog) => {
            toggler_item_update(tog, item, value);
        },
    }
}

fn match_container(container: &mut IpgContainers, item: PyObject, value: PyObject) {
    // TODO: Update containers
    match container {
        IpgContainers::IpgColumn(_) => {},
        IpgContainers::IpgContainer(_) => {},
        IpgContainers::IpgMouseArea(m_area) => {
            mousearea_item_update(m_area, item, value);
        },
        IpgContainers::IpgRow(_) => {},
        IpgContainers::IpgScrollable(scroll) => {
            scrollable_item_update(scroll, item, value);
        },
        IpgContainers::IpgToolTip(_) => {},
        IpgContainers::IpgWindow(_) => {},
    }
}

fn check_if_window(container: &mut IpgContainers) -> usize {
    match container {
        IpgContainers::IpgColumn(_) => 0,
        IpgContainers::IpgContainer(_) => 0,
        IpgContainers::IpgMouseArea(_) => 0,
        IpgContainers::IpgRow(_) => 0,
        IpgContainers::IpgScrollable(_) => 0,
        IpgContainers::IpgToolTip(_) => 0,
        IpgContainers::IpgWindow(wnd_cnt) => wnd_cnt.id.clone(),
    }
}

fn set_state_cont_wnd_ids(state: &mut State, wnd_id: &String, cnt_str_id: String, 
                            cnt_id: usize, name: String) {

        state.container_str_ids.insert(cnt_str_id.clone(), cnt_id);

        let wnd_id_usize_opt = state.windows_str_ids.get(wnd_id);

        let wnd_id_usize = match wnd_id_usize_opt {
            Some(id) => *id,
            None => panic!("{}: could not get window usize id", name),
        };

        state.container_str_ids.insert(cnt_str_id, cnt_id);

        state.container_window_usize_ids.insert(cnt_id, wnd_id_usize);
}

#[pymodule]
fn icedpygui(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_class::<IPG>()?;
    m.add_class::<IpgAlignment>()?;
    m.add_class::<IpgButtonArrows>()?;
    m.add_class::<IpgButtonParams>()?;
    m.add_class::<IpgButtonStyle>()?;
    m.add_class::<IpgCardStyles>()?;
    m.add_class::<IpgCardParams>()?;
    m.add_class::<IpgColor>()?;
    m.add_class::<IpgColumnAlignment>()?;
    m.add_class::<IpgContainerAlignment>()?;
    m.add_class::<IpgCheckboxParams>()?;
    m.add_class::<IpgDatePickerParams>()?;
    m.add_class::<IpgImageContentFit>()?;
    m.add_class::<IpgImageFilterMethod>()?;
    m.add_class::<IpgImageParams>()?;
    m.add_class::<IpgImageRotation>()?;
    m.add_class::<IpgMenuParams>()?;
    m.add_class::<IpgMenuBarStyle>()?;
    m.add_class::<IpgMenuItemType>()?;
    m.add_class::<IpgMenuItemStyle>()?;
    m.add_class::<IpgMenuSepTypes>()?;
    m.add_class::<IpgMouseAreaParams>()?;
    m.add_class::<IpgPickListParams>()?;
    m.add_class::<IpgProgressBarParams>()?;
    m.add_class::<IpgRadioDirection>()?;
    m.add_class::<IpgRadioParams>()?;
    m.add_class::<IpgRowAlignment>()?;
    m.add_class::<IpgScrollableAlignment>()?;
    m.add_class::<IpgScrollableDirection>()?;
    m.add_class::<IpgScrollableParams>()?;
    m.add_class::<IpgSelectableTextParams>()?;
    m.add_class::<IpgSelectableTextHorAlign>()?;
    m.add_class::<IpgSelectableTextVertAlign>()?;
    m.add_class::<IpgSliderParams>()?;
    m.add_class::<IpgSvgParams>()?;
    m.add_class::<TableRowHighLight>()?;
    m.add_class::<TableWidget>()?;
    m.add_class::<IpgTextInputParams>()?;
    m.add_class::<IpgTextParams>()?;
    m.add_class::<IpgTimerParams>()?;
    m.add_class::<IpgTogglerParams>()?;
    m.add_class::<IpgWindowParams>()?;
    m.add_class::<IpgWindowThemes>()?;
    Ok(())
}


#[derive(Debug, Clone, PartialEq)]
#[pyclass]
pub enum IpgAlignment {
    Left,
    Center, 
    Right,
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


pub fn find_parent_uid(ipg_ids: &Vec<IpgIds>, parent_id: String) -> usize {

    for id in ipg_ids.iter() {
        if id.container_id == Some(parent_id.clone()) {
            return id.id
        }
    }
    panic!("Parent id {:?} not found in function find_parent_uid()", parent_id)
}

pub fn delete_item(_id: usize) {

}
