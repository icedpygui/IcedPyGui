//! ipg_card
use crate::app::Message;
use crate::graphics::colors::get_color;
use crate::ipg_widgets::helpers::{try_extract_boolean, try_extract_string};
use crate::{access_callbacks, IpgState};
use super::callbacks::WidgetCallbackIn;
use super::helpers::{try_extract_f64, try_extract_ipg_color, try_extract_rgba_color, try_extract_u64};
use super::ipg_enums::IpgWidgets;

use iced::{Color, Element, Length, Padding};
use iced::widget::{Column, Space, Text};

use crate::iced_aw_widgets::card::{self, Card, CardStyles};

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

#[derive(Debug, Clone)]
pub struct IpgCardStyle {
    pub id: usize,
    pub background: Option<Color>, 
    pub border_radius: f32, 
    pub border_width: f32, 
    pub border_color: Option<Color>, 
    pub head_background: Option<Color>, 
    pub head_text_color: Option<Color>, 
    pub body_background: Option<Color>, 
    pub body_text_color: Option<Color>, 
    pub foot_background: Option<Color>, 
    pub foot_text_color: Option<Color>, 
    pub close_color:Option<Color>,
}

impl IpgCardStyle {
    pub fn new(
        id: usize,
        background: Option<Color>, 
        border_radius: f32, 
        border_width: f32, 
        border_color: Option<Color>, 
        head_background: Option<Color>, 
        head_text_color: Option<Color>, 
        body_background: Option<Color>, 
        body_text_color: Option<Color>, 
        foot_background: Option<Color>, 
        foot_text_color: Option<Color>, 
        close_color:Option<Color>,
    ) -> Self {
        Self {
            id,
            background,
            border_radius,
            border_width,
            border_color,
            head_background, 
            head_text_color, 
            body_background, 
            body_text_color, 
            foot_background, 
            foot_text_color, 
            close_color,
        }
    }
}


pub fn construct_card<'a>(crd: &'a IpgCard,
                            style_opt: Option<&'a IpgWidgets>) 
                            -> Option<Element<'a, Message>> {

    if !crd.show {return None}
    if !crd.is_open {
        let sp: Element<CardMessage> = Space::new(0.0, 0.0).into();
        let sp_mapped: Element<Message> = sp.map(move |message| Message::Card(crd.id, message));
        return Some(sp_mapped)
    }

    let card_style = get_card_style(style_opt);

    let style = custom_style(card_style);

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

    Some(card.map(move |message| Message::Card(crd.id, message)))
    
}

