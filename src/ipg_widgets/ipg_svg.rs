
use crate::app;
use crate::access_callbacks;
use super::callbacks::{WidgetCallbackIn, 
                        WidgetCallbackOut, 
                        get_set_widget_callback_data};
use super::helpers::get_height;
use super::helpers::get_width;
use super::helpers::try_extract_boolean;
use super::helpers::try_extract_f64;
use super::helpers::try_extract_string;

use iced::{Length, Element, Point};
use iced::widget::{Svg, MouseArea};
use iced::mouse::Interaction;
use iced::advanced::svg;

use pyo3::pyclass;
use pyo3::types::IntoPyDict;
use pyo3::{PyObject, Python};


#[derive(Debug, Clone)]
pub struct IpgSvg {
        pub id: usize,
        pub svg_path: String,
        pub width: Length,
        pub height: Length,
        pub style: IpgSvgStyle,
        pub show: bool,
        pub user_data: Option<PyObject>,
}

impl IpgSvg {
    pub fn new( 
        id: usize,
        svg_path: String,
        width: Length,
        height: Length,
        show: bool,
        user_data: Option<PyObject>,
        ) -> Self {
        Self {
            id,
            svg_path,
            width,
            height,
            style: IpgSvgStyle::Default,
            show,
            user_data,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SvgMessage {
    OnPress,
    OnRelease,
    OnRightPress,
    OnRightRelease,
    OnMiddlePress,
    OnMiddleRelease,
    OnEnter,
    OnMove(Point),
    OnExit,
}

pub fn construct_svg(sg: IpgSvg) -> Element<'static, app::Message> {

    // let style: theme::Svg = get_style(sg.style);

    let svg_handle = svg::Handle::from_path(sg.svg_path);

    let svg_widget: Element<SvgMessage> = Svg::new(svg_handle)
                                                .width(sg.width)
                                                .height(sg.height)
                                                // .style(style)
                                                .into();

    let widget: Element<SvgMessage> = 
                MouseArea::new(svg_widget)
                    .on_press(SvgMessage::OnPress)
                    .on_release(SvgMessage::OnRelease)
                    .on_right_press(SvgMessage::OnRightPress)
                    .on_right_release(SvgMessage::OnRightRelease)
                    .on_middle_press(SvgMessage::OnMiddlePress)
                    .on_middle_release(SvgMessage::OnMiddleRelease)
                    .on_enter(SvgMessage::OnEnter)
                    .on_move(SvgMessage::OnMove)
                    .on_exit(SvgMessage::OnExit)
                    //Need to add in the other Interactions
                    .interaction(Interaction::Pointer)
                    .into();

    widget.map(move |message| app::Message::Svg(sg.id, message))

}

pub fn svg_callback(id: usize, message: SvgMessage) {

    let mut wci: WidgetCallbackIn = WidgetCallbackIn::default();
    wci.id = id;

    match message {
        SvgMessage::OnPress => {
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = "on_press".to_string();
            process_callback(wco);
        },
        SvgMessage::OnRelease => {
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = "on_release".to_string();
            process_callback(wco);
        },
        SvgMessage::OnRightPress => {
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = "on_right_press".to_string();
            process_callback(wco);
        },
        SvgMessage::OnRightRelease => {
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = "on_right_release".to_string();
            process_callback(wco);
        },
        SvgMessage::OnMiddlePress => {
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = "on_middle_press".to_string();
            process_callback(wco);
        },
        SvgMessage::OnMiddleRelease => {
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = "on_middle_release".to_string();
            process_callback(wco);
        },
        SvgMessage::OnEnter => {
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = "on_enter".to_string();
            process_callback(wco);
        },
        SvgMessage::OnMove(point) => {
            let mut points: Vec<(String, f32)> = vec![];
            points.push(("x".to_string(), point.x));
            points.push(("y".to_string(), point.y));
            
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = "on_move".to_string();
            wco.points = Some(points);
            process_callback(wco);
        },
        SvgMessage::OnExit => {
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = "on_exit".to_string();
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
        None => panic!("Image Callback could not be found with id {}", wco.id),
    };
              
    if wco.event_name == "on_move".to_string() {

        let points = match wco.points {
            Some(pts) => pts,
            None => panic!("Image Points not found"),
        };

        Python::with_gil(|py| {
            if wco.user_data.is_some() {
                let user_data = match wco.user_data {
                    Some(ud) => ud,
                    None => panic!("Image callback user_data not found."),
                };
                let res = callback.call1(py, (
                                                                    wco.id.clone(), 
                                                                    points.into_py_dict_bound(py), 
                                                                    user_data
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Image: 3 parameter (id, points, user_data) are required or a python error in this function. {er}"),
                }
            } else {
                let res = callback.call1(py, (
                                                                    wco.id.clone(), 
                                                                    points.into_py_dict_bound(py), 
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Image: 2 parameter (id, points) are required or a python error in this function. {er}"),
                }
            } 
        });

    } else {
        Python::with_gil(|py| {
            if wco.user_data.is_some() {
                let user_data = match wco.user_data {
                    Some(ud) => ud,
                    None => panic!("Image callback user_data not found."),
                };
                let res = callback.call1(py, (
                                                                    wco.id.clone(), 
                                                                    user_data
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Image: 2 parameter (id, user_data) are required or a python error in this function. {er}"),
                }
            } else {
                let res = callback.call1(py, (
                                                                    wco.id.clone(),  
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Image: Only 1 parameter (id) is required or a python error in this function. {er}"),
                }
            } 
        });
    }
    
    drop(app_cbs);   

}


#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgSvgParams {
    Height,
    HeightFill,
    ImagePath,
    Show,
    Width,
    WidthFill,
}


pub fn svg_item_update(img: &mut IpgSvg,
                            item: PyObject,
                            value: PyObject,
                            )
{

    let update = try_extract_svg_update(item);

    match update {
        IpgSvgParams::Height => {
            let val = try_extract_f64(value);
            img.height = get_height(Some(val as f32), false);
        },
        IpgSvgParams::HeightFill => {
            let val = try_extract_boolean(value);
            img.height = get_height(None, val);
        },
        IpgSvgParams::ImagePath => {
            img.svg_path = try_extract_string(value);
        },
        IpgSvgParams::Show => {
            img.show = try_extract_boolean(value);
        },
        IpgSvgParams::Width => {
            let val = try_extract_f64(value);
            img.width = get_width(Some(val as f32), false);
        },
        IpgSvgParams::WidthFill => {
            let val = try_extract_boolean(value);
            img.width = get_width(None, val);
        },
    }
}

pub fn try_extract_svg_update(update_obj: PyObject) -> IpgSvgParams {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgSvgParams>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Svg update extraction failed"),
        }
    })
}

#[derive(Debug, Clone)]
pub enum IpgSvgStyle {
    Default,
    // Custom,
}

// fn get_style(st: IpgSvgStyle) -> theme::Svg {
//     match st {
//         IpgSvgStyle::Default => theme::Svg::Default,
//         // IpgSvgStyle::Custom => todo!(),
//     }
// }