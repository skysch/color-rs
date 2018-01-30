// Copyright 2018 Skylor R. Schermer.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
////////////////////////////////////////////////////////////////////////////////
//!
//! Color testing module.
//!
////////////////////////////////////////////////////////////////////////////////

// Local imports.
use Cmyk;
use Hsl;
use Hsv;
use Rgb;

use utilities::close;

////////////////////////////////////////////////////////////////////////////////
// UNIT
////////////////////////////////////////////////////////////////////////////////
/// Resolution of u8 in f32.
const UNIT: f32 = 1.0 / 255.0;

////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////

/// Tests color conversions for the color black.
#[test]
fn color_conversions_black() {
    let black = Rgb::from(0x000000);
    let black_hsl = Hsl::from(black);
    let black_hsv = Hsv::from(black);
    let black_cmyk = Cmyk::from(black);
    
    assert!(close(black_hsl.hue(), 0.0, UNIT));
    assert!(close(black_hsl.saturation(), 0.0, UNIT));
    assert!(close(black_hsl.lightness(), 0.0, UNIT));
    
    assert_eq!(black_cmyk.c, 0);
    assert_eq!(black_cmyk.m, 0);
    assert_eq!(black_cmyk.y, 0);
    assert_eq!(black_cmyk.k, 255);
    
    let black_rgb_a = Rgb::from(black_hsl);
    assert_eq!(black_rgb_a, black);
    
    let black_rgb_b = Rgb::from(black_cmyk);
    assert_eq!(black_rgb_b, black);
    
    assert!(close(black_hsv.hue(), 0.0, UNIT));
    assert!(close(black_hsv.saturation(), 0.0, UNIT));
    assert!(close(black_hsv.value(), 0.0, UNIT));
}

/// Tests color conversions for the color white.
#[test]
fn color_conversions_white() {
    let white = Rgb::from(0xFFFFFF);
    let white_hsl = Hsl::from(white);
    let white_hsv = Hsv::from(white);
    let white_cmyk = Cmyk::from(white);
    
    assert!(close(white_hsl.hue(), 0.0, UNIT));
    assert!(close(white_hsl.saturation(), 0.0, UNIT));
    assert!(close(white_hsl.lightness(), 1.0, UNIT));
    
    assert_eq!(white_cmyk.c, 0);
    assert_eq!(white_cmyk.m, 0);
    assert_eq!(white_cmyk.y, 0);
    assert_eq!(white_cmyk.k, 0);
    
    let white_rgb_a = Rgb::from(white_hsl);
    assert_eq!(white_rgb_a, white);
    
    let white_rgb_b = Rgb::from(white_cmyk);
    assert_eq!(white_rgb_b, white);
    
    assert!(close(white_hsv.hue(), 0.0, UNIT));
    assert!(close(white_hsv.saturation(), 0.0, UNIT));
    assert!(close(white_hsv.value(), 1.0, UNIT));
}

/// Tests color conversions for the color red.
#[test]
fn color_conversions_red() {
    let red = Rgb::from(0xFF0000);
    let red_hsl = Hsl::from(red);
    let red_hsv = Hsv::from(red);
    let red_cmyk = Cmyk::from(red);
    
    assert!(close(red_hsl.hue(), 0.0, UNIT));
    assert!(close(red_hsl.saturation() , 1.0, UNIT));
    assert!(close(red_hsl.lightness(), 0.5, UNIT));
    
    assert_eq!(red_cmyk.c, 0);
    assert_eq!(red_cmyk.m, 255);
    assert_eq!(red_cmyk.y, 255);
    assert_eq!(red_cmyk.k, 0);
    
    let red_rgb_a = Rgb::from(red_hsl);
    assert_eq!(red_rgb_a, red);
    
    let red_rgb_b = Rgb::from(red_cmyk);
    assert_eq!(red_rgb_b, red);
    
    assert!(close(red_hsv.hue(), 0.0, UNIT));
    assert!(close(red_hsv.saturation(), 1.0, UNIT));
    assert!(close(red_hsv.value(), 1.0, UNIT));
}

