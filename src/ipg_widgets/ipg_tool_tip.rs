//! ipg_tool_tip
use iced::Element;
use iced::widget::{Column, text, Tooltip};
use iced::widget::tooltip::Position;
use crate::app::Message;


#[derive(Debug, Clone)]
pub struct IpgToolTip {
    pub id: usize,
    pub position: String,
    pub text_to_display: String,
    pub gap: u16,
    pub padding: f32,
    pub snap_within_viewport: bool,
    pub style: String,
}

impl IpgToolTip {
    pub fn new( 
            id: usize,
            position: String,
            text_to_display: String,
            gap: u16,
            padding: f32,
            snap_within_viewport: bool,
            style: String,
        ) -> Self {
        Self {
            id,
            position,
            text_to_display,
            gap,
            padding,
            snap_within_viewport,
            style,
        }
    }
}

pub fn construct_tool_tip<'a>(tool: &IpgToolTip, content: Vec<Element<'a, Message>>) -> Element<'a, Message> {
        // TODO: tooltip work needed
        // let style = match tool.style.as_str() {
        //     "box" => theme::Container::Box,
        //     "transparent" => theme::Container::Transparent,
        //     _ => panic!("ToolTip must have a style of either box or transparent")
        // };

        let position: Position = match tool.position.as_str() {
                            "followcursor" => Position::FollowCursor,
                            "top"    => Position::Top,
                            "bottom" => Position::Bottom,
                            "left"   => Position::Left,
                            "right"  => Position::Right,
                            _ => panic!("Tooltip position must be eight: followcursor, top, botton, left, or right")
        };
        
        let content: Element<'a, Message> = Column::with_children(content).into();

        Tooltip::new(
                content,
                text(tool.text_to_display.clone()),
                position,
                )
                .gap(tool.gap)
                .padding(tool.padding)
                .snap_within_viewport(tool.snap_within_viewport)
                // .style(style)
                .into()

}