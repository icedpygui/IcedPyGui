//! ipg_opaque
use iced::mouse::Interaction;
use iced::{Color, Element, Length, Theme};
use iced::widget::{container, horizontal_space, mouse_area, opaque, Container};
use pyo3::{pyclass, PyObject, Python};

use crate::{access_callbacks, IpgState};
use crate::app::Message;

use super::callbacks::{container_callback_data, WidgetCallbackIn, WidgetCallbackOut};
use super::helpers::{get_horizontal_alignment, get_vertical_alignment, try_extract_boolean};
use super::ipg_enums::{IpgHorizontalAlignment, IpgVerticalAlignment};


#[derive(Debug, Clone)]
pub struct IpgOpaque {
    pub id: usize,
    pub width: Length,
    pub height: Length,
    pub align_h: Option<IpgHorizontalAlignment>,
    pub align_v: Option<IpgVerticalAlignment>,
    pub include_mouse_area: bool,
    pub show: bool,
    pub style_id: Option<String>, 
}

impl IpgOpaque {
    pub fn new(
        id: usize,
        width: Length,
        height: Length,
        align_h: Option<IpgHorizontalAlignment>,
        align_v: Option<IpgVerticalAlignment>,
        include_mouse_area: bool,
        show: bool,
        style_id: Option<String>,
    ) -> Self {
        Self {
            id,
            width,
            height,
            align_h,
            align_v,
            include_mouse_area,
            show,
            style_id, 
        }
    }
}

#[derive(Debug, Clone)]
pub struct IpgOpaqueStyle {
    pub id: usize,
    pub background_color: Option<Color>,
}

impl IpgOpaqueStyle {
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

pub fn construct_opaque(op: IpgOpaque, 
                        mut content: Vec<Element<'static, Message>>, 
                        style: Option<IpgOpaqueStyle> ) 
                        -> Element<'static, Message> {

    let new_content = if content.len() > 0 {
        content.remove(0)
    } else {
        horizontal_space().into()
    };

    let align_h = get_horizontal_alignment(op.align_h.clone());
    let align_v = get_vertical_alignment(op.align_v.clone());

    let cont: Element<Message> = Container::new(new_content)
                .width(op.width)
                .height(op.height)
                .align_x(align_h)
                .align_y(align_v)
                .style(move|theme|
                    get_styling(&theme, 
                        style.clone(),
                        ))
                .into();
    
    if op.include_mouse_area {
        opaque(mouse_area(cont)
            .on_press(Message::OpaqueOnPress(op.id))
            .interaction(Interaction::Pointer))
    } else {
        opaque(cont).into()
    }

    
}


pub fn get_styling(theme: &Theme,
                style_opt: Option<IpgOpaqueStyle>,  
                ) -> container::Style {
    
    if style_opt.is_none() {
        return container::transparent(theme);
    }

    let style = style_opt.unwrap();

    let background_color = if style.background_color.is_some() {
        style.background_color.unwrap()
    } else {
        Color::TRANSPARENT
    };

    container::Style {
        background: Some(background_color.into()),
        ..Default::default()
    }
    
}

#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgOpaqueParam {
    Show,
}

pub fn opaque_item_update(op: &mut IpgOpaque,
                            item: PyObject,
                            value: PyObject,) {

    let update = try_extract_stack_update(item);

    match update {
        IpgOpaqueParam::Show => {
            op.show = try_extract_boolean(value);
        },
    }
}

pub fn try_extract_stack_update(update_obj: PyObject) -> IpgOpaqueParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgOpaqueParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Opaque update extraction failed"),
        }
    })
}

pub fn opaque_callback(state: &mut IpgState, id: usize, event_name: String) {
    
    let mut wci: WidgetCallbackIn = WidgetCallbackIn::default();
    wci.id = id;

    let mut wco = container_callback_data(state, wci);
    wco.id = id;
    wco.event_name = event_name;
    process_callback(wco);

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
        None => panic!("Opaque Callback could not be found with id {}", wco.id),
    };
              
    Python::with_gil(|py| {
        let res = callback.call1(py, (
                                                            wco.id.clone(),  
                                                            ));
        match res {
            Ok(_) => (),
            Err(er) => panic!("Opaque: Only 1 parameter (id) is required or a python error in this function. {er}"),
        }
        
    });
    
    drop(app_cbs);   

}

