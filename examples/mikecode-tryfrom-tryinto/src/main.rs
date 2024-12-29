#[allow(dead_code)]
#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

impl TryFrom<(String, i32)> for Person {
    type Error = String;

    fn try_from(value: (String, i32)) -> Result<Self, Self::Error> {
        if value.1 > 100 {
            Err("Age is too old".to_string())
        } else {
            Ok(Person {
                name: value.0,
                age: value.1,
            })
        }
    }
}

fn main() {
    let t1 = ("Jim".to_string(), 20);
    let t2 = ("Joe".to_string(), 30);
    let p1: Result<Person, String> = t1.try_into();
    let p2 = Person::try_from(t2);

    match p1 {
        Ok(p) => println!("{:?}", p),
        Err(e) => println!("{}", e),
    }

    match p2 {
        Ok(p) => println!("{:?}", p),
        Err(e) => println!("{}", e),
    }
}
