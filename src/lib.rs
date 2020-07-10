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
//! | "parse" | Enables FromStr implementations for colors. |
//!
//! Only the "parse" feature is enabled by default.
////////////////////////////////////////////////////////////////////////////////
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


// Internal modules.
pub mod utility;
mod color_space;
mod color;
#[cfg(feature = "parse")]
mod parse;

#[cfg(test)]
mod test;

// Exports
pub use color_space::cmyk::Cmyk;
pub use color_space::hsl::Hsl;
pub use color_space::hsv::Hsv;
pub use color_space::rgb::Rgb;
pub use color_space::xyz::Xyz;
pub use crate::color::Color;




// /// Standard SRGB gamma correction matrix. This gives the relative intensities 
// /// of each RGB color component.
// const SRGB_GAMMA_CORRECTION: [[f32; 3]; 3] = [
//     [0.2125, 0.0,     0.0   ],
//     [0.0,     0.7154, 0.0   ],
//     [0.0,     0.0,    0.0721]
// ];

