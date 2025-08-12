
#![allow(dead_code, unused_variables, unused_assignments)]

//! Examples covering "Data Types" from *The Rust Programming Language*,
//! refactored to live inside a `lib.rs` so they can compile and be tested.
//! Each section mirrors the book's examples with light adaptations for a library crate.
//!
//! You can run `cargo test` to execute the included tests and see behavior.
//!
//! NOTE:
//! - The "type annotations needed" compile error from the book is shown here
//!   as a commented example, because keeping it live would break compilation.
//! - The "invalid array element access" example is provided in two ways:
//!   (1) a safe function that returns `Option<T>` instead of panicking, and
//!   (2) a direct indexing function used in a `#[should_panic]` test to demonstrate the runtime panic.
//!
//! This file groups examples into small functions so you can jump around or reuse them.

// ---------------------------------------------------------
// 1) Type annotations and `parse`
// ---------------------------------------------------------

/// Demonstrates adding a type annotation so `.parse()` knows what to produce.
pub fn parse_guess_with_type_annotation() -> u32 {
    let guess: u32 = "42".parse().expect("Not a number!");
    guess
}

/// This is the version that would fail to compile without a type annotation.
/// Leaving it as a comment so the crate compiles:
///
/// ```compile_fail
/// let guess = "42".parse().expect("Not a number!");
/// ```

// ---------------------------------------------------------
// 2) Integer types, literals, defaults
// ---------------------------------------------------------

/// Showcases declaration of various integer types and literals.
pub fn integer_types_and_literals() {
    // Defaults: integer literals default to i32 unless otherwise specified.
    let default_int = 123; // i32 by default

    // Explicit sizes and signedness
    let a: i8 = -5;
    let b: u8 = 250;
    let c: i16 = -1234;
    let d: u16 = 65535;
    let e: i32 = -12_345_678;
    let f: u32 = 12_345_678;
    let g: i64 = -9_223_372_036_854_775_808_i64 + 1;
    let h: u64 = 18_446_744_073_709_551_615_u64;
    let i: i128 = -170_141_183_460_469_231_731_687_303_715_884_105_728_i128;
    let j: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455_u128;

    // Arch-dependent sizes
    let k: isize = -1;
    let l: usize = 42;

    // Integer literals in various bases
    let dec = 98_222;
    let hex = 0xff;
    let oct = 0o77;
    let bin = 0b1111_0000;
    let byte: u8 = b'A';

    // Use variables to avoid "unused variable" warnings in doc builds
    let _ = (default_int, a, b, c, d, e, f, g, h, i, j, k, l, dec, hex, oct, bin, byte);
}

/// Returns the size in bits of isize / usize on this target.
pub fn arch_pointer_width_bits() -> (usize, usize) {
    (std::mem::size_of::<isize>() * 8, std::mem::size_of::<usize>() * 8)
}

// ---------------------------------------------------------
// 3) Integer overflow helpers
// ---------------------------------------------------------

/// Demonstrates standard library helpers for dealing with overflow.
pub fn overflow_helpers_example(x: u8, y: u8) -> (u8, Option<u8>, (u8, bool), u8) {
    use std::num::Wrapping;

    // wrapping_* (wraps in all modes)
    let wrapping = x.wrapping_add(y);

    // checked_* (returns None on overflow)
    let checked = x.checked_add(y);

    // overflowing_* (returns value + did_overflow flag)
    let overflowing = x.overflowing_add(y);

    // saturating_* (saturates at min/max)
    let saturating = x.saturating_add(y);

    (wrapping, checked, overflowing, saturating)
}

// ---------------------------------------------------------
// 4) Floating-point types and numeric ops
// ---------------------------------------------------------

/// Returns an f64 (default) and an explicitly typed f32.
pub fn floating_point_examples() -> (f64, f32) {
    let x = 2.0;     // f64
    let y: f32 = 3.0; // f32
    (x, y)
}

/// Basic numeric operations across integers and floats.
pub fn numeric_operations() -> (i32, f64, i32, f64, i32) {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // results in -1

    // remainder
    let remainder = 43 % 5;

    // return several representatives
    (sum, difference, product, quotient, truncated)
}

// ---------------------------------------------------------
// 5) Booleans
// ---------------------------------------------------------

pub fn boolean_examples() -> (bool, bool) {
    let t = true;
    let f: bool = false;
    (t, f)
}

// ---------------------------------------------------------
// 6) Characters
// ---------------------------------------------------------

pub fn char_examples() -> (char, char, char) {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit annotation
    let heart_eyed_cat = '😻';
    (c, z, heart_eyed_cat)
}

