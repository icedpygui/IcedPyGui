//!lib for all of the python callable functions using pyo3
#![allow(clippy::too_many_arguments, clippy::redundant_closure)]
#![allow(clippy::type_complexity)]
use canvas::canvas_helpers::{build_polygon, get_mid_point, to_radians};
use canvas::draw_canvas::{IpgCanvasState, IpgDrawMode, IpgDrawStatus, IpgWidget};
use canvas::geometries::{IpgArc, IpgBezier, IpgCanvasImage, 
    IpgCanvasWidget, IpgCircle, IpgEllipse, IpgLine, IpgPolyLine, IpgPolygon, IpgRectangle};
use iced::widget::image;
use iced_aw::iced_fonts;
use ipg_widgets::ipg_color_picker::{color_picker_style_update_item, color_picker_update, 
    IpgColorPicker, IpgColorPickerParam, IpgColorPickerStyle, IpgColorPickerStyleParam};
use ipg_widgets::ipg_separator::{separator_item_update, separator_style_update_item, IpgSeparator, IpgSeparatorParam, IpgSeparatorStyle, IpgSeparatorStyleParam, IpgSeparatorType};
use ipg_widgets::ipg_timer_canvas::{canvas_timer_item_update, canvas_timer_style_update_item, 
    IpgCanvasTimer, IpgCanvasTimerParam, IpgCanvasTimerStyle, IpgCanvasTimerStyleParam};
use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::PyObject;

use iced::window::{self, Position};
use iced::{Color, Font, Length, Point, Radians, Rectangle, Size, Theme, Vector};
use iced::widget::text::{self, LineHeight};

use core::panic;
use std::iter::Iterator;
use std::collections::HashMap;

mod app;
use app::App;

mod ipg_widgets;
mod iced_widgets;
mod iced_aw_widgets;
mod graphics;
mod style;
mod canvas;

use ipg_widgets::ipg_button::{button_item_update, button_style_update_item, 
    IpgButton, IpgButtonArrow, IpgButtonParam, IpgButtonStyle, IpgButtonStyleParam};
use ipg_widgets::ipg_canvas::{canvas_item_update, IpgCanvas, 
    IpgCanvasGeometryParam, IpgCanvasParam};
use ipg_widgets::ipg_card::{card_item_update, IpgCard, 
    IpgCardStyle, IpgCardParam};
use ipg_widgets::ipg_checkbox::{checkbox_item_update, checkbox_style_update_item, 
    IpgCheckBox, IpgCheckboxParam, IpgCheckboxStyle, IpgCheckboxStyleParam};
use ipg_widgets::ipg_column::IpgColumn;
use ipg_widgets::ipg_container::{IpgContainer, IpgContainerStyle};
use ipg_widgets::ipg_date_picker::{date_picker_item_update, 
        IpgDatePicker, IpgDatePickerParam};
use ipg_widgets::ipg_events::IpgEvents;
use ipg_widgets::ipg_image::{image_item_update, IpgImage, 
        IpgImageContentFit, IpgImageFilterMethod, 
        IpgImageParam, IpgImageRotation};
// use ipg_widgets::ipg_menu::{menu_bar_style_update_item, menu_item_update, 
//     menu_separator_style_update_item, menu_style_update_item, IpgMenu, IpgMenuBarStyle, 
//     IpgMenuBarStyleParam, IpgMenuParam, IpgMenuSeparatorStyle, IpgMenuSeparatorStyleParam, 
//     IpgMenuSeparatorType, IpgMenuStyle, IpgMenuStyleParam, IpgMenuType, ItemStyles, MenuStyleAll};
use ipg_widgets::ipg_mousearea::{mousearea_item_update, IpgMouseArea, 
        IpgMouseAreaParam, IpgMousePointer};
use ipg_widgets::ipg_opaque::{opaque_item_update, IpgOpaque, 
    IpgOpaqueParam, IpgOpaqueStyle};
use ipg_widgets::ipg_pick_list::{pick_list_item_update, pick_list_style_update_item, 
    IpgPickList, IpgPickListHandle, IpgPickListParam, IpgPickListStyle, IpgPickListStyleParam};
use ipg_widgets::ipg_progress_bar::{progress_bar_item_update, progress_bar_style_update_item, 
    IpgProgressBar, IpgProgressBarParam, IpgProgressBarStyle, IpgProgressBarStyleParam};
use ipg_widgets::ipg_radio::{radio_item_update, radio_style_update_item, IpgRadio, 
    IpgRadioDirection, IpgRadioParam, IpgRadioStyle, IpgRadioStyleParam};
use ipg_widgets::ipg_row::IpgRow;
use ipg_widgets::ipg_rule::{rule_style_update_item, IpgRule, IpgRuleStyle, IpgRuleStyleParam};
use ipg_widgets::ipg_scrollable::{scroll_style_update_item, scrollable_item_update, 
    IpgScrollable, IpgScrollableAlignment, IpgScrollableDirection, IpgScrollableParam, 
    IpgScrollableStyle, IpgScrollableStyleParam};
use ipg_widgets::ipg_selectable_text::{selectable_text_item_update, 
        IpgSelectableText, IpgSelectableTextParam};
use ipg_widgets::ipg_slider::{slider_item_update, slider_style_update_item, IpgSlider, 
    IpgSliderParam, IpgSliderStyle, IpgSliderStyleParam};
use ipg_widgets::ipg_space::IpgSpace;
use ipg_widgets::ipg_stack::{stack_item_update, IpgStack, IpgStackParam};
use ipg_widgets::ipg_svg::{svg_item_update, IpgSvg, IpgSvgContentFit, 
        IpgSvgParam, IpgSvgRotation};
use ipg_widgets::ipg_table::{table_item_update, IpgTable, IpgTableParam, 
        IpgTableRowHighLight, IpgTableWidget,};
use ipg_widgets::ipg_text::{text_item_update, IpgText, IpgTextParam};
use ipg_widgets::ipg_text_input::{text_input_item_update, text_input_style_update_item, 
    IpgTextInput, IpgTextInputParam, IpgTextInputStyle, IpgTextInputStyleParam};
use ipg_widgets::ipg_timer::{timer_item_update, timer_style_update_item, IpgTimer, 
    IpgTimerParam, IpgTimerStyle, IpgTimerStyleParam};
use ipg_widgets::ipg_toggle::{toggler_item_update, toggler_style_update_item, 
    IpgToggler, IpgTogglerParam, IpgTogglerStyle, IpgTogglerStyleParam};
use ipg_widgets::ipg_tool_tip::IpgToolTip;
use ipg_widgets::ipg_window::{get_iced_window_theme, 
        window_item_update, IpgWindow, IpgWindowLevel, IpgWindowMode, 
        IpgWindowParam, IpgWindowTheme};
use ipg_widgets::ipg_enums::{IpgAlignment, IpgContainers, IpgHorizontalAlignment, 
    IpgVerticalAlignment, IpgWidgets};

use ipg_widgets::helpers::{check_for_dup_container_ids, get_height, 
    get_line_height, get_padding_f64, get_shaping, get_width};

use graphics::colors::{get_color, IpgColor};
use style::styling::{readable, IpgStyleStandard};

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

#[derive(Debug)]
pub struct UpdateItems {
    // wid, (item, value)
    pub updates: Vec<(usize, PyObject, PyObject)>, 
    // window_id_widget_id, (window_id, wid, target_container_str_id, move_after(wid), move_before(wid))
    pub moves: Vec<(String, usize, String, Option<usize>, Option<usize>)>,
    // window_id, wid
    pub deletes: Vec<(String, usize)>,
}

pub static UPDATE_ITEMS: Mutex<UpdateItems> = Mutex::new(UpdateItems {
    updates: vec![],
    moves: vec![],
    deletes: vec![],
});

pub fn access_update_items() -> MutexGuard<'static, UpdateItems> {
    UPDATE_ITEMS.lock().unwrap()
}

#[derive(Debug)]
pub struct UpdateCanvasItems {
    // wid, (item, value)
    pub updates: Vec<(usize, PyObject, PyObject)>, 
    // window_id_widget_id, (window_id, wid, target_container_str_id, move_after(wid), move_before(wid))
    pub moves: Vec<(String, usize, String, Option<usize>, Option<usize>)>,
    // window_id, wid
    pub deletes: Vec<(String, usize)>,
}

pub static UPDATE_CANVAS_ITEMS: Mutex<UpdateCanvasItems> = Mutex::new(UpdateCanvasItems {
    updates: vec![],
    moves: vec![],
    deletes: vec![],
});

pub fn access_canvas_update_items() -> MutexGuard<'static, UpdateCanvasItems> {
    UPDATE_CANVAS_ITEMS.lock().unwrap()
}

#[derive(Debug)]
pub struct WindowActions {
    pub mode: Vec<(usize, window::Mode)>,
    pub decorations: Vec<usize>,
    pub resize: Vec<(usize, f32, f32)>,
    pub position: Vec<(usize, f32, f32)>,
    pub level: Vec<(usize, window::Level)>,
}

pub static WINDOW_ACTIONS: Mutex<WindowActions> = Mutex::new(WindowActions {
    mode: vec![],
    decorations: vec![],
    resize: vec![],
    position: vec![],
    level: vec![],
});

pub fn access_window_actions() -> MutexGuard<'static, WindowActions> {
    WINDOW_ACTIONS.lock().unwrap()
}

#[derive(Debug)]
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
    pub window_mode: Lazy<HashMap<window::Id, (usize, window::Mode)>>,

    pub events: Vec<IpgEvents>,
    
    pub container_style: Lazy<HashMap<String, IpgContainerStyle>>,
    pub button_style: Lazy<HashMap<String, IpgButtonStyle>>,
    pub checkbox_style: Lazy<HashMap<String, IpgCheckboxStyle>>,
    pub color_picker_style: Lazy<HashMap<String, IpgColorPickerStyle>>,
    // pub menu_bar_style: Lazy<HashMap<String, IpgMenuBarStyle>>,
    // pub menu_style: Lazy<HashMap<String, IpgMenuStyle>>,
    // pub menu_separator_style: Lazy<HashMap<String, IpgMenuSeparatorStyle>>,
    pub opaque_style: Lazy<HashMap<String, IpgOpaqueStyle>>,
    pub pick_list_style: Lazy<HashMap<String, IpgPickListStyle>>,
    pub progress_bar_style: Lazy<HashMap<String, IpgProgressBarStyle>>,
    pub radio_style:  Lazy<HashMap<String, IpgRadioStyle>>,
    pub rule_style:  Lazy<HashMap<String, IpgRuleStyle>>,
    pub slider_style:  Lazy<HashMap<String, IpgSliderStyle>>,
    pub text_input_style: Lazy<HashMap<String, IpgTextInputStyle>>,
    pub toggler_style: Lazy<HashMap<String, IpgTogglerStyle>>,
    pub scrollable_style: Lazy<HashMap<String, IpgScrollableStyle>>,

    pub keyboard_event_id_enabled: (usize, bool),
    pub mouse_event_id_enabled: (usize, bool),
    pub timer_event_id_enabled: (usize, bool),
    pub canvas_timer_event_id_enabled: (usize, bool),
    pub window_event_id_enabled: (usize, bool),
    pub touch_event_id_enabled: (usize, bool),
    pub timer_duration: u64,
    pub canvas_timer_duration: u64,

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
        window_mode: Lazy::new(||HashMap::new()),
        
        events: vec![],

        container_style: Lazy::new(||HashMap::new()),
        button_style: Lazy::new(||HashMap::new()),
        checkbox_style: Lazy::new(||HashMap::new()),
        color_picker_style: Lazy::new(||HashMap::new()),
        // menu_bar_style: Lazy::new(||HashMap::new()),
        // menu_style: Lazy::new(||HashMap::new()),
        // menu_separator_style: Lazy::new(||HashMap::new()),
        opaque_style: Lazy::new(||HashMap::new()),
        pick_list_style: Lazy::new(||HashMap::new()),
        progress_bar_style: Lazy::new(||HashMap::new()),
        radio_style: Lazy::new(||HashMap::new()),
        rule_style: Lazy::new(||HashMap::new()),
        slider_style: Lazy::new(||HashMap::new()),
        text_input_style: Lazy::new(||HashMap::new()),
        toggler_style: Lazy::new(||HashMap::new()),
        scrollable_style: Lazy::new(||HashMap::new()),
        
        keyboard_event_id_enabled: (0, false),
        mouse_event_id_enabled: (0, false), 
        timer_event_id_enabled: (0, false),
        canvas_timer_event_id_enabled: (0, false),
        window_event_id_enabled: (0, false),
        touch_event_id_enabled: (0, false),
        timer_duration: 0,
        canvas_timer_duration: 0,

    }
);

