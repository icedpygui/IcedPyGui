//! ipg_toggler
use crate::graphics::colors::get_color;
use crate::{access_callbacks, access_user_data1, app, IpgState};
use super::helpers::{get_width, try_extract_boolean, 
    try_extract_f64, try_extract_ipg_color, 
    try_extract_ipg_horizontal_alignment, 
    try_extract_rgba_color, try_extract_string};
use super::callbacks::{set_or_get_widget_callback_data, WidgetCallbackIn};
use super::ipg_enums::{IpgHorizontalAlignment, IpgWidgets};
use iced::widget::text::LineHeight;
use iced::widget::toggler::{self, Status};
use pyo3::{pyclass, PyObject, Python};

use iced::widget::Toggler;
use iced::{alignment, Color, Element, Length, Theme};


#[derive(Debug, Clone)]
pub struct IpgToggler {
    pub id: usize,
    pub parent_id: String,
    pub show: bool,
    pub is_toggled: bool,
    pub label: Option<String>,
    pub width: Length,
    pub size: f32,
    pub text_size: f32,
    pub text_line_height: LineHeight,
    pub text_alignment: IpgHorizontalAlignment,
    pub spacing: f32,
    pub style_id: Option<usize>,
}

impl IpgToggler {
    pub fn new( 
        id: usize,
        parent_id: String,
        show: bool,
        label: Option<String>,
        width: Length,
        size: f32,
        text_size: f32,
        text_line_height: LineHeight,
        text_alignment: IpgHorizontalAlignment,
        spacing: f32,
        style_id: Option<usize>,
        ) -> Self {
        Self {
            id,
            parent_id,
            show,
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


pub fn construct_toggler<'a>(tog: &'a IpgToggler, 
                        style_opt: Option<&IpgWidgets>) 
                        -> Option<Element<'a, app::Message>> {
    
    if !tog.show {
        return None
    }

    let style = get_toggler_style(style_opt);

    let text_alignment = get_text_alignment(&tog.text_alignment);

    let label = match &tog.label {
        Some(label) => label,
        None => &"".to_string(),
    };

    let ipg_tog: Element<TOGMessage> = Toggler::new(tog.is_toggled)
                                                    .label(label.clone())
                                                    .on_toggle(TOGMessage::Toggled)
                                                    .size(tog.size)
                                                    .width(tog.width)
                                                    .text_size(tog.text_size)
                                                    .text_line_height(tog.text_line_height)
                                                    .text_alignment(text_alignment)
                                                    .spacing(tog.spacing)
                                                    .style(move|theme: &Theme, status| {     
                                                        get_styling(theme, status, 
                                                                    style.clone()) 
                                                    })
                                                    .into();

    Some(ipg_tog.map(move |message| app::Message::Toggler(tog.id, message)))
}


pub fn toggle_callback(state: &mut IpgState, id: usize, message: TOGMessage) {

    let mut wci = WidgetCallbackIn{id, ..Default::default()};

    match message {
        TOGMessage::Toggled(on_toggle) => {
            wci.on_toggle = Some(on_toggle);
            let _ = set_or_get_widget_callback_data(state, wci);
            process_callback(id, "toggled".to_string(), on_toggle);
        }
    }
}

pub fn process_callback(id: usize, event_name: String, toggled: bool) 
{
    let ud = access_user_data1();
    let user_data_opt = ud.user_data.get(&id);

    let app_cbs = access_callbacks();

    let callback_present = 
        app_cbs.callbacks.get(&(id, event_name));
    
    let callback = match callback_present {
        Some(cb) => cb,
        None => return,
    };

    let cb = 
        Python::with_gil(|py| {
            callback.clone_ref(py)
        });

    drop(app_cbs);

    Python::with_gil(|py| {
            if user_data_opt.is_some() {
                let res = cb.call1(py, (
                                                            id,
                                                            toggled,  
                                                            user_data_opt.unwrap()
                                                            ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Toggler: 3 parameters (id, toggled, user_data) 
                                        are required or a python error in this function. {er}"),
                }
            } else {
                let res = cb.call1(py, (
                                                            id,
                                                            toggled,  
                                                            ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Toggler: 2 parameter (id, toggled) 
                                        is required or a python error in this function. {er}"),
                }
            } 
    });
    
    drop(ud);
         
}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
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
                            item: &PyObject,
                            value: &PyObject,
                            )
{
    let update = try_extract_toggler_update(item);
    let name = "Toggler".to_string();
    match update {
        IpgTogglerParam::Label => {
            tog.label = Some(try_extract_string(value, name));
        },
        IpgTogglerParam::Show => {
            tog.show = try_extract_boolean(value, name);
        },
        IpgTogglerParam::Width => {
            let val = try_extract_f64(value, name);
            tog.width = get_width(Some(val as f32), false);
        },
        IpgTogglerParam::WidthFill => {
            let val = try_extract_boolean(value, name);
            tog.width = get_width(None, val);
        },
        IpgTogglerParam::HorizontalAlignment => {
            let val = try_extract_ipg_horizontal_alignment(value);
            if val.is_none() {
                panic!("Unable to extract the toggler alignment")
            }
            tog.text_alignment = val.unwrap();
        },
        IpgTogglerParam::LineHeight => {
            let val = try_extract_f64(value, name) as f32; 
            tog.text_line_height = LineHeight::Relative(val);
        },
        IpgTogglerParam::Size => {
            let val = try_extract_f64(value, name) as f32;
            tog.size = val;
        },
        IpgTogglerParam::TextSize => {
            let val = try_extract_f64(value, name) as f32;
            tog.text_size = val;
        },
    }

}


pub fn try_extract_toggler_update(update_obj: &PyObject) -> IpgTogglerParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgTogglerParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Toggler update extraction failed"),
        }
    })
}

