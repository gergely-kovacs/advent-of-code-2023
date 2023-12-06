fn sum_part_numbers(input: &str) -> u32 {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .fold(Vec::new(), |mut acc: Vec<&str>, line| {
            return acc;
        });
    return 0;
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "\
467..114..\
...*......\
..35..633.\
......#...\
617*......\
.....+.58.\
..592.....\
......755.\
...$.*....\
.664.598..";
        assert_eq!(sum_part_numbers(input), 4361);
    }
}
