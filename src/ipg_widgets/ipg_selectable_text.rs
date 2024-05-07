

use crate::app;
use crate::access_callbacks;
use super::callbacks::{WidgetCallbackIn, 
                        WidgetCallbackOut, 
                        get_set_widget_callback_data};
use super::helpers::{get_height, get_width, try_extract_boolean,
                    try_extract_f64, try_extract_string};

use iced::{Length, Element, Point};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::text::{LineHeight, Shaping};
use iced::widget::{MouseArea, Text};
use iced::mouse::Interaction;

use pyo3::pyclass;
use pyo3::types::IntoPyDict;
use pyo3::{PyObject, Python};


#[derive(Debug, Clone)]
pub struct IpgSelectableText {
        pub id: usize,
        pub content: String,
        pub width: Length,
        pub height: Length,
        pub horizontal_alignment: IpgSelectableTextHorAlign,
        pub vertical_alignment: IpgSelectableTextVertAlign,
        pub line_height: LineHeight,
        pub size: f32,
        pub show: bool,
        // pub font: Font,
        pub shaping: Shaping,
        // pub style: Style,
        pub user_data: Option<PyObject>,
}

impl IpgSelectableText {
    pub fn new( 
        id: usize,
        content: String,
        width: Length,
        height: Length,
        horizontal_alignment: IpgSelectableTextHorAlign,
        vertical_alignment: IpgSelectableTextVertAlign,
        line_height: LineHeight,
        size: f32,
        show: bool,
        // font: Font,
        shaping: Shaping,
        // style: Style,
        user_data: Option<PyObject>,
        ) -> Self {
        Self {
            id,
            content,
            width,
            height,
            horizontal_alignment,
            vertical_alignment,
            line_height,
            size,
            show,
            // font: Font,
            shaping,
            // style: Style,
            user_data,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SLTXTMessage {
    OnPress,
    OnRelease,
    OnRightPress,
    OnRightRelease,
    OnMiddlePress,
    OnMiddleRelease,
    OnMove(Point),
    OnEnter,
    OnExit,
}


pub fn construct_selectable_text(sl_text: IpgSelectableText) -> Element<'static, app::Message> {

    let hor_align = get_horizontal_align(sl_text.horizontal_alignment);
    let vert_align = get_vertical_align(sl_text.vertical_alignment);
    
    let content: Element<'_, SLTXTMessage> = 
                        Text::new(sl_text.content.clone())
                            .size(sl_text.size)
                            .line_height(sl_text.line_height)
                            .width(sl_text.width)
                            .height(sl_text.height)
                            .horizontal_alignment(hor_align)
                            .vertical_alignment(vert_align)
                            // font: Font,
                            .shaping(sl_text.shaping)
                            // style: Style,
                            .into();

    let ma: Element<'_, SLTXTMessage> = 
                MouseArea::new(content)
                    .on_press(SLTXTMessage::OnPress)
                    .on_release(SLTXTMessage::OnRelease)
                    .on_right_press(SLTXTMessage::OnRightPress)
                    .on_right_release(SLTXTMessage::OnRightRelease)
                    .on_middle_press(SLTXTMessage::OnMiddlePress)
                    .on_middle_release(SLTXTMessage::OnMiddleRelease)
                    .on_move(SLTXTMessage::OnMove)
                    .on_enter(SLTXTMessage::OnEnter)
                    .on_exit(SLTXTMessage::OnExit)
                    .interaction(Interaction::Pointer)
                    .into();

    ma.map(move |message| app::Message::SelectableText(sl_text.id, message))

}

pub fn selectable_text_callback(id: usize, message: SLTXTMessage) {

    let mut wci: WidgetCallbackIn = WidgetCallbackIn::default();
    wci.id = id;

    match message {
        SLTXTMessage::OnPress => {
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = "on_press".to_string();
            process_callback(wco);
        },
        SLTXTMessage::OnRelease => {
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = "on_release".to_string();
            process_callback(wco);
        },
        SLTXTMessage::OnRightPress => {
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = "on_right_press".to_string();
            process_callback(wco);
        },
        SLTXTMessage::OnRightRelease => {
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = "on_right_release".to_string();
            process_callback(wco);
        },
        SLTXTMessage::OnMiddlePress => {
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = "on_middle_press".to_string();
            process_callback(wco);
        },
        SLTXTMessage::OnMiddleRelease => {
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = "on_middle_release".to_string();
            process_callback(wco);
        },
        SLTXTMessage::OnEnter => {
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = "on_enter".to_string();
            process_callback(wco);
        },
        SLTXTMessage::OnMove(point) => {
            let mut points: Vec<(String, f32)> = vec![];
            points.push(("x".to_string(), point.x));
            points.push(("y".to_string(), point.y));
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = "on_move".to_string();
            wco.points = Some(points);
            process_callback(wco);
        },
        SLTXTMessage::OnExit => {
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = "on_exit".to_string();
            process_callback(wco);
        },
    }    
}


fn process_callback(wco: WidgetCallbackOut) 
{
    let app_cbs = access_callbacks();

    let callback_present = 
                                app_cbs.callbacks.get(&(wco.id, wco.event_name.clone()));

    let callback_opt = match callback_present {
        Some(cb) => cb,
        None => return,
    };

    let callback = match callback_opt {
        Some(cb) => cb,
        None => panic!("SelectableText Callback could not be found with id {}", wco.id),
    };
                  
    if wco.event_name == "on_move".to_string() {

        let points = match wco.points {
            Some(pts) => pts,
            None => panic!("Points not found"),
        };
        Python::with_gil(|py| {

            if wco.user_data.is_some() {
                let user_data = match wco.user_data {
                    Some(dt) => dt,
                    None => panic!("SelectableText: user_data not found"),
                };
                let res = callback.call1(py, (
                                                                    wco.id.clone(), 
                                                                    points.into_py_dict_bound(py), 
                                                                    user_data
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("SelectableText: 3 parameters (id, points, user_data) are required or a python error in this function. {er}"),
                }
            } else {
                let res = callback.call1(py, (
                                                                    wco.id.clone(), 
                                                                    points.into_py_dict_bound(py), 
                                                                    )
                                                                    );
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("SelectableText 2 parameters (id, points) are required or a python error in this function. {er}"),
                }
            } 
        });

    } else {
        Python::with_gil(|py| {
            if wco.user_data.is_some() {
                let user_data = match wco.user_data {
                    Some(dt) => dt,
                    None => panic!("SelectableText user_data not found"),
                };
                let res = callback.call1(py, (
                                                                    wco.id.clone(),
                                                                    user_data
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(_) => panic!("SelectableText: 2 parameters (id, user_data) are required or possibly a non-fatal python error in this function."),
                }                                                
            } else {
                let res = callback.call1(py, (
                                                                    wco.id.clone(),  
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(_) => panic!("SelectableText: Only 1 parameter (id) is required or possibly a non-fatal python error in this function."),
                }
            }
            
        });
    }
    
    drop(app_cbs);

}


#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgSelectableTextParams {
    Text,
    Width,
    WidthFill,
    Height,
    HeightFill,
    HorizontalAlign,
    VerticalAlign,
    LineHeight,
    Size,
    Show,
}

#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgSelectableTextHorAlign {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgSelectableTextVertAlign {
    Top,
    Center,
    Bottom,
}

pub fn selectable_text_item_update(st: &mut IpgSelectableText,
                                        item: PyObject,
                                        value: PyObject
                                    )
{
    let update = try_extract_selectable_update(item);

    match update {
        IpgSelectableTextParams::Text => {
            st.content = try_extract_string(value);
        },
        IpgSelectableTextParams::Width => {
            let val = try_extract_f64(value);
            st.width = get_width(Some(val as f32), false);
        },
        IpgSelectableTextParams::WidthFill => {
            let val = try_extract_boolean(value);
            st.width = get_width(None, val);
        },
        IpgSelectableTextParams::Height => {
            let val = try_extract_f64(value);
            st.height = get_height(Some(val as f32), false);
        },
        IpgSelectableTextParams::HeightFill => {
            let val = try_extract_boolean(value);
            st.height = get_height(None, val);
        }
        IpgSelectableTextParams::HorizontalAlign => {
            st.horizontal_alignment = try_extract_hor_align(value);
        },
        IpgSelectableTextParams::VerticalAlign => {
            st.vertical_alignment = try_extract_vert_align(value);
        },
        IpgSelectableTextParams::LineHeight => {
            let val = try_extract_f64(value) as f32;
            st.line_height = LineHeight::Relative(val);
        },
        IpgSelectableTextParams::Size => {
            st.size = try_extract_f64(value) as f32;
        },
        IpgSelectableTextParams::Show => {
            st.show = try_extract_boolean(value);
        },
    }
}

fn try_extract_selectable_update(update_obj: PyObject) -> IpgSelectableTextParams {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgSelectableTextParams>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Button update extraction failed"),
        }
    })
}

