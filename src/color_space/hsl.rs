// Copyright 2020 Skylor R. Schermer.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
////////////////////////////////////////////////////////////////////////////////
//!
//! Defines a 96-bit HSL color space.
//!
////////////////////////////////////////////////////////////////////////////////

// Local imports.
use crate::Cmyk;
use crate::Hsv;
use crate::Rgb;
use crate::utility::cerp_f32;
use crate::utility::clamped;
use crate::utility::lerp_f32;
use crate::utility::nearly_equal;
use crate::Xyz;

// External library imports.
#[cfg(feature = "serde")]
use serde::Deserialize;
#[cfg(feature = "serde")]
use serde::Serialize;
use tracing::Level;
use tracing::span;

// Standard library imports.
use std::convert::From;
use std::fmt;
use std::f32;


////////////////////////////////////////////////////////////////////////////////
// Hsl
////////////////////////////////////////////////////////////////////////////////
/// The encoded HSL color.
#[derive(Debug, PartialOrd, PartialEq, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Hsl {
    /// The hue component.
    pub(in crate) h: f32,
    /// The saturation component.
    pub(in crate) s: f32,
    /// The lightness component.
    pub(in crate) l: f32,
}


impl Hsl {
    /// Constructs a new `Hsl` color.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Hsl;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Hsl::new(134.0, 0.23, 0.55);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn new(hue: f32, saturation: f32, lightness: f32) -> Self {
        let mut hsl = Hsl {h: 0.0, s: 0.0, l: 0.0};
        hsl.set_hue(hue);
        hsl.set_saturation(saturation);
        hsl.set_lightness(lightness);
        hsl
    }

    /// Returns the hue component of the color.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Hsl;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Hsl::new(134.0, 0.23, 0.55);
    ///
    /// assert_eq!(color.hue(), 134.0);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn hue(&self) -> f32 {
        self.h
    }

    /// Returns the saturation component of the color.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Hsl;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Hsl::new(134.0, 0.23, 0.55);
    ///
    /// assert_eq!(color.saturation(), 0.23);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn saturation(&self) -> f32 {
        self.s
    }

    /// Returns the lightness component of the color.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Hsl;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Hsl::new(134.0, 0.23, 0.55);
    ///
    /// assert_eq!(color.lightness(), 0.55);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn lightness(&self) -> f32 {
        self.l
    }

    /// Sets the hue component of the color in degrees.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Hsl;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let mut color = Hsl::new(134.0, 0.23, 0.55);
    /// color.set_hue(267.0);
    ///
    /// assert_eq!(color.hue(), 267.0);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn set_hue(&mut self, hue: f32) {
        assert!(hue.is_finite());
        self.h = hue % 360.0;
    }

    /// Sets the saturation component of the color as a ratio.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Hsl;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let mut color = Hsl::new(134.0, 0.23, 0.55);
    /// color.set_saturation(0.80);
    ///
    /// assert_eq!(color.saturation(), 0.80);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn set_saturation(&mut self, saturation: f32) {
        self.s = clamped(saturation, 0.0, 1.0);
    }

    /// Sets the lightness component of the color as a ratio.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Hsl;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let mut color = Hsl::new(134.0, 0.23, 0.55);
    /// color.set_lightness(0.80);
    ///
    /// assert_eq!(color.lightness(), 0.80);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn set_lightness(&mut self, lightness: f32) {
        self.l = clamped(lightness, 0.0, 1.0);
    }

    /// Returns an array containing the `[H, S, L]` components.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Hsl;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Hsl::new(134.0, 0.23, 0.55);
    ///
    /// let components = color.components();
    ///
    /// assert_eq!(components, [134.0, 0.23, 0.55]);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn components(&self) -> [f32; 3] {
        [self.h, self.s, self.l]
    }

    /// Performs a component-wise linear interpolation between given colors,
    /// returning the color located at the ratio given by `amount`, which is
    /// clamped between 1 and 0.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Hsl;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let color_a = Hsl::new(34.0, 0.63, 0.35);
    /// let color_b = Hsl::new(322.0, 0.14, 0.95);
    ///
    /// let lerp_color = Hsl::linear_interpolate(color_a, color_b, 0.65);
    ///
    /// assert_eq!(lerp_color, Hsl::new(221.2, 0.3115, 0.74));
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn linear_interpolate<C, D>(start: C, end: D, amount: f32) -> Self 
        where
            C: Into<Self> + Sized,
            D: Into<Self> + Sized,
    {
        let s = start.into();
        let e = end.into();
        Hsl {
            h: lerp_f32(s.h, e.h, amount),
            s: lerp_f32(s.s, e.s, amount),
            l: lerp_f32(s.l, e.l, amount),
        }
    }

    /// Performs a component-wise cubic interpolation between given colors,
    /// returning the color located at the ratio given by `amount`, which is
    /// clamped between 1 and 0. The interpolation function will be consistent
    /// with the slopes given by `start_slope` and `end_slope`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Hsl;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let color_a = Hsl::new(0.24, 0.68, 0.91);
    /// let color_b = Hsl::new(0.84, 0.228, 0.455);
    ///
    /// let lerp_color = Hsl::cubic_interpolate(
    ///     color_a, color_b, 0.0, 0.0, 0.19);
    ///
    /// assert_eq!(lerp_color, Hsl::new(0.29674917, 0.63724893, 0.8669652));
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn cubic_interpolate<C, D>(
        start: C,
        end: D,
        start_slope: f32,
        end_slope: f32,
        amount: f32) -> Self 
        where
            C: Into<Self> + Sized,
            D: Into<Self> + Sized,
    {
        let s = start.into();
        let e = end.into();
        Hsl {
            h: cerp_f32(s.h, e.h, start_slope, end_slope, amount),
            s: cerp_f32(s.s, e.s, start_slope, end_slope, amount),
            l: cerp_f32(s.l, e.l, start_slope, end_slope, amount),
        }
    }

    /// Returns the distance between the given colors in `Hsl` color space.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Hsl;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let color_a = Hsl::new(34.0, 0.63, 0.35);
    /// let color_b = Hsl::new(322.0, 0.14, 0.95);
    ///
    /// assert_eq!(Hsl::distance(color_a, color_b), 0.7027047);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn distance<C>(start: C, end: C) -> f32 
        where C: Into<Self> + Sized
    {
        let s = start.into();
        let e = end.into();
        
        let (shx, shy) = s.h.sin_cos();
        let (ehx, ehy) = e.h.sin_cos();
        let csx = s.l * shx * 2.0;
        let csy = s.l * shy * 2.0;
        let cex = e.l * ehx * 2.0;
        let cey = e.l * ehy * 2.0;

        let s = s.s - e.s;
        let x = csx - cex;
        let y = csy - cey;

        (s*s + x*x + y*y).sqrt() / 6.0f32.sqrt()
    }
}


