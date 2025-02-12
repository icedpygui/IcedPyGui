//! helpers
use std::collections::HashMap;

use crate::graphics::colors::IpgColor;
use crate::style::styling::IpgStyleStandard;
use crate::access_state;
use iced::border::Radius;
use iced::{window, Alignment, Pixels};
use iced::{alignment::{Horizontal, Vertical}, Length, Padding};
use iced::widget::text::{Shaping, LineHeight};

use pyo3::{PyObject, Python};

use super::ipg_enums::{IpgAlignment, IpgHorizontalAlignment, IpgVerticalAlignment};

pub fn check_for_dup_container_ids(id: usize, container_id: Option<String>) {

    let state = access_state();
    
    let parents = match state.ids.get(&id) {
        Some(ids) => ids,
        None => panic!("Ids in check_for_dup_container_ids not found")
    };

    for parent in parents {
        if container_id == parent.container_id {
            panic!("Container Id {:?} is not unique", container_id);
        }
    }
    
    drop(state);
}

pub fn find_key_for_value(ids: HashMap<window::Id, usize>, value: usize) -> window::Id {
    let state = access_state();
    let map = &ids;
    let id = map.iter()
        .find_map(|(key, &val)| if val == value { Some(key) } else { None });
    
    match id {
        Some(id) => {
            let iced_id = *id;
            drop(state);
            iced_id
        },
        None => panic!("Unable to find the iced id via the ipg id {}.", value)
    }
}


pub fn get_alignment(align: IpgAlignment) -> Alignment {

    match align {
        IpgAlignment::Start => Alignment::Start,
        IpgAlignment::Center => Alignment::Center,
        IpgAlignment::End => Alignment::End,
    }
}

pub fn get_horizontal_alignment(h_align: IpgHorizontalAlignment) -> Horizontal {

    match h_align {
        IpgHorizontalAlignment::Left => Horizontal::Left,
        IpgHorizontalAlignment::Center => Horizontal::Center,
        IpgHorizontalAlignment::Right => Horizontal::Right,
    }
}

pub fn get_vertical_alignment(v_align: IpgVerticalAlignment) -> Vertical {
    
    match v_align {
        IpgVerticalAlignment::Top => Vertical::Top,
        IpgVerticalAlignment::Center => Vertical::Center,
        IpgVerticalAlignment::Bottom => Vertical::Bottom,
    }
}

// Standard method for Length using Width
pub fn get_width(width: Option<f32>, width_fill: bool)-> Length {
    // width overrides width_fill
    match width {
        Some(wd) => Length::Fixed(wd),
        None => {
                match width_fill {
                    true => Length::Fill,
                    false => Length::Shrink,
                }
        },
    }
}

// Standard method for Length using Height
pub fn get_height(height: Option<f32>, height_fill: bool)-> Length {
    // height overrides height_fill
    match height {
        Some(ht) => Length::Fixed(ht),
        None => {
            match height_fill {
                true => Length::Fill,
                false => Length::Shrink,
            }
        },
    }
}

// Standard method for padding
pub fn get_padding_f64(padding: Vec<f64>)-> Padding {
    let len = padding.len();
    match len {
    0 => panic!("Padding must have at List of at least 1, 2 or 4 items"),
    1 => Padding::from(padding[0] as f32),
    2 => Padding::from(vec_to_array2_f64(&padding)),
    3 => panic!("Padding must have a List of 1, 2, or 4 items"),
    4 => {
        let mut pad = Padding::default();
        pad = pad.top(padding[0] as f32);
        pad = pad.right(padding[1] as f32);
        pad = pad.bottom(padding[2] as f32);
        pad = pad.left(padding[3] as f32);
        pad
    },
    _ => panic!("Padding must have a List of less than 4 items"),
    }
}

