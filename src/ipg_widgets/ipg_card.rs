#![allow(unused_imports)]

use crate::app::Message;
use crate::{access_callbacks, UpdateItems, delete_item};
use super::callbacks::{WidgetCallbackIn, 
WidgetCallbackOut, get_set_widget_callback_data,
};

use iced::{Element, Length, Padding};
use iced::widget::{Column, Space, Text};

use iced_aw::{Card, CardStyles};

use pyo3::{pyclass, PyObject, Python};


#[derive(Debug, Clone)]
pub struct IpgCard {
    pub id: usize,
    pub is_open: bool,
    pub user_data: Option<PyObject>,
    
    pub button_id: Option<usize>,
    pub width: Length,
    pub height: Length,
    pub max_width: f32,
    pub max_height: f32,
    pub padding_head: Padding,
    pub padding_body: Padding,
    pub padding_foot: Padding,
    pub close_size: f32,
    pub head: String,
    pub body: String,
    pub foot: Option<String>,
    pub style: Option<String>,
}

impl IpgCard {
    pub fn new( 
        id: usize,
        is_open: bool,
        user_data: Option<PyObject>,
        minmax_id: Option<usize>,
        width: Length,
        height: Length,
        max_width: f32,
        max_height: f32,
        padding_head: Padding,
        padding_body: Padding,
        padding_foot: Padding,
        close_size: f32,
        head: String,
        body: String,
        foot: Option<String>,
        style: Option<String>,
        ) -> Self {
        Self {
            id,
            is_open,
            user_data,
            button_id: minmax_id,
            width,
            height,
            max_width,
            max_height,
            padding_head,
            padding_body,
            padding_foot,
            close_size,
            head,
            body,
            foot,
            style,
        }
    }
}

#[derive(Debug, Clone)]
pub enum CardMessage {
    OnClose,
}

// The style enums below are different than iced_aw CardStyles enums though they have the
// same members.  The reason is that the iced_aw CardStyles don't have a Clone method 
// and the python styles are defined as IpgCardStyles. Therefore
// one has to send a Option<String> representing the style, using an IpgCardStyles enum.
// Steps are different based on the intitial contruction and the updating routine.
// 
// Construction phase: 
// lib.add_card() ==> PyObject ==> String ==> construct_card() ==> iced_aw style
// 
// Update phase: 
// lib.update_item() ==> PyObject ==> try_extract (method below) ==> Option<String> returned to update_item
// lib.update_item() => iced update => construction phase.

#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgCardStyles {
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
    Light,
    Dark,
    White,
    Default,
}

pub fn construct_card (crd: IpgCard) -> Element<'static, Message> {

    if !crd.is_open {
        let sp: Element<CardMessage> = Space::new(0.0, 0.0).into();
        let sp_mapped: Element<Message> = sp.map(move |message| Message::Card(crd.id, message)).into();
        return sp_mapped
    }

    let style = get_card_style_from_str(crd.style);

    let head: Element<CardMessage> = Text::new(crd.head.clone())
                                                .width(Length::Fill)
                                                .into();

    let body: Element<CardMessage> = Text::new(crd.body.clone())
                                                .width(Length::Fill)
                                                .into();

    let foot_opt: String= match crd.foot {
                                        Some(foot) => foot,
                                        None => "".to_string(),
                                    };

    let foot: Element<CardMessage> = Text::new(foot_opt.clone())
                                            .width(Length::Fill)
                                            .into();

    let body: Element<CardMessage> = Column::new().push(body).into();

    let card: Element<CardMessage> = Card::new(head, body)
                                                .foot(foot)
                                                .width(crd.width)
                                                .height(crd.height)
                                                .max_width(crd.max_width)
                                                .max_height(crd.max_height)
                                                .padding_head(crd.padding_head)
                                                .padding_body(crd.padding_body)
                                                .padding_foot(crd.padding_foot)
                                                .close_size(crd.close_size)
                                                .on_close(CardMessage::OnClose)
                                                .style(style)
                                                .into();

    let card_mapped: Element<'static, Message> = card.map(move |message| Message::Card(crd.id, message));
    
    card_mapped

}

