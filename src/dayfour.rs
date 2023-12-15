use std::collections::{BTreeMap, BTreeSet, HashSet};

use itertools::Itertools;

pub fn solve_part_one(input: &str) -> u32 {
    let parsed_input = parse_input(input);
    // the input has two parts, first the lottery winning numbers and second the numbers we have
    // we need to count the numbers that we have that are in the winning numbers
    let overlapping_number_counts = parsed_input
        .iter()
        .map(|numbers| {
            let winning_numbers: HashSet<u32> = HashSet::from_iter(numbers[0].iter().cloned());
            let my_numbers: HashSet<u32> = HashSet::from_iter(numbers[1].iter().cloned());
            let overlapping_numbers = winning_numbers.intersection(&my_numbers).collect_vec();
            let count = overlapping_numbers.len();
            // println!(
            //     "winning: {:?} my: {:?} overlap: {:?}",
            //     winning_numbers, my_numbers, overlapping_numbers
            // );
            // return 2 ** count
            if count == 0 {
                return 0;
            }
            u32::pow(2, (count - 1) as u32)
        })
        .collect_vec();
    // println!(
    //     "overlapping number count is {:?}",
    //     overlapping_number_counts
    // );
    overlapping_number_counts.iter().sum()
}

pub fn solve_part_two(input: &str) -> u32 {
    let parsed_input = parse_input(input);
    let mut card_counts: BTreeMap<usize, u32> = BTreeMap::new();
    // we say that we have one of each card
    let num_cards = parsed_input.len();
    for i in 0..num_cards {
        card_counts.insert(i, 1);
    }
    for (group_index, cardgroups) in parsed_input.iter().enumerate() {
        let my_numbers: BTreeSet<u32> = BTreeSet::from_iter(cardgroups[1].iter().cloned());
        let winning_numbers: BTreeSet<u32> = BTreeSet::from_iter(cardgroups[0].iter().cloned());
        let overlapping_numbers = winning_numbers.intersection(&my_numbers).collect_vec();
        let overlap_count = overlapping_numbers.len();
        // this means we have that many cards of the next n overlapping numbers
        //println!("current group: {:?}", group_index);
        let repeating_factor = card_counts.get(&group_index).unwrap();
        for _i in 0..*repeating_factor {
            for j in (group_index + 1)..=(group_index + overlap_count) {
                //println!("adding card to group: {}", j);
                let card_count = card_counts.get_mut(&j).unwrap();
                //let original_count = *card_count;
                // println!(
                //     "j: {} increasing card count from {} to {}",
                //     j,
                //     original_count,
                //     *card_count + 1
                // );
                *card_count += 1
            }
        }

        // println!(
        //     "overlap: {}, group: {}, card_counts: {:?}",
        //     overlap_count, group_index, card_counts
        // );
    }
    println!("final card counts: {:?}", card_counts);
    card_counts.values().sum()
}

fn parse_input(input: &str) -> Vec<Vec<Vec<u32>>> {
    input
        .lines()
        .map(|line| {
            // each line looks like Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            let (_id, cards) = line.split(':').collect_tuple().unwrap();
            let numbers = cards.split('|').collect_vec();
            numbers
                .iter()
                .map(|card| card.split_whitespace().collect_vec())
                .map(|num| num.iter().map(|n| n.parse::<u32>().unwrap()).collect_vec())
                .collect_vec()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn solves_4_1_easy() {
        let input = std::fs::read_to_string("input/4_easy.txt").unwrap();
        assert_eq!(super::solve_part_one(&input), 13);
    }

    #[test]
    fn solves_4_1_hard() {
        let input = std::fs::read_to_string("input/4_real.txt").unwrap();
        assert_eq!(super::solve_part_one(&input), 25571);
    }

    #[test]
    fn solves_4_2_easy() {
        let input = std::fs::read_to_string("input/4_easy.txt").unwrap();
        assert_eq!(super::solve_part_two(&input), 30);
    }

    #[test]
    fn solves_4_2_hard() {
        let input = std::fs::read_to_string("input/4_real.txt").unwrap();
        assert_eq!(super::solve_part_two(&input), 8805731);
    }
}
