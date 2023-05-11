fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let gifts = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five golden rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    for (index, day) in days.into_iter().enumerate() {
        print!("On the {} day of Christmas, my true love gave to me ", &day);
        for gift_i in (0..index + 1).rev() {
            if gift_i > 1 {
                print!("{}, ", &gifts[gift_i]);
            } else if gift_i == 1 {
                print!("{} ", &gifts[gift_i]);
            } else if index != 0 {
                println!("and {}.", &gifts[gift_i]);
            } else {
                println!("{}.", &gifts[gift_i]);
            }
        }
    }
}
