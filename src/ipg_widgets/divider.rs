//! Display an interactive selector of a single value from a range of values to resize containers.
use iced::border::{Border, Radius};
use iced::event::{self, Event};
use iced::advanced::layout;
use iced::{mouse, Background};
use iced::advanced::renderer;
use iced::touch;
use iced::advanced::widget::tree::{self, Tree};
use iced::{
    self, Color, Element, Length, 
    Rectangle, Size, Theme,
};
use iced::advanced::{Clipboard, Layout, Shell, Widget};

// divider version 0.3.1
pub fn divider_horizontal<'a, Message, Theme>(
    widths: Vec<f32>,
    handle_width: f32,
    handle_height: f32,
    on_change: impl Fn((usize, f32)) -> Message + 'a,
) -> Divider<'a, Message, Theme>
where
    Message: Clone,
    Theme: Catalog + 'a,
{
    let mut handle_offsets = vec![-handle_width/2.0; widths.len()-1];
        handle_offsets.extend([-handle_width]);
    Divider::new(
            widths, 
            handle_width, 
            handle_height,
            handle_offsets,
            Direction::Horizontal,
            on_change)
}

pub fn divider_vertical<'a, Message, Theme>(
    heights: Vec<f32>,
    handle_width: f32,
    handle_height: f32,
    on_change: impl Fn((usize, f32)) -> Message + 'a,
) -> Divider<'a, Message, Theme>
where
    Message: Clone,
    Theme: Catalog + 'a,
{
    let widths = heights;
    let mut handle_offsets = vec![-handle_height/2.0; widths.len()-1];
        // last offset pulled in to keep in bounds
        handle_offsets.extend([-handle_height]);
        
    Divider::new(
            widths, 
            handle_width, 
            handle_height,
            handle_offsets,
            Direction::Vertical,
            on_change)
}


#[allow(missing_debug_implementations)]
pub struct Divider<'a, Message, Theme = iced::Theme>
where
    Theme: Catalog,
{
    widths: Vec<f32>,
    handle_width: f32,
    handle_height: f32,
    on_change: Box<dyn Fn((usize, f32)) -> Message + 'a>,
    on_release: Option<Message>,
    width: Length,
    height: Length,
    handle_offsets: Vec<f32>,
    include_last_handle: bool,
    direction: Direction,
    class: Theme::Class<'a>,
}

impl<'a, Message, Theme> Divider<'a, Message, Theme>
where
    Message: Clone,
    Theme: Catalog,
{
    /// The default height of a [`Divider`].
    pub const DEFAULT_HEIGHT: f32 = 21.0;

    /// Creates a new [`Divider`].
    pub fn new<F>(
        widths: Vec<f32>,
        handle_width: f32,
        handle_height: f32,
        handle_offsets: Vec<f32>,
        direction: Direction, 
        on_change: F) 
        -> Self
    where
        F: 'a + Fn((usize, f32)) -> Message,
    {
        Divider {
            widths,
            handle_width,
            handle_height,
            on_change: Box::new(on_change),
            on_release: None,
            width: Length::Fill,
            height: Length::Fill,
            handle_offsets,
            include_last_handle: true,
            direction,
            class: Theme::default(),
        }
    }

    /// Sets the release message of the [`Divider`].
    /// This is called when the mouse is released from the Divider.
    ///
    /// Typically, the user's interaction with the Divider is finished when this message is produced.
    /// This is useful if you need to spawn a long-running task from the Divider's result, where
    /// the default on_change message could create too many events.
    pub fn on_release(mut self, on_release: Message) -> Self {
        self.on_release = Some(on_release);
        self
    }
    /// Sets the width of the [`Divider`] which usually spans the entire width of the items.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`Divider`].
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the handle offsets for alignment of the [`Divider`].
    pub fn handle_offsets(mut self, handle_offsets: Vec<f32>) -> Self {
        self.handle_offsets = handle_offsets;
        self
    }

    /// Sets the include_last_handle of the [`Divider`].
    /// If not included, the total width or height will not change
    pub fn include_last_handle(mut self, include: bool) -> Self {
        self.include_last_handle = include;
        self
    }

    /// Sets the direction of the [`Divided`].
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    /// Sets the style of the [`Divider`].
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self
    where
        Theme::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme>).into();
        self
    }

    /// Sets the style class of the [`Divider`].
    #[must_use]
    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Divider<'_, Message, Theme>