pub fn access_state() -> MutexGuard<'static, State> {
    STATE.lock().unwrap()
}

#[derive(Debug)]
pub struct CanvasState {
    pub canvas_ids_str: Lazy<HashMap<String, usize>>,
    pub curves: Lazy<HashMap<usize, IpgWidget>>,
    pub text_curves: Lazy<HashMap<usize, IpgWidget>>,
    pub image_curves: Lazy<HashMap<usize, IpgWidget>>,
    pub width: Length,
    pub height: Length,
    pub background: Option<Color>,
    pub border_color: Option<Color>,
    pub border_width: Option<f32>,
}

pub static CANVAS_STATE: Mutex<CanvasState> = Mutex::new(
    CanvasState {
        canvas_ids_str: Lazy::new(||HashMap::new()),
        curves: Lazy::new(||HashMap::new()),
        text_curves: Lazy::new(||HashMap::new()),
        image_curves: Lazy::new(||HashMap::new()),
        width: Length::Fill,
        height: Length::Fill,
        background: None,
        border_color: None,
        border_width: None,
        },
);

pub fn access_canvas_state() -> MutexGuard<'static, CanvasState> {
    CANVAS_STATE.lock().unwrap()
}

#[derive(Default, Debug, Clone)]
pub struct IpgState {
    pub ids: HashMap<usize, Vec<IpgIds>>,  // <window_id=usize, Vec<IpgIds=structure>>
    pub last_id: usize,

    pub containers: HashMap<usize, IpgContainers>,
    pub container_ids: HashMap<usize, Vec<usize>>,  // <window_id=usize, vec<container_id=usize>>
    pub container_wnd_str_ids: HashMap<String, String>, // get window string id based on container string id
    pub container_str_ids: HashMap<String, usize>, // get container usize id based on container string
    pub container_window_usize_ids: HashMap<usize, usize>, //get window usize id based on container usize id
    
    pub widgets: HashMap<usize, IpgWidgets>,
    pub widget_container_ids: HashMap<usize, String>, //widget_id=usize, container_id=String
    
    pub windows_iced_ipg_ids: HashMap<window::Id, usize>, // <iced id, ipg id>
    pub windows_str_ids: HashMap<String, usize>,  // <ipg_id=str, ipg id>
    pub windows: Vec<IpgWindow>,
    pub window_debug: HashMap<window::Id, (usize, bool)>, // (wid, debug)
    pub window_theme: HashMap<window::Id, (usize, Theme)>, // (wid, window Theme)
    pub window_mode: HashMap<window::Id, (usize, window::Mode)>,
    pub windows_opened: u16,

    pub container_style: HashMap<String, IpgContainerStyle>,
    pub button_style: HashMap<String, IpgButtonStyle>,
    pub checkbox_style: HashMap<String, IpgCheckboxStyle>,
    pub color_picker_style: HashMap<String, IpgColorPickerStyle>,
    // pub menu_bar_style: HashMap<String, IpgMenuBarStyle>,
    // pub menu_style: HashMap<String, IpgMenuStyle>,
    // pub menu_separator_style: HashMap<String, IpgMenuSeparatorStyle>,
    pub opaque_style: HashMap<String, IpgOpaqueStyle>,
    pub pick_list_style: HashMap<String, IpgPickListStyle>,
    pub progress_bar_style: HashMap<String, IpgProgressBarStyle>,
    pub radio_style:  HashMap<String, IpgRadioStyle>,
    pub rule_style:  HashMap<String, IpgRuleStyle>,
    pub slider_style:  HashMap<String, IpgSliderStyle>,
    pub text_input_style: HashMap<String, IpgTextInputStyle>,
    pub toggler_style: HashMap<String, IpgTogglerStyle>,
    pub scrollable_style: HashMap<String, IpgScrollableStyle>,

    pub keyboard_event_id_enabled: (usize, bool),
    pub mouse_event_id_enabled: (usize, bool),
    pub timer_event_id_enabled: (usize, bool),
    pub canvas_timer_event_id_enabled: (usize, bool),
    pub window_event_id_enabled: (usize, bool),
    pub touch_event_id_enabled: (usize, bool),
    pub timer_duration: u64,
    pub canvas_timer_duration: u64,

    pub mode: Vec<(usize, window::Mode)>,
    pub decorations: Vec<usize>,
    pub resize: Vec<(usize, f32, f32)>,
    pub position: Vec<(usize, f32, f32)>,
    pub level: Vec<(usize, window::Level)>,
}

