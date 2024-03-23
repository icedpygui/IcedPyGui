#![allow(dead_code)]

use crate::{access_callbacks, app, UpdateItems};
use super::helpers::{get_padding, get_width, get_height};
use super::callbacks::{
    WidgetCallbackIn, WidgetCallbackOut, 
    get_set_widget_callback_data
};

use pyo3::{pyclass, PyObject, Python};

use iced::widget::{Button, Space, Text};
use iced::{Border, Color, Element, Length, Padding, theme, Theme, };

use iced::widget::button::{self, Appearance, StyleSheet};

// use iced_aw::BOOTSTRAP_FONT;


#[derive(Debug, Clone)]
pub struct IpgButton {
    pub id: usize,
    pub show: bool,
    pub user_data: Option<PyObject>,

    pub label: String,
    pub width: Length,
    pub height: Length,
    pub padding: Padding,
    pub corner_radius: f32,
    pub style: Option<String>,
    pub arrow_type: Option<String>,
}

impl IpgButton {
    pub fn new( 
        id: usize,
        show: bool,
        user_data: Option<PyObject>,

        label: String,
        width: Length,
        height: Length,
        padding: Padding,
        corner_radius: f32,
        style: Option<String>,
        arrow_type: Option<String>,
        ) -> Self {
        Self {
            id,
            show,
            user_data,
            label,
            width,
            height,
            padding,
            corner_radius,
            style,
            arrow_type,
        }
    }
}

#[derive(Debug, Clone)]
pub enum BTNMessage {
    OnPress,
}

// The style enums below are different than iced ButtonStyles enums though they have the
// same members.  The reason is that the python styles are defined as IpgButtonStyles. Therefore
// one has to send a Option<String> representing the style, using an IpgButtonStyles enum.
// Steps are different based on the intitial contruction and the updating routine.
// 
// Construction phase: 
// lib.add_button() ==> PyObject ==> String ==> construct_button() ==> iced style
// 
// Update phase: 
// lib.update_item() ==> PyObject ==> try_extract (method below) ==> Option<String> returned to update_item
// lib.update_item() => iced update => construction phase.

#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgButtonStyles {
    Primary,
    Secondary,
    Positive,
    Destructive,
    Text
}

// pub enum IpgButtonArrrows {
//     UpArrow,
//     RightArrow,
//     DownArrow,
//     LwftArrow,
// }


pub fn construct_button(btn: IpgButton) -> Element<'static, app::Message> {

    if !btn.show {
        return Space::new(Length::Shrink, Length::Shrink).into()
    }
    
    let style = get_button_style_from_str(btn.style);
    
    let ipg_btn: Element<BTNMessage> = Button::new(Text::new(btn.label.clone()))
                                .height(btn.height)
                                .padding(btn.padding)
                                .width(btn.width)
                                .on_press(BTNMessage::OnPress)
                                .style(theme::Button::Custom(Box::new(
                                    ButtonStyleRadius::new(style, btn.corner_radius))))
                                .into();

    ipg_btn.map(move |message| app::Message::Button(btn.id, message))
}

// fn icon(unicode: char) -> Text<'static> {
//     Text::new(unicode.to_string())
//         .font(BOOTSTRAP_FONT)
//         .size(10)
//         .width(10)
//         .horizontal_alignment(alignment::Horizontal::Center)
//         .vertical_alignment(alignment::Vertical::Center)
// }

// fn left_arrow_icon() -> Text<'static> {
//     icon('\u{f12c}')
// }

// fn right_arrow_icon() -> Text<'static> {
//     icon('\u{f135}')
// }

// fn up_arrow_icon() -> Text<'static> {
//     icon('\u{f12c}')
// }

// fn down_arrow_icon() -> Text<'static> {
//     icon('\u{f12c}')
// }

pub fn button_callback(id: usize, message: BTNMessage) {

    let mut wci = WidgetCallbackIn::default();
    wci.id = id;

    match message {
        BTNMessage::OnPress => {
            // getting only
            let mut wco: WidgetCallbackOut = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = "on_press".to_string();
            process_callback(wco);
        }
    }
}


