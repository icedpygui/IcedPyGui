

use crate::access_callbacks;
use crate::app;
use super::callbacks::{WidgetCallbackIn, 
                        WidgetCallbackOut, 
                        get_set_widget_callback_data};

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
        }
    }
}

#[derive(Debug, Clone)]
pub enum PLMessage {
    OnSelect(String),
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
                                            PLMessage::OnSelect,
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
 

 pub fn pick_list_callback(id: usize, message: PLMessage) {
    let mut wci: WidgetCallbackIn = WidgetCallbackIn::default();
    wci.id = id;

    match message {
        PLMessage::OnSelect(value) => {
            wci.value_str = Some(value);
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = Some("on_select".to_string());
            process_callback(wco);
        },
    }
 }


 pub fn process_callback(wco: WidgetCallbackOut) 
 {
     if !wco.event_name.is_some() {return}
 
     let evt_name = match wco.event_name {
         Some(name) => name,
         None => panic!("event_name not found")
     };
 
     let app_cbs = access_callbacks();
 
     let callback_opt = app_cbs.callbacks.get(&(wco.id, evt_name.clone())).unwrap();
        
     let callback = match callback_opt {
         Some(cb) => cb,
         None => panic!("Callback could not be found with id {}", wco.id),
     };
                   
     Python::with_gil(|py| {
             if wco.user_data.is_some() {
                 callback.call1(py, (
                                         wco.id.clone(), 
                                         evt_name.clone(),
                                         wco.value_str, 
                                         wco.user_data
                                         )
                                 ).unwrap();
             } else {
                 callback.call1(py, (
                                         wco.id.clone(), 
                                         evt_name.clone(),
                                         wco.value_str, 
                                         )
                                 ).unwrap();
             } 
     });

     drop(app_cbs); 

 }