//!ipg_radio

use crate::ipg_widgets::helpers::try_extract_boolean;
use crate::{access_state, access_callbacks};
use crate::app;
use super::helpers::{get_height, get_padding_f64, 
    get_width, try_extract_f64, try_extract_f64_option,
    try_extract_i64_option, try_extract_option_string, try_extract_u16, 
    try_extract_vec_f64, try_extract_vec_str};
use super::ipg_enums::IpgWidgets;
use super::callbacks::WidgetCallbackOut;


use iced::widget::radio::{self, Status};
use iced::{Color, Element, Length, Padding, Pixels, Theme};
use iced::widget::text::{LineHeight, Shaping};
use iced::widget::{Column, Radio, Row, Space};

use pyo3::{pyclass, PyObject, Python};


#[derive(Debug, Clone)]
pub struct IpgRadio {
    pub id: usize,
    pub labels: Vec<String>,
    pub direction: IpgRadioDirection,
    pub spacing: f32,
    pub padding: Padding,
    pub show: bool,
    pub user_data: Option<PyObject>,
    pub is_selected: Option<usize>,
    pub width: Length,
    pub height: Length,
    pub size: f32,
    pub text_spacing: f32,
    pub text_size: f32,
    pub text_line_height: LineHeight,
    pub text_shaping: Shaping,
    pub group_index: usize,
    // pub font: Option<Font>,
    pub style_id: Option<String>,
}

impl IpgRadio {
    pub fn new( 
        id: usize,
        labels: Vec<String>,
        direction: IpgRadioDirection,
        spacing: f32,
        padding: Padding,
        show: bool,
        user_data: Option<PyObject>,
        is_selected: Option<usize>,
        width: Length,
        height: Length,
        size: f32,
        text_spacing: f32,
        text_size: f32,
        text_line_height: LineHeight,
        text_shaping: Shaping,
        radio_index: usize,
        // font: Option<Font>,
        style_id: Option<String>,
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
            width,
            height,
            size,
            text_spacing,
            text_size,
            text_line_height,
            text_shaping,
            group_index: radio_index,
            // font: None,
            style_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IpgRadioStyle {
    pub id: usize,
    pub background_color: Option<Color>,
    pub background_color_hovered: Option<Color>,
    pub dot_color: Option<Color>,
    pub dot_color_hovered: Option<Color>,
    pub border_color: Option<Color>,
    pub border_width: Option<f32>,
    pub text_color: Option<Color>,
}

impl IpgRadioStyle {
    pub fn new(
        id: usize,
        background_color: Option<Color>,
        background_color_hovered: Option<Color>,
        dot_color: Option<Color>,
        dot_color_hovered: Option<Color>,
        border_color: Option<Color>,
        border_width: Option<f32>,
        text_color: Option<Color>,
    ) -> Self {
        Self {
            id,
            background_color,
            background_color_hovered,
            dot_color,
            dot_color_hovered,
            border_color,
            border_width,
            text_color,
        }
    }
}

#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgRadioDirection {
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

    let selected = match radio.is_selected {
        Some(is) => Some(CHOICES[radio.group_index][is]),
        None => None,
    };

    let mut radio_elements = vec![];

    // Due to the closure in the style, had to covert to array of strings
    let mut style: Vec<Option<String>> = vec![];
    for _ in 0..radio.labels.len() {
        style.push(radio.style_id.clone())
    }

    for (i, label) in  radio.labels.iter().enumerate() {

        let style_id: Option<String> = if style[i].is_some() {
            style[i].clone()
        } else {
            None
        };
        
        radio_elements.push(Radio::new(label.clone(), 
                                        CHOICES[radio.group_index][i],
                                        selected,
                                        RDMessage::RadioSelected
                                    )
                                    .size(radio.size)
                                    .spacing(radio.text_spacing)
                                    .text_size(radio.text_size)
                                    .text_line_height(radio.text_line_height)
                                    .text_shaping(radio.text_shaping)
                                    .style(move|theme: &Theme, status| {
                                        get_styling(theme, status, 
                                        style_id.clone(),
                                        )})
                                    .into());
    }

    let rd: Element<RDMessage> = match radio.direction {
            IpgRadioDirection::Horizontal => Row::with_children(radio_elements)
                                                    .spacing(radio.spacing)
                                                    .padding(radio.padding)
                                                    .width(radio.width)
                                                    .height(radio.height)
                                                    .into(),
            IpgRadioDirection::Vertical => Column::with_children(radio_elements)
                                                    .spacing(radio.spacing)
                                                    .padding(radio.padding)
                                                    .width(radio.width)
                                                    .height(radio.height)
                                                    .into(),
    };

    rd.map(move |message| app::Message::Radio(radio.id, message))

}


pub fn radio_callback(id: usize, message: RDMessage) {

    let mut wco = WidgetCallbackOut::default();

    let mut state = access_state();
    let widget_opt = state.widgets.get_mut(&id);

    let widgets = match widget_opt {
        Some(rd) => rd,
        None => panic!("Radio callback with id {} could not be found", id),
    };

    let radio: &mut IpgRadio = match_widgets(widgets);

    let ch_usize = match message {
        RDMessage::RadioSelected(choice) => {
            match choice {
                Choice::Choice0(ch) => {
                    ch as usize
                },
                Choice::Choice1(ch) => {
                    ch as usize
                },
                Choice::Choice2(ch) => {
                    ch as usize
                },
                Choice::Choice3(ch) => {
                    ch as usize
                },
                Choice::Choice4(ch) => {
                    ch as usize
                },
                Choice::Choice5(ch) => {
                    ch as usize
                },
                Choice::Choice6(ch) => {
                    ch as usize
                },
                Choice::Choice7(ch) => {
                    ch as usize
                },
                Choice::Choice8(ch) => {
                    ch as usize
                },
                Choice::Choice9(ch) => {
                    ch as usize
                },
                Choice::Choice10(ch) => {
                    ch as usize
                },
                Choice::Choice11(ch) => {
                    ch as usize
                },
                Choice::Choice12(ch) => {
                    ch as usize
                },
                Choice::Choice13(ch) => {
                    ch as usize
                },
                Choice::Choice14(ch) => {
                    ch as usize
                },
                Choice::Choice15(ch) => {
                    ch as usize
                },
                Choice::Choice16(ch) => {
                    ch as usize
                },
                Choice::Choice17(ch) => {
                    ch as usize
                },
                Choice::Choice18(ch) => {
                    ch as usize
                },
                Choice::Choice19(ch) => {
                    ch as usize
                },
                Choice::Choice20(ch) => {
                    ch as usize
                },
                Choice::Choice21(ch) => {
                    ch as usize
                },
                Choice::Choice22(ch) => {
                    ch as usize
                },
                Choice::Choice23(ch) => {
                    ch as usize
                },
                Choice::Choice24(ch) => {
                    ch as usize
                },
                Choice::Choice25(ch) => {
                    ch as usize
                },
            }
        }, 
        
    };

    radio.is_selected = Some(ch_usize);
    wco.user_data = radio.user_data.clone();
    wco.selected_label = Some(radio.labels[ch_usize].clone());
    wco.selected_index = Some(ch_usize);
    drop(state);

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
                Err(er) => panic!("Radio: 3 parameters (id, value, user_data) are required or a python error in this function. {er}"),
            }
        } else {
            let res = callback.call1(py, (
                                    wco.id.clone(),
                                    (index, label), 
                                    )
                            );
            match res {
                Ok(_) => (),
                Err(er) => panic!("Radio: 2 parameters (id, value) are required or a python error in this function. {er}"),
            }
        } 
    });

    drop(app_cbs);

}


