use chrono::{DateTime, Utc};
use std::fs;
use std::path::Path;

fn print_string(string: &String) {
    println!("I got {} to print!", string);
}

fn rename_download_file() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("temporary_file.txt");
    fs::rename(path, "final_file.txt")?;
    Ok(())
}

fn basic_string_as_ref() {
    let a = String::from("hello");
    let b: &str = a.as_ref();
    println!("{}", b);
}

struct MyStruct(String);

impl AsRef<String> for MyStruct {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

fn main() {
    let a = String::from("Hello reader!");
    print_string(&a);
    println!("Checking whether we can still print the variable: {}", a);

    basic_string_as_ref();

    let aa = MyStruct(String::from("gandhi"));
    let bb = aa.as_ref();
    println!("{}", bb);
}