where
    Message: Clone,
    Theme: Catalog,
    Renderer: iced::advanced::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: Length::Shrink,
        }
    }

    fn layout(
        &self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::atomic(limits, self.width, self.height)
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> event::Status {
        let state = tree.state.downcast_mut::<State>();
        let is_dragging = state.is_dragging;
        let total_bounds = layout.bounds();
        
        // stores the state
        let mut widths = vec![];
        for width in self.widths.iter() {
            match self.direction {
                Direction::Horizontal => {
                    widths.push(*width);
                },
                Direction::Vertical => {
                    widths.push(*width);
                },
            }
        }
        state.handle_bounds = 
            get_handle_bounds(
                total_bounds,
                &widths,
                self.handle_width, 
                self.handle_height,
                &self.handle_offsets,
                self.include_last_handle,
                self.direction);

        state.width_height_bounds =
            get_width_height_bounds(
                total_bounds,
                &widths,
                self.handle_width, 
                self.handle_height, 
                self.direction);

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                let index = 
                    find_mouse_over_handle_bounds(
                        &state.handle_bounds, cursor);
                
                if index.is_some() {
                    state.is_dragging = true;
                    state.index = index.unwrap();
                    return event::Status::Captured;
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. })
            | Event::Touch(touch::Event::FingerLost { .. }) => {
                if is_dragging {
                    if let Some(on_release) = self.on_release.clone() {
                        shell.publish(on_release);
                    }
                    state.is_dragging = false;
                    state.handle_bounds = vec![];
                    state.width_height_bounds = vec![];
                    state.index = 0;

                    return event::Status::Captured;
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { position })
            | Event::Touch(touch::Event::FingerMoved { id: _, position }) => {
                if is_dragging {
                    let end_x = total_bounds.x+total_bounds.width;
                    let end_y = total_bounds.y+total_bounds.height;
                    let handle_bounds = state.handle_bounds[state.index];
                    let w_h_bounds = state.width_height_bounds[state.index];
                    let handle_count = state.handle_bounds.len();
                    let w_h_count = state.width_height_bounds.len();

                    match self.direction {
                        Direction::Horizontal => {
                            if (position.x - handle_bounds.x + handle_bounds.width/2.0).abs() > 0.99 {
                                let new_value = 
                                    // Moving left
                                    if position.x < w_h_bounds.x && state.index == 0 {

                                        state.handle_bounds[state.index].x = w_h_bounds.x;
                                        (state.index, 0.0)
                                    } else 
                                    // Moving left stopping at next divider
                                    if state.index > 0 && position.x < state.handle_bounds[state.index-1].x {

                                        state.handle_bounds[state.index].x = state.handle_bounds[state.index-1].x;
                                        (state.index, 0.0)
                                    } else
                                    // Moving right: stop at next divider
                                    if  state.index < handle_count-1 && (state.index < handle_count) && 
                                        (position.x > state.handle_bounds[state.index+1].x) {

                                        state.handle_bounds[state.index].x = state.handle_bounds[state.index+1].x;
                                        (state.index, 0.0)
                                    } else 
                                    // Moving right: last index and no divider at end
                                    if (handle_count < w_h_count) && 
                                        (position.x > end_x-handle_bounds.width/2.0) {

                                        state.handle_bounds[state.index].x = end_x-handle_bounds.width/2.0;
                                        let new_value = (end_x-handle_bounds.width/2.0-w_h_bounds.x).round();
                                        (state.index, new_value)
                                    }
                                     else {
                                        // moving
                                        state.handle_bounds[state.index].x = position.x;
                                        let new_value = (position.x - w_h_bounds.x).round();
                                        (state.index, new_value)
                                    };
                                
                                shell.publish((self.on_change)(new_value));
                                return event::Status::Captured;
                            }
                        },
                        Direction::Vertical => {
                            if (position.y - handle_bounds.y + handle_bounds.height/2.0).abs() > 0.99 {
                                let new_value = 
                                    // Moving up
                                    if position.y < w_h_bounds.y && state.index == 0 {

                                        state.handle_bounds[state.index].y = w_h_bounds.y;
                                        (state.index, 0.0)
                                    } else 
                                    // Moving left stopping at next divider
                                    if state.index > 0 && position.y < state.handle_bounds[state.index-1].y {

                                        state.handle_bounds[state.index].y = state.handle_bounds[state.index-1].y;
                                        (state.index, 0.0)
                                    } else
                                    // Moving right: stop at next divider
                                    if  state.index < handle_count-1 && (state.index < handle_count) && 
                                        (position.y > state.handle_bounds[state.index+1].y) {

                                        state.handle_bounds[state.index].y = state.handle_bounds[state.index+1].y;
                                        (state.index, 0.0)
                                    } else 
                                    // Moving right: last index and no divider at end
                                    if (handle_count < w_h_count) && 
                                        (position.y > end_y-handle_bounds.height/2.0) {
                                            
                                        state.handle_bounds[state.index].y = end_y-handle_bounds.height/2.0;
                                        let new_value = (end_y-handle_bounds.height/2.0-w_h_bounds.y).round();
                                        (state.index, new_value)
                                    }
                                     else {
                                        // moving
                                        state.handle_bounds[state.index].y = position.y;
                                        let new_value = (position.y - w_h_bounds.y).round();
                                        (state.index, new_value)
                                    };
                                
                                shell.publish((self.on_change)(new_value));
                                return event::Status::Captured;
                            }
                        },
                    }
                }
            },
            _ => {}
        }

        event::Status::Ignored

    }
    
    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        _layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State>();
        let is_mouse_over = 
            find_mouse_over_handle_bounds(
                &state.handle_bounds,
                cursor,);
        
        let status = if state.is_dragging {
            Status::Dragged
        } else if is_mouse_over.is_some() {
            Status::Hovered
        } else {
            Status::Active
        };

        let style = theme.style(&self.class, status);

        for i in 0..self.widths.len() {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: state.width_height_bounds[i],
                    ..renderer::Quad::default()
                },
                Background::Color(Color::TRANSPARENT),
            );
            // fill with the handle
            if !self.include_last_handle && i == self.widths.len()-1{
                break;
            }
            renderer.fill_quad(
                renderer::Quad {
                    bounds: state.handle_bounds[i],
                    border: Border {
                        radius: style.border_radius,
                        width: style.border_width,
                        color: style.border_color,
                    },
                    ..renderer::Quad::default()
                },
                style.background,
            );
        }

    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        _layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        let state = tree.state.downcast_ref::<State>();
        let is_mouse_over = 
            find_mouse_over_handle_bounds(
                &state.handle_bounds,  
                cursor);

        if state.is_dragging || is_mouse_over.is_some(){
            match self.direction {
                Direction::Horizontal => mouse::Interaction::ResizingHorizontally,
                Direction::Vertical => mouse::Interaction::ResizingVertically,
            }
        } else {
            mouse::Interaction::default()
        }
    }
}

