use super::ipg_widget_traits::IpgWidgets;
use crate::app;
use crate::access_state;

use iced::{Length, Element};
use iced::widget::{Column, Text};

#[derive(Clone)]
pub struct IpgCard {
    pub id: usize,
    pub show: bool,
    pub user_data_str: Vec<String>,
    pub user_data_flt: Vec<f64>,
    pub user_data_int: Vec<i64>,
    
    pub width: Length,
    pub height: Length,
    pub max_width: f32,
    pub max_height: f32,
    pub padding_head: f32,
    pub padding_body: f32,
    pub padding_foot: f32,
    pub close_size: f32,
    pub head: String,
    pub body: String,
    pub foot: Option<String>,
}

impl IpgCard {
    pub fn new( 
        id: usize,
        show: bool,
        user_data_str: Vec<String>,
        user_data_flt: Vec<f64>,
        user_data_int: Vec<i64>,
        width: Length,
        height: Length,
        max_width: f32,
        max_height: f32,
        padding_head: f32,
        padding_body: f32,
        padding_foot: f32,
        close_size: f32,
        head: String,
        body: String,
        foot: Option<String>,
        ) -> Self {
        Self {
            id,
            show,
            user_data_str,
            user_data_flt,
            user_data_int,
            width,
            height,
            max_width,
            max_height,
            padding_head,
            padding_body,
            padding_foot,
            close_size,
            head,
            body,
            foot,
        }
    }
}

#[derive(Debug, Clone)]
pub enum CardMessage {
    OnClose(usize),
}

pub fn construct_card(crd: &IpgCard) -> Element<'static, app::Message> {

    let card: Element<'static, CardMessage> = Card::new(Text::new(crd.head.clone()),
                                                        Column::new().push(Text::new(crd.body.clone())))
                                                .width(crd.width)
                                                .height(crd.height)
                                                .max_width(crd.max_width)
                                                .max_height(crd.max_height)
                                                .padding_head(crd.padding_head)
                                                .padding_body(crd.padding_body)
                                                .padding_foot(crd.padding_foot)
                                                .close_size(crd.close_size)
                                                .on_close(CardMessage::OnClose(crd.id.clone()))
                                                .into();

    card.map(app::Message::Card)
}

pub fn card_update(message: CardMessage) {
    match message {
        CardMessage::OnClose(id) => {
            let user_data = get_card_user_data(id);
            app::process_callback(id.clone(), "None".to_string().clone(), 
                                    user_data.0, 
                                    user_data.1, 
                                    user_data.2, 
                                    "button".to_string());
        }
    }
}

fn get_card_user_data(id: usize) -> (Vec<String>, Vec<f64>, Vec<i64>) {
    let state = access_state();
    
    for widget_type in state.widgets.iter() {
        match widget_type {
            IpgWidgets::IpgButton(_) => (),
            IpgWidgets::IpgCard(crd) => {
                if crd.id == id {
                    return (crd.user_data_str.clone(),
                            crd.user_data_flt.clone(),
                            crd.user_data_int.clone())
                }
            }
            IpgWidgets::IpgCheckBox(_) => (),
            IpgWidgets::IpgDatePicker(_) => (),
            IpgWidgets::IpgMenuBar(_) => (),
            IpgWidgets::IpgMenuItem(_) => (),
            IpgWidgets::IpgPickList(_) => (),
            IpgWidgets::IpgProgressBar(_) => (),
            IpgWidgets::IpgRadio(_) => (),
            IpgWidgets::IpgSelectableText(_) => (),
            IpgWidgets::IpgSlider(_) => (),
            IpgWidgets::IpgTable(_) => (),
            IpgWidgets::IpgText(_) => (),
            IpgWidgets::IpgTextInput(_) => (),
        }
        
    }
    panic!("Card widget with id {} not found", id)

}



use iced::{Alignment, BorderRadius, Color, Event, event, Padding, Point, Rectangle, Size, touch};
use iced::advanced::{
        layout::{Limits, Node},
        mouse::{self, Cursor},
        renderer, 
        widget::{Operation, Tree},
        Clipboard, Layout,
        Shell, Widget
    };

