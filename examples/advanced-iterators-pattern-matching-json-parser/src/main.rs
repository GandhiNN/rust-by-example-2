use std::iter::Iterator;

struct CustomType {
    current: usize,
    max: usize,
}

impl CustomType {
    fn new(max: usize) -> Self {
        Self { current: 0, max }
    }
}

impl Iterator for CustomType {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.max {
            None
        } else {
            self.current += 1;
            Some(self.current)
        }
    }
}

fn main() {
    let custom = CustomType::new(10);

    for item in custom {
        println!("Item is {}", item);
    }
}
