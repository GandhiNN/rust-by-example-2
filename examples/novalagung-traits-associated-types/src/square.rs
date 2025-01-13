use crate::shape;

pub struct Square {
    pub side: i64,
}

impl shape::Shape for Square {
    type Area = i64;

    fn area(&self) -> Self::Area {
        self.side * self.side
    }
}
