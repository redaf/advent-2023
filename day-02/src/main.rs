use std::fs;

fn main() {
    let input_txt = fs::read_to_string("input.txt").expect("Could not read input text file");

    println!("--- Day 2: Cube Conundrum ---");

    let sum_1 = possible_games_ids_sum(&input_txt);
    println!("Sum of the IDs of the possible games: {sum_1}\n");

    println!("--- Part Two ---");
    let sum_2 = sets_power_sum(&input_txt);
    println!("sum of the power of the sets: {sum_2}");
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

#[derive(Debug, PartialEq)]
enum ColorCubes {
    Red(usize),
    Green(usize),
    Blue(usize),
}

impl From<&str> for ColorCubes {
    fn from(s: &str) -> Self {
        let mut split = s.split_whitespace();
        let count: usize = split.next().unwrap().parse().unwrap();
        let color = split.next().unwrap();
        match color {
            "red" => ColorCubes::Red(count),
            "green" => ColorCubes::Green(count),
            "blue" => ColorCubes::Blue(count),
            _ => ColorCubes::Red(0),
        }
    }
}

#[derive(Default, PartialEq, Debug, Clone)]
struct CubesCount {
    red: usize,
    green: usize,
    blue: usize,
}

impl CubesCount {
    fn new(red: usize, green: usize, blue: usize) -> CubesCount {
        CubesCount { red, green, blue }
    }

    fn accumulate(&self, cc: ColorCubes) -> CubesCount {
        use ColorCubes::*;
        let mut acc = self.clone();
        match cc {
            Red(count) => acc.red += count,
            Green(count) => acc.green += count,
            Blue(count) => acc.blue += count,
        }
        acc
    }

    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

fn cubes_of_each_color_count(set: &str) -> CubesCount {
    set.split(',')
        .fold(CubesCount::default(), |acc, color_str| {
            acc.accumulate(color_str.into())
        })
}

fn fewest_number_of_cubes_of_each_color(sets: &str) -> CubesCount {
    sets.split(';')
        .map(cubes_of_each_color_count)
        .reduce(|acc, curr| {
            CubesCount::new(
                acc.red.max(curr.red),
                acc.green.max(curr.green),
                acc.blue.max(curr.blue),
            )
        })
        .unwrap()
}

fn game_power(game: &str) -> usize {
    let sets = game.split(':').nth(1).unwrap();
    fewest_number_of_cubes_of_each_color(sets).power()
}

fn sets_power_sum(input: &str) -> usize {
    input.lines().fold(0, |acc, game| acc + game_power(game))
}

#[cfg(test)]
mod tests {
    use crate::{
        are_sets_possible, cubes_of_each_color_count, fewest_number_of_cubes_of_each_color,
        game_power, is_color_cubes_count_ok, is_set_possible, possible_games_ids_sum,
        sets_power_sum, ColorCubes, CubesCount,
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

    #[test]
    fn colorcubes_fromstr_test() {
        assert_eq!(ColorCubes::from("3 blue"), ColorCubes::Blue(3));
        assert_eq!(ColorCubes::from("4 red"), ColorCubes::Red(4));
        assert_eq!(ColorCubes::from("13 green"), ColorCubes::Green(13));
    }

    #[test]
    fn cubes_of_each_color_count_test() {
        assert_eq!(
            CubesCount::new(20, 8, 6),
            cubes_of_each_color_count("8 green, 6 blue, 20 red")
        );
        assert_eq!(
            CubesCount::new(4, 0, 3),
            cubes_of_each_color_count("3 blue, 4 red")
        );
    }

    #[test]
    fn fewest_number_of_cubes_of_each_color_test() {
        assert_eq!(
            fewest_number_of_cubes_of_each_color("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            CubesCount::new(4, 2, 6)
        );
        assert_eq!(
            fewest_number_of_cubes_of_each_color(
                "8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
            ),
            CubesCount::new(20, 13, 6)
        );
    }

    #[test]
    fn game_power_test() {
        assert_eq!(
            game_power("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            48
        );
        assert_eq!(
            game_power("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"),
            1560
        );
    }

    #[test]
    fn sets_power_sum_test() {
        let games = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(sets_power_sum(games), 2286);
    }
}
