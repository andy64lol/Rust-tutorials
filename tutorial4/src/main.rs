//! # Tutorial 4 — Reading Console Input
//!
//! This tutorial shows how to read a line of text typed by the user from **standard
//! input** (`stdin`). It introduces a few ideas that are central to Rust:
//!
//! * **Modules and `use`** — `std::io` is a module in the standard library.
//! * **Ownership by reference** — `&mut input` is a *mutable borrow*.
//! * **`Result` and error handling** — `read_line` can fail, and `.expect(...)`
//!   is one way to handle that possibility.
//!
//! > Are `String`s "objects"? Not in the OOP sense. `String` is a struct (a growable,
//! > heap-allocated, UTF-8 text buffer) that has **methods**, like `.trim()` in the
//! > next tutorial. Rust has no classes and no inheritance; behaviour comes from
//! > methods and traits instead.

// `use` brings a path into scope so we can write `io::stdin()` instead of the full
// path `std::io::stdin()`. It is purely a name convenience, not an import at runtime.
use std::io;

fn main() {
    println!("Hello, world!");

    // `String::new()` creates a new, empty, growable UTF-8 string. It is `mut`
    // because `read_line` will append the user's input into it.
    let mut input = String::new(); // must be a string
    // the strings are somehow a object

    // `io::stdin()` returns a handle to the process's standard input.
    // `.read_line(&mut input)`:
    //   * reads bytes from stdin up to and INCLUDING the newline ('\n'),
    //   * APPENDS them to `input` (it does not replace existing content),
    //   * returns a `Result<usize, io::Error>` where the `Ok` value is the number of
    //     bytes read (0 means end-of-input).
    // `&mut input` passes a MUTABLE BORROW of `input` rather than moving it, so we
    // still own `input` afterwards and can print it.
    io::stdin()
        .read_line(&mut input)
        // `.expect("...")` consumes the `Result`:
        //   * on `Ok(v)` it returns `v`,
        //   * on `Err(e)` it PANICS, printing the given message plus the error.
        // This is fine for a tutorial; production code usually propagates the error
        // with the `?` operator instead of panicking.
        .expect("failed to read line."); // handles error

    // The input still contains the trailing newline we read, so the print below
    // ends up adding a blank line after it.
    println!("{}", input);
}
