// Copyright 2020 Skylor R. Schermer.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
////////////////////////////////////////////////////////////////////////////////
//!
//! Defines a 24-bit RGB color space.
//!
////////////////////////////////////////////////////////////////////////////////

// Local imports.
use crate::Cmyk;
use crate::Hsl;
use crate::Hsv;
use crate::utility::cerp_u8;
use crate::utility::clamped;
use crate::utility::distance;
use crate::utility::lerp_u8;
use crate::Xyz;

// External library imports.
#[cfg(feature = "serde")]
use serde::Deserialize;
#[cfg(feature = "serde")]
use serde::Serialize;
use tracing::Level;
use tracing::event;
use tracing::span;


// Standard library imports.
use std::convert::From;
use std::fmt;
use std::f32;
use std::u8;


////////////////////////////////////////////////////////////////////////////////
// HexCodeParseError
////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// An error which can be returned while parsing an RGB hex code.
/// 
/// This error is returned by [`Rgb::from_hex_code`] if the parse fails.
///
/// [`Rgb::from_hex_code`]: struct.Rgb.html#method.from_hex_code
pub struct RgbHexCodeParseError;


////////////////////////////////////////////////////////////////////////////////
// Rgb
////////////////////////////////////////////////////////////////////////////////
/// The encoded RGB color.
#[derive(Debug, PartialOrd, PartialEq, Eq, Hash, Ord, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Rgb {
    /// The red component.
    pub r: u8,
    /// The green component.
    pub g: u8,
    /// The blue component.
    pub b: u8,
}


