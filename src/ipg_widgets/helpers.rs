#![allow(unused)]

use crate::access_state;
use iced::window;
use iced::{Alignment, alignment::{Horizontal, Vertical}, Length, Padding};
use iced::widget::text::{Shaping, LineHeight};


use crate::iced_widgets::scrollable::{Direction, Properties};

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

pub fn check_for_dup_user_ids(id: usize, user_id: &Option<String>) {

    match user_id {
        Some(_id) => (),
        None => return,
    }

    let state = access_state();

    let parents = match state.ids.get(&id) {
        Some(ids) => ids,
        None => panic!("Ids in check_for_dup_container_ids not found")
    };

    for parent in parents {
        if user_id == &parent.user_id {
            panic!("User id {:?} is not unique", user_id);
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

pub fn get_width(width: Option<f32>, width_fill: bool)-> Length {
    
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

pub fn get_height(height: Option<f32>, height_fill: bool)-> Length {
    
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

pub fn get_length(length: Option<f32>, length_fill: bool)-> Length {
    
    match length 
            {
                Some(wd) => Length::Fixed(wd),
                None => {
                    let wd = 
                        match length_fill {
                            true => Length::Fill,
                            false => Length::Shrink,
                        };
                    wd
                },
            }
}

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


pub fn get_alignment (align: &str)-> Alignment {
    match align {
        "start" => Alignment::Start,
        "center" => Alignment::Center,
        "end" => Alignment::End,
        _ => panic!("Incorrect alignment found")
        }
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

// Nedd to figure out pixel vs f32.
pub fn get_line_height(line_height: (String, f32)) -> LineHeight {
    match line_height.0.as_str() {
        "default" => LineHeight::default(),
        "relative" => LineHeight::Relative(line_height.1),
        _ => panic!("Line_height of {:?} is not correct, check the docs", line_height)
    }
}

pub fn get_scroll_direction(direction_str: Option<String>) -> Direction {

    let scroll = match direction_str {
        Some(direction_str) => match direction_str.as_str() {
                                            "vertical" => Direction::Vertical(Properties::default()),
                                            "horizontal" => Direction::Horizontal(Properties::default()),
                                            "both" => Direction::Both { vertical: Properties::default(), horizontal: Properties::default() },
                                            _ => panic!("Scroll direction must be either vertical, horizontal, or both")
                                        },
        None => Direction::default(),
    };

    scroll
}
