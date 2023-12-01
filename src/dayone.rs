pub fn solve_part_one(input: &str) -> u32 {
    42
}

pub fn solve_part_two(input: &str) -> u32 {
    42
}

fn parse_input(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn solves_1_1_easy() {
        let input = std::fs::read_to_string("input/1_easy.txt").unwrap();
        assert_eq!(super::solve_part_one(&input), 3);
    }

    #[test]
    fn solves_1_1_hard() {
        let input = std::fs::read_to_string("input/1_real.txt").unwrap();
        assert_eq!(super::solve_part_one(&input), 1141);
    }

    #[test]
    fn solves_1_2_easy() {
        let input = std::fs::read_to_string("input/1_easy.txt").unwrap();
        assert_eq!(super::solve_part_one(&input), 3);
    }

    #[test]
    fn solves_1_2_hard() {
        let input = std::fs::read_to_string("input/1_real.txt").unwrap();
        assert_eq!(super::solve_part_one(&input), 1141);
    }
}
