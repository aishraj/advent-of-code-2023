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
    let (raw_state, blocks) = parse_input(input);
    let mut state = raw_state
        .iter()
        .step_by(2)
        .zip(raw_state[1..].iter().step_by(2))
        .map(|(a, b)| {
            let range_end = a + b;
            *a..range_end
        })
        .collect_vec();
    println!("State: {:?}", state);
    for block in blocks {
        state = state
            .iter()
            .flat_map(|item| {
                let mut mapped_ranges = Vec::new();
                let mut unmapped_ranges = vec![item.clone()];
                for (source_range, destination_range) in block.iter() {
                    // Temporary vector to store the ranges that are not mapped in the current iteration
                    let mut temp_unmapped = Vec::new();
                    // Iterate over each unmapped range
                    for item in unmapped_ranges {
                        // Calculate the range before the source range
                        let before_source = item.start..item.end.min(source_range.start);
                        // Calculate the range within the source range
                        let within_source =
                            item.start.max(source_range.start)..(source_range.end).min(item.end);
                        // Calculate the range after the source range
                        let after_source = (source_range.end).max(item.start)..item.end;
                        // If the end of the before_source range is greater than the start, add it to the temp_unmapped vector
                        if before_source.end > before_source.start {
                            temp_unmapped.push(before_source);
                        }
                        // If the end of the within_source range is greater than the start, calculate the corresponding destination range and add it to the mapped_ranges vector
                        if within_source.end > within_source.start {
                            mapped_ranges.push(
                                within_source.start - source_range.start + destination_range.start
                                    ..within_source.end - source_range.start
                                        + destination_range.start,
                            );
                        }
                        // If the end of the after_source range is greater than the start, add it to the temp_unmapped vector
                        if after_source.end > after_source.start {
                            temp_unmapped.push(after_source);
                        }
                    }
                    // Replace the unmapped_ranges vector with the temp_unmapped vector for the next iteration
                    unmapped_ranges = temp_unmapped;
                }
                // Extend the mapped_ranges vector with the remaining unmapped ranges
                mapped_ranges.extend(unmapped_ranges);
                // Return the mapped_ranges vector
                mapped_ranges
            })
            .collect_vec();
        println!("State: {:?}", state);
    }
    println!("Final State: {:?}", state);
    state.sort_by(|a, b| a.start.cmp(&b.start));
    state[0].start
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
        assert_eq!(super::solve_part_two(&input), 46);
    }

    #[test]
    fn solves_5_2_hard() {
        let input = std::fs::read_to_string("input/5_real.txt").unwrap();
        assert_eq!(super::solve_part_two(&input), 6082852);
    }
}
