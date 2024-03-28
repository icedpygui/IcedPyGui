#![allow(unused)]

use crate::ipg_widgets::helpers::try_extract_boolean;
use crate::{access_state, access_callbacks};
use crate::app;
use super::helpers::{try_extract_i64, try_extract_i64_option, try_extract_string, try_extract_vec_str};
use super::ipg_enums::IpgWidgets;
use super::callbacks::{WidgetCallbackIn, 
                        WidgetCallbackOut, 
                        get_set_widget_callback_data};

use iced::{Length, Element, Padding};
use iced::widget::text::{LineHeight, Shaping};
use iced::widget::{Column, Radio, Row, Space};

use pyo3::{pyclass, PyObject, Python};


#[derive(Debug, Clone)]
pub struct IpgRadio {
    pub id: usize,
    pub labels: Vec<String>,
    pub group: String,
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
    pub group_index: usize,
}

impl IpgRadio {
    pub fn new( 
        id: usize,
        labels: Vec<String>,
        group: String,
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
        radio_index: usize,
        ) -> Self {
        Self {
            id,
            labels,
            group,
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
            group_index: radio_index,
        }
    }
}


#[derive(Debug, Clone)]
#[pyclass]
pub enum RadioDirection {
    Horizontal,
    Vertical,
}


#[derive(Debug, Clone)]
pub enum RDMessage {
    RadioSelected(Choice),
}



pub fn construct_radio(radio: IpgRadio) -> Element<'static, app::Message> {

    if !radio.show {
        return Space::new(0.0, 0.0).into()
    }

    let mut radio_elements = vec![];

    let choice_group = get_choice(radio.group_index);

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
        RDMessage::RadioSelected(choice) => {
            match choice {
                Choice::Choice0(ch) => {
                    wci.selected_index = Some(ch as usize);
                    wci.choice_index = Some(0);
                },
                Choice::Choice1(ch) => {
                    wci.selected_index = Some(ch as usize);
                    wci.choice_index = Some(0);
                },
                Choice::Choice2(_) => {
                    wci.selected_index = Some(ch as usize);
                    wci.choice_index = Some(0);
                },
                Choice::Choice3(_) => {
                    wci.selected_index = Some(ch as usize);
                    wci.choice_index = Some(0);
                },
                Choice::Choice4(_) => {
                    wci.selected_index = Some(ch as usize);
                    wci.choice_index = Some(0);
                },
                Choice::Choice5(_) => {
                    wci.selected_index = Some(ch as usize);
                    wci.choice_index = Some(0);
                },
                Choice::Choice6(_) => {
                    wci.selected_index = Some(ch as usize);
                    wci.choice_index = Some(0);
                },
                Choice::Choice7(_) => {
                    wci.selected_index = Some(ch as usize);
                    wci.choice_index = Some(0);
                },
                Choice::Choice8(_) => {
                    wci.selected_index = Some(ch as usize);
                    wci.choice_index = Some(0);
                },
                Choice::Choice9(_) => {
                    wci.selected_index = Some(ch as usize);
                    wci.choice_index = Some(0);
                },
                Choice::Choice10(_) => {
                    wci.selected_index = Some(ch as usize);
                    wci.choice_index = Some(0);
                },
                Choice::Choice11(_) => {
                    wci.selected_index = Some(ch as usize);
                    wci.choice_index = Some(0);
                },
                Choice::Choice12(_) => {
                    wci.selected_index = Some(ch as usize);
                    wci.choice_index = Some(0);
                },
                Choice::Choice13(_) => {
                    wci.selected_index = Some(ch as usize);
                    wci.choice_index = Some(0);
                },
                Choice::Choice14(_) => {
                    wci.selected_index = Some(ch as usize);
                    wci.choice_index = Some(0);
                },
                Choice::Choice15(_) => {
                    wci.selected_index = Some(ch as usize);
                    wci.choice_index = Some(0);
                },
                Choice::Choice16(_) => {
                    wci.selected_index = Some(ch as usize);
                    wci.choice_index = Some(0);
                },
                Choice::Choice17(_) => {
                    wci.selected_index = Some(ch as usize);
                    wci.choice_index = Some(0);
                },
                Choice::Choice18(_) => {
                    wci.selected_index = Some(ch as usize);
                    wci.choice_index = Some(0);
                },
                Choice::Choice19(_) => {
                    wci.selected_index = Some(ch as usize);
                    wci.choice_index = Some(0);
                },
                Choice::Choice20(_) => {
                    wci.selected_index = Some(ch as usize);
                    wci.choice_index = Some(0);
                },
                Choice::Choice21(_) => {
                    wci.selected_index = Some(ch as usize);
                    wci.choice_index = Some(0);
                },
                Choice::Choice22(_) => {
                    wci.selected_index = Some(ch as usize);
                    wci.choice_index = Some(0);
                },
                Choice::Choice23(_) => {
                    wci.selected_index = Some(ch as usize);
                    wci.choice_index = Some(0);
                },
                Choice::Choice24(_) => {
                    wci.selected_index = Some(ch as usize);
                    wci.choice_index = Some(0);
                },
            }
        },
    }

    let mut wco = get_set_widget_callback_data(wci);
    wco.id = id;
    wco.event_name = "on_select".to_string();
    process_callback(wco);
    
}


