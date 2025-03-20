use iced::widget::container;

/// A set of rules that dictate the styling of a [`Table`](crate::Table).
pub trait Catalog {
    /// The supported style of the [`Catalog`].
    type Style: Default + Clone;

    /// The header [`Style`](iced_widget::container::Style) of the [`Catalog`].
    fn header(&self, style: &Self::Style) -> container::Style;
    /// The footer [`Style`](iced_widget::container::Style) of the [`Catalog`].
    fn footer(&self, style: &Self::Style) -> container::Style;
    /// The row [`Style`](iced_widget::container::Style) of the [`Catalog`].
    fn row(&self, style: &Self::Style, index: usize) -> container::Style;
    /// The divider [`Style`](iced_widget::container::Style) of the [`Catalog`].
    fn divider(&self, style: &Self::Style, hovered: bool) -> container::Style;
}

impl Catalog for iced::Theme {
    type Style = ();

    fn header(&self, _style: &Self::Style) -> container::Style {
        container::Style {
            text_color: Some(self.extended_palette().background.strong.text),
            background: Some(self.extended_palette().background.strong.color.into()),
            ..Default::default()
        }
    }

    fn footer(&self, style: &Self::Style) -> container::Style {
        self.header(style)
    }

    fn row(&self, _style: &Self::Style, index: usize) -> container::Style {
        let pair = if index % 2 == 0 {
            self.extended_palette().background.base
        } else {
            self.extended_palette().background.weak
        };

        container::Style {
            text_color: Some(pair.text),
            background: Some(pair.color.into()),
            ..Default::default()
        }
    }

    fn divider(&self, _style: &Self::Style, hovered: bool) -> container::Style {
        let pair = if hovered {
            self.extended_palette().primary.base
        } else {
            self.extended_palette().background.weak
        };

        container::Style {
            background: Some(pair.color.into()),
            ..Default::default()
        }
    }
}

pub(crate) mod wrapper {
    use iced::{mouse::Cursor, Color, Element, Length, Size, Vector};
    use iced::advanced::Widget;
    use iced::widget::container;