/// Tests color conversions for the color lime.
#[test]
fn color_conversions_lime() {
    let lime = Rgb::from(0x00FF00);
    let lime_hsl = Hsl::from(lime);
    let lime_hsv = Hsv::from(lime);
    let lime_cmyk = Cmyk::from(lime);
    
    assert!(close(lime_hsl.hue(), 120.0, UNIT));
    assert!(close(lime_hsl.saturation(), 1.0, UNIT));
    assert!(close(lime_hsl.lightness(), 0.5, UNIT));
    
    assert_eq!(lime_cmyk.c, 255);
    assert_eq!(lime_cmyk.m, 0);
    assert_eq!(lime_cmyk.y, 255);
    assert_eq!(lime_cmyk.k, 0);
    
    let lime_rgb_a = Rgb::from(lime_hsl);
    assert_eq!(lime_rgb_a, lime);
    
    let lime_rgb_b = Rgb::from(lime_cmyk);
    assert_eq!(lime_rgb_b, lime);
    
    assert!(close(lime_hsv.hue(), 120.0, UNIT));
    assert!(close(lime_hsv.saturation(), 1.0, UNIT));
    assert!(close(lime_hsv.value(), 1.0, UNIT));
}

/// Tests color conversions for the color blue.
#[test]
fn color_conversions_blue() {
    let blue = Rgb::from(0x0000FF);
    let blue_hsl = Hsl::from(blue);
    let blue_hsv = Hsv::from(blue);
    let blue_cmyk = Cmyk::from(blue);
    
    assert!(close(blue_hsl.hue(), 240.0, UNIT));
    assert!(close(blue_hsl.saturation(), 1.0, UNIT));
    assert!(close(blue_hsl.lightness(), 0.5, UNIT));
    
    assert_eq!(blue_cmyk.c, 255);
    assert_eq!(blue_cmyk.m, 255);
    assert_eq!(blue_cmyk.y, 0);
    assert_eq!(blue_cmyk.k, 0);
    
    let blue_rgb_a = Rgb::from(blue_hsl);
    assert_eq!(blue_rgb_a, blue);
    
    let blue_rgb_b = Rgb::from(blue_cmyk);
    assert_eq!(blue_rgb_b, blue);
    
    assert!(close(blue_hsv.hue(), 240.0, UNIT));
    assert!(close(blue_hsv.saturation(), 1.0, UNIT));
    assert!(close(blue_hsv.value(), 1.0, UNIT));
}

/// Tests color conversions for the color yellow.
#[test]
fn color_conversions_yellow() {
    let yellow = Rgb::from(0xFFFF00);
    let yellow_hsl = Hsl::from(yellow);
    let yellow_hsv = Hsv::from(yellow);
    let yellow_cmyk = Cmyk::from(yellow);
    
    assert!(close(yellow_hsl.hue(), 60.0, UNIT));
    assert!(close(yellow_hsl.saturation(), 1.0, UNIT));
    assert!(close(yellow_hsl.lightness(), 0.5, UNIT));
    
    assert_eq!(yellow_cmyk.c, 0);
    assert_eq!(yellow_cmyk.m, 0);
    assert_eq!(yellow_cmyk.y, 255);
    assert_eq!(yellow_cmyk.k, 0);
    
    let yellow_rgb_a = Rgb::from(yellow_hsl);
    assert_eq!(yellow_rgb_a, yellow);
    
    let yellow_rgb_b = Rgb::from(yellow_cmyk);
    assert_eq!(yellow_rgb_b, yellow);
    
    assert!(close(yellow_hsv.hue(), 60.0, UNIT));
    assert!(close(yellow_hsv.saturation(), 1.0, UNIT));
    assert!(close(yellow_hsv.value(), 1.0, UNIT));
}

