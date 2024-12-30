use std::fmt;
use std::num::ParseFloatError;

#[derive(Debug)]
struct MyError {
    details: String,
}

impl MyError {
    fn new(msg: &str) -> Self {
        Self {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

/*  description() is deprecated since 1.42 */
// impl Error for MyError {
//     fn description(&self) -> &str {
//         &self.details
//     }
// }

impl From<ParseFloatError> for MyError {
    fn from(err: ParseFloatError) -> Self {
        MyError::new(err.to_string().as_str())
    }
}

fn parse_f64(s: &str, yes: bool) -> Result<f64, MyError> {
    raises_my_error(yes)?;
    let x: f64 = s.parse()?;
    Ok(x)
}

// A test function that returns our error result
fn raises_my_error(yes: bool) -> Result<(), MyError> {
    if yes {
        Err(MyError::new("borked"))
    } else {
        Ok(())
    }
}

fn main() {
    println!(" {:?}", parse_f64("42", false));
    println!(" {:?}", parse_f64("42", true));
    println!(" {:?}", parse_f64("?42", false));
}
