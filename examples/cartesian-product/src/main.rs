fn cartesian_product(a: Vec<String>, b: Vec<String>) -> Vec<Vec<String>> {
    let cross = a
        .iter()
        .flat_map(|a| b.iter().clone().map(move |a| (a, b)))
        .collect();
}

fn main() {
    println!("Hello, world!");
}
