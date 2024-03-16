
use crate::ipg_widgets::ipg_enums::{IpgWidgets, get_set_widget_data};
use crate::app::Message;
use crate::{access_state, access_callbacks};

use iced::{Element, Length, Padding};
use iced::widget::{Column, Text};

use iced_aw::{style, Card, CardStyles};

use pyo3::{Python, PyObject};


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

pub fn construct_card (crd: IpgCard) -> Element<'static, Message> {

    let head: Element<CardMessage> = Text::new(crd.head.clone()).width(Length::Fill).into();

    let body: Element<CardMessage> = Column::new().push(Text::new(crd.body.clone())).into();

    let style = match_style(crd.style.clone());

    let card: Element<CardMessage> = Card::new(head, body)
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

fn match_style(style_opt: Option<String>) -> CardStyles {
    
    let style = match style_opt {
        Some(s) => s,
        None => "default".to_string(),
    };

    match style.as_str() {
        "primary" => CardStyles::Primary,
        "secondary" => CardStyles::Secondary, 
        "success" => CardStyles::Success, 
        "danger" => CardStyles::Danger, 
        "warning" => CardStyles::Warning,
        "info" => CardStyles::Info, 
        "light" => CardStyles::Light, 
        "dark" => CardStyles::Dark, 
        "white" => CardStyles::White, 
        "default" => CardStyles::Default,
        _ => panic!("No matching style found for Card, checked the docs.")
    }
}