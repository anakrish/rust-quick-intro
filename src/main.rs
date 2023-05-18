// To run all snippets:
//  cargo test -- --nocapture

// Run specific snippet via:
//  cargo test <snippet name> -- --nocapture
// Snippets are written as tests in this tutorial.
// See https://doc.rust-lang.org/cargo/commands/cargo-test.html
// for further `cargo test` options.

// For quick debugging, use dbg! macro which prints values.
// Prints even in release mode. Not intended to be used in production code.
// See: https://doc.rust-lang.org/std/macro.dbg.html
// For production code, see https://docs.rs/log/0.4.17/log/macro.debug.html
#[test]
fn printf_debugging() {
    println!("here");
    let a = 2;
    dbg!(a > 0);
}

// To debug a test, note down the executable path from the first couple of lines of output:
//
// Output:
//    Finished test [unoptimized + debuginfo] target(s) in 0.00s
//     Running unittests src/main.rs (target/debug/deps/intro-cd7a6130aa23bcf7)
// ...
//
// Run the binary directly in gdb, passing in the same arguments passed to `cargo test`
// but without the -- separator.
//  gdb --args path/to/executable  <snippet name> --nocapture
//

// To format you code:
//  cargo fmt
// See https://rust-lang.github.io/rustfmt/?version=v1.5.1&search= for rustfmt configuration.

// To apply checks on your code, install clippy subcommand.
//  cargo install clippy
// To perform checks, do
//  cargo clippy
// To perform checks on all targets including tests do
//  cargo clippy --all-targets
fn vec_ops(v: &mut Vec<i64>) {
    if v.len() == 0 {
        println!("empty");
    } else {
        let first = v[0];
        println!("first = {}", first);
    }
}

// To generate documentation for your code:
//  cargo doc --open
// See https://doc.rust-lang.org/rustdoc/index.html
// for documentation comment syntax.
// Also see: https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html
// Example below:

/// This function increments a number.
///
/// # Arguments
///
/// * `n` - A number.
pub fn increment(n: i64) -> i64 {
    n + 1
}

// For code coverage, install tarpaulin or llvm-cov cargo commands.
// (You might also need to install libssl-dev package on debian based systems.)
//  cargo install cargo-tarpaulin
//  cargo install cargo-llvm-cov
//
// To generate code coverage report, do either of the following:
//  cargo-tarpaulin
//  cargo llvm-cov
//  cargo llvm-cov --open
// See https://github.com/xd009642/tarpaulin
//     https://github.com/taiki-e/cargo-llvm-cov
//     https://doc.rust-lang.org/rustc/instrument-coverage.html
fn is_positive(n: i64) -> bool {
    if n >= 0 {
        true
    } else {
        false
    }
}

#[test]
fn test_is_positive_basic() {
    assert_eq!(is_positive(5), true);
}

// Concepts:
// - assert!, assert_eq!
// - format!
// - let
// - macros
// - mut
// - println!
// - rebinding
// - string formatting
// - type inference
// - unused variables
#[test]
fn snippet_1() {
    // Variable bindings are created using `let` expressions.
    // Variables are immutable unless otherwise specified.
    // Types are inferred if possible.
    let x = 7;

    // println! macro is use for text output.
    // See https://doc.rust-lang.org/std/fmt/ for various format specifiers.
    println!("Value of x is {x} {x:03}");

    // format! is similar to println!, but constructs a String instead.
    let s = format!("Value of x is {x} {x:03}");
    println!("{s}");

    // `mut` is used to create variables whose values can be modified.
    let mut y = 7;
    y += 1;
    println!("y = {:03}", y - 1);

    // In tests, conditions can be checked using assert!, assert_eq! macros.
    assert!(x == y - 1);
    assert_eq!(x, y - 1);

    // Variables can be rebound.
    let x = "hello, world";
    println!("{x}");
}

// Utility to fetch type of a variable.
// For now, ignore this function.
fn type_name<T>(_: &T) -> &str {
    std::any::type_name::<T>()
}