use iced::alignment::{Horizontal, Vertical};
use iced::advanced::text::LineHeight;

use iced_aw::graphics::icons::{Icon, ICON_FONT};

const DEFAULT_PADDING: f32 = 10.0;

#[allow(missing_debug_implementations)]
pub struct Card<'a, Message, Renderer = iced::Renderer>
where
    Renderer: iced::advanced::Renderer,
    Renderer::Theme: StyleSheet,
{
    /// The width of the [`Card`](Card).
    width: Length,
    /// The height of the [`Card`](Card).
    height: Length,
    /// The maximum width of the [`Card`](Card).
    max_width: f32,
    /// The maximum height of the [`Card`](Card).
    max_height: f32,
    /// The padding of the head of the [`Card`](Card).
    padding_head: f32,
    /// The padding of the body of the [`Card`](Card).
    padding_body: f32,
    /// The padding of the foot of the [`Card`](Card).
    padding_foot: f32,
    /// The optional size of the close icon of the [`Card`](Card).
    close_size: Option<f32>,
    /// The optional message that is send if the close icon of the [`Card`](Card) is pressed.
    on_close: Option<Message>,
    /// The head [`Element`] of the [`Card`](Card).
    head: Element<'a, Message, Renderer>,
    /// The body [`Element`] of the [`Card`](Card).
    body: Element<'a, Message, Renderer>,
    /// The optional foot [`Element`] of the [`Card`](Card).
    foot: Option<Element<'a, Message, Renderer>>,
    /// The style of the [`Card`](Card).
    style: <Renderer::Theme as StyleSheet>::Style,
}

