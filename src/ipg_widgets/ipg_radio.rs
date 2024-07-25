#![allow(unused)]

use crate::graphics::colors::{match_ipg_color, IpgColor};
use crate::ipg_widgets::helpers::try_extract_boolean;
use crate::style::styling::{self, get_text_pair, lighten, strong};
use crate::{access_state, access_callbacks};
use crate::app;
use super::helpers::{get_height, get_line_height, get_padding_f64, get_width, try_extract_f64, try_extract_f64_option, try_extract_i64, try_extract_i64_option, try_extract_option_string, try_extract_u16, try_extract_vec_f64, try_extract_vec_str};
use super::ipg_enums::IpgWidgets;
use super::callbacks::{WidgetCallbackIn, 
                        WidgetCallbackOut, 
                        get_set_widget_callback_data};
use super::ipg_theme_colors::is_dark;

use iced::theme::palette::Pair;
use iced::widget::radio::{self, Status};
use iced::{theme, Background, Color, Element, Length, Padding, Pixels, Theme};
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
    selected: Option<Choice>,

    pub width: Length,
    pub height: Length,
    pub size: f32,
    pub text_spacing: f32,
    pub text_size: f32,
    pub text_line_height: LineHeight,
    pub text_shaping: Shaping,
    pub group_index: usize,
    // pub font: Option<Font>,
    pub style: Option<String>,
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
        style: Option<String>,
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
            height,
            size,
            text_spacing,
            text_size,
            text_line_height,
            text_shaping,
            group_index: radio_index,
            // font: None,
            style,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IpgRadioStyle {
    pub id: usize,
    pub circle_inner_color: Option<Color>,
    pub circle_inner_hover_color: Option<Color>,
    pub hover_color_factor: Option<f32>,
    pub dot_color: Option<Color>,
    pub border_color: Option<Color>,
    pub border_width: Option<f32>,
    pub text_color: Option<Color>,
}

