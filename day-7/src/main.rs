use itertools::Itertools;
use std::{cmp::Ordering, collections::HashMap};

#[repr(u8)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard = 10,
    OnePair = 20,
    TwoPair = 30,
    ThreeOfAKind = 40,
    FullHouse = 50,
    FourOfAKind = 60,
    FiveOfAKind = 70,
}

struct Hand {
    hand_type: HandType,
    cards: [u8; 5],
    bet: u32,
}

fn map_card_label_to_integer(input: char) -> u8 {
    match input {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Invalid card label"),
    }
}

fn determine_hand_type(cards: [u8; 5]) -> HandType {
    let mut card_counts: HashMap<u8, usize> = HashMap::new();
    for card in cards {
        *card_counts.entry(card).or_default() += 1;
    }
    match card_counts.len() {
        1 => HandType::FiveOfAKind,
        2 => {
            if card_counts.values().any(|&count| count == 4) {
                return HandType::FourOfAKind;
            }
            HandType::FullHouse
        }
        3 => {
            if card_counts.values().any(|&count| count == 3) {
                return HandType::ThreeOfAKind;
            }
            HandType::TwoPair
        }
        4 => HandType::OnePair,
        5 => HandType::HighCard,
        _ => panic!("Invalid card count"),
    }
}

fn parse_hand(input: &str) -> Hand {
    let mut parts = input.split_whitespace();
    let cards = parts
        .next()
        .unwrap()
        .chars()
        .map(map_card_label_to_integer)
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap();
    let bet = parts.last().unwrap().parse::<u32>().unwrap();
    Hand {
        hand_type: determine_hand_type(cards),
        cards,
        bet,
    }
}

fn sort_hands(a: &Hand, b: &Hand) -> Ordering {
    if a.hand_type != b.hand_type {
        if a.hand_type < b.hand_type {
            return Ordering::Less;
        }
        return Ordering::Greater;
    }
    for i in 0..5 {
        if a.cards[i] == b.cards[i] {
            continue;
        }
        if a.cards[i] < b.cards[i] {
            return Ordering::Less;
        }
        return Ordering::Greater;
    }
    Ordering::Equal
}

fn calculate_total_winnings(input: &str) -> u32 {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(parse_hand)
        .sorted_by(sort_hands)
        .enumerate()
        .map(|(i, hand)| hand.bet * (i + 1) as u32)
        .sum()
}

fn main() {
    println!(
        "Total winnings: {}",
        calculate_total_winnings(include_str!("input.txt"))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(calculate_total_winnings(input), 6440);
    }
}