fn match_widgets (widget: &mut IpgWidgets) -> &mut IpgRadio {
    
    match widget {
        IpgWidgets::IpgRadio(radio) => return radio,
        _ => panic!("Radio expected to find radio in IpgWidgets")
    }
}

#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgRadioParam {
    Direction,
    Labels,
    Padding,
    SelectedIndex,
    Show,
    Size,
    Spacing,
    StyleId,
    TextSpacing,
    TextSize,
    LineHeightPixels,
    LineHeightRelative,
    UserData,
    Width,
    WidthFill,
    Height,
    HeightFill,
}


pub fn radio_item_update(rd: &mut IpgRadio,
                            item: PyObject,
                            value: PyObject,
                            )
{
    let update = try_extract_radio_update(item);

    match update {
        IpgRadioParam::Direction => {
            rd.direction = try_extract_radio_direction(value);
        },
        IpgRadioParam::Labels => {
            rd.labels = try_extract_vec_str(value);
        },
        IpgRadioParam::Padding => {
            let val = try_extract_vec_f64(value);
            rd.padding =  get_padding_f64(val);
        },
        IpgRadioParam::SelectedIndex => {
            let index_opt = try_extract_i64_option(value);

            let selected_index = match index_opt {
                Some(index)  => index as usize,
                None => {
                    rd.is_selected = None;
                    return
                }
            };
            
            if selected_index > rd.labels.len()-1 {
                panic!("Radio selected_index is greater than the size of the labels")
            } else {
                rd.is_selected = Some(selected_index);
            }
        },
        IpgRadioParam::Show => {
            rd.show = try_extract_boolean(value);
        },
        IpgRadioParam::Size => {
            rd.size = try_extract_f64(value) as f32;
        },
        IpgRadioParam::Spacing => {
            rd.spacing = try_extract_f64(value) as f32;
        },
        IpgRadioParam::StyleId => {
            rd.style_id = try_extract_option_string(value);
        },
        IpgRadioParam::TextSpacing => {
            rd.text_spacing = try_extract_f64(value) as f32;
        },
        IpgRadioParam::TextSize => {
            rd.text_size = try_extract_f64(value) as f32;
        },
        IpgRadioParam::LineHeightPixels => {
            let val = try_extract_u16(value);
            rd.text_line_height = LineHeight::Absolute(Pixels(val.into()));
        },
        IpgRadioParam::LineHeightRelative => {
            let val = try_extract_f64(value) as f32;
            rd.text_line_height = LineHeight::Relative(val);
        },
        IpgRadioParam::UserData => {
            rd.user_data = Some(value);
        },
        IpgRadioParam::Width => {
            match try_extract_f64_option(value) {
                Some(val) => rd.width = get_width(Some(val as f32), false),
                None => rd.width = Length::Shrink,
            }
        },
        IpgRadioParam::WidthFill => {
            let val = try_extract_boolean(value);
            if val {
                rd.width = get_width(None, val);
            } else {
                rd.width = Length::Shrink;
            }
        },
        IpgRadioParam::Height => {
            match try_extract_f64_option(value) {
                Some(val) => rd.height = get_height(Some(val as f32), false),
                None => rd.height = Length::Shrink,
            }
        },
        IpgRadioParam::HeightFill => {
            let val = try_extract_boolean(value);
            if val {
                rd.height = get_height(None, val);
            } else {
                rd.height = Length::Shrink;
            } 
        },
    }

}


pub fn try_extract_radio_update(update_obj: PyObject) -> IpgRadioParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgRadioParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Radio update extraction failed"),
        }
    })
}


pub fn try_extract_radio_direction(direct_obj: PyObject) -> IpgRadioDirection {
    Python::with_gil(|py| {
        let res = direct_obj.extract::<IpgRadioDirection>(py);
            
        match res {
            Ok(direction) => direction,
            Err(_) => panic!("RadioDirection failed to extract."),
        }
    })  
}

