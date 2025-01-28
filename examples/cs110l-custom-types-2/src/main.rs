struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn next(&self) -> Option<&Box<Node>> {
        self.next.as_ref()
    }
}

struct LinkedList {
    head: Option<Box<Node>>,
    length: usize,
}

impl LinkedList {
    fn new() -> LinkedList {
        LinkedList {
            head: None,
            length: 0,
        }
    }

    fn len(&self) -> usize {
        self.length
    }

    fn front(&self) -> Option<&Box<Node>> {
        self.head.as_ref()
    }

    fn back(&self) -> Option<&Box<Node>> {
        let mut curr_opt = self.front();
        while curr_opt.is_some() {
            let curr_node = curr_opt.unwrap();
            if curr_node.next.is_none() {
                return Some(curr_node);
            }
            // Go to next node
            curr_opt = curr_node.next.as_ref();
        }
        // If the loop above didn't run, the list was empty
        // Return None
        None
    }

    fn front_mut(&mut self) -> Option<&mut Box<Node>> {
        self.head.as_mut()
    }

    fn push_front(&mut self, val: i32) {
        let node = Some(Box::new(Node {
            value: val,
            next: None,
        }));
        self.length = self.length + 1;
        let old_head = std::mem::replace(&mut self.head, node);
        self.head.as_mut().unwrap().next = old_head;
        let tmp = self.head.as_mut();
    }
}

fn main() {
    let mut l1 = LinkedList::new();
    println!("Length: {}", l1.len());
    l1.push_front(1);
    println!("Front: {}", l1.front().unwrap().value);
    println!("Back: {}", l1.back().unwrap().value);

    l1.push_front(2);
    println!("After adding 2:");
    println!("Front: {}", l1.front().unwrap().value);
    println!("Back: {}", l1.back().unwrap().value);

    let node_mut = l1.front_mut();
    node_mut.unwrap().value = 3;
    println!("After changing to 3:");
    println!("Front: {}", l1.front().unwrap().value);
    println!("Back: {}", l1.back().unwrap().value);

    let node_using_next = l1.front().unwrap().next().unwrap();
    println!(
        "Testing the `next` function on Node! Second element: {}",
        node_using_next.value
    );

    println!("Length after adding: {}", l1.len());
}
