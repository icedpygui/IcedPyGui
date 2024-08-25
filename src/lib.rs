//!lib for all of the python callable functions using pyo3
use ipg_widgets::ipg_modal::IpgModal;
use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::PyObject;

use iced::window::{self, Position};
use iced::{Color, Font, Length, Padding, Point, Size, Theme};
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

use ipg_widgets::ipg_button::{button_item_update, IpgButton, IpgButtonArrow, IpgButtonParam, IpgButtonStyle};
use ipg_widgets::ipg_card::{card_item_update, IpgCard, IpgCardStyle, IpgCardParam};
use ipg_widgets::ipg_checkbox::{checkbox_item_update, IpgCheckBox, IpgCheckboxParam, IpgCheckboxStyle};
use ipg_widgets::ipg_column::IpgColumn;
use ipg_widgets::ipg_container::{IpgContainer, IpgContainerStyle};
use ipg_widgets::ipg_date_picker::{date_picker_item_update, IpgDatePicker, IpgDatePickerParam};
use ipg_widgets::ipg_events::{IpgEvents, IpgKeyBoardEvent, IpgMouseEvent, IpgWindowEvent};
use ipg_widgets::ipg_image::{image_item_update, IpgImage, IpgImageContentFit, IpgImageFilterMethod, 
    IpgImageParam, IpgImageRotation};
use ipg_widgets::ipg_menu::{menu_item_update, IpgMenu, IpgMenuParam, IpgMenuSeparatorStyle, 
    IpgMenuSeparatorType, IpgMenuBarStyle, IpgMenuStyle, IpgMenuType};
use ipg_widgets::ipg_mousearea::{mousearea_item_update, IpgMouseArea, IpgMouseAreaParam};
use ipg_widgets::ipg_pick_list::{pick_list_item_update, IpgPickList, IpgPickListHandle, IpgPickListParam, IpgPickListStyle};
use ipg_widgets::ipg_progress_bar::{progress_bar_item_update, IpgProgressBar, IpgProgressBarParam, IpgProgressBarStyle};
use ipg_widgets::ipg_radio::{radio_item_update, IpgRadio, IpgRadioDirection, IpgRadioParam, IpgRadioStyle};
use ipg_widgets::ipg_row::IpgRow;
use ipg_widgets::ipg_rule::{IpgRule, IpgRuleStyle};
use ipg_widgets::ipg_scrollable::{scrollable_item_update, IpgScrollable, IpgScrollableAlignment, 
                                    IpgScrollableDirection, IpgScrollableParam, IpgScrollableStyle};
use ipg_widgets::ipg_selectable_text::{selectable_text_item_update, IpgSelectableText, 
                                        IpgSelectableTextParam};
use ipg_widgets::ipg_slider::{slider_item_update, IpgSlider, IpgSliderParam, IpgSliderStyle};
use ipg_widgets::ipg_space::IpgSpace;
use ipg_widgets::ipg_svg::{svg_item_update, IpgSvg, IpgSvgContentFit, IpgSvgParam, IpgSvgRotation};
use ipg_widgets::ipg_table::{table_item_update, IpgTable, IpgTableParam, IpgTableRowHighLight, IpgTableWidget,};
use ipg_widgets::ipg_text::{text_item_update, IpgText, IpgTextParam};
use ipg_widgets::ipg_text_input::{text_input_item_update, IpgTextInputStyle, IpgTextInput, IpgTextInputParam};
use ipg_widgets::ipg_timer::{timer_item_update, IpgTimer, IpgTimerParams};
use ipg_widgets::ipg_toggle::{toggler_item_update, IpgToggler, IpgTogglerParam, IpgTogglerStyle};
use ipg_widgets::ipg_tool_tip::IpgToolTip;
use ipg_widgets::ipg_window::{get_iced_window_theme, window_item_update, IpgWindow, IpgWindowLevel, IpgWindowMode, IpgWindowParam, IpgWindowTheme};
use ipg_widgets::ipg_enums::{IpgAlignment, IpgContainers, IpgHorizontalAlignment, 
    IpgVerticalAlignment, IpgWidgets};

use ipg_widgets::helpers::{check_for_dup_container_ids, get_height, get_horizontal_alignment, 
    get_line_height, get_padding_f32, get_padding_f64, get_shaping, 
    get_vertical_alignment, get_width};

use graphics::colors::{get_color, IpgColor};
use style::styling::{readable, IpgStyleParam, IpgStyleStandard};

const ICON_FONT_BOOT: Font = Font::with_name("bootstrap-icons");

use std::sync::{Mutex, MutexGuard};
use once_cell::sync::Lazy;

pub const TABLE_INTERNAL_IDS_START: usize = 4_000_000_000;
pub const TABLE_INTERNAL_IDS_END: usize = 4_000_000_999;


#[derive(Debug)]
pub struct Callbacks {
    callbacks: Lazy<HashMap<(usize, String), Option<PyObject>>>,
    callback_events: Lazy<HashMap<(usize, String), PyObject>>,
    user_data: Vec<(usize, Option<PyObject>)>,
}

pub static CALLBACKS: Mutex<Callbacks> = Mutex::new(Callbacks {
    callbacks: Lazy::new(||HashMap::new()),
    callback_events:  Lazy::new(||HashMap::new()),
    user_data: vec![],
});

pub fn access_callbacks() -> MutexGuard<'static, Callbacks> {
    CALLBACKS.lock().unwrap()
}

pub struct WindowActions {
    pub mode: Vec<(window::Mode, usize)>,
    pub decorations: Vec<usize>,
    pub resize: Vec<(usize, f32, f32)>,
    pub position: Vec<(usize, f32, f32)>,
}

pub static WINDOWACTIONS: Mutex<WindowActions> = Mutex::new(WindowActions {
    mode: vec![],
    decorations: vec![],
    resize: vec![],
    position: vec![],
});

pub fn access_window_actions() -> MutexGuard<'static, WindowActions> {
    WINDOWACTIONS.lock().unwrap()
}

pub struct State {
    pub ids: Lazy<HashMap<usize, Vec<IpgIds>>>,  // <window_id=usize, Vec<IpgIds=structure>>
    pub last_id: usize,

    pub containers: Lazy<HashMap<usize, IpgContainers>>,
    pub container_ids: Lazy<HashMap<usize, Vec<usize>>>,  // <window_id=usize, vec<container_id=usize>>
    pub container_str_ids: Lazy<HashMap<String, usize>>, // get container usize id based on container string
    pub container_wnd_str_ids: Lazy<HashMap<String, String>>, // get window string id based on container string id
    pub container_window_usize_ids: Lazy<HashMap<usize, usize>>, //get window usize id based on container usize id

    pub table_internal_ids_counter: usize,

    pub widgets: Lazy<HashMap<usize, IpgWidgets>>,
    pub widget_container_ids: Lazy<HashMap<usize, String>>, //widget_id=usize, container_id=String
    
    pub windows: Vec<IpgWindow>,
    pub windows_iced_ipg_ids: Lazy<HashMap<window::Id, usize>>, // <iced id, ipg id>
    pub windows_str_ids: Lazy<HashMap<String, usize>>,  // <ipg_id=str, ipg id>
    pub window_debug: Lazy<HashMap<window::Id, (usize, bool)>>, // (wid, debug)
    pub window_theme: Lazy<HashMap<window::Id, (usize, Theme)>>, // (wid, window Theme)
    
    pub events: Vec<IpgEvents>,
    
    pub container_style: Lazy<HashMap<String, IpgContainerStyle>>,
    pub button_style: Lazy<HashMap<String, IpgButtonStyle>>,
    pub checkbox_style: Lazy<HashMap<String, IpgCheckboxStyle>>,
    pub menu_bar_style: Lazy<HashMap<String, IpgMenuBarStyle>>,
    pub menu_style: Lazy<HashMap<String, IpgMenuStyle>>,
    pub menu_separator_style: Lazy<HashMap<String, IpgMenuSeparatorStyle>>,
    pub pick_list_style: Lazy<HashMap<String, IpgPickListStyle>>,
    pub progress_bar_style: Lazy<HashMap<String, IpgProgressBarStyle>>,
    pub radio_style:  Lazy<HashMap<String, IpgRadioStyle>>,
    pub rule_style:  Lazy<HashMap<String, IpgRuleStyle>>,
    pub slider_style:  Lazy<HashMap<String, IpgSliderStyle>>,
    pub text_input_style: Lazy<HashMap<String, IpgTextInputStyle>>,
    pub toggler_style: Lazy<HashMap<String, IpgTogglerStyle>>,
    pub scrollable_style: Lazy<HashMap<String, IpgScrollableStyle>>,

}