pub fn process_callback(wco: WidgetCallbackOut) 
{
    let app_cbs = access_callbacks();

    let callback_present = app_cbs.callbacks.get(&(wco.id, wco.event_name.clone()));

    let callback_opt = match callback_present {
        Some(cb) => cb,
        None => return,
    };

    let callback = match callback_opt {
        Some(cb) => cb,
        None => panic!("Button callback could not be found with id {}", wco.id),
    };

    Python::with_gil(|py| {
            if wco.user_data.is_some() {
                let user_data = match wco.user_data {
                    Some(ud) => ud,
                    None => panic!("User Data could not be found in Button callback"),
                };
                let res = callback.call1(py, (
                                                                    wco.id.clone(),  
                                                                    user_data
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(_) => panic!("Button: 2 parameters (id, user_data) are required or possibly a non-fatal python error in this function."),
                }
            } else {
                let res = callback.call1(py, (
                                                                    wco.id.clone(),  
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(_) => panic!("Button: 1 parameter (id) is required or possibly a non-fatal python error in this function."),
                }
            } 
    });
    
    drop(app_cbs);
         
}


pub fn button_item_update(btn: &mut IpgButton,
                            item: String,
                            items: UpdateItems,
                            )
{
    if item == "corner_radius".to_string() {
        btn.corner_radius = match items.value_f64 {
            Some(flt) => flt as f32,
            None => panic!("Button update corner_radius must be a float."),
        };
        return
    }

    if item == "label".to_string() {
        btn.label = match items.value_str {
            Some(str) => str,
            None => panic!("Button update label must be a string.")
        };
        return
    }

    if item == "width".to_string() {
        btn.width = match items.value_f64 {
            Some(wd) => get_width(Some(wd as f32), false),
            None => panic!("Button update width must be a float.")
        };
        return
    }

    if item == "width_fill".to_string() {
        btn.width = match items.value_bool {
            Some(wd) => get_width(None, wd),
            None => panic!("Button update width_fill must be a  boolean.")
        };
        return
    }

    if item == "height".to_string() {
        btn.height = match items.value_f64 {
            Some(ht) => get_height(Some(ht as f32), false),
            None => panic!("Button update height must be a float.")
        };
        return
    }

    if item == "height_fill".to_string() {
        btn.height = match items.value_bool {
            Some(ht) => get_height(None, ht),
            None => panic!("Button update height_fill must be a boolean.")
        };
        return
    }

    if item == "padding".to_string() {
        btn.padding = match items.value_vec_f64 {
            Some(pad) => get_padding(pad),
            None => panic!("Button update padding must be a List of length 1, 2, or 4.")
        };
        return
    }

    if item == "show".to_string() {
        btn.show = match items.value_bool {
            Some(sh) => sh,
            None => panic!("Button update show value must be a boolean.")
        };
        return
    }

    if item == "style".to_string() {
        btn.style = match items.value_str {
            Some(st) => Some(st),
            None => panic!("Button update style must be of string.")
        };
        return
    }

    panic!("Button update item {} could not be found", item)

}


pub fn get_button_style_from_str(style_opt: Option<String>) -> theme::Button {

    let style_str = match style_opt {
        Some(st) => st,
        None => return theme::Button::Primary,
    };

    match style_str.as_str() {
        "Primary" => theme::Button::Primary,
        "Secondary" => theme::Button::Secondary,
        "Positive" => theme::Button::Positive,
        "Destructive" => theme::Button::Destructive,
        "Text" => theme::Button::Text,
        _ => theme::Button::Primary,
    }
}


pub fn get_button_str_from_style(style: IpgButtonStyles) -> Option<String> {
    match style {
        IpgButtonStyles::Primary => Some("Primary".to_string()),
        IpgButtonStyles::Secondary => Some("Secondary".to_string()),
        IpgButtonStyles::Positive => Some("Positive".to_string()),
        IpgButtonStyles::Destructive => Some("Destructive".to_string()),
        IpgButtonStyles::Text => Some("Text".to_string()),
    }
}

pub fn try_extract_button_style(style_obj: PyObject, py: Python<'_>) -> Option<String> {

    let mut style: Option<String> = None;

    let res = style_obj.extract::<IpgButtonStyles>(py);
            if !res.is_err() {
                style = match res {
                    Ok(st) => get_button_str_from_style(st),
                    Err(_) => None,
                }
            }

    style
}

// fn get_button_arrows(arrow_opt: Option<PyObject>) {

//     Python::with_gil(|py| {

//         let arrow: Option<String> = None;

//         if arrow_opt.is_some() {
//             arrow 
//         }
//         let res = value.extract::<IpgButtonStyles>(py);
//             if !res.is_err() {
//                 items.value_str = match res {
//                     Ok(style) => get_button_str_from_style(style),
//                     Err(_) => None,
//                 }
//             }
//     });
    
//     match arrow {
//         IpgButtonArrows::UpArrow => todo!(),
//         IpgButtonArrows::RightArrow => todo!(),
//         IpgButtonArrows::DownArrow => todo!(),
//         IpgButtonArrows::LwftArrow => todo!(),
//     }
// }

pub struct ButtonStyleRadius {
    theme: theme::Button,
    radius: f32,
}

impl ButtonStyleRadius {
    pub fn new(theme: theme::Button, radius: f32) -> Self {
        Self { 
            theme,
            radius,
         }
    }
    fn radius(&mut self, radius: f32) {
        self.radius = radius
    }
}

impl StyleSheet for ButtonStyleRadius {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> Appearance {
        let mut appearance = style.active(&self.theme);
        appearance.border.radius = self.radius.into();

        appearance
    }
}


pub struct ButtonStyle;
impl button::StyleSheet for ButtonStyle {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            text_color: style.extended_palette().background.base.text,
            border: Border::with_radius([4.0; 4]),
            background: Some(Color::TRANSPARENT.into()),
            ..Default::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let plt = style.extended_palette();

        button::Appearance {
            background: Some(plt.primary.weak.color.into()),
            text_color: plt.primary.weak.text,
            ..self.active(style)
        }
    }
}

