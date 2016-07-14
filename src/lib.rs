// The MIT License (MIT)
// 
// Copyright (c) 2016 Skylor R. Schermer
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in 
// all copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
////////////////////////////////////////////////////////////////////////////////
//!
//! Provides color manipulation functions and color model encodings.
//!
////////////////////////////////////////////////////////////////////////////////

// Module declarations.
#[warn(missing_docs)]
pub mod utilities;
#[warn(missing_docs)]
pub mod cmyk;
#[warn(missing_docs)]
pub mod hsl;
#[warn(missing_docs)]
pub mod hsv;
#[warn(missing_docs)]
pub mod rgb;
#[warn(missing_docs)]
pub mod xyz;
#[cfg(test)]
mod tests;

// Re-exports.
pub use cmyk::Cmyk;
pub use hsl::Hsl;
pub use hsv::Hsv;
pub use rgb::Rgb;
pub use xyz::Xyz;

// Module imports.
use utilities::clamped;
use std::fmt;

/// Standard SRGB gamma correction matrix. This gives the relative intensities 
/// of each RGB color component.
#[allow(dead_code)]
const SRGB_GAMMA_CORRECTION: [[f32; 3]; 3] = [
	[0.2125, 0.0,	  0.0	],
	[0.0,	  0.7154, 0.0	],
	[0.0,	  0.0,	  0.0721]
];


////////////////////////////////////////////////////////////////////////////////
// Color
////////////////////////////////////////////////////////////////////////////////
/// An RGB encoded color with extension methods.
#[derive(Debug, PartialOrd, PartialEq, Eq, Hash, Ord, Clone, Copy, Default)]
pub struct Color {
	/// The base RGB format of the color.
	pub rgb: Rgb
}

impl Color {
	/// Creates a new Color from RGB components.
	pub fn new(red: u8, green: u8, blue: u8) -> Self {
		Color {
			rgb: Rgb {r: red, g: green, b: blue}
		}
	}

	/// Returns the red component.
	pub fn red(&self) -> u8 {
		self.rgb.r
	}
	
	/// Returns the green component.
	pub fn green(&self) -> u8 {
		self.rgb.g
	}
	
	/// Returns the blue component.
	pub fn blue(&self) -> u8 {
		self.rgb.b
	}

	/// Returns the cyan component.
	pub fn cyan(&self) -> u8 {
		Cmyk::from(self.rgb).c
	}

	/// Returns the magenta component.
	pub fn magenta(&self) -> u8 {
		Cmyk::from(self.rgb).m
	}

	/// Returns the yellow component.
	pub fn yellow(&self) -> u8 {
		Cmyk::from(self.rgb).y
	}

	/// Returns the key component.
	pub fn key(&self) -> u8 {
		Cmyk::from(self.rgb).k
	}

	/// Returns the hue.
	pub fn hue(&self) -> f32 {
		Hsl::from(self.rgb).hue()
	}

	/// Returns the saturation.
	pub fn hsl_saturation(&self) -> f32 {
		Hsl::from(self.rgb).saturation()
	}

	/// Returns the saturation.
	pub fn hsv_saturation(&self) -> f32 {
		Hsv::from(self.rgb).saturation()
	}

	/// Returns the lightness.
	pub fn lightness(&self) -> f32 {
		Hsl::from(self.rgb).lightness()
	}
	
	/// Sets the red component.
	pub fn set_red(&mut self, value: u8) {
		self.rgb.r = value;
	}
	
	/// Sets the green component.
	pub fn set_green(&mut self, value: u8) {
		self.rgb.g = value;
	}

	/// Sets the blue component.
	pub fn set_blue(&mut self, value: u8) {
		self.rgb.b = value;
	}

	/// Sets the cyan component.
	pub fn set_cyan(&mut self, value: u8) {
		let mut t = Cmyk::from(self.rgb);
		t.c = value;
		self.rgb = Rgb::from(t);
	}

	/// Sets the magenta component.
	pub fn set_magenta(&mut self, value: u8) {
		let mut t = Cmyk::from(self.rgb);
		t.m = value;
		self.rgb = Rgb::from(t);
	}

	/// Sets the yellow component.
	pub fn set_yellow(&mut self, value: u8) {
		let mut t = Cmyk::from(self.rgb);
		t.y = value;
		self.rgb = Rgb::from(t);
	}

	/// Sets the key component.
	pub fn set_key(&mut self, value: u8) {
		let mut t = Cmyk::from(self.rgb);
		t.k = value;
		self.rgb = Rgb::from(t);
	}

	/// Sets the hue.
	pub fn set_hue(&mut self, value: f32) {
		let mut t = Hsv::from(self.rgb);
		t.set_hue(value);
		self.rgb = Rgb::from(t);
	}

	/// Shifts the hue by the given number of degrees.
	pub fn shift_hue(&mut self, degrees: f32) {
		let h = self.hue();
		self.set_hue(h + degrees);
	}

	/// Sets the saturation.
	pub fn set_hsl_saturation(&mut self, value: f32) {
		let mut t = Hsl::from(self.rgb);
		t.set_saturation(value);
		self.rgb = Rgb::from(t);
	}

	/// Sets the saturation.
	pub fn set_hsv_saturation(&mut self, value: f32) {
		let mut t = Hsv::from(self.rgb);
		t.set_saturation(value);
		self.rgb = Rgb::from(t);
	}

