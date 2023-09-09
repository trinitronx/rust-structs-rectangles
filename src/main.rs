/*! # An Example Program Using Structs
 *  To understand when we might want to use structs, let’s write a program that
 *  calculates the area of a rectangle. We’ll start by using single variables,
 *  and then refactor the program until we’re using structs instead.
 */

/// # A short program with one way of calculating the area of a rectangle
///
/// Take the width and height of a rectangle specified in pixels and calculate
/// the area of the rectangle.
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area_simple(width1, height1)
    );

    // Refactoring with Tuples
    tuples();

    // Refactoring with Structs: Adding More Meaning
    structs();

    // Adding Useful Functionality with Derived Traits
    debug_rect();
}

/// # Simple Area function
/// Calculating the area of a rectangle specified by separate `width` and
/// `height` variables
fn area_simple(width: u32, height: u32) -> u32 {
    width * height
}

/// # Refactoring with Tuples
///
/// Another version of our program that uses tuples.
fn tuples() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuples(rect1)
    );
}

/// # Calculate are of a rectangle with Tuples
///
/// Refactored area function using tuples
fn area_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// Refactoring with Structs: Adding More Meaning
/// # Rectangle struct to group `height` and `width` data
///
/// We use structs to add meaning by labeling the data.
/// We can transform the tuple we’re using into a struct with a name for
/// the whole as well as names for the parts, as shown this struct.
struct Rectangle {
    width: u32,
    height: u32,
}

/// # Refactoring with Structs: Adding More Meaning
///
/// Another version of our program that uses structs
fn structs() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_structs(&rect1)
    );
}

/// # Calculate area of a rectangle with Structs
///
/// Refactored area function using structs
fn area_structs(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

/// # Adding Useful Functionality with Derived Traits
///
/// An example of using derived trait `Debug` to print
/// the contents of a struct: `DebugRectangle`
#[derive(Debug)]
struct DebugRectangle {
    width: u32,
    height: u32,
}

/// # Adding Useful Functionality with Derived Traits
///
/// It’d be useful to be able to print an instance of `Rectangle` while we’re
/// debugging our program and see the values for all its fields.
///
/// The `println!` macro and `{}` format specifier won't work yet, because our
/// struct did not implement the `Display` trait.
///
/// Putting the specifier `:?` inside the curly brackets tells `println!` we
/// want to use an output format called `Debug`. The `Debug` trait enables us
/// to print our struct in a way that is useful for developers so we can see
/// its value while we’re debugging our code.
///
/// Rust includes functionality to print out debugging information, but we have
/// to explicitly opt in to make that functionality available for our struct.
/// To do that, we add the outer attribute `#[derive(Debug)]` just before the
/// struct definition, as shown in `DebugRectangle`.
///
/// When we have larger structs, it’s useful to have output that’s a bit easier
/// to read; in those cases, we can use `{:#?}` instead of `{:?}` in the
/// `println!` string. In this example, using the `{:#?}` style
/// will pretty-print the `DebugRectangle` struct.
///
/// The `dbg!` macro, which takes ownership of an expression
/// (as opposed to `println!`, which takes a reference),
/// prints the file and line number of where that `dbg!` macro call occurs in
/// your code along with the resultant value of that expression, and returns
/// ownership of the value.
fn debug_rect() {
    let rect1 = DebugRectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
    println!("Pretty-printed `rect1` is {:#?}", rect1);

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale), // this expression gets printed, then returned
        height: 50,
    };

    // the `dbg!` macro prints the file and line number + an owned & returned
    // expression to `stderr`
    dbg!(&rect1);
}
