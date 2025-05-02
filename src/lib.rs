//!lib for all of the python callable functions using pyo3
#![allow(clippy::too_many_arguments, clippy::redundant_closure)]
#![allow(clippy::type_complexity)]
use canvas::canvas_helpers::{build_polygon, get_mid_point, to_radians};
use canvas::draw_canvas::{IpgCanvasState, IpgDrawMode, IpgDrawStatus, IpgWidget};
use canvas::geometries::{IpgArc, IpgBezier, IpgCanvasImage, IpgCanvasWidget, 
    IpgCircle, IpgEllipse, IpgLine, IpgPolyLine, IpgPolygon, IpgRectangle};

use iced::widget::image;
use iced_aw::iced_fonts;

use ipg_widgets::ipg_color_picker::{color_picker_style_update_item, color_picker_update, 
    IpgColorPicker, IpgColorPickerParam, IpgColorPickerStyle, IpgColorPickerStyleParam};
use ipg_widgets::ipg_divider::{divider_horizontal_item_update, divider_style_update_item, 
    divider_vertical_item_update, IpgDividerHorizontal, IpgDividerParam, IpgDividerStyle, 
    IpgDividerStyleParam, IpgDividerVertical};
use ipg_widgets::ipg_separator::{separator_item_update, separator_style_update_item, 
    IpgSeparator, IpgSeparatorParam, IpgSeparatorStyle, IpgSeparatorStyleParam, IpgSeparatorType};
use ipg_widgets::ipg_timer_canvas::{canvas_timer_item_update, canvas_timer_style_update_item, 
    IpgCanvasTimer, IpgCanvasTimerParam, IpgCanvasTimerStyle, IpgCanvasTimerStyleParam};

use polars::frame::DataFrame;
use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::PyObject;
use pyo3_polars::PyDataFrame;

use iced::window::{self, Position};
use iced::{Color, Font, Length, Point, Radians, Rectangle, Size, Theme, Vector};
use iced::widget::text::{self, LineHeight};

use core::panic;
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
use ipg_widgets::ipg_card::{card_item_update, card_style_update, IpgCard, 
    IpgCardParam, IpgCardStyle, IpgCardStyleParam};
use ipg_widgets::ipg_checkbox::{checkbox_item_update, checkbox_style_update_item, 
    IpgCheckBox, IpgCheckboxParam, IpgCheckboxStyle, IpgCheckboxStyleParam};
use ipg_widgets::ipg_column::{column_item_update, IpgColumn, IpgColumnParam};
use ipg_widgets::ipg_container::{container_item_update, container_style_update_item, 
    IpgContainer, IpgContainerParam, IpgContainerStyle, IpgContainerStyleParam};
use ipg_widgets::ipg_date_picker::{date_picker_item_update, 
        IpgDatePicker, IpgDatePickerParam};
use ipg_widgets::ipg_events::IpgEvents;
use ipg_widgets::ipg_image::{image_item_update, IpgImage, 
        IpgImageContentFit, IpgImageFilterMethod, 
        IpgImageParam, IpgImageRotation};
use ipg_widgets::ipg_menu::{menu_bar_style_update_item, menu_item_update, 
    menu_style_update_item, IpgMenu, IpgMenuBarStyle, IpgMenuBarStyleParam, 
    IpgMenuParam, IpgMenuStyle, IpgMenuStyleParam};
use ipg_widgets::ipg_mousearea::{mousearea_item_update, IpgMouseArea, 
        IpgMouseAreaParam, IpgMousePointer};
use ipg_widgets::ipg_opaque::{opaque_item_update, opaque_style_update_item, 
        IpgOpaque, IpgOpaqueParam, IpgOpaqueStyle};
use ipg_widgets::ipg_pick_list::{convert_pyobject_vec_string, pick_list_item_update, 
    pick_list_style_update_item, IpgPickList, IpgPickListHandle, IpgPickListParam, 
    IpgPickListStyle, IpgPickListStyleParam};
use ipg_widgets::ipg_progress_bar::{progress_bar_item_update, progress_bar_style_update_item, 
    IpgProgressBar, IpgProgressBarParam, IpgProgressBarStyle, IpgProgressBarStyleParam};
use ipg_widgets::ipg_radio::{radio_item_update, radio_style_update_item, IpgRadio, 
    IpgRadioDirection, IpgRadioParam, IpgRadioStyle, IpgRadioStyleParam};
use ipg_widgets::ipg_row::{row_item_update, IpgRow, IpgRowParam};
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
use ipg_widgets::ipg_table::{table_dataframe_update, table_item_update, 
    table_style_update_item, IpgTable, IpgTableParam, IpgTableStyle, IpgTableStyleParam};
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
    get_horizontal_alignment, get_line_height, get_padding_f32, 
    get_padding_f64, get_shaping, get_vertical_alignment, get_width};

use graphics::colors::{get_color, IpgColor};
use style::styling::{readable, IpgStyleStandard};

const ICON_FONT_BOOT: Font = Font::with_name("bootstrap-icons");

use std::sync::{Mutex, MutexGuard};
use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct Events {
    events: Lazy<HashMap<(usize, String), PyObject>>,
}

pub static EVENTS: Mutex<Events> = Mutex::new(Events {
    events:  Lazy::new(||HashMap::new()),
});

pub fn access_events() -> MutexGuard<'static, Events> {
    EVENTS.lock().unwrap()
}

#[derive(Debug)]
pub struct Callbacks {
    callbacks: Lazy<HashMap<(usize, String), PyObject>>,
}

pub static CALLBACKS: Mutex<Callbacks> = Mutex::new(Callbacks {
    callbacks: Lazy::new(||HashMap::new()),
});

pub fn access_callbacks() -> MutexGuard<'static, Callbacks> {
    CALLBACKS.lock().unwrap()
}

#[derive(Debug)]
pub struct UserData1 {
    user_data: Lazy<HashMap<usize, PyObject>>,
}

pub static USERDATA1: Mutex<UserData1> = Mutex::new(UserData1 {
    user_data: Lazy::new(||HashMap::new()),
});

pub fn access_user_data1() -> MutexGuard<'static, UserData1> {
    USERDATA1.lock().unwrap()
}

#[derive(Debug)]
pub struct UserData2 {
    user_data: Lazy<HashMap<usize, PyObject>>,
}

pub static USERDATA2: Mutex<UserData2> = Mutex::new(UserData2 {
    user_data: Lazy::new(||HashMap::new()),
});

pub fn access_user_data2() -> MutexGuard<'static, UserData2> {
    USERDATA2.lock().unwrap()
}

#[derive(Debug)]
pub struct UpdateItems {
    // wid, (item, value)
    pub updates: Vec<(usize, PyObject, PyObject)>, 
    // window_id_widget_id, (window_id, wid, target_container_str_id, move_after(wid), move_before(wid))
    pub moves: Vec<(String, usize, String, Option<usize>, Option<usize>)>,
    // window_id, wid
    pub deletes: Vec<(String, usize)>,
    pub shows: Vec<(String, Vec<(usize, bool)>)>,
    pub dataframes: Vec<(usize, PyObject, PyDataFrame)>,
    pub new_widgets: Lazy<HashMap<usize, IpgWidgets>>,
}

