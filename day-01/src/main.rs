use std::fs;

fn parse_calibration_value(str: &str) -> u32 {
    let first_digit = str
        .chars()
        .filter(|c| c.is_numeric())
        .next()
        .unwrap()
        .to_digit(10)
        .unwrap();
    let last_digit = str
        .chars()
        .rev()
        .filter(|c| c.is_numeric())
        .next()
        .unwrap()
        .to_digit(10)
        .unwrap();
    first_digit * 10 + last_digit
}

fn main() {
    let calibration_document =
        fs::read_to_string("input.txt").expect("Could not read input calibration document");

    let sum = calibration_document
        .lines()
        .fold(0, |acc, str| acc + parse_calibration_value(str));

    println!("Sum of all of the calibration values: {sum}");
}