impl Rgb {
    /// Constructs a new `Rgb` color.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Rgb;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Rgb::new(127, 255, 64);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Rgb {r: red, g: green, b: blue}
    }

    /// Constructs a new `Rgb` color by parsing a hex code.
    ///
    /// Both three and six digit variations are acceptable, and the longest will
    /// be used.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Rgb;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Rgb::from_hex_code("#a1b2c3");
    /// let color_short = Rgb::from_hex_code("#abc");
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn from_hex_code(hex: &str) -> Result<Rgb, RgbHexCodeParseError> {
        if !hex.starts_with("#") || hex.len() < 4 {
            return Err(RgbHexCodeParseError);
        }

        Rgb::from_hex_code_6(&hex[1..])
            .or_else(|_| Rgb::from_hex_code_3(&hex[1..]))
    }

    fn from_hex_code_6(hex: &str) -> Result<Rgb, RgbHexCodeParseError> {
        if hex.len() != 6 {
            return Err(RgbHexCodeParseError);
        }

        match u32::from_str_radix(&hex[0..6], 16) {
            Ok(v) => Ok(Rgb::from(v)),
            Err(_) => Err(RgbHexCodeParseError),
        }
    }

    fn from_hex_code_3(hex: &str) -> Result<Rgb, RgbHexCodeParseError> {
        if hex.len() != 3 {
            return Err(RgbHexCodeParseError);
        }

        match u32::from_str_radix(&hex[0..3], 16) {
            Ok(v) => {
                // Expand three digits into six.
                let mut expanded = 0;
                expanded |= v & 0x00F;
                expanded |= (v & 0x00F) << 4;
                expanded |= (v & 0x0F0) << 4;
                expanded |= (v & 0x0F0) << 8;
                expanded |= (v & 0xF00) << 8;
                expanded |= (v & 0xF00) << 12;
                Ok(Rgb::from(expanded))
            }
            Err(_) => Err(RgbHexCodeParseError),
        }
    }

    /// Returns the red component.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Rgb;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Rgb::new(127, 255, 64);
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
        self.r
    }
    
    /// Returns the green component.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Rgb;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Rgb::new(127, 255, 64);
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
        self.g
    }
    
    /// Returns the blue component.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Rgb;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Rgb::new(127, 255, 64);
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
        self.b
    }
    
    /// Sets the red component.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Rgb;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let mut color = Rgb::new(127, 255, 64);
    /// 
    /// color.set_red(15);
    /// 
    /// assert_eq!(color, Rgb {r: 15, g: 255, b: 64});
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn set_red(&mut self, value: u8) {
        self.r = value;
    }
    
    /// Sets the green component.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Rgb;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let mut color = Rgb::new(127, 255, 64);
    /// 
    /// color.set_green(15);
    /// 
    /// assert_eq!(color, Rgb {r: 127, g: 15, b: 64});
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn set_green(&mut self, value: u8) {
        self.g = value;
    }


    /// Sets the blue component.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Rgb;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let mut color = Rgb::new(127, 255, 64);
    /// 
    /// color.set_blue(15);
    /// 
    /// assert_eq!(color, Rgb {r: 127, g: 255, b: 15});
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn set_blue(&mut self, value: u8) {
        self.b = value;
    }

    /// Returns an array containing the `[R, G, B]` component octets.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Rgb;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Rgb {r: 127, g: 255, b: 64};
    ///
    /// let octets = color.octets();
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
    pub fn octets(&self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }

    /// Returns an array containing the `[R, G, B]` component ratios.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Rgb;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Rgb {r: 127, g: 255, b: 64};
    ///
    /// let ratios = color.ratios();
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
    pub fn ratios(&self) -> [f32; 3] {
        let max = u8::MAX as f32;
        [
            self.r as f32 / max,
            self.g as f32 / max, 
            self.b as f32 / max,
        ]
    }

    /// Returns the `Rgb` hex code of the color.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Rgb;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let color = Rgb {r: 127, g: 255, b: 64};
    ///
    /// assert_eq!(color.hex(), 0x7FFF40);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn hex(&self) -> u32 {
        (self.r as u32) << 16 | (self.g as u32) << 8 | (self.b as u32)
    }

    /// Performs an `Rgb` component-wise linear interpolation between given 
    /// colors, returning the color located at the ratio given by `amount`,
    /// which is clamped between 1 and 0.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Rgb;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let color_a = Rgb {r: 127, g: 255, b: 64};
    /// let color_b = Rgb {r: 15, g: 144, b: 99};
    ///
    /// let lerp_color = Rgb::linear_interpolate(color_a, color_b, 0.65);
    ///
    /// assert_eq!(lerp_color, Rgb {r: 54, g: 182, b: 86});
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
        Rgb {
            r: lerp_u8(s.r, e.r, amount),
            g: lerp_u8(s.g, e.g, amount),
            b: lerp_u8(s.b, e.b, amount),
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
    /// # use color::Rgb;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let color_a = Rgb::new(24, 68, 91);
    /// let color_b = Rgb::new(84, 228, 155);
    ///
    /// let lerp_color = Rgb::cubic_interpolate(
    ///     color_a, color_b, 0.0, 0.0, 0.39);
    ///
    /// assert_eq!(lerp_color, 
    ///     Rgb::new(44, 122, 112));
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
        Rgb {
            r: cerp_u8(s.r, e.r, start_slope, end_slope, amount),
            g: cerp_u8(s.g, e.g, start_slope, end_slope, amount),
            b: cerp_u8(s.b, e.b, start_slope, end_slope, amount),
        }
    }

    /// Returns the distance between the given colors in `Rgb` color space.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use color::Rgb;
    /// # fn example() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let color_a = Rgb {r: 127, g: 255, b: 64};
    /// let color_b = Rgb {r: 15, g: 144, b: 99};
    ///
    /// assert_eq!(Rgb::distance(color_a, color_b), 161.52399);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     example().unwrap();
    /// # }
    /// ```
    pub fn distance<C, D>(start: C, end: D) -> f32 
        where
            C: Into<Self> + Sized,
            D: Into<Self> + Sized,
    {
        let s = start.into();
        let e = end.into();
        
        let r = distance(s.r, e.r) as f32;
        let g = distance(s.g, e.g) as f32;
        let b = distance(s.b, e.b) as f32;

        (r*r + g*g + b*b).sqrt()
    }
}



impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}


impl fmt::UpperHex for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}


impl fmt::LowerHex for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}


