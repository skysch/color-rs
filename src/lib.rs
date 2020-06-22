////////////////////////////////////////////////////////////////////////////////
// Color -- a color library for color editors
////////////////////////////////////////////////////////////////////////////////
// Copyright 2020 Skylor R. Schermer
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! Color library modules.
//!
//! # Features
//!
//! | Feature | Description |
//! | ------- | ----------- |
//! | "serde" | Enables serialization and deserialization of data using [serde](https://crates.io/crates/serde). |
//!
//! By default, there are no features enabled.
////////////////////////////////////////////////////////////////////////////////
// #![doc(html_root_url = "https://docs.rs/color/0.2.1")]
#![warn(anonymous_parameters)]
#![warn(bad_style)]
#![warn(bare_trait_objects)]
#![warn(const_err)]
#![warn(dead_code)]
#![warn(elided_lifetimes_in_paths)]
#![warn(improper_ctypes)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![warn(no_mangle_generic_items)]
#![warn(non_shorthand_field_patterns)]
#![warn(overflowing_literals)]
#![warn(path_statements)]
#![warn(patterns_in_fns_without_body)]
#![warn(private_in_public)]
#![warn(rust_2018_idioms)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unconditional_recursion)]
#![warn(unreachable_pub)]
#![warn(unused)]
#![warn(unused_allocation)]
#![warn(unused_comparisons)]
#![warn(unused_parens)]
#![warn(unused_qualifications)]
#![warn(unused_results)]
#![warn(variant_size_differences)]
#![warn(while_true)]


// Module declarations.
pub mod utilities;
pub mod cmyk;
pub mod hsl;
pub mod hsv;
pub mod rgb;
pub mod xyz;

#[cfg(test)]
mod tests;

// Local imports.
use crate::utilities::clamped;

// External library imports.
#[cfg(feature = "serde")]
use serde::{
    Serialize,
    Deserialize,
};

// Standard library imports.
use std::fmt;

// Exports.
pub use crate::cmyk::Cmyk;
pub use crate::hsl::Hsl;
pub use crate::hsv::Hsv;
pub use crate::rgb::Rgb;
pub use crate::xyz::Xyz;


// /// Standard SRGB gamma correction matrix. This gives the relative intensities 
// /// of each RGB color component.
// const SRGB_GAMMA_CORRECTION: [[f32; 3]; 3] = [
//     [0.2125, 0.0,     0.0   ],
//     [0.0,     0.7154, 0.0   ],
//     [0.0,     0.0,    0.0721]
// ];


////////////////////////////////////////////////////////////////////////////////
// Color
////////////////////////////////////////////////////////////////////////////////
/// An RGB encoded color with extension methods.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(Transparent))]
pub struct Color {
    /// The base RGB format of the color.
    rgb: Rgb
}

impl Color {
    /// Constructs a new `Color`.
    ///
    /// [`Rgb`]: rgb/struct.Rgb.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Color::new(Rgb {r: 127, g: 255, b: 64});
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn new<C>(color: C) -> Self where C: Into<Rgb> {
        Color {
            rgb: color.into(),
        }
    }

