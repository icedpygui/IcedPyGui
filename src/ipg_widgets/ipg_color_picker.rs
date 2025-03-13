//! ipg_color_picker
use crate::graphics::colors::get_color;
use crate::{access_callbacks, access_user_data1, access_user_data2, IpgState};
use crate::app::Message;
use crate::style::styling::IpgStyleStandard;
use super::helpers::{get_height, get_padding_f64, get_width, 
    try_extract_boolean, try_extract_f64, try_extract_ipg_color, 
    try_extract_rgba_color, try_extract_string, try_extract_style_standard, 
    try_extract_vec_f32, try_extract_vec_f64};
use super::ipg_button::{self, get_bootstrap_arrow, 
    try_extract_button_arrow, IpgButtonArrow, IpgButtonStyle};
use super::callbacks::{set_or_get_widget_callback_data, WidgetCallbackIn};
use super::ipg_enums::IpgWidgets;

use iced::widget::{text, Button};
use iced::{Color, Element, Length, Padding, Theme};
use iced_aw::ColorPicker;

use pyo3::{pyclass, PyObject, Python};


#[derive(Debug, Clone)]
pub struct IpgColorPicker {
    pub id: usize,
    pub parent_id: String,
    pub show: bool,
    pub color: Color,
    //button related
    pub label: String,
    pub width: Length,
    pub height: Length,
    pub padding: Padding,
    pub clip: bool,
    pub style_id: Option<usize>,
    pub style_standard: Option<IpgStyleStandard>,
    pub style_arrow: Option<IpgButtonArrow>,
}

