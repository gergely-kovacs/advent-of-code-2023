fn map_digit_strings_into_digits(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

fn sum_scratchcard_points(input: &str) -> usize {
    // split input by lines
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        // filtermap
        .map(|line| {
            // split lines by :
            let card_values = line.split(':').collect::<Vec<&str>>();
            // split result by |
            // [0] is winning numbers
            // [1] is played numbers
            let [winning_numbers_as_string, played_numbers_as_string] =
                match card_values[1].split('|').collect::<Vec<&str>>()[..] {
                    [winning_numbers_as_string, played_numbers_as_string] => {
                        [winning_numbers_as_string, played_numbers_as_string]
                    }
                    _ => panic!("Could not split card values by |"),
                };
            // split both by space
            let winning_numbers = map_digit_strings_into_digits(winning_numbers_as_string);
            let played_numbers = map_digit_strings_into_digits(played_numbers_as_string);
            let matching_number_count = played_numbers
                .iter()
                .filter(|x| winning_numbers.contains(x))
                .count();
            // return 2**(matches - 1)
            match matching_number_count {
                0 => 0,
                _ => 2usize.pow(matching_number_count.saturating_sub(1).try_into().unwrap()),
            }
        })
        // sum
        .sum()
}

fn main() {
    println!("{}", sum_scratchcard_points(include_str!("input.txt")));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let example_input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(super::sum_scratchcard_points(example_input), 13);
    }
}
