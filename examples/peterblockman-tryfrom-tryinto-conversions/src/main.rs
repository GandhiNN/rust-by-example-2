/*
Scenario: let's say we want to ensure that our `Car`
struct does not accept an empty brand name.
*/

#[derive(Debug)]
#[allow(dead_code)]
struct Car {
    brand: String,
}

impl TryFrom<String> for Car {
    type Error = &'static str;

    fn try_from(brand: String) -> Result<Self, Self::Error> {
        if brand.is_empty() {
            Err("Invalid brand name")
        } else {
            Ok(Car { brand })
        }
    }
}

impl TryFrom<Car> for String {
    type Error = &'static str;
    fn try_from(car: Car) -> Result<Self, Self::Error> {
        if car.brand == "KIA" || car.brand == "BMW" {
            Err("Not allowed!")
        } else {
            Ok(car.brand)
        }
    }
}

fn main() {
    let brand_1 = String::from("FORD");
    let car_1 = Car::try_from(brand_1).unwrap();
    println!("car_1: {:?}", car_1);

    let car_2 = Car::try_from(String::from("")).unwrap_err();
    println!("car_2: {:?}", car_2);

    let brand_1: Result<String, &'static str> = car_1.try_into();
    println!("brand_1: {:?}", brand_1.unwrap());

    let car_3 = Car::try_from(String::from("KIA")).unwrap();
    let brand_2: Result<String, &'static str> = car_3.try_into();
    println!("brand_2: {:?}", brand_2.unwrap_err());
}