pub fn get_styling(theme: &Theme, status: Status, 
                    style_id: Option<String>,
                    ) -> radio::Style {
    
    if style_id.is_none() {
        return radio::default(theme, status)
    }
    
    let mut base_style = radio::default(theme, status);

    let state = access_state();

    let style_opt = state.radio_style.get(&style_id.clone().unwrap());
    
    let style = match style_opt {
        Some(st) => st,
        None => panic!("Radio: The style_id '{}' for add_radio_style could not be found", style_id.unwrap())
    };

    base_style.text_color = style.text_color;
    
    if style.background_color.is_some() {
        base_style.background = style.background_color.unwrap().into();
    }

    if style.dot_color.is_some() {
        base_style.dot_color = style.dot_color.unwrap();
    }
    
    // border color changes to inner color during hover
    if style.border_color.is_some() {
        base_style.border_color = style.border_color.unwrap();
    }
    
    if style.border_width.is_some() {
        base_style.border_width = style.border_width.unwrap();
    }
        

    match status {
        Status::Active{..} => base_style,
        Status::Hovered{..} => {
            if style.background_color_hovered.is_some() {
                base_style.background = style.background_color_hovered.unwrap().into();
            }
            if style.dot_color_hovered.is_some() {
                base_style.dot_color = style.dot_color_hovered.unwrap();
            }
            base_style
        },
    }

}


// The number of radio buttons per group is based on the number of Choices.
// Therefore, they are currently limited to 26 per group, but can easily be extended
// to a greater number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice {
    Choice0(Choice0), Choice1(Choice1), Choice2(Choice2), Choice3(Choice3), Choice4(Choice4), 
    Choice5(Choice5), Choice6(Choice6), Choice7(Choice7), Choice8(Choice8), Choice9(Choice9), 
    Choice10(Choice10), Choice11(Choice11), Choice12(Choice12), Choice13(Choice13), Choice14(Choice14), 
    Choice15(Choice15), Choice16(Choice16), Choice17(Choice17), Choice18(Choice18), Choice19(Choice19), 
    Choice20(Choice20), Choice21(Choice21), Choice22(Choice22), Choice23(Choice23), Choice24(Choice24), 
    Choice25(Choice25),
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice0 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
            P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice1 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
            P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice2 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
            P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice3 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
            P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice4 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
            P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice5 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
            P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice6 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
    P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice7 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
    P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice8 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
    P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice9 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
    P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice10 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
    P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice11 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
    P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice12 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
    P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice13 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
    P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice14 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
    P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice15 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
    P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice16 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
    P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice17 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
    P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice18 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
    P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice19 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
    P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice20 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
    P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice21 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
    P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice22 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
    P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice23 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
    P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice24 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
    P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice25 {A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7, I=8, J=9, K=10, L=11, M=12, N=13, O=14, 
    P=15, Q=16, R=17, S=18, T=19, U=20, V=21, W=22, X=23, Y=24, Z=25,}

static CHOICE0: [Choice; 26] = [Choice::Choice0(Choice0::A), Choice::Choice0(Choice0::B), 
                                Choice::Choice0(Choice0::C), Choice::Choice0(Choice0::D), 
                                Choice::Choice0(Choice0::E), Choice::Choice0(Choice0::F), 
                                Choice::Choice0(Choice0::G), Choice::Choice0(Choice0::H), 
                                Choice::Choice0(Choice0::I), Choice::Choice0(Choice0::J), 
                                Choice::Choice0(Choice0::K), Choice::Choice0(Choice0::L), 
                                Choice::Choice0(Choice0::M), Choice::Choice0(Choice0::N), 
                                Choice::Choice0(Choice0::O), Choice::Choice0(Choice0::P), 
                                Choice::Choice0(Choice0::Q), Choice::Choice0(Choice0::R), 
                                Choice::Choice0(Choice0::S), Choice::Choice0(Choice0::T), 
                                Choice::Choice0(Choice0::U), Choice::Choice0(Choice0::V),
                                Choice::Choice0(Choice0::W), Choice::Choice0(Choice0::X), 
                                Choice::Choice0(Choice0::Y), Choice::Choice0(Choice0::Z)];

static CHOICE1: [Choice; 26] = [Choice::Choice1(Choice1::A), Choice::Choice1(Choice1::B), 
                                Choice::Choice1(Choice1::C), Choice::Choice1(Choice1::D), 
                                Choice::Choice1(Choice1::E), Choice::Choice1(Choice1::F), 
                                Choice::Choice1(Choice1::G), Choice::Choice1(Choice1::H), 
                                Choice::Choice1(Choice1::I), Choice::Choice1(Choice1::J), 
                                Choice::Choice1(Choice1::K), Choice::Choice1(Choice1::L), 
                                Choice::Choice1(Choice1::M), Choice::Choice1(Choice1::N), 
                                Choice::Choice1(Choice1::O), Choice::Choice1(Choice1::P), 
                                Choice::Choice1(Choice1::Q), Choice::Choice1(Choice1::R), 
                                Choice::Choice1(Choice1::S), Choice::Choice1(Choice1::T), 
                                Choice::Choice1(Choice1::U), Choice::Choice1(Choice1::V),
                                Choice::Choice1(Choice1::W), Choice::Choice1(Choice1::X), 
                                Choice::Choice1(Choice1::Y), Choice::Choice1(Choice1::Z)];

