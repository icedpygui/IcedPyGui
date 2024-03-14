#![allow(unused)]
use crate::ipg_widgets::ipg_enums::{IpgWidgets, get_set_widget_data};
use crate::{access_state, access_callbacks};
use crate::app;

use iced::{Length, Element, Padding};
use iced::widget::text::{LineHeight, Shaping};
use iced::widget::{Column, Radio, Row};

use pyo3::{PyObject, Python};

#[derive(Debug, Clone)]
pub struct IpgRadio {
    pub id: usize,
    pub labels: Vec<String>,
    pub direction: RadioDirection,
    pub spacing: f32,
    pub padding: Padding,
    pub show: bool,
    pub user_data: Option<PyObject>,
    pub is_selected: Option<usize>,
    pub selected: Option<Choice>,

    pub width: Length,
    pub size: f32,
    pub text_spacing: f32,
    pub text_size: f32,
    pub text_line_height: LineHeight,
    pub text_shaping: Shaping,
    // pub font: Option<Font>,
    // pub style: <Renderer::Theme as StyleSheet>::Style,
    pub cb_name: Option<String>,
}

impl IpgRadio {
    pub fn new( 
        id: usize,
        labels: Vec<String>,
        direction: RadioDirection,
        spacing: f32,
        padding: Padding,
        show: bool,
        user_data: Option<PyObject>,
        is_selected: Option<usize>,
        
        width: Length,
        size: f32,
        text_spacing: f32,
        text_size: f32,
        text_line_height: LineHeight,
        text_shaping: Shaping,
        // font: Option<Font>,
        // style: <Renderer::Theme as StyleSheet>::Style,
        cb_name: Option<String>,
        ) -> Self {
        Self {
            id,
            labels,
            direction,
            spacing,
            padding,
            show,
            user_data,
            is_selected,
            selected: None,
            width,
            size,
            text_spacing,
            text_size,
            text_line_height,
            text_shaping,
            // font: None,
            // style: Default::default(),
            cb_name,
        }
    }
}

#[derive(Debug, Clone)]
pub enum RadioDirection {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone)]
pub enum RDMessage {
    RadioSelected(Choice),
}

// The number of radio buttons per group is based on the number of Choices.
// Therefore, they are currently limited to 26 per group, but can easily be extended
// to a greater number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice {
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
}

impl Choice {
    pub fn into_iter() -> core::array::IntoIter<Choice, 26> {
        [Choice::A, Choice::B, Choice::C, Choice::B, Choice::E, Choice::F, Choice::G, Choice::H, Choice::I, 
        Choice::J, Choice::K, Choice::L, Choice::M, Choice::N, Choice::O, Choice::P, Choice::Q, Choice::R, 
        Choice::S, Choice::T, Choice::U, Choice::V, Choice::W, Choice::X, Choice::Y, Choice::Z,
        ]
        .into_iter()
    }
}


pub fn construct_radio(radio: IpgRadio) -> Element<'static, app::Message> {

        let mut radio_elements = vec![];

        for (i, choice) in  Choice::into_iter().enumerate() {
            if i > radio.labels.len()-1 {break}
                radio_elements.push(Radio::new(radio.labels[i].clone(), 
                                                choice,
                                                radio.selected,
                                                RDMessage::RadioSelected
                                            )
                                            .width(radio.width)
                                            .size(radio.size)
                                            .spacing(radio.text_spacing)
                                            .text_size(radio.text_size)
                                            .text_line_height(radio.text_line_height)
                                            .text_shaping(radio.text_shaping)
                                            .into());
        }

        let rd: Element<RDMessage> = match radio.direction {
                    RadioDirection::Horizontal => Row::with_children(radio_elements)
                                                            .spacing(radio.spacing)
                                                            .padding(radio.padding)
                                                            .into(),
                    RadioDirection::Vertical => Column::with_children(radio_elements)
                                                            .spacing(radio.spacing)
                                                            .padding(radio.padding)
                                                            .into(),
        };


    rd.map(move |message| app::Message::Radio(radio.id, message))

}

pub fn radio_update(id: usize, message: RDMessage) {

    let selected_choice = match message {
        RDMessage::RadioSelected(selected) => selected,
    };
    let mut selected_index = 0;
    for (i, choice) in  Choice::into_iter().enumerate() {

        if choice == selected_choice {
            selected_index = i;
            break;
        }
    }

    let (cb_name, 
        user_data, 
        selected_label_opt,
        _) = get_set_widget_data(
                                id,
                                None,
                                None,
                                None,
                                Some(selected_choice)
                                );

    let event_name = "selected".to_string();

    let selected_label = match selected_label_opt {
        Some(l) => l,
        None => panic!("Selcted_label for radio not found id {id}"),
    };

    process_callback(id, 
                    event_name,
                    selected_index,
                    selected_label,
                    user_data,
                    cb_name
                    );

}


fn process_callback(id: usize,
                    event_name: String,
                    selected_index: usize,
                    selected_label: String, 
                    user_data: Option<PyObject>, 
                    cb_name: Option<String>) 
{
    if !cb_name.is_some() {return}

    let app_cbs = access_callbacks();

    let mut found_callback = None;

    for callback in app_cbs.callbacks.iter() {

        if id == callback.id && cb_name == callback.name {

        found_callback = match callback.cb.clone() {
            Some(cb) => Some(cb),
            None => {drop(app_cbs); panic!("Callback could not be found with id {}", id)},
        };
        break;
        }                   
    };
    drop(app_cbs);

    match found_callback {

    Some(cb) => Python::with_gil(|py| {
        
        if user_data.is_some() {
            cb.call1(py, 
                    (id.clone(),
                            event_name,
                            (selected_index, selected_label),
                            user_data
                            )).unwrap();
        } else {
            cb.call1(py, 
                    (
                        id.clone(), 
                        event_name,
                        (selected_index, selected_label),
                        )).unwrap();
        }      
    }),
    None => panic!("Radio callback could not be found"),
    };

}
