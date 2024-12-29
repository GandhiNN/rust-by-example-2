use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file_v1() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

/*
If the `Result` has an `Ok(T)` variant, then the `?` operator will
return the value inside the `Ok` to the expression and the program
will continue

If the `Result` has an `Err(e)` variant, then the `?` operator will
return the error `Err(E)` to the caller function.
*/

fn read_username_from_file_v2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

/*
The `?` operator allows chaining of operation
*/
fn read_username_from_file_v3() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn main() {
    println!("Hello, world!");
    let _ = read_username_from_file_v1();
    let _ = read_username_from_file_v2();
    let _ = read_username_from_file_v3();
}