impl<'a, Message, Renderer> Card<'a, Message, Renderer>
where
    Renderer: iced::advanced::text::Renderer,
    Renderer::Theme: StyleSheet,
{
    pub fn new<H, B>(head: H, body: B) -> Self
    where
        H: Into<Element<'a, Message, Renderer>>,
        B: Into<Element<'a, Message, Renderer>>,
    {
        Card {
            width: Length::Fill,
            height: Length::Shrink,
            max_width: 4_294_967_295.0,
            max_height: 4_294_967_295.0,
            padding_head: DEFAULT_PADDING,
            padding_body: DEFAULT_PADDING,
            padding_foot: DEFAULT_PADDING,
            close_size: None,
            on_close: None,
            head: head.into(),
            body: body.into(),
            foot: None,
            style: <Renderer::Theme as StyleSheet>::Style::default(),
        }
    }

    /// Sets the [`Element`] of the foot of the [`Card`](Card).
    #[must_use]
    pub fn foot<F>(mut self, foot: F) -> Self
    where
        F: Into<Element<'a, Message, Renderer>>,
    {
        self.foot = Some(foot.into());
        self
    }

    /// Sets the width of the [`Card`](Card).
    #[must_use]
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`Card`](Card).
    #[must_use]
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the maximum width of the [`Card`](Card).
    #[must_use]
    pub fn max_width(mut self, width: f32) -> Self {
        self.max_width = width;
        self
    }

    /// Sets the maximum height of the [`Card`](Card).
    #[must_use]
    pub fn max_height(mut self, height: f32) -> Self {
        self.max_height = height;
        self
    }

    /// Sets the padding of the [`Card`](Card).
    ///
    /// This will set the padding of the head, body and foot to the
    /// same value.
    #[must_use]
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding_head = padding;
        self.padding_body = padding;
        self.padding_foot = padding;
        self
    }

    /// Sets the padding of the head of the [`Card`](Card).
    #[must_use]
    pub fn padding_head(mut self, padding: f32) -> Self {
        self.padding_head = padding;
        self
    }

    /// Sets the padding of the body of the [`Card`](Card).
    #[must_use]
    pub fn padding_body(mut self, padding: f32) -> Self {
        self.padding_body = padding;
        self
    }

    /// Sets the padding of the foot of the [`Card`](Card).
    #[must_use]
    pub fn padding_foot(mut self, padding: f32) -> Self {
        self.padding_foot = padding;
        self
    }

    /// Sets the size of the close icon of the [`Card`](Card).
    #[must_use]
    pub fn close_size(mut self, size: f32) -> Self {
        self.close_size = Some(size);
        self
    }

    /// Sets the message that will be produced when the close icon of the
    /// [`Card`](Card) is pressed.
    ///
    /// Setting this enables the drawing of a close icon on the [`Card`](Card).
    #[must_use]
    pub fn on_close(mut self, msg: Message) -> Self {
        self.on_close = Some(msg);
        self
    }

    /// Sets the style of the [`Card`](Card).
    #[must_use]
    pub fn style(mut self, style: <Renderer::Theme as StyleSheet>::Style) -> Self {
        self.style = style;
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for Card<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + iced::advanced::Renderer + iced::advanced::text::Renderer<Font = iced::Font>,
    Renderer::Theme: StyleSheet,
{
    fn children(&self) -> Vec<Tree> {
        self.foot.as_ref().map_or_else(
            || vec![Tree::new(&self.head), Tree::new(&self.body)],
            |foot| {
                vec![
                    Tree::new(&self.head),
                    Tree::new(&self.body),
                    Tree::new(foot),
                ]
            },
        )
    }

    fn diff(&self, tree: &mut Tree) {
        if let Some(foot) = self.foot.as_ref() {
            tree.diff_children(&[&self.head, &self.body, foot]);
        } else {
            tree.diff_children(&[&self.head, &self.body]);
        }
    }

    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
        let limits = limits.max_width(self.max_width).max_height(self.max_height);

        let head_node = head_node(
            renderer,
            &limits,
            &self.head,
            self.padding_head,
            self.width,
            self.on_close.is_some(),
            self.close_size,
        );

        let mut body_node = body_node(renderer, &limits, &self.body, self.padding_body, self.width);

        body_node.move_to(Point::new(
            body_node.bounds().x,
            body_node.bounds().y + head_node.bounds().height,
        ));

        let mut foot_node = self.foot.as_ref().map_or_else(Node::default, |foot| {
            foot_node(renderer, &limits, foot, self.padding_foot, self.width)
        });

        foot_node.move_to(Point::new(
            foot_node.bounds().x,
            foot_node.bounds().y + head_node.bounds().height + body_node.bounds().height,
        ));

        Node::with_children(
            Size::new(
                body_node.size().width,
                head_node.size().height + body_node.size().height + foot_node.size().height,
            ),
            vec![head_node, body_node, foot_node],
        )
    }

    fn on_event(
        &mut self,
        state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        let mut children = layout.children();

        let head_layout = children
            .next()
            .expect("Native: Layout should have a head layout");
        let mut head_children = head_layout.children();
        let head_status = self.head.as_widget_mut().on_event(
            &mut state.children[0],
            event.clone(),
            head_children
                .next()
                .expect("Native: Layout should have a head content layout"),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );

        let close_status = head_children
            .next()
            .map_or(event::Status::Ignored, |close_layout| {
                match event {
                    Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                    | Event::Touch(touch::Event::FingerPressed { .. }) => self
                        .on_close
                        .clone()
                        // TODO: `let` expressions in this position are experimental
                        // see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
                        .filter(|_| {
                            close_layout
                                .bounds()
                                .contains(cursor.position().unwrap_or_default())
                        })
                        .map_or(event::Status::Ignored, |on_close| {
                            shell.publish(on_close);
                            event::Status::Captured
                        }),
                    _ => event::Status::Ignored,
                }
            });

        let body_layout = children
            .next()
            .expect("Native: Layout should have a body layout");
        let mut body_children = body_layout.children();
        let body_status = self.body.as_widget_mut().on_event(
            &mut state.children[1],
            event.clone(),
            body_children
                .next()
                .expect("Native: Layout should have a body content layout"),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );

        let foot_layout = children
            .next()
            .expect("Native: Layout should have a foot layout");
        let mut foot_children = foot_layout.children();
        let foot_status = self.foot.as_mut().map_or(event::Status::Ignored, |foot| {
            foot.as_widget_mut().on_event(
                &mut state.children[2],
                event,
                foot_children
                    .next()
                    .expect("Native: Layout should have a foot content layout"),
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            )
        });

        head_status
            .merge(close_status)
            .merge(body_status)
            .merge(foot_status)
    }

    fn mouse_interaction(
        &self,
        state: &Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let mut children = layout.children();

        let head_layout = children
            .next()
            .expect("Native: Layout should have a head layout");
        let mut head_children = head_layout.children();

        let head = head_children
            .next()
            .expect("Native: Layout should have a head layout");
        let close_layout = head_children.next();

        let is_mouse_over_close = close_layout.map_or(false, |layout| {
            let bounds = layout.bounds();
            bounds.contains(cursor.position().unwrap_or_default())
        });

        let mouse_interaction = if is_mouse_over_close {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        };

        let body_layout = children
            .next()
            .expect("Native: Layout should have a body layout");
        let mut body_children = body_layout.children();

        let foot_layout = children
            .next()
            .expect("Native: Layout should have a foot layout");
        let mut foot_children = foot_layout.children();

        mouse_interaction
            .max(self.head.as_widget().mouse_interaction(
                &state.children[0],
                head,
                cursor,
                viewport,
                renderer,
            ))
            .max(
                self.body.as_widget().mouse_interaction(
                    &state.children[1],
                    body_children
                        .next()
                        .expect("Native: Layout should have a body content layout"),
                    cursor,
                    viewport,
                    renderer,
                ),
            )
            .max(
                self.foot
                    .as_ref()
                    .map_or_else(mouse::Interaction::default, |foot| {
                        foot.as_widget().mouse_interaction(
                            &state.children[2],
                            foot_children
                                .next()
                                .expect("Native: Layout should have a foot content layout"),
                            cursor,
                            viewport,
                            renderer,
                        )
                    }),
            )
    }

    fn operate<'b>(
        &'b self,
        state: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<Message>,
    ) {
        let mut children = layout.children();
        let head_layout = children.next().expect("Missing Head Layout");
        let body_layout = children.next().expect("Missing Body Layout");
        let foot_layout = children.next().expect("Missing Footer Layout");

        self.head
            .as_widget()
            .operate(&mut state.children[0], head_layout, renderer, operation);
        self.body
            .as_widget()
            .operate(&mut state.children[1], body_layout, renderer, operation);

        if let Some(footer) = &self.foot {
            footer
                .as_widget()
                .operate(&mut state.children[2], foot_layout, renderer, operation);
        };
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let mut children = layout.children();
        let style_sheet = theme.active(&self.style);

        // Background
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border_radius: style_sheet.border_radius.into(),
                border_width: style_sheet.border_width,
                border_color: style_sheet.border_color,
            },
            style_sheet.background,
        );

        // Border
        renderer.fill_quad(
            // TODO: fill not necessary
            renderer::Quad {
                bounds,
                border_radius: style_sheet.border_radius.into(),
                border_width: style_sheet.border_width,
                border_color: style_sheet.border_color,
            },
            Color::TRANSPARENT,
        );

        // ----------- Head ----------------------
        let head_layout = children
            .next()
            .expect("Graphics: Layout should have a head layout");
        draw_head(
            &state.children[0],
            renderer,
            &self.head,
            head_layout,
            cursor,
            viewport,
            theme,
            &self.style,
            self.close_size,
        );

        // ----------- Body ----------------------
        let body_layout = children
            .next()
            .expect("Graphics: Layout should have a body layout");
        draw_body(
            &state.children[1],
            renderer,
            &self.body,
            body_layout,
            cursor,
            viewport,
            theme,
            &self.style,
        );

        // ----------- Foot ----------------------
        let foot_layout = children
            .next()
            .expect("Graphics: Layout should have a foot layout");
        draw_foot(
            state.children.get(2),
            renderer,
            &self.foot,
            foot_layout,
            cursor,
            viewport,
            theme,
            &self.style,
        );
    }
}