// ---------------------------------------------------------
// 7) Tuples
// ---------------------------------------------------------

pub fn tuple_make() -> (i32, f64, u8) {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    tup
}

pub fn tuple_destructure_y() -> f64 {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    y
}

pub fn tuple_indexing() -> (i32, f64, u8) {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    (five_hundred, six_point_four, one)
}

/// Demonstrates unit type `()` return.
pub fn unit_example() {
    // Implicitly returns () (unit)
}

// ---------------------------------------------------------
// 8) Arrays
// ---------------------------------------------------------

pub fn array_make_and_access() -> (i32, i32) {
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    (first, second)
}

pub fn array_type_annotation() -> [i32; 5] {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    a
}

pub fn array_repeated_init() -> [i32; 5] {
    let a = [3; 5];
    a
}

pub fn months_array() -> [&'static str; 12] {
    let months = [
        "January", "February", "March", "April", "May", "June",
        "July", "August", "September", "October", "November", "December"
    ];
    months
}

/// Safe access: returns `Some(value)` if index is in-bounds, else `None`.
pub fn array_get_safe(a: &[i32; 5], index: usize) -> Option<i32> {
    a.get(index).copied()
}

/// Direct indexing (may panic if out-of-bounds). Used by the `#[should_panic]` test below.
pub fn array_index_unsafe(a: &[i32; 5], index: usize) -> i32 {
    a[index]
}

// ---------------------------------------------------------
// 9) Tests mirroring book behavior
// ---------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_guess() {
        assert_eq!(parse_guess_with_type_annotation(), 42u32);
    }

    #[test]
    fn test_pointer_widths() {
        let (isize_bits, usize_bits) = arch_pointer_width_bits();
        assert!(isize_bits == 32 || isize_bits == 64);
        assert!(usize_bits == 32 || usize_bits == 64);
    }

    #[test]
    fn test_overflow_helpers() {
        // choose values that overflow u8 when added
        let (wrap, checked, (overflowed_val, did_overflow), sat) = overflow_helpers_example(250, 10);
        assert_eq!(wrap, 4);               // 260 -> wrap to 4
        assert_eq!(checked, None);         // overflow -> None
        assert_eq!(did_overflow, true);    // overflow happened
        assert_eq!(sat, u8::MAX);          // saturates at 255
        let _ = overflowed_val;            // not asserting exact value here
    }

    #[test]
    fn test_floats_and_ops() {
        let (x, y) = floating_point_examples();
        assert_eq!(x, 2.0);
        assert_eq!(y, 3.0f32);

        let (sum, difference, product, quotient, truncated) = numeric_operations();
        assert_eq!(sum, 15);
        assert!((difference - 91.2).abs() < 1e-10);
        assert_eq!(product, 120);
        assert!((quotient - (56.7 / 32.2)).abs() < 1e-10);
        assert_eq!(truncated, -1);
    }

    #[test]
    fn test_booleans_and_chars() {
        assert_eq!(boolean_examples(), (true, false));
        let (c, z, heart) = char_examples();
        assert_eq!(c, 'z');
        assert_eq!(z, 'ℤ');
        assert_eq!(heart, '😻');
    }

    #[test]
    fn test_tuples() {
        assert_eq!(tuple_make(), (500, 6.4, 1));
        assert_eq!(tuple_destructure_y(), 6.4);
        assert_eq!(tuple_indexing(), (500, 6.4, 1));
    }

    #[test]
    fn test_arrays() {
        assert_eq!(array_make_and_access(), (1, 2));
        assert_eq!(array_type_annotation(), [1, 2, 3, 4, 5]);
        assert_eq!(array_repeated_init(), [3, 3, 3, 3, 3]);

        let months = months_array();
        assert_eq!(months.len(), 12);
        assert_eq!(months[0], "January");
    }

    #[test]
    fn test_array_safe_and_unsafe_access() {
        let a = [1, 2, 3, 4, 5];
        assert_eq!(array_get_safe(&a, 0), Some(1));
        assert_eq!(array_get_safe(&a, 9), None);
        assert_eq!(array_index_unsafe(&a, 1), 2);
    }

    /// Demonstrates the runtime panic when indexing out of bounds,
    /// mirroring the book's "Invalid Array Element Access" behavior.
    #[test]
    #[should_panic]
    fn test_array_index_out_of_bounds_panics() {
        let a = [1, 2, 3, 4, 5];
        // Intentionally index past the end to trigger a panic:
        let _ = array_index_unsafe(&a, 10);
    }
}
