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
