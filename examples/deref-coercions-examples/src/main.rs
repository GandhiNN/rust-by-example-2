use std::ops::Deref;

struct MyBox<T>(Box<T>);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct MyBoxExample<T> {
    a: T,
}

impl<T> Deref for MyBoxExample<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.a
    }
}

struct AnotherBox<T>(T);

impl<T> AnotherBox<T> {
    fn hello(x: T) -> AnotherBox<T> {
        AnotherBox(x)
    }
}

impl<T> Deref for AnotherBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn print_int_reference(m: &i32) {
    println!("{}", m)
}

fn main() {
    // Example 1: Usage
    let x = MyBox(Box::new(5)); // wrapped the value of 5 into MyBox
    let _y = &x; // use the value inside MyBox as i32: type = &MyBox<i32>
    let z = &*x; // type: &i32
    println!("z: {}", z);

    // Example 2: Regular references
    let a = 20;
    let b = &a;
    if a == *b {
        println!("a and *b are equal");
    } else {
        println!("they are not equal");
    }

    // Example 3: Box<T> as reference
    let a = 11;
    let b = Box::new(a);
    println!("Value of *b is {}", *b);

    // Example 5: Smart Pointer as reference
    let a = 8;
    let b = MyBoxExample { a };
    println!("Value of *b is {}", *b);

    // Example 6: Coerce deref
    let b = AnotherBox::hello(111);
    print_int_reference(&b);
}
