fn main() {
    // TBC
    let mut range = 1..11;

    let one: Vec<_> = (&mut range).take(5).collect();
    let two: Vec<_> = range.collect();

    println!("{:?}", one);
    println!("{:?}", two);

    /* using `by_ref` to borrows an iterator, rather than consuming it */
    let mut words = ["hello", "world", "of", "Rust"].into_iter();

    // Take the first two words.
    let hello_world: Vec<_> = words.by_ref().take(2).collect();
    assert_eq!(hello_world, vec!["hello", "world"]);

    // Collect the rest of the words
    // We can only do this because we used `by_ref` earlier.
    let of_rust: Vec<_> = words.collect();
    assert_eq!(of_rust, vec!["of", "Rust"]);
}
