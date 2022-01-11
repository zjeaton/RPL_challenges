use std::io;
use std::collections::HashMap;

fn main() {
    let mut input = String::new();
    let mut v: Vec<f64> = Vec::new();


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
            ',' | ' ' | '-' | '.' => (),
            _ => {
                println!("\nPlease, numbers, spaces, periods, dashes, or commas only.");
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
            let num = num as f64;
            v.push(num);
        } else if let Ok(num) = num.parse::<f64>() {
            v.push(num);
        } else {
            println!("If you're planning to use negative numbers, please put the - at the beginning.");
            return
        }
    }
    print_number_set(&v);

    compute_mean(&v);

    compute_median(&mut v);

    compute_mode(&v);
}

fn print_number_set(v: &Vec<f64>) {
    print!("\nYour number set: ");
    let mut count = 0;
    for num in v {
        print!("{}", num);
        if count == v.len() - 1 {
            println!("\n");
        } else {
            print!(", ");
        }
        count += 1;
    }
}

fn compute_mean(v: &Vec<f64>) {
    let mut sum: f64 = 0.0;
    let mut count = 0;
    for num in v {
        sum += num;
        count += 1;
    }
    let mean: f64 = sum / count as f64;
    println!("The mean of this number set is {}.\n", mean);
}

fn compute_median(v: &mut Vec<f64>) {
    // floats cannot be sorted by .sort() or .sort_unstable(); they must be ordered
    // using the cmp (comparison) module. Pass sort_by a closure, compare and unwrap.
    v.sort_by(|a,b| a.partial_cmp(b).unwrap());
    // if there are an odd number of elements in the vec, take the middle element
    if v.len() % 2 != 0 {
        println!("The median of this set is {}.\n", v[(v.len() - 1) / 2]);
    // if the number of elements are even, take the middle 2 and average them.    
    } else {
        let n1 = v[(v.len() / 2) - 1];
        let n2 = v[v.len() / 2];
        println!("The median of this set is {}.\n", (n1 + n2) / 2.0 );
    }
}

fn compute_mode(v: &Vec<f64>){
    // create map to store numbers in vec and count of occurences
    let mut map = HashMap::<String, u32>::new();
    for num in v {
        // cannot put f64 into HashMap as is, so convert to string.
        let num = num.to_string();
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }
    
    // put it back into a vector
    let mut num_count: Vec<_> = map.iter().collect();
    // place the occurences in decending order
    num_count.sort_by(|a,b| a.1.cmp(b.1).reverse());

    let mut count = 0;
    let mut occurences = 0;
    let mut nums = Vec::new();
    for (key, val) in &num_count {

        if *val > &1 {
            if occurences == 0 {
                occurences = **val;
                nums.push(key);
            } else {
                if *val < &occurences {
                    break
                } else {
                    nums.push(key);
                }
            }
            count += 1;

        } else if *val == &1 {
            if count == 0 {
                println!("There is no mode. All numbers occur just once.\n");
                return
            } else {
                break
            }
        }
    }

    // there are so many variations of mode to plan for.
    if &nums.len() == &num_count.len() {
        if &nums.len() == &1 {
            println!("There is no mode. {} is the only number in the series, and it occurs {} times.\n", &nums[0], &occurences);
        } else if &nums.len() != &1 {
            println!("There is no mode. All numbers occur {} times.\n", &occurences);
        }
    } else {
        if &nums.len() == &1 {
            println!("The mode of this series is {}. It occurs {} times.\n", nums[0], occurences);
        } else if &nums.len() == &2 {
            println!("The mode of this series is {} and {}. They both occur {} times.\n", nums[0], nums[1], occurences);
        } else {
            // sort, because hashmaps can place things in strange order that can be disorienting to read
            nums.sort();
            print!("The mode of this series is ");
            // print all but the last instance, so an an can be added before the last
            for index in 0..&nums.len() - 1 {
                print!("{}, ", nums[index]);
            }
            println!("and {}. They each occur {} times.\n", nums[nums.len() - 1], occurences);
        }
    } 
}