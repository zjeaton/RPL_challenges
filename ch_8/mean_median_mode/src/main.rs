use std::io;

fn main() {
    let mut input = String::new();
    let mut v: Vec<isize> = Vec::new();


    println!("\nMean is the average value of a set of numbers. Median is the number that");
    println!("is closest to the middle of the set. Mode is the number that occurs the");
    println!("most often within the set.");
    println!("\nPlease type out a set of numbers, separated by spaces, or commas if you'd like.");
    println!("(Just don't use commas or periods as thousands separators.)");
    println!("\nI will return the mean, median, and mode for that set of numbers.\n");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // checks to ensure that every character in the string is a digit, comma, space, or dash.
    for c in input.trim().chars() {
        match c {
            '1'| '2'| '3' | '4' | '5' | '6' |'7' | '8' | '9' | '0' => (),
            ',' | ' ' | '-' => (),
            _ => {
                println!("\nPlease, numbers, spaces, or commas only.");
                return
            }
        }

    }
    // replaces all commas with spaces in the input string
    let input = input.replace(",", " ");

    for num in input.split_whitespace() {
        // since the - symbol can be placed anywhere within the string and it will pass
        // the 'is it a number' test in the for loop above, the if let Ok(num) ensures that
        // it actually is a valid number, with the - symbol at the beginning of the digits.
        // num.parse::<isize>() will convert the split string to an isize, and it will push
        // onto the vector if it is valid. 
        if let Ok(num) = num.parse::<isize>() {
            v.push(num);
        } else {
            println!("If you're planning to use negative numbers, please put the - at the beginning.");
            return
        }
    }

    println!("{:?}", v);
}
