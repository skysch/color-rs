// Copyright 2018 Skylor R. Schermer.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
////////////////////////////////////////////////////////////////////////////////
//!
//! Defines a 32-bit CMYK color space.
//!
////////////////////////////////////////////////////////////////////////////////
// Local imports.
use crate::Hsl;
use crate::Rgb;
use crate::utility::clamped;
use crate::utility::distance;
use crate::utility::lerp_u8;
use crate::utility::nearly_equal;

// External library imports.
#[cfg(feature = "serde")]
use serde::{
    Serialize,
    Deserialize,
};

// Standard library imports.
use std::convert::From;
use std::fmt;
use std::u8;


////////////////////////////////////////////////////////////////////////////////
// Cmyk
////////////////////////////////////////////////////////////////////////////////
/// The encoded CMYK color.
#[derive(Debug, PartialOrd, PartialEq, Eq, Hash, Ord, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Cmyk {
    /// The cyan component.
    pub c: u8,
    /// The magenta component.
    pub m: u8,
    /// The yellow component.
    pub y: u8,
    /// The key (black) component.
    pub k: u8,
}


impl Cmyk {
    /// Constructs a new `Cmyk` color.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Cmyk;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Cmyk::new(127, 255, 64, 100);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn new(
        cyan: u8, 
        magenta: u8, 
        yellow: u8,
        key: u8) 
        -> Self 
    {
        Cmyk {c: cyan, m: magenta, y: yellow, k: key}
    }

    /// Returns the cyan component of the color.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Cmyk;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Cmyk {c: 127, m: 255, y: 64, k: 100};
    ///
    /// assert_eq!(color.cyan(), 127);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn cyan(&self) -> u8 {
        self.c
    }

    /// Returns the magenta component of the color.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Cmyk;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Cmyk {c: 127, m: 255, y: 64, k: 100};
    ///
    /// assert_eq!(color.magenta(), 255);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn magenta(&self) -> u8 {
        self.m
    }

    /// Returns the yellow component of the color.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Cmyk;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Cmyk {c: 127, m: 255, y: 64, k: 100};
    ///
    /// assert_eq!(color.yellow(), 64);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn yellow(&self) -> u8 {
        self.y
    }

    /// Returns the key component of the color.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Cmyk;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Cmyk {c: 127, m: 255, y: 64, k: 100};
    ///
    /// assert_eq!(color.key(), 100);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn key(&self) -> u8 {
        self.k
    }

    /// Sets the cyan component of the color.
    ///
    /// Note that the Cmyk color space has more degrees of freedom than
    /// necessary, so multiple Cmyk values may denote the same color. Thus 
    /// setting a component value using `set_cyan` may not result in a
    /// color with the given value in the cyan component.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Cmyk;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let mut color = Cmyk {c: 127, m: 255, y: 64, k: 100};
    ///
    /// color.set_cyan(80);
    ///
    /// assert_eq!(color, Cmyk {c: 80, m: 255, y: 64, k: 100});
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn set_cyan(&mut self, value: u8) {
        self.c = value;
    }

    /// Sets the magenta component of the color.
    ///
    /// Note that the Cmyk color space has more degrees of freedom than
    /// necessary, so multiple Cmyk values may denote the same color. Thus 
    /// setting a component value using `set_magenta` may not result in a
    /// color with the given value in the magenta component.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Cmyk;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let mut color = Cmyk {c: 127, m: 255, y: 64, k: 100};
    ///
    /// color.set_magenta(80);
    ///
    /// assert_eq!(color, Cmyk {c: 127, m: 80, y: 64, k: 100});
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn set_magenta(&mut self, value: u8) {
        self.m = value;
    }

    /// Sets the yellow component of the color.
    ///
    /// Note that the Cmyk color space has more degrees of freedom than
    /// necessary, so multiple Cmyk values may denote the same color. Thus 
    /// setting a component value using `set_yellow` may not result in a
    /// color with the given value in the yellow component.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Cmyk;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let mut color = Cmyk {c: 127, m: 255, y: 64, k: 100};
    ///
    /// color.set_yellow(80);
    ///
    /// assert_eq!(color, Cmyk {c: 127, m: 255, y: 80, k: 100});
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn set_yellow(&mut self, value: u8) {
        self.y = value;
    }

    /// Sets the key component of the color.
    ///
    /// Note that the Cmyk color space has more degrees of freedom than
    /// necessary, so multiple Cmyk values may denote the same color. Thus 
    /// setting a component value using `set_key` may not result in a
    /// color with the given value in the key component.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Cmyk;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let mut color = Cmyk {c: 127, m: 255, y: 64, k: 100};
    ///
    /// color.set_key(80);
    ///
    /// assert_eq!(color, Cmyk {c: 127, m: 255, y: 64, k: 80});
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn set_key(&mut self, value: u8) {
        self.k = value;
    }

    /// Returns an array containing the `[C, M, Y, K]` component octets.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Cmyk;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Cmyk {c: 127, m: 255, y: 64, k: 100};
    ///
    /// let octets = color.octets();
    ///
    /// assert_eq!(octets, [127u8, 255u8, 64u8, 100u8]);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn octets(&self) -> [u8; 4] {
        [self.c, self.m, self.y, self.k]
    }

    /// Returns an array containing the `[C, M, Y, K]` component ratios.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Cmyk;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Cmyk {c: 127, m: 255, y: 64, k: 100};
    ///
    /// let ratios = color.ratios();
    ///
    /// assert_eq!(ratios, [0.49803922, 1.0, 0.2509804, 0.39215687]);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn ratios(&self) -> [f32; 4] {
        let max = u8::MAX as f32;
        [
            self.c as f32 / max,
            self.m as f32 / max,
            self.y as f32 / max,
            self.k as f32 / max,
        ]
    }

    /// Returns the hex code of the color.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Cmyk;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Cmyk {c: 127, m: 255, y: 64, k: 100};
    ///
    /// assert_eq!(color.hex(), 0x7FFF4064);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn hex(&self) -> u32 {
        (self.c as u32) << 24 | 
        (self.m as u32) << 16 | 
        (self.y as u32) << 8 | 
        (self.k as u32)
    }

    /// Performs a component-wise linear interpolation between given colors,
    /// returning the color located at the ratio given by `amount`, which is
    /// clamped between 1 and 0.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Cmyk;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let color_a = Cmyk {c: 127, m: 255, y: 64, k: 100};
    /// let color_b = Cmyk {c: 15, m: 144, y: 99, k: 140};
    ///
    /// let lerp_color = Cmyk::lerp(color_a, color_b, 0.65);
    ///
    /// assert_eq!(lerp_color, Cmyk {c: 54, m: 182, y: 86, k: 126});
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
        Cmyk {
            c: lerp_u8(s.c, e.c, amount),
            m: lerp_u8(s.m, e.m, amount),
            y: lerp_u8(s.y, e.y, amount),
            k: lerp_u8(s.k, e.k, amount),
        }
    }

    /// Returns the distance between the given colors in `Cmyk` color space.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Cmyk;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let color_a = Cmyk {c: 127, y: 255, m: 64, k: 100};
    /// let color_b = Cmyk {c: 15, y: 144, m: 99, k: 140};
    ///
    /// assert_eq!(Cmyk::distance(color_a, color_b), 166.40312);
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
        
        let c = distance(s.c, e.c) as f32;
        let m = distance(s.m, e.m) as f32;
        let y = distance(s.y, e.y) as f32;
        let k = distance(s.k, e.k) as f32;

        (c*c + m*m + y*y + k*k).sqrt()
    }
}


