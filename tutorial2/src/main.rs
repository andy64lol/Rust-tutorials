//! # Tutorial 2 — Variables, Constants, Mutability, and Shadowing
//!
//! This tutorial covers the building blocks of storing data in Rust:
//!
//! * `const` — compile-time constants (must be annotated and are inlined).
//! * `let`   — runtime bindings, immutable by default.
//! * `let mut` — a binding you are allowed to reassign.
//! * Scoping — blocks `{ ... }` introduce a new scope; bindings die at the end of it.
//! * Shadowing — declaring a new `let` with the same name, which can even change the type.
//!
//! ## Key mental model
//!
//! In Rust a variable name (`let`) is *not* the same thing as a `const`. A `const` is a
//! value substituted at compile time and has no memory address of its own; a `let`
//! binding lives in a stack slot (for these simple types) at runtime.

fn main() {
    // --- CONSTANTS ---------------------------------------------------------------
    //
    // `const` declares a constant that is known and *evaluated at compile time*.
    // Rules that differ from `let`:
    //   * You MUST write the type explicitly (`u32` here); inference is not allowed.
    //   * The name is conventionally `SCREAMING_SNAKE_CASE`.
    //   * It is always immutable.
    //   * It can be declared at module scope (outside `main`) as well as inside functions.
    //   * The compiler may inline it at every use site — it is not a normal variable.
    const SECONDS_IN_MINUTES: u32 = 60; // `u32` = unsigned 32-bit integer (no negatives)

    // `{}` is a positional placeholder: it is replaced by the first argument after
    // the format string, here `SECONDS_IN_MINUTES`.
    println!("We have {} seconds in a minute.", SECONDS_IN_MINUTES);

    // Uncommenting the next two lines fails to compile. Unlike a `let` binding, a
    // `const` CANNOT be shadowed and CANNOT change its type: the name already refers
    // to a compile-time value, so redeclaring it with a different type is an error.
    // const SECONDS_IN_MINUTES: i64 = 10;
    // println!("This won't work! {}", SECONDS_IN_MINUTES);

    // --- MUTABLE VARIABLES ------------------------------------------------------
    //
    // `let` alone creates an IMMUTABLE binding: once assigned, it cannot be changed.
    // Adding `mut` makes it mutable. Mutability is about *reassignment*, and the type
    // is fixed for the lifetime of that binding — you may change the value, but to a
    // value of the SAME type.
    let mut x = 4; // no type given -> integer literals default to `i32` (signed 32-bit)
    println!("x is: {}", x);
    x = 5;         // allowed only because of `mut`
    println!("x is: {}", x);

    {
        // A `{}` block opens a NEW scope. Bindings created here are dropped/forgotten
        // when the block ends. Crucially, inside here `let x = ...` creates a brand-new
        // `x` that SHADOWS the outer `x` for the rest of this block only.
        //
        // The right-hand side `x - 2` reads the *outer* `x` (currently 5, a `let mut`),
        // computes 3, and binds the fresh immutable `x` to 3.
        let x = x - 2; // = 5 - 2 = 3
        println!("x is: {}", x); // prints 3
    } // <- inner `x` (3) goes out of scope here

    // Back at the outer scope, the original `x` is still 5, and it is still `mut`
    // (reassignable). The shadowed `x` from inside the block no longer exists.
    println!("x is: {}", x); // prints 5 (the outer `x` was never changed)

    // --- SHADOWING --------------------------------------------------------------
    //
    // A new `let x = ...` at the same scope shadows the previous `x`. The old binding
    // is not mutated or destroyed; it simply becomes unreachable by name. Unlike `mut`
    // reassignment, shadowing CAN change the type.
    let x = 6; // new binding; the previous `x` is now shadowed and unreachable
    println!("x is: {}", x); // prints 6

    // The initialiser on the right runs BEFORE the new binding exists, so it reads the
    // previous value (6) and produces a new immutable binding holding 7.
    let x = x + 1; // = 6 + 1 = 7
    println!("x is: {}", x); // prints 7

    // Shadowing ignores types entirely: this `x` is a `&str` string slice, whereas the
    // previous `x` was an integer. This is impossible with `mut` reassignment.
    let x = "Hello world!";
    println!("{}", x);
}
