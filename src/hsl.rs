// Copyright 2018 Skylor R. Schermer.
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
use cmyk::Cmyk;
use hsv::Hsv;
use rgb::Rgb;
use xyz::Xyz;

use utilities::clamped;
use utilities::lerp_f32;
use utilities::nearly_equal;


// Standard library imports.
use std::convert::From;
use std::fmt;
use std::f32;


////////////////////////////////////////////////////////////////////////////////
// Hsl
////////////////////////////////////////////////////////////////////////////////
/// The encoded HSL color.
#[derive(Debug, PartialOrd, PartialEq, Clone, Copy, Default)]
pub struct Hsl {
    /// The hue component.
    pub(crate) h: f32,
    /// The saturation component.
    pub(crate) s: f32,
    /// The lightness component.
    pub(crate) l: f32,
}


impl Hsl {
    /// Constructs a new `Hsl` color.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Hsl;
    /// # fn example() -> Result<(), Box<Error>> {
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
    /// # fn example() -> Result<(), Box<Error>> {
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
    /// # fn example() -> Result<(), Box<Error>> {
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
    /// # fn example() -> Result<(), Box<Error>> {
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
    /// # fn example() -> Result<(), Box<Error>> {
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
        self.h = clamped(hue, f32::MIN, f32::MAX) % 360.0;
    }

    /// Sets the saturation component of the color as a ratio.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Hsl;
    /// # fn example() -> Result<(), Box<Error>> {
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
    /// # fn example() -> Result<(), Box<Error>> {
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
    /// # fn example() -> Result<(), Box<Error>> {
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
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let color_a = Hsl::new(34.0, 0.63, 0.35);
    /// let color_b = Hsl::new(322.0, 0.14, 0.95);
    ///
    /// let lerp_color = Hsl::lerp(color_a, color_b, 0.65);
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
    pub fn lerp<C>(start: C, end: C, amount: f32) -> Self 
        where C: Into<Self> + Sized
    {
        let s = start.into();
        let e = end.into();
        Hsl {
            h: lerp_f32(s.h, e.h, amount),
            s: lerp_f32(s.s, e.s, amount),
            l: lerp_f32(s.l, e.l, amount),
        }
    }

    /// Returns the distance between the given colors in `Hsl` color space.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Hsl;
    /// # fn example() -> Result<(), Box<Error>> {
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
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}


////////////////////////////////////////////////////////////////////////////////
// Hsl conversions
////////////////////////////////////////////////////////////////////////////////
impl From<[f32; 3]> for Hsl {
    fn from(components: [f32; 3]) -> Self {
        Hsl {
            h: components[0],
            s: components[1],
            l: components[2],
        }
    }
}

impl From<Cmyk> for Hsl {
    fn from(cmyk: Cmyk) -> Self {
        Hsl::from(Rgb::from(cmyk))
    }
}

impl From<Hsv> for Hsl {
    fn from(hsv: Hsv) -> Self {
        Hsl::from(Rgb::from(hsv))
    }
}

impl From<Rgb> for Hsl {
    fn from(rgb: Rgb) -> Self {
        // Find min, max, index of max, and delta.
        let ratios = rgb.ratios();
        let (min, max, max_index, _) = ratios
            .into_iter()
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

            // Correct wrapping.
            if h > 360.0 {h -= 360.0};
            if h < 0.0 {h += 360.0};

            Hsl {h: h, s: s, l: l}
        }

    }
}

impl From<Xyz> for Hsl {
    fn from(xyz: Xyz) -> Self {
        Hsl::from(Rgb::from(xyz))
    }
}