pub static UPDATE_ITEMS: Mutex<UpdateItems> = Mutex::new(UpdateItems {
    updates: vec![],
    moves: vec![],
    deletes: vec![],
    shows: vec![],
    dataframes: vec![],
    new_widgets: Lazy::new(||HashMap::new()),
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
    pub ids_ipd_ids: Lazy<HashMap<usize, Vec<IpgIds>>>,  // <window_id=usize, Vec<IpgIds=structure>>
    pub last_id: usize,
    pub gen_ids: Vec<usize>,

    pub containers: Lazy<HashMap<usize, IpgContainers>>,
    pub container_ids: Lazy<HashMap<usize, Vec<usize>>>,  // <window_id=usize, vec<container_id=usize>>
    pub container_str_ids: Lazy<HashMap<String, usize>>, // get container usize id based on container string
    pub container_wnd_str_ids: Lazy<HashMap<String, String>>, // get window string id based on container string id
    pub container_window_usize_ids: Lazy<HashMap<usize, usize>>, //get window usize id based on container usize id
    
    pub widgets: Lazy<HashMap<usize, IpgWidgets>>,
    pub widget_container_ids: Lazy<HashMap<usize, String>>, //widget_id=usize, container_id=String
    
    pub windows: Vec<IpgWindow>,
    pub windows_iced_ipg_ids: Lazy<HashMap<window::Id, usize>>, // <iced id, ipg id>
    pub windows_str_ids: Lazy<HashMap<String, usize>>,  // <ipg_id=str, ipg id>
    pub window_debug: Lazy<HashMap<window::Id, (usize, bool)>>, // (wid, debug)
    pub window_theme: Lazy<HashMap<window::Id, (usize, Theme)>>, // (wid, window Theme)
    pub window_mode: Lazy<HashMap<window::Id, (usize, window::Mode)>>,

    pub events: Vec<IpgEvents>,
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
        ids_ipd_ids: Lazy::new(||HashMap::new()),
        last_id: 0,
        gen_ids: vec![],

        containers: Lazy::new(||HashMap::new()),
        container_ids: Lazy::new(||HashMap::new()),
        container_str_ids: Lazy::new(||HashMap::new()),
        container_wnd_str_ids: Lazy::new(||HashMap::new()),
        container_window_usize_ids: Lazy::new(||HashMap::new()),

        widgets: Lazy::new(||HashMap::new()),
        widget_container_ids: Lazy::new(||HashMap::new()),

        windows: vec![],
        windows_iced_ipg_ids: Lazy::new(||HashMap::new()),
        windows_str_ids: Lazy::new(||HashMap::new()),
        window_debug: Lazy::new(||HashMap::new()),
        window_theme: Lazy::new(||HashMap::new()),
        window_mode: Lazy::new(||HashMap::new()),
        
        events: vec![],
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
    pub windows_opened: Vec<window::Id>,
    pub windows_hidden: Vec<window::Id>,

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
            windows_opened: vec![],
            windows_hidden: vec![],

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
    group_index: usize,
    theme: Theme,
}

#[pymethods]
impl IPG {
    #[new]
    fn new() -> IPG {
        IPG {
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
    fn generate_id(&self) -> PyResult<usize>
    {
        let mut state = access_state();
        state.last_id += 1;
        let id = state.last_id;
        state.gen_ids.push(id);
        drop(state);
        Ok(id)
    }

    #[pyo3(signature = (
        window_id, 
        title, 
        width, 
        height,
        max_width=None, 
        max_height=None,
        min_width=None, 
        min_height=None,
        pos_x=None, 
        pos_y=None,
        pos_centered=false, 
        resizable=true,
        decorations=true, 
        transparent=false,
        level=IpgWindowLevel::Normal,
        scale_factor=1.0,
        theme=IpgWindowTheme::Dark, 
        exit_on_close=false, 
        on_resize=None, 
        mode=IpgWindowMode::Windowed, 
        debug=false, 
        user_data=None,
        gen_id=None
        ))]
    fn add_window(
        &mut self,
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

        self.theme = get_iced_window_theme(theme.clone());
        let iced_theme = get_iced_window_theme(theme);

        let mut state = access_state();

        if state.windows_str_ids.get(&window_id).is_some() {
            panic!("Window id {} is not unique", window_id)
        };

        if let Some(py) = on_resize {
            add_callback_to_mutex(id, "on_resize".to_string(), py);
        }

        if let Some(py) = user_data {
            add_user_data_to_mutex(id, py);
        }

        state.windows_str_ids.insert(window_id.clone(), id);

        state.ids_ipd_ids.insert(id, vec![IpgIds{id, parent_uid: 0, container_id: Some(window_id.clone()),
                                                parent_id: "".to_string(), is_container: true}]);

        state.container_ids.insert(id, vec![id]);
        // TODO: Only one of these below are needed but some subtle issues arise when not used together.
        // Will need to work through it in the near future.  At the onset, used only one window then
        // iced made multi-window so sort of patch it to work but need to revisit it.
        state.containers.insert(id, IpgContainers::IpgWindow(
            IpgWindow::new(
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
                )));
        
        state.windows.push(
            IpgWindow::new(
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
                ));
 
        drop(state);
        Ok(id)

    }

    #[pyo3(signature = (
        window_id,
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
    fn add_canvas(
        &self,
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

        set_state_of_container(id, window_id.clone(), Some(canvas_id.clone()), prt_id);

        let mut state = access_state();

        set_state_cont_wnd_ids(&mut state, &window_id, canvas_id.clone(), id, "add_canvas".to_string());

        state.containers.insert(id, IpgContainers::IpgCanvas(IpgCanvas::new(
                                                id,
                                            )));
 
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
    
    #[pyo3(signature = (
        window_id, 
        container_id, 
        parent_id=None,
        width=None, 
        width_fill=false, 
        height=None, 
        height_fill=false, 
        clip=false, 
        centered=true,
        max_height=f32::INFINITY, 
        max_width=f32::INFINITY,
        align_x=IpgHorizontalAlignment::Left, 
        align_y=IpgVerticalAlignment::Top,
        padding=vec![0.0], 
        show=true, 
        style_id=None, 
        ))]
    fn add_container(
        &self,
        window_id: String,
        container_id: String,
        // **above required
        parent_id: Option<String>,
        width: Option<f32>,
        width_fill: bool,
        height: Option<f32>,
        height_fill: bool,
        clip: bool,
        centered: bool,
        max_height: f32,
        max_width: f32,
        mut align_x: IpgHorizontalAlignment,
        mut align_y: IpgVerticalAlignment, 
        padding: Vec<f64>, 
        show: bool,
        style_id: Option<usize>,
        ) -> PyResult<usize>
    {
        let id = self.get_id(None);

        let width = get_width(width, width_fill);
        let height = get_height(height, height_fill);
        let padding = get_padding_f64(padding);
        
        let prt_id = match parent_id {
            Some(id) => id,
            None => window_id.clone(),
        };

        if centered {
            align_x = IpgHorizontalAlignment::Center;
            align_y = IpgVerticalAlignment::Center;
        };

        set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

        let mut state = access_state();

        set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_container".to_string());

        state.containers.insert(id, IpgContainers::IpgContainer(
            IpgContainer::new(
                id,
                show,
                padding,
                width,
                height,
                max_width,
                max_height,
                align_x,
                align_y,
                clip,
                style_id, 
            )));

        drop(state);
        Ok(id)

    }

    #[pyo3(signature = ( 
        background_color=None, 
        background_rgba=None,
        border_color=None, 
        border_rgba=None,
        border_radius = vec![0.0], 
        border_width=0.0,
        shadow_color=None, 
        shadow_rgba=None,
        shadow_offset_xy=[0.0, 0.0],
        shadow_blur_radius=0.0,
        text_color=None, 
        text_rgba=None,
        gen_id=None
        ))]
    fn add_container_style(
        &self,
        background_color: Option<IpgColor>,
        background_rgba: Option<[f32; 4]>,
        border_color: Option<IpgColor>,
        border_rgba: Option<[f32; 4]>,
        border_radius: Vec<f32>,
        border_width: f32,
        shadow_color: Option<IpgColor>,
        shadow_rgba: Option<[f32; 4]>,
        shadow_offset_xy: [f32; 2],
        shadow_blur_radius: f32,
        text_color: Option<IpgColor>,
        text_rgba: Option<[f32; 4]>,
        gen_id: Option<usize>,
        ) -> PyResult<usize>
    {
        let id = self.get_id(gen_id);

        let background_color: Option<Color> = 
            get_color(background_rgba, background_color, 1.0, false);
        let border_color: Option<Color> = 
            get_color(border_rgba, border_color, 1.0, false);
        let shadow: Option<Color> = 
            get_color(shadow_rgba, shadow_color, 1.0, false);
        let text_color: Option<Color> = 
            get_color(text_rgba, text_color, 1.0, false);

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgContainerStyle(
            IpgContainerStyle::new( 
                id,
                background_color,
                border_color,
                border_radius,
                border_width,
                shadow,
                shadow_offset_xy,
                shadow_blur_radius,
                text_color,
                )));

        drop(state);
        Ok(id)
    }

    #[pyo3(signature = (
        window_id, 
        container_id, 
        parent_id=None,
        align=IpgAlignment::Start, 
        width=None, height=None,
        width_fill=false, 
        height_fill=false,
        max_width=f32::INFINITY, 
        padding=vec![0.0], 
        spacing=10.0, 
        clip=false, 
        show=true,
        ))]
    fn add_column(
        &self,
        window_id: String,
        container_id: String,
        // **above required
        parent_id: Option<String>,
        align: IpgAlignment,
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
        let id = self.get_id(None);
        
        let width = get_width(width, width_fill);
        let height = get_height(height, height_fill);

        let padding = get_padding_f64(padding);

        let prt_id = match parent_id {
            Some(id) => id,
            None => window_id.clone(),
        };

        set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

        let mut state = access_state();

        set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_column".to_string());

        state.containers
            .insert(id, IpgContainers::IpgColumn(
                IpgColumn::new(
                    id,  
                    show, 
                    spacing, 
                    padding, 
                    width, 
                    height, 
                    max_width, 
                    align,
                    clip,
                )));

    drop(state);
    Ok(id)

    }

    #[pyo3(signature = ( 
        window_id,
        container_id,
        bar_items,
        menu_items,
        bar_width=None,
        item_widths=None,
        parent_id=None, 
        bar_spacing=None,
        bar_padding=None,
        bar_height=None,
        bar_check_bounds_width=None,
        item_spacing=None,
        item_offset=None, 
        on_select=None,
        menu_bar_style=None,
        menu_style=None,
        show=true, 
        user_data=None, 
        gen_id=None
        ))]
    fn add_menu(
        &self,
        window_id: String,
        container_id: String,
        bar_items: usize,
        menu_items: Vec<usize>,
        bar_width: Option<f32>,
        item_widths: Option<Vec<f32>>,
        parent_id: Option<String>,
        bar_spacing: Option<f32>,
        bar_padding: Option<Vec<f32>>,
        bar_height: Option<f32>,
        bar_check_bounds_width: Option<f32>,
        item_spacing: Option<Vec<f32>>,
        item_offset: Option<Vec<f32>>,
        on_select: Option<PyObject>,
        menu_bar_style: Option<usize>,
        menu_style: Option<usize>,
        show: bool,
        user_data: Option<PyObject>,
        gen_id: Option<usize>,
    ) -> PyResult<usize> 
    {
        let id = self.get_id(gen_id);

        if let Some(py) = on_select {
            add_callback_to_mutex(id, "on_select".to_string(), py);
        }

        if let Some(py) = user_data {
            add_user_data_to_mutex(id, py);
        }

        let spacing = bar_spacing.unwrap_or(0.0);

        let padding = get_padding_f32(bar_padding);
        
        let height = get_height(bar_height, false);

        let bar_width = get_width(bar_width, false);

        let item_widths = if item_widths.is_some() {
            let mut widths = vec![];
            for w in item_widths.unwrap() {
                widths.push(get_width(Some(w), false));
            }
            widths
        } else {
            vec![Length::Shrink]
        };
 
        let check_bounds_width = bar_check_bounds_width.unwrap_or(50.0) ;

        let prt_id = match parent_id {
            Some(id) => id,
            None => window_id.clone(),
        };
        
        set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

        let mut state = access_state();

        set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_menu".to_string());

        state.containers.insert(id, IpgContainers::IpgMenu(
            IpgMenu::new(
                id,
                bar_items,
                menu_items,
                bar_width,
                item_widths,
                spacing,
                padding,
                height,
                check_bounds_width,
                item_spacing,
                item_offset,
                menu_bar_style,
                menu_style,
                self.theme.clone(),
                show,
                )));

        drop(state);
        Ok(id)
    }

    #[pyo3(signature = (
        base_color=None,
        base_rgba=None,
        border_color=None,
        border_rgba=None,
        border_radius=None,
        border_width=None,
        shadow_color=None,
        shadow_rgba=None,
        shadow_offset_xy=None,
        shadow_blur_radius=None,
        gen_id=None))]
    fn add_menu_bar_style(
        &self,
        base_color: Option<IpgColor>,
        base_rgba: Option<[f32; 4]>,
        border_color: Option<IpgColor>,
        border_rgba: Option<[f32; 4]>,
        border_radius: Option<Vec<f32>>,
        border_width: Option<f32>,
        shadow_color: Option<IpgColor>,
        shadow_rgba: Option<[f32; 4]>,
        shadow_offset_xy: Option<[f32; 2]>,
        shadow_blur_radius: Option<f32>,
        gen_id: Option<usize>,
        ) -> PyResult<usize>
    {
        let id = self.get_id(gen_id);

        let base: Option<Color> = 
            get_color(base_rgba, base_color, 1.0, false);
        let border_color: Option<Color> = 
            get_color(border_rgba, border_color, 1.0, false);
        let shadow_color: Option<Color> = 
            get_color(shadow_rgba, shadow_color, 1.0, false);

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgMenuBarStyle(
            IpgMenuBarStyle::new( 
                id,
                base,
                border_color,
                border_radius,
                border_width,
                shadow_color,
                shadow_offset_xy,
                shadow_blur_radius,
                )));

        drop(state);
        Ok(id)
    }

    #[pyo3(signature = (
        base_color=None,
        base_rgba=None,
        border_color=None,
        border_rgba=None,
        border_radius=None,
        border_width=None,
        shadow_color=None,
        shadow_rgba=None,
        shadow_offset_xy=None,
        shadow_blur_radius=None,
        path_base_color=None,
        path_base_rgba=None,
        path_border_color=None,
        path_border_rgba=None,
        path_border_radius=None,
        path_border_width=None,
        gen_id=None))]
    fn add_menu_style(
        &self,
        base_color: Option<IpgColor>,
        base_rgba: Option<[f32; 4]>,
        border_color: Option<IpgColor>,
        border_rgba: Option<[f32; 4]>,
        border_radius: Option<Vec<f32>>,
        border_width: Option<f32>,
        shadow_color: Option<IpgColor>,
        shadow_rgba: Option<[f32; 4]>,
        shadow_offset_xy: Option<[f32; 2]>,
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
        
        let base_color: Option<Color> = 
            get_color(base_rgba, base_color, 1.0, false);
        let border_color: Option<Color> = 
            get_color(border_rgba, border_color, 1.0, false);
        let shadow_color: Option<Color> = 
            get_color(shadow_rgba, shadow_color, 1.0, false);
        let path_base: Option<Color> = 
            get_color(path_base_rgba, path_base_color, 1.0, false);
        let path_border_color: Option<Color> = 
            get_color(path_border_rgba, path_border_color, 1.0, false);

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgMenuStyle(
            IpgMenuStyle::new(  
                id,
                base_color,
                border_color,
                border_radius,
                border_width,
                shadow_color,
                shadow_offset_xy,
                shadow_blur_radius,
                path_base,
                path_border_color,
                path_border_radius,
                path_border_width,
                )));

        drop(state);
        Ok(id)
    }

    #[pyo3(signature = (
        window_id, 
        container_id, 
        parent_id=None, 
        gen_id=None, 
        show=true, 
        mouse_pointer=None,
        on_press=None, 
        on_release=None,
        on_right_press=None, 
        on_right_release=None,
        on_middle_press=None, 
        on_middle_release=None,
        on_enter=None, 
        on_move=None, 
        on_exit=None,
        user_data=None,
        ))]
    fn add_mousearea(
        &self,
        window_id: String,
        container_id: String,
        // required above
        parent_id: Option<String>,
        gen_id: Option<usize>,
        show: bool,
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
        ) -> PyResult<usize>
    {
        let id = self.get_id(gen_id);

        let prt_id = match parent_id {
            Some(id) => id,
            None => window_id.clone(),
        };

        if let Some(py) = on_press {
        add_callback_to_mutex(id, "on_press".to_string(), py);
        }
        
        if let Some(py) = on_release {
            add_callback_to_mutex(id, "event_name".to_string(), py);
        }
        
        if let Some(py) = on_right_press {
            add_callback_to_mutex(id, "on_right_press".to_string(), py);
        }
        
        if let Some(py) = on_right_release {
            add_callback_to_mutex(id, "on_right_release".to_string(), py);
        }
        
        if let Some(py) = on_middle_press {
            add_callback_to_mutex(id, "on_middle_press".to_string(), py);
        }
        
        if let Some(py) = on_middle_release {
            add_callback_to_mutex(id, "on_middle_release".to_string(), py);
        }
        
        if let Some(py) = on_enter {
            add_callback_to_mutex(id, "on_enter".to_string(), py);
        }
        
        if let Some(py) = on_move {
            add_callback_to_mutex(id, "on_move".to_string(), py);
        }
        
        if let Some(py) = on_exit {
            add_callback_to_mutex(id, "on_exit".to_string(), py);
        }

        if let Some(py) = user_data {
            add_user_data_to_mutex(id, py);
        }

        set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

        let mut state = access_state();

        set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_mousearea".to_string());

        state.containers.insert(id, IpgContainers::IpgMouseArea(
            IpgMouseArea::new(
                id,
                mouse_pointer,  
                show, 
                )));

        drop(state);
        Ok(id)

    }

    #[pyo3(signature = (
        window_id, 
        container_id, 
        parent_id=None,
        width=None, 
        height=None, 
        width_fill=false, 
        height_fill=false,
        centered=true,
        align_x=IpgHorizontalAlignment::Left, 
        align_y=IpgVerticalAlignment::Top,
        mouse_on_press=None,
        user_data=None,
        show=true, 
        style_id=None,
        gen_id=None,
        ))]
    fn add_opaque_container(
        &self,
        window_id: String,
        container_id: String,
        // required above
        parent_id: Option<String>,
        width: Option<f32>,
        height: Option<f32>,
        width_fill: bool,
        height_fill: bool,
        centered: bool,
        mut align_x: IpgHorizontalAlignment,
        mut align_y: IpgVerticalAlignment,
        mouse_on_press: Option<PyObject>,
        user_data: Option<PyObject>,
        show: bool,
        style_id: Option<usize>,
        gen_id: Option<usize>,
        ) -> PyResult<usize> 
    {
        let id = self.get_id(gen_id);

        let width = get_width(width, width_fill);
        let height = get_height(height, height_fill);

        if centered {
            align_x = IpgHorizontalAlignment::Center;
            align_y = IpgVerticalAlignment::Center;
        };

        let include_mouse_area = if let Some(py) = mouse_on_press {
            add_callback_to_mutex(id, "on_press".to_string(), py);
            true
        } else {
            false
        };

        if let Some(py) = user_data {
            add_user_data_to_mutex(id, py);
        }

        let prt_id = match parent_id {
            Some(id) => id,
            None => window_id.clone(),
        };

        set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

        let mut state = access_state();

        set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_opaque".to_string());

        state.containers.insert(id, IpgContainers::IpgOpaque(
            IpgOpaque::new(
                id,  
                width, 
                height,
                align_x,
                align_y,
                include_mouse_area,
                show,
                style_id
                )));

        drop(state);         
        Ok(id)
    }

    #[pyo3(signature = ( 
        background_color=None, 
        background_rgba=None,
        gen_id=None
        ))]
    fn add_opaque_style(
        &self,
        background_color: Option<IpgColor>,
        background_rgba: Option<[f32; 4]>,
        gen_id: Option<usize>,
        ) -> PyResult<usize>
    {
        let id = self.get_id(gen_id);

        let mut state = access_state();

        let background_color: Option<Color> = get_color(background_rgba, background_color, 1.0, false);

        state.widgets.insert(id, IpgWidgets::IpgOpaqueStyle(
            IpgOpaqueStyle::new( 
                id,
                background_color,
                )));
 
        drop(state);
        Ok(id)
    }

    #[pyo3(signature = (
        window_id, 
        container_id, 
        parent_id=None,
        align=IpgAlignment::Start, 
        width=None, 
        height=None, 
        width_fill=false, 
        height_fill=false,
        padding=vec![0.0], 
        spacing=10.0, 
        clip=false,
        show=true,
        ))]
    fn add_row(
        &self,
        window_id: String,
        container_id: String,
        // required above
        parent_id: Option<String>,
        align: IpgAlignment,
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
        let id = self.get_id(None);

        let width = get_width(width, width_fill);
        let height = get_height(height, height_fill);

        let padding = get_padding_f64(padding);

        let prt_id = match parent_id {
            Some(id) => id,
            None => window_id.clone(),
        };

        set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

        let mut state = access_state();

        set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_row".to_string());

        state.containers.
            insert(id, IpgContainers::IpgRow(IpgRow::new(
                id,  
                show, 
                spacing, 
                padding, 
                width, 
                height, 
                align,
                clip,
            )));

        drop(state);         
        Ok(id)

    }

    #[pyo3(signature = (
        window_id, 
        container_id, 
        parent_id=None,
        width=None, 
        height=None, 
        width_fill=false, 
        height_fill=false,
        hide_index=None, 
        show=true,
        ))]
    fn add_stack(
        &self,
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
        let id = self.get_id(None);

        let width = get_width(width, width_fill);
        let height = get_height(height, height_fill);

        let prt_id = match parent_id {
            Some(id) => id,
            None => window_id.clone(),
        };

        set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

        let mut state = access_state();

        set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_stack".to_string());

        state.containers.insert(id, IpgContainers::IpgStack(
            IpgStack::new(
                id,  
                width, 
                height,
                hide_index,
                show,
                )));

        drop(state);         
        Ok(id)

    }

    #[pyo3(signature = (
        window_id, 
        container_id, 
        parent_id=None,
        width=None, 
        height=None, 
        width_fill=false, 
        height_fill=false, 
        direction=IpgScrollableDirection::Vertical, 
        h_bar_width=10.0, 
        h_bar_margin=0.0, 
        h_scroller_width=10.0,
        h_spacing=0.0, 
        h_bar_alignment=IpgScrollableAlignment::Start,
        v_bar_width=10.0, 
        v_bar_margin=0.0, 
        v_scroller_width=10.0,
        v_spacing=0.0, 
        v_bar_alignment=IpgScrollableAlignment::Start,
        on_scroll=None, 
        user_data=None,
        style_id=None,
        ))]
    fn add_scrollable(
        &self,
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
        h_spacing: f32,
        h_bar_alignment: IpgScrollableAlignment,
        v_bar_width: f32,
        v_bar_margin: f32,
        v_scroller_width: f32,
        v_spacing: f32,
        v_bar_alignment: IpgScrollableAlignment,
        on_scroll: Option<PyObject>,
        user_data: Option<PyObject>,
        style_id: Option<usize>,
        ) -> PyResult<usize>
    {
        let id = self.get_id(None);

        if let Some(py) = on_scroll {
            add_callback_to_mutex(id, "on_scroll".to_string(), py);
        }

        if let Some(py) = user_data {
            add_user_data_to_mutex(id, py);
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

        set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

        let mut state = access_state();

        set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_scrollable".to_string());
      
        state.containers.insert(id, IpgContainers::IpgScrollable(
            IpgScrollable::new( 
                id,
                width,
                height,
                direction,
                h_bar_width,
                h_bar_margin,
                h_scroller_width,
                h_spacing,
                h_bar_alignment,
                v_bar_width,
                v_bar_margin,
                v_scroller_width,
                v_spacing,
                v_bar_alignment,
                style_id,
                )));
 
        drop(state);
        Ok(id)

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
        gen_id=None
        ))]
    fn add_scrollable_style(
        &self,
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

        let background_color: Option<Color> = 
            get_color(background_rgba, background_color, 1.0, false);
        let border_color: Option<Color> = 
            get_color(border_rgba, border_color, 1.0, false);
        let shadow_color: Option<Color> = 
            get_color(shadow_rgba, shadow_color, 1.0, false);
        let text_color: Option<Color> = 
            get_color(text_rgba, text_color, 1.0, false);

        let scrollbar_color: Option<Color> = 
            get_color(scrollbar_rgba, scrollbar_color, 1.0, false);
        let scrollbar_border_color: Option<Color> = 
            get_color(scrollbar_border_rgba, scrollbar_border_color, 1.0, false);
        
        let scroller_color: Option<Color> = 
            get_color(scroller_rgba, scroller_color, 1.0, false);
        let scroller_color_hovered: Option<Color> = 
            get_color(scroller_rgba_hovered, scroller_color_hovered, 1.0, false);
        let scroller_color_dragged: Option<Color> = 
            get_color(scroller_rgba_dragged, scroller_color_dragged, 1.0, false);

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgScrollableStyle(
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

        drop(state);
        Ok(id)

    }
    
    #[pyo3(signature = (
        window_id, 
        table_id, 
        polars_df,
        column_widths,
        height,
        parent_id=None,
        width=None,
        resizer_width=4.0,
        header_enabled=true,
        header_row_height=20.0,
        header_scrollbar_height=5.0,
        header_scrollbar_margin=0.0,
        header_scroller_height=5.0,
        header_scrollbar_spacing=0.0,
        header_row_spacing=0.0,
        footer_height=20.0,
        footer_scrollbar_height=5.0,
        footer_scrollbar_margin=0.0,
        footer_scroller_height=5.0,
        footer_scrollbar_spacing=0.0,
        footer_spacing=0.0,
        body_scrollbar_width=5.0,
        body_scrollbar_margin=0.0,
        body_scroller_width=5.0,
        body_scrollbar_spacing=0.0,
        body_row_highlight=true,
        custom_header_rows=0,
        custom_footer_rows=0,
        control_columns=vec![],
        column_proportional_resize=true,
        row_spacing=0.0,
        row_height=20.0,
        header_body_spacing=5.0,
        body_footer_spacing=5.0,
        resize_columns_enabled=true,
        min_column_width=0.0,
        text_size=14.0,
        table_width_fixed=true,
        gen_id=None,
        style_id=None,  
        show=true,
        on_column_resize=None,
        on_column_resize_release=None,
        user_data=None,
        ))]
    fn add_table(
        &self,
        window_id: String,
        table_id: String,
        polars_df: PyDataFrame,
        column_widths: Vec<f32>,
        height: f32,
        parent_id: Option<String>,
        width: Option<f32>,
        resizer_width: f32,
        header_enabled: bool,
        header_row_height: f32,
        header_scrollbar_height: f32,
        header_scrollbar_margin: f32,
        header_scroller_height: f32,
        header_scrollbar_spacing: f32,
        header_row_spacing: f32,
        footer_height: f32,
        footer_scrollbar_height: f32,
        footer_scrollbar_margin: f32,
        footer_scroller_height: f32,
        footer_scrollbar_spacing: f32,
        footer_spacing: f32,
        body_scrollbar_width: f32,
        body_scrollbar_margin: f32,
        body_scroller_width: f32,
        body_scrollbar_spacing: f32,
        body_row_highlight: bool,
        custom_header_rows: usize,
        custom_footer_rows: usize,
        control_columns: Vec<usize>,
        column_proportional_resize: bool,
        row_spacing: f32,
        row_height: f32,
        header_body_spacing: f32,
        body_footer_spacing: f32,
        resize_columns_enabled: bool,
        min_column_width: Option<f32>,
        text_size: f32,
        table_width_fixed: bool,
        gen_id: Option<usize>,
        style_id: Option<usize>,
        show: bool,
        on_column_resize: Option<PyObject>,
        on_column_resize_release: Option<PyObject>,
        user_data: Option<PyObject>,
    ) -> PyResult<usize> 
    {

        let id = self.get_id(gen_id);

        let prt_id = match parent_id {
            Some(id) => id,
            None => window_id.clone(),
        };

        let mut resize_offset = vec![];
        let mut table_width = 0.0;

        for width in column_widths.iter() { 
            resize_offset.push(Some(0.0));
            table_width += width;
        }

        let df: DataFrame = polars_df.into();
        
        if let Some(py) = user_data {
            add_user_data_to_mutex(id, py);
        }

        if let Some(py) = on_column_resize {
            add_callback_to_mutex(id, "dragging".to_string(), py);
        }

        let released = if let Some(py) = on_column_resize_release {
            add_callback_to_mutex(id, "released".to_string(), py);
            true
        } else {
            false
        };

        set_state_of_container(id, window_id.clone(), Some(table_id.clone()), prt_id);

        let mut state = access_state();

        set_state_cont_wnd_ids(&mut state, &window_id, table_id, id, "add_table".to_string());

        state.containers.insert(id, IpgContainers::IpgTable(
            IpgTable::new( 
                id,
                df,
                column_widths,
                height,
                width,
                resizer_width,
                header_enabled,
                header_row_height,
                header_scrollbar_height,
                header_scrollbar_margin,
                header_scroller_height,
                header_scrollbar_spacing,
                header_row_spacing,
                footer_height,
                footer_scrollbar_height,
                footer_scrollbar_margin,
                footer_scroller_height,
                footer_scrollbar_spacing,
                footer_spacing,
                body_scrollbar_width,
                body_scrollbar_margin,
                body_scroller_width,
                body_scrollbar_spacing,
                body_row_highlight,
                custom_header_rows,
                custom_footer_rows,
                control_columns,
                column_proportional_resize,
                row_spacing,
                row_height,
                header_body_spacing,
                body_footer_spacing,
                resize_columns_enabled,
                min_column_width,
                text_size,
                show,
                table_width_fixed,
                style_id,
                released,
                )));

        drop(state);
        Ok(id)

    }

    #[pyo3(signature = ( 
        header_background_color=None, 
        header_background_rgba=None,
        header_border_color=None, 
        header_border_rgba=None,
        header_border_radius = 0.0, 
        header_border_width=0.0,
        header_text_color=None, 
        header_text_rgba=None,

        body_background_color=None, 
        body_background_rgba=None,
        body_border_color=None, 
        body_border_rgba=None,
        body_border_radius = 0.0, 
        body_border_width=0.0,
        body_text_color=None, 
        body_text_rgba=None,
        body_row_highlight_color=None,
        body_row_highlight_rgba=None,

        footer_background_color=None, 
        footer_background_rgba=None,
        footer_border_color=None, 
        footer_border_rgba=None,
        footer_border_radius = 0.0, 
        footer_border_width=0.0,
        footer_text_color=None, 
        footer_text_rgba=None,

        divider_color=None,
        divider_rgba=None,
        divider_hover_color=None,
        divider_hover_rgba=None,

        scroller_rail_color=None,
        scroller_rail_rgba=None,
        scroller_color=None,
        scroller_rgba=None,
        scroller_hover_color=None,
        scroller_hover_rgba=None,

        gen_id=None
        ))]
    fn add_table_style(
        &self,
        header_background_color: Option<IpgColor>,
        header_background_rgba: Option<[f32; 4]>,
        header_border_color: Option<IpgColor>,
        header_border_rgba: Option<[f32; 4]>,
        header_border_radius: f32,
        header_border_width: f32,
        header_text_color: Option<IpgColor>,
        header_text_rgba: Option<[f32; 4]>,

        body_background_color: Option<IpgColor>,
        body_background_rgba: Option<[f32; 4]>,
        body_border_color: Option<IpgColor>,
        body_border_rgba: Option<[f32; 4]>,
        body_border_radius: f32,
        body_border_width: f32,
        body_text_color: Option<IpgColor>,
        body_text_rgba: Option<[f32; 4]>,
        body_row_highlight_color: Option<IpgColor>,
        body_row_highlight_rgba: Option<[f32; 4]>,

        footer_background_color: Option<IpgColor>,
        footer_background_rgba: Option<[f32; 4]>,
        footer_border_color: Option<IpgColor>,
        footer_border_rgba: Option<[f32; 4]>,
        footer_border_radius: f32,
        footer_border_width: f32,
        footer_text_color: Option<IpgColor>,
        footer_text_rgba: Option<[f32; 4]>,

        divider_color: Option<IpgColor>,
        divider_rgba: Option<[f32; 4]>,
        divider_hover_color: Option<IpgColor>,
        divider_hover_rgba: Option<[f32; 4]>,

        scroller_rail_color: Option<IpgColor>,
        scroller_rail_rgba: Option<[f32; 4]>,
        scroller_color: Option<IpgColor>,
        scroller_rgba: Option<[f32; 4]>,
        scroller_hover_color: Option<IpgColor>,
        scroller_hover_rgba: Option<[f32; 4]>,

        gen_id: Option<usize>,
        ) -> PyResult<usize>
    {
        let id = self.get_id(gen_id);

        let header_background =  
            get_color(header_background_rgba, header_background_color, 1.0, false);
        let header_border_color: Option<Color> = 
            get_color(header_border_rgba, header_border_color, 1.0, false);
        let header_text_color: Option<Color> = 
            get_color(header_text_rgba, header_text_color, 1.0, false);
        
        let body_background: Option<Color> = 
            get_color(body_background_rgba, body_background_color, 1.0, false);
        let body_border_color: Option<Color> = 
            get_color(body_border_rgba, body_border_color, 1.0, false);
        let body_text_color: Option<Color> = 
            get_color(body_text_rgba, body_text_color, 1.0, false);
        let body_row_highlight: Option<Color> = 
            get_color(body_row_highlight_rgba, body_row_highlight_color, 1.0, false);

        let footer_background: Option<Color> = 
            get_color(footer_background_rgba, footer_background_color, 1.0, false);
        let footer_border_color: Option<Color> = 
            get_color(footer_border_rgba, footer_border_color, 1.0, false);
        let footer_text_color: Option<Color> = 
            get_color(footer_text_rgba, footer_text_color, 1.0, false);

        let divider_background: Option<Color> = 
            get_color(divider_rgba, divider_color, 1.0, false);
        let divider_hover_color = 
            get_color(divider_hover_rgba, divider_hover_color, 1.0, false);

        let rail = get_color(scroller_rail_rgba, scroller_rail_color, 1.0, false);
        let scroller = get_color(scroller_rgba, scroller_color, 1.0, false);
        let scroller_hover = get_color(scroller_hover_rgba, scroller_hover_color, 1.0, false);
      
        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgTableStyle(
            IpgTableStyle::new( 
                id,
                header_background,
                header_border_color,
                header_border_radius,
                header_border_width,
                header_text_color,
                
                body_background,
                body_border_color,
                body_border_radius,
                body_border_width,
                body_text_color,
                body_row_highlight,
                
                footer_background,
                footer_border_color,
                footer_border_radius,
                footer_border_width,
                footer_text_color,
                
                divider_background,
                divider_hover_color,

                rail,
                scroller,
                scroller_hover,
                )));

        drop(state);
        Ok(id)
    }

    #[pyo3(signature = (
        window_id, 
        container_id, 
        position, 
        text_to_display, 
        parent_id=None, 
        gap=10, 
        padding=0.0, 
        snap_within_viewport=true, 
        style="box".to_string()
        ))]
    fn add_tool_tip(
        &self,
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

        let id = self.get_id(None);

        let prt_id = match parent_id {
            Some(id) => id,
            None => window_id.clone(),
        };

        set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

        let mut state = access_state();

        set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_tool_tip".to_string());

        state.containers.insert(id, IpgContainers::IpgToolTip(
            IpgToolTip::new( 
                id,
                position,
                text_to_display,
                gap,
                padding,
                snap_within_viewport,
                style,
                )));
        drop(state);
        Ok(id)

    }
    
    #[pyo3(signature = (
        parent_id, 
        label, 
        gen_id=None, 
        on_press=None, 
        width=None, 
        height=None, 
        width_fill=false, 
        height_fill=false, 
        padding=vec![5.0], 
        text_align_x=IpgHorizontalAlignment::Center, 
        text_align_y=IpgVerticalAlignment::Center,
        text_size=16.0, 
        clip=false, 
        style_id=None, 
        style_standard=None, 
        style_arrow=None, 
        user_data=None, 
        show=true, 
        ))]
    fn add_button(
        &self,
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
        text_align_x: IpgHorizontalAlignment,
        text_align_y: IpgVerticalAlignment,
        text_size: f32,
        clip: bool,
        style_id: Option<usize>,
        style_standard: Option<IpgStyleStandard>,
        style_arrow: Option<IpgButtonArrow>,
        user_data: Option<PyObject>,
        show: bool,
        ) -> PyResult<usize> 
    {
        let id = self.get_id(gen_id);

        let width = get_width(width, width_fill);
        let height = get_height(height, height_fill);

        let padding = get_padding_f64(padding);

        let align_x = get_horizontal_alignment(&text_align_x);
        let align_y = get_vertical_alignment(&text_align_y);

        set_state_of_widget(id, parent_id.clone());

        if let Some(py) = on_press {
            add_callback_to_mutex(id, "on_press".to_string(), py);
        }

        if let Some(py) = user_data{
            add_user_data_to_mutex(id, py);
        }

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgButton(
            IpgButton::new(
                id,
                parent_id,
                show,
                label,
                width,
                height,
                padding,
                align_x,
                align_y,
                text_size,
                clip,
                style_id,
                style_standard,
                style_arrow,                              
                )));

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
        border_radius=vec![0.0], 
        border_width=1.0,
        shadow_color=None, 
        shadow_rgba=None,
        shadow_offset_x=0.0, 
        shadow_offset_y=0.0,
        shadow_blur_radius=1.0,
        text_color=None, 
        text_rgba=None,
        gen_id=None
        ))]
    fn add_button_style(
        &self,
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

        let background_color: Option<Color> = 
            get_color(background_rgba, background_color, 1.0, false);
        let background_color_hovered: Option<Color> = 
            get_color(background_rgba_hovered, background_color_hovered, 1.0, false);
        let border_color: Option<Color> = 
            get_color(border_rgba, border_color, 1.0, false);
        let shadow_color: Option<Color> = 
            get_color(shadow_rgba, shadow_color, 1.0, false);
        let text_color: Option<Color> = 
            get_color(text_rgba, text_color, 1.0, false);

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

        drop(state);
        Ok(id)
    }

    #[pyo3(signature = (
        parent_id, 
        head, 
        body,      
        is_open=true, 
        min_max_id=None, 
        foot=None, 
        gen_id=None, 
        close_size=15.0, 
        on_close=None, 
        width=None, 
        width_fill=false, 
        height=None, 
        height_fill=false, 
        max_width=f32::INFINITY, 
        max_height=f32::INFINITY, 
        padding_head=vec![5.0], 
        padding_body=vec![5.0], 
        padding_foot=vec![5.0],
        style_id=None, 
        show=true, 
        user_data=None
        ))]
    fn add_card(
        &self,
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
        width_fill: bool,
        height: Option<f32>,
        height_fill: bool,
        max_width: f32,
        max_height: f32,
        padding_head: Vec<f64>,
        padding_body: Vec<f64>,
        padding_foot: Vec<f64>,
        style_id: Option<usize>,
        show: bool,
        user_data: Option<PyObject>, 
        ) -> PyResult<usize> 
    {
        let id = self.get_id(gen_id);

        if let Some(py) = on_close {
            add_callback_to_mutex(id, "on_close".to_string(), py);
        }

        if let Some(py) = user_data {
            add_user_data_to_mutex(id, py);
        }

        let width = get_width(width, width_fill);
        let height = get_height(height, height_fill);

        let padding_head = get_padding_f64(padding_head);
        let padding_body = get_padding_f64(padding_body);
        let padding_foot = get_padding_f64(padding_foot);

        set_state_of_widget(id, parent_id.clone());

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgCard(
            IpgCard::new(
                id,
                parent_id,
                is_open,
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
                style_id,
                show,
                )));

        drop(state);
        Ok(id)

    }

    #[pyo3(signature = ( 
        background_color=None, 
        background_rgba=None,
        border_radius=10.0, 
        border_width=1.0,
        border_color=None,
        border_rgba=None, 
        head_background_color=None,
        head_background_rgba=None, 
        head_text_color=None,
        head_text_rgba=None,
        body_background_color=None,
        body_background_rgba=None, 
        body_text_color=None,
        body_text_rgba=None, 
        foot_background_color=None,
        foot_background_rgba=None, 
        foot_text_color=None,
        foot_text_rgba=None, 
        close_color=None,
        close_rgba=None,
        gen_id=None
        ))]
    fn add_card_style(
        &self,
        background_color: Option<IpgColor>, 
        background_rgba: Option<[f32; 4]>,
        border_radius: f32, 
        border_width: f32, 
        border_color: Option<IpgColor>,
        border_rgba: Option<[f32; 4]>, 
        head_background_color: Option<IpgColor>,
        head_background_rgba: Option<[f32; 4]>, 
        head_text_color: Option<IpgColor>,
        head_text_rgba: Option<[f32; 4]>,
        body_background_color: Option<IpgColor>,
        body_background_rgba: Option<[f32; 4]>, 
        body_text_color: Option<IpgColor>,
        body_text_rgba: Option<[f32; 4]>, 
        foot_background_color: Option<IpgColor>,
        foot_background_rgba: Option<[f32; 4]>, 
        foot_text_color: Option<IpgColor>,
        foot_text_rgba: Option<[f32; 4]>, 
        close_color:Option<IpgColor>,
        close_rgba:Option<[f32; 4]>,
        gen_id: Option<usize>,
        ) -> PyResult<usize>
    {
        let id = self.get_id(gen_id);

        let background: Option<Color> = 
            get_color(background_rgba, background_color, 1.0, false);
        let border_color: Option<Color> = 
            get_color(border_rgba, border_color, 1.0, false);
        let head_background: Option<Color> = 
            get_color(head_background_rgba, head_background_color, 1.0, false);
        let body_background: Option<Color> = 
            get_color(body_background_rgba, body_background_color, 1.0, false);
        let foot_background: Option<Color> = 
            get_color(foot_background_rgba, foot_background_color, 1.0, false);
        let head_text_color: Option<Color> = 
            get_color(head_text_rgba, head_text_color, 1.0, false);
        let body_text_color: Option<Color> = 
            get_color(body_text_rgba, body_text_color, 1.0, false);
        let foot_text_color: Option<Color> = 
            get_color(foot_text_rgba, foot_text_color, 1.0, false);
        let close_color: Option<Color> = 
            get_color(close_rgba, close_color, 1.0, false);

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgCardStyle(
            IpgCardStyle::new( 
                id,
                background,
                border_radius,
                border_width,
                border_color,
                head_background, 
                head_text_color, 
                body_background, 
                body_text_color, 
                foot_background, 
                foot_text_color, 
                close_color,
                )));

 
        drop(state);
        Ok(id)
    }

    #[pyo3(signature = (
        parent_id, 
        gen_id=None, 
        on_toggle=None, 
        is_checked=false, 
        label="".to_string(), 
        width=None, 
        width_fill=false, 
        size=16.0, 
        spacing=10.0, 
        text_line_height=1.3, 
        text_shaping="basic".to_string(),
        text_size=16.0, 
        icon_x=false, 
        icon_size=25.0, 
        user_data=None, 
        show=true, 
        style_id=None, 
        style_standard=None, 
        ))] 
    fn add_checkbox(
        &self,
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
        
        if let Some(py) = on_toggle {
            add_callback_to_mutex(id, "on_toggle".to_string(), py);
        }

        if let Some(py) = user_data {
            add_user_data_to_mutex(id, py);
        }
       
        let text_shaping = get_shaping(text_shaping);

        let text_line_height = text::LineHeight::Relative(text_line_height);

        let width = get_width(width, width_fill);
        
        set_state_of_widget(id, parent_id.clone());

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgCheckBox(
            IpgCheckBox::new(
                id,
                parent_id,
                show,
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
        gen_id=None
        ))]
    fn add_checkbox_style(
        &self,
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

        let background_color: Option<Color> = 
            get_color(background_rgba, background_color, 1.0, false);
        let background_color_hovered: Option<Color> = 
            get_color(background_rgba_hovered, background_color_hovered, 1.0, false);
        let accent_color: Option<Color> = 
            get_color(accent_rgba, accent_color, 1.0, false);
        let accent_color_hovered: Option<Color> = 
            get_color(accent_rgba_hovered, accent_color_hovered, 1.0, false);
        let border_color: Option<Color> = 
            get_color(border_rgba, border_color, 1.0, false);
        let icon_color: Option<Color> = 
            get_color(icon_rgba, icon_color, 1.0, false);
        let text_color: Option<Color> = 
            get_color(text_rgba, text_color, 1.0, false);

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

        drop(state);
        Ok(id)
    }

    #[pyo3(signature = (
        parent_id, 
        label="Set Color".to_string(), 
        gen_id=None, 
        on_press=None, 
        on_submit=None, 
        on_cancel=None, 
        color_rgba=[0.5, 0.2, 0.7, 1.0], 
        width=None, 
        height=None, 
        width_fill=false, 
        height_fill=false, 
        padding=vec![5.0], 
        clip=false, 
        style_id=None, 
        style_standard=None, 
        style_arrow=None,
        user_data=None,
        show=false, 
        ))]
    fn add_color_picker(
        &self,
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

        if let Some(py) = on_press {
            add_callback_to_mutex(id, "on_press".to_string(), py);
        }

        if let Some(py) = on_submit {
            add_callback_to_mutex(id, "on_submit".to_string(), py);
        }

        if let Some(py) = on_cancel {
            add_callback_to_mutex(id, "on_cancel".to_string(), py);
        }

        if let Some(py) = user_data {
            add_user_data_to_mutex(id, py);
        }

        let color = Color::from(color_rgba);

        let width = get_width(width, width_fill);
        let height = get_height(height, height_fill);

        let padding = get_padding_f64(padding);

        set_state_of_widget(id, parent_id.clone());

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgColorPicker(
            IpgColorPicker::new(
                id,
                parent_id,
                show,
                color,
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
        border_radius = vec![0.0], 
        border_width=1.0,
        shadow_color=None, 
        shadow_rgba=None,
        shadow_offset_x=0.0, 
        shadow_offset_y=0.0,
        shadow_blur_radius=1.0,
        text_color=None, 
        text_rgba=None,
        gen_id=None
        ))]
    fn add_color_picker_style(
        &self,
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

        let background_color: Option<Color> = 
            get_color(background_rgba, background_color, 1.0, false);
        let background_color_hovered: Option<Color> = 
            get_color(background_rgba_hovered, background_color_hovered, 1.0, false);
        let border_color: Option<Color> = 
            get_color(border_rgba, border_color, 1.0, false);
        let shadow_color: Option<Color> = 
            get_color(shadow_rgba, shadow_color, 1.0, false);
        let text_color: Option<Color> = 
            get_color(text_rgba, text_color, 1.0, false);

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

        drop(state);
        Ok(id)

    }

    #[pyo3(signature = (
        parent_id, 
        label="Calendar".to_string(), 
        gen_id=None,
        size_factor=1.0, 
        padding=vec![0.0], 
        on_submit=None, 
        user_data=None,
        show=true,
        show_calendar=false, 
        button_style_standard=None,
        button_style_id=None,
        ))]
    fn add_date_picker(
        &self,
        parent_id: String,
        // ** above required
        label: String,
        gen_id: Option<usize>,
        size_factor: f32,
        padding: Vec<f64>,
        on_submit: Option<PyObject>,
        user_data: Option<PyObject>,
        show: bool,
        show_calendar: bool,
        button_style_standard: Option<IpgStyleStandard>,
        button_style_id: Option<usize>,
        ) -> PyResult<usize> 
    {
        let id = self.get_id(gen_id);

        if size_factor < 1.0 {
            panic!("Size factor for date picker must be > 1.0")
        }

        if let Some(py) = on_submit {
            add_callback_to_mutex(id, "on_submit".to_string(), py);
        }

        if let Some(py) = user_data {
            add_user_data_to_mutex(id, py);
        }

        let padding = get_padding_f64(padding);

        set_state_of_widget(id, parent_id.clone());

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgDatePicker(
            IpgDatePicker::new(
                id,
                parent_id,
                label,
                size_factor,
                padding,
                show,
                show_calendar,
                button_style_standard,
                button_style_id,
                )));

        drop(state);
        Ok(id)

    }

    #[pyo3(signature = (
        parent_id,
        widths,
        handle_width,
        handle_height,
        handle_offsets=None,
        include_last_handle=true,
        on_change=None,
        on_release=None,
        width=None,
        width_fill=true,
        height=None,
        height_fill=true,
        style_id=None,
        gen_id=None,
        user_data=None,
        show=true,
        ))]
    fn add_divider_horizontal(
        &self,
        parent_id: String,
        widths: Vec<f32>,
        handle_width: f32,
        handle_height: f32,
        handle_offsets: Option<Vec<f32>>,
        include_last_handle: bool,
        on_change: Option<PyObject>,
        on_release: Option<PyObject>,
        width: Option<f32>,
        width_fill: bool,
        height: Option<f32>,
        height_fill: bool,
        style_id: Option<usize>,
        gen_id: Option<usize>,
        user_data: Option<PyObject>,
        show: bool,
    ) -> PyResult<usize>
    {
        let id = self.get_id(gen_id);

        if let Some(py) = on_change {
            add_callback_to_mutex(id, "on_change".to_string(), py);
        }

        if let Some(py) = on_release {
            add_callback_to_mutex(id, "on_release".to_string(), py);
        }

        if let Some(py) = user_data {
            add_user_data_to_mutex(id, py);
        }

        let width = get_width(width, width_fill);

        let height = get_height(height, height_fill);

        set_state_of_widget(id, parent_id.clone());

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgDividerHorizontal(
            IpgDividerHorizontal::new(
                id,
                parent_id,
                show,
                widths,
                handle_width,
                handle_height,
                handle_offsets,
                include_last_handle,
                width,
                height,
                style_id,
                )));

        drop(state);
        Ok(id)
    }

    #[pyo3(signature = (
        parent_id,
        heights,
        handle_width,
        handle_height,
        handle_offsets=None,
        include_last_handle=true,
        on_change=None,
        on_release=None,
        width=None,
        width_fill=true,
        height=None,
        height_fill=true,
        style_id=None,
        gen_id=None,
        user_data=None,
        show=true,
        ))]
    fn add_divider_vertical(
        &self,
        parent_id: String,
        heights: Vec<f32>,
        handle_width: f32,
        handle_height: f32,
        handle_offsets: Option<Vec<f32>>,
        include_last_handle: bool,
        on_change: Option<PyObject>,
        on_release: Option<PyObject>,
        width: Option<f32>,
        width_fill: bool,
        height: Option<f32>,
        height_fill: bool,
        style_id: Option<usize>,
        gen_id: Option<usize>,
        user_data: Option<PyObject>,
        show: bool,
    ) -> PyResult<usize>
    {
        let id = self.get_id(gen_id);

        if let Some(py) = on_change {
            add_callback_to_mutex(id, "on_change".to_string(), py);
        }

        if let Some(py) = on_release {
            add_callback_to_mutex(id, "on_release".to_string(), py);
        }

        if let Some(py) = user_data {
            add_user_data_to_mutex(id, py);
        }

        let width = get_width(width, width_fill);

        let height = get_height(height, height_fill);

        set_state_of_widget(id, parent_id.clone());

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgDividerVertical(
            IpgDividerVertical::new(
                id,
                parent_id,
                show,
                heights,
                handle_width,
                handle_height,
                handle_offsets,
                include_last_handle,
                width,
                height,
                style_id,
                )));

        drop(state);
        Ok(id)
    }

    #[pyo3(signature = (
        parent_id, 
        image_path, 
        gen_id=None, 
        width=None, 
        width_fill=false, 
        height=None, 
        height_fill=false, 
        padding=vec![5.0], 
        content_fit=IpgImageContentFit::Contain, 
        filter_method=IpgImageFilterMethod::Linear,
        rotation=IpgImageRotation::Floating,
        rotation_radians=0.0, 
        opacity=1.0,
        mouse_pointer=None,
        on_press=None, 
        on_release=None,
        on_right_press=None, 
        on_right_release=None,
        on_middle_press=None, 
        on_middle_release=None,
        on_enter=None, 
        on_move=None, 
        on_exit=None,
        user_data=None,
        show=true,
        ))]
    fn add_image(
        &self,
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

        if let Some(py) = on_press {
            add_callback_to_mutex(id, "on_press".to_string(), py);
        }
        
        if let Some(py) = on_release {
            add_callback_to_mutex(id, "event_name".to_string(), py);
        }
        
        if let Some(py) = on_right_press {
            add_callback_to_mutex(id, "on_right_press".to_string(), py);
        }
        
        if let Some(py) = on_right_release {
            add_callback_to_mutex(id, "on_right_release".to_string(), py);
        }
        
        if let Some(py) = on_middle_press {
            add_callback_to_mutex(id, "on_middle_press".to_string(), py);
        }
        
        if let Some(py) = on_middle_release {
            add_callback_to_mutex(id, "on_middle_release".to_string(), py);
        }
        
        if let Some(py) = on_enter {
            add_callback_to_mutex(id, "on_enter".to_string(), py);
        }
        
        if let Some(py) = on_move {
            add_callback_to_mutex(id, "on_move".to_string(), py);
        }
        
        if let Some(py) = on_exit {
            add_callback_to_mutex(id, "on_exit".to_string(), py);
        }

        if let Some(py) = user_data {
            add_user_data_to_mutex(id, py);
        }
        
        let width = get_width(width, width_fill);
        let height = get_height(height, height_fill);

        let padding = get_padding_f64(padding);

        set_state_of_widget(id, parent_id.clone());

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgImage(
            IpgImage::new(
                id,
                parent_id,
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
            )));

        drop(state);
        Ok(id)

    }
    
    #[pyo3(signature = (
        background_color=None,
        background_rgba=None,
        background_color_hovered=None,
        background_rgba_hovered=None,
        background_transparent=false,
        border_color=None,
        border_rgba=None,
        border_width=0.0,
        border_radius=0.0,
        gen_id=None
        ))]
    fn add_divider_style(
        &self,
        background_color: Option<IpgColor>,
        background_rgba: Option<[f32; 4]>,
        background_color_hovered: Option<IpgColor>,
        background_rgba_hovered: Option<[f32; 4]>,
        background_transparent: bool,
        border_color: Option<IpgColor>,
        border_rgba: Option<[f32; 4]>,
        border_width: f32,
        border_radius: f32,
        gen_id: Option<usize>,
        ) -> PyResult<usize>
    {
        let id = self.get_id(gen_id);
        
        let background: Option<Color> = 
            get_color(background_rgba, background_color, 1.0, false);
        let background_hovered: Option<Color> = 
            get_color(background_rgba_hovered, background_color_hovered, 1.0, false);
        let border_color: Option<Color> = 
            get_color(border_rgba, border_color, 1.0, false);

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgDividerStyle(
            IpgDividerStyle::new( 
                id,
                background,
                background_hovered,
                background_transparent,
                border_color,
                border_width,
                border_radius,
                )));

        drop(state);
        Ok(id)

    }

    #[pyo3(signature = (
        parent_id, 
        options, 
        gen_id=None, 
        on_select=None, 
        width=None, 
        width_fill=false, 
        padding=vec![5.0],  
        placeholder=None, 
        selected=None, 
        text_size=None, 
        text_line_height=1.2, 
        text_shaping="basic".to_string(), 
        handle=IpgPickListHandle::Default, 
        arrow_size=None, 
        dynamic_closed=None, 
        dynamic_opened=None, 
        custom_static=None,
        style_id=None, 
        user_data=None, 
        show=true,
        ))]
    fn add_pick_list(
        &self,
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

        if let Some(py) = on_select {
            add_callback_to_mutex(id, "on_select".to_string(), py);
        }

        if let Some(py) = user_data {
            add_user_data_to_mutex(id, py);
        }

        let padding = get_padding_f64(padding);

        let text_line_height = text::LineHeight::Relative(text_line_height);
        
        let text_shaping = get_shaping(text_shaping);

        let width = get_width(width, width_fill);

         let options =  convert_pyobject_vec_string(options);

        set_state_of_widget(id, parent_id.clone());

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgPickList(
            IpgPickList::new(  
                id,
                parent_id,
                show,
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
        gen_id=None
        ))]
    fn add_pick_list_style(
        &self,
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
        
        let background_color: Option<Color> = 
            get_color(background_rgba, background_color, 1.0, false);
        let border_color: Option<Color> = 
            get_color(border_rgba, border_color, 1.0, false);
        let border_color_hovered: Option<Color> = 
            get_color(border_rgba_hovered, border_color_hovered, 1.0, false);
        let handle_color: Option<Color> = 
            get_color(handle_rgba, handle_color, 1.0, false);
        let placeholder_color = 
            get_color(placeholder_rgba, placeholder_color, 1.0, false);
        let text_color = 
            get_color(text_rgba, text_color, 1.0, false);

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

        drop(state);
        Ok(id)

    }

    #[pyo3(signature = (
        parent_id, 
        min, 
        max, 
        value,
        gen_id=None, 
        width=None, 
        height=Some(16.0), 
        width_fill=true, 
        height_fill=false,
        style_standard=None, 
        style_id=None, 
        show=true, 
        ))]
    fn add_progress_bar(
        &self,
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

        set_state_of_widget(id, parent_id.clone());

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgProgressBar(
            IpgProgressBar::new(   
                id,
                parent_id,
                show,
                min,
                max,
                value,
                width,
                height,
                style_standard,
                style_id,
                )));

        drop(state);
        Ok(id)

    }

    #[pyo3(signature = (
        background_color=None, 
        background_rgba=None,
        bar_color=None, 
        bar_rgba=None,
        border_color=None, 
        border_rgba=None,
        border_radius=None, 
        border_width=None,
        gen_id=None
        ))]
    fn add_progress_bar_style(
        &self,
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

        drop(state);
        Ok(id)
    }

    #[pyo3(signature = (
        parent_id, 
        labels, 
        gen_id=None,
        direction=IpgRadioDirection::Vertical, 
        spacing= 10.0, 
        padding=vec![10.0], 
        width=None, 
        width_fill=false, 
        height=None, 
        height_fill=false,
        on_select=None, 
        selected_index=None, 
        size=20.0, 
        style_id=None,
        text_spacing=15.0, 
        text_size=16.0,
        text_line_height_pixels=None,
        text_line_height_relative=None, 
        text_shaping="basic".to_string(), 
        user_data=None, 
        show=true, 
        ))]
    fn add_radio(
        &mut self,
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

        if let Some(py) = on_select {
            add_callback_to_mutex(id, "on_select".to_string(), py);
        }

        if let Some(py) = user_data {
            add_user_data_to_mutex(id, py);
        }

        let text_line_height = get_line_height(text_line_height_pixels, text_line_height_relative);
        
        let text_shaping = get_shaping(text_shaping);

        let width = get_width(width, width_fill);
        let height = get_height(height, height_fill);

        set_state_of_widget(id, parent_id.clone());

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgRadio(
            IpgRadio::new( 
                id,
                parent_id,
                labels,
                direction,
                spacing,
                padding,
                show,
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
        gen_id=None
        ))]
    fn add_radio_style(
        &self,
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

        let background_color = 
            get_color(background_rgba, background_color, 1.0, false);
        let background_color_hovered = 
            get_color(background_rgba_hovered, background_color_hovered, 1.0, false);
        let dot_color: Option<Color> = 
            get_color(dot_rgba, dot_color, 1.0, false);
        let dot_color_hovered: Option<Color> = 
            get_color(dot_rgba_hovered, dot_color_hovered, 1.0, false);
        let border_color: Option<Color> = 
            get_color(border_rgba, border_color, 1.0, false);
        let text_color: Option<Color> = 
            get_color(text_rgba, text_color, 1.0, false);

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

        drop(state);
        Ok(id)

    }

    #[pyo3(signature = (
        parent_id, width, 
        width_fill=true, 
        thickness=1,
        style_id=None,
        gen_id=None,
        show=true,
        ))]
    fn add_rule_horizontal(
        &self, 
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

        set_state_of_widget(id, parent_id.clone());

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgRule(
            IpgRule::new(
                id,
                parent_id,
                width,
                height,
                thickness,
                rule_type,
                style_id,
                show,
                )));
 
        drop(state);
        Ok(id)

    }

    #[pyo3(signature = (
        parent_id, 
        height=None, 
        height_fill=true, 
        thickness=1,
        style_id=None, 
        gen_id=None,
        show=true
        ))]
    fn add_rule_vertical(
        &self, 
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

        set_state_of_widget(id, parent_id.clone());

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgRule(
            IpgRule::new(
                id,
                parent_id,
                width,
                height,
                thickness,
                rule_type,
                style_id,
                show, 
                )));
 
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
        gen_id=None
        ))]
    fn add_rule_style(
        &self,
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

        drop(state);
        Ok(id)

    }

    #[pyo3(signature = (
        parent_id, 
        text, 
        gen_id=None, 
        on_press=None, 
        on_release=None, 
        on_right_press=None, 
        on_right_release=None, 
        on_middle_press=None, 
        on_middle_release=None, 
        on_move=None, 
        on_enter=None, 
        on_exit=None, 
        width=None, 
        width_fill=false,
        height=None, 
        height_fill=false, 
        h_align=IpgHorizontalAlignment::Left, 
        v_align=IpgVerticalAlignment::Top, 
        line_height=1.3, 
        size=16.0,
        text_color=None, 
        text_rgba=None, 
        show=true, 
        shaping="basic".to_string(), 
        user_data=None,
        ))]
    fn add_selectable_text(
        &self,
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

        if let Some(py) = on_press {
        add_callback_to_mutex(id, "on_press".to_string(), py);
        }
        
        if let Some(py) = on_release {
            add_callback_to_mutex(id, "event_name".to_string(), py);
        }
        
        if let Some(py) = on_right_press {
            add_callback_to_mutex(id, "on_right_press".to_string(), py);
        }
        
        if let Some(py) = on_right_release {
            add_callback_to_mutex(id, "on_right_release".to_string(), py);
        }
        
        if let Some(py) = on_middle_press {
            add_callback_to_mutex(id, "on_middle_press".to_string(), py);
        }
        
        if let Some(py) = on_middle_release {
            add_callback_to_mutex(id, "on_middle_release".to_string(), py);
        }
        
        if let Some(py) = on_enter {
            add_callback_to_mutex(id, "on_enter".to_string(), py);
        }
        
        if let Some(py) = on_move {
            add_callback_to_mutex(id, "on_move".to_string(), py);
        }
        
        if let Some(py) = on_exit {
            add_callback_to_mutex(id, "on_exit".to_string(), py);
        }

        if let Some(py) = user_data {
            add_user_data_to_mutex(id, py);
        }
        
        let width = get_width(width, width_fill);
        let height = get_height(height, height_fill);

        let line_height = LineHeight::Relative(line_height);

        let shaping = get_shaping(shaping);

        let text_color = get_color(text_rgba, text_color, 1.0, false);

        set_state_of_widget(id, parent_id.clone());

        let mut state = access_state();
        
        state.widgets.insert(id, IpgWidgets::IpgSelectableText(
            IpgSelectableText::new(
                id,
                parent_id,
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
                )));

        drop(state);
        Ok(id)

    }

    #[pyo3(signature = (
        parent_id, 
        separator_type=IpgSeparatorType::Line,
        label=None, label_left_width=20.0,
        label_right_width=20.0,
        dot_radius=4.0, dot_count=1,
        dot_fill=true, dot_border_width=0.0,
        width=None, width_fill=false, 
        height=None, height_fill=false,
        spacing=0.0, style_id=None,
        gen_id=None, show=true
        ))]
    fn add_separator(
        &self,
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

        set_state_of_widget(id, parent_id.clone());

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgSeparator(
            IpgSeparator::new( 
                id,
                parent_id,
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
    fn add_separator_style(
        &self,
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

        drop(state);
        Ok(id)
    }

    #[pyo3(signature = (
        parent_id, 
        min, 
        max, 
        step, 
        value, 
        gen_id=None, 
        width=None, 
        height=None, 
        width_fill=false, 
        on_change=None, 
        on_release=None, 
        style_id=None,
        user_data=None,
        show=true, 
        ))]
    fn add_slider(
        &self,
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

        if let Some(py) = on_change {
            add_callback_to_mutex(id, "on_change".to_string(), py);
        }
        if let Some(py) = on_release {
            add_callback_to_mutex(id, "on_release".to_string(), py);
        }

        if let Some(py) = user_data {
            add_user_data_to_mutex(id, py);
        }
        
        let width = get_width(width, width_fill);
        let height = height.unwrap_or(16.0);

        set_state_of_widget(id, parent_id.clone());

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgSlider(
            IpgSlider::new( 
                id,
                parent_id,
                show,
                min,
                max,
                step,
                value,
                width,
                height,
                style_id,
                )));

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
    fn add_slider_style(
        &self,
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

        let rail_color = 
            get_color(rail_rgba, rail_color, 1.0, false);
        let rail_color_hovered = 
            get_color(rail_rgba_hovered, rail_color_hovered, 1.0, false);
        let handle_color = 
            get_color(handle_rgba, handle_color, 1.0, false);
        let handle_border_color = 
            get_color(handle_border_rgba,handle_border_color,1.0, false);

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

        drop(state);
        Ok(id)

    }

    #[pyo3(signature = (
        parent_id, 
        gen_id=None, 
        width=None, 
        height=None, 
        width_fill=false, 
        height_fill=false, 
        show=true
        ))]
    fn add_space(
        &self,
        parent_id: String,
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
        let height = get_height(height, height_fill);

        set_state_of_widget(id, parent_id.clone());

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgSpace(
            IpgSpace::new( 
                id,
                parent_id,
                width,
                height,
                show,
                )));
 
        drop(state);
        Ok(id)

    }

    #[pyo3(signature = (
        parent_id, 
        svg_path, 
        gen_id=None, 
        width=None, 
        width_fill=false, 
        height=None, 
        height_fill=false,
        content_fit=IpgSvgContentFit::Contain,
        rotation=IpgSvgRotation::Floating,
        rotation_radians=0.0, 
        opacity=1.0,
        mouse_pointer=None, 
        show=true,
        on_press=None, 
        on_release=None,
        on_right_press=None, 
        on_right_release=None,
        on_middle_press=None, 
        on_middle_release=None,
        on_enter=None, 
        on_move=None, 
        on_exit=None, 
        user_data=None,
        ))]
    fn add_svg(
        &self,
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
        show: bool,
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
        ) -> PyResult<usize>
    {
        let id = self.get_id(gen_id);

        if let Some(py) = on_press {
            add_callback_to_mutex(id, "on_press".to_string(), py);
        }
        
        if let Some(py) = on_release {
            add_callback_to_mutex(id, "event_name".to_string(), py);
        }
        
        if let Some(py) = on_right_press {
            add_callback_to_mutex(id, "on_right_press".to_string(), py);
        }
        
        if let Some(py) = on_right_release {
            add_callback_to_mutex(id, "on_right_release".to_string(), py);
        }
        
        if let Some(py) = on_middle_press {
            add_callback_to_mutex(id, "on_middle_press".to_string(), py);
        }
        
        if let Some(py) = on_middle_release {
            add_callback_to_mutex(id, "on_middle_release".to_string(), py);
        }
        
        if let Some(py) = on_enter {
            add_callback_to_mutex(id, "on_enter".to_string(), py);
        }
        
        if let Some(py) = on_move {
            add_callback_to_mutex(id, "on_move".to_string(), py);
        }
        
        if let Some(py) = on_exit {
            add_callback_to_mutex(id, "on_exit".to_string(), py);
        }

        if let Some(py) = user_data {
            add_user_data_to_mutex(id, py);
        }
        
        let width = get_width(width, width_fill);
        let height = get_height(height, height_fill);

        set_state_of_widget(id, parent_id.clone());

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgSvg(
            IpgSvg::new(
                id,
                parent_id,
                svg_path,
                width,
                height,
                content_fit,
                rotation,
                rotation_radians,
                opacity,
                mouse_pointer,
                show,
                )));
 
        drop(state);
        Ok(id)

    }

    #[pyo3(signature = (
        parent_id, 
        content, 
        gen_id=None, 
        width=None, 
        width_fill=false, 
        height=None, 
        height_fill=false,
        centered=true,
        align_x=IpgHorizontalAlignment::Left, 
        align_y=IpgVerticalAlignment::Top,
        line_height=1.3, 
        size=16.0, 
        shaping="basic".to_string(), 
        text_color=None, 
        text_rgba=None,
        show=true,
        ))]
    fn add_text(
        &self,
        parent_id: String,
        content: String,
        // ** above required
        gen_id: Option<usize>,
        width: Option<f32>,
        width_fill: bool,
        height: Option<f32>,
        height_fill: bool,
        centered: bool,
        mut align_x: IpgHorizontalAlignment,
        mut align_y: IpgVerticalAlignment,
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

        if centered {
            align_x = IpgHorizontalAlignment::Center;
            align_y = IpgVerticalAlignment::Center;
        }

        let line_height = LineHeight::Relative(line_height);

        let shaping = get_shaping(shaping);

        let style = get_color(text_rgba, text_color, 1.0, false);

        set_state_of_widget(id, parent_id.clone());

        let mut state = access_state();
        
        state.widgets.insert(id, IpgWidgets::IpgText(
            IpgText::new(
                id,
                parent_id,
                content,
                size,
                line_height,
                width,
                height,
                align_x,
                align_y,
                // font: Font,
                shaping,
                show,
                style,
            )));

        drop(state);
        Ok(id)

    }

    // #[pyo3(signature = (
    //     parent_id,
    //     size=None,
    //     line_height=None,
    //     text_color=None,
    //     text_color_rgba=None,
    //     highlight_bkg_color=None,
    //     highlight_bkg_rgba=None,
    //     highlight_border_color=None,
    //     highlight_border_rgba=None,
    //     highlight_border_radius=None,
    //     highlight_border_width=None,
    //     padding=None,
    //     underline=false,
    //     strikethrough=false,
    //     style_id=None,
    //     show=true,
    //     gen_id=None,
    //     ))]
    // fn add_rich_text(
    //     &self,
    //     parent_id: String,
    //     size: Option<f32>,
    //     line_height: Option<f32>,
    //     text_color: Option<IpgColor>,
    //     text_color_rgba: Option<[f32; 4]>,
    //     highlight_bkg_color: Option<IpgColor>,
    //     highlight_bkg_rgba: Option<[f32; 4]>,
    //     highlight_border_color: Option<IpgColor>,
    //     highlight_border_rgba: Option<[f32; 4]>,
    //     highlight_border_radius: Option<f32>,
    //     highlight_border_width: Option<f32>,
    //     padding: Option<Vec<f32>>,
    //     underline: bool,
    //     strikethrough: bool,
    //     style_id: Option<usize>,
    //     show: bool,
    //     gen_id: Option<usize>,
    //     ) -> PyResult<usize> 
    // {
    
    //     let id = self.get_id(gen_id);

    //     let color = get_color(text_color_rgba, text_color, 1.0, false);
    //     let hl_bkg = get_color(highlight_bkg_rgba, highlight_bkg_color, 1.0, false);
    //     let hl_border_color = get_color(highlight_border_rgba, highlight_border_color, 1.0, false);

    //     let highlight = if hl_bkg.is_some(){
    //         let background = Background::Color(hl_bkg.unwrap());
    //         let mut border = Border::default();
    //         if hl_border_color.is_some() {
    //             border.color = hl_border_color.unwrap();
    //         }
    //         if highlight_border_radius.is_some() {
    //             border.radius = iced::border::Radius::new(highlight_border_radius.unwrap().into())
    //         }
    //         if highlight_border_width.is_some() {
    //             border.width = highlight_border_width.unwrap();
    //         }
    //         Some(Highlight{ background, border })
    //     } else {
    //         None
    //     };

        
    //     let line_height = match line_height {
    //         Some(lh) => Some(LineHeight::Relative(lh)),
    //         None => None,
    //     };

    //     let padding = get_padding_f32(padding);

    //     set_state_of_widget(id, parent_id.clone());

    //     let mut state = access_state();
        
    //     state.widgets.insert(id, IpgWidgets::IpgRichText(
    //         IpgRichText::new(
    //             id,
    //             parent_id,
    //             size,
    //             line_height,
    //             color,
    //             highlight,
    //             padding,
    //             underline,
    //             strikethrough,
    //             show,
    //             style_id,
    //             )));

    //     drop(state);
    //     Ok(id)

    // }

    #[pyo3(signature = (
        parent_id, 
        placeholder, 
        gen_id=None,
        on_input=None, 
        on_submit=None, 
        on_paste=None, 
        width=None, 
        width_fill=false, 
        padding=vec![0.0], 
        size=16.0, 
        line_height_pixels=None,
        line_height_relative=None, 
        user_data=None,
        is_secure=false, 
        style_id=None, show=true,
        ))]
    fn add_text_input(
        &self,
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

        if let Some(py) = on_input {
            add_callback_to_mutex(id, "on_input".to_string(), py);
        }
        if let Some(py) = on_submit {
            add_callback_to_mutex(id, "on_submit".to_string(), py);
        }

        if let Some(py) = on_paste {
            add_callback_to_mutex(id, "on_paste".to_string(), py);
        }

        if let Some(py) = user_data {
            add_user_data_to_mutex(id, py);
        }
        
        let padding = get_padding_f64(padding);

        let width = get_width(width, width_fill);

        let line_height = get_line_height(line_height_pixels, line_height_relative);

        set_state_of_widget(id, parent_id.clone());

        let mut state = access_state();
        
        state.widgets.insert(id, IpgWidgets::IpgTextInput(
            IpgTextInput::new( 
                id,
                parent_id,
                placeholder,
                is_secure,
                // font,
                width,
                padding,
                size,
                line_height,
                style_id,
                show,
                )));

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
    fn add_text_input_style(
        &self,
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

        let background_color = 
            get_color(background_rgba, background_color, 1.0, false);
        let border_color = 
            get_color(border_rgba, border_color, 1.0, false);
        let border_color_hovered = 
            get_color(border_rgba_hovered, border_color_hovered, 1.0, false);
        let border_color_focused = 
            get_color(border_rgba_focused, border_color_focused, 1.0, false);
        // let icon_color = get_color(icon_rgba, icon_color, 1.0, false);
        let placeholder_color = 
            get_color(placeholder_rgba, placeholder_color, 1.0, false);
        let value_color = 
            get_color(value_rgba, value_color, 1.0, false);
        let selection_color = 
            get_color(selection_rgba, selection_color, 1.0, false);

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
                    
        drop(state);
        Ok(id)

    }

    #[pyo3(signature = (
        parent_id, duration_ms, 
        on_start=None, 
        on_tick=None, 
        on_stop=None, 
        label="Start Timer".to_string(), 
        width=None, 
        height=None, 
        width_fill=false, 
        height_fill=false,
        padding=vec![10.0], 
        clip=false, 
        style_id=None, 
        style_standard=None, 
        style_arrow=None, 
        user_data=None,
        gen_id=None, show=true
        ))]
    fn add_timer(
        &self,
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

        if let Some(py) = on_start {
            add_callback_to_mutex(id, "on_start".to_string(), py);
        }
        if let Some(py) = on_tick {
            add_callback_to_mutex(id, "on_tick".to_string(), py);
        }
        if let Some(py) = on_stop {
            add_callback_to_mutex(id, "on_stop".to_string(), py);
        }

        if let Some(py) = user_data {
            add_user_data_to_mutex(id, py);
        }

        let width = get_width(width, width_fill);
        let height = get_height(height, height_fill);

        let padding = get_padding_f64(padding);

        set_state_of_widget(id, parent_id.clone());
        
        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgTimer(
            IpgTimer::new(
                id,
                parent_id,
                duration_ms,
                label,
                width,
                height,
                padding,
                clip,
                style_id,
                style_standard,
                style_arrow,
                show,
                )));

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
        border_radius = vec![0.0], 
        border_width=1.0,
        shadow_color=None, 
        shadow_rgba=None,
        shadow_offset_x=0.0, 
        shadow_offset_y=0.0,
        shadow_blur_radius=1.0,
        text_color=None, 
        text_rgba=None,
        gen_id=None))]
    fn add_timer_style(
        &self,
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

        let background_color: Option<Color> = 
            get_color(background_rgba, background_color, 1.0, false);
        let background_color_hovered: Option<Color> = 
            get_color(background_rgba_hovered, background_color_hovered, 1.0, false);
        let border_color: Option<Color> = 
            get_color(border_rgba, border_color, 1.0, false);
        let shadow_color: Option<Color> = 
            get_color(shadow_rgba, shadow_color, 1.0, false);
        let text_color: Option<Color> = 
            get_color(text_rgba, text_color, 1.0, false);

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

        drop(state);
        Ok(id)

    }

    #[pyo3(signature = (
        parent_id, 
        duration_ms, 
        on_start=None, 
        on_tick=None, 
        on_stop=None, 
        label="Start Timer".to_string(), 
        width=None, 
        height=None, 
        width_fill=false, 
        height_fill=false,
        padding=vec![10.0], 
        clip=false, 
        style_id=None, 
        style_standard=None, 
        style_arrow=None, 
        user_data=None,
        gen_id=None, 
        show=true
        ))]
    fn add_canvas_timer(
        &self,
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

        if let Some(py) = on_start {
            add_callback_to_mutex(id, "on_start".to_string(), py);
        }
        if let Some(py) = on_tick {
            add_callback_to_mutex(id, "on_tick".to_string(), py);
        }
        if let Some(py) = on_stop {
            add_callback_to_mutex(id, "on_stop".to_string(), py);
        }

        if let Some(py) = user_data {
            add_user_data_to_mutex(id, py);
        }

        let width = get_width(width, width_fill);
        let height = get_height(height, height_fill);

        let padding = get_padding_f64(padding);

        set_state_of_widget(id, parent_id);
        
        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgCanvasTimer(
            IpgCanvasTimer::new(
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
                show,
                )));

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
        border_radius = vec![0.0], 
        border_width=1.0,
        shadow_color=None, 
        shadow_rgba=None,
        shadow_offset_x=0.0, 
        shadow_offset_y=0.0,
        shadow_blur_radius=1.0,
        text_color=None, 
        text_rgba=None,
        gen_id=None))]
    fn add_canvas_timer_style(
        &self,
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

        let background_color: Option<Color> = 
            get_color(background_rgba, background_color, 1.0, false);
        let background_color_hovered: Option<Color> = 
            get_color(background_rgba_hovered, background_color_hovered, 1.0, false);
        let border_color: Option<Color> = 
            get_color(border_rgba, border_color, 1.0, false);
        let shadow_color: Option<Color> = 
            get_color(shadow_rgba, shadow_color, 1.0, false);
        let text_color: Option<Color> = 
            get_color(text_rgba, text_color, 1.0, false);

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

        drop(state);
        Ok(id)

    }

    #[pyo3(signature = (
        parent_id, 
        label=None, 
        gen_id=None, 
        toggled=None, 
        width=None, 
        width_fill=false, 
        size=20.0, 
        text_size=16.0,
        text_line_height=1.3, 
        text_alignment=IpgHorizontalAlignment::Center, 
        spacing=10.0, 
        user_data=None, 
        show=true, 
        style_id=None, 
        ))]
    fn add_toggler(
        &self,
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

        if let Some(py) = toggled {
            add_callback_to_mutex(id, "toggled".to_string(), py);
        }

        if let Some(py) = user_data {
            add_user_data_to_mutex(id, py);
        }

        let text_line_height = LineHeight::Relative(text_line_height);

        let width = get_width(width, width_fill);

        set_state_of_widget(id, parent_id.clone());

        let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgToggler(
            IpgToggler::new(
                id,
                parent_id,
                show,
                label,
                width,
                size,
                text_size,
                text_line_height,
                text_alignment,
                spacing,
                style_id,                           
                )));

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
    fn add_toggler_style(
        &self,
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

        let background_color = 
            get_color(background_rgba, background_color, 1.0, false);
        let background_color_toggled = 
            get_color(background_rgba_toggled, background_color_toggled, 1.0, false);
        let background_color_disabled = 
            get_color(background_rgba_disabled, background_color_disabled, 1.0, false);
        let background_border_color = 
            get_color(background_border_rgba, background_border_color, 1.0, false);
        let foreground_color = 
            get_color(foreground_rgba, foreground_color, 1.0, false);
        let foreground_color_toggled = 
            get_color(foreground_rgba_toggled, foreground_color_toggled, 1.0, false);
        let foreground_color_disabled = 
            get_color(foreground_rgba_disabled, foreground_color_disabled, 1.0, false);
        let foreground_border_color = 
            get_color(foreground_border_rgba, foreground_border_color, 1.0, false);
        

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

        drop(state);
        Ok(id)

    }

        #[pyo3(signature = (
            canvas_id,
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
    fn add_arc(
        &self,
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
        let start_angle = start_angle-180.0;
        let end_angle = end_angle-180.0;

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

        let arc = 
            IpgArc { 
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
        Ok(id)

    }

    #[pyo3(signature = (
        canvas_id,
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
    fn add_bezier(
        &self,
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
        let mid_point = get_mid_point(
                                Point::new(points[0].0, points[0].1), 
                                Point::new(points[1].0, points[1].1));

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

        let bezier = 
            IpgBezier{ 
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
        Ok(id)

    }

    #[pyo3(signature = (
        canvas_id,
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
    fn add_circle(
        &self,
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
        
        let circle = 
            IpgCircle{ 
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
        Ok(id)

    }

    #[pyo3(signature = (
        canvas_id,
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
    fn add_ellipse(
        &self,
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

        let ellipse = 
            IpgEllipse{ 
            id, 
            points, 
            center, 
            radii: Vector{x: radius_x, y: radius_y}, 
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
        Ok(id)

    }

    #[pyo3(signature = (
        canvas_id,
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
    fn add_line(
        &self,
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
        
        let line = 
            IpgLine{ 
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
        Ok(id)

    }

    #[pyo3(signature = (
        canvas_id,
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
    fn add_polygon(
        &self,
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
        
        let pg = 
            IpgPolygon{ 
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
        Ok(id)

    }

    #[pyo3(signature = (
        canvas_id,
        points,
        stroke_width,
        stroke_dash_offset=None,
        stroke_dash_segments=None,
        stroke_ipg_color=IpgColor::WHITE,
        stroke_rgba_color=None,
        gen_id=None,
        ))]
    fn add_poly_line(
        &self,
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

        let poly_line = 
            IpgPolyLine{ 
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
        Ok(id)

    }

    #[pyo3(signature = (
        canvas_id,
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
    fn add_rectangle(
        &self,
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

        let rectangle = 
            IpgRectangle{ 
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
        Ok(id)

    }

    #[pyo3(signature = (
        canvas_id,
        image_path,
        width,
        height,
        position_xy,
        align_center=true,
        align_top_left_xy=None,
        degrees=0.0,
        gen_id=None,
        ))]
    fn add_canvas_image(
        &self,
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

        let canvas_image = 
            IpgCanvasImage{ 
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
        Ok(id)

    }

    #[pyo3(signature = (
        enabled=false, 
        on_key_press=None, 
        on_key_release=None,
        user_data=None))]
    fn add_event_keyboard(
        &self, 
        enabled: bool,
        on_key_press: Option<PyObject>,
        on_key_release: Option<PyObject>,
        user_data: Option<PyObject>,
        )  -> PyResult<usize>
    {
        let id = self.get_id(None);

        let mut events = access_events();

        if let Some(py) = on_key_press {
            events.events.insert((id, "key pressed".to_string()), py);
        }
        if let Some(py) = on_key_release {
            events.events.insert((id, "key released".to_string()), py);
        }

        drop(events);

        let mut callback_user_data = access_user_data1();

        if let Some(py) = user_data {
            callback_user_data.user_data.insert(id, py);
        }
       
        drop(callback_user_data);
        
        let mut state = access_state();

        state.keyboard_event_id_enabled = (id, enabled);

        drop(state);
        Ok(id)
    }

    #[pyo3(signature = (
        enabled=false, 
        on_move=None, 
        on_enter_window=None, 
        on_exit_window=None, 
        on_left_press=None, 
        on_left_release=None,
        on_middle_press=None, 
        on_middle_release=None,
        on_right_press=None, 
        on_right_release=None,
        on_middle_scroll_line=None,
        user_data=None))]
    fn add_event_mouse(
        &self, 
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
        let id = self.get_id(None);

        let mut events = access_events();

        if let Some(py) = on_move {
            events.events.insert((id, "move".to_string()), py);
        }
        if let Some(py) = on_enter_window {
            events.events.insert((id, "enter window".to_string()), py);
        }
        if let Some(py) = on_exit_window {
            events.events.insert((id, "exit window".to_string()), py);
        }
        if let Some(py) = on_left_press {
            events.events.insert((id, "left press".to_string()), py);
        }
        if let Some(py) = on_left_release {
            events.events.insert((id, "left release".to_string()), py);
        }
        if let Some(py) = on_middle_press {
            events.events.insert((id, "middle press".to_string()), py);
        }
        if let Some(py) = on_middle_release {
            events.events.insert((id, "middle release".to_string()), py);
        }
        if let Some(py) = on_right_press {
            events.events.insert((id, "right press".to_string()), py);
        }
        if let Some(py) = on_right_release {
            events.events.insert((id, "right release".to_string()), py);
        }
        if let Some(py) = on_middle_scroll_line {
            events.events.insert((id, "middle scroll line".to_string()), py);
        }

        drop(events);

        let mut callback_user_data = access_user_data1();

        if let Some(py) = user_data {
            callback_user_data.user_data.insert(id, py);
        }
       
        drop(callback_user_data);
        
        let mut state = access_state();

        state.mouse_event_id_enabled = (id, enabled);

        drop(state);
        Ok(id)
    }

    #[pyo3(signature = (
        enabled=false, 
        on_closed=None, 
        on_moved=None, 
        on_resized=None,
        on_redraw_requested=None,
        on_close_requested=None,
        on_focused=None, 
        on_unfocused=None,
        on_file_hovered=None,
        on_file_dropped=None,
        on_files_hovered_left=None,
        user_data=None))]
    fn add_event_window(
        &self,
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
        let id = self.get_id(None);

        let mut events = access_events();

        if let Some(py) = on_closed {
            events.events.insert((id, "closed".to_string()), py);
        }
        if let Some(py) = on_moved {
            events.events.insert((id, "moved".to_string()), py);
        }
        if let Some(py) = on_resized {
            events.events.insert((id, "resized".to_string()), py);
        }
        if let Some(py) = on_redraw_requested {
            events.events.insert((id, "redraw requested".to_string()), py);
        }
        if let Some(py) = on_close_requested {
            events.events.insert((id, "close requested".to_string()), py);
        }
        if let Some(py) = on_focused {
            events.events.insert((id, "focused".to_string()), py);
        }
        if let Some(py) = on_unfocused {
            events.events.insert((id, "unfocused".to_string()), py);
        }
        if let Some(py) = on_file_hovered {
            events.events.insert((id, "file hovered".to_string()), py);
        }
        if let Some(py) = on_file_dropped {
            events.events.insert((id, "file dropped".to_string()), py);
        }

        if let Some(py) = on_files_hovered_left {
            events.events.insert((id, "files hovered left".to_string()), py);
        }
       
        drop(events);

        let mut callback_user_data = access_user_data1();

        if let Some(py) = user_data {
            callback_user_data.user_data.insert(id, py);
        }
       
        drop(callback_user_data);
        
        let mut state = access_state();

        state.window_event_id_enabled = (id, enabled);

        drop(state);
        Ok(id)
    }

    #[pyo3(signature = (
        window_id, 
        wid))]
    fn delete_item(
        &self, 
        window_id: String, 
        wid: usize) 
    {
        let mut all_updates = access_update_items();

        all_updates.deletes.push((window_id, wid));

        drop(all_updates);
    }

    #[pyo3(signature = (
        window_id, 
        ids))]
    fn show_items(
        &self, 
        window_id: String, 
        ids: Vec<(usize, bool)>) 
    {
        let mut all_updates = access_update_items();

        all_updates.shows.push((window_id, ids));

        drop(all_updates);
    }

    #[pyo3(signature = (
        wid, 
        param, 
        value))]
    fn update_dataframe(
        &self, 
        wid: usize, 
        param: PyObject, 
        value: PyDataFrame) 
    {
        let mut all_updates = access_update_items();

        all_updates.dataframes.push((wid, param, value));

        drop(all_updates);
    }

    #[pyo3(signature = (wid, param, value))]
    fn update_item(&self, 
                    wid: usize, 
                    param: PyObject, 
                    value: PyObject) 
    {
        let mut all_updates = access_update_items();
        
        all_updates.updates.push((wid, param, value));

        drop(all_updates);
    }

    #[pyo3(signature = (wid, param, value))]
    fn update_canvas_item(&self, 
                            wid: usize, 
                            param: PyObject, 
                            value: PyObject) 
    {
        let mut canvas_items = access_canvas_update_items();

        canvas_items.updates.push((wid, param, value));

        drop(canvas_items);
    }

    #[pyo3(signature = (
        window_id, 
        widget_id, 
        target_container_str_id, 
        move_after=None,
        move_before=None
        ))]
    fn move_widget(
        &self,
        window_id: String,
        widget_id: usize,
        target_container_str_id: String,
        move_after: Option<usize>,
        move_before: Option<usize>)
    {
        let mut all_updates = access_update_items();
        
        all_updates.moves.push((window_id, widget_id, target_container_str_id, move_after, move_before));
        
        drop(all_updates);
    }
    
    #[pyo3(signature = (color))]
    fn get_rgba_color(
        &self, 
        color: IpgColor
        ) -> PyResult<[f32; 4]>
    {
        let rgb = if let Some(base) = get_color(None, Some(color), 1.0, false) {
            base
        } else {
            panic!("Unable to get the rgba format of the color")
        };

        Ok([rgb.r, rgb.g, rgb.b, 1.0])
    }

    #[pyo3(signature = (
        base_color=None, 
        base_rgba=None))]
    fn get_color_palette(
        &self, 
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
    #[pyo3(signature = (gen_id=None))]
    fn get_id(
        &self, 
        gen_id: Option<usize>
        ) -> usize
    {
        // When an id is generated, it is put into the gen_ids state mutex.
        // The below checks that if the user is using the gen_id field for the
        // widget, then it must be in the gen_id vec otherwise, the user must 
        // have made an error.
        let mut state = access_state();

        match gen_id {
            Some(id) => {
                if state.gen_ids.contains(&id) {
                    drop(state);
                    id
                } else {
                    panic!("The gen_id parameter for id {id} was not found in the gen_id list.")
                }
            }
            None => {
                state.last_id += 1;
                let id = state.last_id;
                drop(state);
                id
                },
        }

    }
}