impl<'a, Message, Theme, Renderer> From<Divider<'a, Message, Theme>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: Catalog + 'a,
    Renderer: iced::advanced::Renderer + 'a,
{
    fn from(
        divider: Divider<'a, Message, Theme>,
    ) -> Element<'a, Message, Theme, Renderer> {
        Element::new(divider)
    }
}

fn get_handle_bounds(
    bounds: Rectangle,
    widths_heights: &[f32],
    handle_width: f32,
    handle_height: f32,
    handle_offsets: &[f32],
    include_last_handle: bool,
    direction: Direction,
    ) -> Vec<Rectangle> 
{
    let mut handle_bounds = vec![];
    let mut start = match direction {
            Direction::Horizontal => bounds.x,
            Direction::Vertical => bounds.y,
        };
        for (i, width_height) in widths_heights.iter().enumerate() {
            
            if i == widths_heights.len()-1 {
                if include_last_handle {
                    start += width_height;
                } else {
                    break;
                }
            } else {
                start += width_height;
            }

            let rect = match direction {
                Direction::Horizontal => {
                    Rectangle{ 
                        x: start+handle_offsets[i], 
                        y: bounds.y, 
                        width: handle_width, 
                        height: handle_height,
                    }
                },
                Direction::Vertical => {
                    Rectangle{
                        x: bounds.x,
                        y: start+handle_offsets[i],
                        width: handle_width,
                        height: handle_height,
                    }
                },
            };
                
            handle_bounds.push(rect);

        }
        handle_bounds
}

