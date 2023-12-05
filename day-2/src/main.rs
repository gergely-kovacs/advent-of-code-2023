use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RED_CUBES: Regex = Regex::new(r"(\d+) red").unwrap();
    static ref GREEN_CUBES: Regex = Regex::new(r"(\d+) green").unwrap();
    static ref BLUE_CUBES: Regex = Regex::new(r"(\d+) blue").unwrap();
}

const RED_CUBE_LIMIT: u8 = 12;
const GREEN_CUBE_LIMIT: u8 = 13;
const BLUE_CUBE_LIMIT: u8 = 14;

fn read_input_file() -> String {
    let contents = std::fs::read_to_string("src/input.txt").expect("Failed to read input file");
    return contents;
}

fn filter_possible_games(
    games: &str,
    red_cube_count: u8,
    green_cube_count: u8,
    blue_cube_count: u8,
) -> Vec<u16> {
    return games
        .lines()
        .filter_map(|line| {
            if line.trim().is_empty() {
                return None;
            }

            let game_id_and_rounds: Vec<&str> = line.split(": ").collect();
            let game_id = game_id_and_rounds
                .first()
                .unwrap()
                .split(" ")
                .last()
                .unwrap();
            let rounds: Vec<&str> = game_id_and_rounds.last().unwrap().split("; ").collect();
            if rounds.iter().any(|round| {
                if RED_CUBES
                    .captures_iter(round)
                    .any(|c| c[1].parse::<u8>().unwrap() > red_cube_count)
                    || GREEN_CUBES
                        .captures_iter(round)
                        .any(|c| c[1].parse::<u8>().unwrap() > green_cube_count)
                    || BLUE_CUBES
                        .captures_iter(round)
                        .any(|c| c[1].parse::<u8>().unwrap() > blue_cube_count)
                {
                    return true;
                }
                return false;
            }) {
                return None;
            }
            return Some(game_id.parse::<u16>().unwrap());
        })
        .collect();
}

fn sum_id_of_possible_games(game_ids: Vec<u16>) -> u16 {
    return game_ids.iter().sum();
}

fn calculate_power_of_game(game: &str) -> u16 {
    let max_number_of_red_cubes = RED_CUBES
        .captures_iter(game)
        .map(|c| c[1].parse::<u8>().unwrap())
        .max()
        .unwrap_or(0u8);
    let max_number_of_green_cubes = GREEN_CUBES
        .captures_iter(game)
        .map(|c| c[1].parse::<u8>().unwrap())
        .max()
        .unwrap_or(0u8);
    let max_number_of_blue_cubes = BLUE_CUBES
        .captures_iter(game)
        .map(|c| c[1].parse::<u8>().unwrap())
        .max()
        .unwrap_or(0u8);
    return max_number_of_red_cubes as u16
        * max_number_of_green_cubes as u16
        * max_number_of_blue_cubes as u16;
}

fn sum_power_of_games(games: &str) -> u32 {
    let power_of_each_game = games
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| calculate_power_of_game(line))
        .collect::<Vec<_>>();
    return power_of_each_game.iter().map(|x| *x as u32).sum();
}

fn main() {
    // println!(
    //     "{}",
    //     sum_id_of_possible_games(filter_possible_games(
    //         &read_input_file(),
    //         RED_CUBE_LIMIT,
    //         GREEN_CUBE_LIMIT,
    //         BLUE_CUBE_LIMIT
    //     ))
    // );
    println!("{}", sum_power_of_games(&read_input_file()));
}

#[test]
fn test_part_1() {
    let example = "
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
    assert_eq!(
        sum_id_of_possible_games(filter_possible_games(
            example,
            RED_CUBE_LIMIT,
            GREEN_CUBE_LIMIT,
            BLUE_CUBE_LIMIT
        )),
        8
    );
}

#[test]
fn test_part_2() {
    let example = "
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
    assert_eq!(sum_power_of_games(example), 2286);
}
