#[derive(Debug)]
struct MyStruct {
    one: Vec<char>,
    two: Vec<char>,
    three: String,
}

fn main() {
    let my_string: String = "ABCD1234abcd".into();
    let my_iter = || my_string.chars();
    let my_struct = MyStruct {
        one: my_iter().collect(),
        two: my_iter().filter(|x| x.is_numeric()).collect(),
        three: my_iter().filter(|x| x.is_lowercase()).collect(),
    };
    println!("{:?}", my_struct);
}
