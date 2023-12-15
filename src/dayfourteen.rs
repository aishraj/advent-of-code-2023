use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

use crate::utils::{reverse_cols, reverse_rows, transpose};

pub fn solve_part_one(input: &str) -> usize {
    // transpose the input
    let input = parse_input(input);
    let transposed_input = transpose(input);
    let result = move_rocks(transposed_input);
    let result = transpose(result);
    println!("Result: {:?}", result);
    // In order to compute the result, we multiply the number of "O" in each row multiplied by rownum+1
    let mut c = 0;
    let num_lines = result.len();
    for (i, line) in result.iter().enumerate() {
        let num_stones = line.iter().filter(|c| **c == 'O').count();
        println!("Row {} has {} stones", i, num_stones);
        c += (num_lines - i) * num_stones;
    }
    c
}

fn move_rocks(transposed_input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result = vec![];
    for line in transposed_input {
        let slided_line = slide_stone(line);
        result.push(slided_line);
    }
    result
}

pub fn solve_part_two(input: &str, times: usize) -> usize {
    let mut input = parse_input(input);
    let mut current = 0;
    let mut seen = HashMap::new();
    while current < times {
        current += 1;
        input = solve_cycle_once(input);
        if seen.contains_key(&input) {
            let cycle_lengths = current - seen.get(&input).unwrap();
            println!("Cycle length: {}", cycle_lengths);
            let num_cycles = (times - current) / cycle_lengths;
            println!("Number of cycles: {}", num_cycles);
            current += num_cycles * cycle_lengths;
        }
        seen.insert(input.clone(), current);
    }
    let load = compute_load(input);
    println!("Load: {}", load);
    load
}

fn compute_load(grid: Vec<Vec<char>>) -> usize {
    let mut c = 0;
    let num_lines = grid.len();
    for (i, line) in grid.iter().enumerate() {
        let num_stones = line.iter().filter(|c| **c == 'O').count();
        //println!("Row {} has {} stones", i, num_stones);
        c += (num_lines - i) * num_stones;
    }
    c
}

pub fn solve_cycle_once(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    // in part two we compute the numbers based the following inputs
    // 0. the transpose of the input (north) like we do in part 1
    // 1. the column-wise reverse of the input (west)
    // 2. the row-wise reverse of the input (south)
    // 3. the original input (east)
    let mut input = input.clone();
    for i in 0..4 {
        input = match i {
            0 => {
                let transposed_input = transpose(input);
                let rocks = move_rocks(transposed_input);
                transpose(rocks)
            }
            1 => move_rocks(input),
            2 => {
                //somehow this does not work
                let reversed_rows_input = reverse_rows(input);
                let transposed_input = transpose(reversed_rows_input);
                let rocks = move_rocks(transposed_input);
                let rocks = transpose(rocks);
                reverse_rows(rocks)
            }
            3 => {
                let reversed_cols_input = reverse_cols(input);
                let rocks = move_rocks(reversed_cols_input);
                reverse_cols(rocks)
            }
            _ => panic!("Invalid input"),
        };
        //println!("At the end of cycle {} the grid is:", i,);
        //pretty_print_grid(input.clone());
    }
    input
}

/// This function slides all stone "O" to the left or to the closest # on the right.
fn slide_stone(input: Vec<char>) -> Vec<char> {
    let mut queue = VecDeque::from(input);
    queue.push_front('#');
    let mut result = vec!['.'; queue.len()];
    let mut group_start = 0;
    let mut num_dots = 0;
    for (i, c) in queue.iter().enumerate() {
        if *c == '#' {
            result[i] = *c;
            group_start = i;
            num_dots = 0;
        } else if *c == '.' {
            num_dots += 1;
        } else {
            let next_pos = group_start + (i - num_dots - group_start);
            result[next_pos] = *c;
        }
    }
    return result[1..].iter().copied().collect_vec();
}

fn pretty_print_grid(grid: Vec<Vec<char>>) {
    let mut result = String::new();
    for line in grid {
        let line: String = line.iter().collect();
        result.push_str(&format!("{}\n", line));
    }
    println!("{}", result);
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.trim().chars().collect_vec())
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn slides_basic_stone() {
        let line = "OO.O.O..##".chars().collect_vec();
        let expected = "OOOO....##".chars().collect_vec();
        assert_eq!(super::slide_stone(line), expected);
    }

    #[test]
    fn slides_with_rock_middle() {
        let line = ".O...#O..O".chars().collect_vec();
        let expected = "O....#OO..".chars().collect_vec();
        assert_eq!(super::slide_stone(line), expected);
    }

    #[test]
    fn slides_test_without_transpose() {
        let transposed_input = "OO.O.O..##
        ...OO....O
        .O...#O..O
        .O.#......
        .#.O......
        #.#..O#.##
        ..#...O.#.
        ....O#.O#.
        ....#.....
        .#.O.#O...";
        let expected_lines = "OOOO....##
        OOO.......
        O....#OO..
        O..#......
        .#O.......
        #.#O..#.##
        ..#O....#.
        O....#O.#.
        ....#.....
        .#O..#O...";
        let input_lines = transposed_input.lines().collect_vec();
        let output_lines = expected_lines.lines().collect_vec();
        for i in 0..input_lines.len() {
            let input = input_lines[i].trim().chars().collect_vec();
            let expected = output_lines[i].trim().chars().collect_vec();
            println!("Processing input: {:?}", input);
            let actual = super::slide_stone(input);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_entire_cylce() {
        let expected_raw = ".....#....
        ....#...O#
        ...OO##...
        .OO#......
        .....OOO#.
        .O#...O#.#
        ....O#....
        ......OOOO
        #...O###..
        #..OO#....";
        let input = "O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....";
        let input_vec = parse_input(input);
        let expected_vec = parse_input(expected_raw);
        let actual = super::solve_cycle_once(input_vec);
        assert_eq!(actual, expected_vec);
    }

    #[test]
    fn solves_14_1_easy() {
        let input = std::fs::read_to_string("input/14_easy.txt").unwrap();
        assert_eq!(super::solve_part_one(&input), 136);
    }

    #[test]
    fn solves_14_1_hard() {
        let input = std::fs::read_to_string("input/14_real.txt").unwrap();
        assert_eq!(super::solve_part_one(&input), 108641);
    }

    #[test]
    fn solves_14_2_easy() {
        let input = std::fs::read_to_string("input/14_easy.txt").unwrap();
        assert_eq!(solve_part_two(&input, 1000000000), 64);
    }

    #[test]
    fn solves_14_2_hard() {
        let input = std::fs::read_to_string("input/14_real.txt").unwrap();
        assert_eq!(solve_part_two(&input, 1000000000), 84328);
    }
}
