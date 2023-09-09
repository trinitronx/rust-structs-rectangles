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
}

/// # Simple Area function
/// Calculating the area of a rectangle specified by separate `width` and
/// `height` variables
fn area_simple(width: u32, height: u32) -> u32 {
    width * height
}
