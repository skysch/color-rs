// Copyright 2018 Skylor R. Schermer.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
////////////////////////////////////////////////////////////////////////////////
//!
//! Defines a 96-bit HSV color space.
//!
////////////////////////////////////////////////////////////////////////////////
// Local imports.
use crate::cmyk::Cmyk;
use crate::hsl::Hsl;
use crate::rgb::Rgb;
use crate::xyz::Xyz;

use crate::utilities::clamped;
use crate::utilities::lerp_f32;
use crate::utilities::nearly_equal;


// Standard library imports.
use std::convert::From;
use std::fmt;
use std::f32;

////////////////////////////////////////////////////////////////////////////////
// Hsv
////////////////////////////////////////////////////////////////////////////////
/// The encoded HSV color.
#[derive(Debug, PartialOrd, PartialEq, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize), derive(Deserialize))]
pub struct Hsv {
    /// The hue component.
    pub(crate) h: f32,
    /// The saturation component.
    pub(crate) s: f32,
    /// The value component.
    pub(crate) v: f32,
}


impl Hsv {
    /// Constructs a new `Hsv` color.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Hsv;
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Hsv::new(134.0, 0.23, 0.55);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn new(hue: f32, saturation: f32, value: f32) -> Self {
        let mut hsv = Hsv {h: 0.0, s: 0.0, v: 0.0};
        hsv.set_hue(hue);
        hsv.set_saturation(saturation);
        hsv.set_value(value);
        hsv
    }

    /// Returns the hue component of the color.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Hsv;
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Hsv::new(134.0, 0.23, 0.55);
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
    /// # use color::Hsv;
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Hsv::new(134.0, 0.23, 0.55);
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

    /// Returns the value component of the color.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Hsv;
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Hsv::new(134.0, 0.23, 0.55);
    ///
    /// assert_eq!(color.value(), 0.55);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn value(&self) -> f32 {
        self.v
    }

    /// Sets the hue component of the color in degrees.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Hsv;
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let mut color = Hsv::new(134.0, 0.23, 0.55);
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
        self.h = clamped(hue, f32::MIN, f32::MAX) % 360.0;
    }

    /// Sets the saturation component of the color as a ratio.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Hsv;
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let mut color = Hsv::new(134.0, 0.23, 0.55);
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

    /// Sets the value component of the color as a ratio.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Hsv;
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let mut color = Hsv::new(134.0, 0.23, 0.55);
    /// color.set_value(0.80);
    ///
    /// assert_eq!(color.value(), 0.80);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn set_value(&mut self, value: f32) {
        self.v = clamped(value, 0.0, 1.0);
    }

    /// Returns an array containing the `[H, S, V]` components.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Hsv;
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Hsv::new(134.0, 0.23, 0.55);
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
        [self.h, self.s, self.v]
    }

    /// Performs a component-wise linear interpolation between given colors,
    /// returning the color located at the ratio given by `amount`, which is
    /// clamped between 1 and 0.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Hsv;
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let color_a = Hsv::new(34.0, 0.63, 0.35);
    /// let color_b = Hsv::new(322.0, 0.14, 0.95);
    ///
    /// let lerp_color = Hsv::lerp(color_a, color_b, 0.65);
    ///
    /// assert_eq!(lerp_color, Hsv::new(221.2, 0.3115, 0.74));
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn lerp<C>(start: C, end: C, amount: f32) -> Self 
        where C: Into<Self> + Sized
    {
        let s = start.into();
        let e = end.into();
        Hsv {
            h: lerp_f32(s.h, e.h, amount),
            s: lerp_f32(s.s, e.s, amount),
            v: lerp_f32(s.v, e.v, amount),
        }
    }

    /// Returns the distance between the given colors in `Hsv` color space.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Hsv;
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let color_a = Hsv::new(34.0, 0.63, 0.35);
    /// let color_b = Hsv::new(322.0, 0.14, 0.95);
    ///
    /// assert_eq!(Hsv::distance(color_a, color_b), 0.7027047);
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
        let csx = s.v * shx * 2.0;
        let csy = s.v * shy * 2.0;
        let cex = e.v * ehx * 2.0;
        let cey = e.v * ehy * 2.0;

        let s = s.s - e.s;
        let x = csx - cex;
        let y = csy - cey;

        (s*s + x*x + y*y).sqrt() / 6.0f32.sqrt()
    }
}


impl fmt::Display for Hsv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}


////////////////////////////////////////////////////////////////////////////////
// Hsv conversions
////////////////////////////////////////////////////////////////////////////////
impl From<[f32; 3]> for Hsv {
    fn from(components: [f32; 3]) -> Self {
        Hsv {
            h: components[0],
            s: components[1],
            v: components[2],
        }
    }
}

impl From<Cmyk> for Hsv {
    fn from(cmyk: Cmyk) -> Self {
        Hsv::from(Rgb::from(cmyk))
    }
}

impl From<Hsl> for Hsv {
    fn from(hsl: Hsl) -> Self {
        Hsv::from(Rgb::from(hsl))
    }
}

impl From<Rgb> for Hsv {
    fn from(rgb: Rgb) -> Self {
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

        
        if nearly_equal(delta, 0.0) {
            // No need to compute saturation and hue for grayscale colors.
            Hsv {h: 0.0, s: 0.0, v: max}

        } else {

            // Compute saturation.
            let s = if nearly_equal(max, 0.0)  {
                0.0
            } else {
                delta / max
            };

            // Compute hue.
            let mut h = 60.0 * match max_index {
                0 => ((ratios[1] - ratios[2]) / delta) % 6.0,
                1 => (ratios[2] - ratios[0]) / delta + 2.0,
                2 => (ratios[0] - ratios[1]) / delta + 4.0,
                _ => unreachable!()
            };

            // Correct wrapping.
            h %= 360.0;
            if h < 0.0 {h += 360.0};
            
            Hsv {h: h, s: s, v: max}
        }

    }
}

impl From<Xyz> for Hsv {
    fn from(xyz: Xyz) -> Self {
        Hsv::from(Rgb::from(xyz))
    }
}
