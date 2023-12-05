use regex::Regex;
use std::collections::HashMap;

const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn read_file(path: &str) -> String {
    let contents = std::fs::read_to_string(path).expect("Something went wrong reading the file");
    return contents;
}

fn sum_first_and_last_digit(line: &str) -> u32 {
    let digits = line
        .chars()
        .filter(|c| c.to_digit(10).is_some())
        .collect::<Vec<char>>();
    match digits.len() {
        0 => 0,
        1 => format!("{}{}", digits[0], digits[0]).parse().unwrap(),
        _ => format!("{}{}", digits[0], digits[digits.len() - 1])
            .parse()
            .unwrap(),
    }
}

fn does_contain_number_as_text(substring_of_line: &str) -> bool {
    for number in NUMBERS {
        if substring_of_line.contains(number) {
            return true;
        }
    }
    return false;
}

fn replace_numbers_as_text_in_substring_of_line(text: &str) -> String {
    let numbers_as_text_regex: regex::Regex =
        Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let number_mapping: HashMap<&str, &str> = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let replaced_text = numbers_as_text_regex.replace_all(text, |caps: &regex::Captures| {
        number_mapping
            .get(&caps[0])
            .map(|value| *value)
            .unwrap_or_default()
    });

    return replaced_text.to_string();
}

fn convert_number_as_text_to_number_as_string(line: &str) -> String {
    if !does_contain_number_as_text(line) {
        return line.to_string();
    }

    let mut line_with_parsed_numbers = line.to_string();

    for cursor_position in 3..line_with_parsed_numbers.len() + 1 {
        let substring_of_line = &line_with_parsed_numbers[..cursor_position];
        if does_contain_number_as_text(substring_of_line) {
            line_with_parsed_numbers = line_with_parsed_numbers.replace(
                &substring_of_line,
                replace_numbers_as_text_in_substring_of_line(&substring_of_line).as_str(),
            );
            return convert_number_as_text_to_number_as_string(&line_with_parsed_numbers);
        }
    }

    return line_with_parsed_numbers;
}

fn sum_all_the_lines(input_data: &str) -> u32 {
    let mut sum = 0;
    for line in input_data.lines().filter(|x| !x.is_empty()) {
        let line_with_parsed_numbers = convert_number_as_text_to_number_as_string(line);
        sum += sum_first_and_last_digit(&line_with_parsed_numbers);
    }
    return sum;
}

fn main() {
    println!("{}", sum_all_the_lines(read_file("src/input.txt").as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";

        assert_eq!(sum_all_the_lines(input), 281);
    }
}