////////////////////////////////////////////////////////////////////////////////
// Rgb conversions
////////////////////////////////////////////////////////////////////////////////
impl From<u32> for Rgb {
    fn from(hex: u32) -> Self {
        let span = span!(Level::DEBUG, "Rgb::from<u32>");
        let _enter = span.enter();

        event!(Level::TRACE, "hex={:0X}", hex);
        
        let rgb = Rgb {
            r: ((hex & 0xFF0000) >> 16) as u8,
            g: ((hex & 0x00FF00) >> 8) as u8,
            b: ((hex & 0x0000FF)) as u8,
        };

        event!(Level::TRACE, "Rgb={:?}", rgb);
        rgb
    }
}


impl From<[u8; 3]> for Rgb {
    fn from(octets: [u8; 3]) -> Self {
        let span = span!(Level::DEBUG, "Rgb::from<[u8; 3]>");
        let _enter = span.enter();
        
        event!(Level::TRACE, "octets=[{:0X}, {:0X}, {:0X}]",
            octets[0], octets[1], octets[2]);

        let rgb = Rgb {
            r: octets[0],
            g: octets[1],
            b: octets[2],
        };

        event!(Level::TRACE, "Rgb={:?}", rgb);
        rgb
    }
}

impl From<[f32; 3]> for Rgb {
    fn from(ratios: [f32; 3]) -> Self {
        let span = span!(Level::DEBUG, "Rgb::from<[f32; 3]>");
        let _enter = span.enter();

        event!(Level::TRACE, "ratios=[{}, {}, {}]",
            ratios[0], ratios[1], ratios[2]);
        
        let rgb = Rgb {
            r: (u8::MAX as f32 * clamped(ratios[0], 0.0, 1.0)) as u8,
            g: (u8::MAX as f32 * clamped(ratios[1], 0.0, 1.0)) as u8,
            b: (u8::MAX as f32 * clamped(ratios[2], 0.0, 1.0)) as u8,
        };

        event!(Level::TRACE, "Rgb={:?}", rgb);
        rgb
    }
}

/// Converts the color to an RGB vector.
impl From<Rgb> for [f32; 3] {
    fn from(rgb: Rgb) -> Self {
        let span = span!(Level::DEBUG, "[f32; 3]::from<Rgb>");
        let _enter = span.enter();
        
        [
            (rgb.r as f32) / (u8::MAX as f32),
            (rgb.g as f32) / (u8::MAX as f32),
            (rgb.b as f32) / (u8::MAX as f32)
        ]
    }
}

/// Converts the color to an RGBA vector.
impl From<Rgb> for [f32; 4] {
    fn from(rgb: Rgb) -> Self {
        let span = span!(Level::DEBUG, "Rgb::from<[f32; 4]>");
        let _enter = span.enter();
        
        [
            (rgb.r as f32) / (u8::MAX as f32),
            (rgb.g as f32) / (u8::MAX as f32),
            (rgb.b as f32) / (u8::MAX as f32),
            0.0
        ]
    }
}

impl From<Cmyk> for Rgb {
    fn from(cmyk: Cmyk) -> Self {
        let span = span!(Level::DEBUG, "Rgb::from<Cmyk>");
        let _enter = span.enter();
        
        let ratios = cmyk.ratios();
        let cn = 1.0 - ratios[0];
        let mn = 1.0 - ratios[1];
        let yn = 1.0 - ratios[2];
        let kn = 1.0 - ratios[3];

        event!(Level::TRACE, "cn={}, mn={}, yn={}, kn={}", cn, mn, yn, kn);

        let rgb = Rgb {
            r: (u8::MAX as f32 * cn * kn + 0.5) as u8,
            g: (u8::MAX as f32 * mn * kn + 0.5) as u8,
            b: (u8::MAX as f32 * yn * kn + 0.5) as u8,
        };

        event!(Level::TRACE, "Rgb={:?}", rgb);
        rgb
    }
}