impl IpgState {
    pub fn new() -> Self {
        IpgState {
            ids: HashMap::new(),
            last_id: 0,

            containers: HashMap::new(),
            container_ids: HashMap::new(),
            container_wnd_str_ids: HashMap::new(),
            container_str_ids: HashMap::new(),
            container_window_usize_ids: HashMap::new(),

            widgets: HashMap::new(),
            widget_container_ids: HashMap::new(),

            windows_iced_ipg_ids: HashMap::new(),
            windows_str_ids: HashMap::new(),
            windows: vec![],
            window_debug: HashMap::new(),
            window_theme: HashMap::new(),
            window_mode: HashMap::new(),
            windows_opened: 0,

            container_style: HashMap::new(),
            button_style: HashMap::new(),
            checkbox_style: HashMap::new(),
            color_picker_style: HashMap::new(),
            // menu_bar_style: HashMap::new(),
            // menu_style: HashMap::new(),
            // menu_separator_style: HashMap::new(),
            opaque_style: HashMap::new(),
            pick_list_style: HashMap::new(),
            progress_bar_style: HashMap::new(),
            radio_style: HashMap::new(),
            rule_style: HashMap::new(),
            slider_style: HashMap::new(),
            text_input_style: HashMap::new(),
            toggler_style: HashMap::new(),
            scrollable_style: HashMap::new(),

            keyboard_event_id_enabled: (0, false),
            mouse_event_id_enabled: (0, false), 
            timer_event_id_enabled: (0, false),
            canvas_timer_event_id_enabled: (0, false),
            window_event_id_enabled: (0, false),
            touch_event_id_enabled: (0, false),
            timer_duration: 0,
            canvas_timer_duration: 0,

            mode: vec![],
            decorations: vec![],
            resize: vec![],
            position: vec![],
            level: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub enum IpgUpdateType {
    Add,
    Delete,
    Move,
    Update,
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

        let _ = iced::daemon(App::title, App::update, App::view)
                    .subscription(App::subscription)
                    .theme(App::theme)
                    .font(iced_fonts::REQUIRED_FONT_BYTES)
                    .scale_factor(App::scale_factor)
                    .antialiasing(true)
                    .run_with(||App::new());
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
                        exit_on_close=false, on_resize=None, 
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
            let pos_x = pos_x.unwrap_or(0.0);
            let pos_y = pos_y.unwrap_or(0.0);
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

        state.ids.insert(id, vec![IpgIds{id, parent_uid: 0, container_id: Some(window_id.clone()),
                                                parent_id: "".to_string(), is_container: true}]);

        state.container_ids.insert(id, vec![id]);
        // TODO: Only one of these below are needed but some subtle issues arise when not used together.
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

    #[pyo3(signature = (window_id,
                        canvas_id,
                        width=None,
                        width_fill=false,
                        height=None,
                        height_fill=false,
                        border_width=2.0,
                        border_ipg_color=IpgColor::WHITE,
                        border_rgba_color=None,
                        background_ipg_color=None,
                        background_rgba_color=None,
                        parent_id=None,
                        gen_id=None,
                        ))]
    fn add_canvas(&mut self,
                    window_id: String,
                    canvas_id: String,
                    width: Option<f32>,
                    width_fill: bool,
                    height: Option<f32>,
                    height_fill: bool,
                    border_width: Option<f32>,
                    border_ipg_color: Option<IpgColor>,
                    border_rgba_color: Option<[f32; 4]>,
                    background_ipg_color: Option<IpgColor>,
                    background_rgba_color: Option<[f32; 4]>,
                    parent_id: Option<String>,
                    gen_id: Option<usize>,
                    )  -> PyResult<usize> 
    {
        let id = self.get_id(gen_id);

        let width = get_width(width, width_fill);
        let height = get_height(height, height_fill);
        let background: Option<Color> = get_color(background_rgba_color, background_ipg_color, 1.0, false);

        let border_color = get_color(border_rgba_color, border_ipg_color, 1.0, false);

        let prt_id = match parent_id {
            Some(id) => id,
            None => window_id.clone(),
        };

        set_state_of_container(self.id, window_id.clone(), Some(canvas_id.clone()), prt_id);

        let mut state = access_state();

        set_state_cont_wnd_ids(&mut state, &window_id, canvas_id.clone(), self.id, "add_canvas".to_string());

        state.containers.insert(self.id, IpgContainers::IpgCanvas(IpgCanvas::new(
                                                id,
                                            )));
        state.last_id = id;
        drop(state);

        // set up the CanvasState
        let mut canvas_state = access_canvas_state();
        canvas_state.canvas_ids_str.insert(canvas_id, id);
        canvas_state.width = width;
        canvas_state.height = height;
        canvas_state.background = background;
        canvas_state.border_width = border_width;
        canvas_state.border_color = border_color;
        drop(canvas_state);

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
                        // **above required
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

    // #[pyo3(signature = (window_id, container_id, label,
    //                     parent_id=None, on_open=None,
    //                     align_items=IpgAlignment::Start, 
    //                     width=None, height=None,
    //                     width_fill=false, height_fill=false,
    //                     max_width=f32::INFINITY, padding=vec![0.0], 
    //                     spacing=10.0, clip=false, show=false,
    //                     user_data=None,
    //                     ))]
    // fn add_modal(&mut self,
    //                     window_id: String,
    //                     container_id: String,
    //                     label: String,
    //                     // **above required
    //                     parent_id: Option<String>,
    //                     on_open: Option<PyObject>,
    //                     align_items: IpgAlignment,
    //                     width: Option<f32>,
    //                     height: Option<f32>,
    //                     width_fill: bool,
    //                     height_fill: bool,
    //                     max_width: f32,
    //                     padding: Vec<f64>,
    //                     spacing: f32,
    //                     clip: bool,
    //                     show: bool,
    //                     user_data: Option<PyObject>,
    //                     ) -> PyResult<usize> 
    // {

    //     self.id += 1;

    //     if on_open.is_some() {
    //         add_callback_to_mutex(self.id, "on_open".to_string(), on_open);
    //     }
        
    //     let width = get_width(width, width_fill);
    //     let height = get_height(height, height_fill);

    //     let padding = get_padding_f64(padding);

    //     let prt_id = match parent_id {
    //         Some(id) => id,
    //         None => window_id.clone(),
    //     };

    //     set_state_of_container(self.id, window_id.clone(), Some(container_id.clone()), prt_id);

    //     let mut state = access_state();

    //     set_state_cont_wnd_ids(&mut state, &window_id, container_id, self.id, "add_column".to_string());

    //     state.containers.insert(self.id, IpgContainers::IpgModal(IpgModal::new(
    //                                                             self.id,
    //                                                             label,  
    //                                                             show, 
    //                                                             spacing, 
    //                                                             padding, 
    //                                                             width, 
    //                                                             height, 
    //                                                             max_width, 
    //                                                             align_items,
    //                                                             clip,
    //                                                             user_data,
    //                                                         )));
    //     state.last_id = self.id;
    //     drop(state);
    //     Ok(self.id)

    // }

    #[pyo3(signature = (window_id, container_id, parent_id=None, 
                        gen_id=None, mouse_pointer=None,
                        on_press=None, on_release=None,
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
                        mouse_pointer: Option<IpgMousePointer>,
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
                                    mouse_pointer,  
                                    show, 
                                    user_data
                                )));
        state.last_id = id;
        drop(state);
        Ok(id)

    }

    #[pyo3(signature = (window_id, container_id, parent_id=None,
                        width=None, height=None, 
                        width_fill=false, height_fill=false,
                        horizontal_alignment=None, vertical_alignment=None,
                        mouse_on_press=None,
                        show=true, style_id=None,
                        gen_id=None,
                        ))]
    fn add_opaque_container(&mut self,
                            window_id: String,
                            container_id: String,
                            // required above
                            parent_id: Option<String>,
                            width: Option<f32>,
                            height: Option<f32>,
                            width_fill: bool,
                            height_fill: bool,
                            horizontal_alignment: Option<IpgHorizontalAlignment>,
                            vertical_alignment: Option<IpgVerticalAlignment>,
                            mouse_on_press: Option<PyObject>,
                            show: bool,
                            style_id: Option<String>,
                            gen_id: Option<usize>,
                            ) -> PyResult<usize> 
    {
        let id = self.get_id(gen_id);

        let width = get_width(width, width_fill);
        let height = get_height(height, height_fill);

        let include_mouse_area = if mouse_on_press.is_some() {
            add_callback_to_mutex(id, "on_press".to_string(), mouse_on_press);
            true
        } else {
            false
        };

        let prt_id = match parent_id {
            Some(id) => id,
            None => window_id.clone(),
        };

        set_state_of_container(self.id, window_id.clone(), Some(container_id.clone()), prt_id);

        let mut state = access_state();

        set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_opaque".to_string());

        state.containers.insert(id, IpgContainers::IpgOpaque(IpgOpaque::new(
                                    id,  
                                    width, 
                                    height,
                                    horizontal_alignment,
                                    vertical_alignment,
                                    include_mouse_area,
                                    show,
                                    style_id
                                    )));
        state.last_id = id;
        drop(state);         
        Ok(id)
    }

    #[pyo3(signature = (style_id, 
                        background_color=None, 
                        background_rgba=None,
                        gen_id=None))]
    fn add_opaque_style(&mut self,
                            style_id: String,
                            background_color: Option<IpgColor>,
                            background_rgba: Option<[f32; 4]>,
                            gen_id: Option<usize>,
                            ) -> PyResult<usize>
    {
        let id = self.get_id(gen_id);

        let mut state = access_state();

        let background_color: Option<Color> = get_color(background_rgba, background_color, 1.0, false);

        state.opaque_style.insert(style_id, IpgOpaqueStyle::new( 
                                                    id,
                                                    background_color,
                                                    ));
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
                        hide_index=None, show=true,
                        ))]
    fn add_stack(&mut self,
                    window_id: String,
                    container_id: String,
                    // required above
                    parent_id: Option<String>,
                    width: Option<f32>,
                    height: Option<f32>,
                    width_fill: bool,
                    height_fill: bool,
                    hide_index: Option<usize>,
                    show: bool,
                    ) -> PyResult<usize> 
    {
        self.id += 1;

        let width = get_width(width, width_fill);
        let height = get_height(height, height_fill);

        let prt_id = match parent_id {
            Some(id) => id,
            None => window_id.clone(),
        };

        set_state_of_container(self.id, window_id.clone(), Some(container_id.clone()), prt_id);

        let mut state = access_state();

        set_state_cont_wnd_ids(&mut state, &window_id, container_id, self.id, "add_stack".to_string());

        state.containers.insert(self.id, IpgContainers::IpgStack(IpgStack::new(
                                    self.id,  
                                    width, 
                                    height,
                                    hide_index,
                                    show,
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
                            style_id: Option<usize>,
                            ) -> PyResult<usize>
    {
        self.id += 1;

        if on_scroll.is_some() {
            add_callback_to_mutex(self.id, "on_scroll".to_string(), on_scroll);
        }
        // For scrollable the fill doesn't work well so as long as the fixed is
        // larger than the window, it will fill whatever space is left.
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
      
        state.containers.insert(self.id, IpgContainers::IpgScrollable(
            IpgScrollable::new( 
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

    #[pyo3(signature = ( 
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

        let background_color: Option<Color> = get_color(background_rgba, background_color, 1.0, false);
        let border_color: Option<Color> = get_color(border_rgba, border_color, 1.0, false);
        let shadow_color: Option<Color> = get_color(shadow_rgba, shadow_color, 1.0, false);
        let text_color: Option<Color> = get_color(text_rgba, text_color, 1.0, false);

        let scrollbar_color: Option<Color> = get_color(scrollbar_rgba, scrollbar_color, 1.0, false);
        let scrollbar_border_color: Option<Color> = get_color(scrollbar_border_rgba, scrollbar_border_color, 1.0, false);
        
        let scroller_color: Option<Color> = get_color(scroller_rgba, scroller_color, 1.0, false);
        let scroller_color_hovered: Option<Color> = get_color(scroller_rgba_hovered, scroller_color_hovered, 1.0, false);
        let scroller_color_dragged: Option<Color> = get_color(scroller_rgba_dragged, scroller_color_dragged, 1.0, false);

        let mut state = access_state();

        state.widgets.insert(self.id, IpgWidgets::IpgScrollableStyle(
            IpgScrollableStyle::new(  
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
                )));

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
                        style_id: Option<usize>,
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

        state.widgets.insert(id, IpgWidgets::IpgButton(
            IpgButton::new(
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

    #[pyo3(signature = ( 
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

        let background_color: Option<Color> = get_color(background_rgba, background_color, 1.0, false);
        let background_color_hovered: Option<Color> = get_color(background_rgba_hovered, background_color_hovered, 1.0, false);
        let border_color: Option<Color> = get_color(border_rgba, border_color, 1.0, false);
        let shadow_color: Option<Color> = get_color(shadow_rgba, shadow_color, 1.0, false);
        let text_color: Option<Color> = get_color(text_rgba, text_color, 1.0, false);

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgButtonStyle(
            IpgButtonStyle::new( 
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
                )));

        state.last_id = id;
        drop(state);
        Ok(id)
    }

    #[pyo3(signature = (parent_id, head, body,      
                        is_open=true, min_max_id=None, foot=None, 
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
                min_max_id: Option<usize>,
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

        state.widgets.insert(id, IpgWidgets::IpgCard(
            IpgCard::new(
                id,
                is_open,
                user_data,
                min_max_id,
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
                        style_id: Option<usize>,
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

        state.widgets.insert(id, IpgWidgets::IpgCheckBox(
            IpgCheckBox::new(
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

    #[pyo3(signature = ( 
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

        let background_color: Option<Color> = get_color(background_rgba, background_color, 1.0, false);
        let background_color_hovered: Option<Color> = get_color(background_rgba_hovered, background_color_hovered, 1.0, false);
        let accent_color: Option<Color> = get_color(accent_rgba, accent_color, 1.0, false);
        let accent_color_hovered: Option<Color> = get_color(accent_rgba_hovered, accent_color_hovered, 1.0, false);
        let border_color: Option<Color> = get_color(border_rgba, border_color, 1.0, false);
        let icon_color: Option<Color> = get_color(icon_rgba, icon_color, 1.0, false);
        let text_color: Option<Color> = get_color(text_rgba, text_color, 1.0, false);

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgCheckboxStyle(
            IpgCheckboxStyle::new( 
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
                )));

        state.last_id = id;
        drop(state);
        Ok(id)
    }

    #[pyo3(signature = (parent_id, label="Set Color".to_string(), 
                        gen_id=None, on_press=None, on_submit=None, 
                        on_cancel=None, color_rgba=[0.5, 0.2, 0.7, 1.0], 
                        width=None, height=None, width_fill=false, 
                        height_fill=false, padding=vec![5.0], clip=false, 
                        style_id=None, style_standard=None, 
                        style_arrow=None, user_data=None, show=false, 
                        ))]
    fn add_color_picker(
                        &mut self,
                        parent_id: String,
                        // ** above required
                        label: String,
                        gen_id: Option<usize>,
                        on_press: Option<PyObject>,
                        on_submit: Option<PyObject>,
                        on_cancel: Option<PyObject>,
                        color_rgba: [f32; 4],
                        width: Option<f32>,
                        height: Option<f32>,
                        width_fill: bool,
                        height_fill: bool,
                        padding: Vec<f64>,
                        clip: bool,
                        style_id: Option<usize>,
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

        if on_submit.is_some() {
            add_callback_to_mutex(id, "on_submit".to_string(), on_submit);
        }

        if on_cancel.is_some() {
            add_callback_to_mutex(id, "on_cancel".to_string(), on_cancel);
        }

        let color = Color::from(color_rgba);

        let width = get_width(width, width_fill);
        let height = get_height(height, height_fill);

        let padding = get_padding_f64(padding);

        set_state_of_widget(id, parent_id);

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgColorPicker(
            IpgColorPicker::new(
                id,
                show,
                color,
                user_data,
                // button related
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

    #[pyo3(signature = ( 
                        background_color=None, background_rgba=None,
                        background_color_hovered=None, background_rgba_hovered=None,
                        border_color=None, border_rgba=None,
                        border_radius = vec![0.0], border_width=1.0,
                        shadow_color=None, shadow_rgba=None,
                        shadow_offset_x=0.0, shadow_offset_y=0.0,
                        shadow_blur_radius=1.0,
                        text_color=None, text_rgba=None,
                        gen_id=None))]
    fn add_color_picker_style(&mut self,
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

        let background_color: Option<Color> = get_color(background_rgba, background_color, 1.0, false);
        let background_color_hovered: Option<Color> = get_color(background_rgba_hovered, background_color_hovered, 1.0, false);
        let border_color: Option<Color> = get_color(border_rgba, border_color, 1.0, false);
        let shadow_color: Option<Color> = get_color(shadow_rgba, shadow_color, 1.0, false);
        let text_color: Option<Color> = get_color(text_rgba, text_color, 1.0, false);

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgColorPickerStyle(
            IpgColorPickerStyle::new(
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
                )));

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

        state.widgets.insert(id, IpgWidgets::IpgDatePicker(
            IpgDatePicker::new(
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
                        mouse_pointer=None,
                        on_press=None, on_release=None,
                        on_right_press=None, on_right_release=None,
                        on_middle_press=None, on_middle_release=None,
                        on_enter=None, on_move=None, on_exit=None,
                        user_data=None, show=true,
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
                    mouse_pointer: Option<IpgMousePointer>,
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

    state.widgets.insert(id, IpgWidgets::IpgImage(
        IpgImage::new(
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
            mouse_pointer,
            show,
            user_data,
        )));
        state.last_id = id;
        drop(state);
        Ok(id)
    }

    // #[pyo3(signature = (parent_id, 
    //                     items,
    //                     bar_widths,
    //                     item_widths, 
    //                     bar_spacings=None,
    //                     bar_paddings=None,
    //                     bar_height=None,
    //                     bar_check_bounds_width=None,
    //                     item_spacings=None,
    //                     item_offsets=None, 
    //                     on_select=None,
    //                     menu_bar_style=None,
    //                     menu_style=None,
    //                     show=true, 
    //                     user_data=None, gen_id=None))]
    // fn add_menu(&mut self,
    //                 parent_id: String, 
    //                 items: Vec<Vec<(Option<String>, IpgMenuType, Option<usize>)>>,
    //                 bar_widths: Vec<f32>,
    //                 item_widths: Vec<f32>,
    //                 bar_spacings: Option<f32>,
    //                 bar_paddings: Option<Vec<f32>>,
    //                 bar_height: Option<f32>,
    //                 bar_check_bounds_width: Option<f32>,
    //                 item_spacings: Option<Vec<f32>>,
    //                 item_offsets: Option<Vec<f32>>,
    //                 on_select: Option<PyObject>,
    //                 menu_bar_style: Option<usize>,
    //                 menu_style: Option<usize>,
    //                 show: bool,
    //                 user_data: Option<PyObject>,
    //                 gen_id: Option<usize>,
    //             ) -> PyResult<usize> 
    // {
    //     let id = self.get_id(gen_id);

    //     if on_select.is_some() {
    //         add_callback_to_mutex(id, "on_select".to_string(), on_select);
    //     }

    //     let spacing = bar_spacings.unwrap_or(0.0);

    //     let padding = get_padding_f32(bar_paddings);
        
    //     let height = get_height(bar_height, false);
 
    //     let check_bounds_width = bar_check_bounds_width.unwrap_or(50.0) ;
        
    //     set_state_of_widget(id, parent_id);

    //     let mut state = access_state();

    //     state.widgets.insert(id, IpgWidgets::IpgMenu(
    //         IpgMenu::new(
    //             id,
    //             items,
    //             bar_widths,
    //             item_widths,
    //             spacing,
    //             padding,
    //             height,
    //             check_bounds_width,
    //             item_spacings,
    //             item_offsets,
    //             menu_bar_style,
    //             menu_style,
    //             self.theme.clone(),
    //             show,
    //             user_data,
    //             )));
    //     state.last_id = id;
    //     drop(state);
    //     Ok(id)
    // }

    // #[pyo3(signature = (
    //                     base_color=None,
    //                     base_rgba=None,
    //                     border_color=None,
    //                     border_rgba=None,
    //                     border_radius=None,
    //                     border_width=None,
    //                     shadow_color=None,
    //                     shadow_rgba=None,
    //                     shadow_offset_x=None,
    //                     shadow_offset_y=None,
    //                     shadow_blur_radius=None,
    //                     gen_id=None))]
    // fn add_menu_bar_style(&mut self,
    //                         base_color: Option<IpgColor>,
    //                         base_rgba: Option<[f32; 4]>,
    //                         border_color: Option<IpgColor>,
    //                         border_rgba: Option<[f32; 4]>,
    //                         border_radius: Option<Vec<f32>>,
    //                         border_width: Option<f32>,
    //                         shadow_color: Option<IpgColor>,
    //                         shadow_rgba: Option<[f32; 4]>,
    //                         shadow_offset_x: Option<f32>,
    //                         shadow_offset_y: Option<f32>,
    //                         shadow_blur_radius: Option<f32>,
    //                         gen_id: Option<usize>,
    //                         ) -> PyResult<usize>
    // {
    //     let id = self.get_id(gen_id);

    //     let base: Option<Color> = get_color(base_rgba, base_color, 1.0, false);
    //     let border_color: Option<Color> = get_color(border_rgba, border_color, 1.0, false);
    //     let shadow_color: Option<Color> = get_color(shadow_rgba, shadow_color, 1.0, false);

    //     let mut state = access_state();

    //     state.widgets.insert(id, IpgWidgets::IpgMenuBarStyle(
    //         IpgMenuBarStyle::new( 
    //             id,
    //             base,
    //             border_color,
    //             border_radius,
    //             border_width,
    //             shadow_color,
    //             shadow_offset_x,
    //             shadow_offset_y,
    //             shadow_blur_radius,
    //             )));

    //     state.last_id = id;
    //     drop(state);
    //     Ok(id)
    // }

    // #[pyo3(signature = (
    //                     base_color=None,
    //                     base_rgba=None,
    //                     border_color=None,
    //                     border_rgba=None,
    //                     border_radius=None,
    //                     border_width=None,
    //                     shadow_color=None,
    //                     shadow_rgba=None,
    //                     shadow_offset_x=None,
    //                     shadow_offset_y=None,
    //                     shadow_blur_radius=None,
    //                     path_base_color=None,
    //                     path_base_rgba=None,
    //                     path_border_color=None,
    //                     path_border_rgba=None,
    //                     path_border_radius=None,
    //                     path_border_width=None,
    //                     gen_id=None))]
    // fn add_menu_style(&mut self,
    //                         base_color: Option<IpgColor>,
    //                         base_rgba: Option<[f32; 4]>,
    //                         border_color: Option<IpgColor>,
    //                         border_rgba: Option<[f32; 4]>,
    //                         border_radius: Option<Vec<f32>>,
    //                         border_width: Option<f32>,
    //                         shadow_color: Option<IpgColor>,
    //                         shadow_rgba: Option<[f32; 4]>,
    //                         shadow_offset_x: Option<f32>,
    //                         shadow_offset_y: Option<f32>,
    //                         shadow_blur_radius: Option<f32>,
    //                         path_base_color: Option<IpgColor>,
    //                         path_base_rgba: Option<[f32; 4]>,
    //                         path_border_color: Option<IpgColor>,
    //                         path_border_rgba: Option<[f32; 4]>,
    //                         path_border_radius: Option<Vec<f32>>,
    //                         path_border_width: Option<f32>,
    //                         gen_id: Option<usize>,
    //                         ) -> PyResult<usize>
    // {
    //     let id = self.get_id(gen_id);
        
    //     let base_color: Option<Color> = get_color(base_rgba, base_color, 1.0, false);
    //     let border_color: Option<Color> = get_color(border_rgba, border_color, 1.0, false);
    //     let shadow_color: Option<Color> = get_color(shadow_rgba, shadow_color, 1.0, false);
    //     let path_base: Option<Color> = get_color(path_base_rgba, path_base_color, 1.0, false);
    //     let path_border_color: Option<Color> = get_color(path_border_rgba, path_border_color, 1.0, false);

    //     let mut state = access_state();

    //     state.widgets.insert(id, IpgWidgets::IpgMenuStyle(
    //         IpgMenuStyle::new(  
    //             id,
    //             base_color,
    //             border_color,
    //             border_radius,
    //             border_width,
    //             shadow_color,
    //             shadow_offset_x,
    //             shadow_offset_y,
    //             shadow_blur_radius,
    //             path_base,
    //             path_border_color,
    //             path_border_radius,
    //             path_border_width,
    //             )));
    //     state.last_id = id;
    //     drop(state);
    //     Ok(id)
    // }

    #[pyo3(signature = (parent_id, options, gen_id=None, on_select=None, 
                        width=None, width_fill=false, padding=vec![5.0],  
                        placeholder=None, selected=None, text_size=None, 
                        text_line_height=1.2, text_shaping="basic".to_string(), 
                        handle=IpgPickListHandle::Default, arrow_size=None, 
                        dynamic_closed=None, dynamic_opened=None, custom_static=None,
                        style_id=None, user_data=None, show=true,
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
                        style_id: Option<usize>,
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

        state.widgets.insert(id, IpgWidgets::IpgPickList(
            IpgPickList::new(  
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
                style_id,
            )));
        state.last_id = id;
        drop(state);
        Ok(id)
    }

#[pyo3(signature = (
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

        state.widgets.insert(id, IpgWidgets::IpgPickListStyle(
            IpgPickListStyle::new( 
                id,
                background_color,
                text_color,
                handle_color,
                placeholder_color,
                border_color,
                border_color_hovered,
                border_radius,
                border_width,
                )));
        state.last_id = id;
        drop(state);
        Ok(id)
    }

    #[pyo3(signature = (parent_id, min, max, value,
                        gen_id=None, width=None, height=Some(16.0), 
                        width_fill=true, height_fill=false,
                        style_standard=None, style_id=None, 
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
                            style_id: Option<usize>,
                            show: bool,
                            ) -> PyResult<usize> 
    {

        let id = self.get_id(gen_id);

        let width = get_width(width, width_fill);
        let height: Length = get_height(height, height_fill);

        set_state_of_widget(id, parent_id);

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgProgressBar(
            IpgProgressBar::new(   
                id,
                show,
                min,
                max,
                value,
                width,
                height,
                style_standard,
                style_id,
                )));
        state.last_id = id;
        drop(state);
        Ok(id)

    }

    #[pyo3(signature = ( 
                        background_color=None, background_rgba=None,
                        bar_color=None, bar_rgba=None,
                        border_color=None, border_rgba=None,
                        border_radius=None, border_width=None,
                        gen_id=None))]
    fn add_progress_bar_style(&mut self,
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

        let background: Option<Color> = get_color(background_rgba, background_color, 1.0, false);
        let bar_color: Option<Color> = get_color(bar_rgba, bar_color, 1.0, false);
        let border_color: Option<Color> = get_color(border_rgba, border_color, 1.0, false);

        let mut state = access_state();

         state.widgets.insert(id, IpgWidgets::IpgProgressBarStyle(
            IpgProgressBarStyle::new(  
                id,
                background,
                bar_color,
                border_color,
                border_radius,
                border_width,
                )));
        state.last_id = id;
        drop(state);
        Ok(id)
    }

    #[pyo3(signature = (parent_id, labels, gen_id=None,
                        direction=IpgRadioDirection::Vertical, 
                        spacing= 10.0, padding=vec![10.0], 
                        width=None, width_fill=false, 
                        height=None, height_fill=false,
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
                    style_id: Option<usize>,
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

        state.widgets.insert(id, IpgWidgets::IpgRadio(
            IpgRadio::new( 
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

    #[pyo3(signature = (
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

        let background_color = get_color(background_rgba, background_color, 1.0, false);
        let background_color_hovered = get_color(background_rgba_hovered, background_color_hovered, 1.0, false);
        let dot_color: Option<Color> = get_color(dot_rgba, dot_color, 1.0, false);
        let dot_color_hovered: Option<Color> = get_color(dot_rgba_hovered, dot_color_hovered, 1.0, false);
        let border_color: Option<Color> = get_color(border_rgba, border_color, 1.0, false);
        let text_color: Option<Color> = get_color(text_rgba, text_color, 1.0, false);

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgRadioStyle(
            IpgRadioStyle::new( 
                id,
                background_color,
                background_color_hovered,
                dot_color,
                dot_color_hovered,
                border_color,
                border_width,
                text_color,
                )));

        state.last_id = id;
        drop(state);
        Ok(id)
    }

    #[pyo3(signature = (parent_id, width, 
                        width_fill=true, 
                        thickness=1,
                        style_id=None,
                        gen_id=None,
                        show=true,
                        ))]
    fn add_rule_horizontal(&mut self, 
                            parent_id: String,
                            width: Option<f32>,
                            width_fill: bool,
                            thickness: u16,
                            style_id: Option<usize>,
                            gen_id: Option<usize>,
                            show: bool
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
                                                        style_id,
                                                        show,
                                                        )));
        state.last_id = id;
        drop(state);
        Ok(id)
    }

    #[pyo3(signature = (parent_id, height=None, 
                        height_fill=true, thickness=1,
                        style_id=None, gen_id=None,
                        show=true
                        ))]
    fn add_rule_vertical(&mut self, 
                            parent_id: String,
                            height: Option<f32>,
                            height_fill: bool,
                            thickness: u16,
                            style_id: Option<usize>,
                            gen_id: Option<usize>,
                            show: bool, 
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
                                                        style_id,
                                                        show, 
                                                        )));
        state.last_id = id;
        drop(state);
        Ok(id)
    }

    #[pyo3(signature = (
                        color=None, 
                        color_rgba=None,
                        border_radius=None,
                        fillmode_percent=None,
                        fillmode_padded=None,
                        fillmode_asymmetric_padding=None,
                        gen_id=None))]
    fn add_rule_style(&mut self,
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

        let color = get_color(color_rgba, color, 1.0, false);
        
       let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgRuleStyle(
            IpgRuleStyle::new(
                id,
                color,
                border_radius,
                fillmode_percent,
                fillmode_padded,
                fillmode_asymmetric_padding,
                )));

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

    #[pyo3(signature = (parent_id, 
                        separator_type=IpgSeparatorType::Line,
                        label=None, label_left_width=20.0,
                        label_right_width=20.0,
                        dot_radius=4.0, dot_count=1,
                        dot_fill=true, dot_border_width=0.0,
                        width=None, width_fill=false, 
                        height=None, height_fill=false,
                        spacing=0.0, style_id=None,
                        gen_id=None, show=true))]
    fn add_separator(&mut self,
                        parent_id: String,
                        separator_type: IpgSeparatorType,
                        label: Option<String>,
                        label_left_width: f32,
                        label_right_width: f32,
                        dot_radius: f32,
                        dot_count: usize,
                        dot_fill: bool,
                        dot_border_width: f32,
                        width: Option<f32>, 
                        width_fill: bool,
                        height: Option<f32>,
                        height_fill: bool,
                        spacing: f32,
                        style_id: Option<usize>,
                        gen_id: Option<usize>,
                        show: bool,
                    ) -> PyResult<usize>
    {

        let id = self.get_id(gen_id);

        let width = get_width(width, width_fill);
        let height = get_height(height, height_fill);

        set_state_of_widget(id, parent_id);

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgSeparator(
            IpgSeparator::new( 
                id,
                separator_type,
                label,
                label_left_width,
                label_right_width,
                dot_radius,
                dot_count,
                dot_fill,
                dot_border_width,
                width,
                height,
                spacing,
                style_id,
                show,
                )));

        state.last_id = id;
        drop(state);
        Ok(id)
    }

    #[pyo3(signature = (
                        ipg_color=None,
                        rgba_color=None,
                        border_ipg_color=None,
                        border_rgba_color=None,
                        gen_id=None,
                        ))]
    fn add_separator_style(&mut self,
                                ipg_color: Option<IpgColor>,
                                rgba_color: Option<[f32; 4]>,
                                border_ipg_color: Option<IpgColor>,
                                border_rgba_color: Option<[f32; 4]>,
                                gen_id: Option<usize>,
                                ) -> PyResult<usize> 
    {
        let id = self.get_id(gen_id);

        let color: Option<Color> = 
            get_color(rgba_color, ipg_color, 1.0, false);
        let border_color = 
            get_color(border_rgba_color, border_ipg_color, 1.0, false);

        let mut state = access_state();
        
        state.widgets.insert(id, IpgWidgets::IpgSeparatorStyle(
            IpgSeparatorStyle::new(
                id,
                color,
                border_color,
                )));

        state.last_id = id;
        drop(state);
        Ok(id)
    }

    #[pyo3(signature = (parent_id, min, max, step, value, 
                        gen_id=None, width=None, height=None, 
                        width_fill=false, on_change=None, 
                        on_release=None, style_id=None,
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
                        style_id: Option<usize>,
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
        let height = height.unwrap_or(16.0);

        set_state_of_widget(id, parent_id);

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgSlider(
            IpgSlider::new( 
                id,
                show,
                user_data,
                min,
                max,
                step,
                value,
                width,
                height,
                style_id,
                )));

        state.last_id = id;
        drop(state);
        Ok(id)
    }

    #[pyo3(signature = (
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

        let rail_color = get_color(rail_rgba, rail_color, 1.0, false);
        let rail_color_hovered = get_color(rail_rgba_hovered, rail_color_hovered, 1.0, false);
        let handle_color = get_color(handle_rgba, handle_color, 1.0, false);
        let handle_border_color = get_color(handle_border_rgba,handle_border_color,1.0, false);

        let mut state = access_state();
        
        state.widgets.insert(id, IpgWidgets::IpgSliderStyle(
            IpgSliderStyle::new(
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
                )));

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
                        mouse_pointer=None,
                        on_press=None, on_release=None,
                        on_right_press=None, on_right_release=None,
                        on_middle_press=None, on_middle_release=None,
                        on_enter=None, on_move=None, on_exit=None, 
                        user_data=None, show=true,
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
                    mouse_pointer: Option<IpgMousePointer>,
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
                                                mouse_pointer,
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
                        button_fill_style_standard=None,
                        checkbox_fill_style_id=None,
                        checkbox_fill_style_standard=None,
                        toggler_fill_style_id=None,
                        mixed_widgets_column_style_ids=None,
                        gen_id=None, 
                        on_button=None, 
                        on_checkbox=None,
                        on_toggler=None,
                        on_scroll=None, 
                        show=true,
                        modal_show=false,
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
                    button_fill_style_standard: Option<IpgStyleStandard>,
                    checkbox_fill_style_id: Option<String>,
                    checkbox_fill_style_standard: Option<IpgStyleStandard>,
                    toggler_fill_style_id: Option<String>,
                    mixed_widgets_column_style_ids: Option<HashMap<usize, Vec<String>>>,
                    gen_id: Option<usize>,
                    on_button: Option<PyObject>,
                    on_checkbox: Option<PyObject>,
                    on_toggler: Option<PyObject>,
                    on_scroll: Option<PyObject>,
                    show: bool,
                    modal_show: bool,
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
                            button_ids.push((self.get_id(None), row, *col, false));
                        },
                        IpgTableWidget::Checkbox => {
                            checkbox_ids.push((self.get_id(None), row, *col, false));
                        },
                        IpgTableWidget::Toggler => {
                            toggler_ids.push((self.get_id(None), row, *col, false));
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
                                                    button_fill_style_standard,
                                                    checkbox_fill_style_id,
                                                    checkbox_fill_style_standard,
                                                    toggler_fill_style_id,
                                                    mixed_widgets_column_style_ids,
                                                    show,
                                                    modal_show,
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

        let line_height = LineHeight::Relative(line_height);

        let shaping = get_shaping(shaping);

        let style = get_color(text_rgba, text_color, 1.0, false);

        set_state_of_widget(id, parent_id);

        let mut state = access_state();
        
        state.widgets.insert(id, IpgWidgets::IpgText(
            IpgText::new(
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
                            style_id: Option<usize>,
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
        
        state.widgets.insert(id, IpgWidgets::IpgTextInput(
            IpgTextInput::new( 
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

    #[pyo3(signature = ( 
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
       
        state.widgets.insert(id, IpgWidgets::IpgTextInputStyle(
            IpgTextInputStyle::new( 
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
                )));

        state.last_id = id;                                        
        drop(state);
        Ok(id)
    }

    #[pyo3(signature = (parent_id, duration_ms, 
                        on_start=None, on_tick=None, on_stop=None, 
                        label="Start Timer".to_string(), 
                        width=None, height=None, 
                        width_fill=false, height_fill=false,
                        padding=vec![10.0], clip=false, 
                        style_id=None, style_standard=None, style_arrow=None, 
                        user_data=None, gen_id=None, show=true
                        ))]
    fn add_timer(&mut self,
                        parent_id: String,
                        duration_ms: u64,
                        on_start: Option<PyObject>,
                        on_tick: Option<PyObject>,
                        on_stop: Option<PyObject>,
                        label: String,
                        width: Option<f32>,
                        height: Option<f32>,
                        width_fill: bool,
                        height_fill: bool,
                        padding: Vec<f64>,
                        clip: bool,
                        style_id: Option<usize>,
                        style_standard: Option<IpgStyleStandard>,
                        style_arrow: Option<IpgButtonArrow>,
                        user_data: Option<PyObject>,
                        gen_id: Option<usize>,
                        show: bool,
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
                                                            label,
                                                            width,
                                                            height,
                                                            padding,
                                                            clip,
                                                            style_id,
                                                            style_standard,
                                                            style_arrow,
                                                            user_data,
                                                            show,
                                                            )));
        state.last_id = id;
        drop(state);
        Ok(id)
    }

    #[pyo3(signature = ( 
                        background_color=None, background_rgba=None,
                        background_color_hovered=None, background_rgba_hovered=None,
                        border_color=None, border_rgba=None,
                        border_radius = vec![0.0], border_width=1.0,
                        shadow_color=None, shadow_rgba=None,
                        shadow_offset_x=0.0, shadow_offset_y=0.0,
                        shadow_blur_radius=1.0,
                        text_color=None, text_rgba=None,
                        gen_id=None))]
    fn add_timer_style(&mut self,
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

        let background_color: Option<Color> = get_color(background_rgba, background_color, 1.0, false);
        let background_color_hovered: Option<Color> = get_color(background_rgba_hovered, background_color_hovered, 1.0, false);
        let border_color: Option<Color> = get_color(border_rgba, border_color, 1.0, false);
        let shadow_color: Option<Color> = get_color(shadow_rgba, shadow_color, 1.0, false);
        let text_color: Option<Color> = get_color(text_rgba, text_color, 1.0, false);

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgTimerStyle(
            IpgTimerStyle::new( 
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
                )));

        state.last_id = id;
        drop(state);
        Ok(id)
    }

    #[pyo3(signature = (parent_id, duration_ms, 
                        on_start=None, on_tick=None, on_stop=None, 
                        label="Start Timer".to_string(), 
                        width=None, height=None, 
                        width_fill=false, height_fill=false,
                        padding=vec![10.0], clip=false, 
                        style_id=None, style_standard=None, style_arrow=None, 
                        user_data=None, gen_id=None, show=true
                        ))]
    fn add_canvas_timer(&mut self,
                        parent_id: String,
                        duration_ms: u64,
                        on_start: Option<PyObject>,
                        on_tick: Option<PyObject>,
                        on_stop: Option<PyObject>,
                        label: String,
                        width: Option<f32>,
                        height: Option<f32>,
                        width_fill: bool,
                        height_fill: bool,
                        padding: Vec<f64>,
                        clip: bool,
                        style_id: Option<usize>,
                        style_standard: Option<IpgStyleStandard>,
                        style_arrow: Option<IpgButtonArrow>,
                        user_data: Option<PyObject>,
                        gen_id: Option<usize>,
                        show: bool,
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

        state.widgets.insert(self.id, IpgWidgets::IpgCanvasTimer(IpgCanvasTimer::new(
                                                            id,
                                                            duration_ms,
                                                            label,
                                                            width,
                                                            height,
                                                            padding,
                                                            clip,
                                                            style_id,
                                                            style_standard,
                                                            style_arrow,
                                                            user_data,
                                                            show,
                                                            )));
        state.last_id = id;
        drop(state);
        Ok(id)
    }

    #[pyo3(signature = ( 
                        background_color=None, background_rgba=None,
                        background_color_hovered=None, background_rgba_hovered=None,
                        border_color=None, border_rgba=None,
                        border_radius = vec![0.0], border_width=1.0,
                        shadow_color=None, shadow_rgba=None,
                        shadow_offset_x=0.0, shadow_offset_y=0.0,
                        shadow_blur_radius=1.0,
                        text_color=None, text_rgba=None,
                        gen_id=None))]
    fn add_canvas_timer_style(&mut self,
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

        let background_color: Option<Color> = get_color(background_rgba, background_color, 1.0, false);
        let background_color_hovered: Option<Color> = get_color(background_rgba_hovered, background_color_hovered, 1.0, false);
        let border_color: Option<Color> = get_color(border_rgba, border_color, 1.0, false);
        let shadow_color: Option<Color> = get_color(shadow_rgba, shadow_color, 1.0, false);
        let text_color: Option<Color> = get_color(text_rgba, text_color, 1.0, false);

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgCanvasTimerStyle(
            IpgCanvasTimerStyle::new( 
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
                        style_id: Option<usize>,
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

        state.widgets.insert(id, IpgWidgets::IpgToggler(
            IpgToggler::new(
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

    #[pyo3(signature = (
                        background_color=None,
                        background_rgba=None,
                        background_color_toggled=None,
                        background_rgba_toggled=None,
                        background_color_disabled=None,
                        background_rgba_disabled=None,
                        background_border_color=None,
                        background_border_rgba=None,
                        background_border_width=None,
                        foreground_color=None,
                        foreground_rgba=None,
                        foreground_color_toggled=None,
                        foreground_rgba_toggled=None,
                        foreground_color_disabled=None,
                        foreground_rgba_disabled=None,
                        foreground_border_color=None,
                        foreground_border_rgba=None,
                        foreground_border_width=None,
                        gen_id=None,
                        ))]
    fn add_toggler_style(&mut self,
                        background_color: Option<IpgColor>,
                        background_rgba: Option<[f32; 4]>,
                        background_color_toggled: Option<IpgColor>,
                        background_rgba_toggled: Option<[f32; 4]>,
                        background_color_disabled: Option<IpgColor>,
                        background_rgba_disabled: Option<[f32; 4]>,
                        background_border_color: Option<IpgColor>,
                        background_border_rgba: Option<[f32; 4]>,
                        background_border_width: Option<f32>,
                        foreground_color: Option<IpgColor>,
                        foreground_rgba: Option<[f32; 4]>,
                        foreground_color_toggled: Option<IpgColor>,
                        foreground_rgba_toggled: Option<[f32; 4]>,
                        foreground_color_disabled: Option<IpgColor>,
                        foreground_rgba_disabled: Option<[f32; 4]>,
                        foreground_border_color: Option<IpgColor>,
                        foreground_border_rgba: Option<[f32; 4]>,
                        foreground_border_width: Option<f32>,
                        gen_id: Option<usize>,
                        ) -> PyResult<usize>
    {
        let id = self.get_id(gen_id);

        let background_color = get_color(background_rgba, background_color, 1.0, false);
        let background_color_toggled = get_color(background_rgba_toggled, background_color_toggled, 1.0, false);
        let background_color_disabled = get_color(background_rgba_disabled, background_color_disabled, 1.0, false);
        let background_border_color = get_color(background_border_rgba, background_border_color, 1.0, false);
        let foreground_color = get_color(foreground_rgba, foreground_color, 1.0, false);
        let foreground_color_toggled = get_color(foreground_rgba_toggled, foreground_color_toggled, 1.0, false);
        let foreground_color_disabled = get_color(foreground_rgba_disabled, foreground_color_disabled, 1.0, false);
        let foreground_border_color = get_color(foreground_border_rgba, foreground_border_color, 1.0, false);
        

        let mut state = access_state();
       
        state.widgets.insert(id, IpgWidgets::IpgTogglerStyle(
            IpgTogglerStyle::new(
                id,
                background_color,
                background_color_toggled,
                background_color_disabled,
                background_border_color,
                background_border_width,
                foreground_color,
                foreground_color_toggled,
                foreground_color_disabled,
                foreground_border_color,
                foreground_border_width,
                )));

        state.last_id = id;
        drop(state);
        Ok(id)
    }

        #[pyo3(signature = (canvas_id,
                            center_xy,
                            radius,
                            start_angle,
                            end_angle,
                            stroke_width=2.0,
                            stroke_dash_offset=None,
                            stroke_dash_segments=None,
                            stroke_ipg_color=Some(IpgColor::WHITE),
                            stroke_rgba_color=None,
                            fill_ipg_color=None,
                            fill_rgba_color=None,
                            gen_id=None,
                            ))]
    fn add_arc(&mut self,
                    canvas_id: String,
                    center_xy: (f32, f32),
                    radius: f32,
                    start_angle: f32,
                    end_angle: f32,
                    stroke_width: f32,
                    stroke_dash_offset: Option<usize>,
                    stroke_dash_segments: Option<Vec<f32>>,
                    stroke_ipg_color: Option<IpgColor>,
                    stroke_rgba_color: Option<[f32; 4]>,
                    fill_ipg_color: Option<IpgColor>,
                    fill_rgba_color: Option<[f32; 4]>,
                    gen_id: Option<usize>,
                    )  -> PyResult<usize> 
    {   
        let mid_point = Point::new(center_xy.0, center_xy.1);
        let color = if stroke_rgba_color.is_some() {
            get_color(stroke_rgba_color, None, 1.0, false).unwrap()
        } else {
            get_color(None, stroke_ipg_color, 1.0, false).unwrap()
        };
         
        let fill_color =  get_color(fill_rgba_color, fill_ipg_color, 1.0, false);

        let mut canvas_state = access_canvas_state();
        let canvas_id_opt = canvas_state.canvas_ids_str.get(&canvas_id);
        
        if canvas_id_opt.is_none() {
            panic!("Arc: You need to define a canvas before adding geometries or your canvas_id is incorrect.")
        }
        
        let id = self.get_id(gen_id);

        let arc = IpgArc { 
                            id, 
                            points: vec![], 
                            mid_point, 
                            radius, 
                            color, 
                            fill_color, 
                            width: stroke_width,
                            stroke_dash_offset,
                            stroke_dash_segments,
                            start_angle: Radians(to_radians(&start_angle)), 
                            end_angle: Radians(to_radians(&end_angle)),
                            draw_mode: IpgDrawMode::Display, 
                            status: IpgDrawStatus::Completed,
                            };
                                    

        canvas_state.curves.insert(id, IpgWidget::Arc(arc));
        drop(canvas_state);
        let mut state = access_state();
        state.last_id = id;
        drop(state);

        Ok(id)
    }

    #[pyo3(signature = (canvas_id,
                        points,
                        stroke_width=2.0,
                        stroke_dash_offset=None,
                        stroke_dash_segments=None,
                        stroke_ipg_color=IpgColor::WHITE,
                        stroke_rgba_color=None,
                        fill_ipg_color=None,
                        fill_rgba_color=None,
                        degrees=0.0,
                        gen_id=None,
                        ))]
    fn add_bezier(&mut self,
                    canvas_id: String,
                    points: [(f32, f32); 3],
                    stroke_width: f32,
                    stroke_dash_offset: Option<usize>,
                    stroke_dash_segments: Option<Vec<f32>>,
                    stroke_ipg_color: Option<IpgColor>,
                    stroke_rgba_color: Option<[f32; 4]>,
                    fill_ipg_color: Option<IpgColor>,
                    fill_rgba_color: Option<[f32; 4]>,
                    degrees: f32,
                    gen_id: Option<usize>,
                    )  -> PyResult<usize> 
    {
        let mid_point = get_mid_point(Point::new(points[0].0, points[0].1), Point::new(points[1].0, points[1].1));

        let points = vec![Point::new(points[0].0, points[0].1), 
                                        Point::new(points[1].0, points[1].1), 
                                        Point::new(points[2].0, points[2].1)];

        let color = if stroke_rgba_color.is_some() {
            get_color(stroke_rgba_color, None, 1.0, false).unwrap()
        } else {
            get_color(None, stroke_ipg_color, 1.0, false).unwrap()
        };
         
        let fill_color =  get_color(fill_rgba_color, fill_ipg_color, 1.0, false);

        let mut canvas_state = access_canvas_state();
        let canvas_id_opt = canvas_state.canvas_ids_str.get(&canvas_id);
        
        if canvas_id_opt.is_none() {
            panic!("Bezier: You need to define a canvas before adding geometries or your canvas_id is incorrect.")
        }
        
        let id = self.get_id(gen_id);

        let bezier = IpgBezier{ 
                                    id, 
                                    points, 
                                    mid_point, 
                                    color, 
                                    fill_color, 
                                    width: stroke_width,
                                    stroke_dash_offset,
                                    stroke_dash_segments, 
                                    rotation: degrees, 
                                    draw_mode: IpgDrawMode::Display, 
                                    status: IpgDrawStatus::Completed,
                                    };

        canvas_state.curves.insert(id, IpgWidget::Bezier(bezier));
        drop(canvas_state);

        let mut state = access_state();
        state.last_id = id;
        drop(state);

        Ok(id)
    }

    #[pyo3(signature = (canvas_id,
                        position_xy,
                        radius,
                        stroke_width=2.0,
                        stroke_dash_offset=0,
                        stroke_dash_segments=None,
                        stroke_ipg_color=IpgColor::WHITE,
                        stroke_rgba_color=None,
                        stroke_color_alpha=1.0,
                        fill_ipg_color=None,
                        fill_rgba_color=None,
                        fill_color_alpha=1.0,
                        gen_id=None,
                        ))]
    fn add_circle(&mut self,
                    canvas_id: String,
                    position_xy: (f32, f32),
                    radius: f32,
                    stroke_width: f32,
                    stroke_dash_offset: usize,
                    stroke_dash_segments: Option<Vec<f32>>,
                    stroke_ipg_color: Option<IpgColor>,
                    stroke_rgba_color: Option<[f32; 4]>,
                    stroke_color_alpha: f32,
                    fill_ipg_color: Option<IpgColor>,
                    fill_rgba_color: Option<[f32; 4]>,
                    fill_color_alpha: f32,
                    gen_id: Option<usize>,
                    ) -> PyResult<usize> 
    {
        let center = Point::new(position_xy.0, position_xy.1);
        let circle_point = Point::new(center.x, center.y + radius);

        let color = if stroke_rgba_color.is_some() {
            get_color(stroke_rgba_color, None, stroke_color_alpha, false).unwrap()
        } else {
            get_color(None, stroke_ipg_color, stroke_color_alpha, false).unwrap()
        };
       
        let fill_color =  get_color(fill_rgba_color, fill_ipg_color, fill_color_alpha, false);

        let mut canvas_state = access_canvas_state();
        let canvas_id_opt = canvas_state.canvas_ids_str.get(&canvas_id);
        
        if canvas_id_opt.is_none() {
            panic!("Circle: You need to define a canvas before adding geometries or your canvas_id is incorrect.")
        }
        
        let id = self.get_id(gen_id);
        
        let circle = IpgCircle{ 
                                    id, 
                                    center, 
                                    circle_point, 
                                    radius, 
                                    color, 
                                    fill_color, 
                                    width: stroke_width,
                                    stroke_dash_offset,
                                    stroke_dash_segments, 
                                    draw_mode: IpgDrawMode::Display, 
                                    status: IpgDrawStatus::Completed,
                                    };

        canvas_state.curves.insert(id, IpgWidget::Circle(circle));
        drop(canvas_state);

        let mut state = access_state();
        state.last_id = id;
        drop(state);

        Ok(id)
    }

    #[pyo3(signature = (canvas_id,
                        position_xy,
                        radius_x,
                        radius_y,
                        degrees=0.0,
                        stroke_width=2.0,
                        stroke_dash_offset=None,
                        stroke_dash_segments=None,
                        stroke_ipg_color=IpgColor::WHITE,
                        stroke_rgba_color=None,
                        fill_ipg_color=None,
                        fill_rgba_color=None,
                        gen_id=None,
                        ))]
    fn add_ellipse(&mut self,
                    canvas_id: String,
                    position_xy: (f32, f32),
                    radius_x: f32,
                    radius_y: f32,
                    degrees: f32,
                    stroke_width: f32,
                    stroke_dash_offset: Option<usize>,
                    stroke_dash_segments: Option<Vec<f32>>,
                    stroke_ipg_color: Option<IpgColor>,
                    stroke_rgba_color: Option<[f32; 4]>,
                    fill_ipg_color: Option<IpgColor>,
                    fill_rgba_color: Option<[f32; 4]>,
                    gen_id: Option<usize>,
                    )  -> PyResult<usize> 
    {
        let center = Point::new(position_xy.0, position_xy.1);

        let points = vec![Point::new(center.x+radius_x, center.y), 
                                        Point::new(center.x, center.y+radius_y)];

        let color = if stroke_rgba_color.is_some() {
            get_color(stroke_rgba_color, None, 1.0, false).unwrap()
        } else {
            get_color(None, stroke_ipg_color, 1.0, false).unwrap()
        };
         
        let fill_color =  get_color(fill_rgba_color, fill_ipg_color, 1.0, false);

        let mut canvas_state = access_canvas_state();
        let canvas_id_opt = canvas_state.canvas_ids_str.get(&canvas_id);
        
        if canvas_id_opt.is_none() {
            panic!("Ellipse: You need to define a canvas before adding geometries or your canvas_id is incorrect.")
        }
        
        let id = self.get_id(gen_id);

        let ellipse = IpgEllipse{ 
                                    id, 
                                    points, 
                                    center, 
                                    radii: Vector{x: 0.0, y: 0.0}, 
                                    rotation: Radians(to_radians(&degrees)), 
                                    color, 
                                    fill_color, 
                                    width: stroke_width,
                                    stroke_dash_offset,
                                    stroke_dash_segments, 
                                    draw_mode: IpgDrawMode::Display, 
                                    status: IpgDrawStatus::Completed,
                                    };

        canvas_state.curves.insert(id, IpgWidget::Ellipse(ellipse));
        drop(canvas_state);

        let mut state = access_state();
        state.last_id = id;
        drop(state);

        Ok(id)
    }

    #[pyo3(signature = (canvas_id,
                        start,
                        end,
                        degrees=0.0,
                        stroke_width=2.0,
                        stroke_dash_offset=None,
                        stroke_dash_segments=None,
                        stroke_ipg_color=IpgColor::WHITE,
                        stroke_rgba_color=None,
                        gen_id=None,
                        ))]
    fn add_line(&mut self,
                    canvas_id: String,
                    start: (f32, f32),
                    end: (f32, f32),
                    degrees: f32,
                    stroke_width: f32,
                    stroke_dash_offset: Option<usize>,
                    stroke_dash_segments: Option<Vec<f32>>,
                    stroke_ipg_color: Option<IpgColor>,
                    stroke_rgba_color: Option<[f32; 4]>,
                    gen_id: Option<usize>,
                    )  -> PyResult<usize> 
    {
        let points = vec![Point::new(start.0, start.1), Point::new(end.0, end.1)];

        let mid_point = get_mid_point(points[0], points[1]);

        let color = if stroke_rgba_color.is_some() {
            get_color(stroke_rgba_color, None, 1.0, false).unwrap()
        } else {
            get_color(None, stroke_ipg_color, 1.0, false).unwrap()
        };

        let mut canvas_state = access_canvas_state();
        let canvas_id_opt = canvas_state.canvas_ids_str.get(&canvas_id);
        
        if canvas_id_opt.is_none() {
            panic!("Line: You need to define a canvas before adding geometries or your canvas_id is incorrect.")
        }
        
        let id = self.get_id(gen_id);
        
        let line = IpgLine{ 
                                id, 
                                points, 
                                mid_point, 
                                color, 
                                width: stroke_width,
                                stroke_dash_offset,
                                stroke_dash_segments, 
                                rotation: degrees, 
                                draw_mode: IpgDrawMode::Display, 
                                status: IpgDrawStatus::Completed,
                                };

        canvas_state.curves.insert(id, IpgWidget::Line(line));
        drop(canvas_state);

        let mut state = access_state();
        state.last_id = id;
        drop(state);

        Ok(id)
    }

    #[pyo3(signature = (canvas_id,
                        position_xy,
                        radius,
                        sides,
                        degrees=0.0,
                        stroke_width=2.0,
                        stroke_dash_offset=None,
                        stroke_dash_segments=None,
                        stroke_ipg_color=IpgColor::WHITE,
                        stroke_rgba_color=None,
                        fill_ipg_color=None,
                        fill_rgba_color=None,
                        gen_id=None,
                        ))]
    fn add_polygon(&mut self,
                    canvas_id: String,
                    position_xy: (f32, f32),
                    radius: f32,
                    sides: usize,
                    degrees: f32,
                    stroke_width: f32,
                    stroke_dash_offset: Option<usize>,
                    stroke_dash_segments: Option<Vec<f32>>,
                    stroke_ipg_color: Option<IpgColor>,
                    stroke_rgba_color: Option<[f32; 4]>,
                    fill_ipg_color: Option<IpgColor>,
                    fill_rgba_color: Option<[f32; 4]>,
                    gen_id: Option<usize>,
                    ) -> PyResult<usize> 
    {
        let center = Point::new(position_xy.0, position_xy.1);

        let pg_point = Point::new(center.x+radius, center.y);

        let color = if stroke_rgba_color.is_some() {
            get_color(stroke_rgba_color, None, 1.0, false).unwrap()
        } else {
            get_color(None, stroke_ipg_color, 1.0, false).unwrap()
        };
         
        let fill_color =  get_color(fill_rgba_color, fill_ipg_color, 1.0, false);

        let mut canvas_state = access_canvas_state();
        let canvas_id_opt = canvas_state.canvas_ids_str.get(&canvas_id);
        
        if canvas_id_opt.is_none() {
            panic!("Polygon: You need to define a canvas before adding geometries or your canvas_id is incorrect.")
        }
        
        let id = self.get_id(gen_id);
        
        let pg = IpgPolygon{ 
                            id,
                            points: build_polygon(center, pg_point, sides, degrees),
                            poly_points: sides-1,
                            mid_point: center,
                            pg_point,
                            color,
                            fill_color,
                            width: stroke_width,
                            stroke_dash_offset,
                            stroke_dash_segments,
                            rotation: degrees,
                            draw_mode: IpgDrawMode::Display, 
                            status: IpgDrawStatus::Completed,
                            };

        canvas_state.curves.insert(id, IpgWidget::Polygon(pg));
        drop(canvas_state);

        let mut state = access_state();
        state.last_id = id;
        drop(state);

        Ok(id)
    }

    #[pyo3(signature = (canvas_id,
                        points,
                        stroke_width,
                        stroke_dash_offset=None,
                        stroke_dash_segments=None,
                        stroke_ipg_color=IpgColor::WHITE,
                        stroke_rgba_color=None,
                        gen_id=None,
                        ))]
    fn add_poly_line(&mut self,
                    canvas_id: String,
                    points: Vec<(f32, f32)>,
                    stroke_width: f32,
                    stroke_dash_offset: Option<usize>,
                    stroke_dash_segments: Option<Vec<f32>>,
                    stroke_ipg_color: Option<IpgColor>,
                    stroke_rgba_color: Option<[f32; 4]>,
                    gen_id: Option<usize>,
                    ) -> PyResult<usize> 
    {
        let mut p_points = vec![];

        for pt in points.iter() {
            p_points.push(Point::new(pt.0, pt.1))
        }

        let color = if stroke_rgba_color.is_some() {
            get_color(stroke_rgba_color, None, 1.0, false).unwrap()
        } else {
            get_color(None, stroke_ipg_color, 1.0, false).unwrap()
        };

        let mut canvas_state = access_canvas_state();
        let canvas_id_opt = canvas_state.canvas_ids_str.get(&canvas_id);
        
        if canvas_id_opt.is_none() {
            panic!("PolyLine: You need to define a canvas before adding geometries or your canvas_id is incorrect.")
        }
        
        let id = self.get_id(gen_id);

        let poly_line = IpgPolyLine{ 
                                        id, 
                                        points: p_points, 
                                        poly_points: points.len(), 
                                        mid_point: Point::default(), 
                                        pl_point: Point::default(), 
                                        color, 
                                        width: stroke_width,
                                        stroke_dash_offset,
                                        stroke_dash_segments, 
                                        rotation: 0.0, 
                                        draw_mode: IpgDrawMode::Display, 
                                        status: IpgDrawStatus::Completed,
                                        };
        
        canvas_state.curves.insert(id, IpgWidget::PolyLine(poly_line));
        drop(canvas_state);

        let mut state = access_state();
        state.last_id = id;
        drop(state);

        Ok(id)
    }

    #[pyo3(signature = (canvas_id,
                        top_left_xy,
                        width,
                        height,
                        stroke_width=2.0,
                        stroke_dash_offset=None,
                        stroke_dash_segments=None,
                        stroke_ipg_color=IpgColor::WHITE,
                        stroke_rgba_color=None,
                        fill_ipg_color=None,
                        fill_rgba_color=None,
                        gen_id=None,
                        ))]
    fn add_rectangle(&mut self,
                    canvas_id: String,
                    top_left_xy: (f32, f32),
                    width: f32,
                    height: f32,
                    stroke_width: f32,
                    stroke_dash_offset: Option<usize>,
                    stroke_dash_segments: Option<Vec<f32>>,
                    stroke_ipg_color: Option<IpgColor>,
                    stroke_rgba_color: Option<[f32; 4]>,
                    fill_ipg_color: Option<IpgColor>,
                    fill_rgba_color: Option<[f32; 4]>,
                    gen_id: Option<usize>,
                    ) -> PyResult<usize> 
    {
        let top_left = Point::new(top_left_xy.0, top_left_xy.1);
        let size = Size::new(width, height);
        let mid_point = Point::new(width/2.0, height/2.0);

        let color = if stroke_rgba_color.is_some() {
            get_color(stroke_rgba_color, None, 1.0, false).unwrap()
        } else {
            get_color(None, stroke_ipg_color, 1.0, false).unwrap()
        };

        let fill_color =  get_color(fill_rgba_color, fill_ipg_color, 1.0, false);

        let mut canvas_state = access_canvas_state();
        let canvas_id_opt = canvas_state.canvas_ids_str.get(&canvas_id);
        
        if canvas_id_opt.is_none() {
            panic!("Rectangle: You need to define a canvas before adding geometries or your canvas_id is incorrect.")
        }
        
        let id = self.get_id(gen_id);

        let rectangle = IpgRectangle{ 
                                        id, 
                                        top_left, 
                                        size, 
                                        mid_point, 
                                        color,
                                        fill_color, 
                                        width: stroke_width,
                                        stroke_dash_offset,
                                        stroke_dash_segments, 
                                        rotation: 0.0, 
                                        draw_mode: IpgDrawMode::Display, 
                                        status: IpgDrawStatus::Completed,
                                        };
        
        canvas_state.curves.insert(id, IpgWidget::Rectangle(rectangle));
        drop(canvas_state);

        let mut state = access_state();
        state.last_id = id;
        drop(state);

        Ok(id)
    }

    #[pyo3(signature = (canvas_id,
                        image_path,
                        width,
                        height,
                        position_xy,
                        align_center=true,
                        align_top_left_xy=None,
                        degrees=0.0,
                        gen_id=None,
                        ))]
    fn add_canvas_image(&mut self,
                        canvas_id: String,
                        image_path: String,
                        width: f32,
                        height: f32,
                        position_xy: [f32; 2],
                        align_center: bool,
                        align_top_left_xy: Option<[f32; 2]>,
                        degrees: f32,
                        gen_id: Option<usize>,
                        ) -> PyResult<usize> 
    {
        let bounds = if align_center {
            Rectangle::with_radius(width/2.0)
        } else if align_top_left_xy.is_some() {
            let top_left = align_top_left_xy.unwrap();
            Rectangle::new(Point::new(top_left[0], top_left[1]), Size::new(width, height))
        } else {
            panic!("add_canvas_image: align_center_xy or align_top_left_xy must be defined")
        };

        let position = Point::new(position_xy[0], position_xy[1]);
    
        let path = image::Handle::from_path(image_path);

        let mut canvas_state = access_canvas_state();
        let canvas_id_opt = canvas_state.canvas_ids_str.get(&canvas_id);
        
        if canvas_id_opt.is_none() {
            panic!("Arc: You need to define a canvas before adding geometries or your canvas_id is incorrect.")
        }
        
        let id = self.get_id(gen_id);

        let canvas_image = IpgCanvasImage{ 
                                                id, 
                                                path,
                                                position,
                                                bounds,
                                                width,
                                                height, 
                                                rotation: degrees, 
                                                draw_mode: IpgDrawMode::Display, 
                                                status: IpgDrawStatus::Completed,
                                                };

        canvas_state.image_curves.insert(id, IpgWidget::Image(canvas_image));
        drop(canvas_state);

        let mut state = access_state();
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

        if let Some(py) = on_key_press {
            callbacks.callback_events.insert((self.id, "key pressed".to_string()), py);
        }
        if let Some(py) = on_key_release {
            callbacks.callback_events.insert((self.id, "key released".to_string()), py);
        }

        callbacks.user_data.push((self.id, user_data));

        drop(callbacks);
        
        let mut state = access_state();

        state.keyboard_event_id_enabled = (self.id, enabled);

        state.last_id = self.id;
        drop(state);
        Ok(self.id)
    }

    #[pyo3(signature = (enabled=false, on_move=None, on_enter_window=None, 
                        on_exit_window=None, on_left_press=None, on_left_release=None,
                        on_middle_press=None, on_middle_release=None,
                        on_right_press=None, on_right_release=None,
                        on_middle_scroll_line=None,
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
                        on_middle_scroll_line: Option<PyObject>,
                        user_data: Option<PyObject>,
                        ) -> PyResult<usize>
    {
        self.id += 1;

        let mut callbacks = access_callbacks();

        if let Some(py) = on_move {
            callbacks.callback_events.insert((self.id, "move".to_string()), py);
        }
        if let Some(py) = on_enter_window {
            callbacks.callback_events.insert((self.id, "enter window".to_string()), py);
        }
        if let Some(py) = on_exit_window {
            callbacks.callback_events.insert((self.id, "exit window".to_string()), py);
        }
        if let Some(py) = on_left_press {
            callbacks.callback_events.insert((self.id, "left press".to_string()), py);
        }
        if let Some(py) = on_left_release {
            callbacks.callback_events.insert((self.id, "left release".to_string()), py);
        }
        if let Some(py) = on_middle_press {
            callbacks.callback_events.insert((self.id, "middle press".to_string()), py);
        }
        if let Some(py) = on_middle_release {
            callbacks.callback_events.insert((self.id, "middle release".to_string()), py);
        }
        if let Some(py) = on_right_press {
            callbacks.callback_events.insert((self.id, "right press".to_string()), py);
        }
        if let Some(py) = on_right_release {
            callbacks.callback_events.insert((self.id, "right release".to_string()), py);
        }
        if let Some(py) = on_middle_scroll_line {
            callbacks.callback_events.insert((self.id, "middle scroll line".to_string()), py);
        }

        callbacks.user_data.push((self.id, user_data));

        drop(callbacks);
        
        let mut state = access_state();

        state.mouse_event_id_enabled = (self.id, enabled);

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

        if let Some(py) = on_closed {
            callbacks.callback_events.insert((self.id, "closed".to_string()), py);
        }
        if let Some(py) = on_moved {
                callbacks.callback_events.insert((self.id, "moved".to_string()), py);
        }
        if let Some(py) = on_resized {
                callbacks.callback_events.insert((self.id, "resized".to_string()), py);
        }
        if let Some(py) = on_redraw_requested {
                callbacks.callback_events.insert((self.id, "redraw requested".to_string()), py);
        }
        if let Some(py) = on_close_requested {
            callbacks.callback_events.insert((self.id, "close requested".to_string()), py);
        }
        if let Some(py) = on_focused {
            callbacks.callback_events.insert((self.id, "focused".to_string()), py);
        }
        if let Some(py) = on_unfocused {
            callbacks.callback_events.insert((self.id, "unfocused".to_string()), py);
        }
        if let Some(py) = on_file_hovered {
            callbacks.callback_events.insert((self.id, "file hovered".to_string()), py);
        }
        if let Some(py) = on_file_dropped {
            callbacks.callback_events.insert((self.id, "file dropped".to_string()), py);
        }

        if let Some(py) = on_files_hovered_left {
            callbacks.callback_events.insert((self.id, "files hovered left".to_string()), py);
        }
       
        callbacks.user_data.push((self.id, user_data));

        drop(callbacks);
        
        let mut state = access_state();

        state.window_event_id_enabled = (self.id, enabled);

        state.last_id = self.id;
        drop(state);
        Ok(self.id)
    }

    #[pyo3(signature = (window_id, wid))]
    fn delete_item(&self, window_id: String, wid: usize) 
    {
        let mut all_updates = access_update_items();

        all_updates.deletes.push((window_id, wid));

        drop(all_updates);

    }

    #[pyo3(signature = (wid, item, value))]
    fn update_item(&self, wid: usize, item: PyObject, value: PyObject) {
        let mut all_updates = access_update_items();

        all_updates.updates.push((wid, item, value));

        drop(all_updates);

    }

    #[pyo3(signature = (wid, item, value))]
    fn update_canvas_item(&self, wid: usize, item: PyObject, value: PyObject) {
        let mut canvas_items = access_canvas_update_items();

        canvas_items.updates.push((wid, item, value));

        drop(canvas_items);

    }

    #[pyo3(signature = (window_id, 
                        widget_id, 
                        target_container_str_id, 
                        move_after=None,
                        move_before=None
                        ))]
    fn move_widget(&self,
                    window_id: String,
                    widget_id: usize,
                    target_container_str_id: String,
                    move_after: Option<usize>,
                    move_before: Option<usize>,
                    )
    {
        let mut all_updates = access_update_items();
        
        all_updates.moves.push((window_id, widget_id, target_container_str_id, move_after, move_before));
        
        drop(all_updates);
    }
    
    #[pyo3(signature = (color))]
    fn get_rgba_color(&mut self, color: IpgColor) -> PyResult<[f32; 4]>
    {
 
        let rgb = if let Some(base) = get_color(None, Some(color), 1.0, false) {
            base
        } else {
            panic!("Unable to get the rgba format of the color")
        };

        Ok([rgb.r, rgb.g, rgb.b, 1.0])
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
        IpgWidgets::IpgButtonStyle(btn_style) => {
            button_style_update_item(btn_style, item, value);
        },
        IpgWidgets::IpgCard(crd) => {
            card_item_update(crd, item, value);
        },
        IpgWidgets::IpgCheckBox(chk) => {
            checkbox_item_update(chk, item, value);
        },
        IpgWidgets::IpgCheckboxStyle(chk_style) => {
            checkbox_style_update_item(chk_style, item, value);
        },
        IpgWidgets::IpgColorPicker(cp) => {
            color_picker_update(cp, item, value);
        },
        IpgWidgets::IpgColorPickerStyle(cp_style) => {
            color_picker_style_update_item(cp_style, item, value);
        },
        IpgWidgets::IpgDatePicker(dp) => {
            date_picker_item_update(dp, item, value);
        },
        IpgWidgets::IpgImage(img) => {
            image_item_update(img, item, value);
        },
        // IpgWidgets::IpgMenu(menu) => {
        //     menu_item_update(menu, item, value);
        // },
        // IpgWidgets::IpgMenuStyle(style) => {
        //     menu_style_update_item(style, item, value);
        // },
        // IpgWidgets::IpgMenuBarStyle(style) => {
        //     menu_bar_style_update_item(style, item, value);
        // },
        // IpgWidgets::IpgMenuSeparatorStyle(style) => {
        //     menu_separator_style_update_item(style, item, value);
        // },
        IpgWidgets::IpgPickList(pl) => {
            pick_list_item_update(pl, item, value);
        },
        IpgWidgets::IpgPickListStyle(style) => {
            pick_list_style_update_item(style, item, value);
        },
        IpgWidgets::IpgProgressBar(pb) => {
            progress_bar_item_update(pb, item, value);
        },
        IpgWidgets::IpgProgressBarStyle(style) => {
            progress_bar_style_update_item(style, item, value);
        },
        IpgWidgets::IpgRadio(rd) => {
            radio_item_update(rd, item, value);
        },
        IpgWidgets::IpgRadioStyle(style) => {
            radio_style_update_item(style, item, value);
        },
        IpgWidgets::IpgRule(_) => (),
        IpgWidgets::IpgRuleStyle(style) => {
            rule_style_update_item(style, item, value);
        },
        IpgWidgets::IpgScrollableStyle(style) => {
            scroll_style_update_item(style, item, value)
        },
        IpgWidgets::IpgSelectableText(st) => {
            selectable_text_item_update(st, item, value);
        },
        IpgWidgets::IpgSeparator(sep) => {
            separator_item_update(sep, item, value);
        },
        IpgWidgets::IpgSeparatorStyle(style) => {
            separator_style_update_item(style, item, value);
        },
        IpgWidgets::IpgSlider(slider) => {
            slider_item_update(slider, item, value)
        },
        IpgWidgets::IpgSliderStyle(style) => {
            slider_style_update_item(style, item, value)
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
        IpgWidgets::IpgTextInputStyle(style) => {
            text_input_style_update_item(style, item, value);
        },
        IpgWidgets::IpgTimer(tim) => {
            timer_item_update(tim, item, value);
        },
        IpgWidgets::IpgTimerStyle(style) => {
            timer_style_update_item(style, item, value);
        },
        IpgWidgets::IpgCanvasTimer(ctim) => {
            canvas_timer_item_update(ctim, item, value);
        },
        IpgWidgets::IpgCanvasTimerStyle(style) => {
            canvas_timer_style_update_item(style, item, value);
        },
        IpgWidgets::IpgToggler(tog) => {
            toggler_item_update(tog, item, value);
        },
        IpgWidgets::IpgTogglerStyle(style) => {
            toggler_style_update_item(style, item, value);
        },
    }
}

