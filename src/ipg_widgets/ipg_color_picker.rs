
use crate::access_callbacks;
use crate::app::Message;
use super::ipg_button::{ButtonStyleRadius, get_button_style_from_obj};
use super::callbacks::{WidgetCallbackIn, 
                        WidgetCallbackOut, 
                        get_set_widget_callback_data};

use iced::widget::{Button, Space, Text};
use iced::{Color, Element, Length, Padding, theme,};

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
    pub style: Option<PyObject>,
    pub open: bool,
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
        style: Option<PyObject>,
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
            open: false,
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

   if !cp.show {
        return Space::new(0.0, 0.0).into()
   }

    let style = get_button_style_from_obj(cp.style.clone());

    let btn: Element<ColPikMessage> = Button::new(Text::new(cp.label.clone()))
                                    .height(cp.height)
                                    .padding(cp.padding)
                                    .width(cp.width)
                                    .on_press(ColPikMessage::ChooseColor)
                                    .style(theme::Button::Custom(Box::new(
                                        ButtonStyleRadius::new(style, cp.corner_radius))))
                                    .into();

    if !cp.open {
        
        let mapped_cp: Element<Message> = btn.map(move |message| Message::ColorPicker(cp.id, message));
        return mapped_cp
    }
    
    let color_picker: Element<ColPikMessage> = ColorPicker::new(
                                    cp.show,
                                    cp.color,
                                    btn,
                                    ColPikMessage::OnCancel,
                                    ColPikMessage::OnSubmit,
                                ).into();

    let mapped_cp: Element<Message> = color_picker.map(move |message| Message::ColorPicker(cp.id, message));

    mapped_cp
    
}


pub fn color_picker_update(id: usize, message: ColPikMessage) {

    match message {
        ColPikMessage::ChooseColor => {
            // Non callback just setting the show value.
            let mut wci: WidgetCallbackIn = WidgetCallbackIn::default();
            wci.id = id;
            wci.value_bool = Some(true);
            let _ = get_set_widget_callback_data(wci);
        },
        ColPikMessage::OnCancel => {
            // Non callback just setting the show value.
            let mut wci: WidgetCallbackIn = WidgetCallbackIn::default();
            wci.id = id;
            wci.value_bool = Some(false);
            let _ = get_set_widget_callback_data(wci);
        },
        ColPikMessage::OnSubmit(color) => {
            let mut wci: WidgetCallbackIn = WidgetCallbackIn::default();
            wci.id = id;
            wci.value_bool = Some(false);
            wci.color = Some(convert_color_to_list(color));
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = "on_submit".to_string();
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
                None => panic!("ColorPicker user_data in callback could not be found"),
            };
            let res = callback.call1(py, (
                                                                wco.id.clone(), 
                                                                wco.color,
                                                                user_data
                                                                ));
            match res {
                Ok(_) => (),
                Err(_) =>panic!("ColorPicker: 3 parameters (id, value, user_data) are required or possibly a non-fatal python error in this function."),
            }
        } else {
            let res = callback.call1(py, (
                                                                wco.id.clone(),
                                                                wco.color, 
                                                                ));
            match res {
                Ok(_) => (),
                Err(_) => panic!("ColorPicker: 2 parameters (id, value) are required or possibly a non-fatal python error in this function."),
            }
        } 
    });
    
    drop(app_cbs);

}


pub fn color_picker_item_update(_cp: &mut IpgColorPicker,
                                _item: PyObject,
                                _items: PyObject,
                                ) 
{

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
