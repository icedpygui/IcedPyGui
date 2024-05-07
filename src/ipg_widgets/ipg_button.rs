#![allow(dead_code)]

use crate::{access_callbacks, app};
use super::helpers::{get_height, get_padding, get_width, 
                    try_extract_f64, try_extract_string, 
                    try_extract_boolean, try_extract_vec_f64};
use super::callbacks::{
    WidgetCallbackIn, WidgetCallbackOut, 
    get_set_widget_callback_data
};

use pyo3::{pyclass, PyObject, Python};

use iced::widget::{Button, Space, Text};
use iced::{Border, Color, Element, Length, Padding, theme, Theme, };

use iced::widget::button::{self, Appearance, StyleSheet};

use iced_aw::BootstrapIcon;
use iced_aw::BOOTSTRAP_FONT;
use iced_aw::graphics::icons::icon_to_string;


#[derive(Debug, Clone)]
pub struct IpgButton {
    pub id: usize,
    pub show: bool,
    pub user_data: Option<PyObject>,

    pub label: String,
    pub width: Length,
    pub height: Length,
    pub padding: Padding,
    pub corner_radius: f32,
    pub style: Option<PyObject>,
    pub arrow_style: Option<PyObject>,
}

impl IpgButton {
    pub fn new( 
        id: usize,
        show: bool,
        user_data: Option<PyObject>,

        label: String,
        width: Length,
        height: Length,
        padding: Padding,
        corner_radius: f32,
        style: Option<PyObject>,
        arrow_style: Option<PyObject>,
        ) -> Self {
        Self {
            id,
            show,
            user_data,
            label,
            width,
            height,
            padding,
            corner_radius,
            style,
            arrow_style,
        }
    }
}

#[derive(Debug, Clone)]
pub enum BTNMessage {
    OnPress,
}


#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgButtonStyles {
    Primary,
    Secondary,
    Positive,
    Destructive,
    Text,
}


pub fn construct_button(btn: IpgButton) -> Element<'static, app::Message> {

    if !btn.show {
        return Space::new(Length::Shrink, Length::Shrink).into()
    }

    let mut label = Text::new(btn.label.clone());

    if btn.arrow_style.is_some() {
        let arrow_style = try_extract_button_arrow(btn.arrow_style);
        label = match arrow_style {
            Some(ar) => Text::new(ar).font(BOOTSTRAP_FONT),
            None => panic!("Button: Could not get extract arrow_style")
        };
    }
    
    let style = get_button_style_from_obj(btn.style);
    
    let ipg_btn: Element<BTNMessage> = Button::new(label)
                                .height(btn.height)
                                .padding(btn.padding)
                                .width(btn.width)
                                .on_press(BTNMessage::OnPress)
                                .style(theme::Button::Custom(Box::new(
                                    ButtonStyleRadius::new(style, btn.corner_radius))))
                                .into();

    ipg_btn.map(move |message| app::Message::Button(btn.id, message))
    
}


pub fn button_callback(id: usize, message: BTNMessage) {

    let mut wci = WidgetCallbackIn::default();
    wci.id = id;

    match message {
        BTNMessage::OnPress => {
            // getting only
            let mut wco: WidgetCallbackOut = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = "on_press".to_string();
            process_callback(wco);
        }
    }
}