fn process_callback(wco: WidgetCallbackOut) 
{
    let app_cbs = access_callbacks();

    let callback_present = app_cbs.callbacks.get(&(wco.id, wco.event_name.clone()));

    let callback_opt = match callback_present {
        Some(cb) => cb,
        None => return,
    };

    let callback = match callback_opt {
        Some(cb) => cb,
        None => panic!("Radio Callback could not be found with id {}", wco.id),
    };

    let index = match wco.selected_index {
        Some(idx) => idx,
        None => panic!("Radio callback selected_index could not be found"),
    };

    let label = match wco.selected_label {
        Some(lb) => lb,
        None => panic!("Radio callback selected_label could not be found"),
    };

    Python::with_gil(|py| {
        if wco.user_data.is_some() {
            let user_data = match wco.user_data {
                Some(ud) => ud,
                None => panic!("Radio callback user_data not found."),
            };
            let res = callback.call1(py, (
                                                                wco.id.clone(), 
                                                                (index, label),
                                                                user_data
                                                                ));
            match res {
                Ok(_) => (),
                Err(_) => panic!("Radio: 3 parameters (id, value, user_data) are required or possibly a non-fatal python error in this function."),
            }
        } else {
            let res = callback.call1(py, (
                                    wco.id.clone(),
                                    (index, label), 
                                    )
                            );
            match res {
                Ok(_) => (),
                Err(_) => panic!("Radio: 2 parameters (id, value) are required or possibly a non-fatal python error in this function."),
            }
        } 
    });

    drop(app_cbs);

}


#[derive(Debug, Clone)]
#[pyclass]
pub enum RadioParams {
    Direction,
    Labels,
    Padding,
    SelectedIndex,
    Show,
    Size,
    Spacing,
    TextSpacing,
    TextSize,
    TextLineHeight,
    UserData,
    Width,
    WidthFill,
}


