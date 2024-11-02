//! ipg_card
use crate::app::Message;
use crate::ipg_widgets::helpers::{try_extract_boolean, try_extract_string};
use crate::{access_callbacks, IpgState};
use super::callbacks::{widget_callback_data, WidgetCallbackIn, WidgetCallbackOut
};

use iced::{Element, Length, Padding};
use iced::widget::{Column, Space, Text};

use crate::iced_aw_widgets::card::{Card, CardStyles};

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
    pub style: Option<PyObject>,
}

impl IpgCard {
    pub fn new( 
        id: usize,
        is_open: bool,
        user_data: Option<PyObject>,
        min_max_id: Option<usize>,
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
        style: Option<PyObject>,
        ) -> Self {
        Self {
            id,
            is_open,
            user_data,
            button_id: min_max_id,
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


#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgCardStyle {
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

    let style = get_card_style_from_obj(crd.style);

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

pub fn card_callback(state: &mut IpgState, id: usize, message: CardMessage) {
    match message {
        CardMessage::OnClose => {
            let mut wci: WidgetCallbackIn = WidgetCallbackIn::default();
            wci.id = id;
            wci.value_bool = Some(false);
            let mut wco = widget_callback_data(state, wci);
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
                let res = callback.call1(py, (
                                                                    wco.id.clone(),  
                                                                    user_data
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Card: 2 parameters (id, user_data) are required or a python error in this function. {er}"),
                }
            } else {
                let res = callback.call1(py, (
                                                                    wco.id.clone(),  
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Card: 1 parameter (id) is required or a python error in this function. {er}"),
                }
            } 
    });
    
    drop(app_cbs);
         
}


#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgCardParam {
    Head,
    Body,
    Foot,
    IsOpen,
    Style,
}


pub fn card_item_update(crd: &mut IpgCard,
                            item: PyObject,
                            value: PyObject,
                            )
{
    let update = try_extract_card_update(item);

    match update {
        IpgCardParam::Body => {
            crd.body = try_extract_string(value);
        },
        IpgCardParam::Foot => {
            crd.foot = Some(try_extract_string(value));
        },
        IpgCardParam::Head => {
            crd.head = try_extract_string(value);
        },
        IpgCardParam::IsOpen => {
            crd.is_open = try_extract_boolean(value);
        },
        IpgCardParam::Style => {
            crd.style = Some(value);
        },
    }
}


pub fn get_card_style_from_obj(style_opt: Option<PyObject>) -> CardStyles {

    let style_obj = match style_opt {
        Some(st) => st,
        None => return CardStyles::Primary,
    };

    let ipg_card_style = try_extract_card_style(style_obj);

    match ipg_card_style {
        IpgCardStyle::Primary => CardStyles::Primary,
        IpgCardStyle::Secondary => CardStyles::Secondary, 
        IpgCardStyle::Success => CardStyles::Success, 
        IpgCardStyle::Danger => CardStyles::Danger, 
        IpgCardStyle::Warning => CardStyles::Warning,
        IpgCardStyle::Info => CardStyles::Info, 
        IpgCardStyle::Light => CardStyles::Light, 
        IpgCardStyle::Dark => CardStyles::Dark, 
        IpgCardStyle::White => CardStyles::White, 
        IpgCardStyle::Default => CardStyles::Default,
    }
}


pub fn try_extract_card_style(style_obj: PyObject) -> IpgCardStyle {

    Python::with_gil(|py| {
        let res = style_obj.extract::<IpgCardStyle>(py);
        match res {
            Ok(st) => st,
            Err(_) => panic!("Card style failed to extract."),
        }
    })
}


pub fn try_extract_card_update(update_obj: PyObject) -> IpgCardParam {
    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgCardParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Card update extraction failed."),
        }
    })
}
