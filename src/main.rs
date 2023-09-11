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

    // Implementation of `Rectangle` struct with `area` method
    area_method();

    // Implementation of `Rectangle` struct with `width` "is set?" method
    width_is_set_method();
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

/// # Method Syntax
/// 
/// Methods are similar to functions: we declare them with the fn keyword and a
/// name, they can have parameters and a return value, and they contain some
/// code that’s run when the method is called from somewhere else. Unlike
/// functions, methods are defined within the context of a struct (or an enum
/// or a trait object, which we cover in [Chapter 6][1] and [Chapter 17][2],
/// respectively), and their first parameter is always self, which represents
/// the instance of the struct the method is being called on.
///
/// ## Defining Methods
///
/// Let’s change the area function that has a Rectangle instance as a parameter
/// and instead make an area method defined on the Rectangle struct, as shown
/// in `area_method()`
/// 
/// [1]: https://doc.rust-lang.org/book/ch06-00-enums.html
/// [2]: https://doc.rust-lang.org/book/ch17-02-trait-objects.html
fn area_method() {
    let rect1 = RectangleMethod {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

/// # Implementation of `Rectangle` struct with `area` method
/// 
/// Same as before, we define a struct with `width` and `height` attributes
#[derive(Debug)]
struct RectangleMethod {
    width: u32,
    height: u32,
}

/// # Implementation of `Rectangle` with `area` method
/// 
/// This time, we also implement an `area()` method to calculate area.
impl RectangleMethod {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

/// # Method Syntax - Methods & Fields w/same name
/// 
/// Note that we can choose to give a method the same name as one of the
/// struct’s fields. For example, we can define a method on Rectangle that is
/// also named `width`
/// 
/// ## Defining Methods - `&self` shorthand 
///
/// `&self` is actually short for `self: &Self`. Within an impl block, the type
/// `Self` is an alias for the type that the `impl` block is for. Methods must
/// have a parameter named `self` of type `Self` for their first parameter, so
/// Rust lets you abbreviate this with only the name `self` in the first
/// parameter spot. Note that we still need to use the `&` in front of the
/// `self` shorthand to indicate that this method borrows the `Self` instance,
/// just as we did in rectangle: `&Rectangle`. Methods can take ownership of
/// `self`, borrow `self` immutably, as we’ve done here, or borrow
/// `self` mutably, just as they can any other parameter.
/// 
/// We chose `&self` here for the same reason we used `&Rectangle` in the
/// function version: we don’t want to take ownership, and we just want to read
/// the data in the struct, not write to it. If we wanted to change the
/// instance that we’ve called the method on as part of what the method does,
/// we’d use `&mut self` as the first parameter. Having a method that takes
/// ownership of the instance by using just `self` as the first parameter
/// is rare; this technique is usually used when the method transforms `self`
/// into something else and you want to prevent the caller from using the
/// original instance after the transformation.
/// 
/// When we follow `rect1.width` with parentheses, (e.g. `width()`) Rust knows
/// we mean the method `width`. When we don’t use parentheses, Rust knows we
/// mean the field `width`.
fn width_is_set_method() {
    let rect1 = RectangleMethodWidth {
        width: 30,
        height: 50,
    };

    println!(
        "The width of the rectangle is set? {}",
        rect1.width()
    );
}

/// # Implementation of `Rectangle` struct with `width` method
/// 
/// Same as before, we define a struct with `width` and `height` attributes
#[derive(Debug)]
struct RectangleMethodWidth {
    width: u32,
    height: u32,
}

/// # Implementation of `Rectangle` with `width` method
/// 
/// This time, we implement an `width()` method to return `true` when `width`
/// is nonzero, and `false` when `width` is `0`.
impl RectangleMethodWidth {
    fn width(&self) -> bool {
        if self.width > 0 { true } else { false }
    }
}