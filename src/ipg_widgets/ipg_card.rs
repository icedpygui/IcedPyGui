
use crate::ipg_widgets::ipg_enums::{IpgWidgets, get_set_widget_data};
use crate::app::Message;
use crate::{access_callbacks, access_state, UpdateItems};

use iced::widget::shader::wgpu::core::gfx_if_metal_hidden;
use iced::{Element, Length, Padding};
use iced::widget::{Column, Text};

use iced_aw::{style, Card, CardStyles};

use pyo3::{pyclass, PyObject, Python};


#[derive(Debug, Clone)]
pub struct IpgCard {
    pub id: usize,
    pub show: bool,
    pub user_data: Option<PyObject>,
    
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
    pub cb_name: Option<String>,
}

impl IpgCard {
    pub fn new( 
        id: usize,
        show: bool,
        user_data: Option<PyObject>,
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
        cb_name: Option<String>,
        ) -> Self {
        Self {
            id,
            show,
            user_data,
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
            cb_name,
        }
    }
}

#[derive(Debug, Clone)]
pub enum CardMessage {
    OnClose(usize),
}

// The enums below are different than iced_aw CardStyles enums though they have the
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
                                                .on_close(CardMessage::OnClose(crd.id.clone()))
                                                .style(style)
                                                .into();

    let card_mapped: Element<'static, Message> = card.map(move |message| Message::Card(crd.id, message));
    card_mapped
}

pub fn card_update(id: usize, message: CardMessage) {
    match message {
        CardMessage::OnClose(id) => {

            let (cb_name, user_data,_,_,_) = 
                                            get_set_widget_data(
                                                                id, 
                                                                None,
                                                                None, 
                                                                None,
                                                                None, 
                                                                );
            
            let event_name = "Card_Closed".to_string();

            process_callback(
                            id, 
                            event_name, 
                            user_data,
                            cb_name
                            );
        }
    }
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

    if item == "style".to_string() {
        crd.style = match items.value_str {
            Some(st) => Some(st),
            None => panic!("Style must be of type string.")
        };
        return
    }

    panic!("Card update item {} could not be found", item)

}

fn process_callback(
                    id: usize, 
                    event_name: String,
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
    None => panic!("Checkbox callback not found"),
    };

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

