mod calculation_spec;
mod two_dimentional;

use crate::calculation_spec::{Area, Circumference};

fn calculate_and_print_result_v0<T>(name: String, item: &T)
where
    T: Area,
{
    println!("{} area: {}", name, item.calculate_area())
}

fn calculate_and_print_result(name: String, item: &(impl Area + Circumference)) {
    println!("{} area: {}", name, item.calculate_area());
    println!("{} circumference: {}", name, item.calculate_circumference());
}

fn calculate_and_print_result_v2<T: Area + Circumference>(name: String, item: &T) {
    println!("{} area: {}", name, item.calculate_area());
    println!("{} circumference: {}", name, item.calculate_circumference());
}

fn calculate_and_print_result_v3<T>(name: String, item: &T)
where
    T: Area + Circumference,
{
    println!("{} area: {}", name, item.calculate_area());
    println!("{} circumference: {}", name, item.calculate_circumference());
}

fn new_circle_only_area(radius: i32) -> impl Area {
    two_dimentional::Circle { radius }
}

fn new_square(length: i32) -> impl Area + Circumference {
    two_dimentional::Square { length }
}

fn main() {
    let circle_one = two_dimentional::Circle { radius: 10 };
    let square_one = two_dimentional::Square { length: 5 };
    println!("circle area: {}", circle_one.calculate_area());
    println!("square area: {}", square_one.calculate_area());

    let circle_two = two_dimentional::Circle { radius: 7 };
    let square_two = two_dimentional::Square { length: 14 };
    calculate_and_print_result("circle".to_string(), &circle_two);
    calculate_and_print_result("square".to_string(), &square_two);

    let circle_three = two_dimentional::Circle { radius: 5 };
    let square_three = two_dimentional::Square { length: 8 };
    calculate_and_print_result_v2("circle_3".to_string(), &circle_three);
    calculate_and_print_result_v2("square_3".to_string(), &square_three);

    let circle_four = two_dimentional::Circle { radius: 6 };
    let square_four = two_dimentional::Square { length: 9 };
    calculate_and_print_result_v3("circle_4".to_string(), &circle_four);
    calculate_and_print_result_v3("square_4".to_string(), &square_four);

    let circle_five = new_circle(11);
    let square_five = new_square(13);
    // calculate_and_print_result_v3("circle_five".to_string(), &circle_five); // does not compile
    calculate_and_print_result_v0("circle_five".to_string(), &circle_five);
    calculate_and_print_result_v3("square_five".to_string(), &square_five);
}