impl IpgColorPicker {
    pub fn new( 
        id: usize,
        parent_id: String,
        show: bool,
        color: Color,
        // button related
        label: String,
        width: Length,
        height: Length,
        padding: Padding,
        clip: bool,
        style_id: Option<usize>,
        style_standard: Option<IpgStyleStandard>,
        style_arrow: Option<IpgButtonArrow>,
        ) -> Self {
        Self {
            id,
            parent_id,
            show,
            color,
            // button related
            label,
            width,
            height,
            padding,
            clip,
            style_id,
            style_standard,
            style_arrow,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct IpgColorPickerStyle {
    pub id: usize,
    pub background_color: Option<Color>,
    pub background_color_hovered: Option<Color>,
    pub border_color: Option<Color>,
    pub border_radius: Vec<f32>,
    pub border_width: f32,
    pub shadow_color: Option<Color>,
    pub shadow_offset_x: f32,
    pub shadow_offset_y: f32,
    pub shadow_blur_radius: f32,
    pub text_color: Option<Color>,
}

impl IpgColorPickerStyle {
    pub fn new(
        id: usize,
        background_color: Option<Color>,
        background_color_hovered: Option<Color>,
        border_color: Option<Color>,
        border_radius: Vec<f32>,
        border_width: f32,
        shadow_color: Option<Color>,
        shadow_offset_x: f32,
        shadow_offset_y: f32,
        shadow_blur_radius: f32,
        text_color: Option<Color>,
    ) -> Self {
        Self {
            id,
            background_color,
            background_color_hovered,
            border_color,
            border_radius,
            border_width,
            shadow_color,
            shadow_offset_x,
            shadow_offset_y,
            shadow_blur_radius,
            text_color,
        }
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone)]
pub enum ColPikMessage {
    OnPress,
    OnCancel,
    OnSubmit(Color),
}


pub fn construct_color_picker<'a>(cp: &'a IpgColorPicker,
                                style_opt: Option<&IpgWidgets>,
                                ) -> Option<Element<'a, Message>> {

    let mut label = text(cp.label.clone());
   
    if cp.style_arrow.is_some() {
        let arrow = get_bootstrap_arrow(&cp.style_arrow);
        label = text(arrow).font(iced::Font::with_name("bootstrap-icons"));
    }

    let style = get_cp_style(style_opt);

    let btn: Element<ColPikMessage> = Button::new(label)
                                    .height(cp.height)
                                    .padding(cp.padding)
                                    .width(cp.width)
                                    .on_press(ColPikMessage::OnPress)
                                    .style(move|theme: &Theme, status| {   
                                        ipg_button::get_styling(theme, status,
                                            style.clone(),
                                            cp.style_standard.clone())
                                        })
                                    .into();

    if !cp.show {
        return Some(btn.map(move |message| Message::ColorPicker(cp.id, message)));
    }

    let color_picker: Element<ColPikMessage> = ColorPicker::new(
                                    cp.show,
                                    cp.color,
                                    btn,
                                    ColPikMessage::OnCancel,
                                    ColPikMessage::OnSubmit,
                                ).into();

    Some(color_picker.map(move |message| Message::ColorPicker(cp.id, message)))

}

pub fn color_picker_callback(state: &mut IpgState, id: usize, message: ColPikMessage) {
    let mut wci: WidgetCallbackIn = WidgetCallbackIn::default();
    match message {
        ColPikMessage::OnCancel => {
            wci.id = id;
            wci.value_bool = Some(false);
            let _ = set_or_get_widget_callback_data(state, wci);
            process_callback(id, "on_cancel".to_string(), None);
        },
        ColPikMessage::OnSubmit(color) => {
            wci.id = id;
            wci.value_bool = Some(false);
            wci.color = Some(convert_color_to_list(color));
            let _ = set_or_get_widget_callback_data(state, wci);
            process_callback(id, "on_submit".to_string(), Some(convert_color_to_list(color)));
        },
        ColPikMessage::OnPress => {
            wci.id = id;
            wci.value_bool = Some(true);
            let _ = set_or_get_widget_callback_data(state, wci);
            process_callback(id, "on_press".to_string(), None);
        },
    }
}


pub fn process_callback(id: usize, event_name: String, color: Option<Vec<f64>>) 
{
    let ud = access_user_data1();
    
    let ud_opt = ud.user_data.get(&id);

    let mut ud_opt_chk = false;
    let mut ud2_opt_chk = false;

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
    
    // Needed to split up the callback due to the need
    // to drop as as possible, one needs to be free
    // at all times.
    if ud_opt.is_some() {
        ud_opt_chk = true;
        Python::with_gil(|py| {

            let res = 
                cb.call1(py, (
                        id,
                        color.clone().unwrap(),
                        ud_opt.unwrap()
                        ));
            match res {
                Ok(_) => (),
                Err(er) => panic!("ColorPicker: 3 parameters (id, color, user_data) are required or 
                                        a python error in this function. {er}"),
            }
                
        });
    }

    drop(ud);

    let ud2 = access_user_data2();
    let ud2_opt = ud2.user_data.get(&id);

    if ud2_opt.is_some() {
        ud2_opt_chk = true;
        Python::with_gil(|py| {

            let res = 
                cb.call1(py, (
                        id,
                        color.clone().unwrap(),
                        ud2_opt.unwrap()
                        ));
            match res {
                Ok(_) => (),
                Err(er) => panic!("ColorPicker: 3 parameters (id, color, user_data) 
                                            are required or a python 
                                            error in this function. {er}"),
            }
        });
    }

    drop(ud2);

    if !ud_opt_chk && !ud2_opt_chk {
        Python::with_gil(|py| {
            if color.is_some() {
                let res = 
                    cb.call1(py, (
                            id,
                            color.unwrap(),  
                            ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("ColorPicker: 2 parameter (id, color) is required or possibly a python 
                                            error in this function. {er}"),
                }
            } else {
                let res = 
                    cb.call1(py, (
                            id, 
                            ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("ColorPicker: 1 parameter (id) is required or possibly a python 
                                            error in this function. {er}"),
                }
            }
            
        });
    }
}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgColorPickerParam {
    ArrowStyle,
    Clip,
    Color,
    Height,
    HeightFill,
    Label,
    Padding,
    Show,
    StyleId,
    StyleStandard,
    Width,
    WidthFill,
}

pub fn color_picker_update(cp: &mut IpgColorPicker,
                            item: &PyObject,
                            value: &PyObject,
                            ) 
{
    let update = try_extract_cp_update(item);
    let name = "ColorPicker".to_string();
    match update {
       IpgColorPickerParam::ArrowStyle => {
            cp.style_arrow = Some(try_extract_button_arrow(value));
        },
        IpgColorPickerParam::Color => {
            let rgba = try_extract_rgba_color(value, name);
            cp.color = Color::from(rgba);
        },
        IpgColorPickerParam::Label => {
            cp.label = try_extract_string(value, name);
        },
        IpgColorPickerParam::Height => {
            let val = try_extract_f64(value, name);
            cp.height = get_height(Some(val as f32), false);
        },
        IpgColorPickerParam::HeightFill => {
            let val = try_extract_boolean(value, name);
            cp.height = get_height(None, val);
        },
        IpgColorPickerParam::Padding => {
            let val = try_extract_vec_f64(value, name);
            cp.padding =  get_padding_f64(val);
        },
        IpgColorPickerParam::Clip => {
            cp.clip = try_extract_boolean(value, name);
        }
        IpgColorPickerParam::Show => {
            cp.show = try_extract_boolean(value, name);
        },
        IpgColorPickerParam::StyleId => {
            let val = try_extract_f64(value, name) as usize;
            cp.style_id = Some(val);
        },
        IpgColorPickerParam::StyleStandard => {
            let val = try_extract_style_standard(value, name);
            cp.style_standard = Some(val);
        },
        IpgColorPickerParam::Width => {
            let val = try_extract_f64(value, name);
            cp.width = get_width(Some(val as f32), false);
        },
        IpgColorPickerParam::WidthFill => {
            let val = try_extract_boolean(value, name);
            cp.width = get_width(None, val);
        },
    }
}

pub fn try_extract_cp_update(update_obj: &PyObject) -> IpgColorPickerParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgColorPickerParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Color Picker update extraction failed"),
        }
    })
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgColorPickerStyleParam {
    BackgroundIpgColor,
    BackgroundRbgaColor,
    BackgroundIpgColorHovered,
    BackgroundIpgRgbaHovered,
    BorderIpgColor,
    BorderRgbaColor,
    BorderRadius,
    BorderWidth,
    ShadowIpgColor,
    ShadowRgbaColor,
    ShadowOffsetX,
    ShadowOffsetY,
    ShadowBlurRadius,
    TextIpgColor,
    TextRgbaColor
}

pub fn color_picker_style_update_item(style: &mut IpgColorPickerStyle,
                                        item: &PyObject,
                                        value: &PyObject,) 
{

    let update = try_extract_color_picker_style_update(item);
    let name = "ColorPickerStyle".to_string();
    match update {
        IpgColorPickerStyleParam::BackgroundIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.background_color = get_color(None, Some(color), 1.0, false);
        },
        IpgColorPickerStyleParam::BackgroundRbgaColor => {
            style.background_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgColorPickerStyleParam::BackgroundIpgColorHovered => {
            let color = try_extract_ipg_color(value, name);
            style.background_color_hovered = get_color(None, Some(color), 1.0, false);
        },
        IpgColorPickerStyleParam::BackgroundIpgRgbaHovered => {
            style.background_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgColorPickerStyleParam::BorderIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.border_color = get_color(None, Some(color), 1.0, false);
        },
        IpgColorPickerStyleParam::BorderRgbaColor => {
            style.border_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgColorPickerStyleParam::BorderRadius => {
            style.border_radius = try_extract_vec_f32(value, name);
        },
        IpgColorPickerStyleParam::BorderWidth => {
            style.border_width = try_extract_f64(value, name) as f32;
        },
        IpgColorPickerStyleParam::ShadowIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.shadow_color = get_color(None, Some(color), 1.0, false);
        },
        IpgColorPickerStyleParam::ShadowRgbaColor => {
            style.border_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgColorPickerStyleParam::ShadowOffsetX => {
            style.shadow_offset_x = try_extract_f64(value, name) as f32;
        },
        IpgColorPickerStyleParam::ShadowOffsetY => {
            style.shadow_offset_y = try_extract_f64(value, name) as f32;
        },
        IpgColorPickerStyleParam::ShadowBlurRadius => {
            style.shadow_blur_radius = try_extract_f64(value, name) as f32;
        },
        IpgColorPickerStyleParam::TextIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.text_color = get_color(None, Some(color), 1.0, false);
        },
        IpgColorPickerStyleParam::TextRgbaColor => {
            style.text_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
    }
}

pub fn try_extract_color_picker_style_update(update_obj: &PyObject) -> IpgColorPickerStyleParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgColorPickerStyleParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Color Picker style update extraction failed"),
        }
    })
}

fn get_cp_style(style: Option<&IpgWidgets>) -> Option<IpgButtonStyle>{
    match style {
        Some(st) => {
            match st {
                IpgWidgets::IpgColorPickerStyle(style) => {
                    Some(IpgButtonStyle {
                        id: style.id,
                        background_color: style.background_color,
                        background_color_hovered: style.background_color_hovered,
                        border_color: style.border_color,
                        border_radius: style.border_radius.clone(),
                        border_width: style.border_width,
                        shadow_color: style.shadow_color,
                        shadow_offset_x: style.shadow_offset_x,
                        shadow_offset_y: style.shadow_offset_y,
                        shadow_blur_radius: style.shadow_blur_radius,
                        text_color: style.text_color,
                    })
                }
                _ => None,
            }
        },
        None => None,
    }
}

// pub fn get_styling(theme: &Theme, status: Status,
//                     style_opt: Option<IpgColorPickerStyle>,
//                     style_standard: Option<IpgStyleStandard>,
//                     ) -> button::Style 
// {
//     if style_standard.is_none() && style_opt.is_none() {
//         return button::primary(theme, status)
//     }

//     if style_opt.is_none() && style_standard.is_some() {
//             return get_standard_style(theme, status, style_standard, None, None)
//     }

//     let mut border = Border::default();
//     let mut shadow = Shadow::default();

//     let mut base_style = button::primary(theme, status);
//     let mut hover_style = button::primary(theme, status);

//     let style = style_opt.unwrap_or_default();

//     if style.border_color.is_some() {
//         border.color = style.border_color.unwrap();
//     }

//     border.radius = get_radius(style.border_radius.clone(), 
//                                 "Button".to_string());
//     border.width = style.border_width;

//     if style.shadow_color.is_some() {
//         shadow.color = style.shadow_color.unwrap();
//         shadow.offset = Vector{ x: style.shadow_offset_x, y: style.shadow_offset_y };
//         shadow.blur_radius = style.shadow_blur_radius;
//     }

//     // style_standard overrides style except for border and shadow
//     let style_standard = get_standard_style(theme, status, 
//                                     style_standard, 
//                                     Some(border), Some(shadow));
    
//     base_style.background = if style.background_color.is_some() {
//         Some(style.background_color.unwrap().into())
//     } else {
//         style_standard.background
//     };

//     hover_style.background = if style.background_color_hovered.is_some() {
//         Some(style.background_color_hovered.unwrap().into())
//     } else {
//         style_standard.background
//     };

//     base_style.border = border;
//     hover_style.border = border;

//     base_style.shadow = shadow;
//     hover_style.shadow = shadow;

//     match status {
//         Status::Active | Status::Pressed => base_style,
//         Status::Hovered => hover_style,
//         Status::Disabled => disabled(base_style),
//     }
    
// }

// fn disabled(style: Style) -> Style {
//     Style {
//         background: style
//             .background
//             .map(|background| background.scale_alpha(0.5)),
//         text_color: style.text_color.scale_alpha(0.5),
//         ..style
//     }
// }

fn convert_color_to_list(color: Color) -> Vec<f64> {

    vec![
        rnd_2(color.r),
        rnd_2(color.g),
        rnd_2(color.b),
        rnd_2(color.a),
    ]
}

fn rnd_2(rgba: f32) -> f64 {
    let num = rgba as f64 * 100.0;
    num.round()/100.0
}
