use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Hand(Vec<u64>);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Category {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

pub fn parse_line(line: &str) -> (Hand, u64) {
    let (raw_cards, raw_score) = line.split_ascii_whitespace().collect_tuple().unwrap();
    let cards = raw_cards
        .chars()
        .map(|c| card_value(c))
        .sorted()
        .collect_vec();
    let score = raw_score.parse::<u64>().unwrap();
    (Hand(cards), score)
}

fn card_value(card: char) -> u64 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => card.to_digit(10).unwrap() as u64,
    }
}

impl Hand {
    pub fn category(&self) -> Category {
        let mut counts = vec![0; 15];
        for card in &self.0 {
            counts[*card as usize] += 1;
        }
        let mut counts = counts;
        counts.sort();
        counts.reverse();
        let mut category = Category::HighCard;
        if counts[0] == 5 {
            category = Category::FiveOfAKind;
        } else if counts[0] == 4 {
            category = Category::FourOfAKind;
        } else if counts[0] == 3 && counts[1] == 2 {
            category = Category::FullHouse;
        } else if counts[0] == 3 {
            category = Category::ThreeOfAKind;
        } else if counts[0] == 2 && counts[1] == 2 {
            category = Category::TwoPair;
        } else if counts[0] == 2 {
            category = Category::OnePair;
        }
        return category;
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let category = self.category();
        let other_category = other.category();
        if category == other_category {
            for i in (0..5).rev() {
                let first = self.0[i];
                let second = other.0[i];
                if first != second {
                    return second.partial_cmp(&first);
                }
            }
        }
        return category.partial_cmp(&other_category);
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

pub fn solve_part_one(input: &str) -> u32 {
    let mut hands = input.lines().map(|line| parse_line(line)).collect_vec();
    hands.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let mut score = 0;
    for (i, hand) in hands.iter().rev().enumerate() {
        println!("{}: {:?}", i + 1, hand);
        score += (i + 1) as u32 * hand.1 as u32;
    }
    return score;
}

pub fn solve_part_two(input: &str) -> u32 {
    42
}

fn parse_input(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect()
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
        assert_eq!(super::solve_part_one(&input), 42);
    }

    #[test]
    fn solves_7_2_easy() {
        let input = std::fs::read_to_string("input/7_easy.txt").unwrap();
        assert_eq!(super::solve_part_two(&input), 42);
    }

    #[test]
    fn solves_7_2_hard() {
        let input = std::fs::read_to_string("input/7_real.txt").unwrap();
        assert_eq!(super::solve_part_two(&input), 42);
    }
}
