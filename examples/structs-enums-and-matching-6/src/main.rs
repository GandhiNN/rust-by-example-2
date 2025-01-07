enum Speed {
    Slow = 10,
    Medium = 20,
    Fast = 50,
}

enum Difficulty {
    Easy = 1,
    Medium, // 2
    Hard,   // 3
}

#[derive(Debug)]
enum Value {
    Number(f64),
    Str(String),
    Bool(bool),
}

fn eat_and_dump(v: &Value) {
    use Value::*;
    match *v {
        Number(n) => println!("number is {}", n),
        Str(ref s) => println!("string is '{}'", s),
        Bool(b) => println!("boolean is {}", b),
    }
}

fn main() {
    let s = Speed::Slow;
    let speed = s as u32;
    println!("speed {}", speed);

    use Value::*;
    let n = Number(2.3);
    let s = Str("hello".to_string());
    let b = Bool(true);

    println!("n {:?} s {:?} b {:?}", n, s, b);

    eat_and_dump(&n);
    println!("{:?}", n);

    eat_and_dump(&s);
    println!("{:?}", s);

    eat_and_dump(&b);
    println!("{:?}", b);
}
