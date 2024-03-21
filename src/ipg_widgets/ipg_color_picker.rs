

use crate::{access_callbacks, UpdateItems};
use crate::app::Message;
use super::ipg_button::{ButtonStyleRadius, get_button_style_from_str};
use super::callbacks::{WidgetCallbackIn, 
                        WidgetCallbackOut, 
                        get_set_widget_callback_data};

use iced::widget::{Button, Row, Text};
use iced::{Alignment, Color, Element, Length, Padding, theme,};

use iced_aw::ColorPicker;

use pyo3::{Python, PyObject};


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
    pub style: Option<String>,
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
        style: Option<String>,
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
        }
    }
}


#[derive(Debug, Clone)]
pub enum ColPikMessage {
    ChooseColor,
    OnCancel,
    OnSubmit(Color),
}


pub fn construct_color_picker(cp: IpgColorPicker) -> Element<'static, Message> {

    // let btn_label: Element<Message> = Text::new("Set Color").into();

    let style = get_button_style_from_str(cp.style.clone());
    
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
                                    ColPikMessage::OnCancel,
                                    ColPikMessage::OnSubmit,
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
            // Non callback just setting the show value.
            let mut wci: WidgetCallbackIn = WidgetCallbackIn::default();
            wci.id = id;
            wci.show = Some(true);
            let _ = get_set_widget_callback_data(wci);
        },
        ColPikMessage::OnCancel => {
            // Non callback just setting the show value.
            let mut wci: WidgetCallbackIn = WidgetCallbackIn::default();
            wci.id = id;
            wci.show = Some(false);
            let _ = get_set_widget_callback_data(wci);
        },
        ColPikMessage::OnSubmit(color) => {
            let mut wci: WidgetCallbackIn = WidgetCallbackIn::default();
            wci.id = id;
            wci.show = Some(false);
            wci.color = Some(convert_color_to_list(color));
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = Some("on_submit".to_string());
            process_callback(wco);
        }
    }

}

pub fn color_picker_item_update(_cp: &mut IpgColorPicker,
                                _item: String,
                                _items: UpdateItems,
                                ) 
{

}


fn process_callback(wco: WidgetCallbackOut) 
{
    if !wco.event_name.is_some() {return};

    let app_cbs = access_callbacks();
    
    let mut found_callback = None;

    for callback in app_cbs.callbacks.iter() {

        if wco.id == callback.id && wco.event_name == Some(callback.event_name.clone()) {

            found_callback = match callback.cb.clone() 
                                        {
                                            Some(cb) => Some(cb),
                                            None => {
                                                panic!("Callback could not be found with id {}", wco.id)
                                            },
                                        };
            break;
        }                   
    };

    drop(app_cbs);

    match found_callback {

        Some(cb) => Python::with_gil(|py| {
                    if wco.user_data.is_some() {
                        cb.call1(py, (
                                            wco.id.clone(), 
                                            wco.event_name,
                                            wco.color,  
                                            wco.user_data
                                            )
                                ).unwrap();
                    } else {
                        cb.call1(py, (
                                            wco.id.clone(), 
                                            wco.event_name,
                                            wco.color, 
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
