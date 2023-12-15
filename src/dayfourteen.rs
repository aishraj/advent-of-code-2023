use std::collections::VecDeque;

use itertools::Itertools;

use crate::utils::transpose;

pub fn solve_part_one(input: &str) -> usize {
    // transpose the input
    let input = parse_input(input);
    let transposed_input = transpose(input);
    let mut transposed_result = vec![];
    for line in transposed_input {
        let slided_line = slide_stone(line);
        transposed_result.push(slided_line);
    }
    let result = transpose(transposed_result);
    println!("Result: {:?}", result);
    // In order to compute the result, we multiply the number of "O" in each row multiplied by rownum+1
    let mut c = 0;
    let num_lines = result.len();
    for (i, line) in result.iter().enumerate() {
        let num_stones = line.iter().filter(|c| **c == 'O').count();
        println!("Row {} has {} stones", i, num_stones);
        c += (num_lines - i) * num_stones;
    }
    return c;
}

pub fn solve_part_two(input: &str) -> u32 {
    42
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

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

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
        assert_eq!(super::solve_part_two(&input), 42);
    }

    #[test]
    fn solves_14_2_hard() {
        let input = std::fs::read_to_string("input/14_real.txt").unwrap();
        assert_eq!(super::solve_part_two(&input), 42);
    }
}
