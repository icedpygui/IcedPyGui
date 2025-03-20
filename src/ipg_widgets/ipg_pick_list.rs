//! ipg_pick_list
use crate::access_callbacks;
use crate::access_user_data1;
use crate::app;
use crate::graphics::colors::get_color;
use crate::IpgState;
use super::callbacks::set_or_get_widget_callback_data;
use super::callbacks::WidgetCallbackIn;
use super::helpers::try_extract_ipg_color;
use super::helpers::try_extract_rgba_color;
use super::helpers::try_extract_vec_f32;
use super::helpers::try_extract_vec_str;
use super::helpers::{get_padding_f64, get_radius, get_width};
use super::helpers::{try_extract_boolean, try_extract_f64,
    try_extract_string, try_extract_vec_f64};
use super::ipg_button::{IpgButtonArrow, get_bootstrap_arrow_char};
use super::ipg_enums::IpgWidgets;

use iced::widget::pick_list::{self, Status};
use iced::{Color, Font, Pixels, Theme};
use iced::{Padding, Length, Element};
use iced::widget::PickList;
use iced::widget::pick_list::{Handle, Icon};
use iced::widget::text::{LineHeight, Shaping};

use pyo3::pyclass;
use pyo3::{PyObject, Python};


#[derive(Debug, Clone)]
pub struct IpgPickList {
    pub id: usize,
    pub parent_id: String,
    pub show: bool,
    pub options: Vec<String>,
    pub placeholder: Option<String>,
    pub selected: Option<String>,
    pub width: Length,
    pub padding: Padding,
    pub text_size: Option<f32>,
    pub text_line_height: LineHeight,
    pub text_shaping: Shaping,
    pub handle: IpgPickListHandle,
    pub arrow_size: Option<f32>,
    pub dynamic_closed: Option<IpgButtonArrow>,
    pub dynamic_open: Option<IpgButtonArrow>,
    pub custom_static: Option<IpgButtonArrow>,
    pub style_id: Option<usize>,
}

impl IpgPickList {
    pub fn new( 
        id: usize,
        parent_id: String,
        show: bool,
        options: Vec<String>,
        placeholder: Option<String>,
        selected: Option<String>,
        width: Length,
        padding: Padding,
        text_size: Option<f32>,
        text_line_height: LineHeight,
        text_shaping: Shaping,
        handle: IpgPickListHandle,
        arrow_size: Option<f32>,
        dynamic_closed: Option<IpgButtonArrow>,
        dynamic_open: Option<IpgButtonArrow>,
        custom_static: Option<IpgButtonArrow>,
        style: Option<usize>,
        ) -> Self {
        Self {
            id,
            parent_id,
            show,
            options,
            placeholder,
            selected,
            width,
            padding,
            text_size,
            text_line_height,
            text_shaping,
            handle,
            arrow_size,
            dynamic_closed,
            dynamic_open,
            custom_static,
            style_id: style,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct IpgPickListStyle {
    pub id: usize,
    pub background_color: Option<Color>, // background
    pub text_color: Option<Color>,
    pub handle_color: Option<Color>,
    pub placeholder_color: Option<Color>,
    pub border_color: Option<Color>,
    pub border_color_hovered: Option<Color>,
    pub border_radius: Vec<f32>,
    pub border_width: f32,
}

impl IpgPickListStyle {
    pub fn new(
        id: usize,
        background_color: Option<Color>,
        text_color: Option<Color>,
        handle_color: Option<Color>,
        placeholder_color: Option<Color>,                 
        border_color: Option<Color>,
        border_color_hovered: Option<Color>,
        border_radius: Vec<f32>,
        border_width: f32,
    ) -> Self {
        Self {
            id,
            background_color,
            text_color,
            handle_color,
            placeholder_color,
            border_color,
            border_color_hovered,
            border_radius,
            border_width,
        }
    }
}

#[derive(Debug, Clone)]
pub enum PLMessage {
    OnSelect(String),
}


pub fn construct_picklist<'a>(pick: &'a IpgPickList, 
                                style_opt: Option<&IpgWidgets>) 
                                -> Option<Element<'a, app::Message>> {
    
    if!pick.show {
        return None
    }
    let style = get_pick_list_style(style_opt);
    let placeholder = pick.placeholder.clone().unwrap_or("".to_string());

    let text_size: f32 = pick.text_size.unwrap_or(16.0);

    let handle = get_handle(&pick.handle, 
                                    pick.arrow_size, 
                                    &pick.dynamic_closed,
                                    &pick.dynamic_open,
                                    &pick.custom_static);

   

    let pl: Element<'_, PLMessage> = 
        PickList::new(pick.options.clone(), 
            pick.selected.clone(), 
            PLMessage::OnSelect,
        )
        .placeholder(placeholder)
        .width(pick.width)
        .padding(pick.padding)
        .text_size(text_size)
        .text_line_height(pick.text_line_height)
        .text_shaping(pick.text_shaping)
        .handle(handle)
        .style(move|theme: &Theme, status| {   
            get_styling(theme, status, 
                style.clone(),
            )  
            })
        .into();

    Some(pl.map(move |message| app::Message::PickList(pick.id, message)))

}
 