pub fn process_callback(wco: WidgetCallbackOut) 
{
    let app_cbs = access_callbacks();

    let callback_present = app_cbs.callbacks.get(&(wco.id, wco.event_name.clone()));

    let callback_opt = match callback_present {
        Some(cb) => cb,
        None => return,
    };

    let callback = match callback_opt {
        Some(cb) => cb,
        None => panic!("Button callback could not be found with id {}", wco.id),
    };

    Python::with_gil(|py| {
            if wco.user_data.is_some() {
                let user_data = match wco.user_data {
                    Some(ud) => ud,
                    None => panic!("User Data could not be found in Button callback"),
                };
                let res = callback.call1(py, (
                                                                    wco.id.clone(),  
                                                                    user_data
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Button: 2 parameters (id, user_data) are required or a python error in this function. {er}"),
                }
            } else {
                let res = callback.call1(py, (
                                                                    wco.id.clone(),  
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Button: 1 parameter (id) is required or possibly a python error in this function. {er}"),
                }
            } 
    });
    
    drop(app_cbs);
         
}


#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgButtonParams {
    ArrowStyle,
    CornerRadius,
    Height,
    HeightFill,
    Label,
    Padding,
    Show,
    Style,
    Width,
    WidthFill,
}


pub fn button_item_update(btn: &mut IpgButton,
                            item: PyObject,
                            value: PyObject,
                            )
{
    let update = try_extract_button_update(item);

    match update {
       IpgButtonParams::ArrowStyle => {
            btn.arrow_style = Some(value);
        },
        IpgButtonParams::CornerRadius => {
            btn.corner_radius = try_extract_f64(value) as f32;
        },
        IpgButtonParams::Label => {
            btn.label = try_extract_string(value);
        },
        IpgButtonParams::Height => {
            let val = try_extract_f64(value);
            btn.height = get_height(Some(val as f32), false);
        },
        IpgButtonParams::HeightFill => {
            let val = try_extract_boolean(value);
            btn.height = get_height(None, val);
        },
        IpgButtonParams::Padding => {
            let val = try_extract_vec_f64(value);
            btn.padding =  get_padding(val);
        },
        IpgButtonParams::Show => {
            btn.show = try_extract_boolean(value);
        },
        IpgButtonParams::Style => {
            btn.style = Some(value);
        },
        IpgButtonParams::Width => {
            let val = try_extract_f64(value);
            btn.width = get_width(Some(val as f32), false);
        },
        IpgButtonParams::WidthFill => {
            let val = try_extract_boolean(value);
            btn.width = get_width(None, val);
        },
    }

}


pub fn get_button_style_from_obj(style_opt: Option<PyObject>) -> theme::Button {

    let style_obj = match style_opt {
        Some(st) => st,
        None => return theme::Button::Primary,
    };

    let ipg_btn_style = try_extract_button_style(style_obj);

    match ipg_btn_style {
        IpgButtonStyles::Primary => theme::Button::Primary,
        IpgButtonStyles::Secondary => theme::Button::Secondary,
        IpgButtonStyles::Positive => theme::Button::Positive,
        IpgButtonStyles::Destructive => theme::Button::Destructive,
        IpgButtonStyles::Text => theme::Button::Text,
    }
}


pub fn try_extract_button_style(style_obj: PyObject) -> IpgButtonStyles {

    Python::with_gil(|py| {
        let res = style_obj.extract::<IpgButtonStyles>(py);
            
        match res {
            Ok(st) => st,
            Err(_) => panic!("Button style extraction failed."),
        }
    })  
}

pub fn try_extract_button_arrow(arrow_opt: Option<PyObject>) -> Option<String> {

    let arrow_obj = match arrow_opt {
        Some(ar) => ar,
        None => return None,
    };

    Python::with_gil(|py| {
        let res = arrow_obj.extract::<IpgButtonArrows>(py);

        match res {
            Ok(ar) => return Some(get_bootstrap_arrow(ar)),
            Err(_) => panic!("Button arrow extraction failed"),
        }
    })
}


pub fn try_extract_button_update(update_obj: PyObject) -> IpgButtonParams {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgButtonParams>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Button update extraction failed"),
        }
    })
}


