

use crate::access_callbacks;
use crate::access_state;
use crate::app;
use super::callbacks::{WidgetCallbackIn, 
                        WidgetCallbackOut, 
                        get_set_widget_callback_data};
use super::helpers::get_padding_f64;
use super::helpers::get_radius;
use super::helpers::get_width;
use super::helpers::try_extract_boolean;
use super::helpers::try_extract_f64;
use super::helpers::try_extract_string;
use super::helpers::try_extract_vec_f64;
use super::ipg_button::get_bootstrap_arrow_char;
use super::ipg_button::IpgButtonArrow;

use iced::widget::pick_list::{self, Status};
use iced::{Color, Font, Pixels, Theme};
use iced::{Padding, Length, Element};
use iced::widget::{PickList, Space};
use iced::widget::pick_list::{Handle, Icon};
use iced::widget::text::{LineHeight, Shaping};

use pyo3::pyclass;
use pyo3::{PyObject, Python};


#[derive(Debug, Clone)]
pub struct IpgPickList {
    pub id: usize,
    pub show: bool,
    pub user_data: Option<PyObject>,

    pub options: PyObject,
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
    pub style: Option<String>,
}

impl IpgPickList {
    pub fn new( 
        id: usize,
        show: bool,
        user_data: Option<PyObject>,

        options: PyObject,
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
        style: Option<String>,
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
            handle,
            arrow_size,
            dynamic_closed,
            dynamic_open,
            custom_static,
            style,
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


pub fn construct_picklist(pick: IpgPickList) -> Element<'static, app::Message> {

    if!pick.show {
        return Space::new(0.0, 0.0).into()
    }

    let placeholder = match pick.placeholder.clone() {
        Some(holder) => holder,
        None => "".to_string(),
    };
    let text_size: f32 = match pick.text_size {
        Some(size) => size,
        None => 16.0,
    };

    let handle = get_handle(pick.handle, 
                                    pick.arrow_size, 
                                    pick.dynamic_closed,
                                    pick.dynamic_open,
                                    pick.custom_static);

    let options =  convert_pyobject_vec_string(pick.options);

    let pl: Element<'_, PLMessage> = PickList::new(options.clone(), 
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
                                                pick.style.clone(),
                                            )  
                                            })
                                        .into();

    pl.map(move |message| app::Message::PickList(pick.id, message))

}
 

 pub fn pick_list_callback(id: usize, message: PLMessage) {
    let mut wci: WidgetCallbackIn = WidgetCallbackIn::default();
    wci.id = id;

    match message {
        PLMessage::OnSelect(value) => {
            wci.value_str = Some(value.clone());
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = "on_select".to_string();
            wco.value_str = Some(value);
            process_callback(wco);
        },
    }
 }


 fn process_callback(wco: WidgetCallbackOut) 
 {
    let app_cbs = access_callbacks();

    let callback_present = app_cbs.callbacks.get(&(wco.id, wco.event_name.clone()));

    let callback_opt = match callback_present {
        Some(cb) => cb,
        None => return,
    };

    let callback = match callback_opt {
        Some(cb) => cb,
        None => panic!("PickList Callback could not be found with id {}", wco.id),
    };

    let value = match wco.value_str {
        Some(vl) => vl,
        None => panic!("Picklist selected value could not be found."),
    };
                   
    Python::with_gil(|py| {
        if wco.user_data.is_some() {
        let user_data = match wco.user_data {
            Some(ud) => ud,
            None => panic!("PickList callback user_data not found."),
        };
            let res = callback.call1(py, (
                                                                wco.id.clone(), 
                                                                value, 
                                                                user_data
                                                                ));
            match res {
                Ok(_) => (),
                Err(er) => panic!("PickList: 3 parameters (id, value, user_data) are required or a python error in this function. {er}"),
            }
        } else {
            let res = callback.call1(py, (
                                                                wco.id.clone(), 
                                                                value, 
                                                                ));
            match res {
                Ok(_) => (),
                Err(er) => panic!("InputText: 2 parameters (id, value) are required or a python error in this function. {er}"),
            }
        } 
    });

    drop(app_cbs); 

 }


 fn convert_pyobject_vec_string(options: PyObject) -> Vec<String> {

    let items: Vec<String> = vec![];

    Python::with_gil(|py| {

        let res = options.extract::<Vec<bool>>(py);
        if !res.is_err() {
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
        if !res.is_err() {
            return match res {
                Ok(res) => res,
                Err(_) => panic!("Picklist could not extract List[String]"),
            } 
        }

        let res = options.extract::<Vec<i64>>(py);
        if !res.is_err() { 
            return match res {
                Ok(res) => res.iter().map(|v| v.to_string()).collect(),
                Err(_) => panic!("Picklist could not extract List[int]"),
            } 
        } 
        
        let res = options.extract::<Vec<f64>>(py);
        if !res.is_err() { 
            return match res {
                Ok(res) => res.iter().map(|v| v.to_string()).collect(),
                Err(_) => panic!("Picklist could not extract List[float]"),
            } 
        }

        items
    })

 }


 #[derive(Debug, Clone)]
#[pyclass]
pub enum IpgPickListParam {
    Options,
    Placeholder,
    Padding,
    Show,
    Style,
    TextSize,
    TextLineHeight,
    Width,
    // WidthFill,  see comment below
}


pub fn pick_list_item_update(pl: &mut IpgPickList,
                            item: PyObject,
                            value: PyObject,
                            )
{
    let update = try_extract_pick_list_update(item);

    match update {
        IpgPickListParam::Options => {
            pl.options = value;
            pl.selected = None;
        },
        IpgPickListParam::Placeholder => {
            pl.placeholder = Some(try_extract_string(value));
            pl.selected = None;
        },
        IpgPickListParam::Padding => {
            let val = try_extract_vec_f64(value);
            pl.padding =  get_padding_f64(val);
        },
        IpgPickListParam::Show => {
            pl.show = try_extract_boolean(value);
        },
        IpgPickListParam::Style => {
            let val = try_extract_string(value);
            pl.style = Some(val);
        },
        IpgPickListParam::TextSize => {
            let size = try_extract_f64(value);
            pl.text_size = Some(size as f32);
        },
        IpgPickListParam::TextLineHeight => {
            let val = try_extract_f64(value) as f32;
            pl.text_line_height = LineHeight::Relative(val);
        },
        IpgPickListParam::Width => {
            let val = try_extract_f64(value);
            pl.width = get_width(Some(val as f32), false);
        },
         // TODO: Doesn't work, shrink the box but still displaces the text.
        // IpgPickListParam::WidthFill => {
        //     let val = try_extract_boolean(value);
        //     pl.width = get_width(None, val);
        // },
    }

}

pub fn try_extract_pick_list_update(update_obj: PyObject) -> IpgPickListParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgPickListParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("PickList update extraction failed"),
        }
    })
}

