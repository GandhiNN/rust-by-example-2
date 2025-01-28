struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

fn main() {
    // Assembling a list
    let mut first = Box::new(Node {
        value: 1,
        next: None,
    });
    let mut second = Box::new(Node {
        value: 2,
        next: None,
    });
    let third = Box::new(Node {
        value: 3,
        next: None,
    });

    // Try to connect them into a list
    second.next = Some(third);
    first.next = Some(second);

    // Looping over the list
    let mut curr: Option<&Box<Node>> = Some(&first);
    while curr.is_some() {
        // we can unwrap the Option because we know for sure that curr is Some
        println!("{}", curr.unwrap().value);
        let next: &Option<Box<Node>> = &curr.unwrap().next;
        curr = next.as_ref();
    }
}
