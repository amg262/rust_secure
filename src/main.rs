// Main function entry point for the Rust program.
fn main() {
    // `mut` keyword makes the `x` variable mutable, allowing its value to be changed.
    // Here, `x` is initialized with a value of 2.
    let mut x: i32 = 2;

    // Variables in Rust need to be initialized before they are used.
    // This line declares a variable `_y` of type i32, but it's unused.
    // Prefixing a variable with an underscore suppresses the unused variable warning.
    let _y: i32; // Declared but not used or initialized, so it's okay.

    // Adds 3 to the value of `x`, demonstrating mutability. `x` becomes 5.
    x += 3;

    // Checks if `x` is equal to 5. If not, the program will panic.
    assert_eq!(x, 5);
    // Prints "Success!" to the console if the assertion passes.
    println!("Success!");

    // Declares a mutable variable `a` and initializes it with 0.
    let mut a = 0;

    // Infinite loop.
    loop {
        // If `a` is equal to 5, exit the loop.
        if a == 5 {
            break;
        }
        // Print the value of `a` and then increment it by 1.
        println!("{:?} ", a);
        a += 1;
    }
    // Resets `a` back to 0 to demonstrate the while loop next.
    a = 0;

    // While loop that runs as long as `a` is not equal to 5.
    while a != 5 {
        // Print the current value of `a` and then increment it by 1.
        println!("{:?} ", a);
        a += 1;
    }

    // Calls `first_name` function and stores its return value in `f`.
    let f = first_name();
    // Declares a mutable variable `b` and initializes it with 2.
    let mut b = 2;
    // Calls `last_name` function with `f` as its argument.
    last_name(f);
    // Calls `add_one` function with a mutable reference to `b` and stores its return value in `d`.
    let d = add_one(&mut b);
    // Prints the value of `d`, which is now 3.
    println!("{:?}", d);
}

// Function that returns a `String`.
// Demonstrates creating and mutating a String.
fn first_name() -> String {
    // Creates a new mutable String variable `name` and initializes it with "John".
    let mut name = String::from("John");
    // Prints the value of `name`.
    println!("{:?}", name);
    // Returns `name`. Note: The `return` keyword is optional here; you could simply write `name`.
    return name;
}

// Function that takes a mutable `String` as a parameter and modifies it.
// This demonstrates passing and modifying mutable data.
fn last_name(mut name: String) {
    // Appends " Doe" to the `name`.
    name.push_str(" Doe");
    // Prints the modified `name`.
    println!("{:?}", name);
}

// This function demonstrates borrowing a mutable reference to an integer (`i32`) and modifying it.
// The parameter `x` is a mutable reference to an `i32`, denoted by `&mut i32`.
// Reference (&): Get a pointer to a value.
// Dereference (*): Access the value the pointer is pointing to.
fn add_one(x: &mut i32) -> &mut i32 {
    // Dereferences `x` to access and modify the value it points to.
    // The `*` operator is used to dereference `x`, allowing us to add one to its value.
    // This is a crucial aspect of Rust's safety, as it ensures that the function can only modify `x` if it has been explicitly allowed by the caller.
    *x += 1;

    // Returns a mutable reference to `x`.
    // This return allows further modifications to the original value outside this function,
    // demonstrating Rust's flexibility in managing references.
    x
}