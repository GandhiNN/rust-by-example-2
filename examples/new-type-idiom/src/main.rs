struct Years(i64);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}

struct Days(i64);

impl Days {
    /// Truncates partial years
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn is_adult(age: &Years) -> bool {
    age.0 >= 18
}

fn main() {
    println!("Hello, world!");
    let age = Years(25);
    let age_days = age.to_days();
    println!("Is an adult? {}", is_adult(&age));
    println!("Is an adult? {}", is_adult(&age_days.to_years()));
    // println!("Is an adult? {}", is_adult(&age_days);
}
