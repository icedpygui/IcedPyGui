

use crate::access_callbacks;
use crate::access_state;
use crate::app;
use crate::graphics::colors::match_ipg_color;
use crate::graphics::colors::IpgColor;
use super::callbacks::{WidgetCallbackIn, 
                        WidgetCallbackOut, 
                        get_set_widget_callback_data};
use super::helpers::get_padding;
use super::helpers::get_width;
use super::helpers::try_extract_boolean;
use super::helpers::try_extract_f64;
use super::helpers::try_extract_string;
use super::helpers::try_extract_vec_f64;
use super::ipg_button::get_bootstrap_arrow_char;
use super::ipg_button::IpgButtonArrows;
use crate::style::styling::lighten;


use iced::border::Radius;
use iced::widget::pick_list;
use iced::widget::pick_list::Status;
use iced::widget::pick_list::Style;
use iced::Background;
use iced::Border;
use iced::Color;
use iced::Font;
use iced::Pixels;
use iced::Theme;
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
    pub dynamic_closed: Option<IpgButtonArrows>,
    pub dynamic_open: Option<IpgButtonArrows>,
    pub custom_static: Option<IpgButtonArrows>,
    pub style_background: Option<String>,
    pub style_border: Option<String>,
    pub style_handle_color: Option<String>,
    pub style_text_color: Option<String>,
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
        dynamic_closed: Option<IpgButtonArrows>,
        dynamic_open: Option<IpgButtonArrows>,
        custom_static: Option<IpgButtonArrows>,
        style_background: Option<String>,
        style_border: Option<String>,
        style_handle_color: Option<String>,
        style_text_color: Option<String>,
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
            style_background,
            style_border,
            style_handle_color,
            style_text_color,
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
                                                pick.style_background.clone(), 
                                                pick.style_border.clone(),
                                                pick.style_handle_color.clone(),
                                                pick.style_text_color.clone(),
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
pub enum IpgPickListParams {
    Options,
    Placeholder,
    Padding,
    Show,
    StyleBackground,
    StyleBorder,
    StyleHandleColor,
    StyleTextColor,
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
        IpgPickListParams::Options => {
            pl.options = value;
            pl.selected = None;
        },
        IpgPickListParams::Placeholder => {
            pl.placeholder = Some(try_extract_string(value));
            pl.selected = None;
        },
        IpgPickListParams::Padding => {
            let val = try_extract_vec_f64(value);
            pl.padding =  get_padding(val);
        },
        IpgPickListParams::Show => {
            pl.show = try_extract_boolean(value);
        },
        IpgPickListParams::StyleBackground => {
            let val = try_extract_string(value);
            pl.style_background = Some(val);
        },
        IpgPickListParams::StyleBorder => {
            let val = try_extract_string(value);
            pl.style_border = Some(val);
        },
        IpgPickListParams::StyleHandleColor => {
            let val = try_extract_string(value);
            pl.style_handle_color = Some(val);
        },
        IpgPickListParams::StyleTextColor => {
            let val = try_extract_string(value);
            pl.style_text_color = Some(val);
        },
        IpgPickListParams::TextSize => {
            let size = try_extract_f64(value);
            pl.text_size = Some(size as f32);
        },
        IpgPickListParams::TextLineHeight => {
            let val = try_extract_f64(value) as f32;
            pl.text_line_height = LineHeight::Relative(val);
        },
        IpgPickListParams::Width => {
            let val = try_extract_f64(value);
            pl.width = get_width(Some(val as f32), false);
        },
         // TODO: Doesn't work, shrink the box but still displaces the text.
        // IpgPickListUpdate::WidthFill => {
        //     let val = try_extract_boolean(value);
        //     pl.width = get_width(None, val);
        // },
    }

}

pub fn try_extract_pick_list_update(update_obj: PyObject) -> IpgPickListParams {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgPickListParams>(py);
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
                closed: Option<IpgButtonArrows>,
                opened: Option<IpgButtonArrows>,
                custom: Option<IpgButtonArrows>,
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
                None => get_bootstrap_arrow_char(IpgButtonArrows::ArrowBarRight),
            };

            let arrow_opened = match opened {
                Some(op) => get_bootstrap_arrow_char(op),
                None => get_bootstrap_arrow_char(IpgButtonArrows::ArrowBarRight),
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
                    None => get_bootstrap_arrow_char(IpgButtonArrows::ArrowBarRight),
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


pub fn get_styling(_theme: &Theme, status: Status, 
                    style_background: Option<String>, 
                    style_border: Option<String>,
                    style_handle_color: Option<String>,
                    style_text_color: Option<String>) 
                    -> pick_list::Style {
    
    let state = access_state();

    let background_opt = if style_background.is_some() {
        state.styling_background.get(&style_background.unwrap())
    } else {
        None
    };
    
    let bg_color = match_ipg_color(IpgColor::PRIMARY);
    let (background, hover_factor) = match background_opt {
        Some(bg) => {
            (Background::Color(bg.color), bg.accent)
        },
        None => (Background::Color(bg_color), 0.05),
    };


    let mut border_color = match_ipg_color(IpgColor::PRIMARY);
    let mut radius = Radius::from([5.0; 4]);
    let mut border_width = 1.0;

    let border_opt = if style_border.is_some() {
        state.styling_border.get(&style_border.unwrap())
    } else {
        None
    };

    match border_opt {
        Some(bd) => {
            border_color = bd.color;
            radius = bd.radius;
            border_width = bd.width;
        },
        None => (),
    }

    let border = Border{ color: border_color, width: border_width, radius };

    let text_color_opt = if style_text_color.is_some() {
        state.styling_text_color.get(&style_text_color.unwrap())
    } else {
        None
    };
    
    let text_color = match text_color_opt {
        Some(tc) => {
            tc.color
        },
        None => match_ipg_color(IpgColor::ANTIQUE_WHITE),
    };

    let placeholder_color: Color = lighten(match_ipg_color(IpgColor::ANTIQUE_WHITE), hover_factor);

    let handle_color_opt = if style_handle_color.is_some() {
        state.styling_handle_color.get(&style_handle_color.unwrap())
    } else {
        None
    };

    let handle_color: Color = match handle_color_opt {
        Some(hc) => hc.color,
        None => text_color,
    };

    let active = pick_list::Style {
            background: background,
            border,
            text_color,
            placeholder_color,
            handle_color: handle_color,
            };

    match status {
        Status::Active => active,
        Status::Hovered | Status::Opened => Style {
            border: Border {
                color: lighten(bg_color, hover_factor),
                ..active.border
            },
            ..active
        },
    }

}