static CHOICE2: [Choice; 26] = [Choice::Choice2(Choice2::A), Choice::Choice2(Choice2::B), 
                                Choice::Choice2(Choice2::C), Choice::Choice2(Choice2::D), 
                                Choice::Choice2(Choice2::E), Choice::Choice2(Choice2::F), 
                                Choice::Choice2(Choice2::G), Choice::Choice2(Choice2::H), 
                                Choice::Choice2(Choice2::I), Choice::Choice2(Choice2::J), 
                                Choice::Choice2(Choice2::K), Choice::Choice2(Choice2::L), 
                                Choice::Choice2(Choice2::M), Choice::Choice2(Choice2::N), 
                                Choice::Choice2(Choice2::O), Choice::Choice2(Choice2::P), 
                                Choice::Choice2(Choice2::Q), Choice::Choice2(Choice2::R), 
                                Choice::Choice2(Choice2::S), Choice::Choice2(Choice2::T), 
                                Choice::Choice2(Choice2::U), Choice::Choice2(Choice2::V),
                                Choice::Choice2(Choice2::W), Choice::Choice2(Choice2::X), 
                                Choice::Choice2(Choice2::Y), Choice::Choice2(Choice2::Z)];

static CHOICE3: [Choice; 26] = [Choice::Choice3(Choice3::A), Choice::Choice3(Choice3::B), 
                                Choice::Choice3(Choice3::C), Choice::Choice3(Choice3::D), 
                                Choice::Choice3(Choice3::E), Choice::Choice3(Choice3::F), 
                                Choice::Choice3(Choice3::G), Choice::Choice3(Choice3::H), 
                                Choice::Choice3(Choice3::I), Choice::Choice3(Choice3::J), 
                                Choice::Choice3(Choice3::K), Choice::Choice3(Choice3::L), 
                                Choice::Choice3(Choice3::M), Choice::Choice3(Choice3::N), 
                                Choice::Choice3(Choice3::O), Choice::Choice3(Choice3::P), 
                                Choice::Choice3(Choice3::Q), Choice::Choice3(Choice3::R), 
                                Choice::Choice3(Choice3::S), Choice::Choice3(Choice3::T), 
                                Choice::Choice3(Choice3::U), Choice::Choice3(Choice3::V),
                                Choice::Choice3(Choice3::W), Choice::Choice3(Choice3::X), 
                                Choice::Choice3(Choice3::Y), Choice::Choice3(Choice3::Z)];

static CHOICE4: [Choice; 26] = [Choice::Choice4(Choice4::A), Choice::Choice4(Choice4::B), 
                                Choice::Choice4(Choice4::C), Choice::Choice4(Choice4::D), 
                                Choice::Choice4(Choice4::E), Choice::Choice4(Choice4::F), 
                                Choice::Choice4(Choice4::G), Choice::Choice4(Choice4::H), 
                                Choice::Choice4(Choice4::I), Choice::Choice4(Choice4::J), 
                                Choice::Choice4(Choice4::K), Choice::Choice4(Choice4::L), 
                                Choice::Choice4(Choice4::M), Choice::Choice4(Choice4::N), 
                                Choice::Choice4(Choice4::O), Choice::Choice4(Choice4::P), 
                                Choice::Choice4(Choice4::Q), Choice::Choice4(Choice4::R), 
                                Choice::Choice4(Choice4::S), Choice::Choice4(Choice4::T), 
                                Choice::Choice4(Choice4::U), Choice::Choice4(Choice4::V),
                                Choice::Choice4(Choice4::W), Choice::Choice4(Choice4::X), 
                                Choice::Choice4(Choice4::Y), Choice::Choice4(Choice4::Z)];

static CHOICE5: [Choice; 26] = [Choice::Choice5(Choice5::A), Choice::Choice5(Choice5::B), 
                                Choice::Choice5(Choice5::C), Choice::Choice5(Choice5::D), 
                                Choice::Choice5(Choice5::E), Choice::Choice5(Choice5::F), 
                                Choice::Choice5(Choice5::G), Choice::Choice5(Choice5::H), 
                                Choice::Choice5(Choice5::I), Choice::Choice5(Choice5::J), 
                                Choice::Choice5(Choice5::K), Choice::Choice5(Choice5::L), 
                                Choice::Choice5(Choice5::M), Choice::Choice5(Choice5::N), 
                                Choice::Choice5(Choice5::O), Choice::Choice5(Choice5::P), 
                                Choice::Choice5(Choice5::Q), Choice::Choice5(Choice5::R), 
                                Choice::Choice5(Choice5::S), Choice::Choice5(Choice5::T), 
                                Choice::Choice5(Choice5::U), Choice::Choice5(Choice5::V),
                                Choice::Choice5(Choice5::W), Choice::Choice5(Choice5::X), 
                                Choice::Choice5(Choice5::Y), Choice::Choice5(Choice5::Z)];

static CHOICE6: [Choice; 26] = [Choice::Choice6(Choice6::A), Choice::Choice6(Choice6::B), 
                                Choice::Choice6(Choice6::C), Choice::Choice6(Choice6::D), 
                                Choice::Choice6(Choice6::E), Choice::Choice6(Choice6::F), 
                                Choice::Choice6(Choice6::G), Choice::Choice6(Choice6::H), 
                                Choice::Choice6(Choice6::I), Choice::Choice6(Choice6::J), 
                                Choice::Choice6(Choice6::K), Choice::Choice6(Choice6::L), 
                                Choice::Choice6(Choice6::M), Choice::Choice6(Choice6::N), 
                                Choice::Choice6(Choice6::O), Choice::Choice6(Choice6::P), 
                                Choice::Choice6(Choice6::Q), Choice::Choice6(Choice6::R), 
                                Choice::Choice6(Choice6::S), Choice::Choice6(Choice6::T), 
                                Choice::Choice6(Choice6::U), Choice::Choice6(Choice6::V),
                                Choice::Choice6(Choice6::W), Choice::Choice6(Choice6::X), 
                                Choice::Choice6(Choice6::Y), Choice::Choice6(Choice6::Z)];

