// #![allow(dead_code)]
#![allow(unused_assignments)]

use crate::graphics::colors::{get_color, IpgColor};
use crate::style::styling::IpgStyleStandard;
use crate::{access_callbacks, access_state, app};
use super::helpers::{get_height, get_padding, get_width, 
                    try_extract_f64, try_extract_string, 
                    try_extract_boolean, try_extract_vec_f64};
use super::callbacks::{
    WidgetCallbackIn, WidgetCallbackOut, 
    get_set_widget_callback_data
};

use iced::widget::button::{self, Status, Style};
use pyo3::{pyclass, PyObject, Python};

use iced::widget::{Button, Space, Text};
use iced::{theme, Color, Element, Length, Padding, Theme, Vector };

use crate::graphics::bootstrap::{self, icon_to_char, icon_to_string};


#[derive(Debug, Clone)]
pub struct IpgButton {
    pub id: usize,
    pub show: bool,
    pub user_data: Option<PyObject>,

    pub label: String,
    pub width: Length,
    pub height: Length,
    pub padding: Padding,
    pub clip: bool,
    pub style_standard: Option<IpgStyleStandard>,
    pub style_color: Option<String>,
    pub style_border: Option<String>,
    pub style_shadow: Option<String>,
    pub style_arrow: Option<PyObject>,
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
        clip: bool,
        style_standard: Option<IpgStyleStandard>,
        style_color: Option<String>,
        style_border: Option<String>,
        style_shadow: Option<String>,
        style_arrow: Option<PyObject>,
        ) -> Self {
        Self {
            id,
            show,
            user_data,
            label,
            width,
            height,
            padding,
            clip,
            style_standard,
            style_color,
            style_border,
            style_shadow,
            style_arrow,
        }
    }
}

#[derive(Debug, Clone)]
pub enum BTNMessage {
    OnPress,
}


