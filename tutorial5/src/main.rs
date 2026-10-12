//! # Tutorial 5 — Arithmetic and Type Casting
//!
//! This tutorial covers numeric arithmetic in Rust and the rules that trip people up
//! coming from dynamically typed languages:
//!
//! * Rust performs **no implicit numeric conversions**. `i16 + i32` does not compile.
//!   You must convert with `as` yourself.
//! * Integer division **truncates toward zero** and discards the fractional part.
//! * Arithmetic can **overflow**; how that is handled depends on the build profile.
//! * Numeric literals can carry a **type suffix** and **underscore** separators.
//! * Reading and parsing a number from stdin with `.trim().parse()`.

// `use std::io;` brings the `io` module into scope so we can call `io::stdin()`.
// We need it again at the bottom to read the user's input.
use std::io;

fn main() {
    // --- MIXED TYPES DO NOT AUTO-CONVERT ----------------------------------------
    //
    // We have to convert types manually. This is a deliberate design decision: Rust
    // refuses to silently change a number's type, because that can hide truncation or
    // sign bugs. The compiler error would be "cannot add `i8` to `u8`" and it will
    // even suggest casting one side.
    let x: u8 = 9; // unsigned 8-bit: range 0..=255
    let y: i8 = 10; // signed 8-bit:   range -128..=127
    // let z: i8 = 256; // ERROR at compile time (this one is caught statically): the
    //                  // literal 256 does not fit in i8, whose max is 127.
    // rust does NOT allow you to perform arithmetic with variables that have different
    // types (like i32 and i16). cast them dude CAST THEM.
    // We already mentioned `as` in main.rs from tutorial3.

    // `as` binds tighter than arithmetic operators, so `x as i8 + y` is
    // `(x as i8) + y`. Here x(9) becomes i8 9, then 9 + 10 = 19 (an i8).
    let z = x as i8 + y; // either cast x or y
    println!("{}", z); // 19

    let z = x as i8 - y; // 9 - 10 = -1
    println!("{}", z); // -1

    let z = x as i8 * y; // 9 * 10 = 90
    println!("{}", z); // 90

    let z = x as i8 / y;
    // Integer division truncates toward zero: 9 / 10 = 0. The remainder is thrown
    // away — no rounding, no decimals. x and y are i8 here, so z is also i8.
    println!("{}", z); // 0

    // To get the "real" fractional answer, cast to a floating type BEFORE dividing.
    let x: i16 = 255;
    let y: i16 = 10;
    let z: f32 = x as f32 / y as f32; // convert both operands to f32 first
    println!("{}", z); // 25.5

    // `%` is the remainder (modulo) operator: 255 % 10 = 5 (255 = 25*10 + 5).
    let z = x % y; // mod operation, we get the leftover of the division
    println!("{}", z); // 5

    // --- LITERAL SUFFIXES AND SEPARATORS ----------------------------------------
    //
    // A numeric literal may specify its type with a suffix directly:
    let x = 32.0f32; // f32 suffix on a float literal
    let y = 32_i16; // underscore separator + i16 suffix on an integer literal
    let z = 64 as u8; // you can ALSO use `as` to assign a type
    // `as` has higher precedence than `+`, so this is 32.0 + (32 as f32) + (64 as f32).
    println!("{}", x + y as f32 + (z as f32)); // 32.0 + 32.0 + 64.0 = 128
    // weird, right?
    // println!("{x + y}"); // ERROR: captured identifiers only allow a bare name;
    //                      // arbitrary expressions like `x + y` are not supported there.

    // Underscores are purely visual grouping inside numeric literals; they do not
    // change the value. Both of these are the SAME number: 127000.
    let w = 127000i64; // this seems unreadable, right?
    println!("{}", w);
    let w = 127_000_i64; // YES YOU CAN, prints 127000
    println!("{}", w);

    // `i32::MAX` is the largest value an i32 can hold (2_147_483_647). Casting it to
    // the wider i64 (which can hold far more) and adding 1 gives 2_147_483_648 without
    // overflow. The parentheses make the intended order explicit: cast first, add second.
    let q = (i32::MAX as i64) + 1;
    println!("{}", q); // 2147483648

    // --- INTEGER OVERFLOW -------------------------------------------------------
    //
    // The lines below COMPILE (both operands are u8) but would overflow/underflow at
    // runtime, so they are commented out.
    // let x: u8 = 255;
    // let y: u8 = 1;
    // let z = x + y;
    // `255 + 1 = 256` cannot be represented in u8 (max 255).
    // let z = y - x;
    // `1 - 255` is negative, and u8 cannot represent negatives.
    //
    // What ACTUALLY happens at runtime depends on the build profile:
    //   * Debug builds (default `cargo run`): overflow PANICS with
    //     "attempt to add with overflow" — a safety check helps you catch bugs.
    //   * Release builds (`cargo run --release`): overflow WRAPS around using
    //     two's-complement arithmetic (255 + 1 -> 0), with no error.
    // If you want defined behaviour regardless of profile, use explicit methods:
    //   `x.wrapping_add(y)` (wrap), `x.checked_add(y)` (return `Option`),
    //   `x.saturating_add(y)` (clamp to max/min), `x.overflowing_add(y)` (wrap + flag).

    // --- FLOAT CONVERSIONS LOSE PRECISION ---------------------------------------
    let fx: f32 = 12.01;
    let fy: f64 = 53.12;
    // `fx + fy` does NOT compile: f32 and f64 are different types.
    // CAST EM — but note that casting f64 -> f32 can LOSE PRECISION, because f32 has
    // fewer significand bits than f64. Here `fy as f32` is only approximate.
    let fz = fx + fy as f32; // cast the f64 operand down to f32, then add
    println!("{}", fz);

    // Mixing a float with an integer works the same way: convert the integer first.
    // Careful: `y` is NO LONGER i16 10 here — it was shadowed above by `let y = 32_i16;`,
    // so it is 32. Thus 12.01 + 32.0 = 44.01.
    let cz = fx + y as f32;
    println!("{}", cz);

    // --- PARSING A NUMBER FROM STDIN --------------------------------------------
    //
    // Read one line of text, exactly like tutorial 4.
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Expected to read line"); // panic with this message if reading fails

    // `.trim()` removes leading/trailing whitespace — importantly the trailing '\n'
    // that `read_line` kept. `.parse()` then interprets the text as the type demanded
    // by the annotation (i64). `.parse()` returns a `Result`, because the text might
    // not be a valid number; `.unwrap()` returns the value or panics on failure.
    // (Try running it and typing a non-number to see the panic.)
    let int_input: i64 = input.trim().parse().unwrap();
    println!("{}", int_input + 2); // add 2 and print, e.g. typing 40 prints 42
}
