#![allow(unused_imports)]

use crate::ipg_widgets::helpers::{try_extract_boolean, try_extract_f64, try_extract_string};
use crate::access_callbacks;
use crate::app;
use super::helpers::{get_width, get_shaping};
use super::callbacks::{WidgetCallbackIn, 
                        WidgetCallbackOut, 
                        get_set_widget_callback_data,
                        };

use iced::{Length, Element};
use iced::widget::text::{self, LineHeight, Shaping};
use iced::widget::{Checkbox, Space};
use iced::widget::checkbox::Icon;

use iced_aw::graphics::icons::icon_to_char;
use iced_aw::{BootstrapIcon, BOOTSTRAP_FONT};

use pyo3::{pyclass, PyObject, Python};


#[derive(Debug, Clone)]
pub struct IpgCheckBox {
    pub id: usize,
    pub show: bool,
    pub user_data: Option<PyObject>,

    pub is_checked: bool,
    pub label: String,
    pub width: Length,
    pub size: f32,
    pub spacing: f32,
    pub text_size: f32,
    pub text_line_height: LineHeight,
    pub text_shaping: Shaping,
    // font: Option<Font>,
    pub icon_x: bool,
    pub icon_size: f32,
    pub style: Option<PyObject>,
}

impl IpgCheckBox {
    pub fn new(
        id: usize,
        show: bool,
        user_data: Option<PyObject>,

        is_checked: bool,
        label: String,
        width: Length,
        size: f32,
        spacing: f32,
        text_size: f32,
        text_line_height: LineHeight,
        text_shaping: Shaping,
        icon_x: bool,
        icon_size: f32,
        style: Option<PyObject>,
        ) -> Self {
            Self {
                id,
                show,
                user_data,
                is_checked,
                label,
                width,
                size,
                spacing,
                text_size,
                text_line_height,
                text_shaping,
                icon_x,
                icon_size,
                style,
            }
    }
}

#[derive(Debug, Clone)]
pub enum CHKMessage {
    OnToggle(bool),
}

pub fn construct_checkbox(chk: IpgCheckBox) -> Element<'static, app::Message> {

    if !chk.show {
        return Space::new(Length::Shrink, Length::Shrink).into()
    };
    
    let check = icon_to_char(BootstrapIcon::Check);
    let x = icon_to_char(BootstrapIcon::X);


    let ipg_chk: Element<'_, CHKMessage> = Checkbox::new(chk.label.clone(), 
                            chk.is_checked)
                            .on_toggle(CHKMessage::OnToggle)
                            .size(chk.size)
                            .spacing(chk.spacing)
                            .text_line_height(chk.text_line_height)
                            .text_shaping(chk.text_shaping)
                            .text_size(chk.text_size)
                            //TODO: .font(BOOTSTRAP_FONT)
                            .width(chk.width)
                            .icon(if chk.icon_x {
                                        Icon {
                                        font: BOOTSTRAP_FONT,
                                        code_point: x,
                                        size: Some(iced::Pixels(chk.icon_size)),
                                        line_height: text::LineHeight::Relative(1.0),
                                        shaping: text::Shaping::Basic}
                                    } else {
                                        Icon {
                                            font: BOOTSTRAP_FONT,
                                            code_point: check,
                                            size: Some(iced::Pixels(chk.icon_size)),
                                            line_height: text::LineHeight::Relative(1.0),
                                            shaping: text::Shaping::Basic}
                                    }
                            )
                            .into();

    ipg_chk.map(move |message| app::Message::CheckBox(chk.id, message))
}

pub fn checkbox_callback(id: usize, message: CHKMessage) {

    match message {
        CHKMessage::OnToggle(on_toggle) => {
            let mut wci: WidgetCallbackIn = WidgetCallbackIn::default();
            wci.id = id;
            wci.on_toggle = Some(on_toggle);
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = "on_toggle".to_string();
            process_callback(wco);
        }
    }
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
        None => panic!("Callback could not be found with id {}", wco.id),
    };
             
    Python::with_gil(|py| {
            if wco.user_data.is_some() {
                let user_data = match wco.user_data {
                    Some(ud) => ud,
                    None => panic!("Checkbox callback user_Data could not be found"),
                };
                let res = callback.call1(py, (
                                                                    wco.id.clone(), 
                                                                    wco.is_checked, 
                                                                    user_data
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Checkbox: 3 parameters (id, value, user_data) are required or python error in this function. {er}"),
                }
            } else {
                let res = callback.call1(py, (
                                                                    wco.id.clone(), 
                                                                    wco.is_checked 
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Checkbox: 2 parameters (id, value) are required or a python error in this function. {er}"),
                }
            } 
    });

    drop(app_cbs);

}


#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgCheckboxParams {
    IconSize,
    IconX,
    IsChecked,
    Label,
    Show,
    Size,
    Spacing,
    Style,
    TextLineHeight,
    TextShaping,
    TextSize,
    Width,
    WidthFill,
}

pub fn checkbox_item_update(chk: &mut IpgCheckBox,
                            item: PyObject,
                            value: PyObject,
                            )
{
    let update = try_extract_checkbox_update(item);

    match update {
        IpgCheckboxParams::IconSize => {
            chk.icon_size = try_extract_f64(value) as f32;
        },
        IpgCheckboxParams::IconX => {
            chk.icon_x = try_extract_boolean(value);
        },
        IpgCheckboxParams::IsChecked => {
            chk.is_checked = try_extract_boolean(value);
        },
        IpgCheckboxParams::Label => {
            chk.label = try_extract_string(value);
        },
        IpgCheckboxParams::Show => {
            chk.show = try_extract_boolean(value);
        },
        IpgCheckboxParams::Size => {
            chk.size = try_extract_f64(value) as f32;
        },
        IpgCheckboxParams::Spacing => {
            chk.spacing = try_extract_f64(value) as f32;
        },
        IpgCheckboxParams::Style => {
            // TODO:
            // chk.style = match value_str {
            //     Some(st) => st,
            //     None => panic!("Style must be of type string.")
            // }
        },
        IpgCheckboxParams::TextLineHeight => {
            let tlh = try_extract_f64(value);
            chk.text_line_height =  LineHeight::Relative(tlh as f32);
        },
        IpgCheckboxParams::TextShaping => {
            let ts =try_extract_string(value);
            chk.text_shaping = get_shaping(ts); 
        },
        IpgCheckboxParams::TextSize => {
            chk.text_size = try_extract_f64(value) as f32;
        },
        IpgCheckboxParams::Width => {
            let wd = try_extract_f64(value);
            chk.width =  get_width(Some(wd as f32), false);
        },
        IpgCheckboxParams::WidthFill => {
            let wd = try_extract_boolean(value);
            chk.width =  get_width(None, wd);
        },
    }
}


pub fn try_extract_checkbox_update(update_obj: PyObject) -> IpgCheckboxParams {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgCheckboxParams>(py);

        match res {
            Ok(update) => update,
            Err(_) => panic!("Checkbox update extraction failed"),
        }
    })
}
