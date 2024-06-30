

use crate::access_callbacks;
use crate::access_state;
use crate::app;
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

use iced::widget::pick_list::{self, Status};
use iced::{Color, Font, Pixels, Theme, theme};
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
    pub style_color: Option<String>,
    pub style_border: Option<String>,
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
        style_color: Option<String>,
        style_border: Option<String>,
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
            style_color,
            style_border,
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
                                                pick.style_color.clone(), 
                                                pick.style_border.clone(),
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
    StyleColor,
    StyleBorder,
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
        IpgPickListParams::StyleColor => {
            let val = try_extract_string(value);
            pl.style_color = Some(val);
        },
        IpgPickListParams::StyleBorder => {
            let val = try_extract_string(value);
            pl.style_border = Some(val);
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


pub fn get_styling(theme: &Theme, status: Status, 
                    style_color: Option<String>, 
                    style_border: Option<String>,
                    ) -> pick_list::Style {
    
    let mut base_style = pick_list::default(theme, status);
    let mut hover_style = pick_list::default(theme, status);

    let palette = theme.extended_palette();

    let state = access_state();

    let color_palette_opt = if style_color.is_some() {
        state.styling_color.get(&style_color.unwrap())
    } else {
        None
    };
    
    if color_palette_opt.is_some() {
        let text = if palette.is_dark {
            Color::WHITE
        } else {
            Color::BLACK
        };

        let color_palette = color_palette_opt.unwrap().clone();
        
        let background = theme::palette::Background::new(color_palette.base.unwrap(), text);
        base_style.background = iced::Background::Color(background.weak.color);

        if color_palette.text.is_some() {
            base_style.text_color = color_palette.text.unwrap();
            hover_style.text_color = color_palette.text.unwrap();
        }
        
        if color_palette.border.is_some() {
            let border_color = theme::palette::Background::new(color_palette.border.unwrap(), text);
            base_style.border.color = border_color.base.color;
            hover_style.border.color = border_color.strong.color;
        }

        if color_palette.placeholder.is_some() {
            base_style.placeholder_color = color_palette.placeholder.unwrap(); 
        }

        if color_palette.handle.is_some() {
            base_style.handle_color = color_palette.handle.unwrap(); 
        }
    }

    let border_opt = if style_border.is_some() {
        state.styling_border.get(&style_border.unwrap())
    } else {
        None
    };

    if border_opt.is_some() {
        let bd = border_opt.unwrap();
        base_style.border.radius = bd.radius;
        base_style.border.width = bd.width;
        hover_style.border.radius = bd.radius;
        hover_style.border.width = bd.width;
    }
        

    match status {
        Status::Active => base_style,
        Status::Hovered | Status::Opened => hover_style,
    }

}