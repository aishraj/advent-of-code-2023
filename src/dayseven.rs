use core::fmt::Display;
use itertools::Itertools;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Hand(Vec<u64>);

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cards = self.0.clone();
        let cards: Vec<String> = cards.iter().map(|c| card_string(*c)).collect();
        write!(f, "{}", cards.join(""))
    }
}

fn card_string(card: u64) -> String {
    match card {
        14 => "A".to_string(),
        13 => "K".to_string(),
        12 => "Q".to_string(),
        11 => "J".to_string(),
        10 => "T".to_string(),
        1 => "J".to_string(),
        _ => card.to_string(),
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Category {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

pub fn parse_line(line: &str, part_two: bool) -> (Hand, u64) {
    let (raw_cards, raw_score) = line.split_ascii_whitespace().collect_tuple().unwrap();
    let cards = raw_cards
        .chars()
        .map(|c| card_value(c, part_two))
        .collect_vec();
    let score = raw_score.parse::<u64>().unwrap();
    (Hand(cards), score)
}

fn card_value(card: char, part_two: bool) -> u64 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => {
            if part_two {
                1
            } else {
                11
            }
        }
        'T' => 10,
        _ => card.to_digit(10).unwrap() as u64,
    }
}

impl Hand {
    pub fn category(&self, part_two: bool) -> Category {
        let (counts, num_jokers) =
            self.0
                .iter()
                .fold((BTreeMap::new(), 0), |(mut counts, num_jokers), card| {
                    if part_two && *card == 1 {
                        (counts, num_jokers + 1)
                    } else {
                        *counts.entry(*card).or_insert(0) += 1;
                        (counts, num_jokers)
                    }
                });

        let mut freqs = counts.values().copied().collect_vec();
        freqs.sort();
        if let Some(last_item) = freqs.last_mut() {
            *last_item += num_jokers;
        } else {
            freqs.push(num_jokers);
        }

        match *freqs {
            [1, 1, 1, 1, 1] => Category::HighCard,
            [1, 1, 1, 2] => Category::OnePair,
            [1, 2, 2] => Category::TwoPair,
            [1, 1, 3] => Category::ThreeOfAKind,
            [2, 3] => Category::FullHouse,
            [1, 4] => Category::FourOfAKind,
            [5] => Category::FiveOfAKind,
            _ => panic!("Invalid hand: {:?}", self),
        }
    }
}

pub fn solve(input: &str, part_two: bool) -> u32 {
    let mut hands = input
        .lines()
        .map(|line| parse_line(line, part_two))
        .collect_vec();
    hands.sort_by(|a, b| {
        let category = a.0.category(part_two);
        let other_category = b.0.category(part_two);
        let (cat, num) = (category, a.0 .0.clone());
        let (other_cat, other_num) = (other_category, b.0 .0.clone());
        (cat, num).partial_cmp(&(other_cat, other_num)).unwrap()
    });
    let mut score = 0;
    for (i, hand) in hands.iter().enumerate() {
        println!("{}: {}:{}", i + 1, hand.0, hand.1);
        score += (i + 1) as u32 * hand.1 as u32;
    }
    score
}

pub fn solve_part_one(input: &str) -> u32 {
    solve(input, false)
}

pub fn solve_part_two(input: &str) -> u32 {
    solve(input, true)
}

#[cfg(test)]
mod tests {
    #[test]
    fn solves_7_1_easy() {
        let input = std::fs::read_to_string("input/7_easy.txt").unwrap();
        assert_eq!(super::solve_part_one(&input), 6440);
    }

    #[test]
    fn solves_7_1_hard() {
        let input = std::fs::read_to_string("input/7_real.txt").unwrap();
        assert_eq!(super::solve_part_one(&input), 251136060);
    }

    #[test]
    fn solves_7_2_easy() {
        let input = std::fs::read_to_string("input/7_easy.txt").unwrap();
        assert_eq!(super::solve_part_two(&input), 5905);
    }

    #[test]
    fn solves_7_2_hard() {
        let input = std::fs::read_to_string("input/7_real.txt").unwrap();
        assert_eq!(super::solve_part_two(&input), 249400220);
    }
}
