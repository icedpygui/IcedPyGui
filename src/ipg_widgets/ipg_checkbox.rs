
use crate::{access_callbacks, access_state, UpdateItems};
use crate::app;
use super::helpers::{get_width, get_shaping};
use crate::ipg_widgets::ipg_enums::{IpgWidgets, get_set_widget_data};

use iced::{Length, Element};
use iced::widget::text::{self, LineHeight, Shaping};
use iced::widget::{Checkbox, Space};
use iced::widget::checkbox::Icon;

use iced_aw::graphics::icons::{icon_to_char, icon_to_string};
use iced_aw::{BootstrapIcon, BOOTSTRAP_FONT};
use pyo3::{Python, PyObject};

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
    // style: Style,
    pub cb_name: Option<String>,
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
        cb_name: Option<String>,
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
                cb_name,
            }
    }
}

#[derive(Debug, Clone)]
pub enum CHKMessage {
    Checked(bool),
}

pub fn construct_checkbox(chk: IpgCheckBox) -> Element<'static, app::Message> {

    if !chk.show {
        return Space::new(Length::Shrink, Length::Shrink).into()
    };
    
    let check = icon_to_char(BootstrapIcon::Check);
    let x = icon_to_char(BootstrapIcon::X);


    let ipg_chk: Element<'_, CHKMessage> = Checkbox::new(chk.label.clone(), 
                            chk.is_checked)
                            .on_toggle(CHKMessage::Checked)
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

pub fn checkbox_update(id: usize, message: CHKMessage) {

    match message {
        CHKMessage::Checked(checked) => {
            
            let (cb_name, user_data,_,_,_) = 
                                    get_set_widget_data(
                                                        id, 
                                                        Some(checked), 
                                                        None,
                                                        None,
                                                        None,
                                                        );
            
            let event_name = "Checked".to_string();

            process_callback(
                                id, 
                                event_name, 
                                checked,
                                user_data,
                                cb_name
                            );
        }
    }
}


pub fn checkbox_item_update(chk: &mut IpgCheckBox,
                            item: String,
                            items: UpdateItems,
                            )
{
    if item == "icon_size".to_string() {
        chk.icon_size = match items.value_f64 {
            Some(size) => size as f32,
            None => panic!("A float value is required to update icon_size for the checkbox.")
        };
        return
    }

    if item == "icon_x".to_string() {
        chk.icon_x = match items.value_bool {
            Some(x) => x,
            None => panic!("A bool value is required to update icon_x for the checkbox.")
        };
        return
    }

    if item == "is_checked".to_string() {
        chk.is_checked = match items.value_bool {
            Some(checked) => checked,
            None => panic!("A bool value is required to update is_checked for the checkbox.")
        };
        return
    }

    if item == "label".to_string() {
        chk.label = match items.value_str {
            Some(str) => str,
            None => panic!("A string value is needed to update the button label.")
        };
        return
    }

    if item == "show".to_string() {
        chk.show = match items.value_bool {
            Some(sh) => sh,
            None => panic!("Show value must be either True or False.")
        };
        return
    }

    if item == "size".to_string() {
        chk.size = match items.value_f64 {
            Some(sz) => sz as f32,
            None => panic!("A float is needed to update the size for the checkbox.")
        };
        return
    }

    if item == "spacing".to_string() {
        chk.spacing = match items.value_f64 {
            Some(sp) => sp as f32,
            None => panic!("A float is needed to update the spacing for the checkbox")
        };
        return
    }

    // if item == "style".to_string() {
    //     chk.style = match value_str {
    //         Some(st) => st,
    //         None => panic!("Style must be of type string.")
    //     }
    // }

    if item == "text_line_height".to_string() {
        chk.text_line_height = match items.value_f64 {
            Some(tlh) => LineHeight::Relative(tlh as f32),
            None => panic!("A float is needed to update the text_line_height for the checkbox")
        };
        return
    }

    if item == "text_shaping".to_string() {
        chk.text_shaping = match items.value_str {
            Some(ts) => get_shaping(ts),
            None => panic!("A string = 'basic' or 'advanced' is needed for text_shaping for the checkbox")
        };
        return
    }

    if item == "text_size".to_string() {
        chk.text_size = match items.value_f64 {
            Some(ts) => ts as f32,
            None => panic!("A float is needed to update the text_size for the checkbox")
        };
        return
    }

    if item == "width".to_string() {
        chk.width = match items.value_f64 {
            Some(wd) => get_width(Some(wd as f32), false),
            None => Length::Shrink,
        };
        return
    }

    if item == "width_fill".to_string() {
        chk.width = match items.value_bool {
            Some(wd) => get_width(None, wd),
            None => panic!("A boolean is needed to update the button width_fill.")
        };
        return
    }

    panic!("Checkbox update item {} could not be found", item)

}

fn process_callback(id: usize, 
                        event_name: String,
                        checked: bool,
                        user_data: Option<PyObject>, 
                        cb_name: Option<String>) 
                    {

    if !cb_name.is_some() {return};

    let app_cbs = access_callbacks();

    let mut found_callback = None;

    for callback in app_cbs.callbacks.iter() {

        if id == callback.id && cb_name == callback.name {

            found_callback = match callback.cb.clone() {
                                        Some(cb) => Some(cb),
                                        None => {
                                            panic!("Callback could not be found with id {}", id)
                                        },
                                    };
            break;
        }                   
    };
    drop(app_cbs);

    match found_callback {

        Some(cb) => Python::with_gil(|py| {
            if user_data.is_some() {
                cb.call1(py, (
                                    id.clone(), 
                                    event_name,
                                    checked,  
                                    user_data
                                    )
                                ).unwrap();
            } else {
                cb.call1(py, (
                                    id.clone(), 
                                    event_name,
                                    checked, 
                                    )
                                ).unwrap();
            }                    
        }),
        None => panic!("Checkbox callback not found"),
    };

}
