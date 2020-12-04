

fn carol(day:u8, lyrics:&[&str], ordinal:&[&str]) {
    println!("On the {} day of Christmas my true love gave to me", ordinal[day as usize]);

    for i in (0..day+1).rev() {
        if day != 0 && i == 0 {
            print!("and ");
        } 
        println!("{}", lyrics[i as usize]);
    }

    println!("");

    if day < 12 {
        carol(day+1, lyrics, ordinal);
    }
}

fn main() {
    let lyrics = ["A partridge in a pear tree", "Two turtle doves", "Three French hens",
                    "Four calling birds", "Five gold rings", "Six geese a laying",
                    "Seven swans a swimming", "Eight maids a milking,", "Nine ladies dancing",
                    "Ten lords a leaping", "Eleven pipers piping", "Twelve drummers drumming"];
    let ordinal = ["first", "second", "third", "fourth", "fifth", "sixth", 
                    "seventh", "eight", "ninth", "tenth", "eleventh", "twelfth"];
    carol(0, &lyrics, &ordinal);
}