pub static STATE: Mutex<State> = Mutex::new(
    State {
        ids: Lazy::new(||HashMap::new()),
        last_id: 0,
        
        containers: Lazy::new(||HashMap::new()),
        container_ids: Lazy::new(||HashMap::new()),
        container_str_ids: Lazy::new(||HashMap::new()),
        container_wnd_str_ids: Lazy::new(||HashMap::new()),
        container_window_usize_ids: Lazy::new(||HashMap::new()),

        table_internal_ids_counter: TABLE_INTERNAL_IDS_START,

        widgets: Lazy::new(||HashMap::new()),
        widget_container_ids: Lazy::new(||HashMap::new()),

        
        windows: vec![],
        windows_iced_ipg_ids: Lazy::new(||HashMap::new()),
        windows_str_ids: Lazy::new(||HashMap::new()),
        window_debug: Lazy::new(||HashMap::new()),
        window_theme: Lazy::new(||HashMap::new()),
        
        events: vec![],

        container_style: Lazy::new(||HashMap::new()),
        button_style: Lazy::new(||HashMap::new()),
        checkbox_style: Lazy::new(||HashMap::new()),
        menu_bar_style: Lazy::new(||HashMap::new()),
        menu_style: Lazy::new(||HashMap::new()),
        menu_separator_style: Lazy::new(||HashMap::new()),
        pick_list_style: Lazy::new(||HashMap::new()),
        progress_bar_style: Lazy::new(||HashMap::new()),
        radio_style: Lazy::new(||HashMap::new()),
        rule_style: Lazy::new(||HashMap::new()),
        slider_style: Lazy::new(||HashMap::new()),
        text_input_style: Lazy::new(||HashMap::new()),
        toggler_style: Lazy::new(||HashMap::new()),
        scrollable_style: Lazy::new(||HashMap::new()),
    
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
    gen_ids: Vec<usize>,
    group_index: usize,
    theme: Theme,
}

#[pymethods]
impl IPG {
    #[new]
    fn new() -> IPG {
        IPG {
            id: 0,
            gen_ids: vec![],
            group_index: 0,
            theme: Theme::Dark,
        }
    }

    #[pyo3(signature = ())]
    fn start_session(&self) {

        let state = access_state();
        let mut flags = Flags {keyboard_event_enabled: (0, false),
                                        mouse_event_enabled: (0, false), 
                                        timer_event_enabled: (0, false), 
                                        window_event_enabled: (0, false),
                                        touch_event_enabled: (0, false),
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

        let _ = iced::daemon(App::title, App::update, App::view)
                    .subscription(App::subscription)
                    .theme(App::theme)
                    .scale_factor(App::scale_factor)
                    .antialiasing(true)
                    .run_with(||App::new(flags));
    }

    #[pyo3(signature = ())]
    fn generate_id(&mut self) -> PyResult<usize>
    {
        self.id += 1;
        self.gen_ids.push(self.id);
        let mut state = access_state();
        state.last_id = self.id;
        drop(state);
        Ok(self.id)
    }

    #[pyo3(signature = (window_id, title, width, height,
                        max_width=None, max_height=None,
                        min_width=None, min_height=None,
                        pos_x=None, pos_y=None,
                        pos_centered=false, resizable=true,
                        decorations=true, transparent=false,
                        level=IpgWindowLevel::Normal,
                        scale_factor=1.0,
                        theme=IpgWindowTheme::Dark, 
                        exit_on_close=true, on_resize=None, 
                        mode=IpgWindowMode::Windowed, 
                        debug=false, user_data=None,
                        gen_id=None))]
    fn add_window(&mut self,
                        window_id: String, 
                        title: String, 
                        width: f32, 
                        height: f32,
                        max_width: Option<f32>,
                        max_height: Option<f32>,
                        min_width: Option<f32>,
                        min_height: Option<f32>, 
                        pos_x: Option<f32>,
                        pos_y: Option<f32>,
                        pos_centered: bool,
                        resizable: bool,
                        decorations: bool,
                        transparent: bool,
                        level: IpgWindowLevel,
                        scale_factor: f64,
                        theme: IpgWindowTheme,
                        exit_on_close: bool,
                        on_resize: Option<PyObject>,
                        mode: IpgWindowMode,
                        debug: bool,
                        user_data: Option<PyObject>,
                        gen_id: Option<usize>,
                    ) -> PyResult<usize>
    {
        let id = self.get_id(gen_id);

        let mut window_position = Position::Default;

        let size = Size::new(width, height);

        let mut max_size = Size::INFINITY;

        if max_width.is_some() {
            max_size.width = max_width.unwrap();
        }
        if max_height.is_some() {
            max_size.height = max_height.unwrap();
        }

        let mut min_size = Size::ZERO;

        if min_width.is_some() {
            min_size.width = min_width.unwrap();
        }

        if min_height.is_some() {
            min_size.height = min_height.unwrap();
        }

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

        self.theme = get_iced_window_theme(theme.clone()); // used later for menu
        let iced_theme = get_iced_window_theme(theme);

        let mut state = access_state();

        if state.windows_str_ids.get(&window_id).is_some() {
            panic!("Window id {} is not unique", window_id)
        };

        if on_resize.is_some() {
            add_callback_to_mutex(id, "on_resize".to_string(), on_resize);
        }

        state.windows_str_ids.insert(window_id.clone(), id);

        state.ids.insert(id, vec![IpgIds{id: id, parent_uid: 0, container_id: Some(window_id.clone()),
                                                parent_id: "".to_string(), is_container: true}]);

        state.container_ids.insert(id, vec![id]);
        // TODO: Only one of these below are needed but some suttle issues arise when not used together.
        // Will need to work through it in the near future.  At the onset, used only one window then
        // iced made multi-window so sort of patch it to work but need to revisit it.
        state.containers.insert(id, IpgContainers::IpgWindow(IpgWindow::new(
                                            id,
                                            title.clone(), 
                                            size,
                                            Some(min_size),
                                            Some(max_size),
                                            window_position,
                                            exit_on_close,
                                            iced_theme.clone(), 
                                            resizable,
                                            mode.clone(),
                                            decorations,
                                            transparent,
                                            level.clone(),
                                            scale_factor,
                                            debug,
                                            user_data.clone(),
                                            )));
        
        state.windows.push(IpgWindow::new(
                                        id,
                                        title, 
                                        size,
                                        Some(min_size),
                                        Some(max_size),
                                        window_position,
                                        exit_on_close,
                                        iced_theme, 
                                        resizable,
                                        mode,
                                        decorations,
                                        transparent,
                                        level,
                                        scale_factor,
                                        debug,
                                        user_data,
                                        ));
        state.last_id = id;
        drop(state);

        Ok(id)

    }

    #[pyo3(signature = (window_id, container_id, parent_id=None,
                        width=None, width_fill=false, 
                        height=None, height_fill=false, 
                        clip=false, max_height=f32::INFINITY, max_width=f32::INFINITY,
                        horizontal_alignment=None, vertical_alignment=None,
                        padding=vec![0.0], show=true, style_id=None, 
                        
                       ))]
    fn add_container(&mut self,
                        window_id: String,
                        container_id: String,
                        // **above reuired
                        parent_id: Option<String>,
                        width: Option<f32>,
                        width_fill: bool,
                        height: Option<f32>,
                        height_fill: bool,
                        clip: bool,
                        max_height: f32,
                        max_width: f32,
                        horizontal_alignment: Option<IpgHorizontalAlignment>,
                        vertical_alignment: Option<IpgVerticalAlignment>, 
                        padding: Vec<f64>, 
                        show: bool,
                        style_id: Option<String>,
                        ) -> PyResult<usize>
    {
        self.id += 1;

        let width = get_width(width, width_fill);
        let height = get_height(height, height_fill);
        let padding = get_padding_f64(padding);
        
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
                                                horizontal_alignment,
                                                vertical_alignment,
                                                clip,
                                                style_id, 
                                            )));
        state.last_id = self.id;
        drop(state);

        Ok(self.id)

    }

    #[pyo3(signature = (style_id, 
                        background_color=None, background_rgba=None,
                        border_color=None, border_rgba=None,
                        border_radius = vec![0.0], border_width=1.0,
                        shadow_color=None, shadow_rgba=None,
                        shadow_offset_x=0.0, shadow_offset_y=0.0,
                        shadow_blur_radius=1.0,
                        text_color=None, text_rgba=None,
                        gen_id=None))]
    fn add_container_style(&mut self,
                            style_id: String,
                            background_color: Option<IpgColor>,
                            background_rgba: Option<[f32; 4]>,
                            border_color: Option<IpgColor>,
                            border_rgba: Option<[f32; 4]>,
                            border_radius: Vec<f32>,
                            border_width: f32,
                            shadow_color: Option<IpgColor>,
                            shadow_rgba: Option<[f32; 4]>,
                            shadow_offset_x: f32,
                            shadow_offset_y: f32,
                            shadow_blur_radius: f32,
                            text_color: Option<IpgColor>,
                            text_rgba: Option<[f32; 4]>,
                            gen_id: Option<usize>,
                            ) -> PyResult<usize>
    {
        let id = self.get_id(gen_id);

        let mut state = access_state();

        let background_color: Option<Color> = get_color(background_rgba, background_color, 1.0, false);
        let border_color: Option<Color> = get_color(border_rgba, border_color, 1.0, false);
        let shadow: Option<Color> = get_color(shadow_rgba, shadow_color, 1.0, false);
        let text_color: Option<Color> = get_color(text_rgba, text_color, 1.0, false);

        state.container_style.insert(style_id, IpgContainerStyle::new( 
                                                    id,
                                                    background_color,
                                                    border_color,
                                                    border_radius,
                                                    border_width,
                                                    shadow,
                                                    shadow_offset_x,
                                                    shadow_offset_y,
                                                    shadow_blur_radius,
                                                    text_color,
                                                    ));
        state.last_id = id;
        drop(state);

        Ok(id)
    }

    #[pyo3(signature = (window_id, container_id, parent_id=None,
                        align_items=IpgAlignment::Start, width=None, height=None,
                        width_fill=false, height_fill=false,
                        max_width=f32::INFINITY, padding=vec![0.0], 
                        spacing=10.0, clip=false, show=true,
                        ))]
    fn add_column(&mut self,
                        window_id: String,
                        container_id: String,
                        // **above required
                        parent_id: Option<String>,
                        align_items: IpgAlignment,
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

        let padding = get_padding_f64(padding);

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
    state.last_id = self.id;
    drop(state);
    Ok(self.id)

    }

    #[pyo3(signature = (window_id, container_id, label,
                        parent_id=None, on_open=None,
                        align_items=IpgAlignment::Start, 
                        width=None, height=None,
                        width_fill=false, height_fill=false,
                        max_width=f32::INFINITY, padding=vec![0.0], 
                        spacing=10.0, clip=false, show=false,
                        user_data=None,
                        ))]
    fn add_modal(&mut self,
                        window_id: String,
                        container_id: String,
                        label: String,
                        // **above required
                        parent_id: Option<String>,
                        on_open: Option<PyObject>,
                        align_items: IpgAlignment,
                        width: Option<f32>,
                        height: Option<f32>,
                        width_fill: bool,
                        height_fill: bool,
                        max_width: f32,
                        padding: Vec<f64>,
                        spacing: f32,
                        clip: bool,
                        show: bool,
                        user_data: Option<PyObject>,
                        ) -> PyResult<usize> 
    {

        self.id += 1;

        if on_open.is_some() {
            add_callback_to_mutex(self.id, "on_open".to_string(), on_open);
        }
        
        let width = get_width(width, width_fill);
        let height = get_height(height, height_fill);

        let padding = get_padding_f64(padding);

        let prt_id = match parent_id {
            Some(id) => id,
            None => window_id.clone(),
        };

        set_state_of_container(self.id, window_id.clone(), Some(container_id.clone()), prt_id);

        let mut state = access_state();

        set_state_cont_wnd_ids(&mut state, &window_id, container_id, self.id, "add_column".to_string());

        state.containers.insert(self.id, IpgContainers::IpgModal(IpgModal::new(
                                                                self.id,
                                                                label,  
                                                                show, 
                                                                spacing, 
                                                                padding, 
                                                                width, 
                                                                height, 
                                                                max_width, 
                                                                align_items,
                                                                clip,
                                                                user_data,
                                                            )));
        state.last_id = self.id;
        drop(state);
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
        state.last_id = id;
        drop(state);
        Ok(id)

    }

    #[pyo3(signature = (window_id, container_id, parent_id=None,
                        align_items=IpgAlignment::Start, width=None, height=None, 
                        width_fill=false, height_fill=false,
                        padding=vec![0.0], spacing=10.0, clip=false,
                        show=true,
                        ))]
    fn add_row(&mut self,
                    window_id: String,
                    container_id: String,
                    // required above
                    parent_id: Option<String>,
                    align_items: IpgAlignment,
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

        let padding = get_padding_f64(padding);

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
        state.last_id = self.id;
        drop(state);         
        Ok(self.id)

    }

    #[pyo3(signature = (window_id, container_id, parent_id=None,
                        width=None, height=None, 
                        width_fill=false, height_fill=false, 
                        direction=IpgScrollableDirection::Vertical, 
                        h_bar_width=10.0, h_bar_margin=0.0, 
                        h_scroller_width=10.0, 
                        h_bar_alignment=IpgScrollableAlignment::Start,
                        v_bar_width=10.0, v_bar_margin=0.0, 
                        v_scroller_width=10.0, 
                        v_bar_alignment=IpgScrollableAlignment::Start,
                        on_scroll=None, user_data=None,
                        style_id=None,
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
                            style_id: Option<String>,
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
                                                    style_id,
                                                    )));
        state.last_id = self.id;
        drop(state);
        Ok(self.id)

    }

    #[pyo3(signature = (style_id, 
                        background_color=None, background_rgba=None,
                        border_color=None, border_rgba=None,
                        border_radius = vec![0.0], border_width=1.0,
                        shadow_color=None, shadow_rgba=None,
                        shadow_offset_x=0.0, shadow_offset_y=0.0,
                        shadow_blur_radius=1.0,
                        text_color=None, text_rgba=None,
                        scrollbar_color=None,
                        scrollbar_rgba=None,
                        scrollbar_border_radius=vec![2.0],
                        scrollbar_border_width=1.0,
                        scrollbar_border_color=None,
                        scrollbar_border_rgba=None,
                        scroller_color=None,
                        scroller_rgba=None,
                        scroller_color_hovered=None,
                        scroller_rgba_hovered=None,
                        scroller_color_dragged=None,
                        scroller_rgba_dragged=None,
                        gen_id=None))]
    fn add_scrollable_style(&mut self,
                            style_id: String,
                            background_color: Option<IpgColor>,
                            background_rgba: Option<[f32; 4]>,
                            border_color: Option<IpgColor>,
                            border_rgba: Option<[f32; 4]>,
                            border_radius: Vec<f32>,
                            border_width: f32,
                            shadow_color: Option<IpgColor>,
                            shadow_rgba: Option<[f32; 4]>,
                            shadow_offset_x: f32,
                            shadow_offset_y: f32,
                            shadow_blur_radius: f32,
                            text_color: Option<IpgColor>,
                            text_rgba: Option<[f32; 4]>,
                            scrollbar_color: Option<IpgColor>,
                            scrollbar_rgba: Option<[f32; 4]>,
                            scrollbar_border_radius: Vec<f32>,
                            scrollbar_border_width: f32,
                            scrollbar_border_color: Option<IpgColor>,
                            scrollbar_border_rgba: Option<[f32; 4]>,
                            scroller_color: Option<IpgColor>,
                            scroller_rgba: Option<[f32; 4]>,
                            scroller_color_hovered: Option<IpgColor>,
                            scroller_rgba_hovered: Option<[f32; 4]>,
                            scroller_color_dragged: Option<IpgColor>,
                            scroller_rgba_dragged: Option<[f32; 4]>,
                            gen_id: Option<usize>,
                            ) -> PyResult<usize>
    {
        let id = self.get_id(gen_id);

        let mut state = access_state();

        let background_color: Option<Color> = get_color(background_rgba, background_color, 1.0, false);
        let border_color: Option<Color> = get_color(border_rgba, border_color, 1.0, false);
        let shadow_color: Option<Color> = get_color(shadow_rgba, shadow_color, 1.0, false);
        let text_color: Option<Color> = get_color(text_rgba, text_color, 1.0, false);

        let scrollbar_color: Option<Color> = get_color(scrollbar_rgba, scrollbar_color, 1.0, false);
        let scrollbar_border_color: Option<Color> = get_color(scrollbar_border_rgba, scrollbar_border_color, 1.0, false);
        
        let scroller_color: Option<Color> = get_color(scroller_rgba, scroller_color, 1.0, false);
        let scroller_color_hovered: Option<Color> = get_color(scroller_rgba_hovered, scroller_color_hovered, 1.0, false);
        let scroller_color_dragged: Option<Color> = get_color(scroller_rgba_dragged, scroller_color_dragged, 1.0, false);

        state.scrollable_style.insert(style_id, IpgScrollableStyle::new( 
                                                    id,
                                                    background_color,
                                                    border_color,
                                                    border_radius,
                                                    border_width,
                                                    shadow_color,
                                                    shadow_offset_x,
                                                    shadow_offset_y,
                                                    shadow_blur_radius,
                                                    text_color,
                                                    scrollbar_color,
                                                    scrollbar_border_radius,
                                                    scrollbar_border_width,
                                                    scrollbar_border_color,
                                                    scroller_color,
                                                    scroller_color_hovered,
                                                    scroller_color_dragged,
                                                    ));
        state.last_id = id;
        drop(state);

        Ok(id)
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
        state.last_id = self.id;
        drop(state);
        Ok(self.id)

    }
    
    #[pyo3(signature = (parent_id, label, gen_id=None, on_press=None, 
                        width=None, height=None, width_fill=false, 
                        height_fill=false, padding=vec![5.0], clip=false, 
                        style_id=None, style_standard=None, 
                        style_arrow=None, user_data=None, show=true, 
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
                        style_id: Option<String>,
                        style_standard: Option<IpgStyleStandard>,
                        style_arrow: Option<IpgButtonArrow>,
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

        let padding = get_padding_f64(padding);

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
                                                style_id,
                                                style_standard,
                                                style_arrow,                              
                                                )));
        state.last_id = id;
        drop(state);
        Ok(id)
    
    }

    #[pyo3(signature = (style_id, 
                        background_color=None, background_rgba=None,
                        background_color_hovered=None, background_rgba_hovered=None,
                        border_color=None, border_rgba=None,
                        border_radius = vec![0.0], border_width=1.0,
                        shadow_color=None, shadow_rgba=None,
                        shadow_offset_x=0.0, shadow_offset_y=0.0,
                        shadow_blur_radius=1.0,
                        text_color=None, text_rgba=None,
                        gen_id=None))]
    fn add_button_style(&mut self,
                            style_id: String,
                            background_color: Option<IpgColor>,
                            background_rgba: Option<[f32; 4]>,
                            background_color_hovered: Option<IpgColor>,
                            background_rgba_hovered: Option<[f32; 4]>,
                            border_color: Option<IpgColor>,
                            border_rgba: Option<[f32; 4]>,
                            border_radius: Vec<f32>,
                            border_width: f32,
                            shadow_color: Option<IpgColor>,
                            shadow_rgba: Option<[f32; 4]>,
                            shadow_offset_x: f32,
                            shadow_offset_y: f32,
                            shadow_blur_radius: f32,
                            text_color: Option<IpgColor>,
                            text_rgba: Option<[f32; 4]>,
                            gen_id: Option<usize>,
                            ) -> PyResult<usize>
    {
        let id = self.get_id(gen_id);

        let mut state = access_state();

        let background_color: Option<Color> = get_color(background_rgba, background_color, 1.0, false);
        let background_color_hovered: Option<Color> = get_color(background_rgba_hovered, background_color_hovered, 1.0, false);
        let border_color: Option<Color> = get_color(border_rgba, border_color, 1.0, false);
        let shadow_color: Option<Color> = get_color(shadow_rgba, shadow_color, 1.0, false);
        let text_color: Option<Color> = get_color(text_rgba, text_color, 1.0, false);

        state.button_style.insert(style_id, IpgButtonStyle::new( 
                                                    id,
                                                    background_color,
                                                    background_color_hovered,
                                                    border_color,
                                                    border_radius,
                                                    border_width,
                                                    shadow_color,
                                                    shadow_offset_x,
                                                    shadow_offset_y,
                                                    shadow_blur_radius,
                                                    text_color,
                                                    ));
        state.last_id = id;
        drop(state);
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

        let padding_head = get_padding_f64(padding_head);
        let padding_body = get_padding_f64(padding_body);
        let padding_foot = get_padding_f64(padding_foot);

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
        state.last_id = id;
        drop(state);
        Ok(id)

    }

    #[pyo3(signature = (parent_id, gen_id=None, on_toggle=None, is_checked=false, 
                        label="".to_string(), width=None, width_fill=false, 
                        size=16.0, spacing=10.0, text_line_height=1.3, 
                        text_shaping="basic".to_string(),text_size=16.0, 
                        icon_x=false, icon_size=25.0, user_data=None, 
                        show=true, style_id=None, style_standard=None, 
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
                        style_id: Option<String>,
                        style_standard: Option<IpgStyleStandard>,
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
                                                    style_id,
                                                    style_standard,
                                                    )));
        state.last_id = id;
        drop(state);
        Ok(id)

    }

    #[pyo3(signature = (style_id, 
                        background_color=None, 
                        background_rgba=None,
                        background_color_hovered=None,
                        background_rgba_hovered=None,
                        accent_color=None,
                        accent_rgba=None,
                        accent_color_hovered=None,
                        accent_rgba_hovered=None,
                        border_color=None, 
                        border_rgba=None,
                        border_radius=vec![2.0], 
                        border_width=1.0,
                        icon_color=None, 
                        icon_rgba=None,
                        text_color=None, 
                        text_rgba=None,
                        gen_id=None))]
    fn add_checkbox_style(&mut self,
                            style_id: String,
                            background_color: Option<IpgColor>,
                            background_rgba: Option<[f32; 4]>,
                            background_color_hovered: Option<IpgColor>,
                            background_rgba_hovered: Option<[f32; 4]>,
                            accent_color: Option<IpgColor>,
                            accent_rgba: Option<[f32; 4]>,
                            accent_color_hovered: Option<IpgColor>,
                            accent_rgba_hovered: Option<[f32; 4]>,
                            border_color: Option<IpgColor>,
                            border_rgba: Option<[f32; 4]>,
                            border_radius: Vec<f32>,
                            border_width: f32,
                            icon_color: Option<IpgColor>,
                            icon_rgba: Option<[f32; 4]>,
                            text_color: Option<IpgColor>,
                            text_rgba: Option<[f32; 4]>,
                            gen_id: Option<usize>,
                            ) -> PyResult<usize>
    {
        let id = self.get_id(gen_id);

        let mut state = access_state();

        let background_color: Option<Color> = get_color(background_rgba, background_color, 1.0, false);
        let background_color_hovered: Option<Color> = get_color(background_rgba_hovered, background_color_hovered, 1.0, false);
        let accent_color: Option<Color> = get_color(accent_rgba, accent_color, 1.0, false);
        let accent_color_hovered: Option<Color> = get_color(accent_rgba_hovered, accent_color_hovered, 1.0, false);
        let border_color: Option<Color> = get_color(border_rgba, border_color, 1.0, false);
        let icon_color: Option<Color> = get_color(icon_rgba, icon_color, 1.0, false);
        let text_color: Option<Color> = get_color(text_rgba, text_color, 1.0, false);

        state.checkbox_style.insert(style_id, IpgCheckboxStyle::new( 
                                                    id,
                                                    background_color,
                                                    background_color_hovered,
                                                    accent_color,
                                                    accent_color_hovered,
                                                    border_color,
                                                    border_radius,
                                                    border_width,
                                                    icon_color,
                                                    text_color,
                                                    ));
        state.last_id = id;
        drop(state);
        Ok(id)
    }

    #[pyo3(signature = (parent_id, label="Calendar".to_string(), gen_id=None,
                        size_factor=1.0, padding=vec![0.0], on_submit=None, 
                        user_data=None, show=false, 
                        button_style_standard=None,
                        button_style_id=None,
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
                        button_style_standard: Option<IpgStyleStandard>,
                        button_style_id: Option<String>,
                        ) -> PyResult<usize> 
    {
        let id = self.get_id(gen_id);

        if size_factor < 1.0 {
            panic!("Size factor for date picker must be > 1.0")
        }

        if on_submit.is_some() {
            add_callback_to_mutex(id, "on_submit".to_string(), on_submit);
        }

        let padding = get_padding_f64(padding);

        set_state_of_widget(id, parent_id);

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgDatePicker(IpgDatePicker::new(
                                                    id,
                                                    label,
                                                    size_factor,
                                                    padding,
                                                    show,
                                                    user_data,
                                                    button_style_standard,
                                                    button_style_id,
                                                    )));
        state.last_id = id;
        drop(state);
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

    let padding = get_padding_f64(padding);

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
        state.last_id = id;
        drop(state);
        Ok(id)
    }

    #[pyo3(signature = (parent_id, 
                        items,
                        bar_widths,
                        item_widths, 
                        bar_spacings=None,
                        bar_paddings=None,
                        bar_height=None,
                        bar_check_bounds_width=None,
                        item_spacings=None,
                        item_offsets=None, 
                        on_select=None,
                        menu_bar_style=None,
                        menu_style=None,
                        button_bar_style_all=None,
                        button_item_style_all=None,
                        checkbox_item_style_all=None,
                        circle_item_style_all=None,
                        dot_item_style_all=None,
                        label_item_style_all=None,
                        line_item_style_all=None,
                        text_item_style_all=None,
                        toggler_item_style_all=None, 
                        item_styles=None,
                        show=true, 
                        user_data=None, gen_id=None))]
    fn add_menu(&mut self, 
                    parent_id: String, 
                    items: PyObject,
                    bar_widths: Vec<f32>,
                    item_widths: Vec<f32>,
                    bar_spacings: Option<f32>,
                    bar_paddings: Option<Vec<f32>>,
                    bar_height: Option<f32>,
                    bar_check_bounds_width: Option<f32>,
                    item_spacings: Option<Vec<f32>>,
                    item_offsets: Option<Vec<f32>>,
                    on_select: Option<PyObject>,
                    menu_bar_style: Option<String>,
                    menu_style: Option<String>,
                    button_bar_style_all: Option<(Option<IpgStyleStandard>, Option<String>)>,
                    button_item_style_all: Option<(Option<IpgStyleStandard>, Option<String>)>,
                    checkbox_item_style_all: Option<(Option<IpgStyleStandard>, Option<String>)>,
                    circle_item_style_all: Option<String>,
                    dot_item_style_all: Option<String>,
                    label_item_style_all: Option<String>,
                    line_item_style_all: Option<String>,
                    text_item_style_all: Option<String>,
                    toggler_item_style_all: Option<String>,
                    item_styles: Option<Vec<(usize, usize, Option<IpgStyleStandard>, Option<String>)>>,
                    show: bool,
                    user_data: Option<PyObject>,
                    gen_id: Option<usize>,
                ) -> PyResult<usize> 
    {
        let id = self.get_id(gen_id);

        if on_select.is_some() {
            add_callback_to_mutex(id, "on_select".to_string(), on_select);
        }

        let spacing = if bar_spacings.is_some() {
            bar_spacings.unwrap()
        } else {
            0.0
        };

        let padding = if bar_paddings.is_some() {
            get_padding_f32(bar_paddings.unwrap())
        } else {
            Padding::ZERO
        };
        
        let height = get_height(bar_height, false);
 
        let check_bounds_width = if bar_check_bounds_width.is_some() {
            bar_check_bounds_width.unwrap()
        } else {
            50.0
        };
        
        set_state_of_widget(id, parent_id);

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgMenu(IpgMenu::new(
                                                                id,
                                                                items,
                                                                bar_widths,
                                                                item_widths,
                                                                spacing,
                                                                padding,
                                                                height,
                                                                check_bounds_width,
                                                                item_spacings,
                                                                item_offsets,
                                                                menu_bar_style,
                                                                menu_style,
                                                                button_bar_style_all,
                                                                button_item_style_all,
                                                                checkbox_item_style_all,
                                                                circle_item_style_all,
                                                                dot_item_style_all,
                                                                label_item_style_all,
                                                                line_item_style_all,
                                                                text_item_style_all,
                                                                toggler_item_style_all,
                                                                item_styles,
                                                                self.theme.clone(),
                                                                show,
                                                                user_data,
                                                                )));
        state.last_id = id;
        drop(state);
        Ok(id)
    }

    #[pyo3(signature = (style_id,
                        base_color=None,
                        base_rgba=None,
                        border_color=None,
                        border_rgba=None,
                        border_radius=None,
                        border_width=None,
                        shadow_color=None,
                        shadow_rgba=None,
                        shadow_offset_x=None,
                        shadow_offset_y=None,
                        shadow_blur_radius=None,
                        gen_id=None))]
    fn add_menu_bar_style(&mut self,
                            style_id: String,
                            base_color: Option<IpgColor>,
                            base_rgba: Option<[f32; 4]>,
                            border_color: Option<IpgColor>,
                            border_rgba: Option<[f32; 4]>,
                            border_radius: Option<Vec<f32>>,
                            border_width: Option<f32>,
                            shadow_color: Option<IpgColor>,
                            shadow_rgba: Option<[f32; 4]>,
                            shadow_offset_x: Option<f32>,
                            shadow_offset_y: Option<f32>,
                            shadow_blur_radius: Option<f32>,
                            gen_id: Option<usize>,
                            ) -> PyResult<usize>
    {
        let id = self.get_id(gen_id);

        let base: Option<Color> = get_color(base_rgba, base_color, 1.0, false);
        let border_color: Option<Color> = get_color(border_rgba, border_color, 1.0, false);
        let shadow_color: Option<Color> = get_color(shadow_rgba, shadow_color, 1.0, false);

        let mut state = access_state();

        state.menu_bar_style.insert(style_id, IpgMenuBarStyle::new( 
                                                    id,
                                                    base,
                                                    border_color,
                                                    border_radius,
                                                    border_width,
                                                    shadow_color,
                                                    shadow_offset_x,
                                                    shadow_offset_y,
                                                    shadow_blur_radius,
                                                    ));
        state.last_id = id;
        drop(state);
        Ok(id)
    }

    #[pyo3(signature = (style_id,
                        base_color=None,
                        base_rgba=None,
                        border_color=None,
                        border_rgba=None,
                        border_radius=None,
                        border_width=None,
                        shadow_color=None,
                        shadow_rgba=None,
                        shadow_offset_x=None,
                        shadow_offset_y=None,
                        shadow_blur_radius=None,
                        path_base_color=None,
                        path_base_rgba=None,
                        path_border_color=None,
                        path_border_rgba=None,
                        path_border_radius=None,
                        path_border_width=None,
                        gen_id=None))]
    fn add_menu_style(&mut self,
                            style_id: String,
                            base_color: Option<IpgColor>,
                            base_rgba: Option<[f32; 4]>,
                            border_color: Option<IpgColor>,
                            border_rgba: Option<[f32; 4]>,
                            border_radius: Option<Vec<f32>>,
                            border_width: Option<f32>,
                            shadow_color: Option<IpgColor>,
                            shadow_rgba: Option<[f32; 4]>,
                            shadow_offset_x: Option<f32>,
                            shadow_offset_y: Option<f32>,
                            shadow_blur_radius: Option<f32>,
                            path_base_color: Option<IpgColor>,
                            path_base_rgba: Option<[f32; 4]>,
                            path_border_color: Option<IpgColor>,
                            path_border_rgba: Option<[f32; 4]>,
                            path_border_radius: Option<Vec<f32>>,
                            path_border_width: Option<f32>,
                            gen_id: Option<usize>,
                            ) -> PyResult<usize>
    {
        let id = self.get_id(gen_id);
        
        let base: Option<Color> = get_color(base_rgba, base_color, 1.0, false);
        let border_color: Option<Color> = get_color(border_rgba, border_color, 1.0, false);
        let shadow_color: Option<Color> = get_color(shadow_rgba, shadow_color, 1.0, false);
        let path_base: Option<Color> = get_color(path_base_rgba, path_base_color, 1.0, false);
        let path_border_color: Option<Color> = get_color(path_border_rgba, path_border_color, 1.0, false);

        let mut state = access_state();

        state.menu_style.insert(style_id, IpgMenuStyle::new( 
                                                    id,
                                                    base,
                                                    border_color,
                                                    border_radius,
                                                    border_width,
                                                    shadow_color,
                                                    shadow_offset_x,
                                                    shadow_offset_y,
                                                    shadow_blur_radius,
                                                    path_base,
                                                    path_border_color,
                                                    path_border_radius,
                                                    path_border_width,
                                                    ));
        state.last_id = id;
        drop(state);
        Ok(id)
    }

    #[pyo3(signature = (style_id,
                        separator_type,
                        height=20.0,
                        height_fill=false,
                        width=None,
                        width_fill=true,
                        quad_ratios=None,
                        separator_color=None,
                        separator_rgba=None,
                        separator_border_color=None,
                        separator_border_rgba=None,
                        separator_border_width=None,
                        separator_border_radius=None,
                        separator_shadow_color=None,
                        separator_shadow_rgba=None,
                        separator_shadow_offset=None,
                        separator_shadow_blur_radius=None,
                        background_color=None,
                        background_rgba=None,
                        background_border_color=None,
                        background_border_rgba=None,
                        background_border_width=None,
                        background_border_radius=None,
                        background_shadow_color=None,
                        background_shadow_rgba=None,
                        background_shadow_offset=None,
                        background_shadow_blur_radius=None,
                        gen_id=None,
                        ))]
    fn add_menu_separator_style(&mut self,
                                style_id: String,
                                separator_type: IpgMenuSeparatorType,
                                height: f32,
                                height_fill: bool,
                                width: Option<f32>,
                                width_fill: bool,
                                quad_ratios: Option<[f32; 2]>,
                                separator_color: Option<IpgColor>,
                                separator_rgba: Option<[f32; 4]>,
                                separator_border_color: Option<IpgColor>,
                                separator_border_rgba: Option<[f32; 4]>,
                                separator_border_width: Option<f32>,
                                separator_border_radius: Option<Vec<f32>>,
                                separator_shadow_color: Option<IpgColor>,
                                separator_shadow_rgba: Option<[f32; 4]>,
                                separator_shadow_offset: Option<[f32; 2]>,
                                separator_shadow_blur_radius: Option<f32>,
                                background_color: Option<IpgColor>,
                                background_rgba: Option<[f32; 4]>,
                                background_border_color: Option<IpgColor>,
                                background_border_rgba: Option<[f32; 4]>,
                                background_border_width: Option<f32>,
                                background_border_radius: Option<Vec<f32>>,
                                background_shadow_color: Option<IpgColor>,
                                background_shadow_rgba: Option<[f32; 4]>,
                                background_shadow_offset: Option<[f32; 2]>,
                                background_shadow_blur_radius: Option<f32>,
                                gen_id: Option<usize>,
                                ) -> PyResult<usize> 
    {
        let id = self.get_id(gen_id);

        let width = get_width(width, width_fill);

        let height = get_height(Some(height), height_fill);

        let sep_color: Option<Color> = 
            get_color(separator_rgba, separator_color, 1.0, false);
        let sep_border_color = 
            get_color(separator_border_rgba, separator_border_color, 1.0, false);
        let sep_shadow_color = 
            get_color(separator_shadow_rgba, separator_shadow_color, 1.0, false);
        let bg_color: Option<Color> = 
            get_color(background_rgba, background_color, 1.0, false);
        let bg_border_color =
            get_color(background_border_rgba, background_border_color, 1.0, false);
        let bg_shadow_color =
            get_color(background_shadow_rgba, background_shadow_color, 1.0, false);

        let mut state = access_state();
        
        state.menu_separator_style.insert(style_id, 
                    IpgMenuSeparatorStyle::new( 
                                                id,
                                                separator_type,
                                                width,
                                                height,
                                                quad_ratios,
                                                sep_color,
                                                sep_border_color,
                                                separator_border_width,
                                                separator_border_radius,
                                                sep_shadow_color,
                                                separator_shadow_offset,
                                                separator_shadow_blur_radius,
                                                bg_color,
                                                bg_border_color,
                                                background_border_width,
                                                background_border_radius,
                                                bg_shadow_color,
                                                background_shadow_offset,
                                                background_shadow_blur_radius,
                                                ));
        state.last_id = id;
        drop(state);
        Ok(id)
    }

    #[pyo3(signature = (parent_id, options, gen_id=None, on_select=None, 
                        width=None, width_fill=false, padding=vec![5.0],  
                        placeholder=None, selected=None, text_size=None, 
                        text_line_height=1.2, text_shaping="basic".to_string(), 
                        handle=IpgPickListHandle::Default, arrow_size=None, 
                        dynamic_closed=None, dynamic_opened=None, custom_static=None,
                        style=None, user_data=None, show=true,
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
                        handle: IpgPickListHandle,
                        arrow_size: Option<f32>,
                        dynamic_closed: Option<IpgButtonArrow>,
                        dynamic_opened: Option<IpgButtonArrow>,
                        custom_static: Option<IpgButtonArrow>,
                        style: Option<String>,
                        user_data: Option<PyObject>,
                        show: bool,
                    ) -> PyResult<usize>
    {

        let id = self.get_id(gen_id);

        if on_select.is_some() {
            add_callback_to_mutex(id, "on_select".to_string(), on_select);
        }

        let padding = get_padding_f64(padding);

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
                                                        handle,
                                                        arrow_size,
                                                        dynamic_closed,
                                                        dynamic_opened,
                                                        custom_static,
                                                        style,
                                                    )));
        state.last_id = id;
        drop(state);
        Ok(id)
    }

