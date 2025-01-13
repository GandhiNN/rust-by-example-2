use crate::shape;
use std::f64::consts;

pub struct Circle {
    pub radius: f64,
}

impl shape::Shape for Circle {
    type Area = f64;

    fn area(&self) -> Self::Area {
        consts::PI * self.radius * self.radius
    }
}