#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgButtonArrows {
    ArrowBarLeft,
    ArrowBarRight,
    ArrowBarUp,
    ArrowClockwise,
    ArrowCounterclockwise,
    ArrowDown,
    ArrowDownCircle,
    ArrowDownCircleFill,
    ArrowDownLeft,
    ArrowDownLeftCircle,
    ArrowDownLeftCircleFill,
    ArrowDownLeftSquare,
    ArrowDownLeftSquareFill,
    ArrowDownRight,
    ArrowDownRightCircle,
    ArrowDownRightCircleFill,
    ArrowDownRightSquare,
    ArrowDownRightSquareFill,
    ArrowDownShort,
    ArrowDownSquare,
    ArrowDownSquareFill,
    ArrowDownUp,
    ArrowLeft,
    ArrowLeftCircle,
    ArrowLeftCircleFill,
    ArrowLeftRight,
    ArrowLeftShort,
    ArrowLeftSquare,
    ArrowLeftSquareFill,
    ArrowNinezerodegDown,
    ArrowNinezerodegLeft,
    ArrowNinezerodegRight,
    ArrowNinezerodegUp,
    ArrowRepeat,
    ArrowReturnLeft,
    ArrowReturnRight,
    ArrowRight,
    ArrowRightCircle,
    ArrowRightCircleFill,
    ArrowRightShort,
    ArrowRightSquare,
    ArrowRightSquareFill,
    ArrowThroughHeart,
    ArrowThroughHeartFill,
    ArrowUp,
    ArrowUpCircle,
    ArrowUpCircleFill,
    ArrowUpLeft,
    ArrowUpLeftCircle,
    ArrowUpLeftCircleFill,
    ArrowUpLeftSquare,
    ArrowUpLeftSquareFill,
    ArrowUpRight,
    ArrowUpRightCircle,
    ArrowUpRightCircleFill,
    ArrowUpRightSquare,
    ArrowUpRightSquareFill,
    ArrowUpShort,
    ArrowUpSquare,
    ArrowUpSquareFill,
    Arrows,
    ArrowsAngleContract,
    ArrowsAngleExpand,
    ArrowsCollapse,
    ArrowsCollapseVertical,
    ArrowsExpand,
    ArrowsExpandVertical,
    ArrowsFullscreen,
    ArrowsMove,
    ArrowsVertical,
}


