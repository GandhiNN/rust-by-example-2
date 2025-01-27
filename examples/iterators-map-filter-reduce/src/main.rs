use std::collections::HashSet;

fn basic_iterator() {
    let v1 = [1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
}

fn basic_iterator_traits() {
    let v1 = [1, 2, 3];
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

fn use_iter() {
    // create a collection: Array of i32
    let numbers = [1, 2, 3];
    println!("before iter(): {:?}", numbers);

    // we are not taking ownership of the collection
    for num in numbers.iter() {
        println!("{}", num);
    }

    // thus we still can call `numbers`
    println!("after iter: {:?}", numbers);
}

fn use_iter_mut() {
    // create a collection: Array of i32
    let mut numbers = [1, 2, 3];
    println!("before iter_mut: {:?}", numbers);

    // we will modify the element
    for num in numbers.iter_mut() {
        *num += 1; // mutates each element by adding 1
        println!("{}", num)
    }

    // but `numbers` still have the ownership, so we can use it
    println!("after iter_mut: {:?}", numbers);
}

fn use_into_iter() {
    // create a collection: Array of String
    let stringz = [
        String::from("haha"),
        String::from("hehe"),
        String::from("hoho"),
    ];

    // use into_iter(): here, Vec<T> where T does not implement Copy
    // then the method will take ownership of the collection and its elements
    for stri in stringz.into_iter() {
        println!("{}", stri);
    }

    // strings is now inaccessible, because T = String does not implement Copy
    // println!("{:?}", stringz);

    // create a new collection: Array of i32
    let numbers = [1, 2, 3];

    // use into_iter(): here, Vec<T> where T implements Copy
    // then the collection will be fully copied
    for num in numbers.into_iter() {
        println!("{}", num);
    }

    // because the collection is copied, we can still access numbers
    println!("{:?}", numbers);
}

fn map_transforming_elements_to_vector() {
    let numbers = [1, 2, 3, 4, 5];
    // Type annotations are needed
    // Here, we transform the 'numbers' array into a vector
    let squares: Vec<_> = numbers.iter().map(|&x| x * x).collect();
    println!("Map - Squares: {:?}", squares);
}

fn transform_to_hashset() {
    let numbers = [1, 2, 1, 2, 3, 4, 5, 5, 6];
    // Type annotations are needed
    // Here, we transform the 'numbers' array into a hashset
    let unique: HashSet<_> = numbers.into_iter().collect();
    println!("Unique: {:?}", unique);
}

fn filtering_elements() {
    let numbers = [10, 20, 30, 40, 50];
    let even_numbers: Vec<_> = numbers.iter().filter(|&x| x % 2 == 0).collect();
    println!("Filter - Evens: {:?}", even_numbers);
}

fn fold_aggregating_elements() {
    let numbers = [1111, 222, 33, 10, 20, 40];
    // acc is the accummulator
    let sum = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("Fold - Sum: {}", sum);
}

fn main() {
    println!("basic_iterator()");
    basic_iterator();
    println!();

    println!("basic_iterator_traits()");
    basic_iterator_traits();
    println!();

    println!("use_iter()");
    use_iter();
    println!();

    println!("use_iter_mut()");
    use_iter_mut();
    println!();

    println!("use_into_iter()");
    use_into_iter();
    println!();

    println!("map_transforming_elements");
    map_transforming_elements_to_vector();
    println!();

    println!("transform_array_to_hashset");
    transform_to_hashset();
    println!();

    println!("filtering_elements");
    filtering_elements();
    println!();

    println!("fold_aggregating_elements");
    fold_aggregating_elements();
    println!();
}
