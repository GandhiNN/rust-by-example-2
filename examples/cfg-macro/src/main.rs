/*
    Configuration conditional checks are possible through two different operators:
    1. The cfg attribute: #[cfg(...)] in attribute position -> enables conditional compilation
    2. The cfg! macro: cfg!(...) in boolean expressions -> evaluates to boolean literals for checks at run-time
*/

// This function only gets compiled if the target OS is Linux
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

// This function only gets compiled if the target OS is *not* linux
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!");
}

fn main() {
    are_you_on_linux();

    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }
}