pub fn card_callback(id: usize, message: CardMessage) {
    match message {
        CardMessage::OnClose => {
            let mut wci: WidgetCallbackIn = WidgetCallbackIn::default();
            wci.id = id;
            wci.value_bool = Some(false);
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = "on_close".to_string();
            process_callback(wco);
        }
    }
}


pub fn process_callback(wco: WidgetCallbackOut) 
{
    let app_cbs = access_callbacks();

    let callback_present = app_cbs.callbacks.get(&(wco.id, wco.event_name));

    let callback_opt = match callback_present {
        Some(cb) => cb,
        None => return,
    };
       
    let callback = match callback_opt {
        Some(cb) => cb,
        None => panic!("Card callback could not be found with id {}", wco.id),
    };

    Python::with_gil(|py| {
            if wco.user_data.is_some() {
                let user_data = match wco.user_data {
                    Some(ud) => ud,
                    None => panic!("User Data could not be found in Card callback"),
                };
                callback.call1(py, (
                                        wco.id.clone(),  
                                        user_data
                                        )
                                ).unwrap();
            } else {
                callback.call1(py, (
                                        wco.id.clone(),  
                                        )
                                ).unwrap();
            } 
    });
    
    drop(app_cbs);
         
}


pub fn card_item_update(crd: &mut IpgCard,
                            item: String,
                            items: UpdateItems,
                            )
{
    if item == "head".to_string() {
        crd.head = match items.value_str {
            Some(head) => head,
            None => panic!("A string value is needed to update Card head."),
        };
        return
    }

    if item == "body".to_string() {
        crd.body = match items.value_str {
            Some(body) => body,
            None => panic!("A string value is needed to update Card body."),
        };
        return
    }

    if item == "foot".to_string() {
        crd.foot = match items.value_str {
            Some(foot) => Some(foot),
            None => panic!("A string value is needed to update Card foot."),
        };
        return
    }

    if item == "is_open".to_string() {
        crd.is_open = match items.value_bool {
            Some(open) => open,
            None => panic!("A boolean value is needed to update card is_open"),
        };
        return
    }

    if item == "style".to_string() {
        crd.style = match items.value_str {
            Some(st) => Some(st),
            None => panic!("Style must be of type string.")
        };
        return
    }

    panic!("Card update item >{}< could not be found", item)

}


pub fn get_card_style_from_str(style_opt: Option<String>) -> CardStyles {

    let style_str = match style_opt {
        Some(st) => st,
        None => return CardStyles::Primary,
    };

    match style_str.as_str() {
        "Primary" => CardStyles::Primary,
        "Secondary" => CardStyles::Secondary, 
        "Success" => CardStyles::Success, 
        "Danger" => CardStyles::Danger, 
        "Warning" => CardStyles::Warning,
        "Info" => CardStyles::Info, 
        "Light" => CardStyles::Light, 
        "Dark" => CardStyles::Dark, 
        "White" => CardStyles::White, 
        "Default" => CardStyles::Default,
        _ => CardStyles::Default,
    }
}

pub fn get_card_str_from_style(style: IpgCardStyles) -> Option<String> {
        match style {
            IpgCardStyles::Primary => Some("Primary".to_string()),
            IpgCardStyles::Secondary => Some("Secondary".to_string()), 
            IpgCardStyles::Success => Some("Success".to_string()), 
            IpgCardStyles::Danger => Some("Danger".to_string()), 
            IpgCardStyles::Warning => Some("Warning".to_string()),
            IpgCardStyles::Info => Some("Info".to_string()), 
            IpgCardStyles::Light => Some("Light".to_string()), 
            IpgCardStyles::Dark => Some("Dark".to_string()), 
            IpgCardStyles::White => Some("White".to_string()), 
            IpgCardStyles::Default => Some("Default".to_string()),
        }
}

pub fn try_extract_card_style(style_obj: PyObject, py: Python<'_>) -> Option<String> {

    let mut style: Option<String> = None;

    let res = style_obj.extract::<IpgCardStyles>(py);
            if !res.is_err() {
                style = match res {
                    Ok(st) => get_card_str_from_style(st),
                    Err(_) => None,
                }
            }

    style
}

