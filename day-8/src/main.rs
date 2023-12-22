use std::collections::HashMap;

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
    let instructions = *input.split("\n\n").collect::<Vec<&str>>().first().unwrap();
    let mut instructions_iter = instructions.chars().peekable();
    let network = *input.split("\n\n").collect::<Vec<&str>>().last().unwrap();
    let nodes = parse_map(network);
    let mut steps_taken = 0;
    while current_node != destination_node {
        if instructions_iter.peek().is_none() {
            instructions_iter = instructions.chars().peekable();
        }
        let instruction = instructions_iter.next().unwrap();
        current_node = get_next_node(current_node, instruction, &nodes);
        steps_taken += 1;
    }
    steps_taken
}

fn main() {
    println!(
        "{:?}",
        calculate_number_of_steps_to_reach_destination(include_str!("input.txt"))
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
}