fn match_container(container: &mut IpgContainers, 
                    item: PyObject, 
                    value: PyObject, 
                    canvas_state: &mut IpgCanvasState,
                    ) 
{
    match container {
        IpgContainers::IpgCanvas(_can) => {
            canvas_item_update(canvas_state, item, value);
        }
        IpgContainers::IpgMouseArea(m_area) => {
            mousearea_item_update(m_area, item, value);
        },
        IpgContainers::IpgOpaque(op) => {
            opaque_item_update(op, item, value);
        },
        IpgContainers::IpgStack(stack) => {
            stack_item_update(stack, item, value);
        },
        IpgContainers::IpgTable(table) => {
            table_item_update(table, item, value);
        },
        IpgContainers::IpgScrollable(scroll) => {
            scrollable_item_update(scroll, item, value);
        },
        IpgContainers::IpgWindow(wnd) => {
            window_item_update(wnd, item, value);
        },
        _ => (),
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
    m.add_class::<IpgButtonStyleParam>()?;
    m.add_class::<IpgCanvasParam>()?;
    m.add_class::<IpgCanvasGeometryParam>()?;
    m.add_class::<IpgDrawMode>()?;
    m.add_class::<IpgCanvasWidget>()?;
    m.add_class::<IpgCardStyle>()?;
    m.add_class::<IpgCardParam>()?;
    m.add_class::<IpgCheckboxParam>()?;
    m.add_class::<IpgCheckboxStyleParam>()?;
    m.add_class::<IpgColor>()?;
    m.add_class::<IpgColorPickerParam>()?;
    m.add_class::<IpgColorPickerStyleParam>()?;
    m.add_class::<IpgDatePickerParam>()?;
    m.add_class::<IpgImageContentFit>()?;
    m.add_class::<IpgImageFilterMethod>()?;
    m.add_class::<IpgImageParam>()?;
    m.add_class::<IpgImageRotation>()?;
    // m.add_class::<IpgMenuParam>()?;
    // m.add_class::<IpgMenuType>()?;
    // m.add_class::<IpgMenuStyleParam>()?;
    // m.add_class::<IpgMenuBarStyleParam>()?;
    m.add_class::<IpgMouseAreaParam>()?;
    m.add_class::<IpgMousePointer>()?;
    m.add_class::<IpgOpaqueParam>()?;
    m.add_class::<IpgPickListParam>()?;
    m.add_class::<IpgPickListStyleParam>()?;
    m.add_class::<IpgPickListHandle>()?;
    m.add_class::<IpgProgressBarParam>()?;
    m.add_class::<IpgProgressBarStyleParam>()?;
    m.add_class::<IpgRadioDirection>()?;
    m.add_class::<IpgRadioParam>()?;
    m.add_class::<IpgRadioStyleParam>()?;
    m.add_class::<IpgRuleStyleParam>()?;
    m.add_class::<IpgScrollableDirection>()?;
    m.add_class::<IpgScrollableParam>()?;
    m.add_class::<IpgScrollableStyleParam>()?;
    m.add_class::<IpgSelectableTextParam>()?;
    m.add_class::<IpgSeparatorParam>()?;
    m.add_class::<IpgSeparatorType>()?;
    m.add_class::<IpgSeparatorStyleParam>()?;
    m.add_class::<IpgSliderParam>()?;
    m.add_class::<IpgSliderStyleParam>()?;
    m.add_class::<IpgStackParam>()?;
    m.add_class::<IpgStyleStandard>()?;
    m.add_class::<IpgSvgParam>()?;
    m.add_class::<IpgTableRowHighLight>()?;
    m.add_class::<IpgTableParam>()?;
    m.add_class::<IpgTableWidget>()?;
    m.add_class::<IpgTextInputParam>()?;
    m.add_class::<IpgTextInputStyleParam>()?;
    m.add_class::<IpgTextParam>()?;
    m.add_class::<IpgTimerParam>()?;
    m.add_class::<IpgTimerStyleParam>()?;
    m.add_class::<IpgCanvasTimerParam>()?;
    m.add_class::<IpgCanvasTimerStyleParam>()?;
    m.add_class::<IpgTogglerParam>()?;
    m.add_class::<IpgTogglerStyleParam>()?;
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
        Some(id) => *id,
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
        None => panic!("The main window id could not be found using parent_id {}, check that your parent_id matches a container ", parent_id)
    };

    let wnd_id_usize = match state.windows_str_ids.get(&wnd_id_str) {
        Some(id) => *id,
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

pub fn find_parent_uid(ipg_ids: &[IpgIds], parent_id: String) -> usize {

    for id in ipg_ids.iter() {
        if id.container_id == Some(parent_id.clone()) {
            return id.id
        }
    }
    panic!("Parent id {:?} not found in function find_parent_uid()", parent_id)
}

pub fn get_id() -> usize {
    let mut state = access_state();
    state.last_id += 1;
    let id = state.last_id;
    drop(state);
    id
}