/// Tests color conversions for the color cyan.
#[test]
fn color_conversions_cyan() {
    let cyan = Rgb::from(0x00FFFF);
    let cyan_hsl = Hsl::from(cyan);
    let cyan_hsv = Hsv::from(cyan);
    let cyan_cmyk = Cmyk::from(cyan);
    
    assert!(close(cyan_hsl.hue(), 180.0, UNIT));
    assert!(close(cyan_hsl.saturation(), 1.0, UNIT));
    assert!(close(cyan_hsl.lightness(), 0.5, UNIT));
    
    assert_eq!(cyan_cmyk.c, 255);
    assert_eq!(cyan_cmyk.m, 0);
    assert_eq!(cyan_cmyk.y, 0);
    assert_eq!(cyan_cmyk.k, 0);
    
    let cyan_rgb_a = Rgb::from(cyan_hsl);
    assert_eq!(cyan_rgb_a, cyan);
    
    let cyan_rgb_b = Rgb::from(cyan_cmyk);
    assert_eq!(cyan_rgb_b, cyan);
    
    assert!(close(cyan_hsv.hue(), 180.0, UNIT));
    assert!(close(cyan_hsv.saturation(), 1.0, UNIT));
    assert!(close(cyan_hsv.value(), 1.0, UNIT));
}

/// Tests color conversions for the color magenta.
#[test]
fn color_conversions_magenta() {
    let magenta = Rgb::from(0xFF00FF);
    let magenta_hsl = Hsl::from(magenta);
    let magenta_hsv = Hsv::from(magenta);
    let magenta_cmyk = Cmyk::from(magenta);
    
    assert!(close(magenta_hsl.hue(), 300.0, UNIT));
    assert!(close(magenta_hsl.saturation(), 1.0, UNIT));
    assert!(close(magenta_hsl.lightness(), 0.5, UNIT));
    
    assert_eq!(magenta_cmyk.c, 0);
    assert_eq!(magenta_cmyk.m, 255);
    assert_eq!(magenta_cmyk.y, 0);
    assert_eq!(magenta_cmyk.k, 0);
    
    let magenta_rgb_a = Rgb::from(magenta_hsl);
    assert_eq!(magenta_rgb_a, magenta);
    
    let magenta_rgb_b = Rgb::from(magenta_cmyk);
    assert_eq!(magenta_rgb_b, magenta);
    
    assert!(close(magenta_hsv.hue(), 300.0, UNIT));
    assert!(close(magenta_hsv.saturation(), 1.0, UNIT));
    assert!(close(magenta_hsv.value(), 1.0, UNIT));
}

/// Tests color conversions for the color silver.
#[test]
fn color_conversions_silver() {
    let silver = Rgb::from(0xC0C0C0);
    let silver_hsl = Hsl::from(silver);
    let silver_hsv = Hsv::from(silver);
    let silver_cmyk = Cmyk::from(silver);
    
    assert!(close(silver_hsl.hue(), 0.0, UNIT));
    assert!(close(silver_hsl.saturation(), 0.0, UNIT));
    assert!(close(silver_hsl.lightness(), 0.75, UNIT));
    
    assert_eq!(silver_cmyk.c, 0);
    assert_eq!(silver_cmyk.m, 0);
    assert_eq!(silver_cmyk.y, 0);
    assert_eq!(silver_cmyk.k, 63);
    
    let silver_rgb_a = Rgb::from(silver_hsl);
    assert_eq!(silver_rgb_a, silver);
    
    let silver_rgb_b = Rgb::from(silver_cmyk);
    assert_eq!(silver_rgb_b, silver);
    
    assert!(close(silver_hsv.hue(), 0.0, UNIT));
    assert!(close(silver_hsv.saturation(), 0.0, UNIT));
    assert!(close(silver_hsv.value(), 0.75, UNIT));
}

