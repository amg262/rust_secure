// This is a simple program to demonstrate the use of mut and loops in Rust
/// # Mutability
/// In Rust, variables are immutable by default. This means that once a value is assigned to a variable, it cannot be changed.
/// To make a variable mutable, the mut keyword is used.
fn main() {
    // mut is required to make a variable mutable
    // i32 is a 32-bit signed integer, we have to assign a value to it
    let mut x: i32 = 2; // Uninitialized but used, ERROR !
    let _y: i32; // Uninitialized but also unused, only a Warning !
    x += 3;

    assert_eq!(x, 5);
    println!("Success!");

    let mut a = 0;

    loop {
        if a == 5 {
            break;
        }
        println!("{:?} ", a);
        a += 1;
    }
    a = 0;

    while a != 5 {
        println!("{:?} ", a);
        a += 1;
    }

    let f = first_name();
    last_name(f);
}

fn first_name() -> String {
    let mut name = String::from("John");
    println!("{:?}", name);
    return name;
}

fn last_name(mut name: String) {
    name.push_str(" Doe");
    println!("{:?}", name);
}