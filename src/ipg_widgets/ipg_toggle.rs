//! ipg_toggler
use crate::{access_callbacks, access_state, app};
use super::helpers::{get_width, try_extract_boolean, try_extract_f64, 
    try_extract_ipg_horizontal_alignment, try_extract_string};
use super::callbacks::{
    WidgetCallbackIn, WidgetCallbackOut, 
    get_set_widget_callback_data
};
use super::ipg_enums::IpgHorizontalAlignment;
use iced::widget::text::LineHeight;
use iced::widget::toggler::{self, Status};
use pyo3::{pyclass, PyObject, Python};

use iced::widget::{Space, Toggler};
use iced::{alignment, Color, Element, Length, Theme};



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
    pub text_alignment: IpgHorizontalAlignment,
    pub spacing: f32,
    pub style_id: Option<String>,
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
        text_alignment: IpgHorizontalAlignment,
        spacing: f32,
        style_id: Option<String>,
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
            style_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IpgTogglerStyle {
    pub id: usize,
    pub background_color: Option<Color>,
    pub background_color_toggled: Option<Color>,
    pub background_color_disabled: Option<Color>,
    pub background_border_color: Option<Color>,
    pub background_border_width: Option<f32>,
    pub foreground_color: Option<Color>,
    pub foreground_color_toggled: Option<Color>,
    pub foreground_color_disabled: Option<Color>,
    pub foreground_border_color: Option<Color>,
    pub foreground_border_width: Option<f32>,
}

impl IpgTogglerStyle {
    pub fn new(
        id: usize,
        background_color: Option<Color>,
        background_color_toggled: Option<Color>,
        background_color_disabled: Option<Color>,
        background_border_color: Option<Color>,
        background_border_width: Option<f32>,
        foreground_color: Option<Color>,
        foreground_color_toggled: Option<Color>,
        foreground_color_disabled: Option<Color>,
        foreground_border_color: Option<Color>,
        foreground_border_width: Option<f32>,
    ) -> Self {
        Self {
            id,
            background_color,
            background_color_toggled,
            background_color_disabled,
            background_border_color,
            background_border_width,
            foreground_color,
            foreground_color_toggled,
            foreground_color_disabled,
            foreground_border_color,
            foreground_border_width,
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

    let label = match tog.label {
        Some(label) => label,
        None => "".to_string(),
    };

    let ipg_tog: Element<TOGMessage> = Toggler::new(tog.is_toggled)
                                                    .label(label)
                                                    .on_toggle(TOGMessage::Toggled)
                                                    .size(tog.size)
                                                    .width(tog.width)
                                                    .text_size(tog.text_size)
                                                    .text_line_height(tog.text_line_height)
                                                    .text_alignment(text_alignment)
                                                    .spacing(tog.spacing)
                                                    .style(move|theme: &Theme, status| {     
                                                        get_styling(theme, status, 
                                                                    tog.style_id.clone()) 
                                                    })
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
pub enum IpgTogglerParam {
    HorizontalAlignment,
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
        IpgTogglerParam::Label => {
            tog.label = Some(try_extract_string(value));
        },
        IpgTogglerParam::Show => {
            tog.show = try_extract_boolean(value);
        },
        IpgTogglerParam::Width => {
            let val = try_extract_f64(value);
            tog.width = get_width(Some(val as f32), false);
        },
        IpgTogglerParam::WidthFill => {
            let val = try_extract_boolean(value);
            tog.width = get_width(None, val);
        },
        IpgTogglerParam::HorizontalAlignment => {
            let val: IpgHorizontalAlignment = try_extract_ipg_horizontal_alignment(value);
            tog.text_alignment = val;
        },
        IpgTogglerParam::LineHeight => {
            let val = try_extract_f64(value) as f32; 
            tog.text_line_height = LineHeight::Relative(val);
        },
        IpgTogglerParam::Size => {
            let val = try_extract_f64(value) as f32;
            tog.size = val;
        },
        IpgTogglerParam::TextSize => {
            let val = try_extract_f64(value) as f32;
            tog.text_size = val;
        },
    }

}


pub fn try_extract_toggler_update(update_obj: PyObject) -> IpgTogglerParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgTogglerParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Toggler update extraction failed"),
        }
    })
}

fn get_text_alignment(ta: IpgHorizontalAlignment) -> alignment::Horizontal {
    match ta {
        IpgHorizontalAlignment::Left => alignment::Horizontal::Left,
        IpgHorizontalAlignment::Center => alignment::Horizontal::Center,
        IpgHorizontalAlignment::Right => alignment::Horizontal::Right,
    }
}



pub fn get_styling(theme: &Theme, status: Status, 
                    style_id: Option<String>,
                    ) -> toggler::Style {
    
    let mut tog_style = toggler::default(theme, status);

    let state = access_state();

    if style_id.is_none() {
        return tog_style
    }

    let style_opt = state.toggler_style.get(&style_id.clone().unwrap());
    
    let style = match style_opt {
        Some(st) => st,
        None => panic!("Toggler: The style_id '{}' for add_toggler_style could not be found", style_id.unwrap())
    };

    // The background color for active or hovered can have two colors, one for untoggled and toggled.
    // The relationship of the bg and fg colors is:
    // Untoggled: bg=color.strong & fg=color.base
    // Toggled: bg=color & fg=contrasting color  
    if style.background_color.is_some() {
        tog_style.background = style.background_color.unwrap().into();
    }

    if style.foreground_color.is_some() {
        tog_style.foreground = style.foreground_color.unwrap().into();
    }
    
    // background and foreground border color is the same for active, hover and toggled
    if style.background_border_color.is_some() {
        tog_style.background_border_color = style.background_border_color.unwrap();
    }

    if style.background_border_width.is_some() {
        tog_style.background_border_width = style.background_border_width.unwrap();
    }
    
    if style.foreground_border_color.is_some() {
        tog_style.foreground_border_color = style.foreground_border_color.unwrap();
    }

    if style.foreground_border_width.is_some() {
        tog_style.foreground_border_width = style.foreground_border_width.unwrap();
    }
        
    match status {
        Status::Active { is_toggled } | Status::Hovered { is_toggled } => {
            if is_toggled {
                if style.background_color_toggled.is_some() {
                    tog_style.background = style.background_color_toggled.unwrap().into();
                }
            }
        }
        Status::Disabled => todo!(),
    }

    match status {
        Status::Active { is_toggled } => {
            if is_toggled {
                if style.foreground_color_toggled.is_some() {
                    tog_style.foreground = style.foreground_color_toggled.unwrap().into();
                }
            }
        }
        Status::Hovered { is_toggled } => {
            if is_toggled {
                if style.foreground_color_toggled.is_some() {
                    tog_style.foreground = Color {
                                                a: 0.5,
                                                ..style.foreground_color_toggled.unwrap().into()
                                            };
                }
                
            }
        }
        Status::Disabled => todo!(),
    }

    tog_style

}
