#![allow(dead_code)]

use crate::{access_callbacks, app};
use super::helpers::{get_width, 
                    try_extract_f64, try_extract_string, 
                    try_extract_boolean};
use super::callbacks::{
    WidgetCallbackIn, WidgetCallbackOut, 
    get_set_widget_callback_data
};

use pyo3::{pyclass, PyObject, Python};

use iced::widget::{Space, Toggler};
use iced::{Element, Length};


#[derive(Debug, Clone)]
pub struct IpgToggler {
    pub id: usize,
    pub show: bool,
    pub user_data: Option<PyObject>,

    pub is_toggled: bool,
    pub label: Option<String>,
    pub width: Length,
}

impl IpgToggler {
    pub fn new( 
        id: usize,
        show: bool,
        user_data: Option<PyObject>,

        label: Option<String>,
        width: Length,
        ) -> Self {
        Self {
            id,
            show,
            user_data,
            is_toggled: false,
            label,
            width,
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

    let ipg_tog: Element<TOGMessage> = Toggler::new(tog.label, tog.is_toggled, TOGMessage::Toggled)
                                                    .width(tog.width)
                                                    .into();

    ipg_tog.map(move |message| app::Message::Toggler(tog.id, message))
}


pub fn toggle_callback(id: usize, message: TOGMessage) {

    let mut wci = WidgetCallbackIn::default();
    wci.id = id;

    match message {
        TOGMessage::Toggled(is_toggled) => {
            wci.is_toggled = Some(is_toggled);
            let mut wco: WidgetCallbackOut = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.is_toggled = Some(is_toggled);
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
                                                                    wco.is_toggled,  
                                                                    user_data
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(_) => panic!("Toggler: 2 parameters (id, user_data) are required or possibly a non-fatal python error in this function."),
                }
            } else {
                let res = callback.call1(py, (
                                                                    wco.id.clone(),
                                                                    wco.is_toggled,  
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(_) => panic!("Toggler: 1 parameter (id) is required or possibly a non-fatal python error in this function."),
                }
            } 
    });
    
    drop(app_cbs);
         
}


#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgTogglerParams {
    Label,
    Show,
    Width,
    WidthFill,
}


pub fn button_item_update(tog: &mut IpgToggler,
                            item: PyObject,
                            value: PyObject,
                            )
{
    let update = try_extract_toggle_update(item);

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
    }

}


pub fn try_extract_toggle_update(update_obj: PyObject) -> IpgTogglerParams {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgTogglerParams>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Toggler update extraction failed"),
        }
    })
}