/// Calculates the layout of the head.
fn head_node<Message, Renderer>(
    renderer: &Renderer,
    limits: &Limits,
    head: &Element<'_, Message, Renderer>,
    padding: f32,
    width: Length,
    on_close: bool,
    close_size: Option<f32>,
) -> Node
where
    Renderer: iced::advanced::Renderer + iced::advanced::text::Renderer<Font = iced::Font>,
{
    let pad = Padding::from(padding as u16);
    let mut limits = limits
        .loose()
        .width(width)
        .height(head.as_widget().height())
        .pad(pad);

    let close_size = close_size.unwrap_or_else(|| renderer.default_size());
    let mut close = if on_close {
        limits = limits.shrink(Size::new(close_size, 0.0));
        Some(Node::new(Size::new(close_size + 1.0, close_size + 1.0)))
    } else {
        None
    };

    let mut head = head.as_widget().layout(renderer, &limits);
    let mut size = limits.resolve(head.size());

    head.move_to(Point::new(padding, padding));
    head.align(Alignment::Start, Alignment::Center, head.size());

    if let Some(node) = close.as_mut() {
        size = Size::new(size.width + close_size, size.height);

        node.move_to(Point::new(size.width - padding, padding));
        node.align(Alignment::End, Alignment::Center, node.size());
    }

    Node::with_children(
        size.pad(pad),
        match close {
            Some(node) => vec![head, node],
            None => vec![head],
        },
    )
}