impl fmt::Display for Cmyk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}


impl fmt::UpperHex for Cmyk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "#{:02X}{:02X}{:02X}{:02X}", self.c, self.m, self.y, self.k)
    }
}


impl fmt::LowerHex for Cmyk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "#{:02x}{:02x}{:02x}{:02x}", self.c, self.m, self.y, self.k)
    }
}



////////////////////////////////////////////////////////////////////////////////
// Cmyk conversions
////////////////////////////////////////////////////////////////////////////////
impl From<u32> for Cmyk {
    fn from(hex: u32) -> Self {
        Cmyk {
            c: ((hex & 0xFF000000) >> 24) as u8,
            m: ((hex & 0x00FF0000) >> 16) as u8,
            y: ((hex & 0x0000FF00) >> 8) as u8,
            k: ((hex & 0x000000FF)) as u8,
        }
    }
}


impl From<[u8; 4]> for Cmyk {
    fn from(octets: [u8; 4]) -> Self {
        Cmyk {
            c: octets[0],
            m: octets[1],
            y: octets[2],
            k: octets[3],
        }
    }
}


impl From<[f32; 4]> for Cmyk {
    fn from(ratios: [f32; 4]) -> Self {
        Cmyk {
            c: (u8::MAX as f32 * clamped(ratios[0], 0.0, 1.0)) as u8,
            m: (u8::MAX as f32 * clamped(ratios[1], 0.0, 1.0)) as u8,
            y: (u8::MAX as f32 * clamped(ratios[2], 0.0, 1.0)) as u8,
            k: (u8::MAX as f32 * clamped(ratios[3], 0.0, 1.0)) as u8,
        }
    }
}


impl From<Rgb> for Cmyk {
    fn from(rgb: Rgb) -> Self {
        // Find min, max, index of max, and delta.
        let ratios = rgb.ratios();
        let max = ratios
            .iter()
            .fold(ratios[0], |max, &x| {
                if x > max {x} else {max}
            });

        if nearly_equal(max, 0.0) {
            // No need to compute components for black.
            Cmyk { c: 0, m: 0, y: 0, k: 255}

        } else {
            let kn = 1.0 - max;
            let cn = (1.0 - ratios[0] - kn) / max;
            let mn = (1.0 - ratios[1] - kn) / max;
            let yn = (1.0 - ratios[2] - kn) / max;
            
            Cmyk {
                c: (cn * u8::MAX as f32 + 0.5) as u8,
                m: (mn * u8::MAX as f32 + 0.5) as u8,
                y: (yn * u8::MAX as f32 + 0.5) as u8,
                k: (kn * u8::MAX as f32 + 0.5) as u8,
            }
        }
    }
}


impl From<Hsl> for Cmyk {
    fn from(hsl: Hsl) -> Self {
        Cmyk::from(Rgb::from(hsl))
    }
}
