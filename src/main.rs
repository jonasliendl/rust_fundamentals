
fn get_item(index: usize) {
    //let index = 3; // this looks like an unsigned integer, but it's actually a usize
    let vec = vec![1, 2, 3, 4, 5];

    // Retrieve a value at a specific index
    let value = vec.get(index).unwrap();

    // print the value
    println!("The value at index {} is {:?}", index, value);
}

fn sum_vec(vec: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in vec {
        sum += i;
    }
    sum
}

fn main() {
    let vec: Vec<i32> = vec![1, 10, 230, 213];
    get_item(3);

    println!("Sum of vec: {}", sum_vec(&vec));

    // Retrieve a value at a specific index
    match vec.get(2) {
        Some(value) => println!("The value at index 2 is: {}", value),
        None => println!("There is no value at index 2!"),
    }

    match vec.last() {
        Some(value) => println!("The last value in the vector is: {}", value),
        None => println!("The vector is empty!"),
    }

    // Retrieve the last value

    // Retrieve the first value using pattern matching
    match vec.first() {
        Some(first_value) => println!("The first value in the vector is: {}", first_value),
        None => println!("The vector is empty!"),
    }
}
