fn foo() {
    let x = 75;
    println!("{}", x);
}

fn foo_box() {
    let x = Box::new(30);
    println!("`x` points to {}", *x);
}

fn pointer_mutability() {
    let x = Box::new(11);
    let y = Box::new(31);
    // x = y; // Not allowed, x is immutable
    // *x = 43; // Not allowed, *x is immutable
    let mut x = Box::new(99);
    x = y; // OK, x is mutable
    *x = 43; // Ok, x is mutable
    println!("{}", x);
}

fn returning_owning_pointers() -> Box<i32> {
    Box::new(100)
}

fn consuming_owning_pointers(y: Box<i32>) {
    println!("{}", y);
}

fn pointer_to_unique_pointers() {
    let x = 123;
    let mut y = Box::new(x);
    *y = 44;
    println!("y is {}", y);
    println!("x is still {}", x);
}

fn main() {
    // Basic owning pointers
    println!("Hello, world!");
    foo();
    foo_box();
    pointer_mutability();

    // Returning owning pointers
    let y = returning_owning_pointers();
    println!("{}", y);

    // Consuming owning pointers
    let xx = Box::new(100);
    consuming_owning_pointers(xx);

    // Pointer to unique pointers
    pointer_to_unique_pointers();
}
