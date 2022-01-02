use std::io;

fn main() {
    let mut x: u128 = 0;
    let mut y: u128 = 1;
    let mut input = String::new();
    let mut n: u128 = 1;

    println!("\nPlease type a number from 1 to 186.");
    println!("I will find the Fibonacci number that corresponds to that index.");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: u8 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => u8::MAX,
    };
    
    if input <= 186 {
        for _ in 1..input {
            n = x + y;
            x = y;
            y = n;
        }
        println!("The Fibonacci number at index {} is {}.", input, n)
    } else {
        println!("I said a number from 1 to 186.")
    }
}
