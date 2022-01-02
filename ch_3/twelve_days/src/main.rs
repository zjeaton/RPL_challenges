fn main() {
    let mut index = 1;

    let a = [ 
        "a partridge in a pear tree.",
        "2 turtle doves", 
        "3 french hens", 
        "4 calling birds", 
        "5 golden rings", 
        "6 geese a laying", 
        "7 swans a swimming",
        "8 maids a milking", 
        "9 ladies dancing", 
        "10 lords a leaping", 
        "11 pipers piping", 
        "12 drummers drumming"
    ];

    while index <= 12 {
        let suff = match index {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th"
        };

        let and = match index {
            1 => "",
            _ => "and "
        };

        println!("On the {}{} day of Christmas,", index, suff);
        println!("My true love gave to me");

        if index > 1 { 
            let mut gift_index = index;
            while gift_index > 1 {
                println!("{}", a[gift_index - 1]); 
                gift_index -= 1;
            }
        }
        println!("{}{}", and, a[0]);

        println!("");
        index += 1;
    }
}
