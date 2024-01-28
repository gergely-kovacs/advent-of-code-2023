use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

fn parse_map(input: &str) -> HashMap<&str, Vec<String>> {
    input.lines().filter(|x| !x.is_empty()).fold(
        HashMap::new(),
        |mut acc: HashMap<&str, Vec<String>>, line| {
            let mut line_split_by_equal = line.split(" = ");
            let current_node = line_split_by_equal.next().unwrap();
            let connected_nodes = line_split_by_equal
                .next()
                .unwrap()
                .split(", ")
                .map(|x| x.replace(['(', ')'], ""))
                .collect::<Vec<String>>();
            acc.insert(current_node, connected_nodes);
            acc
        },
    )
}

fn get_next_node<'a>(
    current_node: &'a str,
    instruction: char,
    nodes: &'a HashMap<&'a str, Vec<String>>,
) -> &'a str {
    match instruction {
        'L' => nodes.get(current_node).unwrap().first().unwrap(),
        'R' => nodes.get(current_node).unwrap().last().unwrap(),
        _ => panic!("Unknown instruction"),
    }
}

fn calculate_number_of_steps_to_reach_destination(input: &str) -> usize {
    let starting_node = "AAA";
    let destination_node = "ZZZ";
    let mut current_node = starting_node;
    let mut instructions = input
        .split("\n\n")
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
        .chars()
        .cycle();
    let network = *input.split("\n\n").collect::<Vec<&str>>().last().unwrap();
    let nodes = parse_map(network);
    let mut steps_taken = 0;
    while current_node != destination_node {
        let instruction = instructions.next().unwrap();
        current_node = get_next_node(current_node, instruction, &nodes);
        steps_taken += 1;
    }
    steps_taken
}

fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn calculate_number_of_steps_to_reach_destination_parallel(input: &str) -> usize {
    let mut instructions = input
        .split("\n\n")
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
        .chars()
        .cycle();
    let network = *input.split("\n\n").collect::<Vec<&str>>().last().unwrap();
    let nodes = parse_map(network);
    let starting_nodes: HashSet<&str> =
        HashSet::from_iter(nodes.keys().cloned().filter(|x| x.ends_with('A')));
    let mut current_nodes = starting_nodes.clone();
    let mut steps_taken = 0;
    let mut reached_destinations_with_step_count: HashMap<&str, usize> = HashMap::new();
    while !current_nodes.is_empty() {
        let instruction = instructions.next().unwrap();
        current_nodes = current_nodes
            .iter()
            .map(|x| get_next_node(x, instruction, &nodes))
            .collect();
        steps_taken += 1;

        if current_nodes.iter().any(|x| x.ends_with('Z')) {
            reached_destinations_with_step_count.insert(
                current_nodes.iter().find(|x| x.ends_with('Z')).unwrap(),
                steps_taken,
            );

            current_nodes.retain(|x| !x.ends_with('Z'));
        }
    }

    lcm(&reached_destinations_with_step_count.values().cloned().collect::<Vec<usize>>())
}

fn main() {
    println!(
        "{:?}",
        calculate_number_of_steps_to_reach_destination(include_str!("input.txt"))
    );
    println!(
        "{:?}",
        calculate_number_of_steps_to_reach_destination_parallel(include_str!("input.txt"))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example_1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(calculate_number_of_steps_to_reach_destination(input), 2);
    }

    #[test]
    fn test_part_1_example_2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(calculate_number_of_steps_to_reach_destination(input), 6);
    }

    #[test]
    fn test_part_2() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(
            calculate_number_of_steps_to_reach_destination_parallel(input),
            6
        );
    }
}
