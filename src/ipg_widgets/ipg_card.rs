//! ipg_card
use crate::app::Message;
use crate::ipg_widgets::helpers::{try_extract_boolean, try_extract_string};
use crate::{access_callbacks, IpgState};
use super::callbacks::{set_or_get_widget_callback_data, WidgetCallbackIn, WidgetCallbackOut
};
use super::helpers::try_extract_u64;

use iced::{Color, Element, Length, Padding};
use iced::widget::{Column, Space, Text};

use crate::iced_aw_widgets::card::Card;

use pyo3::{pyclass, PyObject, Python};


#[derive(Debug, Clone)]
pub struct IpgCard {
    pub id: usize,
    pub is_open: bool,
    
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
    pub style_id: Option<usize>,
    pub show: bool,
}

impl IpgCard {
    pub fn new( 
        id: usize,
        is_open: bool,
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
        style_id: Option<usize>,
        show: bool,
        ) -> Self {
        Self {
            id,
            is_open,
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
            style_id,
            show,
        }
    }
}

#[derive(Debug, Clone)]
pub enum CardMessage {
    OnClose,
}

#[derive(Debug, Clone, Default)]
pub struct IpgCardStyle {
    pub id: usize,
    pub background_color: Option<Color>,
}

impl IpgCardStyle {
    pub fn new(
        id: usize,
        background_color: Option<Color>,
    ) -> Self {
        Self {
            id,
            background_color,
        }
    }
}

// #[derive(Debug, Clone, PartialEq)]
// #[pyclass(eq, eq_int)]
// pub enum IpgCardStyle {
//     Primary,
//     Secondary,
//     Success,
//     Danger,
//     Warning,
//     Info,
//     Light,
//     Dark,
//     White,
//     Default,
// }

pub fn construct_card<'a>(crd: &'a IpgCard) -> Element<'a, Message> {

    if !crd.is_open {
        let sp: Element<CardMessage> = Space::new(0.0, 0.0).into();
        let sp_mapped: Element<Message> = sp.map(move |message| Message::Card(crd.id, message));
        return sp_mapped
    }

    let style = get_card_style_from_obj(&crd.style);

    let head: Element<CardMessage> = Text::new(crd.head.clone())
                                                .width(Length::Fill)
                                                .into();

    let body: Element<CardMessage> = Text::new(crd.body.clone())
                                                .width(Length::Fill)
                                                .into();

    let foot_opt: String= match &crd.foot {
                                        Some(foot) => foot.clone(),
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

    card.map(move |message| Message::Card(crd.id, message))
    
}

pub fn card_callback(state: &mut IpgState, id: usize, message: CardMessage) {
    match message {
        CardMessage::OnClose => {
            let wci: WidgetCallbackIn = 
                WidgetCallbackIn{
                    id,
                    value_bool: Some(false),
                    ..Default::default()
                };
            process_callback(id, "on_close".to_string());
        }
    }
}


pub fn process_callback(id: usize, event_name: String) 
{
    let app_cbs = access_callbacks();

    let callback_present = app_cbs.callbacks.get(&(id, event_name));

    let callback_opt = match callback_present {
        Some(cb) => cb,
        None => return,
    };
       
    let callback = match callback_opt {
        Some(cb) => cb,
        None => panic!("Card callback could not be found with id {}", id),
    };

    let user_data_opt = app_cbs.user_data.get(&id);

    Python::with_gil(|py| {
            if user_data_opt.is_some() {
                let res = callback.call1(py, (
                                                                    id,  
                                                                    user_data_opt.unwrap()
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Card: 2 parameters (id, user_data) are required or a python error in this function. {er}"),
                }
            } else {
                let res = callback.call1(py, (
                                                                    id,  
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Card: 1 parameter (id) is required or a python error in this function. {er}"),
                }
            } 
    });
    
    drop(app_cbs);
         
}


#[derive(Debug, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgCardParam {
    Head,
    Body,
    Foot,
    IsOpen,
    StyleId,
    Show,
}


pub fn card_item_update(crd: &mut IpgCard,
                            item: &PyObject,
                            value: &PyObject,
                            )
{
    let update = try_extract_card_update(item);
    let name = "Card".to_string();
    match update {
        IpgCardParam::Body => {
            crd.body = try_extract_string(value, name);
        },
        IpgCardParam::Foot => {
            crd.foot = Some(try_extract_string(value, name));
        },
        IpgCardParam::Head => {
            crd.head = try_extract_string(value, name);
        },
        IpgCardParam::IsOpen => {
            crd.is_open = try_extract_boolean(value, name);
        },
        IpgCardParam::StyleId => {
            crd.style_id = Some(try_extract_u64(value, name) as usize);
        },
        IpgCardParam::Show => {
            crd.show = try_extract_boolean(value, name);
        },
    }
}


// pub fn get_card_style_from_obj(style_opt: &Option<PyObject>) -> CardStyles {

//     let style_obj = match style_opt {
//         Some(st) => st,
//         None => return CardStyles::Primary,
//     };

//     let ipg_card_style = try_extract_card_style(style_obj);

//     match ipg_card_style {
//         IpgCardStyle::Primary => CardStyles::Primary,
//         IpgCardStyle::Secondary => CardStyles::Secondary, 
//         IpgCardStyle::Success => CardStyles::Success, 
//         IpgCardStyle::Danger => CardStyles::Danger, 
//         IpgCardStyle::Warning => CardStyles::Warning,
//         IpgCardStyle::Info => CardStyles::Info, 
//         IpgCardStyle::Light => CardStyles::Light, 
//         IpgCardStyle::Dark => CardStyles::Dark, 
//         IpgCardStyle::White => CardStyles::White, 
//         IpgCardStyle::Default => CardStyles::Default,
//     }
// }


pub fn try_extract_card_style(style_obj: &PyObject) -> IpgCardStyle {

    Python::with_gil(|py| {
        let res = style_obj.extract::<IpgCardStyle>(py);
        match res {
            Ok(st) => st,
            Err(_) => panic!("Card style failed to extract."),
        }
    })
}


pub fn try_extract_card_update(update_obj: &PyObject) -> IpgCardParam {
    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgCardParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Card update extraction failed."),
        }
    })
}