 pub fn pick_list_callback(state: &mut IpgState, id: usize, message: PLMessage) {

    let mut wci: WidgetCallbackIn = WidgetCallbackIn{id, ..Default::default()};

    match message {
        PLMessage::OnSelect(selected) => {
            wci.value_str = Some(selected.clone());
            let _ = set_or_get_widget_callback_data(state, wci);
            
            process_callback(id, "on_select".to_string(), Some(selected));
        },
    }
 }


 fn process_callback(id: usize, event_name: String, selected: Option<String>) 
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
        if user_data_opt.is_some() && selected.is_some() {
            let res = cb.call1(py, (
                                                        id, 
                                                        selected.unwrap(), 
                                                        user_data_opt.unwrap()
                                                        ));
            match res {
                Ok(_) => (),
                Err(er) => panic!("PickList: 3 parameters (id, value, user_data) 
                                    are required or a python error in this function. {er}"),
            }
        } else {
            let res = cb.call1(py, (
                                                        id, 
                                                        selected.unwrap(), 
                                                        ));
            match res {
                Ok(_) => (),
                Err(er) => panic!("InputText: 2 parameters (id, value) 
                                    are required or a python error in this function. {er}"),
            }
        } 
    });

    drop(ud); 

 }


 pub fn convert_pyobject_vec_string(options: PyObject) -> Vec<String> {

    let items: Vec<String> = vec![];

    Python::with_gil(|py| {

        let res = options.extract::<Vec<bool>>(py);
        if res.is_ok() {
            return match res {
                Ok(res) => {
                    res.iter().map(|v| {
                        if *v {
                            "True".to_string()
                        } else {
                           "False".to_string()
                        }
                    }).collect()
                },
                Err(_) => panic!("Picklist could not extract List[bool]"),
            }
        }

        let res = options.extract::<Vec<String>>(py);
        if res.is_ok() {
            return match res {
                Ok(res) => res,
                Err(_) => panic!("Picklist could not extract List[String]"),
            } 
        }

        let res = options.extract::<Vec<i64>>(py);
        if res.is_ok() { 
            return match res {
                Ok(res) => res.iter().map(|v| v.to_string()).collect(),
                Err(_) => panic!("Picklist could not extract List[int]"),
            } 
        } 
        
        let res = options.extract::<Vec<f64>>(py);
        if res.is_ok() { 
            return match res {
                Ok(res) => res.iter().map(|v| v.to_string()).collect(),
                Err(_) => panic!("Picklist could not extract List[float]"),
            } 
        }

        items
    })

 }


 #[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgPickListParam {
    Options,
    Placeholder,
    Padding,
    Selected,
    Show,
    Style,
    TextSize,
    TextLineHeight,
    Width,
    // WidthFill,  see comment below
}


