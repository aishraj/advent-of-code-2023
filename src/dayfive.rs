use itertools::Itertools;
/// Algorithm:
/// 1. Parse input into state and blocks
/// 2. for each block
///     1. check if the item in the state is in the source range
///     2. if it is, replace it with the destination range
///     3.  if it isnot then use the item in the state
///     4. this is the new state
/// return the lowest value in the state
pub fn solve_part_one(input: &str) -> u64 {
    let (mut state, blocks) = parse_input(input);
    for block in blocks {
        state = state
            .iter()
            .map(|item| {
                let mut item = *item;
                for (source_range, destination_range) in block.iter() {
                    if source_range.contains(&item) {
                        item = destination_range.start + (item - source_range.start);
                        break;
                    }
                }
                item
            })
            .collect_vec();
    }
    state.iter().min().unwrap().clone()
}

pub fn solve_part_two(input: &str) -> u64 {
    42
}

fn parse_block(input: Vec<&str>) -> Vec<(std::ops::Range<u64>, std::ops::Range<u64>)> {
    // Each block looks like the following:
    //seed-to-soil map:
    // 50 98 2
    // 52 50 48
    //println!("Parsing block: {:?}", input[0]);
    let input = input[1..].to_vec();
    let mut ranges = Vec::new();
    for line in input {
        let line = line.trim().split_whitespace().collect_vec();
        //println!("Parsing line: {:?}", line);
        let destination_start = line[0].parse::<u64>().unwrap();
        let source_start = line[1].parse::<u64>().unwrap();
        let range_capacity = line[2].parse::<u64>().unwrap();
        let source_range = source_start..(source_start + range_capacity);
        let destination_range = destination_start..(destination_start + range_capacity);
        ranges.push((source_range, destination_range));
    }
    //println!("Parsed block: {:?}", ranges);
    ranges
}

fn parse_input(
    input: &str,
) -> (
    Vec<u64>,
    Vec<Vec<(std::ops::Range<u64>, std::ops::Range<u64>)>>,
) {
    let groups = input.split("\n\n").collect::<Vec<_>>();
    //seeds: 79 14 55 13
    let initial_state = groups[0].split(":").collect::<Vec<_>>()[1];
    let initial_state = initial_state
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let mut blocks = Vec::new();
    for block in groups[1..].to_vec() {
        blocks.push(parse_block(block.split("\n").collect::<Vec<_>>()));
    }
    (initial_state, blocks)
}

#[cfg(test)]
mod tests {

    #[test]
    fn parses_a_block() {
        let raw_block = "fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4";
        let parsed_block = super::parse_block(raw_block.split("\n").collect::<Vec<_>>());
        assert_eq!(
            parsed_block,
            vec![
                (53..61, 49..57),
                (11..53, 0..42),
                (0..7, 42..49),
                (7..11, 57..61)
            ]
        );
    }

    #[test]
    fn parses_b_block() {
        let raw_block = "seed-to-soil map:
50 98 2
52 50 48";
        let parsed_block = super::parse_block(raw_block.split("\n").collect::<Vec<_>>());
        assert_eq!(parsed_block, vec![(98..100, 50..52), (50..98, 52..100),]);
    }

    #[test]
    fn solves_5_1_easy() {
        let input = std::fs::read_to_string("input/5_easy.txt").unwrap();
        assert_eq!(super::solve_part_one(&input), 35);
    }

    #[test]
    fn solves_5_1_hard() {
        let input = std::fs::read_to_string("input/5_real.txt").unwrap();
        assert_eq!(super::solve_part_one(&input), 3374647);
    }

    #[test]
    fn solves_5_2_easy() {
        let input = std::fs::read_to_string("input/5_easy.txt").unwrap();
        assert_eq!(super::solve_part_one(&input), 42);
    }

    #[test]
    fn solves_5_2_hard() {
        let input = std::fs::read_to_string("input/5_real.txt").unwrap();
        assert_eq!(super::solve_part_one(&input), 42);
    }
}
