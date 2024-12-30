use std::fmt;
trait Show {
    fn show(&self) -> String;
}

// add new method for i32 primitive
impl Show for i32 {
    fn show(&self) -> String {
        format!("four-byte signed {}", self)
    }
}

// add new method for f64 primitive
impl Show for f64 {
    fn show(&self) -> String {
        format!("eight-byte float {}", self)
    }
}

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn new(fname: String, lname: String) -> Self {
        Self {
            first_name: fname,
            last_name: lname,
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

impl fmt::Debug for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.full_name())
    }
}

fn main() {
    let answer = 42;
    let maybe_pi = 3.147;
    let s1 = answer.show();
    let s2 = maybe_pi.show();
    println!("show {}", s1);
    println!("show {}", s2);

    // Instantiate struct
    let p = Person::new("Gandhi".to_string(), "Ngakan".to_string());
    let res = p.full_name();
    println!("{}", res);
}