/// Calculates the layout of the body.
fn body_node<Message, Renderer>(
    renderer: &Renderer,
    limits: &Limits,
    body: &Element<'_, Message, Renderer>,
    padding: f32,
    width: Length,
) -> Node
where
    Renderer: iced::advanced::Renderer,
{
    let pad = Padding::from(padding as u16);
    let limits = limits
        .clone()
        .loose()
        .width(width)
        .height(body.as_widget().height())
        .pad(pad);

    let mut body = body.as_widget().layout(renderer, &limits);
    let size = limits.resolve(body.size());

    body.move_to(Point::new(padding, padding));
    body.align(Alignment::Start, Alignment::Start, size);

    Node::with_children(size.pad(pad), vec![body])
}

/// Calculates the layout of the foot.
fn foot_node<Message, Renderer>(
    renderer: &Renderer,
    limits: &Limits,
    foot: &Element<'_, Message, Renderer>,
    padding: f32,
    width: Length,
) -> Node
where
    Renderer: iced::advanced::Renderer,
{
    let pad = Padding::from(padding as u16);
    let limits = limits
        .clone()
        .loose()
        .width(width)
        .height(foot.as_widget().height())
        .pad(pad);

    let mut foot = foot.as_widget().layout(renderer, &limits);
    let size = limits.resolve(foot.size());

    foot.move_to(Point::new(padding, padding));
    foot.align(Alignment::Start, Alignment::Center, size);

    Node::with_children(size.pad(pad), vec![foot])
}

/// Draws the head of the card.
#[allow(clippy::too_many_arguments)]
fn draw_head<Message, Renderer>(
    state: &Tree,
    renderer: &mut Renderer,
    head: &Element<'_, Message, Renderer>,
    layout: Layout<'_>,
    cursor: Cursor,
    viewport: &Rectangle,
    theme: &Renderer::Theme,
    style: &<Renderer::Theme as StyleSheet>::Style,
    close_size: Option<f32>,
) where
    Renderer: iced::advanced::Renderer + iced::advanced::text::Renderer<Font = iced::Font>,
    Renderer::Theme: StyleSheet,
{
    let mut head_children = layout.children();
    let style_sheet = theme.active(style);
    let bounds = layout.bounds();
    let border_radius = style_sheet.border_radius;

    // Head background
    renderer.fill_quad(
        renderer::Quad {
            bounds,
            border_radius: BorderRadius::from(border_radius),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        },
        style_sheet.head_background,
    );

    // cover rounded button of header
    renderer.fill_quad(
        renderer::Quad {
            bounds: Rectangle {
                x: bounds.x,
                y: bounds.y + bounds.height - border_radius,
                width: bounds.width,
                height: border_radius,
            },
            border_radius: (0.0).into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        },
        style_sheet.head_background,
    );

    head.as_widget().draw(
        state,
        renderer,
        theme,
        &renderer::Style {
            text_color: style_sheet.head_text_color,
        },
        head_children
            .next()
            .expect("Graphics: Layout should have a head content layout"),
        cursor,
        viewport,
    );

    if let Some(close_layout) = head_children.next() {
        let close_bounds = close_layout.bounds();
        let is_mouse_over_close = close_bounds.contains(cursor.position().unwrap_or_default());

        renderer.fill_text(iced::advanced::text::Text {
            content: &Icon::X.to_string(),
            bounds: Rectangle {
                x: close_bounds.center_x(),
                y: close_bounds.center_y(),
                height: close_bounds.height,
                ..close_bounds
            },
            size: close_size.unwrap_or_else(|| renderer.default_size())
                + if is_mouse_over_close { 1.0 } else { 0.0 },
            color: style_sheet.close_color,
            font: ICON_FONT,
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
            line_height: LineHeight::Relative(1.3),
            shaping: iced::widget::text::Shaping::Advanced,
        });
    }
}