pub fn get_padding_f32(padding: Option<Vec<f32>>)-> Padding {
    let padding = match padding {
        None => return Padding::ZERO,
        Some(p) => p,
    };
    let len = padding.len();
    match len {
    0 => panic!("Padding must have at List of at least 1, 2 or 4 items"),
    1 => Padding::from(padding[0]),
    2 => Padding::from(vec_to_array2_f32(&padding)),
    3 => panic!("Padding must have a List of 1, 2, or 4 items"),
    4 => {
        let mut pad = Padding::default();
        pad = pad.top(padding[0]);
        pad = pad.right(padding[1]);
        pad = pad.bottom(padding[2]);
        pad = pad.left(padding[3]);
        pad
    },
    _ => panic!("Padding must have a List of less than 4 items"),
    }
}

pub fn get_radius(border_radius: Vec<f32>, widget_name: String) -> Radius {
    if border_radius.len() == 1 {
        Radius::from(border_radius[0])
    } else if border_radius.len() == 4 {
        let mut rad = Radius::default();
        rad = rad.top_left(border_radius[0]);
        rad = rad.top_right(border_radius[1]); 
        rad = rad.bottom_right(border_radius[2]); 
        rad = rad.bottom_left(border_radius[3]);
        rad
    } else {
        panic!("{} style: Border radius must be a list of 1 or 4 items", widget_name)
    }
}

fn vec_to_array2_f64(arr: &[f64]) -> [f32; 2] {
    [arr[0] as f32, arr[1] as f32]
}

fn vec_to_array2_f32(arr: &[f32]) -> [f32; 2] {
    [arr[0], arr[1]]
}

pub fn get_shaping(shape: String) -> Shaping {
    match shape.as_str() {
        "basic" => Shaping::Basic,
        "advanced" => Shaping::Advanced,
        _ => panic!("The shape {shape} is not allowed, used either 'basic' or 'advanced'")
    }
}

pub fn get_line_height(pixels: Option<u16>, relative: Option<f32>) -> LineHeight {
    if let Some(pixs) =  pixels {
        LineHeight::Absolute(Pixels(pixs.into()))
    } else if let Some(rel) = relative {
        LineHeight::Relative(rel)
    } else {
        LineHeight::default()
    }
}


pub const MONTH_NAMES: [&str; 13] = ["", "January", "Feburary", "March", 
                                        "April", "May", "June", "July", 
                                        "August", "September", "October", 
                                        "November", "December"];
                                
pub const DATE_FORMATS: [&str; 3] = ["mm-dd-YYYY", "YYYY-mm-dd", "mm-dd-YY"];
pub const WEEKDAYS: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
pub const DAYS: [&str; 7] = ["S", "M", "T", "W", "T", "F", "S"];

pub fn format_date(format: String, year: i32, month: usize, day: usize) -> String {

    match format.as_str() {
        "YYYY-mm-dd" => {
            let mon_str = convert_to_len_two(month);
            let day_str = convert_to_len_two(day);
            format!("{}-{}-{}", year, mon_str, day_str)
        },
        "mm-dd-YYYY" => {
            let mon_str = convert_to_len_two(month);
            let day_str = convert_to_len_two(day);
            format!("{}-{}-{}", mon_str, day_str, year)
        },
        "mm-dd-YY" => {
            let mon_str = convert_to_len_two(month);
            let day_str = convert_to_len_two(day);
            let s = year.to_string();
            format!("{}-{}-{}", mon_str, day_str, &s[2..4])
        },
        _ => panic!("Calendar Date format {} not found", format)
    }
}

fn convert_to_len_two(value: usize) -> String {

    if value < 10 {
        "0".to_string() + &value.to_string() 
    } else {
        value.to_string()
    }
}

pub fn try_extract_i64(value: PyObject) -> i64 {
    Python::with_gil(|py| {
        let res = value.extract::<i64>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("Unable to extract python integer"),
        }
    })  
}

pub fn try_extract_f64(value: PyObject) -> f64 {
    Python::with_gil(|py| {
        let res = value.extract::<f64>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("Unable to extract python float"),
        }
    })  
}


pub fn try_extract_i64_option(value: PyObject) -> Option<i64> {
    Python::with_gil(|py| {
        let res = value.extract::<i64>(py);
        match res {
            Ok(val) => Some(val),
            Err(_) => None,
        }
    })  
}