fn get_text_alignment(ta: &IpgHorizontalAlignment) -> alignment::Horizontal {
    match ta {
        IpgHorizontalAlignment::Left => alignment::Horizontal::Left,
        IpgHorizontalAlignment::Center => alignment::Horizontal::Center,
        IpgHorizontalAlignment::Right => alignment::Horizontal::Right,
    }
}



pub fn get_styling(theme: &Theme, status: Status, 
                    style_opt: Option<IpgTogglerStyle>,
                    ) -> toggler::Style {
    
    let mut tog_style = toggler::default(theme, status);

    if style_opt.is_none() {
        return tog_style
    }
    
    let style = style_opt.unwrap();

    // The background color for active or hovered can have two colors, one for untoggled and toggled.
    // The relationship of the bg and fg colors is:
    // Untoggled: bg=color.strong & fg=color.base
    // Toggled: bg=color & fg=contrasting color  
    if style.background_color.is_some() {
        tog_style.background = style.background_color.unwrap();
    }

    if style.foreground_color.is_some() {
        tog_style.foreground = style.foreground_color.unwrap();
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
            if is_toggled && style.background_color_toggled.is_some() {
                tog_style.background = style.background_color_toggled.unwrap();
            }
        }
        Status::Disabled => (),
    }

    match status {
        Status::Active { is_toggled } => {
            if is_toggled && style.foreground_color_toggled.is_some() {
                tog_style.foreground = style.foreground_color_toggled.unwrap();
            }
        }
        Status::Hovered { is_toggled } => {
            if is_toggled && style.foreground_color_toggled.is_some() {
                tog_style.foreground = 
                    Color {
                        a: 0.5,
                        ..style.foreground_color_toggled.unwrap()
                    };
                } 
        }
        Status::Disabled => (),
    }

    tog_style

}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgTogglerStyleParam {
    BackgroundIpgColor,
    BackgroundRbgaColor,
    BackgroundIpgColorToggled,
    BackgroundRgbaToggled,
    BackgroundIpgColorDisabled,
    BackgroundRbgaColorDisabled,
    BackgroundBorderIpgColor,
    BackgroundBorderRgbaColor,
    BackgroundBorderWidth,

    ForegroundIpgColor,
    ForegroundRbgaColor,
    ForegroundIpgColorToggled,
    ForegroundRgbaToggled,
    ForegroundIpgColorDisabled,
    ForegroundRbgaColorDisabled,
    ForegroundBorderIpgColor,
    ForegroundBorderRgbaColor,
    ForegroundBorderWidth,
}

pub fn toggler_style_update_item(style: &mut IpgTogglerStyle,
                            item: &PyObject,
                            value: &PyObject,) 
{
    let update = try_extract_toggler_style_update(item);
    let name = "ogglerStyle".to_string();
    match update {
        IpgTogglerStyleParam::BackgroundIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.background_color = get_color(None, Some(color), 1.0, false);
        },
        IpgTogglerStyleParam::BackgroundRbgaColor => {
            style.background_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgTogglerStyleParam::BackgroundIpgColorToggled => {
            let color = try_extract_ipg_color(value, name);
            style.background_color_toggled = get_color(None, Some(color), 1.0, false);
        },
        IpgTogglerStyleParam::BackgroundRgbaToggled => {
            style.background_color_toggled = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgTogglerStyleParam::BackgroundIpgColorDisabled => {
            let color = try_extract_ipg_color(value, name);
            style.background_color_toggled = get_color(None, Some(color), 1.0, false);
        },
        IpgTogglerStyleParam::BackgroundRbgaColorDisabled => {
            style.background_color_toggled = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgTogglerStyleParam::BackgroundBorderIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.background_border_color = get_color(None, Some(color), 1.0, false);
        },
        IpgTogglerStyleParam::BackgroundBorderRgbaColor => {
            style.background_border_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgTogglerStyleParam::BackgroundBorderWidth => {
            style.background_border_width = Some(try_extract_f64(value, name) as f32);
        },
        IpgTogglerStyleParam::ForegroundIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.foreground_color = get_color(None, Some(color), 1.0, false);
        },
        IpgTogglerStyleParam::ForegroundRbgaColor => {
            style.foreground_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgTogglerStyleParam::ForegroundIpgColorToggled => {
            let color = try_extract_ipg_color(value, name);
            style.foreground_color_toggled = get_color(None, Some(color), 1.0, false);
        },
        IpgTogglerStyleParam::ForegroundRgbaToggled => {
            style.foreground_color_toggled = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgTogglerStyleParam::ForegroundIpgColorDisabled => {
            let color = try_extract_ipg_color(value, name);
            style.foreground_color_toggled = get_color(None, Some(color), 1.0, false);
        },
        IpgTogglerStyleParam::ForegroundRbgaColorDisabled => {
            style.foreground_color_toggled = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgTogglerStyleParam::ForegroundBorderIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.foreground_border_color = get_color(None, Some(color), 1.0, false);
        },
        IpgTogglerStyleParam::ForegroundBorderRgbaColor => {
            style.foreground_border_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgTogglerStyleParam::ForegroundBorderWidth => {
            style.foreground_border_width = Some(try_extract_f64(value, name) as f32);
        },
    }
}

pub fn get_toggler_style(style: Option<&IpgWidgets>) -> Option<IpgTogglerStyle>{
    match style {
        Some(st) => {
            match st {
                IpgWidgets::IpgTogglerStyle(style) => {
                    Some(style.clone())
                }
                _ => None,
            }
        },
        None => None,
    }
}

pub fn try_extract_toggler_style_update(update_obj: &PyObject) -> IpgTogglerStyleParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgTogglerStyleParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Toggler style update extraction failed"),
        }
    })
}
