/*
Scenario: we have a `Car` struct with a property
called `brand` which is of type `String`. Now, suppose
we have a variable `brand_1: String` and we want to convert it
to an instance of `Car`.

Solution: We need to implement the `From` trait for the `Car` struct
*/

#[derive(Debug)]
#[allow(dead_code)]
struct Car {
    brand: String,
}

impl From<String> for Car {
    fn from(brand: String) -> Self {
        Car { brand }
    }
}

impl From<Car> for String {
    fn from(car: Car) -> Self {
        car.brand
    }
}

fn main() {
    let brand_1 = String::from("FORD");
    let car_1 = Car::from(brand_1);
    println!("car_1: {:?}", car_1);

    let car_1_brand: String = car_1.into();
    println!("car_1_brand: {:?}", car_1_brand);
}
