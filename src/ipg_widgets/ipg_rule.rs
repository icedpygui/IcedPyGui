use iced::{Element, Length};
use iced::widget::{Container, Rule};
use crate::app;


#[derive(Debug, Clone)]
pub struct IpgRule {
    pub id: usize,
    pub width: Length,
    pub height: Length,
    pub rule_type: String,
}

impl IpgRule {
    pub fn new(id: usize, 
                width: Length, 
                height: Length, 
                rule_type: String) 
                -> Self {
        Self {
            id,
            width,
            height,
            rule_type,
        }
    }
}

// Looks reversed but not.  The only controllale parameter for horizontal
// is the thickness of the line which is height.  The opposite for vertical.
// To control the other dimension, need to put into a container.
pub fn construct_rule(rule: IpgRule) -> Element<'static, app::Message> {
    if rule.rule_type == "h".to_string() {
        return construct_horizontal(rule)
    } else {
        return construct_rule_vertical(rule)
    }

}

// The width or height parameters seems to have no effect so set to 0.
fn construct_horizontal(rule: IpgRule) -> Element<'static, app::Message>{

    let rule_h: Element<app::Message> = Rule::horizontal(5).into();

    Container::new(rule_h).width(rule.width).into()

}

fn construct_rule_vertical(rule: IpgRule) -> Element<'static, app::Message> {

    let rule_v: Element<app::Message> = Rule::vertical(5).into();

    Container::new(rule_v).height(rule.height).into()

}
