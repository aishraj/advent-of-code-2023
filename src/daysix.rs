use core::time;

use itertools::Itertools;

pub fn solve_part_one(input: &str) -> u32 {
    let (times, dist) = parse_input(input);
    let mut prod = 1;
    for i in 0..dist.len() {
        let time = times[i];
        let dist = dist[i];
        let mut numbers = vec![];
        for x in 0..=time {
            let dx = x * (time - x);
            if dx > dist {
                numbers.push(x);
            }
        }
        println!("numbers: {:?}", numbers);
        prod *= numbers.len() as u32;
    }
    return prod;
}

pub fn solve_part_two(input: &str) -> u32 {
    42
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let lines: Vec<&str> = input.lines().collect();
    let mut ans = 0;

    let times: Vec<u32> = lines[0].split(':').collect_vec()[1]
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let dist: Vec<u32> = lines[1].split(':').collect_vec()[1]
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    return (times, dist);
}

#[cfg(test)]
mod tests {

    #[test]
    fn parses_simple_input() {
        let input = "Time:      7  15   30\nDistance:  9  40  200";
        let (times, dist) = super::parse_input(input);
        assert_eq!(times, vec![7, 15, 30]);
        assert_eq!(dist, vec![9, 40, 200]);
    }

    #[test]
    fn solves_6_1_easy() {
        let input = std::fs::read_to_string("input/6_easy.txt").unwrap();
        assert_eq!(super::solve_part_one(&input), 288);
    }

    #[test]
    fn solves_6_1_hard() {
        let input = std::fs::read_to_string("input/6_real.txt").unwrap();
        assert_eq!(super::solve_part_one(&input), 2065338);
    }

    #[test]
    fn solves_6_2_easy() {
        let input = std::fs::read_to_string("input/6_easy.txt").unwrap();
        assert_eq!(super::solve_part_two(&input), 42);
    }

    #[test]
    fn solves_6_2_hard() {
        let input = std::fs::read_to_string("input/6_real.txt").unwrap();
        assert_eq!(super::solve_part_two(&input), 42);
    }
}