    /// Returns the red [`Rgb`] component of the color.
    ///
    /// [`Rgb`]: rgb/struct.Rgb.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Color::new(Rgb {r: 127, g: 255, b: 64});
    ///
    /// assert_eq!(color.red(), 127);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn red(&self) -> u8 {
        self.rgb.r
    }
    
    /// Returns the green [`Rgb`] component of the color.
    ///
    /// [`Rgb`]: rgb/struct.Rgb.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Color::new(Rgb {r: 127, g: 255, b: 64});
    ///
    /// assert_eq!(color.green(), 255);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn green(&self) -> u8 {
        self.rgb.g
    }
    
    /// Returns the blue [`Rgb`] component of the color.
    ///
    /// [`Rgb`]: rgb/struct.Rgb.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Color::new(Rgb {r: 127, g: 255, b: 64});
    ///
    /// assert_eq!(color.blue(), 64);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn blue(&self) -> u8 {
        self.rgb.b
    }

    /// Returns the cyan [`Cymk`] component of the color.
    ///
    /// [`Cymk`]: cmyk/struct.Cymk.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Color::new(Rgb {r: 127, g: 255, b: 64});
    ///
    /// assert_eq!(color.cyan(), 128);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn cyan(&self) -> u8 {
        Cmyk::from(self.rgb).c
    }

    /// Returns the magenta [`Cymk`] component of the color.
    ///
    /// [`Cymk`]: cmyk/struct.Cymk.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Color::new(Rgb {r: 127, g: 255, b: 64});
    ///
    /// assert_eq!(color.magenta(), 0);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn magenta(&self) -> u8 {
        Cmyk::from(self.rgb).m
    }

    /// Returns the yellow [`Cymk`] component of the color.
    ///
    /// [`Cymk`]: cmyk/struct.Cymk.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Color::new(Rgb {r: 127, g: 255, b: 64});
    ///
    /// assert_eq!(color.yellow(), 191);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn yellow(&self) -> u8 {
        Cmyk::from(self.rgb).y
    }

    /// Returns the key [`Cymk`] component of the color.
    ///
    /// [`Cymk`]: cmyk/struct.Cymk.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Color::new(Rgb {r: 127, g: 255, b: 64});
    ///
    /// assert_eq!(color.key(), 0);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn key(&self) -> u8 {
        Cmyk::from(self.rgb).k
    }

    /// Returns the hue [`Hsl`] component of the color in degrees.
    ///
    /// [`Hsl`]: hsl/struct.Cymk.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Color::new(Rgb {r: 127, g: 255, b: 64});
    ///
    /// assert_eq!(color.hue(), 100.20943);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn hue(&self) -> f32 {
        Hsl::from(self.rgb).hue()
    }

    /// Returns the saturation [`Hsl`] component of the color as a ratio.
    ///
    /// [`Hsl`]: hsl/struct.Cymk.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Color::new(Rgb {r: 127, g: 255, b: 64});
    ///
    /// assert_eq!(color.hsl_saturation(), 0.5987461);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn hsl_saturation(&self) -> f32 {
        Hsl::from(self.rgb).saturation()
    }

    /// Returns the saturation [`Hsv`] component of the color as a ratio.
    ///
    /// [`Hsv`]: hsv/struct.Cymk.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Color::new(Rgb {r: 127, g: 255, b: 64});
    ///
    /// assert_eq!(color.hsv_saturation(), 0.7490196);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn hsv_saturation(&self) -> f32 {
        Hsv::from(self.rgb).saturation()
    }

    /// Returns the lightness [`Hsl`] component of the color as a ratio.
    ///
    /// [`Hsl`]: hsl/struct.Cymk.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Color::new(Rgb {r: 127, g: 255, b: 64});
    ///
    /// assert_eq!(color.lightness(), 0.6254902);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn lightness(&self) -> f32 {
        Hsl::from(self.rgb).lightness()
    }

    /// Returns the value [`Hsv`] component of the color as a ratio.
    ///
    /// [`Hsv`]: hsv/struct.Cymk.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Color::new(Rgb {r: 127, g: 255, b: 64});
    ///
    /// assert_eq!(color.value(), 1.0);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn value(&self) -> f32 {
        Hsv::from(self.rgb).value()
    }

    /// Sets the red [`Rgb`] component of the color.
    ///
    /// [`Rgb`]: rgb/struct.Rgb.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let mut color = Color::new(Rgb {r: 127, g: 255, b: 64});
    ///
    /// color.set_red(80);
    ///
    /// assert_eq!(color.red(), 80);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn set_red(&mut self, value: u8) {
        self.rgb.r = value;
    }

    /// Sets the green [`Rgb`] component of the color.
    ///
    /// [`Rgb`]: rgb/struct.Rgb.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let mut color = Color::new(Rgb {r: 127, g: 255, b: 64});
    ///
    /// color.set_green(80);
    ///
    /// assert_eq!(color.green(), 80);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn set_green(&mut self, value: u8) {
        self.rgb.g = value;
    }

    /// Sets the blue [`Rgb`] component of the color.
    ///
    /// [`Rgb`]: rgb/struct.Rgb.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let mut color = Color::new(Rgb {r: 127, g: 255, b: 64});
    ///
    /// color.set_blue(80);
    ///
    /// assert_eq!(color.blue(), 80);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn set_blue(&mut self, value: u8) {
        self.rgb.b = value;
    }

    /// Sets the cyan [`Cymk`] component of the color.
    ///
    /// Note that the CYMK color space has more degrees of freedom than
    /// necessary, so multiple CYMK values may denote the same color. Thus 
    /// setting a component value using `set_cyan` may not result in a
    /// color with the given value in the cyan component.
    ///
    /// [`Cymk`]: cymk/struct.Cymk.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let mut color = Color::new(Rgb {r: 127, g: 255, b: 64});
    ///
    /// color.set_cyan(80);
    ///
    /// assert_eq!(color.cyan(), 80);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn set_cyan(&mut self, value: u8) {
        let mut t = Cmyk::from(self.rgb);
        t.c = value;
        self.rgb = Rgb::from(t);
    }

    /// Sets the magenta [`Cymk`] component of the color. 
    ///
    /// Note that the CYMK color space has more degrees of freedom than
    /// necessary, so multiple CYMK values may denote the same color. Thus 
    /// setting a component value using `set_magenta` may not result in a
    /// color with the given value in the magenta component.
    ///
    /// [`Cymk`]: cymk/struct.Cymk.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let mut color = Color::new(Rgb {r: 127, g: 255, b: 64});
    ///
    /// color.set_magenta(80);
    ///
    /// assert_eq!(color, Rgb {r: 127, g: 175, b: 64}.into());
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn set_magenta(&mut self, value: u8) {
        let mut t = Cmyk::from(self.rgb);
        t.m = value;
        self.rgb = Rgb::from(t);
    }

    /// Sets the yellow [`Cymk`] component of the color.
    ///
    /// Note that the CYMK color space has more degrees of freedom than
    /// necessary, so multiple CYMK values may denote the same color. Thus 
    /// setting a component value using `set_yellow` may not result in a
    /// color with the given value in the yellow component.
    ///
    /// [`Cymk`]: cymk/struct.Cymk.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let mut color = Color::new(Rgb {r: 127, g: 255, b: 64});
    ///
    /// color.set_yellow(80);
    ///
    /// assert_eq!(color, Rgb {r: 127, g: 255, b: 175}.into());
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn set_yellow(&mut self, value: u8) {
        let mut t = Cmyk::from(self.rgb);
        t.y = value;
        self.rgb = Rgb::from(t);
    }

    /// Sets the key [`Cymk`] component of the color.
    ///
    /// Note that the CYMK color space has more degrees of freedom than
    /// necessary, so multiple CYMK values may denote the same color. Thus 
    /// setting a component value using `set_key` may not result in a
    /// color with the given value in the key component.
    ///
    /// [`Cymk`]: cymk/struct.Cymk.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let mut color = Color::new(Rgb {r: 127, g: 255, b: 64});
    ///
    /// color.set_key(80);
    ///
    /// assert_eq!(color, Rgb {r: 87, g: 175, b: 44}.into());
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn set_key(&mut self, value: u8) {
        let mut t = Cmyk::from(self.rgb);
        t.k = value;
        self.rgb = Rgb::from(t);
    }

    /// Sets the hue [`Hsl`]/['Hsv'] component of the color in degrees.
    ///
    /// Note that the HSL/HSV color space has more degrees of freedom than
    /// necessary, so multiple HSL/HSV values may denote the same color. Thus 
    /// setting a component value using `set_hue` may not result in a
    /// color with the given value in the hue component.
    ///
    /// [`Hsl`]: hsl/struct.Hsl.html
    /// [`Hsv`]: hsv/struct.Hsv.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let mut color = Color::new(Rgb {r: 127, g: 255, b: 64});
    ///
    /// color.set_hue(0.24);
    ///
    /// assert_eq!(color, Rgb {r: 255, g: 64, b: 63}.into());
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn set_hue(&mut self, value: f32) {
        let mut t = Hsv::from(self.rgb);
        t.set_hue(value);
        self.rgb = Rgb::from(t);
    }

    /// Sets the saturation [`Hsl`] component of the color as a ratio.
    ///
    /// Note that the HSL/HSV color space has more degrees of freedom than
    /// necessary, so multiple HSL/HSV values may denote the same color. Thus 
    /// setting a component value using `set_hsl_saturation` may not result in a
    /// color with the given value in the hsl_saturation component.
    ///
    /// [`Hsl`]: hsl/struct.Hsl.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let mut color = Color::new(Rgb {r: 127, g: 255, b: 64});
    ///
    /// color.set_hsl_saturation(0.24);
    ///
    /// assert_eq!(color, Rgb {r: 151, g: 181, b: 136}.into());
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn set_hsl_saturation(&mut self, value: f32) {
        let mut t = Hsl::from(self.rgb);
        t.set_saturation(value);
        self.rgb = Rgb::from(t);
    }

    /// Sets the saturation [`Hsv`] component of the color as a ratio.
    ///
    /// Note that the HSL/HSV color space has more degrees of freedom than
    /// necessary, so multiple HSL/HSV values may denote the same color. Thus 
    /// setting a component value using `set_hsv_saturation` may not result in a
    /// color with the given value in the hsv_saturation component.
    ///
    /// [`Hsv`]: hsv/struct.Hsv.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let mut color = Color::new(Rgb {r: 127, g: 255, b: 64});
    ///
    /// color.set_hsv_saturation(0.24);
    ///
    /// assert_eq!(color, Rgb {r: 213, g: 255, b: 193}.into());
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn set_hsv_saturation(&mut self, value: f32) {
        let mut t = Hsv::from(self.rgb);
        t.set_saturation(value);
        self.rgb = Rgb::from(t);
    }

    /// Sets the lightness [`Hsl`] component of the color as a ratio.
    ///
    /// Note that the HSL/HSV color space has more degrees of freedom than
    /// necessary, so multiple HSL/HSV values may denote the same color. Thus 
    /// setting a component value using `set_lightness` may not result in a
    /// color with the given value in the lightness component.
    ///
    /// [`Hsl`]: hsl/struct.Hsl.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let mut color = Color::new(Rgb {r: 127, g: 255, b: 64});
    ///
    /// color.set_lightness(0.24);
    ///
    /// assert_eq!(color, Rgb {r: 48, g: 97, b: 24}.into());
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn set_lightness(&mut self, value: f32) {
        let mut t = Hsl::from(self.rgb);
        t.set_lightness(value);
        self.rgb = Rgb::from(t);
    }

    /// Sets the value [`Hsv`] component of the color as a ratio.
    ///
    /// Note that the HSL/HSV color space has more degrees of freedom than
    /// necessary, so multiple HSL/HSV values may denote the same color. Thus 
    /// setting a component value using `set_value` may not result in a
    /// color with the given value in the value component.
    ///
    /// [`Hsv`]: hsv/struct.Hsv.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let mut color = Color::new(Rgb {r: 127, g: 255, b: 64});
    ///
    /// color.set_value(0.24);
    ///
    /// assert_eq!(color, Rgb {r: 30, g: 61, b: 15}.into());
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn set_value(&mut self, value: f32) {
        let mut t = Hsv::from(self.rgb);
        t.set_value(value);
        self.rgb = Rgb::from(t);
    }

    /// Shifts the hue [`Hsl`]/['Hsv'] component of the color by the given 
    /// number of degrees.
    ///
    /// Note that the HSL/HSV color space has more degrees of freedom than
    /// necessary, so multiple HSL/HSV values may denote the same color. Thus 
    /// setting a component value using `shift_hue` may not result in a
    /// color with the given value in the hue component.
    ///
    /// [`Hsl`]: hsl/struct.Hsl.html
    /// [`Hsv`]: hsv/struct.Hsv.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let mut color = Color::new(Rgb {r: 127, g: 255, b: 64});
    ///
    /// color.shift_hue(65.0);
    ///
    /// assert_eq!(color, Rgb {r: 63, g: 255, b: 207}.into());
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn shift_hue(&mut self, degrees: f32) {
        let h = self.hue();
        self.set_hue(h + degrees);
    }

    /// Increases the saturation [`Hsl`] component of the color by the given 
    /// ratio.
    ///
    /// Note that the HSL/HSV color space has more degrees of freedom than
    /// necessary, so multiple HSL/HSV values may denote the same color. Thus 
    /// setting a component value using `hsl_saturate` may not result in a
    /// color with the given value in the saturation component.
    ///
    /// [`Hsl`]: hsl/struct.Hsl.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let mut color = Color::new(Rgb {r: 127, g: 255, b: 64});
    ///
    /// color.hsl_saturate(0.34);
    ///
    /// assert_eq!(color, Rgb {r: 132, g: 235, b: 82}.into());
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn hsl_saturate(&mut self, value: f32) {
        let s = self.hsl_saturation();
        let v = clamped(value, 0.0, 1.0);
        self.set_hsl_saturation(s + (s * v));
    }

    /// Decreases the saturation [`Hsl`] component of the color by the given 
    /// ratio.
    ///
    /// Note that the HSL/HSV color space has more degrees of freedom than
    /// necessary, so multiple HSL/HSV values may denote the same color. Thus 
    /// setting a component value using `hsl_saturate` may not result in a
    /// color with the given value in the saturation component.
    ///
    /// [`Hsl`]: hsl/struct.Hsl.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let mut color = Color::new(Rgb {r: 127, g: 255, b: 64});
    ///
    /// color.hsl_desaturate(0.34);
    ///
    /// assert_eq!(color, Rgb {r: 145, g: 196, b: 121}.into());
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn hsl_desaturate(&mut self, value: f32) {
        let s = self.hsl_saturation();
        let v = clamped(value, 0.0, 1.0);
        self.set_hsl_saturation(s - (s * v));
    }

    /// Increases the saturation [`Hsv`] component of the color by the given 
    /// ratio.
    ///
    /// Note that the HSL/HSV color space has more degrees of freedom than
    /// necessary, so multiple HSL/HSV values may denote the same color. Thus 
    /// setting a component value using `hsv_saturate` may not result in a
    /// color with the given value in the saturation component.
    ///
    /// [`Hsv`]: hsv/struct.Hsv.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let mut color = Color::new(Rgb {r: 127, g: 255, b: 64});
    ///
    /// color.hsv_saturate(0.34);
    ///
    /// assert_eq!(color, Rgb {r: 84, g: 255, b: 0}.into());
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn hsv_saturate(&mut self, value: f32) {
        let s = self.hsv_saturation();
        let v = clamped(value, 0.0, 1.0);
        self.set_hsv_saturation(s + (s * v));
    }

    /// Decreases the saturation [`Hsv`] component of the color by the given 
    /// ratio.
    ///
    /// Note that the HSL/HSV color space has more degrees of freedom than
    /// necessary, so multiple HSL/HSV values may denote the same color. Thus 
    /// setting a component value using `hsv_saturate` may not result in a
    /// color with the given value in the saturation component.
    ///
    /// [`Hsv`]: hsv/struct.Hsv.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let mut color = Color::new(Rgb {r: 127, g: 255, b: 64});
    ///
    /// color.hsv_desaturate(0.34);
    ///
    /// assert_eq!(color, Rgb {r: 170, g: 255, b: 128}.into());
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn hsv_desaturate(&mut self, value: f32) {
        let s = self.hsv_saturation();
        let v = clamped(value, 0.0, 1.0);
        self.set_hsv_saturation(s - (s * v));
    }

    /// Increases the lightness [`Hsl`] component of the color by the given 
    /// ratio.
    ///
    /// Note that the HSL/HSV color space has more degrees of freedom than
    /// necessary, so multiple HSL/HSV values may denote the same color. Thus 
    /// setting a component value using `lighten` may not result in a
    /// color with the given value in the lightness component.
    ///
    /// [`Hsl`]: hsl/struct.Hsl.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let mut color = Color::new(Rgb {r: 127, g: 255, b: 64});
    ///
    /// color.lighten(0.34);
    ///
    /// assert_eq!(color, Rgb {r: 205, g: 238, b: 189}.into());
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn lighten(&mut self, value: f32) {
        let l = self.lightness();
        let v = clamped(value, 0.0, 1.0);
        self.set_lightness(l + (l * v));
    }

    /// Decreases the lightness [`Hsl`] component of the color by the given 
    /// ratio.
    ///
    /// Note that the HSL/HSV color space has more degrees of freedom than
    /// necessary, so multiple HSL/HSV values may denote the same color. Thus 
    /// setting a component value using `darken` may not result in a
    /// color with the given value in the lightness component.
    ///
    /// [`Hsl`]: hsl/struct.Hsl.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let mut color = Color::new(Rgb {r: 127, g: 255, b: 64});
    ///
    /// color.darken(0.34);
    ///
    /// assert_eq!(color, Rgb {r: 83, g: 168, b: 42}.into());
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn darken(&mut self, value: f32) {
        let l = self.lightness();
        let v = clamped(value, 0.0, 1.0);
        self.set_lightness(l - (l * v));
    }

    /// Returns an array containing the [`[R, G, B]`] component octets.
    ///
    /// [`[R, G, B]`]: rgb/struct.Rgb.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Color::new(Rgb {r: 127, g: 255, b: 64});
    ///
    /// let octets = color.rgb_octets();
    ///
    /// assert_eq!(octets, [127u8, 255u8, 64u8]);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn rgb_octets(&self) -> [u8; 3] {
        self.rgb.octets()
    }

    /// Returns an array containing the [`[C, M, Y, K]`] component octets.
    ///
    /// [`[C, M, Y, K]`]: cmyk/struct.Cmyk.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Color::new(Rgb {r: 127, g: 255, b: 64});
    ///
    /// let octets = color.cmyk_octets();
    ///
    /// assert_eq!(octets, [128u8, 0u8, 191u8, 0u8]);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn cmyk_octets(&self) -> [u8; 4] {
        Cmyk::from(self.rgb).octets()
    }

    /// Returns an array containing the [`[H, S, L]`] components.
    ///
    /// [`[H, S, L]`]: hsl/struct.Hsl.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Color::new(Rgb {r: 127, g: 255, b: 64});
    ///
    /// let components = color.hsl_components();
    ///
    /// assert_eq!(components, [100.20943f32, 0.5987461f32, 0.6254902f32]);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn hsl_components(&self) -> [f32; 3] {
        Hsl::from(self.rgb).components()
    }

    /// Returns an array containing the [`[H, S, V]`] components.
    ///
    /// [`[H, S, V]`]: hsv/struct.Hsv.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Color::new(Rgb {r: 127, g: 255, b: 64});
    ///
    /// let components = color.hsv_components();
    ///
    /// assert_eq!(components, [100.20943f32, 0.7490196f32, 1.0f32]);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn hsv_components(&self) -> [f32; 3] {
        Hsv::from(self.rgb).components()
    }

    /// Returns an array containing the [`[R, G, B]`] component ratios.
    ///
    /// [`[R, G, B]`]: rgb/struct.Rgb.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Color::new(Rgb {r: 127, g: 255, b: 64});
    ///
    /// let ratios = color.rgb_ratios();
    ///
    /// assert_eq!(ratios, [0.49803922f32, 1.0f32, 0.2509804f32]);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn rgb_ratios(&self) -> [f32; 3] {
        self.rgb.ratios()
    }

    /// Returns an array containing the [`[C, M, Y, K]`] component ratios.
    ///
    /// [`[C, M, Y, K]`]: cmyk/struct.Cmyk.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Color::new(Rgb {r: 127, g: 255, b: 64});
    ///
    /// let ratios = color.cmyk_ratios();
    ///
    /// assert_eq!(ratios, [0.5019608f32, 0.0f32, 0.7490196f32, 0.0f32]);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn cmyk_ratios(&self) -> [f32; 4] {
        Cmyk::from(self.rgb).ratios()
    }

    /// Returns the [`Rgb`] hex code of the color.
    ///
    /// [`Rgb`]: rgb/struct.Rgb.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Color::new(Rgb {r: 127, g: 255, b: 64});
    ///
    /// assert_eq!(color.rgb_hex(), 0x7FFF40);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn rgb_hex(&self) -> u32 {
        self.rgb.hex()
    }

    /// Returns the [`Cmyk`] hex code of the color.
    ///
    /// [`Cmyk`]: cmyk/struct.Cmyk.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Color::new(Rgb {r: 127, g: 255, b: 64});
    ///
    /// assert_eq!(color.cmyk_hex(), 0x8000BF00);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn cmyk_hex(&self) -> u32 {
        Cmyk::from(self.rgb).hex()
    }

    /// Performs an [`Rgb`] component-wise linear interpolation between given 
    /// colors, returning the color located at the ratio given by `amount`,
    /// which is clamped between 1 and 0.
    ///
    /// [`Rgb`]: rgb/struct.Rgb.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let color_a = Color::new(Rgb {r: 127, g: 255, b: 64});
    /// let color_b = Color::new(Rgb {r: 15, g: 144, b: 99});
    ///
    /// let lerp_color = Color::rgb_lerp(color_a, color_b, 0.65);
    ///
    /// assert_eq!(lerp_color, Rgb {r: 54, g: 182, b: 86}.into());
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn rgb_lerp<C>(start: C, end: C, amount: f32) -> Self 
        where C: Into<Rgb> + Sized
    {
        Rgb::lerp(start.into(), end.into(), amount).into()
    }

    /// Performs an [`Cmyk`] component-wise linear interpolation between given 
    /// colors, returning the color located at the ratio given by `amount`,
    /// which is clamped between 1 and 0.
    ///
    /// [`Cmyk`]: cmyk/struct.Cmyk.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let color_a = Color::new(Rgb {r: 127, g: 255, b: 64});
    /// let color_b = Color::new(Rgb {r: 15, g: 144, b: 99});
    ///
    /// let lerp_color = Color::cmyk_lerp(color_a, color_b, 0.65);
    ///
    /// assert_eq!(lerp_color, Rgb {r: 44, g: 183, b: 98}.into());
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn cmyk_lerp<C>(start: C, end: C, amount: f32) -> Self 
        where C: Into<Cmyk> + Sized
    {
        Cmyk::lerp(start.into(), end.into(), amount).into()
    }

    /// Performs an [`Hsl`] component-wise linear interpolation between given 
    /// colors, returning the color located at the ratio given by `amount`,
    /// which is clamped between 1 and 0.
    ///
    /// [`Hsl`]: hsl/struct.Hsl.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let color_a = Color::new(Rgb {r: 127, g: 255, b: 64});
    /// let color_b = Color::new(Rgb {r: 15, g: 144, b: 99});
    ///
    /// let lerp_color = Color::hsl_lerp(color_a, color_b, 0.65);
    ///
    /// assert_eq!(lerp_color, Rgb {r: 28, g: 186, b: 76}.into());
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn hsl_lerp<C>(start: C, end: C, amount: f32) -> Self 
        where C: Into<Hsl> + Sized
    {
        Hsl::lerp(start.into(), end.into(), amount).into()
    }

    /// Performs an [`Hsv`] component-wise linear interpolation between given 
    /// colors, returning the color located at the ratio given by `amount`,
    /// which is clamped between 1 and 0.
    ///
    /// [`Hsv`]: hsv/struct.Hsv.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let color_a = Color::new(Rgb {r: 127, g: 255, b: 64});
    /// let color_b = Color::new(Rgb {r: 15, g: 144, b: 99});
    ///
    /// let lerp_color = Color::hsv_lerp(color_a, color_b, 0.65);
    ///
    /// assert_eq!(lerp_color, Rgb {r: 28, g: 182, b: 75}.into());
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn hsv_lerp<C>(start: C, end: C, amount: f32) -> Self 
        where C: Into<Hsv> + Sized
    {
        Hsv::lerp(start.into(), end.into(), amount).into()
    }

    /// Returns the distance between the given colors in [`Rgb`] color space.
    ///
    /// [`Rgb`]: rgb/struct.Rgb.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let color_a = Color::new(Rgb {r: 127, g: 255, b: 64});
    /// let color_b = Color::new(Rgb {r: 15, g: 144, b: 99});
    ///
    /// assert_eq!(Color::rgb_distance(color_a, color_b), 161.52399);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn rgb_distance<C>(start: C, end: C) -> f32 
        where C: Into<Rgb> + Sized
    {
        Rgb::distance(start.into(), end.into())
    }

    /// Returns the distance between the given colors in [`Cmyk`] color space.
    ///
    /// [`Cmyk`]: cmyk/struct.Cmyk.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let color_a = Color::new(Rgb {r: 127, g: 255, b: 64});
    /// let color_b = Color::new(Rgb {r: 15, g: 144, b: 99});
    ///
    /// assert_eq!(Color::cmyk_distance(color_a, color_b), 186.12361);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn cmyk_distance<C>(start: C, end: C) -> f32 
        where C: Into<Cmyk> + Sized
    {
        Cmyk::distance(start.into(), end.into())
    }

    /// Returns the distance between the given colors in [`Hsl`] color space.
    ///
    /// [`Hsl`]: hsl/struct.Hsl.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let color_a = Color::new(Rgb {r: 127, g: 255, b: 64});
    /// let color_b = Color::new(Rgb {r: 15, g: 144, b: 99});
    ///
    /// assert_eq!(Color::hsl_distance(color_a, color_b), 0.71319157);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn hsl_distance<C>(start: C, end: C) -> f32 
        where C: Into<Hsl> + Sized
    {
        Hsl::distance(start.into(), end.into())
    }

    /// Returns the distance between the given colors in [`Hsv`] color space.
    ///
    /// [`Hsv`]: hsv/struct.Hsv.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::{Color, Rgb};
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let color_a = Color::new(Rgb {r: 127, g: 255, b: 64});
    /// let color_b = Color::new(Rgb {r: 15, g: 144, b: 99});
    ///
    /// assert_eq!(Color::hsv_distance(color_a, color_b), 1.1794227);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn hsv_distance<C>(start: C, end: C) -> f32 
        where C: Into<Hsv> + Sized
    {
        Hsv::distance(start.into(), end.into())
    }
}



impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}


impl fmt::UpperHex for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "#{:X}", self.rgb)
    }
}


impl fmt::LowerHex for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "#{:x}", self.rgb)
    }
}


impl From<Cmyk> for Color {
    fn from(cmyk: Cmyk) -> Color {
        Color {rgb: Rgb::from(cmyk)}
    }
}

impl From<Hsl> for Color {
    fn from(hsl: Hsl) -> Color {
        Color {rgb: Rgb::from(hsl)}
    }
}

impl From<Rgb> for Color {
    fn from(rgb: Rgb) -> Color {
        Color {rgb: rgb}
    }
}

impl From<Hsv> for Color {
    fn from(hsv: Hsv) -> Color {
        Color {rgb: Rgb::from(hsv)}
    }
}

impl From<Xyz> for Color {
    fn from(xyz: Xyz) -> Color {
        Color {rgb: Rgb::from(xyz)}
    }
}

/// Converts the color to an RGB vector.
impl From<Color> for [f32; 3] {
    fn from(color: Color) -> Self {
        Rgb::from(color).into()
    }
}

/// Converts the color to an RGBA vector.
impl From<Color> for [f32; 4] {
    fn from(color: Color) -> Self {
        Rgb::from(color).into()
    }
}

/// Converts the color to an Rgb.
impl From<Color> for Rgb {
    fn from(color: Color) -> Self {
        color.rgb
    }
}

/// Converts the color to a Cmyk.
impl From<Color> for Cmyk {
    fn from(color: Color) -> Self {
        color.rgb.into()
    }
}

/// Converts the color to a Hsl.
impl From<Color> for Hsl {
    fn from(color: Color) -> Self {
        color.rgb.into()
    }
}
/// Converts the color to a Hsv.
impl From<Color> for Hsv {
    fn from(color: Color) -> Self {
        color.rgb.into()
    }
}