/// Tests color conversions for the color gray.
#[test]
fn color_conversions_gray() {
    let gray = Rgb::from(0x808080);
    let gray_hsl = Hsl::from(gray);
    let gray_hsv = Hsv::from(gray);
    let gray_cmyk = Cmyk::from(gray);
    
    assert!(close(gray_hsl.hue(), 0.0, UNIT));
    assert!(close(gray_hsl.saturation(), 0.0, UNIT));
    assert!(close(gray_hsl.lightness(), 0.5, UNIT));
    
    assert_eq!(gray_cmyk.c, 0);
    assert_eq!(gray_cmyk.m, 0);
    assert_eq!(gray_cmyk.y, 0);
    assert_eq!(gray_cmyk.k, 127);
    
    let gray_rgb_a = Rgb::from(gray_hsl);
    assert_eq!(gray_rgb_a, gray);
    
    let gray_rgb_b = Rgb::from(gray_cmyk);
    assert_eq!(gray_rgb_b, gray);
    
    assert!(close(gray_hsv.hue(), 0.0, UNIT));
    assert!(close(gray_hsv.saturation(), 0.0, UNIT));
    assert!(close(gray_hsv.value(), 0.50, UNIT));
}

/// Tests color conversions for the color maroon.
#[test]
fn color_conversions_maroon() {
    let maroon = Rgb::from(0x800000);
    let maroon_hsl = Hsl::from(maroon);
    let maroon_hsv = Hsv::from(maroon);
    let maroon_cmyk = Cmyk::from(maroon);
    
    assert!(close(maroon_hsl.hue(), 0.0, UNIT));
    assert!(close(maroon_hsl.saturation(), 1.0, UNIT));
    assert!(close(maroon_hsl.lightness(), 0.25, UNIT));
    
    assert_eq!(maroon_cmyk.c, 0);
    assert_eq!(maroon_cmyk.m, 255);
    assert_eq!(maroon_cmyk.y, 255);
    assert_eq!(maroon_cmyk.k, 127);
    
    let maroon_rgb_a = Rgb::from(maroon_hsl);
    assert_eq!(maroon_rgb_a, maroon);
    
    let maroon_rgb_b = Rgb::from(maroon_cmyk);
    assert_eq!(maroon_rgb_b, maroon);
    
    assert!(close(maroon_hsv.hue(), 0.0, UNIT));
    assert!(close(maroon_hsv.saturation(), 1.0, UNIT));
    assert!(close(maroon_hsv.value(), 0.5, UNIT));
}

/// Tests color conversions for the color olive.
#[test]
fn color_conversions_olive() {
    let olive = Rgb::from(0x808000);
    let olive_hsl = Hsl::from(olive);
    let olive_hsv = Hsv::from(olive);
    let olive_cmyk = Cmyk::from(olive);
    
    assert!(close(olive_hsl.hue(), 60.0, UNIT));
    assert!(close(olive_hsl.saturation(), 1.0, UNIT));
    assert!(close(olive_hsl.lightness(), 0.25, UNIT));
    
    assert_eq!(olive_cmyk.c, 0);
    assert_eq!(olive_cmyk.m, 0);
    assert_eq!(olive_cmyk.y, 255);
    assert_eq!(olive_cmyk.k, 127);
    
    let olive_rgb_a = Rgb::from(olive_hsl);
    assert_eq!(olive_rgb_a, olive);
    
    let olive_rgb_b = Rgb::from(olive_cmyk);
    assert_eq!(olive_rgb_b, olive);
    
    assert!(close(olive_hsv.hue(), 60.0, UNIT));
    assert!(close(olive_hsv.saturation(), 1.0, UNIT));
    assert!(close(olive_hsv.value(), 0.5, UNIT));
}

/// Tests color conversions for the color green.
#[test]
fn color_conversions_green() {
    let green = Rgb::from(0x008000);
    let green_hsl = Hsl::from(green);
    let green_hsv = Hsv::from(green);
    let green_cmyk = Cmyk::from(green);
    
    assert!(close(green_hsl.hue(), 120.0, UNIT));
    assert!(close(green_hsl.saturation(), 1.0, UNIT));
    assert!(close(green_hsl.lightness(), 0.25, UNIT));
    
    assert_eq!(green_cmyk.c, 255);
    assert_eq!(green_cmyk.m, 0);
    assert_eq!(green_cmyk.y, 255);
    assert_eq!(green_cmyk.k, 127);
    
    let green_rgb_a = Rgb::from(green_hsl);
    assert_eq!(green_rgb_a, green);
    
    let green_rgb_b = Rgb::from(green_cmyk);
    assert_eq!(green_rgb_b, green);
    
    assert!(close(green_hsv.hue(), 120.0, UNIT));
    assert!(close(green_hsv.saturation(), 1.0, UNIT));
    assert!(close(green_hsv.value(), 0.5, UNIT));
}

