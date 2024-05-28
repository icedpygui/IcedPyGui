#![allow(dead_code)]
use std::collections::BTreeMap;

use crate::{access_state, IpgAlignment};
use iced::window;
use iced::{alignment::{Horizontal, Vertical}, Length, Padding};
use iced::widget::text::{Shaping, LineHeight};

use pyo3::{PyObject, Python};

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


pub fn get_usize_of_id(iced_id: window::Id) -> usize {

    let iced_id_str = format!("{:?}", iced_id);

    let mut numeric_vec: Vec<i32> = vec![];

    for c in iced_id_str.chars() {
        if c.is_numeric() {
            numeric_vec.push(c as i32 - 0x30);
        }
    }

    if numeric_vec.len() == 0 {
        panic!("usize of Id could not be obtained")
    }

    let mut acc: i32 = 0;

    for num in numeric_vec {
        acc *= 10;
        acc += num;
    }
    
    acc as usize
    
}


pub fn convert_vecs<T, U>(vector: Vec<T>) -> Vec<U>
  where
	T: TryInto<U>,
	<T as std::convert::TryInto<U>>::Error: std::fmt::Display
{
	vector
		.into_iter()
		.map(|value_t|
			match TryInto::try_into(value_t) {
				Ok(value_u) => value_u,
				Err(why) => {
					let t = std::any::type_name::<T>();
					let u = std::any::type_name::<U>();
					panic!("Error converting from {t} to {u}: {why}")
				}
			}
		)
		.collect()
}

// Standard method for Length using Width
pub fn get_width(width: Option<f32>, width_fill: bool)-> Length {
    // width overrides width_fill
    match width 
            {
                Some(wd) => Length::Fixed(wd),
                None => {
                    let wd = 
                        match width_fill {
                            true => Length::Fill,
                            false => Length::Shrink,
                        };
                    wd
                },
            }
}

// Standard method for Length using Height
pub fn get_height(height: Option<f32>, height_fill: bool)-> Length {
    // height overrides height_fill
    match height 
            {
                Some(ht) => Length::Fixed(ht),
                None => {
                    let ht = 
                        match height_fill {
                            true => Length::Fill,
                            false => Length::Shrink,
                        };
                    ht
                },
            }
}

// Standard method for padding
pub fn get_padding(padding: Vec<f64>)-> Padding {
    let len = padding.len();
    match len {
    0 => panic!("Padding must have at List of at least 1, 2 or 4 items"),
    1 => Padding::from(padding[0] as f32),
    2 => Padding::from(vec_to_array2(&padding)),
    3 => panic!("Padding must have a List of 1, 2, or 4 items"),
    4 => Padding::from(vec_to_array4(&padding)),
    _ => panic!("Padding must have a List of less than 4 items"),
    }
}

fn vec_to_array2(arr: &[f64]) -> [f32; 2] {

    [arr[0] as f32, arr[1] as f32]
    
}

fn vec_to_array4(arr: &[f64]) -> [f32; 4] {
    [arr[0] as f32, arr[1] as f32, arr[2] as f32, arr[3] as f32]
}


pub fn get_horizontal_alignment(align: &str) -> Horizontal {
    match align {
        "left" => Horizontal::Left,
        "center" => Horizontal::Center,
        "right" => Horizontal::Right,
        _ => panic!("Incorrect Horizontal Alignment found {}", align)
    }
}

pub fn get_vertical_alignment(align: &str) -> Vertical {
    match align {
        "top" => Vertical::Top,
        "center" => Vertical::Center,
        "bottom" => Vertical::Bottom,
        _ => panic!("Incorrect Vertical Alignment found {}", align)
    }
}

pub fn get_shaping(shape: String) -> Shaping {
    match shape.as_str() {
        "basic" => return Shaping::Basic,
        "advanced" => return Shaping::Advanced,
        _ => panic!("The shape {shape} is not allowed, used either 'basic' or 'advanced'")
    }
}

// TODO: Need to figure out pixel vs f32, just using f32 for now.
pub fn get_line_height(line_height: Option<f32>) -> LineHeight {
    match line_height {
        Some(lh) => LineHeight::Relative(lh),
        None => LineHeight::default(),
    }
}


pub const MONTH_NAMES: [&'static str; 13] = ["", "January", "Feburary", "March", 
                                        "April", "May", "June", "July", 
                                        "August", "September", "October", 
                                        "November", "December"];
                                
pub const DATE_FORMATS: [&'static str; 3] = ["mm-dd-YYYY", "YYYY-mm-dd", "mm-dd-YY"];
pub const WEEKDAYS: [&'static str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
pub const DAYS: [&'static str; 7] = ["S", "M", "T", "W", "T", "F", "S"];

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


// pub fn try_extract_vec_str_option(value: PyObject) -> Option<Vec<String>> {
//     Python::with_gil(|py| {
//         let res = value.extract::<Vec<String>>(py);
//         match res {
//             Ok(val) => Some(val),
//             Err(_) => None,
//         }
//     })  
// }

pub fn try_extract_boolean(value: PyObject) -> bool {
    Python::with_gil(|py| {
        let res = value.extract::<bool>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("Unable to extract python bool"),
        }
    })  
}


pub fn try_extract_dict(items: PyObject) -> BTreeMap<String, Vec<String>> {
    Python::with_gil(|py| {

        let res = items.extract::<BTreeMap<String, Vec<String>>>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("Unable to extract python dict"),
        }
    })
}

pub fn try_extract_ipg_alignment(value: PyObject) -> IpgAlignment {
    Python::with_gil(|py| {

        let res = value.extract::<IpgAlignment>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("Unable to extract python object for Alignment"),
        }
    })
}

pub fn get_container_id_via_string(id: String) -> usize {

    let state = access_state();

    let container_id_opt = state.container_str_ids.get(&id);

    let container_id: usize = if container_id_opt.is_some() {
        *container_id_opt.unwrap()
    } else {
        panic!("add_styling_background: parent_id not found.")
    };

    drop(state);

    container_id
}