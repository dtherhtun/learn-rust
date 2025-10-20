fn main() {
    let gifts = [
        "a Partridge in a Pear Tree",
        "Two Turtle Doves",
        "Three French Hens",
        "Four Calling Birds",
        "Five Gold Rings",
        "Six Geese a Laying",
        "Seven Swans a Swimming",
        "Eight Maids a Milking",
        "Nine Ladies Dancing",
        "Ten Lords a Leaping",
        "Eleven Pipers Piping",
        "Twelve Drummers Drumming",
    ];

    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth",
    ];

    for day in 0..12 {
        println!("\nOn the {} day of Christmas my true love sent to me:", days[day]);

        for gift_index in (0..=day).rev() {
            if day > 0 && gift_index == 0 {
                println!("and {}", gifts[gift_index]);
            } else {
                println!("{}", gifts[gift_index]);
            }
        }
    }
}
