//! ipg_divider

use iced::{Background, Color, Element, Length, Theme};
use pyo3::{pyclass, PyObject, Python};

use crate::{access_callbacks, access_user_data1, app, graphics::colors::get_color, IpgState};

use super::{callbacks::{set_or_get_widget_callback_data, WidgetCallbackIn}, 
divider::{self, divider_horizontal, divider_vertical, Direction, Status, Style}, helpers::{ 
    get_radius, try_extract_boolean, try_extract_f32, 
    try_extract_f64, try_extract_ipg_color, try_extract_rgba_color, try_extract_vec_f32}, 
    ipg_enums::IpgWidgets};


#[derive(Debug, Clone, PartialEq)]
pub struct IpgDividerHorizontal {
    pub id: usize,
    pub parent_id: String,
    pub show: bool,
    pub widths: Vec<f32>,
    pub handle_width: f32,
    pub handle_height: f32,
    pub handle_offsets: Option<Vec<f32>>,
    pub include_last_handle: bool,
    pub width: Length,
    pub height: Length,
    pub index_in_use: usize,
    pub value_in_use: f32,
    pub style_id: Option<usize>,
}

impl IpgDividerHorizontal {
    pub fn new( 
        id: usize,
        parent_id: String,
        show: bool,
        widths: Vec<f32>,
        handle_width: f32,
        handle_height: f32,
        handle_offsets: Option<Vec<f32>>,
        include_last_handle: bool,
        width: Length,
        height: Length,
        style_id: Option<usize>,
    ) -> Self {
        Self {
            id,
            parent_id,
            show,
            widths,
            handle_width,
            handle_height,
            handle_offsets,
            include_last_handle,
            width,
            height,
            index_in_use: 0,
            value_in_use: 0.0,
            style_id,
        }
    } 
}

#[derive(Debug, Clone, PartialEq)]
pub struct IpgDividerVertical {
    pub id: usize,
    pub parent_id: String,
    pub show: bool,
    pub heights: Vec<f32>,
    pub handle_width: f32,
    pub handle_height: f32,
    pub handle_offsets: Option<Vec<f32>>,
    pub include_last_handle: bool,
    pub width: Length,
    pub height: Length,
    pub index_in_use: usize,
    pub value_in_use: f32,
    pub style_id: Option<usize>,
}

impl IpgDividerVertical {
    pub fn new( 
        id: usize,
        parent_id: String,
        show: bool,
        heights: Vec<f32>,
        handle_width: f32,
        handle_height: f32,
        handle_offsets: Option<Vec<f32>>,
        include_last_handle: bool,
        width: Length,
        height: Length,
        style_id: Option<usize>,
    ) -> Self {
        Self {
            id,
            parent_id,
            show,
            heights,
            handle_width,
            handle_height,
            handle_offsets,
            include_last_handle,
            width,
            height,
            index_in_use: 0,
            value_in_use: 0.0,
            style_id,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IpgDividerStyle {
    pub id: usize,
    pub background: Option<Color>,
    pub background_hovered: Option<Color>,
    pub background_transparent: bool,
    pub border_color: Option<Color>,
    pub border_width: f32,
    pub border_radius: f32,
}

impl IpgDividerStyle {
    pub fn new( 
        id: usize,
        background: Option<Color>,
        background_hovered: Option<Color>,
        background_transparent: bool,
        border_color: Option<Color>,
        border_width: f32,
        border_radius: f32,
    ) -> Self {
        Self {
            id,
            background,
            background_hovered,
            background_transparent,
            border_color,
            border_width,
            border_radius,
        }
    }
}


#[derive(Debug, Clone)]
pub enum DivMessage {
    OnChange((usize, usize, f32)),
    OnRelease,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgDividerDirection {
    /// Horizontal resizing
    Horizontal,
    /// Vertical resizing
    Vertical,
}

pub fn construct_divider_horizontal<'a>(
        divider: &'a IpgDividerHorizontal, 
        style_opt: Option<&IpgWidgets>) 
        -> Option<Element<'a, app::Message>> {

    if !divider.show {
        return None
    }

    let style = get_divider_style(style_opt);

    let offsets = match divider.handle_offsets.clone() {
        Some(offsets) => offsets,
        None => {
            let mut offsets = vec![-divider.handle_width/2.0; divider.widths.len()-1];
            offsets.extend([-divider.handle_width]);
            offsets
        }
    };

    let div: Element<DivMessage, Theme> = 
        divider_horizontal(
            divider.id,
            divider.widths.clone(),
            divider.handle_width,
            divider.handle_height, 
            DivMessage::OnChange
            )
            .on_release(DivMessage::OnRelease)
            .direction(Direction::Horizontal)
            .width(divider.width)
            .height(divider.height)
            .handle_offsets(offsets)
            .include_last_handle(divider.include_last_handle)
            .style(move|theme, status|
                get_styling(theme, status,
                style.clone())
                )
            .into();

    Some(div.map(move |message| app::Message::Divider(divider.id, message)))
}

pub fn construct_divider_vertical<'a>(
        divider: &'a IpgDividerVertical, 
        style_opt: Option<&IpgWidgets>) 
        -> Option<Element<'a, app::Message>> {