#[test]
// Concepts:
// - :? Debug formatter
// - let destructuring
// - arrays
// - primitive types: integer types, str, bool, f32, f64
// - #[rustfmt::skip]
// - String
// - tuples
// - tuple.0
// - vec!
fn snippet_2() {
    // Ask rustfmt to ignore the following satatement.
    //    #[rustfmt::skip]
    let t1 = (
        // Suffixes are used to specify types of integer literals.
        1i8,
        2i16,
        3i32,
        4i64,
        1u8,
        2u16,
        3u32,
        4u64,
        true,
        // str is statically allocated with known compile-time size.
        "Hello",
        1.5f32,
        // Question: Why is a nested tuple used here?
        (
            1.0 / 3.0,
            // Arrays are created using the [value; count] syntax.
            [5; 3],
            // Vectors are created using vec! macro.
            vec![5; 3],
            // String is dynamically allocated.
            "Hello".to_owned(),
            // Unit type
            (),
        ),
    );

    // Display (default printing) is not provided for tuples and many other types.
    // The following line results in a compile error.
    // println!("{t1}");

    // Use the debug formatter :? to print values for debugging purposes.
    println!("t1 = {t1:?}");

    // Use :? for debug formatting.
    println!("t1 is a {}", type_name(&t1));

    // Access elements of tuples thus:
    println!("0th element {}", t1.0);
    println!("3rd element {}", t1.3);

    // Tuples can be destructured as below
    let (a, b) = (4, 8);
    println!("a+b = {}", a + b);
}

// Concepts:
// - ? operator
// - HashMap, HashSet
// - if-else is an expression
// - for loop
// - ParameterizedType::<T>::fcn
// - type inference
// - Option<T>
// - return
// - use
// - unwrap
fn snippet_3_code() -> Option<i32> {
    // See https://doc.rust-lang.org/std/ for standard library documentation.

    // To avoid having to type `std::collections::HashMap` everywhere.
    // See: https://doc.rust-lang.org/reference/items/use-declarations.html
    use std::collections::{HashMap, HashSet};

    // See: https://doc.rust-lang.org/std/collections/struct.HashMap.html
    let mut map = HashMap::new();
    // Rust can infer type based on first use.
    map.insert("A", 1);

    // For loop.
    // Note the destructuring of each entry in the map.
    // See https://doc.rust-lang.org/reference/expressions/loop-expr.html for other loops.
    for (k, v) in [("B", 2), ("C", 3)] {
        map.insert(k, v);
    }

    // Iterate through the items in the map.
    for (k, v) in &map {
        println!("{k} : {v}");
    }

    // Same as above.
    for (k, v) in map.iter() {
        println!("{k} : {v}");
    }

    println!("map = {map:?}");

    // Rust idiom is to use Option<T> to indicate whether a value exists or not.
    // See: https://doc.rust-lang.org/std/option/
    let (v1, v2) = (map.get("A"), map.get("X"));
    println!("v1 = {v1:?}, v2 = {v2:?}");

    // Use is_some(), is_none() to determine whether value exists or not.
    println!("queries: {} {}", v1.is_some(), v2.is_none());

    // Use if let to destructure an option value.
    if let Some(i) = v1 {
        println!("v1's value is {i}");
    }
    if let Some(i) = v2 {
        println!("v1's value is {i}");
    }

    // In Rust, if-else is an expression.
    // Here we use if let to fetch the value of an option to be able to use the value later.
    let v1_val = if let Some(i) = v1 {
        println!("executing if branch");
        i
    } else {
        // Use return to return from a function.
        return None;
    }; // Note the semicolon.
    println!("fetched v1_val using if-else. It is {v1_val}");

    // Use ? to return early from functions upon encountering a None.
    // This works only if the function itself returns an Option.
    let v1_val = v1?;
    println!("fetched v1_val using ?. It is {v1_val}");

    // The following line will cause a return out of the function since v2 is None.
    // Comment the two lines to let control proceed.
    let v2_val = v2?;
    println!("fetched v2_val using ?. It is {v2_val}");

    // Note, in C++ you'd call functions in parameterized types thus: ParameterizedType<T>::fcn.
    // In Rust, it is: ParameterizedType::<T>::fcn.
    //
    let mut set = HashSet::<&str>::new();

    // See https://doc.rust-lang.org/std/collections for available collection types.

    // The last expression in a function is its return value. Explicit `return` is not needed.
    Some(0)
}

#[test]
fn snippet_3() {
    snippet_3_code();
}

