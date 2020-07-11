// Copyright 2018 Skylor R. Schermer.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
////////////////////////////////////////////////////////////////////////////////
//!
//! Defines general purpose functions common use.
//!
////////////////////////////////////////////////////////////////////////////////
// Standard library imports.
use std::f32;
use std::ops::Sub;


////////////////////////////////////////////////////////////////////////////////
// nearly_equal
////////////////////////////////////////////////////////////////////////////////
/// Returns true if the given [`f32`] values are nearly equal, taking into
/// account relative error and infinites. 
///
/// [`f32`]: https://doc.rust-lang.org/std/primitive.f32.html
///
/// # Example
///
/// ```rust
/// # use std::error::Error;
/// # use std::f32;
/// # use color::utility::nearly_equal;
/// # fn example() -> Result<(), Box<dyn Error>> {
/// # //-------------------------------------------------------------------
/// assert!(nearly_equal(0.000002f32, 0.000001999999999f32));
///
/// // Infinities compare nearly equal:
/// assert!(nearly_equal(f32::INFINITY, f32::INFINITY));
///
/// // NANs do not compare nearly equal:
/// assert!(!nearly_equal(f32::NAN, f32::NAN));
/// # //-------------------------------------------------------------------
/// #     Ok(())
/// # }
/// #
/// # fn main() {
/// #     example().unwrap();
/// # }
/// ```
#[inline]
pub fn nearly_equal(a: f32, b: f32) -> bool {
    let abs_a = a.abs();
    let abs_b = b.abs();
    let diff = (a - b).abs();

    if a == b { // Shortcut, handles infinities.
        true
    } else if a == 0.0 || b == 0.0 || diff < f32::MIN_POSITIVE {
        // a or b is zero or both are extremely close to it
        // relative error is less meaningful here
        diff < (f32::EPSILON * f32::MIN_POSITIVE)
    } else { // Use relative error.
        (diff / f32::min(abs_a + abs_b, f32::MAX)) < f32::EPSILON
    }
}


////////////////////////////////////////////////////////////////////////////////
// close
////////////////////////////////////////////////////////////////////////////////
/// Returns true if the given [`f32`] values are with `precision` distance from 
/// eachother.
///
/// [`f32`]: https://doc.rust-lang.org/std/primitive.f32.html
///
/// # Example
///
/// ```rust
/// # use std::error::Error;
/// # use std::f32;
/// # use color::utility::close;
/// # fn example() -> Result<(), Box<dyn Error>> {
/// # //-------------------------------------------------------------------
/// assert!(close(0.02f32, 0.03f32, 0.011));
///
/// // Infinities do not compare close:
/// assert!(!close(f32::INFINITY, f32::INFINITY, 0.01));
///
/// // NANs do not compare close:
/// assert!(!close(f32::NAN, f32::NAN, 0.01));
/// # //-------------------------------------------------------------------
/// #     Ok(())
/// # }
/// #
/// # fn main() {
/// #     example().unwrap();
/// # }
/// ```
#[inline]
#[cfg(test)]
pub fn close(a: f32, b: f32, precision: f32) -> bool {
    (a - b).abs() < precision
}

////////////////////////////////////////////////////////////////////////////////
// clamped
////////////////////////////////////////////////////////////////////////////////
/// Returns the given value clamped between the provided bounds.
///
/// # Example
///
/// ```rust
/// # use std::error::Error;
/// # use color::utility::clamped;
/// # fn example() -> Result<(), Box<dyn Error>> {
/// # //-------------------------------------------------------------------
/// assert_eq!(clamped(2.2, 4.3, 7.4), 4.3);
/// assert_eq!(clamped(5.0, 4.3, 7.4), 5.0);
/// assert_eq!(clamped(7.6, 4.3, 7.4), 7.4);
/// # //-------------------------------------------------------------------
/// #     Ok(())
/// # }
/// #
/// # fn main() {
/// #     example().unwrap();
/// # }
/// ```
#[inline]
pub fn clamped<T>(value: T, lower_bound: T, upper_bound: T) -> T
    where T: PartialOrd
{
    assert!(lower_bound <= upper_bound);
    if value < lower_bound {
        lower_bound
    } else if value > upper_bound {
        upper_bound
    } else {
        value
    }
}