pub fn try_extract_u16(value: PyObject) -> u16 {
    Python::with_gil(|py| {
        let res = value.extract::<u16>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("Unable to extract u16"),
        }
    })  
}

pub fn try_extract_u64(value: PyObject) -> u64 {
    Python::with_gil(|py| {
        let res = value.extract::<u64>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("Unable to extract u64"),
        }
    })  
}

pub fn try_extract_f64_option(value: PyObject) -> Option<f64> {
    Python::with_gil(|py| {
        let res = value.extract::<f64>(py);
        match res {
            Ok(val) => Some(val),
            Err(_) => None,
        }
    })  
}

pub fn try_extract_vec_f64(value: PyObject) -> Vec<f64> {
    Python::with_gil(|py| {
        let res = value.extract::<Vec<f64>>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("Unable to extract python list[float]"),
        }
    })  
}

pub fn try_extract_vec_f32(value: PyObject) -> Vec<f32> {
    Python::with_gil(|py| {
        let res = value.extract::<Vec<f32>>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("Unable to extract python list[float]"),
        }
    })  
}

pub fn try_extract_vec_u16(value: PyObject) -> Vec<u16> {
    Python::with_gil(|py| {
        let res = value.extract::<Vec<u16>>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("Unable to extract python list[uint]"),
        }
    })  
}

pub fn try_extract_array_2(value: PyObject) -> [f32; 2] {
    Python::with_gil(|py| {

        let res = value.extract::<[f32; 2]>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("Unable to extract python object for 2 item list"),
        }
    })
}

pub fn try_extract_string(value: PyObject) -> String {
    Python::with_gil(|py| {
        let res = value.extract::<String>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("Unable to extract python str"),
        }
    })  
}

pub fn try_extract_vec_str(value: PyObject) -> Vec<String> {
    Python::with_gil(|py| {
        let res = value.extract::<Vec<String>>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("Unable to extract python list[str]"),
        }
    })  
}

pub fn try_extract_boolean(value: PyObject) -> bool {
    Python::with_gil(|py| {
        let res = value.extract::<bool>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("Unable to extract python bool"),
        }
    })  
}

// These alignments return options so that only the canvas text alignment needs one py value type
pub fn try_extract_ipg_horizontal_alignment(value: PyObject) -> Option<IpgHorizontalAlignment> {
    Python::with_gil(|py| {

        let res = value.extract::<IpgHorizontalAlignment>(py);
        match res {
            Ok(val) => Some(val),
            Err(_) => None,
        }
    })
}

pub fn try_extract_ipg_vertical_alignment(value: PyObject) -> Option<IpgVerticalAlignment> {
    Python::with_gil(|py| {

        let res = value.extract::<IpgVerticalAlignment>(py);
        match res {
            Ok(val) => Some(val),
            Err(_) => None,
        }
    })
}

pub fn try_extract_ipg_alignment(value: PyObject) -> Option<IpgAlignment> {
    Python::with_gil(|py| {

        let res = value.extract::<IpgAlignment>(py);
        match res {
            Ok(val) => Some(val),
            Err(_) => None,
        }
    })
}

pub fn try_extract_style_standard(value: PyObject) -> IpgStyleStandard {
    Python::with_gil(|py| {

        let res = value.extract::<IpgStyleStandard>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("Unable to extract python object for StyleStandard"),
        }
    })
}

pub fn try_extract_ipg_color(value: PyObject) -> IpgColor {
    Python::with_gil(|py| {

        let res = value.extract::<IpgColor>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("Unable to extract python object for IpgColor"),
        }
    })
}

pub fn try_extract_rgba_color(value: PyObject) -> [f32; 4] {
    Python::with_gil(|py| {

        let res = value.extract::<[f32; 4]>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("Unable to extract python object for RGBA color"),
        }
    })
}

pub fn try_extract_point(value: PyObject) -> [f32; 2] {
    Python::with_gil(|py| {

        let res = value.extract::<[f32; 2]>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("Unable to extract python object for point"),
        }
    })
}

