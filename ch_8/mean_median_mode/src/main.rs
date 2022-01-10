use std::io;

fn main() {
    let mut input = String::new();
    let mut v: Vec<usize> = Vec::new();


    println!("\nMean is the average value of a set of numbers. Median is the number that");
    println!("is closest to the middle of the set. Mode is the number that occurs the");
    println!("most often within the set.");
    println!("\nPlease type out a set of numbers, separated by spaces, or commas if you'd like.");
    println!("(Just don't use commas or periods as thousands separators.)");
    println!("\nI will return the mean, median, and mode for that set of numbers.\n");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    for c in input.trim().chars() {
        match c {
            '1'| '2'| '3' | '4' | '5' | '6' |'7' | '8' | '9' | '0' => (),
            ',' | ' ' => (),
            _ => {
                println!("\nPlease, numbers, spaces, or commas only.");
                return
            }
        }

    }
    let input = input.replace(",", " ");

    for num in input.split_whitespace() {
        let num: usize = num.parse().unwrap();
        v.push(num);
    }

    println!("{:?}", v);
}
