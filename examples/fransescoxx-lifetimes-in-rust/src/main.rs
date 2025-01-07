// Explicit Lifetimes in Function Signatures
// This function accepts two references as its arguments
// and return the longest of the two (also still as a reference)
//
// The compiler will not be able to determine if one will outlive
// the other, so it won't risk to deallocate any of them 'intelligently'
// Hence we have to tell the compiler for how long the two references
// will live
//
// In this example, we tell the compiler that the two references will
// live as long as the 'lifetime' of the calling function
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn main() {
    let r;
    {
        let x = 5;
        r = &x;
        println!("{}", r); // x still lives here
    } // x goes out of scope here
      // println!("{}", r); // compile error

    let string1 = String::from("long string"); // static - will live as long as the runtime
    let string2 = "short"; // reference

    let result = longest(&string1, string2);
    println!("The longest string is {}", result);
}