fn try_extract_hor_align(value: PyObject) -> IpgSelectableTextHorAlign {
    
    Python::with_gil(|py| {
        let res = value.extract::<IpgSelectableTextHorAlign>(py);
        match res {
            Ok(h_align) => h_align,
            Err(_) => panic!("IpgSectableText: unable to extract IpgSelectableTextHorAlign"),
        }
    })
}

fn try_extract_vert_align(value: PyObject) -> IpgSelectableTextVertAlign {
    
    Python::with_gil(|py| {
        let res = value.extract::<IpgSelectableTextVertAlign>(py);
        match res {
            Ok(v_align) => v_align,
            Err(_) => panic!("IpgSelectableText: unable to extract IpgSelectableTextHorAlign"),
        }
    })
}

fn get_horizontal_align(ha: IpgSelectableTextHorAlign) -> Horizontal {
    match ha {
        IpgSelectableTextHorAlign::Left => Horizontal::Left,
        IpgSelectableTextHorAlign::Center => Horizontal::Center,
        IpgSelectableTextHorAlign::Right => Horizontal::Right,
    }
}

fn get_vertical_align(va: IpgSelectableTextVertAlign) -> Vertical {
    match va {
        IpgSelectableTextVertAlign::Top => Vertical::Top,
        IpgSelectableTextVertAlign::Center => Vertical::Center,
        IpgSelectableTextVertAlign::Bottom => Vertical::Bottom,
    }
}
