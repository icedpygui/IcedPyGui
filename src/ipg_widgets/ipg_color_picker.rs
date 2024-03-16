
use crate::ipg_widgets::ipg_enums::{IpgWidgets, get_set_widget_data};
use crate::{access_state, access_callbacks};
use crate::app::Message;
use crate::ipg_widgets::ipg_button::{ButtonStyleRadius, get_button_style};

use iced::widget::{Button, Row, Space, Text};
use iced::{alignment, Alignment, Border, Color, Element, Length, Padding, Renderer, theme, Theme, };
use iced::font::Font;
use iced::widget::button::{self, Appearance, StyleSheet};

use iced_aw::ColorPicker;

use pyo3::{Python, PyObject};

const ICON_FONT: Font = Font::with_name("icons");


#[derive(Debug, Clone)]
pub struct IpgColorPicker {
    pub id: usize,
    pub show: bool,
    pub color: Color,
    pub user_data: Option<PyObject>,

    //button related
    pub label: String,
    pub width: Length,
    pub height: Length,
    pub padding: Padding,
    pub corner_radius: f32,
    pub style: String,
    pub cb_name: Option<String>,
}

impl IpgColorPicker {
    pub fn new( 
        id: usize,
        show: bool,
        color: Color,
        user_data: Option<PyObject>,
        // button related
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
            color,
            user_data,
            // button related
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
pub enum ColPikMessage {
    ChooseColor,
    SubmitColor(Color),
    CancelColor,
}


pub fn construct_color_picker(cp: IpgColorPicker) -> Element<'static, Message> {

    let btn_label: Element<Message> = Text::new("Set Color").into();

    let style = get_button_style(cp.style.clone());
    
    let btn: Element<ColPikMessage> = Button::new(Text::new(cp.label.clone()))
                                .height(cp.height)
                                .padding(cp.padding)
                                .width(cp.width)
                                .on_press(ColPikMessage::ChooseColor)
                                .style(theme::Button::Custom(Box::new(
                                    ButtonStyleRadius::new(style, cp.corner_radius))))
                                .into();

    let color_picker: Element<ColPikMessage> = ColorPicker::new(
                                    cp.show,
                                    cp.color,
                                    btn,
                                    ColPikMessage::CancelColor,
                                    ColPikMessage::SubmitColor,
                                ).into();

    let mapped_cp: Element<Message> = color_picker.map(move |message| Message::ColorPicker(cp.id, message));

    let row: Element<Message> = Row::new()
                                .align_items(Alignment::Center)
                                .spacing(10)
                                .push(mapped_cp)
                                .into();
    row
}


pub fn color_picker_update(id: usize, message: ColPikMessage) {

    match message {
        ColPikMessage::ChooseColor => {
            let show = true;
            open_close_color_picker(id, show);
        },
        ColPikMessage::CancelColor => {
            let show: bool = false;
            open_close_color_picker(id, show);
        },
        ColPikMessage::SubmitColor(color) => {
            let show = false;
            let color_list: Vec<f64> = convert_color_to_list(color);
            let (cb_name, user_data,_,_,_) = 
                                            get_set_widget_data(
                                                                id, 
                                                                Some(show),
                                                                None, 
                                                                Some(color_list.clone()),
                                                                None, 
                                                                );
            let event_name = "Color_Submitted".to_string();
            
            let show = false;
            
            process_callback(
                            id, 
                            event_name, 
                            color_list,
                            user_data,
                            cb_name
                            );
        }
    }

}

pub fn color_picker_item_update(
                                cp: &mut IpgColorPicker,
                                item: String,
                                value_str: Option<String>,
                                value_bool: Option<bool>,
                                value_i64: Option<i64>,
                                value_f64: Option<f64>,
                                value_tup_str_i64: Option<(String, i64)>,
                                value_tup_str_f64: Option<(String, f64)>,
                                value_vec_f64: Option<Vec<f64>>,
                                ) 
{

}


fn open_close_color_picker(id: usize, show: bool) {

    // Non callback just seeting the show value.
    let (_, _, _, _,_) = get_set_widget_data(id, 
                                            Some(show),
                                            None, 
                                            None,
                                            None, 
                                            );

}

fn process_callback(
                    id: usize, 
                    event_name: String,
                    color: Vec<f64>,
                    user_data: Option<PyObject>, 
                    cb_name: Option<String>
                    ) 
{

    if !cb_name.is_some() {return};

    let app_cbs = access_callbacks();
    
    let mut found_callback = None;

    for callback in app_cbs.callbacks.iter() {

        if id == callback.id && cb_name == callback.name {

            found_callback = match callback.cb.clone() 
                                        {
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
                                            color,  
                                            user_data
                                            )
                                ).unwrap();
                    } else {
                        cb.call1(py, (
                                            id.clone(), 
                                            event_name,
                                            color, 
                                            )
                                ).unwrap();
                    }                    
                    }),
        None => panic!("Checkbox callback not found"),
    };

}


fn convert_color_to_list(color: Color) -> Vec<f64> {

    vec![
        rnd_2(color.r),
        rnd_2(color.g),
        rnd_2(color.b),
        rnd_2(color.a),
    ]
}

fn rnd_2(rgba: f32) -> f64 {
    let num = rgba as f64 * 100.0;
    num.round()/100.0
}