static CHOICE7: [Choice; 26] = [Choice::Choice7(Choice7::A), Choice::Choice7(Choice7::B), 
                                Choice::Choice7(Choice7::C), Choice::Choice7(Choice7::D), 
                                Choice::Choice7(Choice7::E), Choice::Choice7(Choice7::F), 
                                Choice::Choice7(Choice7::G), Choice::Choice7(Choice7::H), 
                                Choice::Choice7(Choice7::I), Choice::Choice7(Choice7::J), 
                                Choice::Choice7(Choice7::K), Choice::Choice7(Choice7::L), 
                                Choice::Choice7(Choice7::M), Choice::Choice7(Choice7::N), 
                                Choice::Choice7(Choice7::O), Choice::Choice7(Choice7::P), 
                                Choice::Choice7(Choice7::Q), Choice::Choice7(Choice7::R), 
                                Choice::Choice7(Choice7::S), Choice::Choice7(Choice7::T), 
                                Choice::Choice7(Choice7::U), Choice::Choice7(Choice7::V),
                                Choice::Choice7(Choice7::W), Choice::Choice7(Choice7::X), 
                                Choice::Choice7(Choice7::Y), Choice::Choice7(Choice7::Z)];

static CHOICE8: [Choice; 26] = [Choice::Choice8(Choice8::A), Choice::Choice8(Choice8::B), 
                                Choice::Choice8(Choice8::C), Choice::Choice8(Choice8::D), 
                                Choice::Choice8(Choice8::E), Choice::Choice8(Choice8::F), 
                                Choice::Choice8(Choice8::G), Choice::Choice8(Choice8::H), 
                                Choice::Choice8(Choice8::I), Choice::Choice8(Choice8::J), 
                                Choice::Choice8(Choice8::K), Choice::Choice8(Choice8::L), 
                                Choice::Choice8(Choice8::M), Choice::Choice8(Choice8::N), 
                                Choice::Choice8(Choice8::O), Choice::Choice8(Choice8::P), 
                                Choice::Choice8(Choice8::Q), Choice::Choice8(Choice8::R), 
                                Choice::Choice8(Choice8::S), Choice::Choice8(Choice8::T), 
                                Choice::Choice8(Choice8::U), Choice::Choice8(Choice8::V),
                                Choice::Choice8(Choice8::W), Choice::Choice8(Choice8::X), 
                                Choice::Choice8(Choice8::Y), Choice::Choice8(Choice8::Z)];

static CHOICE9: [Choice; 26] = [Choice::Choice9(Choice9::A), Choice::Choice9(Choice9::B), 
                                Choice::Choice9(Choice9::C), Choice::Choice9(Choice9::D), 
                                Choice::Choice9(Choice9::E), Choice::Choice9(Choice9::F), 
                                Choice::Choice9(Choice9::G), Choice::Choice9(Choice9::H), 
                                Choice::Choice9(Choice9::I), Choice::Choice9(Choice9::J), 
                                Choice::Choice9(Choice9::K), Choice::Choice9(Choice9::L), 
                                Choice::Choice9(Choice9::M), Choice::Choice9(Choice9::N), 
                                Choice::Choice9(Choice9::O), Choice::Choice9(Choice9::P), 
                                Choice::Choice9(Choice9::Q), Choice::Choice9(Choice9::R), 
                                Choice::Choice9(Choice9::S), Choice::Choice9(Choice9::T), 
                                Choice::Choice9(Choice9::U), Choice::Choice9(Choice9::V),
                                Choice::Choice9(Choice9::W), Choice::Choice9(Choice9::X), 
                                Choice::Choice9(Choice9::Y), Choice::Choice9(Choice9::Z)];

static CHOICE10: [Choice; 26] = [Choice::Choice10(Choice10::A), Choice::Choice10(Choice10::B), 
                                Choice::Choice10(Choice10::C), Choice::Choice10(Choice10::D), 
                                Choice::Choice10(Choice10::E), Choice::Choice10(Choice10::F), 
                                Choice::Choice10(Choice10::G), Choice::Choice10(Choice10::H), 
                                Choice::Choice10(Choice10::I), Choice::Choice10(Choice10::J), 
                                Choice::Choice10(Choice10::K), Choice::Choice10(Choice10::L), 
                                Choice::Choice10(Choice10::M), Choice::Choice10(Choice10::N), 
                                Choice::Choice10(Choice10::O), Choice::Choice10(Choice10::P), 
                                Choice::Choice10(Choice10::Q), Choice::Choice10(Choice10::R), 
                                Choice::Choice10(Choice10::S), Choice::Choice10(Choice10::T), 
                                Choice::Choice10(Choice10::U), Choice::Choice10(Choice10::V),
                                Choice::Choice10(Choice10::W), Choice::Choice10(Choice10::X), 
                                Choice::Choice10(Choice10::Y), Choice::Choice10(Choice10::Z)];

static CHOICE11: [Choice; 26] = [Choice::Choice11(Choice11::A), Choice::Choice11(Choice11::B), 
                                Choice::Choice11(Choice11::C), Choice::Choice11(Choice11::D), 
                                Choice::Choice11(Choice11::E), Choice::Choice11(Choice11::F), 
                                Choice::Choice11(Choice11::G), Choice::Choice11(Choice11::H), 
                                Choice::Choice11(Choice11::I), Choice::Choice11(Choice11::J), 
                                Choice::Choice11(Choice11::K), Choice::Choice11(Choice11::L), 
                                Choice::Choice11(Choice11::M), Choice::Choice11(Choice11::N), 
                                Choice::Choice11(Choice11::O), Choice::Choice11(Choice11::P), 
                                Choice::Choice11(Choice11::Q), Choice::Choice11(Choice11::R), 
                                Choice::Choice11(Choice11::S), Choice::Choice11(Choice11::T), 
                                Choice::Choice11(Choice11::U), Choice::Choice11(Choice11::V),
                                Choice::Choice11(Choice11::W), Choice::Choice11(Choice11::X), 
                                Choice::Choice11(Choice11::Y), Choice::Choice11(Choice11::Z)];

