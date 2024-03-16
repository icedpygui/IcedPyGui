
use crate::ipg_widgets::ipg_enums::{IpgWidgets, get_set_widget_data};
use crate::{access_state, access_callbacks};
use crate::app;

use iced::{Padding, Length, Element};
use iced::widget::PickList;
use iced::widget::text::{LineHeight, Shaping};

use pyo3::{PyObject, Python};


#[derive(Debug, Clone)]
pub struct IpgPickList {
    pub id: usize,
    pub show: bool,
    pub user_data: Option<PyObject>,

    pub options: Vec<String>,
    pub placeholder: Option<String>,
    pub selected: Option<String>,
    pub width: Length,
    pub padding: Padding,
    pub text_size: Option<f32>,
    pub text_line_height: LineHeight,
    pub text_shaping: Shaping,
    // font: Option<Renderer::Font>,
    // handle: Handle<Renderer::Font>,
    // style: <Renderer::Theme as StyleSheet>::Style,
    pub cb_name: Option<String>,
}

impl IpgPickList {
    pub fn new( 
        id: usize,
        show: bool,
        user_data: Option<PyObject>,

        options: Vec<String>,
        placeholder: Option<String>,
        selected: Option<String>,
        width: Length,
        padding: Padding,
        text_size: Option<f32>,
        text_line_height: LineHeight,
        text_shaping: Shaping,
        // font: Option<Renderer::Font>,
        // handle: Handle<Renderer::Font>,
        // style: <Renderer::Theme as StyleSheet>::Style,
        cb_name: Option<String>,
        ) -> Self {
        Self {
            id,
            show,
            user_data,
            options,
            placeholder,
            selected,
            width,
            padding,
            text_size,
            text_line_height,
            text_shaping,
            // font: Option<Renderer::Font>,
            // handle: Handle<Renderer::Font>,
            // style: <Renderer::Theme as StyleSheet>::Style,
            cb_name,
        }
    }
}

#[derive(Debug, Clone)]
pub enum PLMessage {
    Selected(String),
}

pub fn construct_picklist(pick: IpgPickList) -> Element<'static, app::Message> {

    let placeholder = match pick.placeholder.clone() {
        Some(holder) => holder,
        None => "".to_string(),
    };
    let text_size: f32 = match pick.text_size {
        Some(size) => size,
        None => 16.0,
    };

    let pl: Element<'_, PLMessage> = PickList::new(pick.options.clone(), 
                                            pick.selected.clone(), 
                                            PLMessage::Selected,
                                        )
                                        .placeholder(placeholder)
                                        .width(pick.width)
                                        .padding(pick.padding)
                                        .text_size(text_size)
                                        .text_line_height(pick.text_line_height)
                                        .text_shaping(pick.text_shaping)
                                        .into();

    pl.map(move |message| app::Message::PickList(pick.id, message))

}
 

 pub fn pick_list_update(id: usize, message: PLMessage) {

    match message {
        PLMessage::Selected(value) => {
            let (cb_name, user_data,_,_,_) = 
                            get_set_widget_data(
                                                id, 
                                                None,
                                                Some(value.clone()),
                                                None,
                                                None, 
                                                );
            
            process_callback(id, 
                            value.clone(), 
                            user_data,
                            cb_name, 
                            );
        },
    }
 }


fn process_callback(id: usize,
                    data: String, 
                    user_data: Option<PyObject>, 
                    cb_name: Option<String>) 
{
    if !cb_name.is_some() {return}

    let event_name = "Button_Pressed".to_string();

    let app_cbs = access_callbacks();

    let mut found_callback = None;

    for callback in app_cbs.callbacks.iter() {

        if id == callback.id && cb_name == callback.name {

            found_callback = match callback.cb.clone() {
                Some(cb) => Some(cb),
                None => {drop(app_cbs); panic!("Callback could not be found with id {}", id)},
            };
            break;
        }                   
    };
    drop(app_cbs);

    match found_callback {

        Some(cb) => Python::with_gil(|py| {
            if user_data.is_some() {
                cb.call1(py, 
                        (id.clone(),
                                event_name,
                                data, 
                                user_data,
                        )).unwrap();
            } else {
                cb.call1(py, 
                    (id.clone(),
                            event_name,
                            data, 
                    )).unwrap();
            }
                                    
                            }),
        None => panic!("Picklist callback unable to return call"),
    };
}    