fn match_widget(
    widget: &mut IpgWidgets, 
    item: &PyObject, 
    value: &PyObject) 
{
    match widget {
        IpgWidgets::IpgButton(btn) => {
            button_item_update(btn, item, value);
        },
        IpgWidgets::IpgButtonStyle(style) => {
            button_style_update_item(style, item, value);
        },
        IpgWidgets::IpgCard(card) => {
            card_item_update(card, item, value);
        },
        IpgWidgets::IpgCardStyle(style) => {
            card_style_update(style, item, value);
        },
        IpgWidgets::IpgCheckBox(chk) => {
            checkbox_item_update(chk, item, value);
        },
        IpgWidgets::IpgCheckboxStyle(style) => {
            checkbox_style_update_item(style, item, value);
        },
        IpgWidgets::IpgColorPicker(cp) => {
            color_picker_update(cp, item, value);
        },
        IpgWidgets::IpgColorPickerStyle(style) => {
            color_picker_style_update_item(style, item, value);
        },
        IpgWidgets::IpgContainerStyle(style) => {
            container_style_update_item(style, item, value);
        },
        IpgWidgets::IpgDatePicker(dp) => {
            date_picker_item_update(dp, item, value);
        },
        IpgWidgets::IpgDividerHorizontal(div) => {
            divider_horizontal_item_update(div, item, value);
        },
        IpgWidgets::IpgDividerVertical(div) => {
            divider_vertical_item_update(div, item, value);
        },
        IpgWidgets::IpgDividerStyle(style) => {
            divider_style_update_item(style, item, value);
        }
        IpgWidgets::IpgImage(img) => {
            image_item_update(img, item, value);
        },
        IpgWidgets::IpgMenuStyle(style) => {
            menu_style_update_item(style, item, value);
        },
        IpgWidgets::IpgMenuBarStyle(style) => {
            menu_bar_style_update_item(style, item, value);
        },
        IpgWidgets::IpgOpaqueStyle(style) => {
            opaque_style_update_item(style, item, value);
        }
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
        IpgWidgets::IpgTableStyle(style) => {
            table_style_update_item(style, item, value);
        }
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

fn match_container(
    container: &mut IpgContainers, 
    item: &PyObject, 
    value: &PyObject, 
    canvas_state: &mut IpgCanvasState,
    last_id: usize,
    ) -> Option<usize>
{
    match container {
        IpgContainers::IpgCanvas(_can) => {
            canvas_item_update(canvas_state, item, value, last_id)
        },
        IpgContainers::IpgColumn(col) => {
            column_item_update(col, item, value);
            None
        },
        IpgContainers::IpgContainer(cont) => {
            container_item_update(cont, item, value);
            None
        },
        IpgContainers::IpgMenu(menu) => {
            menu_item_update(menu, item, value);
            None
        },
        IpgContainers::IpgMouseArea(m_area) => {
            mousearea_item_update(m_area, item, value);
            None
        },
        IpgContainers::IpgOpaque(op) => {
            opaque_item_update(op, item, value);
            None
        },
        IpgContainers::IpgRow(row) => {
            row_item_update(row, item, value);
            None
        },
        IpgContainers::IpgStack(stack) => {
            stack_item_update(stack, item, value);
            None
        },
        IpgContainers::IpgTable(table) => {
            table_item_update(table, item, value);
            None
        },
        IpgContainers::IpgScrollable(scroll) => {
            scrollable_item_update(scroll, item, value);
            None
        },
        IpgContainers::IpgWindow(wnd) => {
            window_item_update(wnd, item, value);
            None
        },
        _ => None,
    }
}

fn match_container_for_df(
    container: &mut IpgContainers, 
    item: &PyObject, 
    value: &PyDataFrame) 
{
    if let IpgContainers::IpgTable(table) = container {
        table_dataframe_update(table, item, value);
    }
}

fn set_state_cont_wnd_ids(
    state: &mut State, 
    wnd_id: &String, 
    cnt_str_id: String, 
    cnt_id: usize, name: String) 
{
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
    m.add_class::<IpgCardParam>()?;
    m.add_class::<IpgCardStyleParam>()?;
    m.add_class::<IpgCheckboxParam>()?;
    m.add_class::<IpgCheckboxStyleParam>()?;
    m.add_class::<IpgColor>()?;
    m.add_class::<IpgColorPickerParam>()?;
    m.add_class::<IpgColorPickerStyleParam>()?;
    m.add_class::<IpgColumnParam>()?;
    m.add_class::<IpgContainerParam>()?;
    m.add_class::<IpgContainerStyleParam>()?;
    m.add_class::<IpgDatePickerParam>()?;
    m.add_class::<IpgDividerParam>()?;
    m.add_class::<IpgDividerStyleParam>()?;
    m.add_class::<IpgImageContentFit>()?;
    m.add_class::<IpgImageFilterMethod>()?;
    m.add_class::<IpgImageParam>()?;
    m.add_class::<IpgImageRotation>()?;
    m.add_class::<IpgMenuParam>()?;
    m.add_class::<IpgMenuStyleParam>()?;
    m.add_class::<IpgMenuBarStyleParam>()?;
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
    m.add_class::<IpgRowParam>()?;
    m.add_class::<IpgRuleStyleParam>()?;
    m.add_class::<IpgScrollableDirection>()?;
    m.add_class::<IpgScrollableParam>()?;
    m.add_class::<IpgScrollableStyleParam>()?;
    m.add_class::<IpgSelectableTextParam>()?;
    m.add_class::<IpgSeparatorParam>()?;
    m.add_class::<IpgSeparatorType>()?;
    m.add_class::<IpgSeparatorParam>()?;
    m.add_class::<IpgSeparatorStyleParam>()?;
    m.add_class::<IpgSliderParam>()?;
    m.add_class::<IpgSliderStyleParam>()?;
    m.add_class::<IpgStackParam>()?;
    m.add_class::<IpgStyleStandard>()?;
    m.add_class::<IpgSvgParam>()?;
    m.add_class::<IpgTableParam>()?;
    m.add_class::<IpgTableStyleParam>()?;
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
    parent_id: String) 
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
    
    let parent_uid = find_parent_uid(state.ids_ipd_ids.get(&wnd_id_usize).unwrap(), parent_id.clone());
    
    state.ids_ipd_ids.get_mut(&wnd_id_usize).unwrap().push(IpgIds{id, parent_uid, container_id,
                                                        parent_id, is_container: true});

    state.container_ids.get_mut(&wnd_id_usize).unwrap().push(id);

    drop(state);

}

fn set_state_of_widget(
    id: usize,  
    parent_id: String)
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

    let parent_uid = find_parent_uid(state.ids_ipd_ids.get(&wnd_id_usize).unwrap(), parent_id.clone());
    
    state.ids_ipd_ids.get_mut(&wnd_id_usize).unwrap().push(IpgIds{id, parent_uid, container_id: None,
                                                        parent_id, is_container: false});

    drop(state);
}