fn get_bootstrap_arrow(arrow: IpgButtonArrows) -> String {
    match arrow {
        IpgButtonArrows::ArrowBarLeft => icon_to_string(BootstrapIcon::ArrowBarLeft),
        IpgButtonArrows::ArrowBarRight => icon_to_string(BootstrapIcon::ArrowBarRight),
        IpgButtonArrows::ArrowBarUp => icon_to_string(BootstrapIcon::ArrowBarUp),
        IpgButtonArrows::ArrowClockwise => icon_to_string(BootstrapIcon::ArrowClockwise),
        IpgButtonArrows::ArrowCounterclockwise => icon_to_string(BootstrapIcon::ArrowCounterclockwise),
        IpgButtonArrows::ArrowDown => icon_to_string(BootstrapIcon::ArrowDown),
        IpgButtonArrows::ArrowDownCircle => icon_to_string(BootstrapIcon::ArrowDownCircle),
        IpgButtonArrows::ArrowDownCircleFill => icon_to_string(BootstrapIcon::ArrowDownCircleFill),
        IpgButtonArrows::ArrowDownLeft => icon_to_string(BootstrapIcon::ArrowDownLeft),
        IpgButtonArrows::ArrowDownLeftCircle => icon_to_string(BootstrapIcon::ArrowDownLeftCircle),
        IpgButtonArrows::ArrowDownLeftCircleFill => icon_to_string(BootstrapIcon::ArrowDownLeftCircleFill),
        IpgButtonArrows::ArrowDownLeftSquare => icon_to_string(BootstrapIcon::ArrowDownLeftSquare),
        IpgButtonArrows::ArrowDownLeftSquareFill => icon_to_string(BootstrapIcon::ArrowDownLeftSquareFill),
        IpgButtonArrows::ArrowDownRight => icon_to_string(BootstrapIcon::ArrowDownRight),
        IpgButtonArrows::ArrowDownRightCircle => icon_to_string(BootstrapIcon::ArrowDownRightCircle),
        IpgButtonArrows::ArrowDownRightCircleFill => icon_to_string(BootstrapIcon::ArrowDownRightCircleFill),
        IpgButtonArrows::ArrowDownRightSquare => icon_to_string(BootstrapIcon::ArrowDownRightSquare),
        IpgButtonArrows::ArrowDownRightSquareFill => icon_to_string(BootstrapIcon::ArrowDownRightSquareFill),
        IpgButtonArrows::ArrowDownShort => icon_to_string(BootstrapIcon::ArrowDownShort),
        IpgButtonArrows::ArrowDownSquare => icon_to_string(BootstrapIcon::ArrowDownSquare),
        IpgButtonArrows::ArrowDownSquareFill => icon_to_string(BootstrapIcon::ArrowDownSquareFill),
        IpgButtonArrows::ArrowDownUp => icon_to_string(BootstrapIcon::ArrowDownUp),
        IpgButtonArrows::ArrowLeft => icon_to_string(BootstrapIcon::ArrowLeft),
        IpgButtonArrows::ArrowLeftCircle => icon_to_string(BootstrapIcon::ArrowLeftCircle),
        IpgButtonArrows::ArrowLeftCircleFill => icon_to_string(BootstrapIcon::ArrowLeftCircleFill),
        IpgButtonArrows::ArrowLeftRight => icon_to_string(BootstrapIcon::ArrowLeftRight),
        IpgButtonArrows::ArrowLeftShort => icon_to_string(BootstrapIcon::ArrowLeftShort),
        IpgButtonArrows::ArrowLeftSquare => icon_to_string(BootstrapIcon::ArrowLeftSquare),
        IpgButtonArrows::ArrowLeftSquareFill => icon_to_string(BootstrapIcon::ArrowLeftSquareFill),
        IpgButtonArrows::ArrowNinezerodegDown => icon_to_string(BootstrapIcon::ArrowNinezerodegDown),
        IpgButtonArrows::ArrowNinezerodegLeft => icon_to_string(BootstrapIcon::ArrowNinezerodegLeft),
        IpgButtonArrows::ArrowNinezerodegRight => icon_to_string(BootstrapIcon::ArrowNinezerodegRight),
        IpgButtonArrows::ArrowNinezerodegUp => icon_to_string(BootstrapIcon::ArrowNinezerodegUp),
        IpgButtonArrows::ArrowRepeat => icon_to_string(BootstrapIcon::ArrowRepeat),
        IpgButtonArrows::ArrowReturnLeft => icon_to_string(BootstrapIcon::ArrowReturnLeft),
        IpgButtonArrows::ArrowReturnRight => icon_to_string(BootstrapIcon::ArrowReturnRight),
        IpgButtonArrows::ArrowRight => icon_to_string(BootstrapIcon::ArrowRight),
        IpgButtonArrows::ArrowRightCircle => icon_to_string(BootstrapIcon::ArrowRightCircle),
        IpgButtonArrows::ArrowRightCircleFill => icon_to_string(BootstrapIcon::ArrowRightCircleFill),
        IpgButtonArrows::ArrowRightShort => icon_to_string(BootstrapIcon::ArrowRightShort),
        IpgButtonArrows::ArrowRightSquare => icon_to_string(BootstrapIcon::ArrowRightSquare),
        IpgButtonArrows::ArrowRightSquareFill => icon_to_string(BootstrapIcon::ArrowRightSquareFill),
        IpgButtonArrows::ArrowThroughHeart => icon_to_string(BootstrapIcon::ArrowThroughHeart),
        IpgButtonArrows::ArrowThroughHeartFill => icon_to_string(BootstrapIcon::ArrowThroughHeartFill),
        IpgButtonArrows::ArrowUp => icon_to_string(BootstrapIcon::ArrowUp),
        IpgButtonArrows::ArrowUpCircle => icon_to_string(BootstrapIcon::ArrowUpCircle),
        IpgButtonArrows::ArrowUpCircleFill => icon_to_string(BootstrapIcon::ArrowUpCircleFill),
        IpgButtonArrows::ArrowUpLeft => icon_to_string(BootstrapIcon::ArrowUpLeft),
        IpgButtonArrows::ArrowUpLeftCircle => icon_to_string(BootstrapIcon::ArrowUpLeftCircle),
        IpgButtonArrows::ArrowUpLeftCircleFill => icon_to_string(BootstrapIcon::ArrowUpLeftCircleFill),
        IpgButtonArrows::ArrowUpLeftSquare => icon_to_string(BootstrapIcon::ArrowUpLeftSquare),
        IpgButtonArrows::ArrowUpLeftSquareFill => icon_to_string(BootstrapIcon::ArrowUpLeftSquareFill),
        IpgButtonArrows::ArrowUpRight => icon_to_string(BootstrapIcon::ArrowUpRight),
        IpgButtonArrows::ArrowUpRightCircle => icon_to_string(BootstrapIcon::ArrowUpRightCircle),
        IpgButtonArrows::ArrowUpRightCircleFill => icon_to_string(BootstrapIcon::ArrowUpRightCircleFill),
        IpgButtonArrows::ArrowUpRightSquare => icon_to_string(BootstrapIcon::ArrowUpRightSquare),
        IpgButtonArrows::ArrowUpRightSquareFill => icon_to_string(BootstrapIcon::ArrowUpRightSquareFill),
        IpgButtonArrows::ArrowUpShort => icon_to_string(BootstrapIcon::ArrowUpShort),
        IpgButtonArrows::ArrowUpSquare => icon_to_string(BootstrapIcon::ArrowUpSquare),
        IpgButtonArrows::ArrowUpSquareFill => icon_to_string(BootstrapIcon::ArrowUpSquareFill),
        IpgButtonArrows::Arrows => icon_to_string(BootstrapIcon::Arrows),
        IpgButtonArrows::ArrowsAngleContract => icon_to_string(BootstrapIcon::ArrowsAngleContract),
        IpgButtonArrows::ArrowsAngleExpand => icon_to_string(BootstrapIcon::ArrowsAngleExpand),
        IpgButtonArrows::ArrowsCollapse => icon_to_string(BootstrapIcon::ArrowsCollapse),
        IpgButtonArrows::ArrowsCollapseVertical => icon_to_string(BootstrapIcon::ArrowsCollapseVertical),
        IpgButtonArrows::ArrowsExpand => icon_to_string(BootstrapIcon::ArrowsExpand),
        IpgButtonArrows::ArrowsExpandVertical => icon_to_string(BootstrapIcon::ArrowsExpandVertical),
        IpgButtonArrows::ArrowsFullscreen => icon_to_string(BootstrapIcon::ArrowsFullscreen),
        IpgButtonArrows::ArrowsMove => icon_to_string(BootstrapIcon::ArrowsMove),
        IpgButtonArrows::ArrowsVertical => icon_to_string(BootstrapIcon::ArrowsVertical),
    }
}



pub struct ButtonStyleRadius {
    theme: theme::Button,
    radius: f32,
}

impl ButtonStyleRadius {
    pub fn new(theme: theme::Button, radius: f32) -> Self {
        Self { 
            theme,
            radius,
         }
    }
    pub fn radius(&mut self, radius: f32) {
        self.radius = radius
    }
}

impl StyleSheet for ButtonStyleRadius {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> Appearance {
        let mut appearance = style.active(&self.theme);
        appearance.border.radius = self.radius.into();

        appearance
    }
}


pub struct ButtonStyle;
impl button::StyleSheet for ButtonStyle {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            text_color: style.extended_palette().background.base.text,
            border: Border::with_radius([4.0; 4]),
            background: Some(Color::TRANSPARENT.into()),
            ..Default::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let plt = style.extended_palette();

        button::Appearance {
            background: Some(plt.primary.weak.color.into()),
            text_color: plt.primary.weak.text,
            ..self.active(style)
        }
    }
}