pub fn pick_list_item_update(pl: &mut IpgPickList,
                            item: &PyObject,
                            value: &PyObject,
                            )
{
    let update = try_extract_pick_list_update(item);
    let name = "PickList".to_string();
    match update {
        IpgPickListParam::Options => {
            pl.options = try_extract_vec_str(value, name);
            pl.selected = None;
        },
        IpgPickListParam::Placeholder => {
            pl.placeholder = Some(try_extract_string(value, name));
            pl.selected = None;
        },
        IpgPickListParam::Padding => {
            pl.padding =  get_padding_f64(try_extract_vec_f64(value, name));
        },
        IpgPickListParam::Selected => {
            pl.selected = Some(try_extract_string(value, name));
        }
        IpgPickListParam::Show => {
            pl.show = try_extract_boolean(value, name);
        },
        IpgPickListParam::Style => {
            pl.style_id = Some(try_extract_f64(value, name) as usize);
        },
        IpgPickListParam::TextSize => {
            pl.text_size = Some(try_extract_f64(value, name) as f32);
        },
        IpgPickListParam::TextLineHeight => {
            pl.text_line_height = LineHeight::Relative(try_extract_f64(value, name) as f32);
        },
        IpgPickListParam::Width => {
            let val = try_extract_f64(value, name);
            pl.width = get_width(Some(val as f32), false);
        },
         // TODO: Doesn't work, shrink the box but still displaces the text.
        // IpgPickListParam::WidthFill => {
        //     let val = try_extract_boolean(value);
        //     pl.width = get_width(None, val);
        // },
    }

}

pub fn try_extract_pick_list_update(update_obj: &PyObject) -> IpgPickListParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgPickListParam>(py);
        match res {
            Ok(update) => update,
            Err(error) => panic!("PickList update extraction failed {}", error),
        }
    })
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgPickListHandle {
    Default,
    Arrow,
    Dynamic,
    None,
    Static,
}