#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgPickListHandle {
    Default,
    Arrow,
    Dynamic,
    None,
    Static,
}

fn get_handle(ipg_handle: IpgPickListHandle, 
                arrow_size: Option<f32>,
                closed: Option<IpgButtonArrow>,
                opened: Option<IpgButtonArrow>,
                custom: Option<IpgButtonArrow>,
            ) -> Handle<Font> 
{
    match ipg_handle {
        IpgPickListHandle::Default => Handle::default(),
        IpgPickListHandle::Arrow => {
            if arrow_size.is_some() {
                Handle::Arrow { size: Some(Pixels(arrow_size.unwrap())) }
            } else {
                Handle::Arrow { size: None }
            }
        },
        IpgPickListHandle::Dynamic => {
            let arrow_closed = match closed {
                Some(cls) => get_bootstrap_arrow_char(cls),
                None => get_bootstrap_arrow_char(IpgButtonArrow::ArrowBarRight),
            };

            let arrow_opened = match opened {
                Some(op) => get_bootstrap_arrow_char(op),
                None => get_bootstrap_arrow_char(IpgButtonArrow::ArrowBarRight),
            };

            let size = if arrow_size.is_some() {
                Some(Pixels(arrow_size.unwrap())) 
            } else {
                None
            };

            Handle::Dynamic { closed: Icon { code_point: arrow_closed, 
                                            font: iced::Font::with_name("bootstrap-icons"), 
                                            size: size, 
                                            line_height: Default::default(), 
                                            shaping: Default::default()}, 
                               open: Icon {code_point: arrow_opened,
                                            font: iced::Font::with_name("bootstrap-icons"), 
                                            size: size, 
                                            line_height: Default::default(), 
                                            shaping: Default::default()} 
                            }
        },
        IpgPickListHandle::None => Handle::None,
        IpgPickListHandle::Static => {
                let custom_type = match custom {
                    Some(cust) => get_bootstrap_arrow_char(cust),
                    None => get_bootstrap_arrow_char(IpgButtonArrow::ArrowBarRight),
                };
                let size = if arrow_size.is_some() {
                    Some(Pixels(arrow_size.unwrap())) 
                } else {
                    None
                };

                Handle::Static(Icon { code_point: custom_type, 
                    font: iced::Font::with_name("bootstrap-icons"), 
                    size: size, 
                    line_height: Default::default(), 
                    shaping: Default::default()
                }
            )
        },
    }
}


pub fn get_styling(theme: &Theme, status: Status, 
                    style_str: Option<String>, 
                    ) -> pick_list::Style {
    
    let mut active_style = pick_list::default(theme, Status::Active);
   
    let state = access_state();

    if style_str.is_none() {
        return match status {
            Status::Active => active_style,
            Status::Hovered | Status::Opened => pick_list::default(theme, Status::Hovered),
        }
    }

    let style_opt = state.pick_list_style.get(&style_str.clone().unwrap());
    
    let style = match style_opt {
        Some(st) => st,
        None => panic!("PiclList: The style_id {} for add_pick_list_style could not be found", style_str.unwrap())
    };


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

    let mut hover_opened_style = active_style.clone();
    
    if style.border_color_hovered.is_some() {
        hover_opened_style.border.color = style.border_color_hovered.unwrap();
    }
    
    match status {
        Status::Active => active_style,
        Status::Hovered | Status::Opened => hover_opened_style,
    }

}
