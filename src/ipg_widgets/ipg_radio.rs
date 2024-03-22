#![allow(unused)]

use crate::{access_state, access_callbacks};
use crate::app;
use super::ipg_enums::IpgWidgets;
use super::callbacks::{WidgetCallbackIn, 
                        WidgetCallbackOut, 
                        get_set_widget_callback_data};

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

pub fn radio_callback(id: usize, message: RDMessage) {

    let mut wci: WidgetCallbackIn = WidgetCallbackIn::default();
    wci.id = id;

    match message {
        RDMessage::RadioSelected(selected) => {
            wci.choice = Some(selected);
        },
    }
    
    let mut wco = get_set_widget_callback_data(wci);

    let selected_label = match wco.selected_label {
        Some(ref lb) => lb,
        None => panic!("Selected_label for radio not found id {}", wco.id),
    };
    wco.id = id;
    wco.event_name = Some("on_select".to_string());
    process_callback(wco);

}


pub fn process_callback(wco: WidgetCallbackOut) 
{
    if !wco.event_name.is_some() {return}

    let evt_name = match wco.event_name {
        Some(name) => name,
        None => panic!("event_name not found")
    };

    let app_cbs = access_callbacks();

    let callback_opt = app_cbs.callbacks.get(&(wco.id, evt_name.clone())).unwrap();
       
    let callback = match callback_opt {
        Some(cb) => cb,
        None => panic!("Callback could not be found with id {}", wco.id),
    };
                  
    Python::with_gil(|py| {
            if wco.user_data.is_some() {
                callback.call1(py, (
                                        wco.id.clone(), 
                                        evt_name.clone(),
                                        (wco.selected_index, wco.selected_label),
                                        wco.user_data
                                        )
                                ).unwrap();
            } else {
                callback.call1(py, (
                                        wco.id.clone(),
                                        evt_name.clone(),
                                        (wco.selected_index, wco.selected_label), 
                                        )
                                ).unwrap();
            } 
    });

    drop(app_cbs);

}
