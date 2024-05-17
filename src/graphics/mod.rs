//! A module fitting `iced_graphics`.
#![allow(dead_code)]
pub mod bootstrap_icon;
pub mod bootstrap;
pub mod required;
pub mod nerd;

use iced::Font;

/// The default icon font bytes for loading the font into iced.
pub const BOOTSTRAP_FONT_BYTES: &[u8] = include_bytes!("./fonts/bootstrap-icons.ttf");
/// the icon font that has all nerd fonts.
pub const NERD_FONT_BYTES: &[u8] = include_bytes!("./fonts/nerd-icons.ttf");

/// The bootstrap icon font.
pub const BOOTSTRAP_FONT: Font = Font::with_name("bootstrap-icons");
/// The nerd icon font.
pub const NERD_FONT: Font = Font::with_name("Symbols Nerd Font");

/// The default cupertino font bytes for loading the font into the system.
pub const SF_UI_ROUNDED_BYTES: &[u8] = include_bytes!("./fonts/SFUIRounded.ttf");

/// The default cupertino font for alerts and button.
pub const SF_UI_ROUNDED: iced::Font = iced::Font::with_name(".SF UI Rounded");