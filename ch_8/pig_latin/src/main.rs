use std::io;

fn main() {
    let mut input = String::new();

    println!("\nPlease enter an english word, phrase, or text, and I will convert it to pig latin.");
    println!("Also, if you want to include numbers, please spell them out.\n");

    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");

    // program will panic if the user hits enter without any input
    if input.starts_with('\r') || input.starts_with('\n') {
        println!("Please type at least one word.\n");
        return
    }

    // if there are numbers in the string, print error and return
    let mut count = 0;
    for c in input.trim().chars() {
        match c {
            '1'| '2'| '3' | '4' | '5' | '6' |'7' | '8' | '9' | '0' => {
                println!("\nC'mon, try it again without numbers.");
                return
            },
            _ => (),
        }
        count += 1;
    }

    // catches if any letter requires more than one byte, which filters out
    // words written in languages with letters the outside US-ASCII alphabet
    if input.trim().as_bytes().len() > count {
        println!("\nPlease only use english words and punctution.\n");
        return
    }

    println!("\n{}", input);

}
