

use crate::{access_callbacks, app, IpgAlignment};
use super::helpers::{get_width, try_extract_boolean, try_extract_f64, try_extract_ipg_alignment, try_extract_string};
use super::callbacks::{
    WidgetCallbackIn, WidgetCallbackOut, 
    get_set_widget_callback_data
};


use iced::widget::text::LineHeight;
use pyo3::{pyclass, PyObject, Python};

use iced::widget::{Space, Toggler};
use iced::{alignment, Element, Length};



#[derive(Debug, Clone)]
pub struct IpgToggler {
    pub id: usize,
    pub show: bool,
    pub user_data: Option<PyObject>,

    pub is_toggled: bool,
    pub label: Option<String>,
    pub width: Length,
    pub size: f32,
    pub text_size: f32,
    pub text_line_height: LineHeight,
    pub text_alignment: IpgAlignment,
    pub spacing: f32,
}

impl IpgToggler {
    pub fn new( 
        id: usize,
        show: bool,
        user_data: Option<PyObject>,

        label: Option<String>,
        width: Length,
        size: f32,
        text_size: f32,
        text_line_height: LineHeight,
        text_alignment: IpgAlignment,
        spacing: f32,
        ) -> Self {
        Self {
            id,
            show,
            user_data,
            is_toggled: false,
            label,
            width,
            size,
            text_size,
            text_line_height,
            text_alignment,
            spacing,
        }
    }
}

#[derive(Debug, Clone)]
pub enum TOGMessage {
    Toggled(bool),
}


pub fn construct_toggler(tog: IpgToggler) -> Element<'static, app::Message> {

    if !tog.show {
        return Space::new(Length::Shrink, Length::Shrink).into()
    }

    let text_alignment = get_text_alignment(tog.text_alignment);

    let ipg_tog: Element<TOGMessage> = Toggler::new(tog.label, tog.is_toggled, TOGMessage::Toggled)
                                                    .size(tog.size)
                                                    .width(tog.width)
                                                    .text_size(tog.text_size)
                                                    .text_line_height(tog.text_line_height)
                                                    .text_alignment(text_alignment)
                                                    .spacing(tog.spacing)
                                                    .into();

    ipg_tog.map(move |message| app::Message::Toggler(tog.id, message))
}


pub fn toggle_callback(id: usize, message: TOGMessage) {

    let mut wci = WidgetCallbackIn::default();
    wci.id = id;

    match message {
        TOGMessage::Toggled(on_toggle) => {
            wci.on_toggle = Some(on_toggle);
            let mut wco: WidgetCallbackOut = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.on_toggle = Some(on_toggle);
            wco.event_name = "toggled".to_string();
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
        None => panic!("Toggler callback could not be found with id {}", wco.id),
    };

    Python::with_gil(|py| {
            if wco.user_data.is_some() {
                let user_data = match wco.user_data {
                    Some(ud) => ud,
                    None => panic!("User Data could not be found in Toggler callback"),
                };
                let res = callback.call1(py, (
                                                                    wco.id.clone(),
                                                                    wco.on_toggle,  
                                                                    user_data
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Toggler: 2 parameters (id, user_data) are required or a python error in this function. {er}"),
                }
            } else {
                let res = callback.call1(py, (
                                                                    wco.id.clone(),
                                                                    wco.on_toggle,  
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Toggler: 1 parameter (id) is required or a python error in this function. {er}"),
                }
            } 
    });
    
    drop(app_cbs);
         
}


#[derive(Debug, Clone, PartialEq)]
#[pyclass]
pub enum IpgTogglerParams {
    Alignment,
    Label,
    LineHeight,
    Show,
    Size,
    TextSize,
    Width,
    WidthFill,
}


pub fn toggler_item_update(tog: &mut IpgToggler,
                            item: PyObject,
                            value: PyObject,
                            )
{
    let update = try_extract_toggler_update(item);
  
    match update {
        IpgTogglerParams::Label => {
            tog.label = Some(try_extract_string(value));
        },
        IpgTogglerParams::Show => {
            tog.show = try_extract_boolean(value);
        },
        IpgTogglerParams::Width => {
            let val = try_extract_f64(value);
            tog.width = get_width(Some(val as f32), false);
        },
        IpgTogglerParams::WidthFill => {
            let val = try_extract_boolean(value);
            tog.width = get_width(None, val);
        },
        IpgTogglerParams::Alignment => {
            let val: IpgAlignment = try_extract_ipg_alignment(value);
            tog.text_alignment = val;
        },
        IpgTogglerParams::LineHeight => {
            let val = try_extract_f64(value) as f32; 
            tog.text_line_height = LineHeight::Relative(val);
        },
        IpgTogglerParams::Size => {
            let val = try_extract_f64(value) as f32;
            tog.size = val;
        },
        IpgTogglerParams::TextSize => {
            let val = try_extract_f64(value) as f32;
            tog.text_size = val;
        },
    }

}


pub fn try_extract_toggler_update(update_obj: PyObject) -> IpgTogglerParams {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgTogglerParams>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Toggler update extraction failed"),
        }
    })
}

fn get_text_alignment(ta: IpgAlignment) -> alignment::Horizontal {
    match ta {
        IpgAlignment::Left => alignment::Horizontal::Left,
        IpgAlignment::Center => alignment::Horizontal::Center,
        IpgAlignment::Right => alignment::Horizontal::Right,
    }
}