impl fmt::Display for Hsl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}


////////////////////////////////////////////////////////////////////////////////
// Hsl conversions
////////////////////////////////////////////////////////////////////////////////
impl From<[f32; 3]> for Hsl {
    fn from(components: [f32; 3]) -> Self {
        let span = span!(Level::DEBUG, "Hsl::from<[f32; 3]>");
        let _enter = span.enter();
        
        Hsl::new(
            components[0],
            components[1],
            components[2],
        )
    }
}

impl From<Cmyk> for Hsl {
    fn from(cmyk: Cmyk) -> Self {
        let span = span!(Level::DEBUG, "Hsl::from<Cmyk>");
        let _enter = span.enter();
        
        Hsl::from(Rgb::from(cmyk))
    }
}

impl From<Hsv> for Hsl {
    fn from(hsv: Hsv) -> Self {
        let span = span!(Level::DEBUG, "Hsl::from<Hsv>");
        let _enter = span.enter();
        
        Hsl::from(Rgb::from(hsv))
    }
}

impl From<Rgb> for Hsl {
    fn from(rgb: Rgb) -> Self {
        let span = span!(Level::DEBUG, "Hsl::from<Rgb>");
        let _enter = span.enter();

        // Find min, max, index of max, and delta.
        let ratios = rgb.ratios();
        let (min, max, max_index, _) = ratios
            .iter()
            .fold((ratios[0], ratios[0], 0, 0), |(min, max, i, c), &x| {
                match (x < min, x > max) {
                    (true, false) => (x, max, i, c+1),
                    (false, true) => (min, x, c, c+1),
                    _ => (min, max, i, c+1)
                }
            });
        let delta = max - min;

        // Compute lightness.
        let l = (max + min) / 2.0;
        
        if nearly_equal(delta, 0.0) {
            // No need to compute saturation and hue for grayscale colors.
            Hsl {h: 0.0, s: 0.0, l: l}

        } else {

            // Compute saturation.
            let s = if l > 0.5 {
                delta / (2.0 - delta)
            } else {
                delta / (max + min)
            };

            // Compute hue.
            let mut h = 60.0 * match max_index {
                0 => (ratios[1] - ratios[2]) / delta,
                1 => (ratios[2] - ratios[0]) / delta + 2.0,
                2 => (ratios[0] - ratios[1]) / delta + 4.0,
                _ => unreachable!()
            };

            Hsl::new(h, s, l)
        }

    }
}

impl From<Xyz> for Hsl {
    fn from(xyz: Xyz) -> Self {
        let span = span!(Level::DEBUG, "Hsl::from<Xyz>");
        let _enter = span.enter();

        Hsl::from(Rgb::from(xyz))
    }
}
