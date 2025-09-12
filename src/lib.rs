pub mod css;
mod utils;

#[allow(non_camel_case_types)]
#[deprecated(
    since = "1.2.0",
    note = "Please use `css::{color, colors, Color, Colors, COLORS_DATA}` instead, This will no longer exist above version 1.3.0 ."
)]
pub type color = css::color;

#[deprecated(
    since = "1.2.0",
    note = "Please use `css::colors` instead, This will no longer exist above version 1.3.0 ."
)]
pub mod colors {
    pub use crate::css::colors::*;
}

#[allow(non_snake_case)]
#[deprecated(
    since = "1.2.0",
    note = "Please use `css::Color` instead, This will no longer exist above version 1.3.0 ."
)]
pub type Color = css::Color;

#[allow(non_snake_case)]
#[deprecated(
    since = "1.2.0",
    note = "Please use `css::Colors` instead, This will no longer exist above version 1.3.0 ."
)]
pub mod Colors {
    pub use crate::css::Colors::*;
}

#[deprecated(
    since = "1.2.0",
    note = "Please use `css::COLORS_DATA` instead, This will no longer exist above version 1.3.0 ."
)]
pub static COLORS_DATA: [(&str, [u8; 3]); 148] = css::COLORS_DATA;
