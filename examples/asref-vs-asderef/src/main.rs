use std::ops::Deref;

fn match_ref(input: &str) {
    let option_name: Option<String> = Some(input.to_owned());
    match option_name {
        Some(ref n) => println!("Name is {}", n),
        None => println!("No name provided"),
    }
    println!("{:?}", option_name)
}

fn match_with_ampersand(name: &str) {
    let option_name: Option<String> = Some(name.to_owned());
    match &option_name {
        Some(n) => println!("Name is {}", n),
        None => println!("No name provided"),
    }
    println!("{:?}", option_name)
}

fn try_greet(option_name: Option<&str>) {
    match option_name {
        Some(name) => println!("Name is {}", name),
        None => println!("No name provided"),
    }
}

fn my_as_deref<T: Deref>(x: &Option<T>) -> Option<&T::Target> {
    match *x {
        None => None,
        Some(ref t) => Some(t.deref()),
    }
}

fn greet(name: &str) {
    println!("Name is {}", name);
}

fn main() {
    match_ref("Gandhi");
    match_with_ampersand("Rendy");

    let option_name: Option<String> = Some("Alice".to_owned());
    try_greet(my_as_deref(&option_name));
    println!("{:?}", option_name);

    let option_name: Option<String> = Some("HEHEHE".to_owned());
    option_name.as_deref().map(greet);
    println!("{:?}", option_name);
}
