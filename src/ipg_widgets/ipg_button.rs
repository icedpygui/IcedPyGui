#![allow(unused)]

use crate::ipg_widgets::ipg_enums::{IpgWidgets, get_set_widget_data};
use crate::app;
use crate::{access_state, access_callbacks};
use super::helpers::{get_padding, get_width, get_height};

use pyo3::{Python, PyObject};

use iced::widget::{Button, Space, Text};
use iced::{alignment, Border, Color, Element, Length, Padding, theme, Theme, };
use iced::font::Font;
use iced::widget::button::{self, Appearance, StyleSheet};

const ICON_FONT: Font = Font::with_name("icons");


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
    pub style: String,
    pub cb_name: Option<String>,
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
        style: String,
        cb_name: Option<String>,
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
            cb_name,
        }
    }
}

#[derive(Debug, Clone)]
pub enum BTNMessage {
    ButtonPressed,
}

pub fn construct_button(btn: IpgButton) -> Element<'static, app::Message> {

    // let btn_text: Element<'static, BTNMessage> = Text::new(btn.content.clone())
    //                                         .font(ICON_FONT)
    //                                         .into();
    if !btn.show {
        return Space::new(Length::Shrink, Length::Shrink).into()
    }
    
    let style = get_button_style(btn.style.clone());
    
    let ipg_btn: Element<BTNMessage> = Button::new(Text::new(btn.label.clone()))
                                .height(btn.height)
                                .padding(btn.padding)
                                .width(btn.width)
                                .on_press(BTNMessage::ButtonPressed)
                                .style(theme::Button::Custom(Box::new(
                                    ButtonStyleRadius::new(style, btn.corner_radius))))
                                .into();

    ipg_btn.map(move |message| app::Message::Button(btn.id, message))
}

fn icon(unicode: char) -> Text<'static> {
    Text::new(unicode.to_string())
        .font(ICON_FONT)
        .size(10)
        .width(10)
        .horizontal_alignment(alignment::Horizontal::Center)
        .vertical_alignment(alignment::Vertical::Center)
}

fn left_arrow_icon() -> Text<'static> {
    icon('\u{f12c}')
}

fn right_arrow_icon() -> Text<'static> {
    icon('\u{f135}')
}

fn up_arrow_icon() -> Text<'static> {
    icon('\u{f12c}')
}

fn down_arrow_icon() -> Text<'static> {
    icon('\u{f12c}')
}

pub fn button_update(id: usize, message: BTNMessage) {

    let (cb_name, user_data,_,_, _) = 
                                    get_set_widget_data(
                                                        id, 
                                                        None, 
                                                        None,
                                                        None,
                                                        None,
                                                        );

    match message {
        BTNMessage::ButtonPressed => {
            let event_name = "Button_Pressed".to_string();
            process_callback(id.clone(),
                                event_name,   
                                user_data, 
                                cb_name);
        }
    }
}

pub fn button_item_update(btn: &mut IpgButton,
                            item: String,
                            value_str: Option<String>,
                            value_bool: Option<bool>,
                            _value_i64: Option<i64>,
                            value_f64: Option<f64>,
                            _value_tup_str_i64: Option<(String, i64)>,
                            _value_tup_str_f64: Option<(String, f64)>,
                            value_vec_f64: Option<Vec<f64>>,
                            )
{
    if item == "corner_radius".to_string() {
        btn.corner_radius = match value_f64 {
            Some(flt) => flt as f32,
            None => panic!("A float value is needed to update button corner radius."),
        };
        return
    }

    if item == "label".to_string() {
        btn.label = match value_str {
            Some(str) => str,
            None => panic!("A string value is needed to update the button label.")
        };
        return
    }

    if item == "width".to_string() {
        btn.width = match value_f64 {
            Some(wd) => get_width(Some(wd as f32), false),
            None => panic!("A float is needed to update the button width.")
        };
        return
    }

    if item == "width_fill".to_string() {
        btn.width = match value_bool {
            Some(wd) => get_width(None, wd),
            None => panic!("A boolean is needed to update the button width_fill.")
        };
        return
    }

    if item == "height".to_string() {
        btn.height = match value_f64 {
            Some(ht) => get_height(Some(ht as f32), false),
            None => panic!("A float is needed to update the button height.")
        };
        return
    }

    if item == "height_fill".to_string() {
        btn.height = match value_bool {
            Some(ht) => get_height(None, ht),
            None => panic!("A boolean is needed to update the button height_fill.")
        };
        return
    }

    if item == "padding".to_string() {
        btn.padding = match value_vec_f64 {
            Some(pad) => get_padding(pad),
            None => panic!("Padding must have a List of length 1, 2, or 4.")
        };
        return
    }

    if item == "show".to_string() {
        btn.show = match value_bool {
            Some(sh) => sh,
            None => panic!("Show value must be either True or False.")
        };
        return
    }

    if item == "style".to_string() {
        btn.style = match value_str {
            Some(st) => st,
            None => panic!("Style must be of type string.")
        };
        return
    }

    panic!("Button update item {} could not be found", item)

}


fn process_callback(id: usize,
                    event_name: String, 
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
                None => panic!("Callback could not be found with id {}", id),
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
                                    user_data
                                    )
                                ).unwrap();
            } else {
                cb.call1(py, (
                                    id.clone(), 
                                    event_name, 
                                    )
                                ).unwrap();
            } 
        }),
        None => panic!("Button callback could not be found"),
    };
            
}                   


pub fn get_button_style(style: String) -> theme::Button {

    let style = match style.as_str() {
        "primary" => theme::Button::Primary,
        "secondary" => theme::Button::Secondary,
        "positive" => theme::Button::Positive,
        "destructive" => theme::Button::Destructive,
        "text" => theme::Button::Text,
        _ => panic!("Button style not found"),
    };
    style
}

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
