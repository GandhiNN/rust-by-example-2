fn process_numbers(numbers: &[i32]) -> Vec<i32> {
    numbers
        .iter()
        .filter(|&n| n > &0) // keep only positive numbers
        .map(|&n| n * 2) // double each number
        .filter(|&n| n % 3 == 0) // keep only multiples of 3
        .collect() // collect again into a Vec<i32>
}

fn main() {
    let numbers = vec![-2, 5, 8, 11, -9, 3, 4];
    let result = process_numbers(&numbers);
    println!("Processed numbers are: {:?}", result); // [6]
}
