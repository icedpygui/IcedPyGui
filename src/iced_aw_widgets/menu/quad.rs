
use iced::{
    advanced::{
        layout::{Limits, Node},
        renderer,
        widget::Tree,
        Layout, Widget,
    }, border::Radius, mouse::Cursor, Background, Border, Color, Element, Length, Rectangle, Shadow, Size
};

#[derive(Debug, Clone, Copy)]
pub struct Quad {
    /// Width of the quad
    pub width: Length,
    /// Height of the quad
    pub height: Length,

    /// Methods for creating inner bounds
    pub inner_bounds: InnerBounds,

    /// Color of the quad
    pub quad_color: Background,
    /// Border of the quad
    pub quad_border: Border,
    /// Shadow of the quad
    pub quad_shadow: Shadow,

    /// Background color of the quad
    pub bg_color: Option<Background>,
    /// Border of the background
    pub bg_border: Border,
    /// Shadow of the background
    pub bg_shadow: Shadow,
}
impl Default for Quad {
    fn default() -> Self {
        Self {
            width: Length::Fill,
            height: Length::Fill,
            inner_bounds: InnerBounds::Ratio(0.5, 0.5),

            quad_color: Color::from([0.5; 3]).into(),
            quad_border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::default(),
            },
            quad_shadow: Shadow::default(),

            bg_color: None,
            bg_border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::default(),
            },
            bg_shadow: Shadow::default(),
        }
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Quad
where
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size::new(self.width, self.height)
    }

    fn layout(&self, _tree: &mut Tree, _renderer: &Renderer, limits: &Limits) -> Node {
        let limits = limits.width(self.width).height(self.height);
        Node::new(limits.max())
    }

    fn draw(
        &self,
        _state: &Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: Cursor,
        _viewport: &Rectangle,
    ) {
        if let Some(b) = self.bg_color {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: layout.bounds(),
                    border: self.bg_border,
                    shadow: self.bg_shadow,
                },
                b,
            );
        }
        renderer.fill_quad(
            renderer::Quad {
                bounds: self.inner_bounds.get_bounds(layout.bounds()),
                border: self.quad_border,
                shadow: self.quad_shadow,
            },
            self.quad_color,
        );
    }
}

impl<'a, Message, Theme, Renderer> From<Quad> for Element<'a, Message, Theme, Renderer>
where
    Renderer: 'a + renderer::Renderer,
    Theme: 'a,
{
    fn from(value: Quad) -> Self {
        Self::new(value)
    }
}


use iced::Padding;

#[derive(Debug, Clone, Copy)]
pub enum InnerBounds {
    /// Create inner bounds ratio to the outer bounds
    Ratio(f32, f32),
    /// Create inner bounds by padding the outer bounds
    Padding(Padding),
    /// Create square inner bounds
    Square(f32),

}
impl InnerBounds {
    /// Gets the inner bounds of the Set type.
    #[must_use]
    pub fn get_bounds(&self, outer_bounds: Rectangle) -> Rectangle {
        use InnerBounds::{Padding, Ratio, Square};
        match self {
            Ratio(w, h) => {
                let width = w * outer_bounds.width;
                let height = h * outer_bounds.height;
                let x = outer_bounds.x + (outer_bounds.width - width) * 0.5;
                let y = outer_bounds.y + (outer_bounds.height - height) * 0.5;
                Rectangle {
                    x,
                    y,
                    width,
                    height,
                }
            }
            Padding(p) => {
                let x = outer_bounds.x + p.left;
                let y = outer_bounds.y + p.top;
                let width = outer_bounds.width - p.horizontal();
                let height = outer_bounds.width - p.vertical();
                Rectangle {
                    x,
                    y,
                    width,
                    height,
                }
            }
            Square(l) => {
                let width = *l;
                let height = *l;
                let x = outer_bounds.x + (outer_bounds.width - width) * 0.5;
                let y = outer_bounds.y + (outer_bounds.height - height) * 0.5;
                Rectangle {
                    x,
                    y,
                    width,
                    height,
                }
            }
        }
    }
}