fn set_state_of_widget_running_state(
    state: &mut IpgState,
    id: usize,  
    parent_id: String)
{
    let wnd_id_str = match state.container_wnd_str_ids.get(&parent_id) {
        Some(id) => id.clone(),
        None => panic!("The main window id could not be found using parent_id {}, check that your parent_id matches a container ", parent_id)
    };

    let wnd_id_usize = match state.windows_str_ids.get(&wnd_id_str) {
        Some(id) => *id,
        None => panic!("window id {} could not be found in set_state_of_widget", wnd_id_str),
    };

    let parent_uid = find_parent_uid(state.ids.get(&wnd_id_usize).unwrap(), parent_id.clone());
    
    state.ids.get_mut(&wnd_id_usize).unwrap().push(IpgIds{id, parent_uid, container_id: None,
                                                        parent_id, is_container: false});

}

fn add_callback_to_mutex(
    id: usize, 
    event_name: String, 
    py_obj: PyObject, 
    ) 
{
    let mut app_cbs = access_callbacks();
    app_cbs.callbacks.insert((id, event_name), py_obj);
    drop(app_cbs);
}

fn add_user_data_to_mutex(
    id: usize, 
    user_data: PyObject) 
{
    let mut lock = USERDATA1.try_lock();
    if let Ok(ref mut ud) = lock {
        ud.user_data.insert(id, user_data);
        
    } else {
        let mut temp_ud = access_user_data2();
        temp_ud.user_data.insert(id, user_data);
        drop(temp_ud);
    }
    drop(lock);
}

pub fn find_parent_uid(
    ipg_ids: &[IpgIds], 
    parent_id: String) 
    -> usize 
{
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
