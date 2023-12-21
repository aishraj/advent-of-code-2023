use aoc2023_lib::daytwentyone::solve_part_one;

fn main() {
    let input = std::fs::read_to_string("input/21_real.txt").unwrap();
    let res = solve_part_one(&input, 64);
    println!("{}", res);
}