/// Draws the body of the card.
#[allow(clippy::too_many_arguments)]
fn draw_body<Message, Renderer>(
    state: &Tree,
    renderer: &mut Renderer,
    body: &Element<'_, Message, Renderer>,
    layout: Layout<'_>,
    cursor: Cursor,
    viewport: &Rectangle,
    theme: &Renderer::Theme,
    style: &<Renderer::Theme as StyleSheet>::Style,
) where
    Renderer: iced::advanced::Renderer + iced::advanced::text::Renderer<Font = iced::Font>,
    Renderer::Theme: StyleSheet,
{
    let mut body_children = layout.children();
    let style_sheet = theme.active(style);

    // Body background
    renderer.fill_quad(
        renderer::Quad {
            bounds: layout.bounds(),
            border_radius: (0.0).into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        },
        style_sheet.body_background,
    );

    body.as_widget().draw(
        state,
        renderer,
        theme,
        &renderer::Style {
            text_color: style_sheet.body_text_color,
        },
        body_children
            .next()
            .expect("Graphics: Layout should have a body content layout"),
        cursor,
        viewport,
    );
}

/// Draws the foot of the card.
#[allow(clippy::too_many_arguments)]
fn draw_foot<Message, Renderer>(
    state: Option<&Tree>,
    renderer: &mut Renderer,
    foot: &Option<Element<'_, Message, Renderer>>,
    layout: Layout<'_>,
    cursor: Cursor,
    viewport: &Rectangle,
    theme: &Renderer::Theme,
    style: &<Renderer::Theme as StyleSheet>::Style,
) where
    Renderer: iced::advanced::Renderer + iced::advanced::text::Renderer<Font = iced::Font>,
    Renderer::Theme: StyleSheet,
{
    let mut foot_children = layout.children();
    let style_sheet = theme.active(style);

    // Foot background
    renderer.fill_quad(
        renderer::Quad {
            bounds: layout.bounds(),
            border_radius: style_sheet.border_radius.into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        },
        style_sheet.foot_background,
    );

    if let Some((foot, state)) = foot.as_ref().zip(state) {
        foot.as_widget().draw(
            state,
            renderer,
            theme,
            &renderer::Style {
                text_color: style_sheet.foot_text_color,
            },
            foot_children
                .next()
                .expect("Graphics: Layout should have a foot content layout"),
            cursor,
            viewport,
        );
    }
}

impl<'a, Message, Renderer> From<Card<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Renderer: 'a + iced::advanced::Renderer + iced::advanced::text::Renderer<Font = iced::Font>,
    Renderer::Theme: StyleSheet,
    Message: Clone + 'a,
{
    fn from(card: Card<'a, Message, Renderer>) -> Self {
        Element::new(card)
    }
}


use iced::{Background, Theme};

/// The appearance of a [`Card`](crate::native::card::Card).
#[derive(Clone, Copy, Debug)]
pub struct Appearance {
    /// The background of the [`Card`](crate::native::card::Card).
    pub background: Background,

    /// The border radius of the [`Card`](crate::native::card::Card).
    pub border_radius: f32,

    /// The border width of the [`Card`](crate::native::card::Card).
    pub border_width: f32,

    /// The border color of the [`Card`](crate::native::card::Card).
    pub border_color: Color,

