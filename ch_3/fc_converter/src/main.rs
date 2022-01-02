use std::io;

fn main() {
    println!("Press 1 for Fahrenheit -> Celsius,\nPress 2 for Celsius -> Fahrenheit:");

    let selection = input();

    match selection {
        1 => {
            println!("Enter degrees Fahrenheit to convert to Celsius:");
            let selection = input();
            match selection {
                i64::MAX => {
                    println!("Degrees are usually measured in numbers, silly.");
                }
                _ => { 
                    let c = ((selection - 32) * 5) / 9;
                    println!("{}°F is equal to {}°C.", selection, c)
                }
            }
        },
        2 => {
            println!("Enter degrees Celsius to convert to Fahrenheit:");
            let selection = input();
            match selection {
                i64::MAX => {
                    println!("Degrees are usually measured in numbers, silly.");
                }
                _ => {
                    let f = ((selection * 9) / 5) + 32;
                    println!("{}°C is equal to {}°F.", selection, f)
                }
            }
        },
        _ => println!("Please enter a 1 or 2.")
    }
}

fn input() -> i64 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: i64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => i64::MAX,
    };
    input
}