static CHOICE12: [Choice; 26] = [Choice::Choice12(Choice12::A), Choice::Choice12(Choice12::B), 
                                Choice::Choice12(Choice12::C), Choice::Choice12(Choice12::D), 
                                Choice::Choice12(Choice12::E), Choice::Choice12(Choice12::F), 
                                Choice::Choice12(Choice12::G), Choice::Choice12(Choice12::H), 
                                Choice::Choice12(Choice12::I), Choice::Choice12(Choice12::J), 
                                Choice::Choice12(Choice12::K), Choice::Choice12(Choice12::L), 
                                Choice::Choice12(Choice12::M), Choice::Choice12(Choice12::N), 
                                Choice::Choice12(Choice12::O), Choice::Choice12(Choice12::P), 
                                Choice::Choice12(Choice12::Q), Choice::Choice12(Choice12::R), 
                                Choice::Choice12(Choice12::S), Choice::Choice12(Choice12::T), 
                                Choice::Choice12(Choice12::U), Choice::Choice12(Choice12::V),
                                Choice::Choice12(Choice12::W), Choice::Choice12(Choice12::X), 
                                Choice::Choice12(Choice12::Y), Choice::Choice12(Choice12::Z)];

static CHOICE13: [Choice; 26] = [Choice::Choice13(Choice13::A), Choice::Choice13(Choice13::B), 
                                Choice::Choice13(Choice13::C), Choice::Choice13(Choice13::D), 
                                Choice::Choice13(Choice13::E), Choice::Choice13(Choice13::F), 
                                Choice::Choice13(Choice13::G), Choice::Choice13(Choice13::H), 
                                Choice::Choice13(Choice13::I), Choice::Choice13(Choice13::J), 
                                Choice::Choice13(Choice13::K), Choice::Choice13(Choice13::L), 
                                Choice::Choice13(Choice13::M), Choice::Choice13(Choice13::N), 
                                Choice::Choice13(Choice13::O), Choice::Choice13(Choice13::P), 
                                Choice::Choice13(Choice13::Q), Choice::Choice13(Choice13::R), 
                                Choice::Choice13(Choice13::S), Choice::Choice13(Choice13::T), 
                                Choice::Choice13(Choice13::U), Choice::Choice13(Choice13::V),
                                Choice::Choice13(Choice13::W), Choice::Choice13(Choice13::X), 
                                Choice::Choice13(Choice13::Y), Choice::Choice13(Choice13::Z)];

static CHOICE14: [Choice; 26] = [Choice::Choice14(Choice14::A), Choice::Choice14(Choice14::B), 
                                Choice::Choice14(Choice14::C), Choice::Choice14(Choice14::D), 
                                Choice::Choice14(Choice14::E), Choice::Choice14(Choice14::F), 
                                Choice::Choice14(Choice14::G), Choice::Choice14(Choice14::H), 
                                Choice::Choice14(Choice14::I), Choice::Choice14(Choice14::J), 
                                Choice::Choice14(Choice14::K), Choice::Choice14(Choice14::L), 
                                Choice::Choice14(Choice14::M), Choice::Choice14(Choice14::N), 
                                Choice::Choice14(Choice14::O), Choice::Choice14(Choice14::P), 
                                Choice::Choice14(Choice14::Q), Choice::Choice14(Choice14::R), 
                                Choice::Choice14(Choice14::S), Choice::Choice14(Choice14::T), 
                                Choice::Choice14(Choice14::U), Choice::Choice14(Choice14::V),
                                Choice::Choice14(Choice14::W), Choice::Choice14(Choice14::X), 
                                Choice::Choice14(Choice14::Y), Choice::Choice14(Choice14::Z)];

static CHOICE15: [Choice; 26] = [Choice::Choice15(Choice15::A), Choice::Choice15(Choice15::B), 
                                Choice::Choice15(Choice15::C), Choice::Choice15(Choice15::D), 
                                Choice::Choice15(Choice15::E), Choice::Choice15(Choice15::F), 
                                Choice::Choice15(Choice15::G), Choice::Choice15(Choice15::H), 
                                Choice::Choice15(Choice15::I), Choice::Choice15(Choice15::J), 
                                Choice::Choice15(Choice15::K), Choice::Choice15(Choice15::L), 
                                Choice::Choice15(Choice15::M), Choice::Choice15(Choice15::N), 
                                Choice::Choice15(Choice15::O), Choice::Choice15(Choice15::P), 
                                Choice::Choice15(Choice15::Q), Choice::Choice15(Choice15::R), 
                                Choice::Choice15(Choice15::S), Choice::Choice15(Choice15::T), 
                                Choice::Choice15(Choice15::U), Choice::Choice15(Choice15::V),
                                Choice::Choice15(Choice15::W), Choice::Choice15(Choice15::X), 
                                Choice::Choice15(Choice15::Y), Choice::Choice15(Choice15::Z)];

static CHOICE16: [Choice; 26] = [Choice::Choice16(Choice16::A), Choice::Choice16(Choice16::B), 
                                Choice::Choice16(Choice16::C), Choice::Choice16(Choice16::D), 
                                Choice::Choice16(Choice16::E), Choice::Choice16(Choice16::F), 
                                Choice::Choice16(Choice16::G), Choice::Choice16(Choice16::H), 
                                Choice::Choice16(Choice16::I), Choice::Choice16(Choice16::J), 
                                Choice::Choice16(Choice16::K), Choice::Choice16(Choice16::L), 
                                Choice::Choice16(Choice16::M), Choice::Choice16(Choice16::N), 
                                Choice::Choice16(Choice16::O), Choice::Choice16(Choice16::P), 
                                Choice::Choice16(Choice16::Q), Choice::Choice16(Choice16::R), 
                                Choice::Choice16(Choice16::S), Choice::Choice16(Choice16::T), 
                                Choice::Choice16(Choice16::U), Choice::Choice16(Choice16::V),
                                Choice::Choice16(Choice16::W), Choice::Choice16(Choice16::X), 
                                Choice::Choice16(Choice16::Y), Choice::Choice16(Choice16::Z)];