    /// The background of the head of the [`Card`](crate::native::card::Card).
    pub head_background: Background,

    /// The text color of the head of the [`Card`](crate::native::card::Card).
    pub head_text_color: Color,

    /// The background of the body of the [`Card`](crate::native::card::Card).
    pub body_background: Background,

    /// The text color of the body of the [`Card`](crate::native::card::Card).
    pub body_text_color: Color,

    /// The background of the foot of the [`Card`](crate::native::card::Card).
    pub foot_background: Background,

    /// The text color of the foot of the [`Card`](crate::native::card::Card).
    pub foot_text_color: Color,

    /// The color of the close icon of the [`Card`](crate::native::card::Card).
    pub close_color: Color,
}

/// The appearance of a [`Card`](crate::native::card::Card).
#[allow(missing_docs, clippy::missing_docs_in_private_items)]
trait StyleSheet {
    type Style: Default;
    /// The normal appearance of a [`Card`](crate::native::card::Card).
    fn active(&self, style: &Self::Style) -> Appearance;
}

#[derive(Default)]
#[allow(missing_docs, clippy::missing_docs_in_private_items)]
/// Default Prebuilt ``Card`` Styles
pub enum CardStyles {
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
    Light,
    Dark,
    White,
    #[default]
    Default,
    Custom(Box<dyn StyleSheet<Style = Theme>>),
}

impl CardStyles {
    /// Creates a custom [`BadgeStyles`] style variant.
    pub fn custom(style_sheet: impl StyleSheet<Style = Theme> + 'static) -> Self {
        Self::Custom(Box::new(style_sheet))
    }
}

impl StyleSheet for Theme {
    type Style = CardStyles;

    fn active(&self, style: &Self::Style) -> Appearance {
        let palette = self.extended_palette();
        let foreground = self.palette();

        let backing_with_text = |color: Color, text_color: Color| Appearance {
            border_color: color,
            head_background: color.into(),
            head_text_color: text_color,
            close_color: text_color,
            background: palette.background.base.color.into(),
            body_text_color: foreground.text,
            foot_text_color: foreground.text,
            ..Appearance::default()
        };

        let backing_only = |color: Color| Appearance {
            border_color: color,
            head_background: color.into(),
            background: palette.background.base.color.into(),
            body_text_color: foreground.text,
            foot_text_color: foreground.text,
            ..Appearance::default()
        };

        match style {
            CardStyles::Primary => backing_with_text(iced::Color, iced_aw::style::colors::WHITE),
            CardStyles::Secondary => backing_with_text(iced_aw::style::colors::SECONDARY, iced_aw::style::colors::WHITE),
            CardStyles::Success => backing_with_text(iced_aw::style::colors::SUCCESS, iced_aw::style::colors::WHITE),
            CardStyles::Danger => backing_with_text(iced_aw::style::colors::DANGER, iced_aw::style::colors::WHITE),
            CardStyles::Warning => backing_only(iced_aw::style::colors::WARNING),
            CardStyles::Info => backing_only(iced_aw::style::colors::INFO),
            CardStyles::Light => backing_only(iced_aw::style::colors::LIGHT),
            CardStyles::Dark => backing_with_text(iced_aw::style::colors::DARK, iced_aw::style::colors::WHITE),
            CardStyles::White => backing_only(iced_aw::style::colors::WHITE),
            CardStyles::Default => backing_only([0.87, 0.87, 0.87].into()),
            CardStyles::Custom(custom) => custom.active(self),
        }
    }
}

impl Default for Appearance {
    fn default() -> Self {
        Self {
            background: Color::WHITE.into(),
            border_radius: 10.0,
            border_width: 1.0,
            border_color: [0.87, 0.87, 0.87].into(),
            head_background: Background::Color([0.87, 0.87, 0.87].into()),
            head_text_color: Color::BLACK,
            body_background: Color::TRANSPARENT.into(),
            body_text_color: Color::BLACK,
            foot_background: Color::TRANSPARENT.into(),
            foot_text_color: Color::BLACK,
            close_color: Color::BLACK,
        }
    }
}
