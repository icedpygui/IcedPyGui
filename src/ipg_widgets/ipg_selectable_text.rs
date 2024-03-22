

use crate::app;
use crate::access_callbacks;
use super::callbacks::{WidgetCallbackIn, 
                        WidgetCallbackOut, 
                        get_set_widget_callback_data};

use iced::{Length, Element, Point};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::text::{LineHeight, Shaping};
use iced::widget::{MouseArea, Text};
use iced::mouse::Interaction;

use pyo3::{PyObject, Python};


#[derive(Debug, Clone)]
pub struct IpgSelectableText {
        pub id: usize,
        pub content: String,
        pub width: Length,
        pub height: Length,
        pub horizontal_alignment: Horizontal,
        pub vertical_alignment: Vertical,
        pub line_height: LineHeight,
        pub size: f32,
        pub show: bool,
        // pub font: Font,
        pub shaping: Shaping,
        // pub style: Style,
        pub user_data: Option<PyObject>,
}

impl IpgSelectableText {
    pub fn new( 
        id: usize,
        content: String,
        width: Length,
        height: Length,
        horizontal_alignment: Horizontal,
        vertical_alignment: Vertical,
        line_height: LineHeight,
        size: f32,
        show: bool,
        // font: Font,
        shaping: Shaping,
        // style: Style,
        user_data: Option<PyObject>,
        ) -> Self {
        Self {
            id,
            content,
            width,
            height,
            horizontal_alignment,
            vertical_alignment,
            line_height,
            size,
            show,
            // font: Font,
            shaping,
            // style: Style,
            user_data,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SLTXTMessage {
    OnPress,
    OnRelease,
    OnRightPress,
    OnRightRelease,
    OnMiddlePress,
    OnMiddleRelease,
    OnMove(Point),
    OnEnter,
    OnExit,
}

pub fn construct_selectable_text(sl_text: IpgSelectableText) -> Element<'static, app::Message> {
    
    let content: Element<'_, SLTXTMessage> = 
                        Text::new(sl_text.content.clone())
                            .size(sl_text.size)
                            .line_height(sl_text.line_height)
                            .width(sl_text.width)
                            .height(sl_text.height)
                            .horizontal_alignment(sl_text.horizontal_alignment)
                            .vertical_alignment(sl_text.vertical_alignment)
                            // font: Font,
                            .shaping(sl_text.shaping)
                            // style: Style,
                            .into();

    let ma: Element<'_, SLTXTMessage> = 
                MouseArea::new(content)
                    .on_press(SLTXTMessage::OnPress)
                    .on_release(SLTXTMessage::OnRelease)
                    .on_right_press(SLTXTMessage::OnRightPress)
                    .on_right_release(SLTXTMessage::OnRightRelease)
                    .on_middle_press(SLTXTMessage::OnMiddlePress)
                    .on_middle_release(SLTXTMessage::OnMiddleRelease)
                    .on_move(SLTXTMessage::OnMove)
                    .on_enter(SLTXTMessage::OnEnter)
                    .on_exit(SLTXTMessage::OnExit)
                    .interaction(Interaction::Pointer)
                    .into();

    ma.map(move |message| app::Message::SelectableText(sl_text.id, message))

}

pub fn selectable_text_callback(id: usize, message: SLTXTMessage) {

    let mut wci: WidgetCallbackIn = WidgetCallbackIn::default();
    wci.id = id;

    match message {
        SLTXTMessage::OnPress => {
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = Some("on_press".to_string());
            process_callback(wco);
        },
        SLTXTMessage::OnRelease => {
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = Some("on_release".to_string());
            process_callback(wco);
        },
        SLTXTMessage::OnRightPress => {
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = Some("on_right_press".to_string());
            process_callback(wco);
        },
        SLTXTMessage::OnRightRelease => {
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = Some("on_right_release".to_string());
            process_callback(wco);
        },
        SLTXTMessage::OnMiddlePress => {
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = Some("on_middle_press".to_string());
            process_callback(wco);
        },
        SLTXTMessage::OnMiddleRelease => {
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = Some("on_middle_release".to_string());
            process_callback(wco);
        },
        SLTXTMessage::OnEnter => {
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = Some("on_enter".to_string());
            process_callback(wco);
        },
        SLTXTMessage::OnMove(point) => {
            wci.point = Some(point);
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = Some("on_move".to_string());
            process_callback(wco);
        },
        SLTXTMessage::OnExit => {
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = Some("on_exit".to_string());
            process_callback(wco);
        },
    }    
}


fn process_callback(wco: WidgetCallbackOut) 
{
    if !wco.event_name.is_some() {return}

    let evt_name = match wco.event_name {
        Some(name) => name,
        None => panic!("event_name not found")
    };

    let app_cbs = access_callbacks();

    let callback_opt = app_cbs.callbacks.get(&(wco.id, evt_name.clone())).unwrap();
       
    let callback = match callback_opt {
        Some(cb) => cb,
        None => panic!("Callback could not be found with id {}", wco.id),
    };
                  
    if evt_name == "on_move".to_string() {

        Python::with_gil(|py| {
            if wco.user_data.is_some() {
                callback.call1(py, (
                                        wco.id.clone(), 
                                        evt_name.clone(),
                                        wco.points, 
                                        wco.user_data
                                        )
                                ).unwrap();
            } else {
                callback.call1(py, (
                                        wco.id.clone(), 
                                        evt_name.clone(),
                                        wco.points, 
                                        )
                                ).unwrap();
            } 
        });

    } else {
        Python::with_gil(|py| {
            if wco.user_data.is_some() {
                callback.call1(py, (
                                        wco.id.clone(), 
                                        evt_name.clone(), 
                                        wco.user_data
                                        )
                                ).unwrap();
            } else {
                callback.call1(py, (
                                        wco.id.clone(), 
                                        evt_name.clone(), 
                                        )
                                ).unwrap();
            } 
        });
    }
    
    drop(app_cbs);

}
