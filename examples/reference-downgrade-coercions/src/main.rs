struct RefHolder<'a> {
    x: &'a i64,
}

impl<'a> RefHolder<'a> {
    fn new(x: &'a i64) -> RefHolder<'a> {
        RefHolder { x }
    }
}

fn print_num(y: &i64) {
    println!("y: {}", y);
}

fn main() {
    // Create `x`
    let mut x = 10;

    // Make sure `y` is `&mut i64`
    let y = &mut x;

    // Package the downgraded reference into a struct.
    let z = RefHolder::new(y);

    // Print `y` downgrading it to an `&i64`
    print_num(y);

    // Use the `z` reference again
    println!("z.x: {}", z.x);
}