pub fn radio_item_update(rd: &mut IpgRadio,
                            item: PyObject,
                            value: PyObject,
                            )
{
    let update = try_extract_radio_update(item);

    match update {
        RadioParams::Direction => {
            rd.direction = try_extract_radio_direction(value);
        },
        RadioParams::Labels => {
            rd.labels = try_extract_vec_str(value);
        },
        RadioParams::Padding => {

        },
        RadioParams::SelectedIndex => {

            let index_opt = try_extract_i64_option(value);

            let selected_index = match index_opt {
                Some(index)  => index as usize,
                None => {
                    rd.selected = None;
                    return
                },
            };
            
            if selected_index > rd.labels.len()-1 {
                panic!("Radio selected_index is greater than the size of the labels")
            } else {
                for (i, choice) in Choice::into_iter().enumerate() {
                    if i == selected_index {
                        rd.selected = Some(choice);
                        break;
                    }
                }
            }
        },
        RadioParams::Show => {

        },
        RadioParams::Size => {

        },
        RadioParams::Spacing => {

        },
        RadioParams::TextSpacing => {

        },
        RadioParams::TextSize => {

        },
        RadioParams::TextLineHeight => {

        },
        RadioParams::UserData => {

        },
        RadioParams::Width => {

        },
        RadioParams::WidthFill => {

        },
    }

}


pub fn try_extract_radio_update(update_obj: PyObject) -> RadioParams {

    Python::with_gil(|py| {
        let res = update_obj.extract::<RadioParams>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Radio update extraction failed"),
        }
    })
}


pub fn try_extract_radio_direction(direct_obj: PyObject) -> RadioDirection {
    Python::with_gil(|py| {
        let res = direct_obj.extract::<RadioDirection>(py);
            
        match res {
            Ok(direction) => direction,
            Err(_) => panic!("RadioDirection failed to extract."),
        }
    })  
}

 
// The number of radio buttons per group is based on the number of Choices.
// Therefore, they are currently limited to 26 per group, but can easily be extended
// to a greater number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice {
    Choice0(Choice0),
    Choice1(Choice1),
    Choice2(Choice2),
    Choice3(Choice3),
    Choice4(Choice4),
    Choice5(Choice5),
    Choice6(Choice6),
    Choice7(Choice7),
    Choice8(Choice8),
    Choice9(Choice9),
    Choice10(Choice10),
    Choice11(Choice11),
    Choice12(Choice12),
    Choice13(Choice13),
    Choice14(Choice14),
    Choice15(Choice15),
    Choice16(Choice16),
    Choice17(Choice17),
    Choice18(Choice18),
    Choice19(Choice19),
    Choice20(Choice20),
    Choice21(Choice21),
    Choice22(Choice22),
    Choice23(Choice23),
    Choice24(Choice24),
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Choice0 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
            P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Choice1 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
            P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Choice2 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
            P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Choice3 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
            P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Choice4 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
            P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Choice5 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
            P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Choice6 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
    P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Choice7 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
    P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Choice8 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
    P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Choice9 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
    P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Choice10 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
    P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Choice11 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
    P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Choice12 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
    P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Choice13 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
    P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Choice14 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
    P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Choice15 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
    P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Choice16 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
    P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Choice17 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
    P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Choice18 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
    P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Choice19 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
    P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Choice20 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
    P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Choice21 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
    P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Choice22 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
    P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Choice23 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
    P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Choice24 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
    P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}


static CHOICE0: Vec<Choice0> = vec![Choice0::A, Choice0::B, Choice0::C, Choice0::D, Choice0::E, Choice0::F, 
                                    Choice0::G, Choice0::H, Choice0::I, Choice0::J, Choice0::K, Choice0::L, 
                                    Choice0::M, Choice0::N, Choice0::O, Choice0::P, Choice0::Q, Choice0::R, 
                                    Choice0::S, Choice0::T, Choice0::U, Choice0::V, Choice0::W, Choice0::X, 
                                    Choice0::Y, Choice0::Z];
static CHOICE1: Vec<Choice1> = vec![Choice1::A, Choice1::B, Choice1::C, Choice1::D, Choice1::E, Choice1::F, 
                                    Choice1::G, Choice1::H, Choice1::I, Choice1::J, Choice1::K, Choice1::L, 
                                    Choice1::M, Choice1::N, Choice1::O, Choice1::P, Choice1::Q, Choice1::R, 
                                    Choice1::S, Choice1::T, Choice1::U, Choice1::V, Choice1::W, Choice1::X, 
                                    Choice1::Y, Choice1::Z];