// Concepts:
//  - deref using *
//  - rustup doc [item]
//  - std::iter::Iterator
//    - any
//    - all
//    - collect
//    - filter
//    - fold
//    - map
//  - lambdas
//  - pass by value
//  - pass by &
//  - pass by &mut
fn compute_and_store(coeffs: &[f64], k: f64, values: &mut Vec<f64>) {
    // Start with v = 1
    let mut v = 1f64;
    for c in coeffs {
        // Multiply each coefficient
        v *= c;
    }
    // Add constant k
    v += k;
    values.push(v);
    println!("value 1 = {v}");

    // Type `rustup doc` in a terminal to bring up rust documentation.
    // Type `rustup doc std::iter::Iterator` to bring up documentation for std::iter.
    // Idiomatic rust typically  uses various operations over iterators instead of writing loops.
    let v = coeffs.iter().fold(1f64, |v, c| v * c) + k;
    //                               ^ operation (lambda)
    //                         ^ starting value
    values.push(v);
    println!("value 2 = {v}");

    let v = coeffs.iter().map(|c| c * 2.0).fold(1f64, |v, c| v * c) + k;
    println!("value 3 = {v}");

    // Some predicates:
    let q = (
        coeffs.iter().any(|c| *c < 0.0),
        coeffs.iter().all(|c| c > &0.0),
    );
    println!("q = {q:?}");

    // Filter coefficients.
    let set: Vec<&f64> = coeffs.iter().filter(|c| **c > 2.0).collect();
    println!("set 1 = {set:?}");
    let set: Vec<f64> = coeffs.iter().filter(|c| **c > 2.0).cloned().collect();
    println!("set 2 = {set:?}");
}

#[test]
// Concepts:
//  - &, &mut for arguments
//  - vec!
fn snippet_4() {
    let mut values = vec![];
    compute_and_store(&[1f64, 2.0, 3.0], 10f64, &mut values);
}

fn append(value: i64, collection: &mut Vec<i64>) {
    collection.push(value)
}

fn exists(value: i64, collection: &Vec<i64>) -> bool {
    collection.iter().any(|v| *v == value)
}

#[test]
// Concepts:
//  - borrow semantics
fn snippet_5() {
    let mut collection = vec![];
    append(5, &mut collection);

    let first = &collection[0];
    println!("first = {first}");

    let r = exists(5, &collection);
    println!("r = {r}, first = {first}");

    // Cannot pass a reference in a mutable reference context.
    // There is no way in rust to cast away "constness", unlike in C/C++.
    // Even in `unsafe` code.
    // append(6, &collection);

    // Append to collection.
    append(6, &mut collection);

    // Uncomment the following line.
    // println!("first = {first}");

    // It causes compile error:
    //    cannot borrow `collection` as mutable because it is also borrowed as immutable
    // This is because append takes a mutable reference to collection and can modify the
    // collection. There is no guarantee that any references we held for the collection
    // (e.g first) is valid anymore; modifying the collection could have caused a memory
    // allocation.
    // In Rust,
    //   1) there can be many active references to an object
    //   2) Or there can by only one mutable reference to an object.
    // This avoids a whole bunch of memory corruption errors at compile-time itself.
    let first = &collection[0];
    println!("first = {first}");

    let first = &mut collection[0];
    append(7, &mut collection);
    // Uncomment the following line to observe the error.
    //*first = 8;
}

// Concepts:
// - crates.io/
// - docs.rs
// - lazy_static
// - 'static lifetime
//
// To add a crate to your project, use `cargo add`
//  cargo add lazy_static
// It also shows which features of the crate are enabled or disabled.
//
// A crate for creating lazily evaluated statics is lazy_static.
//  cargo add lazy_static
// See https://docs.rs/lazy_static/latest/lazy_static/
//
// You can search for crates at https://crates.io/

// Import frequently used items from crates.
use lazy_static::lazy_static;
use std::collections::HashMap;

// Define a static table of state capitals using the lazy_static! macro.
lazy_static! {
    // When functions return references or variables store references, a lifetime
    // must be specified if the lifetime cannot be inferred.
    // The 'static lifetime indicates that the strs live as long as the program.
    // Rust enforces lifetime checks at compile time and prevents memory access errors
    // arising from use-after-free.
    static ref STATE_CAPITALS : HashMap<&'static str, &'static str> = {
    // Create and populate table of capitals.
    let mut table = HashMap::new();
    table.insert("Massachussetts", "Boston");
    table.insert("Washington", "Olympia");

    table
    };
}

// Concepts:
// - anyhow
// - closures
// - bail
// - Err
// - ok_or, ok_or_else to convert option to error.
// - Result
// - ? operator to propagate errors
//
//
// It also shows which features of the crate are enabled or disabled.
// anyhow crate is a popular crate for error management.
//  cargo add anyhow
// See https://docs.rs/anyhow/latest/anyhow/
use anyhow::{anyhow, bail, Result};

