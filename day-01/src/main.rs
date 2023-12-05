use std::fs;

fn main() {
    let calibration_document =
        fs::read_to_string("input.txt").expect("Could not read input calibration document");

    let sum_1 = calibration_document
        .lines()
        .fold(0, |acc, str| acc + parse_calibration_value_1(str));

    println!("--- Day 1: Trebuchet?! ---\n");
    println!("Part 1");
    println!("Sum of all of the calibration values: {sum_1}");

    let sum_2 = calibration_document
        .lines()
        .fold(0, |acc, str| acc + parse_calibration_value_2(str));

    println!("Part 2");
    println!("Sum of all of the calibration values: {sum_2}");
}

fn parse_calibration_value_1(str: &str) -> u32 {
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

fn parse_digit_2(s: &str) -> u32 {
    let c = s.chars().nth(0).unwrap();
    if c.is_numeric() {
        return c.to_digit(10).unwrap();
    }

    if let Some(num) = parse_string_number(s) {
        return num;
    }

    return parse_digit_2(&s[1..]);
}

fn parse_calibration_value_2(s: &str) -> u32 {
    let rev = s.chars().rev().collect::<String>();
    let first_digit = parse_digit_2(s);
    let last_digit = parse_digit_2(rev.as_str());
    first_digit * 10 + last_digit
}

fn parse_string_number(s: &str) -> Option<u32> {
    if s.starts_with("one") || s.starts_with("eno") {
        Some(1)
    } else if s.starts_with("two") || s.starts_with("owt") {
        Some(2)
    } else if s.starts_with("three") || s.starts_with("eerht") {
        Some(3)
    } else if s.starts_with("four") || s.starts_with("ruof") {
        Some(4)
    } else if s.starts_with("five") || s.starts_with("evif") {
        Some(5)
    } else if s.starts_with("six") || s.starts_with("xis") {
        Some(6)
    } else if s.starts_with("seven") || s.starts_with("neves") {
        Some(7)
    } else if s.starts_with("eight") || s.starts_with("thgie") {
        Some(8)
    } else if s.starts_with("nine") || s.starts_with("enin") {
        Some(9)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_calibration_value_2, parse_digit_2, parse_string_number};

    #[test]
    fn numbers() {
        assert_eq!(Some(2), parse_string_number("two1nine"));
        assert_eq!(Some(3), parse_string_number("eerhtowt"));
    }

    #[test]
    fn first_digit_2() {
        assert_eq!(1, parse_digit_2("zz1"));
        assert_eq!(2, parse_digit_2("abctwo4"));
    }

    #[test]
    fn parser_part_2() {
        assert_eq!(parse_calibration_value_2("two1nine"), 29);
        assert_eq!(parse_calibration_value_2("eightwothree"), 83);
        assert_eq!(parse_calibration_value_2("abcone2threexyz"), 13);
        assert_eq!(parse_calibration_value_2("xtwone3four"), 24);
        assert_eq!(parse_calibration_value_2("4nineeightseven2"), 42);
        assert_eq!(parse_calibration_value_2("zoneight234"), 14);
        assert_eq!(parse_calibration_value_2("7pqrstsixteen"), 76);
    }
}
