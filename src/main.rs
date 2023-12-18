use aoc2023_lib::dayeighteen::solve_part_one;

fn main() {
    let input = std::fs::read_to_string("input/18_real.txt").unwrap();
    assert_eq!(solve_part_one(&input), 42);
}
