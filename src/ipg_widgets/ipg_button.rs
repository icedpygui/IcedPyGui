#![allow(dead_code)]

use crate::style::styling::lighten;
use crate::{access_callbacks, access_state, app};
use super::helpers::{get_height, get_padding, get_width, 
                    try_extract_f64, try_extract_string, 
                    try_extract_boolean, try_extract_vec_f64};
use super::callbacks::{
    WidgetCallbackIn, WidgetCallbackOut, 
    get_set_widget_callback_data
};

use iced::border::Radius;
use iced::theme::palette;
use iced::widget::button::{self, Status, Style};
use pyo3::{pyclass, PyObject, Python};

use iced::widget::{Button, Space, Text};
use iced::{Background, Border, Color, Element, Length, Padding, Theme };

use crate::graphics::bootstrap::{self, icon_to_string};


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
    pub corner_radius: f32,
    pub style: Option<PyObject>,
    pub style_custom: bool,
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
        clip: bool,
        corner_radius: f32,
        style: Option<PyObject>,
        style_custom: bool,
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
            clip,
            corner_radius,
            style,
            style_custom,
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
            Some(ar) => Text::new(ar).font(iced::Font::with_name("bootstrap-icons")),
            None => panic!("Button: Could not get extract arrow_style")
        };
    }

    let radius = btn.corner_radius.clone();

    let ipg_btn: Element<BTNMessage> = Button::new(label)
                                .height(btn.height)
                                .padding(btn.padding)
                                .width(btn.width)
                                .on_press(BTNMessage::OnPress)
                                .clip(btn.clip)
                                .style(move|theme: &Theme, status| {
                                    if btn.style_custom {
                                        get_custom_styling(theme, status, btn.id)
                                    } else {
                                        get_button_style(btn.style.clone(), theme, status, radius)
                                    }})
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


/// A primary button; denoting a main action.
pub fn theme_primary(theme: &Theme, status: Status, radius: f32) -> Style {
    let palette = theme.extended_palette();
    let base = styled(palette.primary.strong, radius);

    match status {
        Status::Active | Status::Pressed => base,
        Status::Hovered => Style {
            background: Some(Background::Color(palette.primary.base.color)),
            ..base
        },
        Status::Disabled => disabled(base),
    }
}

/// A secondary button; denoting a complementary action.
pub fn theme_secondary(theme: &Theme, status: Status, radius: f32) -> Style {
    let palette = theme.extended_palette();
    let base = styled(palette.secondary.base, radius);

    match status {
        Status::Active | Status::Pressed => base,
        Status::Hovered => Style {
            background: Some(Background::Color(palette.secondary.strong.color)),
            ..base
        },
        Status::Disabled => disabled(base),
    }
}

/// A success button; denoting a good outcome.
pub fn theme_success(theme: &Theme, status: Status, radius: f32) -> Style {
    let palette = theme.extended_palette();
    let base = styled(palette.success.base, radius);

    match status {
        Status::Active | Status::Pressed => base,
        Status::Hovered => Style {
            background: Some(Background::Color(palette.success.strong.color)),
            ..base
        },
        Status::Disabled => disabled(base),
    }
}

/// A danger button; denoting a destructive action.
pub fn theme_danger(theme: &Theme, status: Status, radius: f32) -> Style {
    let palette = theme.extended_palette();
    let base = styled(palette.danger.base, radius);

    match status {
        Status::Active | Status::Pressed => base,
        Status::Hovered => Style {
            background: Some(Background::Color(palette.danger.strong.color)),
            ..base
        },
        Status::Disabled => disabled(base),
    }
}

/// A text button; useful for links.
pub fn theme_text(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();

    let base = Style {
        text_color: palette.background.base.text,
        ..Style::default()
    };

    match status {
        Status::Active | Status::Pressed => base,
        Status::Hovered => Style {
            text_color: palette.background.base.text.scale_alpha(0.8),
            ..base
        },
        Status::Disabled => disabled(base),
    }
}

fn styled(pair: palette::Pair, radius: f32) -> Style {
    Style {
        background: Some(Background::Color(pair.color)),
        text_color: pair.text,
        border: Border::rounded(radius),
        ..Style::default()
    }
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

pub fn get_button_style(style_opt: Option<PyObject>, 
                                theme: &Theme, 
                                status: Status, 
                                radius: f32) -> Style {

    let style_obj = match style_opt {
        Some(st) => st,
        None => return theme_primary(theme, status, radius),
    };

    let ipg_btn_style = try_extract_button_style(style_obj);

    match ipg_btn_style {
        IpgButtonStyles::Primary => theme_primary(theme, status, radius),
        IpgButtonStyles::Secondary => theme_secondary(theme, status, radius),
        IpgButtonStyles::Positive => theme_success(theme, status, radius),
        IpgButtonStyles::Destructive => theme_danger(theme, status, radius),
        IpgButtonStyles::Text => theme_text(theme, status),
    }
}

fn get_custom_styling(_theme: &Theme, status: Status, id: usize) -> button::Style {
    
    let state = access_state();

    let background_opt = state.styling_background.get(&id);
    let border_opt = state.styling_border.get(&id);
    let shadow_opt = state.styling_shadow.get(&id);
    let text_color_opt = state.styling_text_color.get(&id);
    let bg_color_opt = state.styling_background_color.get(&id);

    let background = match background_opt {
        Some(bg) => *bg,
        None => Background::Color(Color::TRANSPARENT),
    };

    let bg_color = match bg_color_opt {
        Some(bg) => *bg,
        None => Color::TRANSPARENT,
    };

    let border = match border_opt {
        Some(bd) => *bd,
        None => Border{color: Color::TRANSPARENT, radius: Radius::from([5.0; 4]), width: 1.0},
    };

    let shadow = match shadow_opt {
        Some(sh) => *sh,
        None => Default::default(),
    };

    let text_color: Color = match text_color_opt {
        Some(tc) => *tc,
        None => Default::default(),
    };

    let base_style = button::Style {
            background: Some(background),
            border,
            shadow,
            text_color,
            };

    match status {
        Status::Active | Status::Pressed => base_style,
        Status::Hovered => Style {
            background: Some(Background::Color(lighten(bg_color, 0.05))),
            ..base_style
        },
        Status::Disabled => disabled(base_style),
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
