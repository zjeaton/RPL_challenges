use std::io;

fn main() {
    let mut input = String::new();
    let mut text = Vec::new();

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
        println!("\nPlease only use english words and punctuation.\n");
        return
    }

    // put each word into a vec
    for word in input.split_whitespace() {
        text.push(word);
    }

    println!("\n{:?}", text);

    let mut text_vec = Vec::new();
    for word in &text {

        let mut count = 0;
        let mut front_punc = Vec::new();
        let mut letters = String::new();
        let mut end_punc = Vec::new();

        let mut front_punc_string = String::new();
        let mut front_punc_count: usize = 0;
        for c in word.chars() {
            match c {
                ('a'..='z') | ('A'..='Z') => break,
                _ => {
                    front_punc.push(c);
                    front_punc_count += 1;
                },
            }
        }
        for &c in &front_punc {
            front_punc_string.push(c);
        }
        println!("{}", front_punc_string);

        let mut end_punc_string = String::new();
        let mut end_punc_string_rev = String::new();
        let mut end_punc_count: usize = 0;
        for c in word.chars().rev() {
            match c {
                ('a'..='z') | ('A'..='Z') => break,
                _ => {
                    end_punc.push(c);
                    end_punc_count += 1;
                },
            }
        }
        for &c in &end_punc {
            end_punc_string.push(c);
        }

        for c in end_punc_string.chars().rev() {
            end_punc_string_rev.push(c);
        }

        let mut front_punc_removed = ""; 
        front_punc_removed = &word[front_punc_count..word.len()];
        println!("{}", front_punc_removed);

        let mut punc_removed = "";
        punc_removed = &front_punc_removed[0..front_punc_removed.len() - end_punc_count];
        println!("{}", punc_removed);

        for c in punc_removed.chars() {
            match c {
                'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => {
                    if count == 0 {
                        letters = punc_removed.to_string();
                        letters.push('w');
                        letters.push('a');
                        letters.push('y');
                        break
                    }
                },
                _ => letters = word.to_string(),
            }
            count += 1;
        }
        let rebuilt_word = front_punc_string + &letters + &end_punc_string_rev;
        text_vec.push(rebuilt_word);
    }
    println!("{:?}", text_vec);
}
