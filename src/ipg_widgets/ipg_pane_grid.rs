#![allow(unused)]
use iced::widget::{container, Container, Column, pane_grid::State, text};
use iced::{Element, Length, Padding};

use crate::app::Message;

#[derive(Debug)]
pub struct IpgPaneGrid {
    pub id: usize,
    pub width: Length,
    pub height: Length,
    pub spacing: f32,
    pub padding: Padding,
    pub show: bool,

    pane_ids: Vec<usize>,
    pane_widget_ids: Vec<(usize, Vec<usize>)>,
}

impl IpgPaneGrid {
    pub fn new(
        id: usize,
        width: Length,
        height: Length,
        spacing: f32,
        padding: Padding,
        show: bool,
    ) -> Self {
        Self {
            id,
            width,
            height,
            spacing,
            padding,
            show,
            pane_ids: vec![],
            pane_widget_ids: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub enum PGMessage {

}

pub fn construct_pane_grid(pngd: &IpgPaneGrid, panes: Vec<Element<'static, Message>>) 
                                                        -> Element<'static, Message> 
{

    let _pane_grid: Vec<Element<'static, Message>> = vec![text("Pane Grid").into()];

    let content = Column::with_children(panes);

    container(content)
        .width(pngd.width)
        .height(pngd.height)
        .padding(pngd.padding)
        .into()

}

pub fn pane_grid_update(_pg: PGMessage) {

}


#[derive(Debug, Clone)]
pub struct IpgPane {
    pub id: usize,
    pub add_direction: String,
    pub ratio: f32,
}

impl IpgPane {
    pub fn new(id: usize, add_direction: String, ratio: f32) -> Self {
        Self {
            id,
            add_direction,
            ratio,
        }
    }
}

pub fn construct_pane(ipg_pane: &IpgPane, content: Vec<Element<'static, Message>>) -> Element<'static, Message> {

    if ipg_pane.add_direction == "first".to_string() {
        let (_panes, _current_pane) = State::new(ipg_pane);
    }
    

    let content = Column::with_children(content)
                                                            .width(Length::Shrink)
                                                            .height(Length::Shrink);
    Container::new(content).into()
}

pub fn pane_update(_pg: PGMessage) {

}

// const PANE_ID_COLOR_UNFOCUSED: Color = Color::from_rgb(
//     0xFF as f32 / 255.0,
//     0xC7 as f32 / 255.0,
//     0xC7 as f32 / 255.0,
// );
// const PANE_ID_COLOR_FOCUSED: Color = Color::from_rgb(
//     0xFF as f32 / 255.0,
//     0x47 as f32 / 255.0,
//     0x47 as f32 / 255.0,
// );

// fn handle_hotkey(key_code: keyboard::KeyCode) -> Option<PGMessage> {
//     use keyboard::KeyCode;
//     use pane_grid::Direction;

//     let direction = match key_code {
//         KeyCode::Up => Some(Direction::Up),
//         KeyCode::Down => Some(Direction::Down),
//         KeyCode::Left => Some(Direction::Left),
//         KeyCode::Right => Some(Direction::Right),
//         _ => None,
//     };

//     match key_code {
//         KeyCode::W => Some(PGMessage::CloseFocused),
//         _ => direction.map(PGMessage::FocusAdjacent),
//     }
// }

// fn view_content(
//     pane: pane_grid::Pane,
//     total_panes: usize,
//     is_pinned: bool,
//     size: Size,
// ) -> Element<'static, PGMessage> {
//     let button = |label, message| {
//         button(
//             text(label)
//                 .width(Length::Fill)
//                 .horizontal_alignment(alignment::Horizontal::Center)
//                 .size(16),
//         )
//         .width(Length::Fill)
//         .padding(8)
//         .on_press(message)
//     };

//     let mut controls = column![
//         button(
//             "Split horizontally",
//             PGMessage::Split(pane_grid::Axis::Horizontal, pane),
//         ),
//         button(
//             "Split vertically",
//             PGMessage::Split(pane_grid::Axis::Vertical, pane),
//         )
//     ]
//     .spacing(5)
//     .max_width(160);

//     if total_panes > 1 && !is_pinned {
//         controls = controls.push(
//             button("Close", PGMessage::Close(pane))
//                 .style(theme::Button::Destructive),
//         );
//     }

//     let content = column![
//         text(format!("{}x{}", size.width, size.height)).size(24),
//         controls,
//     ]
//     .width(Length::Fill)
//     .spacing(10)
//     .align_items(Alignment::Center);

//     Container::new(scrollable(content))
//         .width(Length::Fill)
//         .height(Length::Fill)
//         .padding(5)
//         .center_y()
//         .into()
// }

// fn view_controls(
//     pane: pane_grid::Pane,
//     total_panes: usize,
//     is_pinned: bool,
//     is_maximized: bool,
// ) -> Element<'static, PGMessage> {
//     let mut row = Row::with_children(vec![]).spacing(5);

//     if total_panes > 1 {
//         let toggle: Element<'static, PGMessage> = {
//             let (content, message) = if is_maximized {
//                 ("Restore", PGMessage::Restore)
//             } else {
//                 ("Maximize", PGMessage::Maximize(pane))
//             };
//             button(text(content).size(14))
//                 .style(theme::Button::Secondary)
//                 .padding(3)
//                 .on_press(message)
//                 .into()
//         };

//         row = row.push(toggle);
//     }

//     let mut close = button(text("Close").size(14))
//         .style(theme::Button::Destructive)
//         .padding(3);

//     if total_panes > 1 && !is_pinned {
//         close = close.on_press(PGMessage::Close(pane));
//     }

//     row.push(close).into()
// }


// mod style {
//     use iced::widget::container;
//     use iced::Theme;

//     pub fn title_bar_active(theme: &Theme) -> container::Appearance {
//         let palette = theme.extended_palette();

//         container::Appearance {
//             text_color: Some(palette.background.strong.text),
//             background: Some(palette.background.strong.color.into()),
//             ..Default::default()
//         }
//     }

//     pub fn title_bar_focused(theme: &Theme) -> container::Appearance {
//         let palette = theme.extended_palette();

//         container::Appearance {
//             text_color: Some(palette.primary.strong.text),
//             background: Some(palette.primary.strong.color.into()),
//             ..Default::default()
//         }
//     }

//     pub fn pane_active(theme: &Theme) -> container::Appearance {
//         let palette = theme.extended_palette();

//         container::Appearance {
//             background: Some(palette.background.weak.color.into()),
//             border_width: 2.0,
//             border_color: palette.background.strong.color,
//             ..Default::default()
//         }
//     }

//     pub fn pane_focused(theme: &Theme) -> container::Appearance {
//         let palette = theme.extended_palette();

//         container::Appearance {
//             background: Some(palette.background.weak.color.into()),
//             border_width: 2.0,
//             border_color: palette.primary.strong.color,
//             ..Default::default()
//         }
//     }
// }
