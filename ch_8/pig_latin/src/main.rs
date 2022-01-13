// A few bugs that I'm going to ignore- 
// 1. if a word is all caps, it puts it to lowercase and only capitalizes the first letter
// 2. if a word contains no vowels it doesn't move any letters and slaps 'ay' on the end
// words like HTML are screwed twice - it becomes 'Htmlay'
// 3. I've found that copying text blocks from web pages and pasting in results in chars that
//    are more than one byte, throwing an error that is supposed to trigger with letters from 
//    foreign (outside the US) alphabets. Typed into the command line those blocks work, though. 

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
        println!("\nPlease only use english words and punctuation.\n");
        return
    }

    // put each word into a vec
    let mut text = Vec::new();
    for word in input.split_whitespace() {
        text.push(word);
    }

    // create another vec to hold the words once they have been manipulated
    let mut text_vec = Vec::new();
    for word in &text {
        let mut count = 0;

        // separate each word into 3 parts, front punctuation, word, end punctuation
        let mut front_punc = Vec::new();
        let mut letters = String::new();
        let mut end_punc = Vec::new();

        let mut front_punc_string = String::new();
        let mut front_punc_count: usize = 0;

        // check for front punctuation by iterating over the chars until a letter is
        // reached. store the punctuation in a vec and count how many there are.
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

        // flip the word around and do the same thing - iterate, store, count
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
        // push the (reversed) end punctuation into a string
        for &c in &end_punc {
            end_punc_string_rev.push(c);
        }

        // iterate over the (backwards) string in reverse and store it in another string
        for c in end_punc_string_rev.chars().rev() {
            end_punc_string.push(c);
        }
 
        // pop the front puctuation off the front, using the front_punc_count 
        // to guide the number of chars to remove
        let front_punc_removed = &word[front_punc_count..word.len()];

        // remove the end punctuation basically the same way
        let punc_removed = &front_punc_removed[0..front_punc_removed.len() - end_punc_count];

        let mut cap = false;
        let mut consanants = Vec::new();
        let mut consanant_count = 0;

        // iterate over the string with the punctuation removed
        for c in punc_removed.chars() {
            // start with a new 'letters' every iteration
            letters = punc_removed.to_string();
            match c  {
                // stop iterating when it hits a vowel
                'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => {
                    if count == 0 {
                        letters.push('w');
                        break
                    } else {
                        break
                    }
                },
                _ =>  {
                    // need to know if the first letter was capped, so the reformed
                    // word can be capped when finished.
                    if c.is_uppercase() {
                        cap = true;
                    }
                    // store the consanants before the first vowel in a vec & count them.
                    consanants.push(c);
                    consanant_count += 1;
                },
            }
            count += 1;
        }

        // push the consanants onto the end of the word
        for c in consanants {
            letters.push(c);
        }

        // remove the front consanants, using the consanant count, then push 'ay' on the end
        letters = letters[consanant_count..letters.len()].to_string();
        letters.push('a');
        letters.push('y');

        // if it was capped to begin with, capitalize the first letter of the new string
        if cap == true {
            letters = letters.to_lowercase();
            letters = cap_first_letter(letters);
        }

        // rebuild the whole word, adding back the front punctuation, pig-latinized
        // text, and end punctuation.
        let rebuilt_word = front_punc_string + &letters + &end_punc_string;
        text_vec.push(rebuilt_word);
    }

    // assemble the phrase back by printing each rebuilt word with a space in between
    print!("\n");
    for word in text_vec {
        print!("{} ", word);
    }
    println!("");
}

// the function to capitalize the first letter.
fn cap_first_letter(s: String) -> String {
    let mut letters = Vec::new();
    // iterate over the string and push chars into a vec
    for c in s.chars() {
        letters.push(c);
    }
    
    // copy the first letter (char) and convert it to a string
    let first_letter = letters[0].to_string();
    // set that one-letter string to uppercase (chars don't have a to_uppercase)
    let first_letter = first_letter.to_uppercase();

    // iterate over the one-character string as chars, and push it into
    // a vector (this is probably not the best way to do this, but I knew
    // it would work, and I couldn't find a simpler solution at the moment.)
    let mut chars = Vec::new();
    for c in first_letter.chars() {
        chars.push(c);
    }

    // replace the first char of the letters vec with the uppercase letter in the chars vec
    letters[0] = chars[0];

    // iterate over the letters vec and create a string
    let mut string = String::new();
    for c in letters {
        string.push(c);
    }
    string
}
