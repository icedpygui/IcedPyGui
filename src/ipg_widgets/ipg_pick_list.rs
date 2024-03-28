

use crate::access_callbacks;
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

use iced::{Padding, Length, Element};
use iced::widget::{PickList, Space};
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
    // font: Option<Renderer::Font>,
    // handle: Handle<Renderer::Font>,
    // style: <Renderer::Theme as StyleSheet>::Style,
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
        // font: Option<Renderer::Font>,
        // handle: Handle<Renderer::Font>,
        // style: <Renderer::Theme as StyleSheet>::Style,
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
            // font: Option<Renderer::Font>,
            // handle: Handle<Renderer::Font>,
            // style: <Renderer::Theme as StyleSheet>::Style,
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
                Err(_) => panic!("PickList: 3 parameters (id, value, user_data) are required or possibly a non-fatal python error in this function."),
            }
        } else {
            let res = callback.call1(py, (
                                                                wco.id.clone(), 
                                                                value, 
                                                                ));
            match res {
                Ok(_) => (),
                Err(_) => panic!("InputText: 2 parameters (id, value) are required or possibly a non-fatal python error in this function."),
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