// The Result type is used to return values in functions that can raise errors.
// Use of Result with the ? operator leads to cleaner, idiomatic code.
fn get_capital_for_state(state: &str) -> Result<&'static str> {
    let capital = if let Some(c) = STATE_CAPITALS.get(state) {
        println!("Capital of {state} is {c}");
        c
    } else {
        // Error in Rust is conveyed using the Err type.
        // Use anyhow to create error and then return it.
        return Err(anyhow!("No capital found for {state}"));
    };

    // Same as above.
    let capital = if let Some(c) = STATE_CAPITALS.get(state) {
        println!("Capital of {state} is {c}");
    } else {
        // Use bail to create and return error.
        bail!("No capital found for {state}");
    };

    // Idiomatic to use ok_or.
    let capital = STATE_CAPITALS
        .get(state)
        .ok_or(anyhow!("No capital found for {state}"))?;

    // ok_or_else causes lazy evaluation of the error expression.
    // A closure is used to contruct the error.
    let capital = STATE_CAPITALS
        .get(state)
        .ok_or_else(|| anyhow!("No capital found for {state}"))?;

    // To return a result value, use Ok(value)
    Ok(capital)
}

// Concepts:
// - error propagation using ?
// - Unit type
#[test]
fn snippet_6() -> Result<()> {
    // Get the capital or propagate the error.
    let capital = get_capital_for_state("Washington")?;
    println!("Successfully fetched value {capital}");

    // Uncomment the following line to raise error.
    // let capital = get_capital_for_state("Texas");

    // Return unit value.
    Ok(())
}

// Concepts:
// - #[derive(trait)]
// To support debug printing using the :? format specifier, derive from
// the debug trait. Comment the line below to see what happens.
#[derive(Debug)]

// Concepts:
// - enum
// Define and enum that can hold a number or a string.
// An enum is similar to a discriminated union in C or a variant in Ocaml.
// See https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
enum Value {
    Number(i64),
    String(String),
}

// Concepts:
// - match
// - references
// - _ placeholder
fn is_number(v: &Value) -> bool {
    // This function takes a value by reference. The value cannot be modified since it
    // is not a mutable reference.

    // Use match expression to operate on an enum.
    // See https://doc.rust-lang.org/reference/expressions/match-expr.html
    match v {
        // The _ placeholder can be used if we don't care about a value.
        // Another way is to use an identifier with _ prefix: e.g: _n
        Value::Number(_) => true,

        // _ can be used to handle cases that are not explicitly specified in
        // a match expression.
        _ => false,
    }
}

// Concepts
// - as expression for type conversion.
// - match guards
fn sqrt(v: &Value) -> Result<Value> {
    Ok(match v {
        // Use a guard expression to add further constraints.
        // In this case, a guard is used to match positive numbers.
        Value::Number(n) if *n >= 0 => {
            // as expression is used to convert i64 to f64 so that
            // sqrt can be called. The result is converted back to i64 for
            // storing in a Value.
            Value::Number((*n as f64).sqrt() as i64)
        }
        Value::Number(_) => bail!("cannot compute sqrt of negative number"),
        _ => bail!("cannot compute sqrt of string"),
    })
}

// Concepts
// - use tuples to match over multiple values
fn add_values(a: Value, b: Value) -> Result<Value> {
    // To perform checks on more than 1 value at a time, it is idiomatic
    // to write a match expression over a tuple using those values.
    Ok(match (a, b) {
        (Value::Number(na), Value::Number(nb)) => Value::Number(na + nb),
        (Value::String(sa), Value::String(sb)) => Value::String(sa + &sb),
        _ => bail!("Cannot add values of different types"),
    })
}

#[test]
fn snippet_7() -> Result<()> {
    // Construct enums using variant names.
    let a = Value::Number(5);
    println!("a is number = {}", is_number(&a));

    let b = Value::Number(6);
    let c = add_values(a, b)?;
    println!("c = {c:?}");

    println!("sqrt(c) = {:?}", sqrt(&c)?);

    println!(
        "{:?}",
        add_values(
            Value::String("Hello".to_owned()),
            Value::String(" world!".to_owned())
        )
    );

    // Print the error instead of propagating it.
    println!(
        "{:?}",
        add_values(Value::Number(5), Value::String("abc".to_owned()))
    );

    Ok(())
}

fn main() {
    println!("Hello, world!");
}

// Future topics:
// file handling, system calls, executing other programs, concurrency support
// calling into C/C++, socket grpc/socket programming, passing parameters to functions, libraries
// borrow semantics, lifetime