////////////////////////////////////////////////////////////////////////////////
// distance
////////////////////////////////////////////////////////////////////////////////
/// Returns the distance between the given values.
///
/// # Example
///
/// ```rust
/// # use std::error::Error;
/// # use color::utility::distance;
/// # fn example() -> Result<(), Box<dyn Error>> {
/// # //-------------------------------------------------------------------
/// assert_eq!(distance(2.2, 4.3), 2.0999999999999996);
/// assert_eq!(distance(4.3, 2.2), 2.0999999999999996);
/// # //-------------------------------------------------------------------
/// #     Ok(())
/// # }
/// #
/// # fn main() {
/// #     example().unwrap();
/// # }
/// ```
#[inline]
pub fn distance<T>(a: T, b: T) -> T where T: Sub<Output=T> + PartialOrd {
    if a > b {a - b} else {b - a}
}

////////////////////////////////////////////////////////////////////////////////
// lerp_u8
////////////////////////////////////////////////////////////////////////////////
/// Performs a linear interpolation between `start` and `end`, returning the 
/// value located at the ratio given by `amount`, which is clamped between 0 and
/// 1. 
///
/// # Example
///
/// ```rust
/// # use std::error::Error;
/// # use color::utility::lerp_u8;
/// # fn example() -> Result<(), Box<dyn Error>> {
/// # //-------------------------------------------------------------------
/// assert_eq!(lerp_u8(15, 167, 0.34), 66);
/// # //-------------------------------------------------------------------
/// #     Ok(())
/// # }
/// #
/// # fn main() {
/// #     example().unwrap();
/// # }
/// ```
#[inline]
pub fn lerp_u8(start: u8, end:u8, amount: f32) -> u8 {
    let a = if start > end {
        1.0 - clamped(amount, 0.0, 1.0)
    } else {
        clamped(amount, 0.0, 1.0)
    };

    let s = if start > end {end} else {start};
    let e = if start > end {start} else {end};
    (((e-s) as f32) * a) as u8 + s
}

////////////////////////////////////////////////////////////////////////////////
// lerp_f32
////////////////////////////////////////////////////////////////////////////////
/// Performs a linear interpolation between `start` and `end`, returning the 
/// value located at the ratio given by `amount`, which is clamped between 0 and
/// 1. 
///
/// # Example
///
/// ```rust
/// # use std::error::Error;
/// # use color::utility::lerp_f32;
/// # fn example() -> Result<(), Box<dyn Error>> {
/// # //-------------------------------------------------------------------
/// assert_eq!(lerp_f32(15.0, 167.0, 0.34), 66.68);
/// # //-------------------------------------------------------------------
/// #     Ok(())
/// # }
/// #
/// # fn main() {
/// #     example().unwrap();
/// # }
/// ```
#[inline]
pub fn lerp_f32(start: f32, end:f32, amount: f32) -> f32 {
    let a = if start > end {
        1.0 - clamped(amount, 0.0, 1.0)
    } else {
        clamped(amount, 0.0, 1.0)
    };

    let s = if start > end {end} else {start};
    let e = if start > end {start} else {end};
    ((e-s) * a) + s
}

////////////////////////////////////////////////////////////////////////////////
// cerp_f32
////////////////////////////////////////////////////////////////////////////////
/// Performs a cubic interpolation between `start` and `end`, returning the 
/// value located at the ratio given by `amount`, which is clamped between 0
/// and 1. The interpolation function will be consistent with the slopes given
/// by `start_slope` and `end_slope`.
///
/// # Example
///
/// ```rust
/// # use std::error::Error;
/// # use color::utility::cerp_f32;
/// # fn example() -> Result<(), Box<dyn Error>> {
/// # //-------------------------------------------------------------------
/// assert_eq!(cerp_f32(15.0, 167.0, 0.0, 0.0, 0.34), 55.765186);
/// # //-------------------------------------------------------------------
/// #     Ok(())
/// # }
/// #
/// # fn main() {
/// #     example().unwrap();
/// # }
/// ```
#[inline]
pub fn cerp_f32(
    start: f32,
    end: f32,
    start_slope: f32,
    end_slope: f32,
    amount: f32)
    -> f32
{
    let a = if start > end {
        1.0 - clamped(amount, 0.0, 1.0)
    } else {
        clamped(amount, 0.0, 1.0)
    };

    let s = if start > end {end} else {start};
    let e = if start > end {start} else {end};

    let a2 = a * a;
    let a3 = a2 * a;

    (2.0*a3 - 3.0*a2 + 1.0) * s
        + (a3 - 2.0*a2 + a) * start_slope
        + (-2.0*a3 + 3.0*a2) * e
        + (a3 - a2) * end_slope
}
