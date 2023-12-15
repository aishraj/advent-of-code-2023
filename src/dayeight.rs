use std::collections::BTreeMap;

use itertools::Itertools;

pub fn solve_part_one(input: &str) -> u32 {
    let (seed, memory) = parse_input(input);
    simulate(&seed, memory).try_into().unwrap()
}

pub fn solve_part_two(input: &str) -> u64 {
    let (seed, memory) = parse_input(input);
    simulate_two(&seed, memory)
}

fn parse_input(input: &str) -> (String, BTreeMap<String, (String, String)>) {
    let (seed, lines) = input.split("\n\n").collect_tuple().unwrap();
    let mut mapping = BTreeMap::new();
    for line in lines.lines() {
        let (key, value) = line.split(" = ").collect_tuple().unwrap();
        let key = key.trim();
        let value_parts: (&str, &str) = value.split(',').collect_tuple().unwrap();
        let left = value_parts.0.trim()[1..].to_string();
        let right_raw = value_parts.1.trim();
        let right = right_raw[..right_raw.len() - 1].to_string();
        mapping.insert(key.to_string(), (left, right));
    }
    return (seed.trim().to_string(), mapping);
}

fn simulate(instructions: &str, memory: BTreeMap<String, (String, String)>) -> u64 {
    let mut current_pos = 0;
    let mut acc = 0;
    let instructions: Vec<String> = instructions.chars().map(|c| c.to_string()).collect();
    let num_instructions = instructions.len();
    let mut current_node = "AAA".to_string();
    while current_node != *"ZZZ" {
        let instruction = &instructions[current_pos % num_instructions];
        println!("{}: {}: {}", current_pos, instruction, current_node);
        let next_node = if instruction == "L" {
            memory.get(&current_node).unwrap().0.clone()
        } else {
            memory.get(&current_node).unwrap().1.clone()
        };
        current_node = next_node;
        current_pos += 1;
        acc += 1;
    }
    acc
}

fn simulate_two(instructions: &str, memory: BTreeMap<String, (String, String)>) -> u64 {
    let instructions: Vec<String> = instructions.chars().map(|c| c.to_string()).collect();
    let num_instructions = instructions.len();
    let current_nodes = memory
        .keys()
        .filter(|k| k.ends_with('A')).cloned()
        .collect_vec();
    let mut num_steps = vec![];
    for node in current_nodes.iter() {
        let mut current_pos = 0;
        let mut current_node = node.clone();
        let mut acc = 0;
        while !current_node.ends_with('Z') {
            let instruction = &instructions[current_pos % num_instructions];
            println!("{}: {}: {}", current_pos, instruction, current_node);
            let next_node = if instruction == "L" {
                memory.get(&current_node).unwrap().0.clone()
            } else {
                memory.get(&current_node).unwrap().1.clone()
            };
            current_node = next_node;
            current_pos += 1;
            acc += 1;
        }
        num_steps.push(acc);
    }
    // compute the lcm of the numbers in num_steps
    let mut lcm = num_steps[0];
    for i in 1..num_steps.len() {
        lcm = num::integer::lcm(lcm, num_steps[i]);
    }
    lcm
}

#[cfg(test)]
mod tests {
    #[test]
    fn solves_8_1_easy() {
        let input = std::fs::read_to_string("input/8_easy.txt").unwrap();
        assert_eq!(super::solve_part_one(&input), 2);
    }

    #[test]
    fn solves_8_1_med() {
        let input = std::fs::read_to_string("input/8_easy1.txt").unwrap();
        assert_eq!(super::solve_part_one(&input), 6);
    }

    #[test]
    fn solves_8_1_hard() {
        let input = std::fs::read_to_string("input/8_real.txt").unwrap();
        assert_eq!(super::solve_part_one(&input), 20221);
    }

    #[test]
    fn solves_8_2_easy() {
        let input = std::fs::read_to_string("input/8_easy2.txt").unwrap();
        assert_eq!(super::solve_part_two(&input), 6);
    }

    #[test]
    fn solves_8_2_hard() {
        let input = std::fs::read_to_string("input/8_real.txt").unwrap();
        assert_eq!(super::solve_part_two(&input), 14616363770447);
    }
}
