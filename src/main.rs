use aoc2023_lib::dayfourteen;

fn main() {
    let input = std::fs::read_to_string("input/14_real.txt").unwrap();
    let res = dayfourteen::solve_part_two(&input, 1000000000);
    println!("Result: {}", res);
}