	/// Saturates the color in the HSL color space by the given proportion.
	pub fn hsl_saturate(&mut self, value: f32) {
		let s = self.hsl_saturation();
		let v = clamped(value, 0.0, 1.0);
		self.set_hsl_saturation(s + (s * v));
	}

	/// Desaturates the color in the HSL color space by the given proportion.
	pub fn hsl_desaturate(&mut self, value: f32) {
		let s = self.hsl_saturation();
		let v = clamped(value, 0.0, 1.0);
		self.set_hsl_saturation(s - (s * v));
	}

	/// Saturates the color in the HSV color space by the given proportion.
	pub fn hsv_saturate(&mut self, value: f32) {
		let s = self.hsv_saturation();
		let v = clamped(value, 0.0, 1.0);
		self.set_hsv_saturation(s + (s * v));
	}

	/// Desaturates the color in the HSV color space by the given proportion.
	pub fn hsv_desaturate(&mut self, value: f32) {
		let s = self.hsv_saturation();
		let v = clamped(value, 0.0, 1.0);
		self.set_hsv_saturation(s - (s * v));
	}

	/// Sets the lightness.
	pub fn set_lightness(&mut self, value: f32) {
		let mut t = Hsl::from(self.rgb);
		t.set_lightness(value);
		self.rgb = Rgb::from(t);
	}


	/// Lightens the color by the given proportion.
	pub fn lighten(&mut self, value: f32) {
		let l = self.lightness();
		let v = clamped(value, 0.0, 1.0);
		self.set_lightness(l + (l * v));
	}

	/// Darkens the color by the given proportion.
	pub fn darken(&mut self, value: f32) {
		let l = self.lightness();
		let v = clamped(value, 0.0, 1.0);
		self.set_lightness(l - (l * v));
	}

	/// Returns an array containing the [R, G, B] component octets.
	pub fn rgb_octets(&self) -> [u8; 3] {
		self.rgb.octets()
	}

	/// Returns an array containing the [C, M, Y, K] component octets.
	pub fn cmyk_octets(&self) -> [u8; 4] {
		Cmyk::from(self.rgb).octets()
	}

	/// Returns an array containing the [H, S, L] components.
	pub fn hsl_components(&self) -> [f32; 3] {
		Hsl::from(self.rgb).components()
	}

	/// Returns an array containing the [H, S, V] components.
	pub fn hsv_components(&self) -> [f32; 3] {
		Hsv::from(self.rgb).components()
	}

	/// Returns an array containing the [R, G, B] component ratios.
	pub fn rgb_ratios(&self) -> [f32; 3] {
		self.rgb.ratios()
	}

	/// Returns an array containing the [C, M, Y, K] component ratios.
	pub fn cmyk_ratios(&self) -> [f32; 4] {
		Cmyk::from(self.rgb).ratios()
	}

	/// Returns the RGB hex code.
	pub fn rgb_hex(&self) -> u32 {
		self.rgb.hex()
	}

	/// Returns the CMYK hex code.
	pub fn cmyk_hex(&self) -> u32 {
		Cmyk::from(self.rgb).hex()
	}

	/// Performs an RGB component-wise linear interpolation between the colors 
	/// `start` and `end`, returning the color located at the ratio given by 
	/// `amount`, which is clamped between 1 and 0.
	pub fn rgb_lerp<C>(start: C, end: C, amount: f32) -> Self 
		where C: Into<Rgb> + Sized
	{
		Rgb::lerp(start.into(), end.into(), amount).into()
	}

	/// Performs a CMYK component-wise linear interpolation between the colors 
	/// `start` and `end`, returning the color located at the ratio given by 
	/// `amount`, which is clamped between 1 and 0.
	pub fn cmyk_lerp<C>(start: C, end: C, amount: f32) -> Self 
		where C: Into<Cmyk> + Sized
	{
		Cmyk::lerp(start.into(), end.into(), amount).into()
	}

	/// Performs an HSL component-wise linear interpolation between the colors 
	/// `start` and `end`, returning the color located at the ratio given by 
	/// `amount`, which is clamped between 1 and 0.
	pub fn hsl_lerp<C>(start: C, end: C, amount: f32) -> Self 
		where C: Into<Hsl> + Sized
	{
		Hsl::lerp(start.into(), end.into(), amount).into()
	}

	/// Returns the distance between the given colors in RGB color space.
	pub fn rgb_distance<C>(start: C, end: C) -> f32 
		where C: Into<Rgb> + Sized
	{
		Rgb::distance(start.into(), end.into())
	}

	/// Returns the distance between the given colors in CMYK color space.
	pub fn cmyk_distance<C>(start: C, end: C) -> f32 
		where C: Into<Cmyk> + Sized
	{
		Cmyk::distance(start.into(), end.into())
	}

	/// Returns the distance between the given colors in HSL color space.
	pub fn hsl_distance<C>(start: C, end: C) -> f32 
		where C: Into<Hsl> + Sized
	{
		Hsl::distance(start.into(), end.into())
	}
}



impl fmt::Display for Color {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "{:?}", self)
	}
}


impl fmt::UpperHex for Color {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "#{:X}", self.rgb)
	}
}


impl fmt::LowerHex for Color {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
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