fn get_handle(ipg_handle: &IpgPickListHandle, 
                arrow_size: Option<f32>,
                closed: &Option<IpgButtonArrow>,
                opened: &Option<IpgButtonArrow>,
                custom: &Option<IpgButtonArrow>,
            ) -> Handle<Font> 
{
    match ipg_handle {
        IpgPickListHandle::Default => Handle::default(),
        IpgPickListHandle::Arrow => {
            match arrow_size {
                Some(ars) => Handle::Arrow { size: Some(Pixels(ars)) },
                None => Handle::Arrow { size: None },
            }
        },
        IpgPickListHandle::Dynamic => {
            let arrow_closed = match closed {
                Some(cls) => get_bootstrap_arrow_char(cls),
                None => get_bootstrap_arrow_char(&IpgButtonArrow::ArrowBarRight),
            };

            let arrow_opened = match opened {
                Some(op) => get_bootstrap_arrow_char(op),
                None => get_bootstrap_arrow_char(&IpgButtonArrow::ArrowBarRight),
            };

            let size = arrow_size.map(Pixels);

            Handle::Dynamic { closed: Icon { code_point: arrow_closed, 
                                            font: iced::Font::with_name("bootstrap-icons"), 
                                            size, 
                                            line_height: Default::default(), 
                                            shaping: Default::default()}, 
                               open: Icon {code_point: arrow_opened,
                                            font: iced::Font::with_name("bootstrap-icons"), 
                                            size, 
                                            line_height: Default::default(), 
                                            shaping: Default::default()} 
                            }
        },
        IpgPickListHandle::None => Handle::None,
        IpgPickListHandle::Static => {
                let custom_type = match custom {
                    Some(cust) => get_bootstrap_arrow_char(cust),
                    None => get_bootstrap_arrow_char(&IpgButtonArrow::ArrowBarRight),
                };

                let size = arrow_size.map(Pixels);

                Handle::Static(Icon { code_point: custom_type, 
                    font: iced::Font::with_name("bootstrap-icons"), 
                    size, 
                    line_height: Default::default(), 
                    shaping: Default::default()
                }
            )
        },
    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgPickListStyleParam {
    BackgroundIpgColor,
    BackgroundRbgaColor,
    BorderIpgColor,
    BorderRgbaColor,
    BorderRadius,
    BorderWidth,
    HandleIpgColor,
    HandleRgbaColor,
    PlaceholderIpgColor,
    PlaceholderRgbaColor,
    TextIpgColor,
    TextRgbaColor,
}

pub fn pick_list_style_update_item(style: &mut IpgPickListStyle,
                            item: &PyObject,
                            value: &PyObject,) 
{
    let update = try_extract_pick_list_style_update(item);
    let name = "PickListStyle".to_string();
    match update {
        IpgPickListStyleParam::BackgroundIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.background_color = get_color(None, Some(color), 1.0, false);
        },
        IpgPickListStyleParam::BackgroundRbgaColor => {
            style.background_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgPickListStyleParam::BorderIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.border_color = get_color(None, Some(color), 1.0, false);
        },
        IpgPickListStyleParam::BorderRgbaColor => {
            style.border_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgPickListStyleParam::BorderRadius => {
            style.border_radius = try_extract_vec_f32(value, name);
        },
        IpgPickListStyleParam::BorderWidth => {
            style.border_width = try_extract_f64(value, name) as f32;
        },
        IpgPickListStyleParam::HandleIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.handle_color = get_color(None, Some(color), 1.0, false);
        },
        IpgPickListStyleParam::HandleRgbaColor => {
            style.handle_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgPickListStyleParam::PlaceholderIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.placeholder_color = get_color(None, Some(color), 1.0, false);
        },
        IpgPickListStyleParam::PlaceholderRgbaColor => {
            style.placeholder_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgPickListStyleParam::TextIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.text_color = get_color(None, Some(color), 1.0, false);
        },
        IpgPickListStyleParam::TextRgbaColor => {
            style.text_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        
    }
}

pub fn get_styling(theme: &Theme, status: Status, 
                    style_opt: Option<IpgPickListStyle>, 
                    ) -> pick_list::Style {
    
    let mut active_style = pick_list::default(theme, Status::Active);

    if style_opt.is_none() {
        return match status {
                Status::Active => active_style,
                Status::Hovered | Status::Opened => pick_list::default(theme, Status::Hovered),
            }
    }

    let style = style_opt.unwrap();

    if style.background_color.is_some() {
        active_style.background = style.background_color.unwrap().into();
    }
    
    if style.handle_color.is_some() {
        active_style.handle_color = style.handle_color.unwrap();
    }

    if style.placeholder_color.is_some() {
        active_style.placeholder_color = style.placeholder_color.unwrap();
    }

    if style.text_color.is_some() {
        active_style.text_color = style.text_color.unwrap();
    }

    active_style.border.radius = get_radius(style.border_radius.clone(), "PickList".to_string());
    active_style.border.width = style.border_width;

    
    if style.border_color.is_some() && status == Status::Active {
        active_style.border.color = style.border_color.unwrap();
    }

    let mut hover_opened_style = active_style;
    
    if style.border_color_hovered.is_some() {
        hover_opened_style.border.color = style.border_color_hovered.unwrap();
    }
    
    match status {
        Status::Active => active_style,
        Status::Hovered | Status::Opened => hover_opened_style,
    }

}

pub fn try_extract_pick_list_style_update(update_obj: &PyObject) -> IpgPickListStyleParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgPickListStyleParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Pick List style update extraction failed"),
        }
    })
}

fn get_pick_list_style(style: Option<&IpgWidgets>) -> Option<IpgPickListStyle>{
    match style {
        Some(IpgWidgets::IpgPickListStyle(style)) => {
            Some(style.clone())
        }
            _ => None,
        }
}