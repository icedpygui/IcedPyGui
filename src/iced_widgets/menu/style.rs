//! Change the appearance of menu bars and their menus.
use iced::{Background, Border, Color, Padding, Shadow, Theme, Vector};

/// The appearance of a menu bar and its menus.
#[derive(Debug, Clone, Copy)]
pub struct Appearance {
    /// The background of the menu bar.
    pub bar_background: Background,
    /// The border of the menu bar.
    pub bar_border: Border,
    /// The shadow of the menu bar.
    pub bar_shadow: Shadow,
    /// Expand the menu bar background
    pub bar_background_expand: Padding,

    /// The background of the menus.
    pub menu_background: Background,
    /// The border of the menus.
    pub menu_border: Border,
    /// The shadow of the menus
    pub menu_shadow: Shadow,
    /// Expand the menu background
    pub menu_background_expand: Padding,

    /// The backgraound of the path
    pub path: Background,
    /// The border of the path
    pub path_border: Border,
}
impl std::default::Default for Appearance {
    fn default() -> Self {
        Self {
            bar_background: Color::from([0.85; 3]).into(),
            bar_border: Border {
                radius: [8.0; 4].into(),
                ..Default::default()
            },
            bar_shadow: Shadow::default(),
            bar_background_expand: [5; 4].into(),

            menu_background: Color::from([0.85; 3]).into(),
            menu_border: Border {
                radius: [8.0; 4].into(),
                ..Default::default()
            },
            menu_shadow: Shadow {
                color: Color::from([0.0, 0.0, 0.0, 0.5]),
                offset: Vector::ZERO,
                blur_radius: 10.0,
            },
            menu_background_expand: [5; 4].into(),
            path: Color::from([0.3; 3]).into(),
            path_border: Border {
                radius: [6.0; 4].into(),
                ..Default::default()
            },
        }
    }
}

/// The style sheet of a menu bar and its menus.
pub trait StyleSheet {
    /// The supported style of the [`StyleSheet`].
    type Style: Default;

    /// Produces the [`Appearance`] of a menu bar and its menus.
    fn appearance(&self, style: &Self::Style) -> Appearance;
}

/// The style of a menu bar and its menus
#[derive(Default)]
#[allow(missing_debug_implementations)]
pub enum MenuBarStyle {
    /// The default style.
    #[default]
    Default,
    /// A [`Theme`] that uses a `Custom` palette.
    Custom(Box<dyn StyleSheet<Style = Theme>>),
}

impl<F: Fn(&Theme) -> Appearance + 'static> From<F> for MenuBarStyle {
    fn from(f: F) -> Self {
        Self::Custom(Box::new(f))
    }
}

impl<F: Fn(&Theme) -> Appearance> StyleSheet for F {
    type Style = Theme;

    fn appearance(&self, style: &Self::Style) -> Appearance {
        (self)(style)
    }
}

impl StyleSheet for Theme {
    type Style = MenuBarStyle;

    fn appearance(&self, style: &Self::Style) -> Appearance {
        let palette = self.extended_palette();

        match style {
            MenuBarStyle::Default => Appearance {
                bar_background: palette.background.base.color.into(),
                menu_background: palette.background.base.color.into(),
                path: palette.primary.weak.color.into(),
                ..Default::default()
            },
            MenuBarStyle::Custom(c) => c.appearance(self),
        }
    }
}
