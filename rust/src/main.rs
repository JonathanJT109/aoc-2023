/*
* def fibonacci(n: unsigned int) -> vector of numbers
*   fibonacci numbers form 0 -> n
*   return vector of numbers
*/
use std::io;

fn fibonacci_sequence(n: &u32) -> Vec<u32> {
    let mut result = Vec::<u32>::new();
    let mut count: u32 = 0;
    let mut a: u32 = 0;
    let mut b: u32 = 1;

    println!("N is {}", n);

    if *n == 1 {
        result.push(a);
        return result;
    }

    while count < *n {
        result.push(a);
        let sum = a + b;
        a = b;
        b = sum;
        count += 1;
    }

    result
}

fn main() {
    // TODO: print() error

    println!("Please enter some number: ");
    let mut user_response = String::new();

    io::stdin()
        .read_line(&mut user_response)
        .expect("Failed to read line");

    let user_number: u32 = user_response.trim().parse().unwrap_or(1);

    let answer = fibonacci_sequence(&user_number);

    print!("Sequence: ");
    for num in &answer {
        print!("{num} ")
    }
    println!()
}