    if !divider.show {
        return None
    }

    let style = get_divider_style(style_opt);

    let offsets = match divider.handle_offsets.clone() {
        Some(offsets) => offsets,
        None => {
            let mut offsets = vec![-divider.handle_height/2.0; divider.heights.len()-1];
            offsets.extend([-divider.handle_height]);
            offsets
        }
    };

    let div: Element<DivMessage, Theme> = 
        divider_vertical(
            divider.id,
            divider.heights.clone(),
            divider.handle_width,
            divider.handle_height, 
            DivMessage::OnChange
            )
            .on_release(DivMessage::OnRelease)
            .direction(Direction::Vertical)
            .width(divider.width)
            .height(divider.height)
            .handle_offsets(offsets)
            .include_last_handle(divider.include_last_handle)
            .style(move|theme, status|
                get_styling(theme, status,
                style.clone())
                )
            .into();

    Some(div.map(move |message| app::Message::Divider(divider.id, message)))
}

pub fn divider_callback(state: &mut IpgState, id: usize, message: DivMessage) {

    let mut wci: WidgetCallbackIn = WidgetCallbackIn{id, ..Default::default()};
           
    match message {
        DivMessage::OnChange((id, index, value)) => {
            wci.value_f32 = Some(value);
            wci.value_usize = Some(index);
            wci.value_str = Some("on_change".to_string());
            let _ = set_or_get_widget_callback_data(state, wci);
            process_callback(
                id, 
                "on_change".to_string(), 
                index, 
                value);
        },
        DivMessage::OnRelease => {
            // to be consistent, returning values for both
            wci.value_str = Some("on_release".to_string());
            let wco = set_or_get_widget_callback_data(state, wci);
            process_callback(
                id, 
                "on_release".to_string(), 
                wco.value_usize.unwrap(), 
                wco.value_f32.unwrap());
        },
    }
}

pub fn process_callback(id: usize, event_name: String, index: usize, value: f32) 
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
                                                        index, 
                                                        value, 
                                                        user_data_opt,
                                                        ));
            match res {
                Ok(_) => (),
                Err(er) => panic!("Divider: 4 parameters (id, value, user_data) 
                                    are required or a python error in this function. {er}"),
            }
        } else {
            let res = cb.call1(py, (
                                                        id,
                                                        index, 
                                                        value, 
                                                        ));
            match res {
                Ok(_) => (),
                Err(er) => panic!("Divider: 3 parameters (id, value) 
                                    are required or a python error in this function. {er}"),
            }
        }
    });

    drop(ud); 

}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgDividerParam {
    HandleWidth,
    HandleHeight,
    Widths,
    Heights,
    StyleId,
    Show,
}

pub fn divider_horizontal_item_update(
        divider: &mut IpgDividerHorizontal, 
        item: &PyObject, 
        value: &PyObject) {

    let update = try_extract_divider_update(item);
    let name = "Divider".to_string();
    match update {
        IpgDividerParam::HandleWidth => {
            divider.handle_width = try_extract_f32(value, name);
        },
        IpgDividerParam::HandleHeight => {
            divider.handle_height = try_extract_f32(value, name);
        },
        IpgDividerParam::Widths => {
            divider.widths = try_extract_vec_f32(value, name);
        },
        IpgDividerParam::Heights => {
            panic!("Horizontal Divider must use the Widths not Heights");
        },
        IpgDividerParam::StyleId => {
            divider.style_id = Some(try_extract_f64(value, name) as usize);
        },
        IpgDividerParam::Show => {
            divider.show = try_extract_boolean(value, name);
        },
        
    }
}

