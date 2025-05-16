//! themes

use iced::Color;
use pyo3::pyclass;

use crate::ipg_widgets::ipg_enums::IpgHorizontalAlignment;
pub static CHART_DEFAULT_WIDTH: f32 = 600.0;
pub static CHART_DEFAULT_HEIGHT: f32 = 400.0;

pub static CHART_DEFAULT_TITLE_HEIGHT: f32 = 30.0;
pub static CHART_DEFAULT_SUB_TITLE_HEIGHT: f32 = 20.0;

pub static CHART_DEFAULT_X_AXIS_HEIGHT: f32 = 30.0;
pub static CHART_DEFAULT_X_AXIS_NAME_GAP: f32 = 5.0;

pub static CHART_DEFAULT_Y_AXIS_WIDTH: f32 = 40.0;
pub static CHART_DEFAULT_Y_AXIS_NAME_GAP: f32 = 8.0;
pub static CHART_DEFAULT_Y_AXIS_SPLIT_NUMBER: usize = 6;
pub static CHART_DEFAULT_FONT_SIZE: f32 = 14.0;

pub static CHART_DEFAULT_SERIES_STROKE_WIDTH: f32 = 2.0;

pub static THEME_DARK: &str = "dark";
pub static THEME_ANT: &str = "ant";
pub static THEME_GRAFANA: &str = "grafana";

static LIGHT_THEME_NAME: &str = "light";


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgChartTheme {
    DarkTheme,
    AntTheme,
    VintageTheme,
    ShineTheme,
    WaldenTheme,
    WesterosTheme,
    ChalkTheme,
    GrafanaTheme,
    ShadcnTheme,
}


#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    pub is_light: bool,
    pub font_family: String,
    pub margin: [f32; 4],
    pub width: f32,
    pub height: f32,
    pub background_color: Color,

    // title
    pub title_font_size: f32,
    pub title_font_color: Color,
    pub title_font_weight: Option<String>,
    pub title_margin: Option<[f32; 4]>,
    pub title_align: IpgHorizontalAlignment,
    pub title_height: f32,

    // sub title
    pub sub_title_font_size: f32,
    pub sub_title_font_color: Color,
    pub sub_title_margin: Option<[f32; 4]>,
    pub sub_title_align: IpgHorizontalAlignment,
    pub sub_title_height: f32,

    // legend
    pub legend_font_size: f32,
    pub legend_font_color: Color,
    pub legend_align: IpgHorizontalAlignment,
    pub legend_margin: Option<[f32; 4]>,

    // x axis
    pub x_axis_font_size: f32,
    pub x_axis_stroke_color: Color,
    pub x_axis_font_color: Color,
    pub x_axis_name_gap: f32,
    pub x_axis_height: f32,

    // y axis
    pub y_axis_font_size: f32,
    pub y_axis_font_color: Color,
    pub y_axis_stroke_color: Color,
    pub y_axis_split_number: usize,
    pub y_axis_name_gap: f32,

    // grid
    pub grid_stroke_color: Color,
    pub grid_stroke_width: f32,

    // series
    pub series_stroke_width: f32,
    pub series_label_font_size: f32,
    pub series_label_font_color: Color,
    pub series_colors: Vec<Color>,

    // table
    pub table_header_color: Color,
    pub table_body_colors: Vec<Color>,
    pub table_border_color: Color,
}
