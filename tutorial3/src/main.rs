//! # Tutorial 3 — Primitive Data Types
//!
//! Rust's primitive types fall into two families:
//!
//! * **Scalar types** — a single value: integers, floating-point numbers, booleans,
//!   and characters.
//! * **Compound types** — several values grouped together: tuples and arrays.
//!
//! ## A note on "primitive" and `Copy`
//!
//! All scalar types (and tuples/arrays built from them) implement the `Copy` trait.
//! "Copy" means assigning them does a cheap bit-for-bit copy instead of moving
//! ownership. That is why, later in this file, `let q = x;` copies `x` and `x` remains
//! usable afterwards — you can keep using both `q` and `x`.

fn main() {
    // --- SCALAR TYPES -----------------------------------------------------------

    // == Integers ==
    //
    // Rust has signed integers (`iN`, can be negative) and unsigned integers (`uN`,
    // only zero and positive). N is the bit width. Available widths:
    //
    //   Signed:   i8, i16, i32, i64, i128, isize
    //   Unsigned: u8, u16, u32, u64, u128, usize
    //
    // `isize`/`usize` are pointer-sized (32 bits on 32-bit systems, 64 on 64-bit) and
    // are used for indexing and sizes. Signed types don't "have a more limited range"
    // overall — they split the same N bits between negatives and positives:
    //   i8 -> -128..=127        (2^7 negatives, 2^7 - 1 non-negatives)
    //   u8 -> 0..=255           (all 2^8 values are non-negative)
    // Both hold 2^8 = 256 distinct values; the signed one spends half of them on
    // negatives, so it cannot reach as high on the positive side.

    println!("Hello, world!");

    let x = 3; // an integer literal with no annotation DEFAULTS to `i32`
    println!("{}", x);

    let x: u16 = 2; // explicitly annotate the type: `u16` = unsigned 16-bit
    println!("{}", x);

    // Assigning a `Copy` scalar copies it. `q` takes a copy of `x`'s value AND type
    // (u16 here). Because it is a copy, neither binding borrows from the other.
    let q = x;
    println!("{q}, {x}"); // `{q}` / `{x}` are "captured identifiers": shorthand for
                          // `{}` + passing `q`/`x` as arguments.
    // btw this is a trick to use "{x}" instead of "{}", x
    // I will use the second one throughout the files

    // == Floating-point numbers ==
    //
    //   f32 — 32-bit IEEE-754 single precision
    //   f64 — 64-bit IEEE-754 double precision (the DEFAULT)
    let y: f32 = 10.92; // annotated f32; without the annotation this would be f64
    println!("{}", y);

    // == Booleans ==
    //
    // `bool` has exactly two values: `true` and `false`. Size is 1 byte.
    let z: bool = true; // or false
    println!("{}", z);

    // == Characters ==
    //
    // `char` is a single Unicode scalar value, 4 bytes wide, written with SINGLE
    // quotes. It can hold any Unicode character (e.g. 'a', 'ñ', '💣'), unlike a
    // `&str` which is written with double quotes and can hold several characters.
    let w: char = 'a';
    println!("{}", w);

    // --- COMPOUND TYPES ---------------------------------------------------------

    // == Tuples ==
    //
    // A tuple groups a FIXED number of values that may have DIFFERENT types.
    // Its type is written `(T1, T2, ...)`; its length is part of the type, so a
    // 3-element tuple is a different type from a 2-element tuple.
    let mut tup: (i32, bool, char) = (1, true, 's');
    // Implicitly you could write `let tup = (1, true, 's');`. The explicit
    // annotation pins each position to a concrete type:
    // let tup2: (u8, char) = (2122, 'a'); // ERROR: 2122 does not fit in u8 (0..=255)
    // println!("{}", tup);                 // ERROR: tuples do NOT implement Display,
    //                                      //        so `{}` cannot print the whole tuple
    //
    // Access individual elements with `.index`, starting at 0:
    println!("{}", tup.0); // 1
    println!("{}", tup.1); // true
    println!("{}", tup.2); // s

    tup.0 = 10; // mutate element 0 (allowed because `tup` is `mut`)
    println!("{}", tup.0);

    // Reassigning the WHOLE tuple is allowed, but the value must have the exact same
    // shape and types: 3 elements, `(i32, bool, char)`. You cannot change the arity.
    tup = (30, false, 'w');
    println!("{}", tup.0);
    println!("{}", tup.1);
    println!("{}", tup.2);

    tup.2 = w; // an element can also be assigned from a variable
    println!("{}", tup.2);

    tup = (12, z, 'd'); // ...and from several variables
    println!("{}", tup.0);
    println!("{}", tup.1);
    println!("{}", tup.2);

    // == Arrays ==
    //
    // An array holds a FIXED number of values, ALL of the same type `T`.
    // Its type is written `[T; N]` — both the element type AND the length are part
    // of the type. (For a growable, heap-allocated sequence you would use `Vec<T>`,
    // covered in a later tutorial.)
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5]; // five i32 values
    // Access with `name[index]`, starting at 0. Bounds are checked at runtime:
    // an out-of-range index panics instead of reading foreign memory.
    println!("{}", arr[0]); // 1
    println!("{}", arr[1]); // 2
    println!("{}", arr[2]); // 3
    println!("{}", arr[3]); // 4
    println!("{}", arr[4]); // 5

    arr[4] = 7; // reassign element at index 4
    println!("{}", arr[4]); // 7

    // An array can be declared WITHOUT an initialiser, as long as it is definitely
    // assigned before it is read. The type annotation is required here because there
    // is no initial value from which to infer the element type.
    let mut arr2: [i32; 3];
    // println!("{}", arr2[0]); // ERROR: used before being initialised
    // arr2[0] = 10;            // ERROR: cannot assign a single element of an
    // arr2[1] = 20;            //        uninitialised array — the whole array must
    // arr2[2] = 30;            //        be initialised at once
    // println!("{}, {}, {}", arr2[0], arr2[1], arr2[2]);
    arr2 = [1, 2, 3]; // now it is fully initialised
    println!("{}, {}, {}", arr2[0], arr2[1], arr2[2]);
    arr2 = [72, 12, 9]; // and can be reassigned wholesale
    println!("{}, {}, {}", arr2[0], arr2[1], arr2[2]);

    // let arr3: [i32; 3] = []; // ERROR: `[]` is an empty array, which is length 0;
    //                          // the annotation demands length 3

    // --- TYPE CASTING -----------------------------------------------------------
    //
    // The `as` keyword performs an explicit numeric cast. For integer -> integer it
    // truncates the high bits (mod 2^N); for float -> integer it truncates toward
    // zero; widening performed here (u16 -> i128) is lossless.
    let v: i128 = x as i128; // x is u16 = 2, so v = 2 as i128
    println!("{}", v);
}