#[pyo3(signature = (style_id,
                    background_color=None,
                    background_rgba=None,
                    text_color=None,
                    text_rgba=None,
                    handle_color=None,
                    handle_rgba=None,
                    placeholder_color=None,
                    placeholder_rgba=None,
                    border_color=None,
                    border_rgba=None,
                    border_color_hovered=None,
                    border_rgba_hovered=None,
                    border_radius=vec![2.0],
                    border_width=1.0,
                    gen_id=None))]
    fn add_pick_list_style(&mut self,
                            style_id: String,
                            background_color: Option<IpgColor>,
                            background_rgba: Option<[f32; 4]>,
                            text_color: Option<IpgColor>,
                            text_rgba: Option<[f32; 4]>,
                            handle_color: Option<IpgColor>,
                            handle_rgba: Option<[f32; 4]>,
                            placeholder_color: Option<IpgColor>,
                            placeholder_rgba: Option<[f32; 4]>,
                            border_color: Option<IpgColor>,
                            border_rgba: Option<[f32; 4]>,
                            border_color_hovered: Option<IpgColor>,
                            border_rgba_hovered: Option<[f32; 4]>,
                            border_radius: Vec<f32>,
                            border_width: f32,
                            gen_id: Option<usize>,
                            ) -> PyResult<usize>
    {
        let id = self.get_id(gen_id);
        
        let background_color: Option<Color> = get_color(background_rgba, background_color, 1.0, false);
        let border_color: Option<Color> = get_color(border_rgba, border_color, 1.0, false);
        let border_color_hovered: Option<Color> = get_color(border_rgba_hovered, border_color_hovered, 1.0, false);
        let handle_color: Option<Color> = get_color(handle_rgba, handle_color, 1.0, false);
        let placeholder_color = get_color(placeholder_rgba, placeholder_color, 1.0, false);
        let text_color = get_color(text_rgba, text_color, 1.0, false);

        let mut state = access_state();

        state.pick_list_style.insert(style_id, IpgPickListStyle::new( 
                                                    id,
                                                    background_color,
                                                    text_color,
                                                    handle_color,
                                                    placeholder_color,
                                                    border_color,
                                                    border_color_hovered,
                                                    border_radius,
                                                    border_width,
                                                    ));
        state.last_id = id;
        drop(state);
        Ok(id)
    }

    #[pyo3(signature = (parent_id, min, max, value,
                        gen_id=None, width=None, height=Some(16.0), 
                        width_fill=true, height_fill=false,
                        style_standard=None, style=None, 
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
                            style_standard: Option<IpgStyleStandard>,
                            style: Option<String>,
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
                                                style_standard,
                                                style,
                                            )));
        state.last_id = id;
        drop(state);
        Ok(id)

    }

    #[pyo3(signature = (style_id, 
                        background_color=None, background_rgba=None,
                        bar_color=None, bar_rgba=None,
                        border_color=None, border_rgba=None,
                        border_radius=None, border_width=None,
                        gen_id=None))]
    fn add_progress_bar_style(&mut self,
                                style_id: String,
                                background_color: Option<IpgColor>,
                                background_rgba: Option<[f32; 4]>,
                                bar_color: Option<IpgColor>,
                                bar_rgba: Option<[f32; 4]>,
                                border_color: Option<IpgColor>,
                                border_rgba: Option<[f32; 4]>,
                                border_radius: Option<Vec<f32>>,
                                border_width: Option<f32>,
                                gen_id: Option<usize>,
                                ) -> PyResult<usize>
    {
        let id = self.get_id(gen_id);

        let mut state = access_state();

        let background: Option<Color> = get_color(background_rgba, background_color, 1.0, false);
        let bar_color: Option<Color> = get_color(bar_rgba, bar_color, 1.0, false);
        let border_color: Option<Color> = get_color(border_rgba, border_color, 1.0, false);

        state.progress_bar_style.insert(style_id, IpgProgressBarStyle::new( 
                                                    id,
                                                    background,
                                                    bar_color,
                                                    border_color,
                                                    border_radius,
                                                    border_width,
                                                    ));
        state.last_id = id;
        drop(state);
        Ok(id)
    }

    #[pyo3(signature = (parent_id, labels, gen_id=None,
                        direction=IpgRadioDirection::Vertical, 
                        spacing= 10.0, padding=vec![10.0], 
                        width=None, width_fill=false, height=None, height_fill=false,
                        on_select=None, selected_index=None, 
                        size=20.0, style_id=None,
                        text_spacing=15.0, text_size=16.0,
                        text_line_height_pixels=None,
                        text_line_height_relative=None, 
                        text_shaping="basic".to_string(), 
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
                    style_id: Option<String>,
                    text_spacing: f32,
                    text_size: f32,
                    text_line_height_pixels: Option<u16>,
                    text_line_height_relative: Option<f32>,
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

        let padding = get_padding_f64(padding);

        if on_select.is_some() {
            add_callback_to_mutex(id, "on_select".to_string(), on_select);
        }

        let text_line_height = get_line_height(text_line_height_pixels, text_line_height_relative);
        
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
                                        style_id,
                                    )));
        self.group_index += 1;
        state.last_id = id;
        drop(state);                                      
        Ok(id)

    }

    #[pyo3(signature = (style_id,
                        background_color=None,
                        background_rgba=None,
                        background_color_hovered=None,
                        background_rgba_hovered=None,
                        border_color=None, 
                        border_rgba=None,
                        border_width=None,
                        dot_color=None, 
                        dot_rgba=None,
                        dot_color_hovered=None, 
                        dot_rgba_hovered=None,
                        text_color=None, 
                        text_rgba=None,
                        gen_id=None))]
    fn add_radio_style(&mut self,
                            style_id: String,
                            background_color: Option<IpgColor>,
                            background_rgba: Option<[f32; 4]>,
                            background_color_hovered: Option<IpgColor>,
                            background_rgba_hovered: Option<[f32; 4]>,
                            border_color: Option<IpgColor>,
                            border_rgba: Option<[f32; 4]>,
                            border_width: Option<f32>,
                            dot_color: Option<IpgColor>,
                            dot_rgba: Option<[f32; 4]>,
                            dot_color_hovered: Option<IpgColor>,
                            dot_rgba_hovered: Option<[f32; 4]>,
                            text_color: Option<IpgColor>,
                            text_rgba: Option<[f32; 4]>,
                            gen_id: Option<usize>,
                            ) -> PyResult<usize>
    {
        let id = self.get_id(gen_id);

        let mut state = access_state();

        let background_color = get_color(background_rgba, background_color, 1.0, false);
        let background_color_hovered = get_color(background_rgba_hovered, background_color_hovered, 1.0, false);
        let dot_color: Option<Color> = get_color(dot_rgba, dot_color, 1.0, false);
        let dot_color_hovered: Option<Color> = get_color(dot_rgba_hovered, dot_color_hovered, 1.0, false);
        let border_color: Option<Color> = get_color(border_rgba, border_color, 1.0, false);
        let text_color: Option<Color> = get_color(text_rgba, text_color, 1.0, false);

        state.radio_style.insert(style_id, IpgRadioStyle::new( 
                                                    id,
                                                    background_color,
                                                    background_color_hovered,
                                                    dot_color,
                                                    dot_color_hovered,
                                                    border_color,
                                                    border_width,
                                                    text_color,
                                                    ));
        state.last_id = id;
        drop(state);
        Ok(id)
    }

    #[pyo3(signature = (parent_id, width, 
                        width_fill=true, 
                        thickness=1,
                        style=None,
                        gen_id=None, 
                        ))]
    fn add_rule_horizontal(&mut self, 
                            parent_id: String,
                            width: Option<f32>,
                            width_fill: bool,
                            thickness: u16,
                            style: Option<String>,
                            gen_id: Option<usize>, 
                            ) -> PyResult<usize> 
    {
        let id = self.get_id(gen_id);

        let rule_type = "h".to_string();

        let width = get_width(width, width_fill);
        let height = get_height(None, true);

        set_state_of_widget(id, parent_id);

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgRule(IpgRule::new(
                                                        id,
                                                        width,
                                                        height,
                                                        thickness,
                                                        rule_type,
                                                        style,
                                                        )));
        state.last_id = id;
        drop(state);
        Ok(id)
    }

    #[pyo3(signature = (parent_id, height=None, 
                        height_fill=true, thickness=1,
                        style=None, gen_id=None 
                        ))]
    fn add_rule_vertical(&mut self, 
                            parent_id: String,
                            height: Option<f32>,
                            height_fill: bool,
                            thickness: u16,
                            style: Option<String>,
                            gen_id: Option<usize>, 
                            ) -> PyResult<usize> 
    {
        let id = self.get_id(gen_id);

        let rule_type = "v".to_string();

        let width = get_width(None, true); // not used for vertical, just defaulted
        let height = get_height(height, height_fill);

        set_state_of_widget(id, parent_id);

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgRule(IpgRule::new(
                                                        id,
                                                        width,
                                                        height,
                                                        thickness,
                                                        rule_type,
                                                        style, 
                                                        )));
        state.last_id = id;
        drop(state);
        Ok(id)
    }

    #[pyo3(signature = (style_id,
                        color=None, 
                        color_rgba=None,
                        border_radius=None,
                        fillmode_percent=None,
                        fillmode_padded=None,
                        fillmode_asymmetric_padding=None,
                        gen_id=None))]
    fn add_rule_style(&mut self,
                            style_id: String,
                            color: Option<IpgColor>,
                            color_rgba: Option<[f32; 4]>,
                            border_radius: Option<Vec<f32>>,
                            fillmode_percent: Option<f32>,
                            fillmode_padded: Option<u16>,
                            fillmode_asymmetric_padding: Option<Vec<u16>>,
                            gen_id: Option<usize>,
                            ) -> PyResult<usize>
    {
        let id = self.get_id(gen_id);

        let mut state = access_state();

        let color = get_color(color_rgba, color, 1.0, false);
        
        state.rule_style.insert(style_id, IpgRuleStyle::new( 
                                                    id,
                                                    color,
                                                    border_radius,
                                                    fillmode_percent,
                                                    fillmode_padded,
                                                    fillmode_asymmetric_padding,
                                                    ));
        state.last_id = id;
        drop(state);
        Ok(id)
    }

    #[pyo3(signature = (parent_id, text, gen_id=None, 
                        on_press=None, on_release=None, 
                        on_right_press=None, on_right_release=None, 
                        on_middle_press=None, on_middle_release=None, 
                        on_move=None, on_enter=None, on_exit=None, 
                        width=None, width_fill=false,
                        height=None, height_fill=false, 
                        h_align=IpgHorizontalAlignment::Left, 
                        v_align=IpgVerticalAlignment::Top, 
                        line_height=1.3, size=16.0,
                        text_color=None, text_rgba=None, 
                        show=true, shaping="basic".to_string(), 
                        user_data=None
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
                            width_fill: bool,
                            height: Option<f32>,
                            height_fill: bool,
                            h_align: IpgHorizontalAlignment,
                            v_align: IpgVerticalAlignment,
                            line_height: f32,
                            size: f32,
                            text_color: Option<IpgColor>,
                            text_rgba: Option<[f32; 4]>,
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

        let text_color = get_color(text_rgba, text_color, 1.0, false);

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
                                                    text_color,
                                                    user_data,
                                                    )));
        state.last_id = id;
        drop(state);
        Ok(id)

    }

    #[pyo3(signature = (parent_id, min, max, step, value, 
                        gen_id=None, width=None, height=None, 
                        width_fill=false, on_change=None, 
                        on_release=None, style=None,
                        user_data=None, show=true, 
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
                        style: Option<String>,
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
                                                style,
                                                )));
        state.last_id = id;
        drop(state);
        Ok(id)
    }

    #[pyo3(signature = (style_id,
                        rail_color=None,
                        rail_rgba=None,
                        rail_color_hovered=None,
                        rail_rgba_hovered=None,
                        rail_width=None,
                        rail_border_radius=None,
                        handle_circle_radius=None,
                        handle_rectangle_width=None,
                        handle_rectangle_border_radius=None,
                        handle_color=None,
                        handle_rgba=None,
                        handle_border_width=None,
                        handle_border_color=None,
                        handle_border_rgba=None,
                        gen_id=None,
                        ))]
    fn add_slider_style(&mut self,
                        style_id: String,
                        rail_color: Option<IpgColor>,
                        rail_rgba: Option<[f32; 4]>,
                        rail_color_hovered: Option<IpgColor>,
                        rail_rgba_hovered: Option<[f32; 4]>,
                        rail_width: Option<f32>,
                        rail_border_radius: Option<Vec<f32>>,
                        handle_circle_radius: Option<f32>,
                        handle_rectangle_width: Option<u16>,
                        handle_rectangle_border_radius: Option<Vec<f32>>,
                        handle_color: Option<IpgColor>,
                        handle_rgba: Option<[f32; 4]>,
                        handle_border_width: Option<f32>,
                        handle_border_color: Option<IpgColor>,
                        handle_border_rgba: Option<[f32; 4]>,
                        gen_id: Option<usize>,
                        )  -> PyResult<usize>
    {
        let id = self.get_id(gen_id);

        let mut state = access_state();

        
        let rail_color = get_color(rail_rgba, rail_color, 1.0, false);
        let rail_color_hovered = get_color(rail_rgba_hovered, rail_color_hovered, 1.0, false);
        let handle_color = get_color(handle_rgba, handle_color, 1.0, false);
        let handle_border_color = get_color(handle_border_rgba,handle_border_color,1.0, false);
        
        state.slider_style.insert(style_id, IpgSliderStyle::new( 
                                                    id,
                                                    rail_color,
                                                    rail_color_hovered,
                                                    rail_width,
                                                    rail_border_radius,
                                                    handle_circle_radius,
                                                    handle_rectangle_width,
                                                    handle_rectangle_border_radius,
                                                    handle_color,
                                                    handle_border_width,
                                                    handle_border_color,
                                                    ));
        state.last_id = id;
        drop(state);
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
        state.last_id = id;
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
        state.last_id = id;
        drop(state);
        Ok(id)
    }

    #[pyo3(signature = (window_id, table_id, title, data, 
                        data_length, width, height, parent_id=None,
                        row_highlight=None, highlight_amount=0.15,
                        column_widths=vec![50.0],
                        button_fill_columns=None,
                        checkbox_fill_columns=None,
                        toggler_fill_columns=None,
                        mixed_widgets_columns=None,
                        button_fill_style_id=None,
                        checkbox_fill_style_id=None,
                        toggler_fill_style_id=None,
                        mixed_widgets_column_style_ids=None,
                        gen_id=None, 
                        on_button=None, 
                        on_checkbox=None,
                        on_toggler=None,
                        on_scroll=None, 
                        show=true,
                        button_user_data=None,
                        checkbox_user_data=None,
                        toggler_user_data=None,
                        scroller_user_data=None,
                        ))]
    fn add_table(&mut self,
                    window_id: String,
                    table_id: String,
                    title: String,
                    data: PyObject,
                    data_length: usize,
                    width: f32,
                    height: f32,
                    // **above required
                    parent_id: Option<String>,
                    row_highlight: Option<IpgTableRowHighLight>,
                    highlight_amount: f32,
                    column_widths: Vec<f32>,
                    button_fill_columns: Option<Vec<usize>>,
                    checkbox_fill_columns: Option<Vec<usize>>,
                    toggler_fill_columns: Option<Vec<usize>>,
                    mixed_widgets_columns: Option<HashMap<usize, Vec<IpgTableWidget>>>,
                    button_fill_style_id: Option<String>,
                    checkbox_fill_style_id: Option<String>,
                    toggler_fill_style_id: Option<String>,
                    mixed_widgets_column_style_ids: Option<HashMap<usize, Vec<String>>>,
                    gen_id: Option<usize>,
                    on_button: Option<PyObject>,
                    on_checkbox: Option<PyObject>,
                    on_toggler: Option<PyObject>,
                    on_scroll: Option<PyObject>,
                    show: bool,
                    button_user_data: Option<PyObject>,
                    checkbox_user_data: Option<PyObject>,
                    toggler_user_data: Option<PyObject>,
                    scroller_user_data: Option<PyObject>,
                ) -> PyResult<usize> 
    {

        let id = self.get_id(gen_id);

        let prt_id = match parent_id {
            Some(id) => id,
            None => window_id.clone(),
        };

        set_state_of_container(id, window_id.clone(), Some(table_id.clone()), prt_id);

        let mut state = access_state();

        set_state_cont_wnd_ids(&mut state, &window_id, table_id, id, "add_table".to_string());

        // Need to generate the ids for the widgets and the boolean values
        // Keeping the ids organized in a hashmap for now, may need only a vec.
        let mut button_ids: Vec<(usize, usize, usize, bool)> = vec![]; // (id, row, col, bool)
        let mut checkbox_ids: Vec<(usize, usize, usize, bool)> = vec![];
        let mut toggler_ids: Vec<(usize, usize, usize, bool)> = vec![];
            
        if mixed_widgets_columns.is_some() {
            let table_widgets_hash = mixed_widgets_columns.unwrap();
            for (col, table_widgets) in table_widgets_hash.iter() {
                for (row, widget) in table_widgets.iter().enumerate() {
                    match widget {
                        IpgTableWidget::Button => {
                            button_ids.push((self.get_id(None), row, col.clone(), false));
                        },
                        IpgTableWidget::Checkbox => {
                            checkbox_ids.push((self.get_id(None), row, col.clone(), false));
                        },
                        IpgTableWidget::Toggler => {
                            toggler_ids.push((self.get_id(None), row, col.clone(), false));
                        },
                    }
                }
            }
        }

        if button_fill_columns.is_some() {
            for col in button_fill_columns.unwrap() {
                for row in 0..data_length {
                    button_ids.push((self.get_id(None), row, col, false));
                }
            }
        }

        if checkbox_fill_columns.is_some() {
            for col in checkbox_fill_columns.unwrap() {
                for row in 0..data_length {
                    checkbox_ids.push((self.get_id(None), row, col, false));
                }
            }
        }

        if toggler_fill_columns.is_some() {
            for col in toggler_fill_columns.unwrap() {
                for row in 0..data_length {
                    toggler_ids.push((self.get_id(None), row, col, false));
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

        if on_scroll.is_some() {
            add_callback_to_mutex(id, "on_scroll".to_string(), on_scroll);
        }

        let scroller_id = TABLE_INTERNAL_IDS_START + id;

        state.containers.insert(id, IpgContainers::IpgTable(IpgTable::new( 
                                                    id,
                                                    title,
                                                    data,
                                                    data_length,
                                                    width,
                                                    height,
                                                    row_highlight,
                                                    highlight_amount,
                                                    column_widths,
                                                    button_ids,
                                                    checkbox_ids,
                                                    toggler_ids,
                                                    button_fill_style_id,
                                                    checkbox_fill_style_id,
                                                    toggler_fill_style_id,
                                                    mixed_widgets_column_style_ids,
                                                    show,
                                                    button_user_data,
                                                    checkbox_user_data,
                                                    toggler_user_data,
                                                    scroller_user_data,
                                                    scroller_id,
                                                    )));
        state.last_id = self.id;
        drop(state);
        Ok(id)

    }

    #[pyo3(signature = (parent_id, content, gen_id=None, 
                        width=None, width_fill=false, 
                        height=None, height_fill=false,
                        horizontal_alignment=None, 
                        vertical_alignment=None,
                        line_height=1.3, size=16.0, 
                        shaping="basic".to_string(), 
                        text_color=None, text_rgba=None,
                        show=true,
                        ))]
    fn add_text(&mut self,
                    parent_id: String,
                    content: String,
                    // ** above required
                    gen_id: Option<usize>,
                    width: Option<f32>,
                    width_fill: bool,
                    height: Option<f32>,
                    height_fill: bool,
                    horizontal_alignment: Option<IpgHorizontalAlignment>,
                    vertical_alignment: Option<IpgVerticalAlignment>,
                    line_height: f32,
                    size: f32,
                    shaping: String,
                    text_color: Option<IpgColor>,
                    text_rgba: Option<[f32; 4]>,
                    show: bool,
                    ) -> PyResult<usize> 
    {
    
        let id = self.get_id(gen_id);

        let width = get_width(width, width_fill);
        let height = get_height(height, height_fill);

        let horizontal_alignment =  get_horizontal_alignment(horizontal_alignment);
        let vertical_alignment = get_vertical_alignment(vertical_alignment);
        
        let line_height = LineHeight::Relative(line_height);

        let shaping = get_shaping(shaping);

        let style = get_color(text_rgba, text_color, 1.0, false);

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
                                        style,
                                    )));
        state.last_id = id;
        drop(state);
        Ok(id)

    }

    #[pyo3(signature = (parent_id, placeholder, gen_id=None,
                        on_input=None, on_submit=None, 
                        on_paste=None, width=None, width_fill=false, 
                        padding=vec![0.0], 
                        size=16.0, 
                        line_height_pixels=None,
                        line_height_relative=None, 
                        user_data=None, is_secure=false, 
                        style_id=None, show=true,
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
                            line_height_pixels: Option<u16>,
                            line_height_relative: Option<f32>,
                            user_data: Option<PyObject>,
                            is_secure: bool,
                            style_id: Option<String>,
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
        
        let padding = get_padding_f64(padding);

        let width = get_width(width, width_fill);

        let line_height = get_line_height(line_height_pixels, line_height_relative);

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
                                                                style_id,
                                                                show,
                                                                )));
        state.last_id = id;
        drop(state);
        Ok(id)
    }

    #[pyo3(signature = (style_id, 
                        background_color=None,
                        background_rgba=None,
                        border_color=None,
                        border_rgba=None,
                        border_color_hovered=None,
                        border_rgba_hovered=None,
                        border_color_focused=None,
                        border_rgba_focused=None,
                        border_width=None,
                        border_radius=None,
                        // icon_color=None,
                        // icon_rgba=None,
                        placeholder_color=None,
                        placeholder_rgba=None,
                        value_color=None,
                        value_rgba=None,
                        selection_color=None,
                        selection_rgba=None,
                        gen_id=None))]
    fn add_text_input_style(&mut self,
                                style_id: String,
                                background_color: Option<IpgColor>,
                                background_rgba: Option<[f32; 4]>,
                                border_color: Option<IpgColor>,
                                border_rgba: Option<[f32; 4]>,
                                border_color_hovered: Option<IpgColor>,
                                border_rgba_hovered: Option<[f32; 4]>,
                                border_color_focused: Option<IpgColor>,
                                border_rgba_focused: Option<[f32; 4]>,
                                border_width: Option<f32>,
                                border_radius: Option<Vec<f32>>,
                                // icon_color: Option<IpgColor>,
                                // icon_rgba: Option<[f32; 4]>,
                                placeholder_color: Option<IpgColor>,
                                placeholder_rgba: Option<[f32; 4]>,
                                value_color: Option<IpgColor>,
                                value_rgba: Option<[f32; 4]>,
                                selection_color: Option<IpgColor>,
                                selection_rgba: Option<[f32; 4]>,
                                gen_id: Option<usize>,
                                ) -> PyResult<usize>
    {
        let id = self.get_id(gen_id);

        let background_color = get_color(background_rgba, background_color, 1.0, false);
        let border_color = get_color(border_rgba, border_color, 1.0, false);
        let border_color_hovered = get_color(border_rgba_hovered, border_color_hovered, 1.0, false);
        let border_color_focused = get_color(border_rgba_focused, border_color_focused, 1.0, false);
        // let icon_color = get_color(icon_rgba, icon_color, 1.0, false);
        let placeholder_color = get_color(placeholder_rgba, placeholder_color, 1.0, false);
        let value_color = get_color(value_rgba, value_color, 1.0, false);
        let selection_color = get_color(selection_rgba, selection_color, 1.0, false);

        let mut state = access_state();
       
        state.text_input_style.insert(style_id, IpgTextInputStyle::new( 
                                                id,
                                                background_color,
                                                border_color,
                                                border_color_hovered,
                                                border_color_focused,
                                                border_width,
                                                border_radius,
                                                // icon_color,
                                                placeholder_color,
                                                value_color,
                                                selection_color,
                                                ));
        state.last_id = id;                                        
        drop(state);
        Ok(id)
    }

    #[pyo3(signature = (parent_id, duration_ms, on_start=None, on_tick=None, on_stop=None, 
                        start_label="Start Timer".to_string(), 
                        stop_label="Stop Timer".to_string(), width=None, height=None, 
                        width_fill=false, height_fill=false, padding=vec![10.0], 
                        button_style_id=None, button_style_standard=None, button_style_arrow=None, 
                        user_data=None, gen_id=None))]
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
                        button_style_id: Option<String>,
                        button_style_standard: Option<IpgStyleStandard>,
                        button_style_arrow: Option<IpgButtonArrow>,
                        user_data: Option<PyObject>,
                        gen_id: Option<usize>,
                    ) -> PyResult<usize>
    {
        let id = self.get_id(gen_id);

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

        let padding = get_padding_f64(padding);

        set_state_of_widget(self.id, parent_id);
        
        let mut state = access_state();

        state.widgets.insert(self.id, IpgWidgets::IpgTimer(IpgTimer::new(
                                                            id,
                                                            duration_ms,
                                                            start_label,
                                                            stop_label,
                                                            width,
                                                            height,
                                                            padding,
                                                            button_style_id,
                                                            button_style_standard,
                                                            button_style_arrow,
                                                            user_data, 
                                                            )));
        state.last_id = id;
        drop(state);
        Ok(id)
    }

    #[pyo3(signature = (parent_id, label=None, gen_id=None, toggled=None, 
                        width=None, width_fill=false, size=20.0, text_size=16.0,
                        text_line_height=1.3, text_alignment=IpgHorizontalAlignment::Center, 
                        spacing=10.0, user_data=None, show=true, style_id=None, 
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
                        text_alignment: IpgHorizontalAlignment,
                        spacing: f32,
                        user_data: Option<PyObject>,
                        show: bool,
                        style_id: Option<String>,
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
                                                style_id,                           
                                                )));
        state.last_id = id;
        drop(state);
        Ok(id)
    
    }

    #[pyo3(signature = (style_id, 
                        background_color=None,
                        background_rgba=None,
                        background_color_toggled=None,
                        background_rgba_toggled=None,
                        background_border_color=None,
                        background_border_rgba=None,
                        background_border_width=None,
                        foreground_color=None,
                        foreground_rgba=None,
                        foreground_color_toggled=None,
                        foreground_rgba_toggled=None,
                        foreground_border_color=None,
                        foreground_border_rgba=None,
                        foreground_border_width=None,
                        gen_id=None,
                        ))]
    fn add_toggler_style(&mut self,
                        style_id: String, 
                        background_color: Option<IpgColor>,
                        background_rgba: Option<[f32; 4]>,
                        background_color_toggled: Option<IpgColor>,
                        background_rgba_toggled: Option<[f32; 4]>,
                        background_border_color: Option<IpgColor>,
                        background_border_rgba: Option<[f32; 4]>,
                        background_border_width: Option<f32>,
                        foreground_color: Option<IpgColor>,
                        foreground_rgba: Option<[f32; 4]>,
                        foreground_color_toggled: Option<IpgColor>,
                        foreground_rgba_toggled: Option<[f32; 4]>,
                        foreground_border_color: Option<IpgColor>,
                        foreground_border_rgba: Option<[f32; 4]>,
                        foreground_border_width: Option<f32>,
                        gen_id: Option<usize>,
                        ) -> PyResult<usize>
    {
        let id = self.get_id(gen_id);

        let background_color = get_color(background_rgba, background_color, 1.0, false);
        let background_color_toggled = get_color(background_rgba_toggled, background_color_toggled, 1.0, false);
        let background_border_color = get_color(background_border_rgba, background_border_color, 1.0, false);
        let foreground_color = get_color(foreground_rgba, foreground_color, 1.0, false);
        let foreground_color_toggled = get_color(foreground_rgba_toggled, foreground_color_toggled, 1.0, false);
        let foreground_border_color = get_color(foreground_border_rgba, foreground_border_color, 1.0, false);
        

        let mut state = access_state();
       
        state.toggler_style.insert(style_id, IpgTogglerStyle::new( 
                                                id,
                                                background_color,
                                                background_color_toggled,
                                                background_border_color,
                                                background_border_width,
                                                foreground_color,
                                                foreground_color_toggled,
                                                foreground_border_color,
                                                foreground_border_width,
                                                ));
        state.last_id = id;
        drop(state);
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

        let mut callbacks = access_callbacks();

        match on_key_press {
            Some(py) => {
                callbacks.callback_events.insert((self.id, "key press".to_string()), py);
            },
            None => (),
        }
        match on_key_release {
            Some(py) => {
                callbacks.callback_events.insert((self.id, "key release".to_string()), py);
            },
            None => (),
        }

        callbacks.user_data.push((self.id, user_data));

        drop(callbacks);
        
        let mut state = access_state();

        state.events.push(IpgEvents::Keyboard(IpgKeyBoardEvent::new(
                                                                    self.id,
                                                                    enabled, 
                                                                    )));
        state.last_id = self.id;
        drop(state);
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

        let mut callbacks = access_callbacks();

        match on_move {
            Some(py) => {
                callbacks.callback_events.insert((self.id, "move".to_string()), py);
            },
            None => (),
        }
        match on_enter_window {
            Some(py) => {
                callbacks.callback_events.insert((self.id, "enter window".to_string()), py);
            },
            None => (),
        }
        match on_exit_window {
            Some(py) => {
                callbacks.callback_events.insert((self.id, "exit window".to_string()), py);
            },
            None => (),
        }
        match on_left_press {
            Some(py) => {
                callbacks.callback_events.insert((self.id, "left press".to_string()), py);
            },
            None => (),
        }
        match on_left_release {
            Some(py) => {
                callbacks.callback_events.insert((self.id, "left release".to_string()), py);
            },
            None => (),
        }
        match on_middle_press {
            Some(py) => {
                callbacks.callback_events.insert((self.id, "middle press".to_string()), py);
            },
            None => (),
        }
        match on_middle_release {
            Some(py) => {
                callbacks.callback_events.insert((self.id, "middle release".to_string()), py);
            },
            None => (),
        }
        match on_right_press {
            Some(py) => {
                callbacks.callback_events.insert((self.id, "right press".to_string()), py);
            },
            None => (),
        }
        match on_right_release {
            Some(py) => {
                callbacks.callback_events.insert((self.id, "right release".to_string()), py);
            },
            None => (),
        }
        match on_middle_scroll {
            Some(py) => {
                callbacks.callback_events.insert((self.id, "middle scrollLine".to_string()), py);
            },
            None => (),
        }

        callbacks.user_data.push((self.id, user_data));

        drop(callbacks);
        
        let mut state = access_state();

        state.events.push(IpgEvents::Mouse(IpgMouseEvent::new(
                                                            self.id,
                                                            enabled, 
                                                            )));
        state.last_id = self.id;
        drop(state);
        Ok(self.id)
    }

    #[pyo3(signature = (enabled=false, on_closed=None, 
                        on_moved=None, on_resized=None,
                        on_redraw_requested=None,
                        on_close_requested=None,
                        on_focused=None, on_unfocused=None,
                        on_file_hovered=None,
                        on_file_dropped=None,
                        on_files_hovered_left=None,
                        user_data=None))]
    fn add_event_window(&mut self,
                        enabled: bool,
                        on_closed: Option<PyObject>,
                        on_moved: Option<PyObject>,
                        on_resized: Option<PyObject>,
                        on_redraw_requested: Option<PyObject>,
                        on_close_requested: Option<PyObject>,
                        on_focused: Option<PyObject>,
                        on_unfocused: Option<PyObject>,
                        on_file_hovered: Option<PyObject>,
                        on_file_dropped: Option<PyObject>,
                        on_files_hovered_left: Option<PyObject>,
                        user_data: Option<PyObject>,
                        ) -> PyResult<usize>
    {
        self.id += 1;

        let mut callbacks = access_callbacks();

        match on_closed {
            Some(py) => {
                callbacks.callback_events.insert((self.id, "closed".to_string()), py);
            },
            None => (),
        }
        match on_moved {
            Some(py) => {
                callbacks.callback_events.insert((self.id, "moved".to_string()), py);
            },
            None => (),
        }
        match on_resized {
            Some(py) => {
                callbacks.callback_events.insert((self.id, "resized".to_string()), py);
            },
            None => (),
        }
        match on_redraw_requested {
            Some(py) => {
                callbacks.callback_events.insert((self.id, "redraw requested".to_string()), py);
            },
            None => (),
        }
        match on_close_requested {
            Some(py) => {
                callbacks.callback_events.insert((self.id, "close requested".to_string()), py);
            },
            None => (),
        }
        match on_focused {
            Some(py) => {
                callbacks.callback_events.insert((self.id, "focused".to_string()), py);
            },
            None => (),
        }
        match on_unfocused {
            Some(py) => {
                callbacks.callback_events.insert((self.id, "unfocused".to_string()), py);
            },
            None => (),
        }
        match on_file_hovered {
            Some(py) => {
                callbacks.callback_events.insert((self.id, "file hovered".to_string()), py);
            },
            None => (),
        }
        match on_file_dropped {
            Some(py) => {
                callbacks.callback_events.insert((self.id, "file dropped".to_string()), py);
            },
            None => (),
        }
        match on_files_hovered_left {
            Some(py) => {
                callbacks.callback_events.insert((self.id, "files hovered left".to_string()), py);
            },
            None => (),
        }

        callbacks.user_data.push((self.id, user_data));

        drop(callbacks);
        
        let mut state = access_state();

        state.events.push(IpgEvents::Window(IpgWindowEvent::new(
                                                            self.id,
                                                            enabled, 
                                                            )));
        state.last_id = self.id;
        drop(state);
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
        drop(state);

    }


    #[pyo3(signature = (wid, item, value))]
    fn update_item(&self, wid: usize, item: PyObject, value: PyObject) {

        let mut state = access_state();

        let widget = state.widgets.get_mut(&wid);

        if widget.is_some() {
            match_widget(widget.unwrap(), item, value);
                drop(state);
        } else {
            match state.containers.get_mut(&wid) {

                Some(cnt) => {
                    match_container(cnt, item.clone(), value.clone());
                    drop(state);
                },
                None => panic!("Item_update: Widget, Container, or Window with id {wid} not found.")
            }
        }

    }

    #[pyo3(signature = (base_color=None, base_rgba=None))]
    fn get_color_palette(&mut self, 
                            base_color: Option<IpgColor>,
                            base_rgba: Option<[f32; 4]>,
                        ) -> PyResult<([f32; 4], [f32; 4], [f32; 4])>
    {
        let base: Option<Color> = get_color(base_rgba, base_color, 1.0, false);

        let text_color = readable(base.unwrap(), Color::WHITE);

        let palette = iced::theme::palette::Background::new(base.unwrap(), text_color);

        let color = palette.strong.color;
        let strong = [color.r, color.g, color.b, color.a];
        let color = palette.weak.color;
        let weak = [color.r, color.g, color.b, color.a];
        let color = text_color;
        let text = [color.r, color.g, color.b, color.a];

        Ok((strong, weak, text)) 
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
        IpgContainers::IpgModal(_) => {},
        IpgContainers::IpgMouseArea(m_area) => {
            mousearea_item_update(m_area, item, value);
        },
        IpgContainers::IpgTable(table) => {
            table_item_update(table, item, value);
        },
        IpgContainers::IpgRow(_) => {},
        IpgContainers::IpgScrollable(scroll) => {
            scrollable_item_update(scroll, item, value);
        },
        IpgContainers::IpgToolTip(_) => {},
        IpgContainers::IpgWindow(wnd) => {
            window_item_update(wnd, item, value);
        },
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
    m.add_class::<IpgHorizontalAlignment>()?;
    m.add_class::<IpgVerticalAlignment>()?;
    m.add_class::<IpgButtonArrow>()?;
    m.add_class::<IpgButtonParam>()?;
    m.add_class::<IpgCardStyle>()?;
    m.add_class::<IpgCardParam>()?;
    m.add_class::<IpgColor>()?;
    m.add_class::<IpgCheckboxParam>()?;
    m.add_class::<IpgDatePickerParam>()?;
    m.add_class::<IpgImageContentFit>()?;
    m.add_class::<IpgImageFilterMethod>()?;
    m.add_class::<IpgImageParam>()?;
    m.add_class::<IpgImageRotation>()?;
    m.add_class::<IpgMenuParam>()?;
    m.add_class::<IpgMenuSeparatorType>()?;
    m.add_class::<IpgMenuType>()?;
    m.add_class::<IpgMouseAreaParam>()?;
    m.add_class::<IpgPickListParam>()?;
    m.add_class::<IpgPickListHandle>()?;
    m.add_class::<IpgProgressBarParam>()?;
    m.add_class::<IpgRadioDirection>()?;
    m.add_class::<IpgRadioParam>()?;
    m.add_class::<IpgScrollableDirection>()?;
    m.add_class::<IpgScrollableParam>()?;
    m.add_class::<IpgSelectableTextParam>()?;
    m.add_class::<IpgSliderParam>()?;
    m.add_class::<IpgStyleParam>()?;
    m.add_class::<IpgStyleStandard>()?;
    m.add_class::<IpgSvgParam>()?;
    m.add_class::<IpgTableRowHighLight>()?;
    m.add_class::<IpgTableParam>()?;
    m.add_class::<IpgTableWidget>()?;
    m.add_class::<IpgTextInputParam>()?;
    m.add_class::<IpgTextParam>()?;
    m.add_class::<IpgTimerParams>()?;
    m.add_class::<IpgTogglerParam>()?;
    m.add_class::<IpgWindowParam>()?;
    m.add_class::<IpgWindowLevel>()?;
    m.add_class::<IpgWindowTheme>()?;
    m.add_class::<IpgWindowMode>()?;
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

fn add_callback_to_mutex(id: usize, event_name: String, py_obj: Option<PyObject>) {
    let mut app_cbs = access_callbacks();

        app_cbs.callbacks.insert((id, event_name), py_obj);

        drop(app_cbs);
}

pub fn find_parent_uid(ipg_ids: &Vec<IpgIds>, parent_id: String) -> usize {

    for id in ipg_ids.iter() {
        if id.container_id == Some(parent_id.clone()) {
            return id.id
        }
    }
    panic!("Parent id {:?} not found in function find_parent_uid()", parent_id)
}


