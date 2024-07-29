
use iced::Length;
use iced::widget::{
    self, button, center, column, container, mouse_area,
    opaque, stack, text,
};
use iced::{Color, Element, Task};


pub fn main() -> iced::Result {
    iced::application("Modal - Iced", App::update, App::view)
        .run()
}

#[derive(Default)]
struct App {
    show_modal: bool,
}

#[derive(Debug, Clone)]
enum Message {
    ShowModal,
    HideModal,
}

impl App {
    
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ShowModal => {
                self.show_modal = true;
                widget::focus_next()
            }
            Message::HideModal => {
                self.hide_modal();
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let base = container(
            button(text("Show Modal")).on_press(Message::ShowModal),
        )
        .width(Length::Fixed(300.0))
        .height(Length::Fixed(300.0))
        .padding(10);

        if self.show_modal {
            let content = container(
                column![
                    text("Sign Up").size(24),
                ]
                .spacing(20),
            )
            .width(100)
            .padding(10)
            .style(container::rounded_box);

            modal(base, content, Message::HideModal)
        } else {
            base.into()
        }
    }
}

impl App {
    fn hide_modal(&mut self) {
        self.show_modal = false;
    }
}


fn modal<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
    on_blur: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    stack![
        base.into(),
        mouse_area(center(opaque(content)).style(|_theme| {
            container::Style {
                background: Some(
                    Color {
                        a: 0.8,
                        ..Color::BLACK
                    }
                    .into(),
                ),
                ..container::Style::default()
            }
        }))
        .on_press(on_blur)
    ]
    .into()
}