pub fn card_callback(_state: &mut IpgState, id: usize, message: CardMessage) {
    match message {
        CardMessage::OnClose => {
            let _ = 
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


#[derive(Debug, Clone, PartialEq)]
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

pub fn get_card_style(style: Option<&IpgWidgets>) -> Option<IpgCardStyle>{
    match style {
        Some(st) => {
            match st {
                IpgWidgets::IpgCardStyle(style) => {
                    Some(style.clone())
                }
                _ => None,
            }
        },
        None => None,
    }
}

fn custom_style(ipg_style_opt: Option<IpgCardStyle>) -> CardStyles {

    let ipg_style = if ipg_style_opt.is_none() {
        IpgCardStyle{
            id: 0,
            background: None,
            border_radius: 10.0,
            border_width: 1.0,
            border_color: None,
            head_background: None,
            head_text_color: None,
            body_background: None,
            body_text_color: None,
            foot_background: None,
            foot_text_color: None,
            close_color: None,
        }
    } else {
        ipg_style_opt.unwrap()
    };

    let background = ipg_style.background.unwrap_or_else(||Color::WHITE).into();
    let border_radius = ipg_style.border_radius;
    let border_width = ipg_style.border_width;
    let border_color = ipg_style.border_color.unwrap_or_else(||[0.87, 0.87, 0.87].into()).into();
    let head_background = ipg_style.head_background.unwrap_or_else(||[0.87, 0.87, 0.87].into()).into();
    let head_text_color = ipg_style.head_text_color.unwrap_or_else(||Color::BLACK);
    let body_background = ipg_style.body_background.unwrap_or_else(||Color::TRANSPARENT).into();
    let body_text_color = ipg_style.body_text_color.unwrap_or_else(||Color::BLACK);
    let foot_background = ipg_style.foot_background.unwrap_or_else(||Color::TRANSPARENT).into();
    let foot_text_color = ipg_style.foot_text_color.unwrap_or_else(||Color::BLACK);
    let close_color = ipg_style.close_color.unwrap_or_else(||Color::BLACK);

    let custom= card::Appearance{ 
        background, 
        border_radius, 
        border_width, 
        border_color, 
        head_background, 
        head_text_color, 
        body_background, 
        body_text_color, 
        foot_background, 
        foot_text_color, 
        close_color };


    CardStyles::Custom(custom)

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

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgCardStyleParam {
    BackgroundIpgColor,
    BackgroundRgbaColor,
    BorderIpgColor,
    BorderRgbaColor,
    BorderRadius,
    BorderWidth,
    HeadBackgroundIpgColor,
    HeadBackgroundRgbaColor,
    HeadTextIpgColor,
    HeadTextRgbaColor,
    BodyBackgroundIpgColor,
    BodyBackgroundRgbaColor,
    BodyTextIpgColor,
    BodyTextRgbaColor,
    FootBackgroundIpgColor,
    FootBackgroundRgbaColor,
    FootTextIpgColor,
    FootTextRgbaColor,
    CloseIpgColor,
    CloseRgbaColor,
}

pub fn card_style_update(style: &mut IpgCardStyle,
                        item: &PyObject,
                        value: &PyObject,) {

    let update = try_extract_card_style_update(item);
    let name = "Card Style".to_string();
    
    match update {
        IpgCardStyleParam::BackgroundIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.background = get_color(None, Some(color), 1.0, false);
        },
        IpgCardStyleParam::BackgroundRgbaColor => {
            style.background = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgCardStyleParam::BorderIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.border_color = get_color(None, Some(color), 1.0, false);
        },
        IpgCardStyleParam::BorderRgbaColor => {
            style.border_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgCardStyleParam::BorderRadius => {
            style.border_radius = try_extract_f64(value, name) as f32;
        },
        IpgCardStyleParam::BorderWidth => {
            style.border_width = try_extract_f64(value, name) as f32;
        },
        IpgCardStyleParam::HeadBackgroundIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.head_background = get_color(None, Some(color), 1.0, false);
        },
        IpgCardStyleParam::HeadBackgroundRgbaColor => {
             style.head_background = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgCardStyleParam::HeadTextIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.head_text_color = get_color(None, Some(color), 1.0, false);
        },
        IpgCardStyleParam::HeadTextRgbaColor => {
             style.head_text_color= Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgCardStyleParam::BodyBackgroundIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.body_background = get_color(None, Some(color), 1.0, false);
        },
        IpgCardStyleParam::BodyBackgroundRgbaColor => {
             style.body_background = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgCardStyleParam::BodyTextIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.body_text_color = get_color(None, Some(color), 1.0, false);
        },
        IpgCardStyleParam::BodyTextRgbaColor => {
             style.body_text_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgCardStyleParam::FootBackgroundIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.foot_background = get_color(None, Some(color), 1.0, false);
        },
        IpgCardStyleParam::FootBackgroundRgbaColor => {
             style.foot_background = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgCardStyleParam::FootTextIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.foot_text_color = get_color(None, Some(color), 1.0, false);
        },
        IpgCardStyleParam::FootTextRgbaColor => {
             style.foot_text_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgCardStyleParam::CloseIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.close_color = get_color(None, Some(color), 1.0, false);
        },
        IpgCardStyleParam::CloseRgbaColor => {
             style.close_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
    }
}

fn try_extract_card_style_update(update_obj: &PyObject) -> IpgCardStyleParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgCardStyleParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Card style parameter update extraction failed"),
        }
    })
}
