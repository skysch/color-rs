
color-rs
========

This is an color library meant to provide flexible and convenient color manipulation functions.


Design
------

The design goals of this library are as follows:

+ Available operations must be independent of the underlying representation.
+ Most features of the library must be exposed through a single `Color` object.
+ Easily extensible.

At it's core, the library simply provides a collection of different color models (implemented as simple structs) along with a comprehensive suite of [`Into`](https://doc.rust-lang.org/std/convert/trait.Into.html) and [`From`](https://doc.rust-lang.org/std/convert/trait.From.html) implementations for each. Each of these structs provide functions for manipulating their state in terms of their own color models. The following color models are currently supported:

+ [`Rgb`](src/rgb.rs)
+ [`Hsl`](src/hsl.rs)
+ [`Hsv`](src/hsv.rs)
+ [`Cmyk`](src/cmyk.rs)
+ [`Xyz`](src/xyz.rs)

To make these functions and conversions implicit, there is an additional struct simply named [`Color`](), which provides access to all of the functions each color model provides. It does this by maintaining an internal default encoding (currently `Rgb`) and converting to and from this encoding whenever a function is called that would manipulate it in some manner not provided by the default encoding.

Future Plans
------------


+ More color spaces, especially more complex ones that are better models of human perception.
+ Better performance. So far everything has been implemented to the degree that it works.
+ Selectable underlying representation for `Color`.
+ Better interaction with external libraries. This would probably require some feature-gated conversions, and better alpha channel support.
+ Color depth generics.
+ More common application algorithms such as interpolation and blending, name lookup, palette functions, sort & compare, color correction, etc..