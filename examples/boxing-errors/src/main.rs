use std::fmt::{Display, Formatter, Result};
use std::{error, result};

// Change the alias to use `Box<dyn error::Error>`
type CodeResult<T> = result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct EmptyVec;

impl Display for EmptyVec {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EmptyVec {}

fn double_first(vec: Vec<&str>) -> CodeResult<i32> {
    vec.first()
        .ok_or_else(|| EmptyVec.into()) // converts to Box
        .and_then(|s| {
            s.parse::<i32>()
                .map_err(|e| e.into()) // converts to Box
                .map(|i| 2 * i)
        })
}

fn print(result: CodeResult<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}