static CHOICE17: [Choice; 26] = [Choice::Choice17(Choice17::A), Choice::Choice17(Choice17::B), 
                                Choice::Choice17(Choice17::C), Choice::Choice17(Choice17::D), 
                                Choice::Choice17(Choice17::E), Choice::Choice17(Choice17::F), 
                                Choice::Choice17(Choice17::G), Choice::Choice17(Choice17::H), 
                                Choice::Choice17(Choice17::I), Choice::Choice17(Choice17::J), 
                                Choice::Choice17(Choice17::K), Choice::Choice17(Choice17::L), 
                                Choice::Choice17(Choice17::M), Choice::Choice17(Choice17::N), 
                                Choice::Choice17(Choice17::O), Choice::Choice17(Choice17::P), 
                                Choice::Choice17(Choice17::Q), Choice::Choice17(Choice17::R), 
                                Choice::Choice17(Choice17::S), Choice::Choice17(Choice17::T), 
                                Choice::Choice17(Choice17::U), Choice::Choice17(Choice17::V),
                                Choice::Choice17(Choice17::W), Choice::Choice17(Choice17::X), 
                                Choice::Choice17(Choice17::Y), Choice::Choice17(Choice17::Z)];

static CHOICE18: [Choice; 26] = [Choice::Choice18(Choice18::A), Choice::Choice18(Choice18::B), 
                                Choice::Choice18(Choice18::C), Choice::Choice18(Choice18::D), 
                                Choice::Choice18(Choice18::E), Choice::Choice18(Choice18::F), 
                                Choice::Choice18(Choice18::G), Choice::Choice18(Choice18::H), 
                                Choice::Choice18(Choice18::I), Choice::Choice18(Choice18::J), 
                                Choice::Choice18(Choice18::K), Choice::Choice18(Choice18::L), 
                                Choice::Choice18(Choice18::M), Choice::Choice18(Choice18::N), 
                                Choice::Choice18(Choice18::O), Choice::Choice18(Choice18::P), 
                                Choice::Choice18(Choice18::Q), Choice::Choice18(Choice18::R), 
                                Choice::Choice18(Choice18::S), Choice::Choice18(Choice18::T), 
                                Choice::Choice18(Choice18::U), Choice::Choice18(Choice18::V),
                                Choice::Choice18(Choice18::W), Choice::Choice18(Choice18::X), 
                                Choice::Choice18(Choice18::Y), Choice::Choice18(Choice18::Z)];

static CHOICE19: [Choice; 26] = [Choice::Choice19(Choice19::A), Choice::Choice19(Choice19::B), 
                                Choice::Choice19(Choice19::C), Choice::Choice19(Choice19::D), 
                                Choice::Choice19(Choice19::E), Choice::Choice19(Choice19::F), 
                                Choice::Choice19(Choice19::G), Choice::Choice19(Choice19::H), 
                                Choice::Choice19(Choice19::I), Choice::Choice19(Choice19::J), 
                                Choice::Choice19(Choice19::K), Choice::Choice19(Choice19::L), 
                                Choice::Choice19(Choice19::M), Choice::Choice19(Choice19::N), 
                                Choice::Choice19(Choice19::O), Choice::Choice19(Choice19::P), 
                                Choice::Choice19(Choice19::Q), Choice::Choice19(Choice19::R), 
                                Choice::Choice19(Choice19::S), Choice::Choice19(Choice19::T), 
                                Choice::Choice19(Choice19::U), Choice::Choice19(Choice19::V),
                                Choice::Choice19(Choice19::W), Choice::Choice19(Choice19::X), 
                                Choice::Choice19(Choice19::Y), Choice::Choice19(Choice19::Z)];

static CHOICE20: [Choice; 26] = [Choice::Choice20(Choice20::A), Choice::Choice20(Choice20::B), 
                                Choice::Choice20(Choice20::C), Choice::Choice20(Choice20::D), 
                                Choice::Choice20(Choice20::E), Choice::Choice20(Choice20::F), 
                                Choice::Choice20(Choice20::G), Choice::Choice20(Choice20::H), 
                                Choice::Choice20(Choice20::I), Choice::Choice20(Choice20::J), 
                                Choice::Choice20(Choice20::K), Choice::Choice20(Choice20::L), 
                                Choice::Choice20(Choice20::M), Choice::Choice20(Choice20::N), 
                                Choice::Choice20(Choice20::O), Choice::Choice20(Choice20::P), 
                                Choice::Choice20(Choice20::Q), Choice::Choice20(Choice20::R), 
                                Choice::Choice20(Choice20::S), Choice::Choice20(Choice20::T), 
                                Choice::Choice20(Choice20::U), Choice::Choice20(Choice20::V),
                                Choice::Choice20(Choice20::W), Choice::Choice20(Choice20::X), 
                                Choice::Choice20(Choice20::Y), Choice::Choice20(Choice20::Z)];

static CHOICE21: [Choice; 26] = [Choice::Choice21(Choice21::A), Choice::Choice21(Choice21::B), 
                                Choice::Choice21(Choice21::C), Choice::Choice21(Choice21::D), 
                                Choice::Choice21(Choice21::E), Choice::Choice21(Choice21::F), 
                                Choice::Choice21(Choice21::G), Choice::Choice21(Choice21::H), 
                                Choice::Choice21(Choice21::I), Choice::Choice21(Choice21::J), 
                                Choice::Choice21(Choice21::K), Choice::Choice21(Choice21::L), 
                                Choice::Choice21(Choice21::M), Choice::Choice21(Choice21::N), 
                                Choice::Choice21(Choice21::O), Choice::Choice21(Choice21::P), 
                                Choice::Choice21(Choice21::Q), Choice::Choice21(Choice21::R), 
                                Choice::Choice21(Choice21::S), Choice::Choice21(Choice21::T), 
                                Choice::Choice21(Choice21::U), Choice::Choice21(Choice21::V),
                                Choice::Choice21(Choice21::W), Choice::Choice21(Choice21::X), 
                                Choice::Choice21(Choice21::Y), Choice::Choice21(Choice21::Z)];