    pub fn header<'a, Message, Theme, Renderer>(
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
        style: <Theme as super::Catalog>::Style,
    ) -> Element<'a, Message, Theme, Renderer>
    where
        Renderer: iced::advanced::Renderer + 'a,
        Theme: super::Catalog + 'a,
        Message: 'a,
    {
        Wrapper {
            content: content.into(),
            target: Target::Header,
            style,
        }
        .into()
    }

    pub fn footer<'a, Message, Theme, Renderer>(
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
        style: <Theme as super::Catalog>::Style,
    ) -> Element<'a, Message, Theme, Renderer>
    where
        Renderer: iced::advanced::Renderer + 'a,
        Theme: super::Catalog + 'a,
        Message: 'a,
    {
        Wrapper {
            content: content.into(),
            target: Target::Footer,
            style,
        }
        .into()
    }

    pub fn row<'a, Message, Theme, Renderer>(
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
        style: <Theme as super::Catalog>::Style,
        index: usize,
    ) -> Element<'a, Message, Theme, Renderer>
    where
        Renderer: iced::advanced::Renderer + 'a,
        Theme: super::Catalog + 'a,
        Message: 'a,
    {
        Wrapper {
            content: content.into(),
            target: Target::Row { index },
            style,
        }
        .into()
    }

    enum Target {
        Header,
        Footer,
        Row { index: usize },
    }

    impl Target {
        fn appearance<Theme>(
            &self,
            theme: &Theme,
            style: &<Theme as super::Catalog>::Style,
        ) -> container::Style
        where
            Theme: super::Catalog,
        {
            match self {
                Target::Header => theme.header(style),
                Target::Footer => theme.footer(style),
                Target::Row { index } => theme.row(style, *index),
            }
        }
    }

    struct Wrapper<'a, Message, Theme, Renderer>
    where
        Renderer: iced::advanced::Renderer,
        Theme: super::Catalog,
    {
        content: Element<'a, Message, Theme, Renderer>,
        target: Target,
        style: <Theme as super::Catalog>::Style,
    }

    impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer>
        for Wrapper<'_, Message, Theme, Renderer>
    where
        Renderer: iced::advanced::Renderer,
        Theme: super::Catalog,
    {
        fn size(&self) -> Size<Length> {
            self.content.as_widget().size()
        }

        fn layout(
            &self,
            state: &mut iced::advanced::widget::Tree,
            renderer: &Renderer,
            limits: &iced::advanced::layout::Limits,
        ) -> iced::advanced::layout::Node {
            self.content.as_widget().layout(state, renderer, limits)
        }

        fn draw(
            &self,
            state: &iced::advanced::widget::Tree,
            renderer: &mut Renderer,
            theme: &Theme,
            style: &iced::advanced::renderer::Style,
            layout: iced::advanced::Layout<'_>,
            cursor: Cursor,
            viewport: &iced::Rectangle,
        ) {
            let appearance = self.target.appearance::<Theme>(theme, &self.style);

            renderer.fill_quad(
                iced::advanced::renderer::Quad {
                    bounds: layout.bounds(),
                    border: appearance.border,
                    shadow: Default::default(),
                },
                appearance
                    .background
                    .unwrap_or_else(|| Color::TRANSPARENT.into()),
            );

            let style = appearance
                .text_color
                .map(|text_color| iced::advanced::renderer::Style { text_color })
                .unwrap_or(*style);

            self.content
                .as_widget()
                .draw(state, renderer, theme, &style, layout, cursor, viewport)
        }

        fn tag(&self) -> iced::advanced::widget::tree::Tag {
            self.content.as_widget().tag()
        }

        fn state(&self) -> iced::advanced::widget::tree::State {
            self.content.as_widget().state()
        }

        fn children(&self) -> Vec<iced::advanced::widget::Tree> {
            self.content.as_widget().children()
        }

        fn diff(&self, tree: &mut iced::advanced::widget::Tree) {
            self.content.as_widget().diff(tree)
        }

        fn operate(
            &self,
            state: &mut iced::advanced::widget::Tree,
            layout: iced::advanced::Layout<'_>,
            renderer: &Renderer,
            operation: &mut dyn iced::advanced::widget::Operation,
        ) {
            self.content
                .as_widget()
                .operate(state, layout, renderer, operation)
        }

        fn on_event(
            &mut self,
            state: &mut iced::advanced::widget::Tree,
            event: iced::Event,
            layout: iced::advanced::Layout<'_>,
            cursor: Cursor,
            renderer: &Renderer,
            clipboard: &mut dyn iced::advanced::Clipboard,
            shell: &mut iced::advanced::Shell<'_, Message>,
            viewport: &iced::Rectangle,
        ) -> iced::event::Status {
            self.content.as_widget_mut().on_event(
                state, event, layout, cursor, renderer, clipboard, shell, viewport,
            )
        }

        fn mouse_interaction(
            &self,
            state: &iced::advanced::widget::Tree,
            layout: iced::advanced::Layout<'_>,
            cursor: Cursor,
            viewport: &iced::Rectangle,
            renderer: &Renderer,
        ) -> iced::advanced::mouse::Interaction {
            self.content
                .as_widget()
                .mouse_interaction(state, layout, cursor, viewport, renderer)
        }

        fn overlay<'b>(
            &'b mut self,
            state: &'b mut iced::advanced::widget::Tree,
            layout: iced::advanced::Layout<'_>,
            renderer: &Renderer,
            translation: Vector,
        ) -> Option<iced::advanced::overlay::Element<'b, Message, Theme, Renderer>> {
            self.content
                .as_widget_mut()
                .overlay(state, layout, renderer, translation)
        }
    }

    impl<'a, Message, Theme, Renderer> From<Wrapper<'a, Message, Theme, Renderer>>
        for Element<'a, Message, Theme, Renderer>
    where
        Renderer: iced::advanced::Renderer + 'a,
        Theme: super::Catalog + 'a,
        Message: 'a,
    {
        fn from(wrapper: Wrapper<'a, Message, Theme, Renderer>) -> Self {
            Element::new(wrapper)
        }
    }
}
