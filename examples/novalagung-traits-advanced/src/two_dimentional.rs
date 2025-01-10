use crate::calculation_spec;
use std::f64::consts;

pub struct Circle {
    pub radius: i32,
}

impl calculation_spec::Area for Circle {
    fn calculate_area(&self) -> f64 {
        // cast to f64 since self.radius is i32
        consts::PI * (self.radius.pow(2) as f64)
    }
}

impl calculation_spec::Circumference for Circle {
    fn calculate_circumference(&self) -> f64 {
        // self.radius is i32
        2.0 * consts::PI * (self.radius) as f64
    }
}

pub struct Square {
    pub length: i32,
}

impl calculation_spec::Area for Square {
    fn calculate_area(&self) -> f64 {
        // (s ^ 2)
        // casting to f64 since self.length is i32
        self.length.pow(2) as f64
    }
}

impl calculation_spec::Circumference for Square {
    fn calculate_circumference(&self) -> f64 {
        4.0 * (self.length) as f64
    }
}