fn get_width_height_bounds(
    bounds: Rectangle,
    widths_heights: &[f32],
    handle_width: f32,
    handle_height: f32,
    direction: Direction,
    ) -> Vec<Rectangle> 
{
    let mut w_h_bounds = vec![];
    let mut start = match direction {
            Direction::Horizontal => bounds.x,
            Direction::Vertical => bounds.y,
        };
        for width_height in widths_heights.iter() {
            let rect = match direction {
                Direction::Horizontal => {
                    Rectangle{ 
                        x: start, 
                        y: bounds.y, 
                        width: *width_height, 
                        height: handle_height,
                    }
                },
                Direction::Vertical => {
                    Rectangle{
                        x: bounds.x,
                        y: start,
                        width: handle_width,
                        height: *width_height,
                    }
                },
            };
                
            w_h_bounds.push(rect);

            match direction {
                Direction::Horizontal => {
                    start += width_height;
                },
                Direction::Vertical => {
                    start += width_height;
                },
            }
            
        }
        w_h_bounds
}

fn find_mouse_over_handle_bounds(
    handle_bounds: &[Rectangle],
    cursor: mouse::Cursor) 
    -> Option<usize> {
        for (index, bounds) in handle_bounds.iter().enumerate() {
            if cursor.is_over(*bounds) {
                return Some(index)
            }
        }
        None
}

/// The direction of [`Scrollable`].
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Direction {
    /// Horizontal resizing
    #[default]
    Horizontal,
    /// Vertical resizing
    Vertical,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct State {
    is_dragging: bool,
    index: usize,
    handle_bounds: Vec<Rectangle>,
    width_height_bounds: Vec<Rectangle>,
}

/// The possible status of a [`Divider`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// The [`Divider`] can be interacted with.
    Active,
    /// The [`Divider`] is being hovered.
    Hovered,
    /// The [`Divider`] is being dragged.
    Dragged,
}

/// The appearance of a Divider.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// The [`Background`] of the handle.
    pub background: Background,
    /// The border width of the handle.
    pub border_width: f32,
    /// The border [`Color`] of the handle.
    pub border_color: Color,
    /// The border [`Radius`] of the handle.
    pub border_radius: Radius,
}

/// The theme catalog of a [`Divider`].
pub trait Catalog: Sized {
    /// The item class of the [`Catalog`].
    type Class<'a>;

    /// The default class produced by the [`Catalog`].
    fn default<'a>() -> Self::Class<'a>;

    /// The [`Style`] of a class with the given status.
    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
}

/// A styling function for a [`Divider`].
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(primary)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

/// The default style of a [`Divider`].
pub fn primary(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();

    let color = match status {
        Status::Active => palette.primary.strong.color,
        Status::Hovered => palette.primary.base.color,
        Status::Dragged => palette.primary.strong.color,
    };

    Style {
        background: color.into(),
        border_color: Color::TRANSPARENT,
        border_width: 0.0,
        border_radius: 0.0.into()
    }
}

pub fn transparent(theme: &Theme, status: Status) -> Style {
    let mut style = primary(theme, status);
    style.background = Color::TRANSPARENT.into();
    style
}