impl From<Hsl> for Rgb {
    fn from(hsl: Hsl) -> Self {
        let span = span!(Level::DEBUG, "Rgb::from<Hsl>");
        let _enter = span.enter();
        
        let (h, s, l) = (hsl.hue(), hsl.saturation(), hsl.lightness());

        event!(Level::TRACE, "Hsl {{ h={}, s={}, l={} }}", h, s, l);

        // Compute intermediate values.
        let ci: f32 = s * (1.0 - (2.0 * l - 1.0).abs());
        let xi: f32 = ci * (1.0 - (h / 60.0 % 2.0 - 1.0).abs());
        let mi: f32 = l - ci / 2.0;

        event!(Level::TRACE, "ci={}, xi={}, mi={}", ci, xi, mi);

        // Scale and cast.
        let c = ((u8::MAX as f32) * ci) as u8;
        let x = ((u8::MAX as f32) * xi) as u8;
        let m = ((u8::MAX as f32) * mi) as u8;

        event!(Level::TRACE, "c={}, x={}, m={}", c, x, m);

        // Use hue hextant to select RGB color.
        let rgb = match h {
            h if   0.0 <= h && h <  60.0 => Rgb::new(c+m, x+m,   m),
            h if  60.0 <= h && h < 120.0 => Rgb::new(x+m, c+m,   m),
            h if 120.0 <= h && h < 180.0 => Rgb::new(  m, c+m, x+m),
            h if 180.0 <= h && h < 240.0 => Rgb::new(  m, x+m, c+m),
            h if 240.0 <= h && h < 300.0 => Rgb::new(x+m,   m, c+m),
            h if 300.0 <= h && h < 360.0 => Rgb::new(c+m,   m, x+m),
            _ => unreachable!(),
        };

        event!(Level::TRACE, "Rgb={:?}", rgb);
        rgb 
    }
}

impl From<Hsv> for Rgb {
    fn from(hsv: Hsv) -> Self {
        let span = span!(Level::DEBUG, "Rgb::from<Hsv>");
        let _enter = span.enter();
        
        let (h, s, v) = (hsv.hue(), hsv.saturation(), hsv.value());

        event!(Level::TRACE, "Hsv {{ h={}, s={}, v={} }}", h, s, v);

        let c = v * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = v - c;

        event!(Level::TRACE, "c={}, x={}, m={}", c, x, m);

        let (ri, gi, bi) = match h {
            h if   0.0 <= h && h <  60.0 => (  c,   x, 0.0),
            h if  60.0 <= h && h < 120.0 => (  x,   c, 0.0),
            h if 120.0 <= h && h < 180.0 => (0.0,   c,   x),
            h if 180.0 <= h && h < 240.0 => (0.0,   x,   c),
            h if 240.0 <= h && h < 300.0 => (  x, 0.0,   c),
            h if 300.0 <= h && h < 360.0 => (  c, 0.0,   x),
            _ => unreachable!(),
        };

        event!(Level::TRACE, "ri={}, gi={}, bi={}", ri, gi, bi);

        let rgb = Rgb {
            r: ((ri + m) * (u8::MAX as f32)) as u8,
            g: ((gi + m) * (u8::MAX as f32)) as u8,
            b: ((bi + m) * (u8::MAX as f32)) as u8,
        };

        event!(Level::TRACE, "Rgb={:?}", rgb);
        rgb
    }
}

impl From<Xyz> for Rgb {
    fn from(xyz: Xyz) -> Self {
        let span = span!(Level::DEBUG, "Rgb::from<Xyz>");
        let _enter = span.enter();
        
        let (x, y, z) = (xyz.x(), xyz.y(), xyz.z()); 

        event!(Level::TRACE, "Xyz {{ x={}, y={}, z={} }}", x, y, z);

        let ri = x *  3.2404542 + y * -1.5371385 + z * -0.4985314;
        let gi = x * -0.9692660 + y *  1.8760108 + z *  0.0415560;
        let bi = x *  0.0556434 + y * -0.2040259 + z *  1.0572252;

        event!(Level::TRACE, "ri={}, gi={}, bi={}", ri, gi, bi);

        let rgb = Rgb {
            r: (ri * u8::MAX as f32) as u8,
            g: (gi * u8::MAX as f32) as u8,
            b: (bi * u8::MAX as f32) as u8,
        };

        event!(Level::TRACE, "Rgb={:?}", rgb);
        rgb
    }
}
