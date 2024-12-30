use std::fmt;

struct FRange {
    val: f64,
    end: f64,
    incr: f64,
}

fn range(x1: f64, x2: f64, skip: f64) -> FRange {
    FRange {
        val: x1,
        end: x2,
        incr: skip,
    }
}

impl Iterator for FRange {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.val;
        if res >= self.end {
            None
        } else {
            self.val += self.incr;
            Some(res)
        }
    }
}

impl fmt::Display for FRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "test")
    }
}

impl fmt::Debug for FRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

fn dump<T>(value: &T)
where
    T: fmt::Debug,
{
    println!("value is {:?}", value);
}

#[derive(Debug)]
struct Foo {
    name: String,
}

impl Foo {
    fn new(name: String) -> Self {
        Self { name }
    }
}

fn main() {
    for x in range(0.0, 1.0, 0.1) {
        println!("{:.1}", x);
    }

    let v: Vec<f64> = range(0.0, 1.0, 0.1).map(|x| x.sin()).collect();
    dump(&v);

    let f = Foo::new(String::from("Toni"));
    dump(&f);
}