/// Tests color conversions for the color purple.
#[test]
fn color_conversions_purple() {
    let purple = Rgb::from(0x800080);
    let purple_hsl = Hsl::from(purple);
    let purple_hsv = Hsv::from(purple);
    let purple_cmyk = Cmyk::from(purple);
    
    assert!(close(purple_hsl.hue(), 300.0, UNIT));
    assert!(close(purple_hsl.saturation(), 1.0, UNIT));
    assert!(close(purple_hsl.lightness(), 0.25, UNIT));
    
    assert_eq!(purple_cmyk.c, 0);
    assert_eq!(purple_cmyk.m, 255);
    assert_eq!(purple_cmyk.y, 0);
    assert_eq!(purple_cmyk.k, 127);
    
    let purple_rgb_a = Rgb::from(purple_hsl);
    assert_eq!(purple_rgb_a, purple);
    
    let purple_rgb_b = Rgb::from(purple_cmyk);
    assert_eq!(purple_rgb_b, purple);
    
    assert!(close(purple_hsv.hue(), 300.0, UNIT));
    assert!(close(purple_hsv.saturation(), 1.0, UNIT));
    assert!(close(purple_hsv.value(), 0.5, UNIT));
}

/// Tests color conversions for the color teal.
#[test]
fn color_conversions_teal() {
    let teal = Rgb::from(0x008080);
    let teal_hsl = Hsl::from(teal);
    let teal_hsv = Hsv::from(teal);
    let teal_cmyk = Cmyk::from(teal);
    
    assert!(close(teal_hsl.hue(), 180.0, UNIT));
    assert!(close(teal_hsl.saturation(), 1.0, UNIT));
    assert!(close(teal_hsl.lightness(), 0.25, UNIT));
    
    assert_eq!(teal_cmyk.c, 255);
    assert_eq!(teal_cmyk.m, 0);
    assert_eq!(teal_cmyk.y, 0);
    assert_eq!(teal_cmyk.k, 127);
    
    let teal_rgb_a = Rgb::from(teal_hsl);
    assert_eq!(teal_rgb_a, teal);
    
    let teal_rgb_b = Rgb::from(teal_cmyk);
    assert_eq!(teal_rgb_b, teal);
    
    assert!(close(teal_hsv.hue(), 180.0, UNIT));
    assert!(close(teal_hsv.saturation(), 1.0, UNIT));
    assert!(close(teal_hsv.value(), 0.5, UNIT));
}

/// Tests color conversions for the color navy.
#[test]
fn color_conversions_navy() {
    let navy = Rgb::from(0x000080);
    let navy_hsl = Hsl::from(navy);
    let navy_hsv = Hsv::from(navy);
    let navy_cmyk = Cmyk::from(navy);
    
    assert!(close(navy_hsl.hue(), 240.0, UNIT));
    assert!(close(navy_hsl.saturation(), 1.0, UNIT));
    assert!(close(navy_hsl.lightness(), 0.25, UNIT));
    
    assert_eq!(navy_cmyk.c, 255);
    assert_eq!(navy_cmyk.m, 255);
    assert_eq!(navy_cmyk.y, 0);
    assert_eq!(navy_cmyk.k, 127);
    
    let navy_rgb_a = Rgb::from(navy_hsl);
    assert_eq!(navy_rgb_a, navy);
    
    let navy_rgb_b = Rgb::from(navy_cmyk);
    assert_eq!(navy_rgb_b, navy);
    
    assert!(close(navy_hsv.hue(), 240.0, UNIT));
    assert!(close(navy_hsv.saturation(), 1.0, UNIT));
    assert!(close(navy_hsv.value(), 0.5, UNIT));
}