use std::fs;

fn main() {
    let input_txt = fs::read_to_string("input.txt").expect("Could not read input text file");

    println!("--- Day 2: Cube Conundrum ---\n");

    println!("Part 1");
    let sum = possible_games_ids_sum(&input_txt);
    println!("Sum of the IDs of the possible games: {sum}");
}

fn possible_games_ids_sum(input: &str) -> usize {
    input.lines().enumerate().fold(0, |acc, (game_id, line)| {
        let mut split = line.split(':');
        let sets = split.nth(1).unwrap();
        if are_sets_possible(sets) {
            acc + (game_id + 1)
        } else {
            acc
        }
    })
}

fn are_sets_possible(sets: &str) -> bool {
    sets.split(';').all(is_set_possible)
}

fn is_set_possible(set: &str) -> bool {
    let mut cubes = set.split(',');
    cubes.all(is_color_cubes_count_ok)
}

fn is_color_cubes_count_ok(cubes_count: &str) -> bool {
    let mut split = cubes_count.split_whitespace();
    let count: usize = split.next().unwrap().parse().unwrap();
    let color = split.next().unwrap();
    match color {
        "red" => count <= 12,
        "green" => count <= 13,
        "blue" => count <= 14,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        are_sets_possible, is_color_cubes_count_ok, is_set_possible, possible_games_ids_sum,
    };

    #[test]
    fn is_color_cubes_count_ok_test() {
        assert_eq!(is_color_cubes_count_ok("4 red"), true);
        assert_eq!(is_color_cubes_count_ok("5 green"), true);
        assert_eq!(is_color_cubes_count_ok("6 blue"), true);
        assert_eq!(is_color_cubes_count_ok("13 red"), false);
        assert_eq!(is_color_cubes_count_ok("14 green"), false);
        assert_eq!(is_color_cubes_count_ok("15 blue"), false);
    }

    #[test]
    fn is_set_possible_test() {
        assert_eq!(is_set_possible("3 blue, 4 red"), true);
        assert_eq!(is_set_possible("1 red, 2 green, 6 blue"), true);
        assert_eq!(is_set_possible("8 green, 6 blue, 20 red"), false);
        assert_eq!(is_set_possible("3 green, 15 blue, 14 red"), false);
    }

    #[test]
    fn are_sets_possible_test() {
        assert_eq!(
            are_sets_possible("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            true
        );
        assert_eq!(
            are_sets_possible("1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            true
        );
        assert_eq!(
            are_sets_possible("8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"),
            false
        );
        assert_eq!(
            are_sets_possible("1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"),
            false
        );
        assert_eq!(
            are_sets_possible("6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
            true
        );
    }

    #[test]
    fn possible_games_ids_sum_test() {
        let games = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(possible_games_ids_sum(games), 8);
    }
}
