use std::ops::Deref;

// Custom smart pointer
// The underlying value will be stored on the stack
// instead of the heap
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn take_str_example(str: &str) {
    println!("{}", str);
}

fn take_string(string: &String) {
    println!("result of take_string: {}", string)
}

fn take_str(str: &str) {
    println!("result of take_str: {}", str)
}

fn main() {
    // Value is in the stack
    let x = 10;

    // Regular pointer to x
    let x_ptr = &x;

    // Dereferencing for the sake of example
    println!("Value using regular pointer: {}", *x_ptr);

    // Smart pointer (Box) to an integer on the heap
    let x = Box::new(10);

    // Dereference to underlying value happens automatically
    println!("Value using smart pointer: {}", *x);

    let x = MyBox::new(10);
    println!("Value using our smart pointer: {}", *x); // compiler error

    // Deref coercion without the complicated syntax
    let x = Box::new(String::from("some string literal"));

    // Box -> String -> str -> &str
    take_str_example(&(*(*x)));

    // Box -> String then takes a string slice to get &str
    take_str_example(&(*x)[..]);

    // Calling deref on Box then on String
    take_str_example(x.deref().deref());

    // Deref coercion which is equivalent to the above
    take_str_example(&x);

    let xx = Box::new(String::from("hehehe another example"));

    // Rust will resolve to deref only once to match signature
    take_string(&xx);

    take_str(&xx);
}
