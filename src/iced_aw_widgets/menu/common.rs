use iced::{Padding, Rectangle};

/* /// The condition of when to close a menu
#[derive(Debug, Clone, Copy)]
pub struct CloseCondition {
    /// Close menus when the cursor moves outside the check bounds
    pub leave: bool,

    /// Close menus when the cursor clicks outside the check bounds
    pub click_outside: bool,

    /// Close menus when the cursor clicks inside the check bounds
    pub click_inside: bool,
}
 */

///
/// ## FakeHovering:
///
/// Places cursors at the path items,
/// useful when you want to customize the styling of each item in the path,
/// or you simple want the look of the items when they are hovered over.
///
/// The downside is when some widgets in the path don't response to hovering,
/// the path won't be fully drawn, and when you want uniform path styling
/// but some widgets response to hovering differently.
///
/// ## Backdrop:
///
/// Draws a rectangle behind each path item,
/// requires path items to have transparent backgrounds,
/// useful when you want uniform path styling.
///
/// The downside is,
/// depending on the style you're going for,
/// oftentimes manually syncing the path's styling to the path items' is necessary,
/// the default styling simply can't cover most use cases.
pub enum DrawPath {
    /// FakeHovering
    FakeHovering,
    /// Backdrop
    Backdrop,
}

/// X+ goes right and Y+ goes down
#[derive(Debug, Clone, Copy)]
pub(super) enum Direction {
    Positive,
    Negative,
}
impl Direction {
    pub(super) fn flip(self) -> Self {
        match self {
            Self::Positive => Self::Negative,
            Self::Negative => Self::Positive,
        }
    }
}

/// Axis
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy)]
pub(super) enum Axis {
    Horizontal,
    Vertical,
}

pub(super) type Index = Option<usize>;

/// Should be returned from the recursive event processing function,
/// tells the caller which type of event has been processed
pub(super) enum RecEvent {
    Event,
    Close,
    None,
}

#[derive(Debug, Clone, Copy)]
/// Scroll speed
pub struct ScrollSpeed {
    /// Speed of line-based scroll movement
    pub line: f32,
    /// Speed of Pixel scroll movement
    pub pixel: f32,
}

pub fn pad_rectangle(rect: Rectangle, padding: Padding) -> Rectangle {
    Rectangle {
        x: rect.x - padding.left,
        y: rect.y - padding.top,
        width: rect.width + padding.horizontal(),
        height: rect.height + padding.vertical(),
    }
}


/// Methods for creating inner bounds
#[allow(missing_debug_implementations)]
pub enum InnerBounds {
    /// Create inner bounds ratio to the outer bounds
    Ratio(f32, f32),
    /// Create inner bounds by padding the outer bounds
    Padding(Padding),
    /// Create square inner bounds
    Square(f32),
    /// Create inner bounds with a custom function
    Custom(Box<dyn Fn(Rectangle) -> Rectangle>),
}
impl InnerBounds {
    /// Gets the inner bounds of the Set type.
    #[must_use]
    pub fn get_bounds(&self, outer_bounds: Rectangle) -> Rectangle {
        use InnerBounds::{Custom, Padding, Ratio, Square};
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
            Custom(f) => f(outer_bounds),
        }
    }
}
