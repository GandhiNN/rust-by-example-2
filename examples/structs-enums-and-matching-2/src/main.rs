#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
}

// Creating associated functions
impl Person {
    fn new(first: &str, name: &str) -> Self {
        Self {
            first_name: first.to_string(),
            last_name: name.to_string(),
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn copy(&self) -> Self {
        Self::new(&self.first_name, &self.last_name)
    }

    fn set_first_name(&mut self, name: &str) {
        self.first_name = name.to_string();
    }

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

#[derive(Debug)]
struct A {
    s: &'static str,
}

fn how(i: u32) -> &'static str {
    match i {
        0 => "none",
        1 => "one",
        _ => "many",
    }
}

#[derive(Debug)]
struct B<'b> {
    s: &'b str,
}

fn main() {
    let p = Person::new("John", "Smith");
    println!("person p {} {}", p.first_name, p.last_name);
    println!("fullname p {}", p.full_name());

    let mut q = p.copy();
    println!("person q {} {}", q.first_name, q.last_name);
    println!("fullname q {}", q.full_name());
    q.set_first_name("Gandhi");
    println!("fullname q {}", q.full_name());

    let r = q.copy();
    let (first, last) = r.to_tuple(); // value of r moved
    println!("{} {}", first, last);
    // println!("{:?}", r); // r has been moved

    /* Lifetimes */
    let a = A { s: "hello dammit" };
    println!("{:?}", a.s);

    let res = how(3);
    println!("{}", res);

    let b = B { s: "huahaha" };
    println!("{:?}", b.s);
}
