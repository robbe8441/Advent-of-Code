use std::{fs::File, io::Read};

static DIGITS: [(&str, &str); 9] = [
    ("one", "o1ne"),
    ("two", "t2wo"),
    ("three", "th3ree"),
    ("four", "fo4ur"),
    ("five", "fi5ve"),
    ("six", "s6ix"),
    ("seven", "se7ven"),
    ("eight", "eig8ht"),
    ("nine", "ni9ne"),
];

fn main() {
    let mut string = String::new();
    let mut file = File::open("./input.txt").unwrap();
    file.read_to_string(&mut string).unwrap();

    let mut result = 0;

    for line in string.lines() {
        let mut res_str: String = line.to_string();

        for &(word, rep) in DIGITS.iter() {
            res_str = res_str.replace(word, rep);
        }

        let digits: Vec<u32> = res_str.chars()
            .filter_map(|c| c.to_digit(10))
            .collect();

        if let (Some(first_digit), Some(last_digit)) = (digits.first(), digits.last()) {
            result += first_digit * 10 + last_digit;
        }
    }

    println!("{:?}", result);
}