pub fn divider_vertical_item_update(
        divider: &mut IpgDividerVertical, 
        item: &PyObject, 
        value: &PyObject) {

    let update = try_extract_divider_update(item);
    let name = "Divider".to_string();
    match update {
        IpgDividerParam::HandleWidth => {
            divider.handle_width = try_extract_f32(value, name);
        },
        IpgDividerParam::HandleHeight => {
            divider.handle_height = try_extract_f32(value, name);
            
        },
        IpgDividerParam::Widths => {
            panic!("Vertical Divider must use the Heights not Widths");
        },
        IpgDividerParam::Heights => {
            divider.heights = try_extract_vec_f32(value, name);
        },
        IpgDividerParam::StyleId => {
            divider.style_id = Some(try_extract_f64(value, name) as usize);
        },
        IpgDividerParam::Show => {
            divider.show = try_extract_boolean(value, name);
        },
        
    }
}


fn try_extract_divider_update(update_obj: &PyObject) -> IpgDividerParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgDividerParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Divider update extraction failed"),
        }
    })
}

fn get_styling(theme: &Theme, 
                status: Status,
                style_opt: Option<IpgDividerStyle>) 
                -> Style {

    if style_opt.is_none() {
        return divider::primary(theme, status)
    }     
    
    let style = style_opt.unwrap();

    if style.background_transparent {
        return divider::transparent(theme, status);
    }

    let mut base_style = divider::primary(theme, status);

    if style.background.is_some() {
        base_style.background = Background::Color(style.background.unwrap());
    };

    base_style.border_radius = get_radius(vec![style.border_radius; 4], 
                                            "Divider".to_string());
    
    if style.border_color.is_some() {
        base_style.border_color = style.border_color.unwrap();
    }

    base_style.border_width = style.border_width;

    let mut hovered_style = base_style;

    if style.background_hovered.is_some() {
        hovered_style.background = style.background_hovered.unwrap().into();
    }

    match status 
    {
        Status::Active => base_style,
        Status::Hovered => hovered_style,
        Status::Dragged => base_style, // active and drag are same
    }


}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgDividerStyleParam {
    BackgroundIpgColor,
    BackgroundRgbaColor,
    BackgroundTransparent,
    BorderIpgColor,
    BorderRgbaColor,
    BorderWidth,
    BorderRadius,
}

pub fn divider_style_update_item(
        style: &mut IpgDividerStyle,
        item: &PyObject,
        value: &PyObject,) 
{
    let update = try_extract_divider_style_update(item);
    let name = "DividerStyle".to_string();
    match update {
        IpgDividerStyleParam::BackgroundIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.background = get_color(None, Some(color), 1.0, false);
        },
        IpgDividerStyleParam::BackgroundRgbaColor => {
            style.background = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgDividerStyleParam::BackgroundTransparent => {
            style.background_transparent = try_extract_boolean(value, name)
        }
        IpgDividerStyleParam::BorderIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.border_color = get_color(None, Some(color), 1.0, false);
        },
        IpgDividerStyleParam::BorderRgbaColor => {
            style.border_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgDividerStyleParam::BorderWidth => {
            style.border_width = try_extract_f32(value, name);
        },
        IpgDividerStyleParam::BorderRadius =>  {
             style.border_radius = try_extract_f32(value, name);
        },
    }
}

pub fn try_extract_divider_style_update(update_obj: &PyObject) -> IpgDividerStyleParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgDividerStyleParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Divider style update extraction failed"),
        }
    })
}

fn get_divider_style(style: Option<&IpgWidgets>) -> Option<IpgDividerStyle>{
    match style {
        Some(IpgWidgets::IpgDividerStyle(style)) => {
            Some(style.clone())
        }
            _ => None,
        }
}
