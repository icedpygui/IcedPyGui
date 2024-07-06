//! Define the colors of a theme.
#![allow(dead_code)]
use iced::{color, Color};
use pyo3::pyclass;


#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgPaletteSet{
    Dark1,
    Dark2,
    Dark3,
    Dark4,
}

pub fn get_palette_set(set: IpgPaletteSet) -> IpgPalette {
    match set {
        IpgPaletteSet::Dark1 => DARK1,
        IpgPaletteSet::Dark2 => DARK2,
        IpgPaletteSet::Dark3 => DARK3,
        IpgPaletteSet::Dark4 => DARK4,
    }
}

/// A color palette.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IpgPalette {
    pub first: Color,
    pub second: Color,
    pub third: Color,
    pub fourth: Color,
}

impl IpgPalette {
    pub const BLACK1: Self = Self {
        first: color!(0x233142),
        second: color!(0x455d7a),
        third: color!(0xf95959),
        fourth: color!(0xe3e3e3),
    };

    pub const BLACK2: Self = Self {
        first: color!(0x222831),
        second: color!(0x393e46),
        third: color!(0xf96d00),
        fourth: color!(0xf2f2f2),
    };
    pub const BLACK3: Self = Self {
        first: color!(0xf70776),
        second: color!(0xc3195d),
        third: color!(0x680747),
        fourth: color!(0x141010),
    };

    pub const BLACK4: Self = Self {
        first: color!(0xdbd8e3),
        second: color!(0x5c5470),
        third: color!(0x352f44),
        fourth: color!(0x2a2438),
    };
}


pub static DARK1: IpgPalette = IpgPalette::BLACK1;
pub static DARK2: IpgPalette = IpgPalette::BLACK2;
pub static DARK3: IpgPalette = IpgPalette::BLACK3;
pub static DARK4: IpgPalette = IpgPalette::BLACK4;