static CHOICE22: [Choice; 26] = [Choice::Choice22(Choice22::A), Choice::Choice22(Choice22::B), 
                                Choice::Choice22(Choice22::C), Choice::Choice22(Choice22::D), 
                                Choice::Choice22(Choice22::E), Choice::Choice22(Choice22::F), 
                                Choice::Choice22(Choice22::G), Choice::Choice22(Choice22::H), 
                                Choice::Choice22(Choice22::I), Choice::Choice22(Choice22::J), 
                                Choice::Choice22(Choice22::K), Choice::Choice22(Choice22::L), 
                                Choice::Choice22(Choice22::M), Choice::Choice22(Choice22::N), 
                                Choice::Choice22(Choice22::O), Choice::Choice22(Choice22::P), 
                                Choice::Choice22(Choice22::Q), Choice::Choice22(Choice22::R), 
                                Choice::Choice22(Choice22::S), Choice::Choice22(Choice22::T), 
                                Choice::Choice22(Choice22::U), Choice::Choice22(Choice22::V),
                                Choice::Choice22(Choice22::W), Choice::Choice22(Choice22::X), 
                                Choice::Choice22(Choice22::Y), Choice::Choice22(Choice22::Z)];

static CHOICE23: [Choice; 26] = [Choice::Choice23(Choice23::A), Choice::Choice23(Choice23::B), 
                                Choice::Choice23(Choice23::C), Choice::Choice23(Choice23::D), 
                                Choice::Choice23(Choice23::E), Choice::Choice23(Choice23::F), 
                                Choice::Choice23(Choice23::G), Choice::Choice23(Choice23::H), 
                                Choice::Choice23(Choice23::I), Choice::Choice23(Choice23::J), 
                                Choice::Choice23(Choice23::K), Choice::Choice23(Choice23::L), 
                                Choice::Choice23(Choice23::M), Choice::Choice23(Choice23::N), 
                                Choice::Choice23(Choice23::O), Choice::Choice23(Choice23::P), 
                                Choice::Choice23(Choice23::Q), Choice::Choice23(Choice23::R), 
                                Choice::Choice23(Choice23::S), Choice::Choice23(Choice23::T), 
                                Choice::Choice23(Choice23::U), Choice::Choice23(Choice23::V),
                                Choice::Choice23(Choice23::W), Choice::Choice23(Choice23::X), 
                                Choice::Choice23(Choice23::Y), Choice::Choice23(Choice23::Z)];

static CHOICE24: [Choice; 26] = [Choice::Choice24(Choice24::A), Choice::Choice24(Choice24::B), 
                                Choice::Choice24(Choice24::C), Choice::Choice24(Choice24::D), 
                                Choice::Choice24(Choice24::E), Choice::Choice24(Choice24::F), 
                                Choice::Choice24(Choice24::G), Choice::Choice24(Choice24::H), 
                                Choice::Choice24(Choice24::I), Choice::Choice24(Choice24::J), 
                                Choice::Choice24(Choice24::K), Choice::Choice24(Choice24::L), 
                                Choice::Choice24(Choice24::M), Choice::Choice24(Choice24::N), 
                                Choice::Choice24(Choice24::O), Choice::Choice24(Choice24::P), 
                                Choice::Choice24(Choice24::Q), Choice::Choice24(Choice24::R), 
                                Choice::Choice24(Choice24::S), Choice::Choice24(Choice24::T), 
                                Choice::Choice24(Choice24::U), Choice::Choice24(Choice24::V),
                                Choice::Choice24(Choice24::W), Choice::Choice24(Choice24::X), 
                                Choice::Choice24(Choice24::Y), Choice::Choice24(Choice24::Z)];

static CHOICE25: [Choice; 26] = [Choice::Choice25(Choice25::A), Choice::Choice25(Choice25::B), 
                                Choice::Choice25(Choice25::C), Choice::Choice25(Choice25::D), 
                                Choice::Choice25(Choice25::E), Choice::Choice25(Choice25::F), 
                                Choice::Choice25(Choice25::G), Choice::Choice25(Choice25::H), 
                                Choice::Choice25(Choice25::I), Choice::Choice25(Choice25::J), 
                                Choice::Choice25(Choice25::K), Choice::Choice25(Choice25::L), 
                                Choice::Choice25(Choice25::M), Choice::Choice25(Choice25::N), 
                                Choice::Choice25(Choice25::O), Choice::Choice25(Choice25::P), 
                                Choice::Choice25(Choice25::Q), Choice::Choice25(Choice25::R), 
                                Choice::Choice25(Choice25::S), Choice::Choice25(Choice25::T), 
                                Choice::Choice25(Choice25::U), Choice::Choice25(Choice25::V),
                                Choice::Choice25(Choice25::W), Choice::Choice25(Choice25::X), 
                                Choice::Choice25(Choice25::Y), Choice::Choice25(Choice25::Z)];

static CHOICES: [&[Choice; 26]; 26] = [&CHOICE0, &CHOICE1, &CHOICE2, &CHOICE3, &CHOICE4, 
                                        &CHOICE5, &CHOICE6, &CHOICE7, &CHOICE8, &CHOICE9, 
                                        &CHOICE10, &CHOICE11, &CHOICE12, &CHOICE13, &CHOICE14, 
                                        &CHOICE15, &CHOICE16, &CHOICE17, &CHOICE18, &CHOICE19, 
                                        &CHOICE20, &CHOICE21, &CHOICE22, &CHOICE23, &CHOICE24,
                                        &CHOICE25];

