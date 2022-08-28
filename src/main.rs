fn main() {
    let numerals = vec!["first".to_string(), "second".to_string(), "third".to_string(), "fourth".to_string(), "fifth".to_string(), "sixth".to_string(), "seventh".to_string(), "eighth".to_string(), "ninth".to_string(), "tenth".to_string(), "eleventh".to_string(), "twelfth".to_string()];
    let gifts = vec!["A partridge in a pear tree", "Two turtledoves", "Three French hens", "Four calling birds", "Five gold rings (five golden rings)", "Six geese a-laying", "Seven swans a-swimming", "Eight maids a-milking", "Nine ladies dancing", "Ten lords a-leaping", "Eleven pipers piping", "Twelve drummers drumming"];

    if numerals.len() != gifts.len() {
        panic!("Numerals and gifts should have same length!");
    }

    for i in 0..numerals.len() {
        println!("On {} day of Christmas, my true love sent to me", numerals[i]);

        for j in (0..=i).rev() {
            let phrase = match j {
                _ if i!=0 && j == 0 => format!("And {}", gifts[j].to_lowercase()),
                _ => format!("{}", gifts[j])
            };

            println!("{phrase}");
        }
        println!();
    }
}
