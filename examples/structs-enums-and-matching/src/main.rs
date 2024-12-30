// pass by reference
fn dump(s: &str) {
    println!("{}", s);
}

fn add_mul(x: f64, y: f64) -> (f64, f64) {
    (x + y, x * y)
}

fn main() {
    /* Ownership Basic */
    let s1 = "hello dolly";
    dump(s1);
    println!("s1 {}", s1);

    // block scoping
    let a = 10;
    let b = "hello";
    {
        let c = "hello".to_string();
        // a, b, and c are visible
        println!("{}", a);
        dump(b);
        dump(c.as_str());
    }

    // the string `c` is dropped
    // a, b are still visible
    for i in 0..a {
        let b = &b[1..];
        // original b is no longer visible, it's shadowed
    }
    // the slice `b` is dropped
    // i is __not__ visible!

    /* Tuples */
    let t = add_mul(2.0, 10.0);

    // can debug print
    println!("t {:?}", t);

    // can 'index' the value
    println!("add {} mul {}", t.0, t.1);

    // can 'extract' values
    let (add, mul) = t;
    println!("add {} mul {}", add, mul);

    // Tuples can contain different types
    let tuple = ("hello", 5, 'c');

    assert_eq!(tuple.0, "hello");
    assert_eq!(tuple.1, 5);
    assert_eq!(tuple.2, 'c');

    // using `enumerate`
    println!("Using enumerate method on Tuple");
    for t in ["zero", "one", "two"].iter().enumerate() {
        println!("{} {}", t.0, t.1);
    }

    // using `zip`
    println!("Using zip method on Tuple");
    let names = ["ten", "hundred", "thousand"];
    let nums = [10, 100, 1000];
    for p in names.iter().zip(nums.iter()) {
        println!("{:?}", p);
        println!("{} {}", p.0, p.1);
    }
}