impl IpgRadioStyle {
    pub fn new(
        id: usize,
        circle_inner_color: Option<Color>,
        circle_inner_hover_color: Option<Color>,
        hover_color_factor: Option<f32>,
        dot_color: Option<Color>,
        border_color: Option<Color>,
        border_width: Option<f32>,
        text_color: Option<Color>,
    ) -> Self {
        Self {
            id,
            circle_inner_color,
            circle_inner_hover_color,
            hover_color_factor,
            dot_color,
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

    let mut selected = radio.selected;
    selected = match radio.is_selected {
        Some(is) => Some(CHOICES[radio.group_index][is]),
        None => None,
    };

    let mut radio_elements = vec![];

    // Due to the closure in the style, had to covert to array of strings
    let mut style: Vec<Option<String>> = vec![];
    for i in 0..radio.labels.len() {
        style.push(radio.style.clone())
    }

    for (i, label) in  radio.labels.iter().enumerate() {

        let style_str = if style[i].is_some() {
            style[i].clone().unwrap()
        } else {
            "none".to_string()
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
                                        style_str.clone(),
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

    let mut ch_usize: usize = 0;

    match message {
        RDMessage::RadioSelected(choice) => {
            match choice {
                Choice::Choice0(ch) => {
                    ch_usize = ch as usize;
                },
                Choice::Choice1(ch) => {
                    ch_usize = ch as usize;
                },
                Choice::Choice2(ch) => {
                    ch_usize = ch as usize;
                },
                Choice::Choice3(ch) => {
                    ch_usize = ch as usize;
                },
                Choice::Choice4(ch) => {
                    ch_usize = ch as usize;
                },
                Choice::Choice5(ch) => {
                    ch_usize = ch as usize;
                },
                Choice::Choice6(ch) => {
                    ch_usize = ch as usize;
                },
                Choice::Choice7(ch) => {
                    ch_usize = ch as usize;
                },
                Choice::Choice8(ch) => {
                    ch_usize = ch as usize;
                },
                Choice::Choice9(ch) => {
                    ch_usize = ch as usize;
                },
                Choice::Choice10(ch) => {
                    ch_usize = ch as usize;
                },
                Choice::Choice11(ch) => {
                    ch_usize = ch as usize;
                },
                Choice::Choice12(ch) => {
                    ch_usize = ch as usize;
                },
                Choice::Choice13(ch) => {
                    ch_usize = ch as usize;
                },
                Choice::Choice14(ch) => {
                    ch_usize = ch as usize;
                },
                Choice::Choice15(ch) => {
                    ch_usize = ch as usize;
                },
                Choice::Choice16(ch) => {
                    ch_usize = ch as usize;
                },
                Choice::Choice17(ch) => {
                    ch_usize = ch as usize;
                },
                Choice::Choice18(ch) => {
                    ch_usize = ch as usize;
                },
                Choice::Choice19(ch) => {
                    ch_usize = ch as usize;
                },
                Choice::Choice20(ch) => {
                    ch_usize = ch as usize;
                },
                Choice::Choice21(ch) => {
                    ch_usize = ch as usize;
                },
                Choice::Choice22(ch) => {
                    ch_usize = ch as usize;
                },
                Choice::Choice23(ch) => {
                    ch_usize = ch as usize;
                },
                Choice::Choice24(ch) => {
                    ch_usize = ch as usize;
                },
            }
        },
    }

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
    Style,
    TextSpacing,
    TextSize,
    TextLineHeightPixels,
    TextLineHeightRelative,
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
                },
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
        IpgRadioParam::Style => {
            rd.style = try_extract_option_string(value);
        },
        IpgRadioParam::TextSpacing => {
            rd.text_spacing = try_extract_f64(value) as f32;
        },
        IpgRadioParam::TextSize => {
            rd.text_size = try_extract_f64(value) as f32;
        },
        IpgRadioParam::TextLineHeightPixels => {
            let val = try_extract_u16(value);
            rd.text_line_height = LineHeight::Absolute(Pixels(val.into()));
        },
        IpgRadioParam::TextLineHeightRelative => {
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
                    style_str: String,
                    ) -> radio::Style {
    
    let mut active_style = radio::default(theme, status);
    let mut hover_style = radio::default(theme, status);

    let state = access_state();

    if style_str == "none".to_string() {
        return match status {
            Status::Active{..} => active_style,
            Status::Hovered{..}  => hover_style,
        }
    }

    let style_opt = state.radio_style.get(&style_str.clone());
    
    let style = match style_opt {
        Some(st) => st,
        None => panic!("Radio: The style_id '{}' for add_radio_style could not be found", style_str)
    };

    let text_opt: Option<Color> = if style.text_color.is_some() {
        style.text_color
    } else {
        None
    };
    let text = if text_opt.is_some() {
        text_opt.unwrap()
    } else {
        if is_dark(theme.palette().background) {
            Color::WHITE
        } else {
            Color::BLACK
        }
    };

    active_style.text_color = text_opt;
    hover_style.text_color = text_opt;

    // The hover status changes the circle inner color to background weak
    // and the border color to match the inner_hover_color.
    // Therefore, if inner color defined then just do a strong to show change
    if style.circle_inner_color.is_some() {
        active_style.background = style.circle_inner_color.unwrap().into();
    }

    if style.circle_inner_hover_color.is_some() {
        hover_style.background = style.circle_inner_hover_color.unwrap().into();
        hover_style.border_color = style.circle_inner_hover_color.unwrap().into();
    }

    if style.circle_inner_color.is_some() && style.circle_inner_hover_color.is_none() {
        hover_style.background = strong(style.circle_inner_color.unwrap(), 
                                        style.hover_color_factor).into();
        hover_style.border_color = strong(style.circle_inner_color.unwrap(), 
                                        style.hover_color_factor).into();
    } 
    
    if style.dot_color.is_some() {
        active_style.dot_color = style.dot_color.unwrap();
        hover_style.dot_color = style.dot_color.unwrap();
    }
    
    // border color changes to inner color during hover
    if style.border_color.is_some() {
        active_style.border_color = style.border_color.unwrap();
    }
    
    if style.border_width.is_some() {
        active_style.border_width = style.border_width.unwrap();
        hover_style.border_width = style.border_width.unwrap();
    }
        

    match status {
        Status::Active{..} => active_style,
        Status::Hovered{..} => hover_style,
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


static CHOICES: [&[Choice; 26]; 2] = [&CHOICE0, &CHOICE1];

