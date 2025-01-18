fn iterator_filter(a: Vec<i32>) {
    let mut iter = a.iter().filter(|x| x.is_positive());
    println!("{:#?}", iter);
}

fn main() {
    let a = [0i32, 1, 2];
    iterator_filter(a.to_vec());
}