#[test]
fn test_get_handle_bounds() {
    let widths_heights = vec![100.0, 100.0, 100.0, 100.0];
    let hz_bounds = Rectangle { x: 50.0, 
                                        y: 50.0, 
                                        width: 462.0, 
                                        height: 21.0 };
    let hz_handle_width = 4.0;
    let hz_handle_height = 21.0;
    let mut hz_handle_offsets = vec![-hz_handle_height/2.0; widths_heights.len()-1];
        hz_handle_offsets.extend([-hz_handle_height]);
    let hz_include_last_handle =true; 
    let hz_direction = Direction::Horizontal;

    let vt_bounds = Rectangle { x: 50.0, 
                                        y: 50.0, 
                                        width: 100.0, 
                                        height: 462.0 };
    let vt_handle_width = 100.0;
    let vt_handle_height = 4.0;
    let mut vt_handle_offsets = vec![-vt_handle_height/2.0; widths_heights.len()-1];
        vt_handle_offsets.extend([-vt_handle_height]);
    let vt_include_last_handle =true;
    let vt_direction = Direction::Vertical;

    let hz_bounds = 
        get_handle_bounds(
            hz_bounds, 
            &widths_heights, 
            hz_handle_width, 
            hz_handle_height,
            &hz_handle_offsets,
            hz_include_last_handle, 
            hz_direction);

    let vt_bounds = 
        get_handle_bounds(
            vt_bounds, 
            &widths_heights, 
            vt_handle_width, 
            vt_handle_height,
            &vt_handle_offsets,
            vt_include_last_handle, 
            vt_direction);

    let hz_results = vec![
        Rectangle { x: 139.5, y: 50.0, width: 4.0, height: 21.0 },
        Rectangle { x: 239.5, y: 50.0, width: 4.0, height: 21.0 },
        Rectangle { x: 339.5, y: 50.0, width: 4.0, height: 21.0 },
        Rectangle { x: 429.0, y: 50.0, width: 4.0, height: 21.0 }];

    let vt_results = vec![
        Rectangle { x: 50.0, y: 148.0, width: 100.0, height: 4.0 },
        Rectangle { x: 50.0, y: 248.0, width: 100.0, height: 4.0 },
        Rectangle { x: 50.0, y: 348.0, width: 100.0, height: 4.0 },
        Rectangle { x: 50.0, y: 446.0, width: 100.0, height: 4.0 }];
        
    assert_eq!(hz_results, hz_bounds);
    assert_eq!(vt_results, vt_bounds);

}

#[test]
fn test_find_mouse_over_handle_bounds() {
    let handle_bounds = vec![
        Rectangle {x: 150.0,y: 50.0,width: 4.0,height: 21.0 },
        Rectangle { x: 254.0, y: 50.0, width: 4.0, height: 21.0 },
        Rectangle { x: 358.0, y: 50.0, width: 4.0, height: 21.0 },
        Rectangle { x: 462.0, y: 50.0, width: 4.0, height: 21.0 }];

    let pass_cursor = mouse::Cursor::Available(iced::Point { x: 360.0, y: 55.0 });

    assert_eq!(find_mouse_over_handle_bounds(&handle_bounds, pass_cursor), Some(2));

    let fail_cursor = mouse::Cursor::Available(iced::Point { x: 360.0, y: 75.0 });

     assert_eq!(find_mouse_over_handle_bounds(&handle_bounds, fail_cursor), None);

}

#[test] 
fn test_get_width_height_bounds() {
    let widths_heights = vec![100.0, 100.0, 100.0, 100.0];
    let hz_bounds = Rectangle { x: 50.0, 
                                        y: 50.0, 
                                        width: 416.0, 
                                        height: 21.0 };
    let hz_handle_width = 4.0;
    let hz_handle_height = 21.0;
    let hz_direction = Direction::Horizontal;

    let vt_bounds = Rectangle { x: 50.0, 
                                        y: 50.0, 
                                        width: 100.0, 
                                        height: 416.0 };
    let vt_handle_width = 100.0;
    let vt_handle_height = 4.0;
    let vt_direction = Direction::Vertical;

    let hz_bounds = 
        get_width_height_bounds(
            hz_bounds, 
            &widths_heights, 
            hz_handle_width, 
            hz_handle_height, 
            hz_direction);

    let vt_bounds = 
        get_width_height_bounds(
            vt_bounds, 
            &widths_heights, 
            vt_handle_width, 
            vt_handle_height, 
            vt_direction);
    
    let hz_results = vec![
        Rectangle { x: 50.0, y: 50.0, width: 100.0, height: 21.0 },
        Rectangle { x: 150.0, y: 50.0, width: 100.0, height: 21.0 },
        Rectangle { x: 250.0, y: 50.0, width: 100.0, height: 21.0 },
        Rectangle { x: 350.0, y: 50.0, width: 100.0, height: 21.0 }];

    let vt_results = vec![
        Rectangle { x: 50.0, y: 50.0, width: 100.0, height: 100.0 },
        Rectangle { x: 50.0, y: 150.0, width: 100.0, height: 100.0 },
        Rectangle { x: 50.0, y: 250.0, width: 100.0, height: 100.0 },
        Rectangle { x: 50.0, y: 350.0, width: 100.0, height: 100.0 }];

    assert_eq!(hz_results, hz_bounds);
    assert_eq!(vt_results, vt_bounds);

}