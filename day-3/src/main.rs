#[derive(Debug)]
struct NumberInLine {
    number: usize,
    start_index: usize,
    end_index: usize,
}

impl Clone for NumberInLine {
    fn clone(&self) -> Self {
        Self {
            number: self.number,
            start_index: self.start_index,
            end_index: self.end_index,
        }
    }
}

fn read_input_file() -> String {
    std::fs::read_to_string("input.txt").unwrap()
}

fn get_index_of_symbol((i, c): (usize, char)) -> Option<usize> {
    if c != '.' && !c.is_ascii_alphanumeric() {
        Some(i)
    } else {
        None
    }
}

fn find_numbers_in_line(line: &str) -> Vec<NumberInLine> {
    line.trim().chars().enumerate().fold(
        Vec::new(),
        |mut numbers: Vec<NumberInLine>, (character_index, c)| {
            if !c.is_ascii_digit() {
                return numbers;
            }
            if numbers.is_empty() {
                numbers.push(NumberInLine {
                    number: c.to_digit(10).unwrap() as usize,
                    start_index: character_index,
                    end_index: character_index,
                })
            } else {
                let last_number = numbers.last_mut().unwrap();
                if last_number.end_index == character_index - 1 {
                    last_number.end_index = character_index;
                    last_number.number = line[last_number.start_index..=last_number.end_index]
                        .parse::<usize>()
                        .unwrap();
                    return numbers;
                }
                numbers.push(NumberInLine {
                    number: c.to_digit(10).unwrap() as usize,
                    start_index: character_index,
                    end_index: character_index,
                })
            }
            numbers
        },
    )
}

fn sum_part_numbers(input: &str) -> usize {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .enumerate()
        .filter_map(|(line_index, line)| {
            let numbers_in_line = find_numbers_in_line(line);

            if numbers_in_line.is_empty() {
                return None;
            }

            let symbol_indices_in_prev_line = input
                .lines()
                .nth(line_index.checked_sub(1).unwrap_or_default())
                .unwrap_or_default()
                .chars()
                .enumerate()
                .filter_map(get_index_of_symbol)
                .collect::<Vec<usize>>();

            let symbol_indices_in_current_line = line
                .chars()
                .enumerate()
                .filter_map(get_index_of_symbol)
                .collect::<Vec<usize>>();

            let symbol_indices_of_next_line = input
                .lines()
                .nth(line_index.checked_add(1).unwrap_or_default())
                .unwrap_or_default()
                .chars()
                .enumerate()
                .filter_map(get_index_of_symbol)
                .collect::<Vec<usize>>();

            let symbol_indices = [
                &symbol_indices_in_prev_line[..],
                &symbol_indices_in_current_line[..],
                &symbol_indices_of_next_line[..],
            ]
            .concat();

            if symbol_indices.is_empty() {
                return None;
            }

            let final_numbers_of_line: Vec<usize> = numbers_in_line
                .iter()
                .filter_map(|number_in_line| {
                    if symbol_indices.iter().any(|&symbol_index| {
                        let start_minus_one = number_in_line
                            .start_index
                            .checked_sub(1)
                            .unwrap_or(number_in_line.start_index);
                        let end_plus_one = number_in_line
                            .end_index
                            .checked_add(1)
                            .unwrap_or(number_in_line.end_index);
                        start_minus_one <= symbol_index && symbol_index <= end_plus_one
                    }) {
                        return Some(number_in_line.number);
                    }
                    None
                })
                .collect();

            Some(final_numbers_of_line.iter().sum::<usize>())
        })
        .sum()
}

fn sum_gear_ratios(input: &str) -> usize {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .enumerate()
        .filter_map(|(line_index, line)| {
            let asterisk_positions_in_current_line: Vec<usize> = line
                .chars()
                .enumerate()
                .filter_map(|(character_index, c)| {
                    if c == '*' {
                        Some(character_index)
                    } else {
                        None
                    }
                })
                .collect();

            if asterisk_positions_in_current_line.is_empty() {
                return None;
            }

            let numbers_in_previous_line = find_numbers_in_line(
                input
                    .lines()
                    .nth(line_index.checked_sub(1).unwrap_or_default())
                    .unwrap_or_default(),
            );
            let numbers_in_current_line = find_numbers_in_line(line);
            let numbers_in_next_line = find_numbers_in_line(
                input
                    .lines()
                    .nth(line_index.checked_add(1).unwrap_or_default())
                    .unwrap_or_default(),
            );
            let numbers = [
                &numbers_in_previous_line[..],
                &numbers_in_current_line[..],
                &numbers_in_next_line[..],
            ]
            .concat();

            if numbers.is_empty() {
                return None;
            }

            Some(asterisk_positions_in_current_line.iter().fold(
                0usize,
                |sum_of_gear_ratios_in_line, asterisk_index| {
                    let numbers_next_to_asterisk = numbers.iter().filter_map(|number| {
                        if number.start_index.checked_sub(1).unwrap_or_default() <= *asterisk_index
                            && *asterisk_index
                                <= number.end_index.checked_add(1).unwrap_or_default()
                        {
                            Some(number.number)
                        } else {
                            None
                        }
                    });

                    if numbers_next_to_asterisk.clone().count() != 2usize {
                        return sum_of_gear_ratios_in_line;
                    }

                    sum_of_gear_ratios_in_line + numbers_next_to_asterisk.clone().product::<usize>()
                },
            ))
        })
        .sum()
}

fn main() {
    println!("Part 1: {}", sum_part_numbers(&read_input_file()));
    println!("Part 2: {}", sum_gear_ratios(&read_input_file()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(sum_part_numbers(input), 4361);
    }

    #[test]
    fn test_part_2() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(sum_gear_ratios(input), 467835);
    }
}
