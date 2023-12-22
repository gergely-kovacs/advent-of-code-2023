use itertools::Itertools;
use std::{cmp::Ordering, collections::HashMap};

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard = 10,
    OnePair = 20,
    TwoPair = 30,
    ThreeOfAKind = 40,
    FullHouse = 50,
    FourOfAKind = 60,
    FiveOfAKind = 70,
}

#[derive(Debug)]
struct Hand {
    hand_type: HandType,
    cards: [u8; 5],
    bet: u32,
}

fn map_card_label_to_integer(input: char, is_joker_a_wildcard: bool) -> u8 {
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
        'J' => {
            if is_joker_a_wildcard {
                1
            } else {
                11
            }
        }
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Invalid card label"),
    }
}

fn get_hand_type_without_wildcard(card_counts: HashMap<u8, usize>) -> HandType {
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

fn get_hand_type_with_wildcard(card_counts: HashMap<u8, usize>) -> HandType {
    match card_counts.len() {
        1 => HandType::FiveOfAKind,
        2 => HandType::FiveOfAKind,
        3 => match card_counts[&1u8] {
            // 1 1 1 13 14 -> four of a kind
            3 => HandType::FourOfAKind,
            // 1 1 13 10 10 -> four of a kind
            2 => HandType::FourOfAKind,
            1 => {
                // 1 10 10 10 14 -> four of a kind
                if card_counts.values().any(|&count| count == 3) {
                    return HandType::FourOfAKind;
                }
                // 1 10 10 14 14 -> full house
                HandType::FullHouse
            }
            _ => panic!("Invalid joker count"),
        },
        4 => HandType::ThreeOfAKind,
        5 => HandType::OnePair,
        _ => panic!("Invalid card count"),
    }
}

fn determine_hand_type(cards: [u8; 5], is_joker_a_wildcard: bool) -> HandType {
    let mut card_counts: HashMap<u8, usize> = HashMap::new();
    for card in cards {
        *card_counts.entry(card).or_default() += 1;
    }
    if is_joker_a_wildcard && cards.contains(&1u8) {
        return get_hand_type_with_wildcard(card_counts);
    }
    get_hand_type_without_wildcard(card_counts)
}

fn parse_hand(input: &str, is_joker_a_wildcard: bool) -> Hand {
    let mut parts = input.split_whitespace();
    let cards = parts
        .next()
        .unwrap()
        .chars()
        .map(|card| map_card_label_to_integer(card, is_joker_a_wildcard))
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap();
    let bet = parts.last().unwrap().parse::<u32>().unwrap();
    Hand {
        hand_type: determine_hand_type(cards, is_joker_a_wildcard),
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

fn calculate_total_winnings(input: &str, is_joker_a_wildcard: bool) -> u32 {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| parse_hand(line, is_joker_a_wildcard))
        .sorted_by(sort_hands)
        .enumerate()
        .map(|(i, hand)| {
            // dbg!(&hand);
            hand.bet * (i + 1) as u32
        })
        .sum()
}

fn main() {
    println!(
        "Total winnings part 1: {}",
        calculate_total_winnings(include_str!("input.txt"), false)
    );
    println!(
        "Total winnings part 2: {}",
        calculate_total_winnings(include_str!("input.txt"), true)
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
        let is_joker_a_wildcard = false;
        assert_eq!(calculate_total_winnings(input, is_joker_a_wildcard), 6440);
    }

    #[test]
    fn test_part_2() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let is_joker_a_wildcard = true;
        assert_eq!(calculate_total_winnings(input, is_joker_a_wildcard), 5905);
    }
}