pub fn construct_button(btn: IpgButton) -> Element<'static, app::Message> {

    if !btn.show {
        return Space::new(Length::Shrink, Length::Shrink).into()
    }

    let mut label = Text::new(btn.label.clone());

    if btn.style_arrow.is_some() {
        let arrow_style = try_extract_button_arrow(btn.style_arrow);
        label = match arrow_style {
            Some(ar) => Text::new(ar).font(iced::Font::with_name("bootstrap-icons")),
            None => panic!("Button: Could not get extract arrow_style")
        };
    }

    let ipg_btn: Element<BTNMessage> = Button::new(label)
                                .height(btn.height)
                                .padding(btn.padding)
                                .width(btn.width)
                                .on_press(BTNMessage::OnPress)
                                .clip(btn.clip)
                                .style(move|theme: &Theme, status| {   
                                    get_styling(theme, status, 
                                        btn.style_standard.clone(),
                                        btn.style_color.clone(), 
                                        btn.style_border.clone(), 
                                        btn.style_shadow.clone(),
                                    )  
                                    })
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
    Height,
    HeightFill,
    Label,
    Padding,
    Show,
    StyleColor,
    StyleBorder,
    StyleShadow,
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
            btn.style_arrow = Some(value);
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
        IpgButtonParams::StyleColor => {
            let val = try_extract_string(value);
            btn.style_color = Some(val);
        },
        IpgButtonParams::StyleBorder => {
            let val = try_extract_string(value);
            btn.style_border = Some(val);
        },
        IpgButtonParams::StyleShadow => {
            let val = try_extract_string(value);
            btn.style_shadow = Some(val);
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


pub fn get_standard_style(theme: &Theme, status: Status, style: Option<IpgStyleStandard>) -> Style {
    match style {
        Some(IpgStyleStandard::Primary) => button::primary(theme, status),
        Some(IpgStyleStandard::Success) => button::success(theme, status),
        Some(IpgStyleStandard::Danger) => button::danger(theme, status),
        Some(IpgStyleStandard::Text) => button::text(theme, status),
        None => button::primary(theme, status),
    }
}

pub fn get_styling(theme: &Theme, status: Status,
                    style_standard: Option<IpgStyleStandard>, 
                    style_color: Option<String>, 
                    style_border: Option<String>, 
                    style_shadow: Option<String>,
                    ) -> button::Style 
{
    if style_standard.is_none() && style_color.is_none() {
        return button::primary(theme, status)
    }
    let state = access_state();

    let border_opt = if style_border.is_some() {
        state.styling_border.get(&style_border.unwrap())
    } else {
        None
    };

    let shadow_opt = if style_shadow.is_some() {
        state.styling_shadow.get(&style_shadow.unwrap())
    } else {
        None
    };

    let mut base_style = button::primary(theme, status);
    
    if border_opt.is_some() {
        let border = border_opt.unwrap();
        base_style.border.radius = border.radius;
        base_style.border.width = border.width;
    }

    if shadow_opt.is_some() {
        let shadow = shadow_opt.unwrap();
        base_style.shadow.offset = Vector{ x: shadow.offset_x, y: shadow.offset_y };
        base_style.shadow.blur_radius = shadow.blur_radius;
    }

    let palette = theme.extended_palette();

    if style_standard.is_some() {
        let style_std = style_standard.unwrap().clone();
        
        // if shadow or border is used, will use the standard color
        let mut shadow_color = palette.primary.weak.color;
        let mut border_color = palette.primary.strong.color;

        let mut style = match style_std {
            IpgStyleStandard::Primary => {
                button::primary(theme, status)
            },
            IpgStyleStandard::Success => {
                shadow_color = palette.success.weak.color;
                border_color = palette.success.strong.color;
                button::success(theme, status)
            },
            IpgStyleStandard::Danger => {
                shadow_color = palette.danger.weak.color;
                border_color = palette.danger.strong.color;
                button::danger(theme, status)
            },
            IpgStyleStandard::Text => {
                button::text(theme, status)
            },
        };

        if border_opt.is_some() {
            style.border.color = border_color;
            style.border.width = base_style.border.width;
            style.border.radius = base_style.border.radius;
        }
        if shadow_opt.is_some() {
            style.shadow.color = shadow_color;
            style.shadow.blur_radius = base_style.shadow.blur_radius;
            style.shadow.offset = base_style.shadow.offset;
        }

        return style
    }

    let color_palette_opt = if style_color.is_some() {
        state.styling_color.get(&style_color.unwrap())
    } else {
        None
    };

    let mut hover_style = button::Style::default();

    if color_palette_opt.is_some() {
        let text = if palette.is_dark {
            Color::WHITE
        } else {
            Color::BLACK
        };

        let mut color_palette = color_palette_opt.unwrap().clone();
        
        if color_palette.base.is_none() {
            color_palette.base = get_color(None, Some(IpgColor::PRIMARY), 1.0, false)
        }

        let background = theme::palette::Background::new(color_palette.base.unwrap(), text);
        base_style.background = Some(iced::Background::Color(background.weak.color));

        if color_palette.text.is_some() {
            base_style.text_color = color_palette.text.unwrap();
        } else {
            base_style.text_color = background.base.text;
        }
        
        if color_palette.border.is_some() {
            base_style.border.color = color_palette.border.unwrap();
        }

        if color_palette.shadow.is_some() {
            base_style.shadow.color = color_palette.shadow.unwrap();
        }

        hover_style = base_style.clone();
        hover_style.background = Some(iced::Background::Color(background.strong.color));
    }
        
    let style = match status {
        Status::Active | Status::Pressed => base_style,
        Status::Hovered => hover_style,
        Status::Disabled => disabled(base_style),
    };
    
    style

}


fn disabled(style: Style) -> Style {
    Style {
        background: style
            .background
            .map(|background| background.scale_alpha(0.5)),
        text_color: style.text_color.scale_alpha(0.5),
        ..style
    }
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


pub fn get_bootstrap_arrow(arrow: IpgButtonArrows) -> String {
    match arrow {
        IpgButtonArrows::ArrowBarLeft => icon_to_string(bootstrap::Bootstrap::ArrowBarLeft),
        IpgButtonArrows::ArrowBarRight => icon_to_string(bootstrap::Bootstrap::ArrowBarRight),
        IpgButtonArrows::ArrowBarUp => icon_to_string(bootstrap::Bootstrap::ArrowBarUp),
        IpgButtonArrows::ArrowClockwise => icon_to_string(bootstrap::Bootstrap::ArrowClockwise),
        IpgButtonArrows::ArrowCounterclockwise => icon_to_string(bootstrap::Bootstrap::ArrowCounterclockwise),
        IpgButtonArrows::ArrowDown => icon_to_string(bootstrap::Bootstrap::ArrowDown),
        IpgButtonArrows::ArrowDownCircle => icon_to_string(bootstrap::Bootstrap::ArrowDownCircle),
        IpgButtonArrows::ArrowDownCircleFill => icon_to_string(bootstrap::Bootstrap::ArrowDownCircleFill),
        IpgButtonArrows::ArrowDownLeft => icon_to_string(bootstrap::Bootstrap::ArrowDownLeft),
        IpgButtonArrows::ArrowDownLeftCircle => icon_to_string(bootstrap::Bootstrap::ArrowDownLeftCircle),
        IpgButtonArrows::ArrowDownLeftCircleFill => icon_to_string(bootstrap::Bootstrap::ArrowDownLeftCircleFill),
        IpgButtonArrows::ArrowDownLeftSquare => icon_to_string(bootstrap::Bootstrap::ArrowDownLeftSquare),
        IpgButtonArrows::ArrowDownLeftSquareFill => icon_to_string(bootstrap::Bootstrap::ArrowDownLeftSquareFill),
        IpgButtonArrows::ArrowDownRight => icon_to_string(bootstrap::Bootstrap::ArrowDownRight),
        IpgButtonArrows::ArrowDownRightCircle => icon_to_string(bootstrap::Bootstrap::ArrowDownRightCircle),
        IpgButtonArrows::ArrowDownRightCircleFill => icon_to_string(bootstrap::Bootstrap::ArrowDownRightCircleFill),
        IpgButtonArrows::ArrowDownRightSquare => icon_to_string(bootstrap::Bootstrap::ArrowDownRightSquare),
        IpgButtonArrows::ArrowDownRightSquareFill => icon_to_string(bootstrap::Bootstrap::ArrowDownRightSquareFill),
        IpgButtonArrows::ArrowDownShort => icon_to_string(bootstrap::Bootstrap::ArrowDownShort),
        IpgButtonArrows::ArrowDownSquare => icon_to_string(bootstrap::Bootstrap::ArrowDownSquare),
        IpgButtonArrows::ArrowDownSquareFill => icon_to_string(bootstrap::Bootstrap::ArrowDownSquareFill),
        IpgButtonArrows::ArrowDownUp => icon_to_string(bootstrap::Bootstrap::ArrowDownUp),
        IpgButtonArrows::ArrowLeft => icon_to_string(bootstrap::Bootstrap::ArrowLeft),
        IpgButtonArrows::ArrowLeftCircle => icon_to_string(bootstrap::Bootstrap::ArrowLeftCircle),
        IpgButtonArrows::ArrowLeftCircleFill => icon_to_string(bootstrap::Bootstrap::ArrowLeftCircleFill),
        IpgButtonArrows::ArrowLeftRight => icon_to_string(bootstrap::Bootstrap::ArrowLeftRight),
        IpgButtonArrows::ArrowLeftShort => icon_to_string(bootstrap::Bootstrap::ArrowLeftShort),
        IpgButtonArrows::ArrowLeftSquare => icon_to_string(bootstrap::Bootstrap::ArrowLeftSquare),
        IpgButtonArrows::ArrowLeftSquareFill => icon_to_string(bootstrap::Bootstrap::ArrowLeftSquareFill),
        IpgButtonArrows::ArrowNinezerodegDown => icon_to_string(bootstrap::Bootstrap::ArrowNinezerodegDown),
        IpgButtonArrows::ArrowNinezerodegLeft => icon_to_string(bootstrap::Bootstrap::ArrowNinezerodegLeft),
        IpgButtonArrows::ArrowNinezerodegRight => icon_to_string(bootstrap::Bootstrap::ArrowNinezerodegRight),
        IpgButtonArrows::ArrowNinezerodegUp => icon_to_string(bootstrap::Bootstrap::ArrowNinezerodegUp),
        IpgButtonArrows::ArrowRepeat => icon_to_string(bootstrap::Bootstrap::ArrowRepeat),
        IpgButtonArrows::ArrowReturnLeft => icon_to_string(bootstrap::Bootstrap::ArrowReturnLeft),
        IpgButtonArrows::ArrowReturnRight => icon_to_string(bootstrap::Bootstrap::ArrowReturnRight),
        IpgButtonArrows::ArrowRight => icon_to_string(bootstrap::Bootstrap::ArrowRight),
        IpgButtonArrows::ArrowRightCircle => icon_to_string(bootstrap::Bootstrap::ArrowRightCircle),
        IpgButtonArrows::ArrowRightCircleFill => icon_to_string(bootstrap::Bootstrap::ArrowRightCircleFill),
        IpgButtonArrows::ArrowRightShort => icon_to_string(bootstrap::Bootstrap::ArrowRightShort),
        IpgButtonArrows::ArrowRightSquare => icon_to_string(bootstrap::Bootstrap::ArrowRightSquare),
        IpgButtonArrows::ArrowRightSquareFill => icon_to_string(bootstrap::Bootstrap::ArrowRightSquareFill),
        IpgButtonArrows::ArrowThroughHeart => icon_to_string(bootstrap::Bootstrap::ArrowThroughHeart),
        IpgButtonArrows::ArrowThroughHeartFill => icon_to_string(bootstrap::Bootstrap::ArrowThroughHeartFill),
        IpgButtonArrows::ArrowUp => icon_to_string(bootstrap::Bootstrap::ArrowUp),
        IpgButtonArrows::ArrowUpCircle => icon_to_string(bootstrap::Bootstrap::ArrowUpCircle),
        IpgButtonArrows::ArrowUpCircleFill => icon_to_string(bootstrap::Bootstrap::ArrowUpCircleFill),
        IpgButtonArrows::ArrowUpLeft => icon_to_string(bootstrap::Bootstrap::ArrowUpLeft),
        IpgButtonArrows::ArrowUpLeftCircle => icon_to_string(bootstrap::Bootstrap::ArrowUpLeftCircle),
        IpgButtonArrows::ArrowUpLeftCircleFill => icon_to_string(bootstrap::Bootstrap::ArrowUpLeftCircleFill),
        IpgButtonArrows::ArrowUpLeftSquare => icon_to_string(bootstrap::Bootstrap::ArrowUpLeftSquare),
        IpgButtonArrows::ArrowUpLeftSquareFill => icon_to_string(bootstrap::Bootstrap::ArrowUpLeftSquareFill),
        IpgButtonArrows::ArrowUpRight => icon_to_string(bootstrap::Bootstrap::ArrowUpRight),
        IpgButtonArrows::ArrowUpRightCircle => icon_to_string(bootstrap::Bootstrap::ArrowUpRightCircle),
        IpgButtonArrows::ArrowUpRightCircleFill => icon_to_string(bootstrap::Bootstrap::ArrowUpRightCircleFill),
        IpgButtonArrows::ArrowUpRightSquare => icon_to_string(bootstrap::Bootstrap::ArrowUpRightSquare),
        IpgButtonArrows::ArrowUpRightSquareFill => icon_to_string(bootstrap::Bootstrap::ArrowUpRightSquareFill),
        IpgButtonArrows::ArrowUpShort => icon_to_string(bootstrap::Bootstrap::ArrowUpShort),
        IpgButtonArrows::ArrowUpSquare => icon_to_string(bootstrap::Bootstrap::ArrowUpSquare),
        IpgButtonArrows::ArrowUpSquareFill => icon_to_string(bootstrap::Bootstrap::ArrowUpSquareFill),
        IpgButtonArrows::Arrows => icon_to_string(bootstrap::Bootstrap::Arrows),
        IpgButtonArrows::ArrowsAngleContract => icon_to_string(bootstrap::Bootstrap::ArrowsAngleContract),
        IpgButtonArrows::ArrowsAngleExpand => icon_to_string(bootstrap::Bootstrap::ArrowsAngleExpand),
        IpgButtonArrows::ArrowsCollapse => icon_to_string(bootstrap::Bootstrap::ArrowsCollapse),
        IpgButtonArrows::ArrowsCollapseVertical => icon_to_string(bootstrap::Bootstrap::ArrowsCollapseVertical),
        IpgButtonArrows::ArrowsExpand => icon_to_string(bootstrap::Bootstrap::ArrowsExpand),
        IpgButtonArrows::ArrowsExpandVertical => icon_to_string(bootstrap::Bootstrap::ArrowsExpandVertical),
        IpgButtonArrows::ArrowsFullscreen => icon_to_string(bootstrap::Bootstrap::ArrowsFullscreen),
        IpgButtonArrows::ArrowsMove => icon_to_string(bootstrap::Bootstrap::ArrowsMove),
        IpgButtonArrows::ArrowsVertical => icon_to_string(bootstrap::Bootstrap::ArrowsVertical),
    }
}

pub fn get_bootstrap_arrow_char(arrow: IpgButtonArrows) -> char {
    match arrow {
        IpgButtonArrows::ArrowBarLeft => icon_to_char(bootstrap::Bootstrap::ArrowBarLeft),
        IpgButtonArrows::ArrowBarRight => icon_to_char(bootstrap::Bootstrap::ArrowBarRight),
        IpgButtonArrows::ArrowBarUp => icon_to_char(bootstrap::Bootstrap::ArrowBarUp),
        IpgButtonArrows::ArrowClockwise => icon_to_char(bootstrap::Bootstrap::ArrowClockwise),
        IpgButtonArrows::ArrowCounterclockwise => icon_to_char(bootstrap::Bootstrap::ArrowCounterclockwise),
        IpgButtonArrows::ArrowDown => icon_to_char(bootstrap::Bootstrap::ArrowDown),
        IpgButtonArrows::ArrowDownCircle => icon_to_char(bootstrap::Bootstrap::ArrowDownCircle),
        IpgButtonArrows::ArrowDownCircleFill => icon_to_char(bootstrap::Bootstrap::ArrowDownCircleFill),
        IpgButtonArrows::ArrowDownLeft => icon_to_char(bootstrap::Bootstrap::ArrowDownLeft),
        IpgButtonArrows::ArrowDownLeftCircle => icon_to_char(bootstrap::Bootstrap::ArrowDownLeftCircle),
        IpgButtonArrows::ArrowDownLeftCircleFill => icon_to_char(bootstrap::Bootstrap::ArrowDownLeftCircleFill),
        IpgButtonArrows::ArrowDownLeftSquare => icon_to_char(bootstrap::Bootstrap::ArrowDownLeftSquare),
        IpgButtonArrows::ArrowDownLeftSquareFill => icon_to_char(bootstrap::Bootstrap::ArrowDownLeftSquareFill),
        IpgButtonArrows::ArrowDownRight => icon_to_char(bootstrap::Bootstrap::ArrowDownRight),
        IpgButtonArrows::ArrowDownRightCircle => icon_to_char(bootstrap::Bootstrap::ArrowDownRightCircle),
        IpgButtonArrows::ArrowDownRightCircleFill => icon_to_char(bootstrap::Bootstrap::ArrowDownRightCircleFill),
        IpgButtonArrows::ArrowDownRightSquare => icon_to_char(bootstrap::Bootstrap::ArrowDownRightSquare),
        IpgButtonArrows::ArrowDownRightSquareFill => icon_to_char(bootstrap::Bootstrap::ArrowDownRightSquareFill),
        IpgButtonArrows::ArrowDownShort => icon_to_char(bootstrap::Bootstrap::ArrowDownShort),
        IpgButtonArrows::ArrowDownSquare => icon_to_char(bootstrap::Bootstrap::ArrowDownSquare),
        IpgButtonArrows::ArrowDownSquareFill => icon_to_char(bootstrap::Bootstrap::ArrowDownSquareFill),
        IpgButtonArrows::ArrowDownUp => icon_to_char(bootstrap::Bootstrap::ArrowDownUp),
        IpgButtonArrows::ArrowLeft => icon_to_char(bootstrap::Bootstrap::ArrowLeft),
        IpgButtonArrows::ArrowLeftCircle => icon_to_char(bootstrap::Bootstrap::ArrowLeftCircle),
        IpgButtonArrows::ArrowLeftCircleFill => icon_to_char(bootstrap::Bootstrap::ArrowLeftCircleFill),
        IpgButtonArrows::ArrowLeftRight => icon_to_char(bootstrap::Bootstrap::ArrowLeftRight),
        IpgButtonArrows::ArrowLeftShort => icon_to_char(bootstrap::Bootstrap::ArrowLeftShort),
        IpgButtonArrows::ArrowLeftSquare => icon_to_char(bootstrap::Bootstrap::ArrowLeftSquare),
        IpgButtonArrows::ArrowLeftSquareFill => icon_to_char(bootstrap::Bootstrap::ArrowLeftSquareFill),
        IpgButtonArrows::ArrowNinezerodegDown => icon_to_char(bootstrap::Bootstrap::ArrowNinezerodegDown),
        IpgButtonArrows::ArrowNinezerodegLeft => icon_to_char(bootstrap::Bootstrap::ArrowNinezerodegLeft),
        IpgButtonArrows::ArrowNinezerodegRight => icon_to_char(bootstrap::Bootstrap::ArrowNinezerodegRight),
        IpgButtonArrows::ArrowNinezerodegUp => icon_to_char(bootstrap::Bootstrap::ArrowNinezerodegUp),
        IpgButtonArrows::ArrowRepeat => icon_to_char(bootstrap::Bootstrap::ArrowRepeat),
        IpgButtonArrows::ArrowReturnLeft => icon_to_char(bootstrap::Bootstrap::ArrowReturnLeft),
        IpgButtonArrows::ArrowReturnRight => icon_to_char(bootstrap::Bootstrap::ArrowReturnRight),
        IpgButtonArrows::ArrowRight => icon_to_char(bootstrap::Bootstrap::ArrowRight),
        IpgButtonArrows::ArrowRightCircle => icon_to_char(bootstrap::Bootstrap::ArrowRightCircle),
        IpgButtonArrows::ArrowRightCircleFill => icon_to_char(bootstrap::Bootstrap::ArrowRightCircleFill),
        IpgButtonArrows::ArrowRightShort => icon_to_char(bootstrap::Bootstrap::ArrowRightShort),
        IpgButtonArrows::ArrowRightSquare => icon_to_char(bootstrap::Bootstrap::ArrowRightSquare),
        IpgButtonArrows::ArrowRightSquareFill => icon_to_char(bootstrap::Bootstrap::ArrowRightSquareFill),
        IpgButtonArrows::ArrowThroughHeart => icon_to_char(bootstrap::Bootstrap::ArrowThroughHeart),
        IpgButtonArrows::ArrowThroughHeartFill => icon_to_char(bootstrap::Bootstrap::ArrowThroughHeartFill),
        IpgButtonArrows::ArrowUp => icon_to_char(bootstrap::Bootstrap::ArrowUp),
        IpgButtonArrows::ArrowUpCircle => icon_to_char(bootstrap::Bootstrap::ArrowUpCircle),
        IpgButtonArrows::ArrowUpCircleFill => icon_to_char(bootstrap::Bootstrap::ArrowUpCircleFill),
        IpgButtonArrows::ArrowUpLeft => icon_to_char(bootstrap::Bootstrap::ArrowUpLeft),
        IpgButtonArrows::ArrowUpLeftCircle => icon_to_char(bootstrap::Bootstrap::ArrowUpLeftCircle),
        IpgButtonArrows::ArrowUpLeftCircleFill => icon_to_char(bootstrap::Bootstrap::ArrowUpLeftCircleFill),
        IpgButtonArrows::ArrowUpLeftSquare => icon_to_char(bootstrap::Bootstrap::ArrowUpLeftSquare),
        IpgButtonArrows::ArrowUpLeftSquareFill => icon_to_char(bootstrap::Bootstrap::ArrowUpLeftSquareFill),
        IpgButtonArrows::ArrowUpRight => icon_to_char(bootstrap::Bootstrap::ArrowUpRight),
        IpgButtonArrows::ArrowUpRightCircle => icon_to_char(bootstrap::Bootstrap::ArrowUpRightCircle),
        IpgButtonArrows::ArrowUpRightCircleFill => icon_to_char(bootstrap::Bootstrap::ArrowUpRightCircleFill),
        IpgButtonArrows::ArrowUpRightSquare => icon_to_char(bootstrap::Bootstrap::ArrowUpRightSquare),
        IpgButtonArrows::ArrowUpRightSquareFill => icon_to_char(bootstrap::Bootstrap::ArrowUpRightSquareFill),
        IpgButtonArrows::ArrowUpShort => icon_to_char(bootstrap::Bootstrap::ArrowUpShort),
        IpgButtonArrows::ArrowUpSquare => icon_to_char(bootstrap::Bootstrap::ArrowUpSquare),
        IpgButtonArrows::ArrowUpSquareFill => icon_to_char(bootstrap::Bootstrap::ArrowUpSquareFill),
        IpgButtonArrows::Arrows => icon_to_char(bootstrap::Bootstrap::Arrows),
        IpgButtonArrows::ArrowsAngleContract => icon_to_char(bootstrap::Bootstrap::ArrowsAngleContract),
        IpgButtonArrows::ArrowsAngleExpand => icon_to_char(bootstrap::Bootstrap::ArrowsAngleExpand),
        IpgButtonArrows::ArrowsCollapse => icon_to_char(bootstrap::Bootstrap::ArrowsCollapse),
        IpgButtonArrows::ArrowsCollapseVertical => icon_to_char(bootstrap::Bootstrap::ArrowsCollapseVertical),
        IpgButtonArrows::ArrowsExpand => icon_to_char(bootstrap::Bootstrap::ArrowsExpand),
        IpgButtonArrows::ArrowsExpandVertical => icon_to_char(bootstrap::Bootstrap::ArrowsExpandVertical),
        IpgButtonArrows::ArrowsFullscreen => icon_to_char(bootstrap::Bootstrap::ArrowsFullscreen),
        IpgButtonArrows::ArrowsMove => icon_to_char(bootstrap::Bootstrap::ArrowsMove),
        IpgButtonArrows::ArrowsVertical => icon_to_char(bootstrap::Bootstrap::ArrowsVertical),
    }
}
