use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct MyStruct {
    message: String,
}

fn convert_json_to_struct() {
    // create a raw JSON string from the json! macro and
    // turn it into a MyStruct struct
    let raw_json_string = json!({"message": "Hello World!"});
    let my_struct: MyStruct = serde_json::from_str(&raw_json_string.to_string()).unwrap();
    println!("{:?}", my_struct);
}

fn main() {
    println!("Hello, world!");
    convert_json_to_struct();
}
