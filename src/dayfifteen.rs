pub fn solve_part_one(input: &str) -> u32 {
    parse(input).iter().map(|s| hash_function(s)).sum()
}

pub fn solve_part_two(input: &str) -> u32 {
    42
}

fn parse(input: &str) -> Vec<&str> {
    input.split(',').collect()
}

fn hash_function(s: &str) -> u32 {
    s.chars().fold(0, |h, c| ((h + c as u32) * 17) % 256)
}

#[cfg(test)]
mod tests {
    #[test]
    fn solves_15_1_easy() {
        let input = std::fs::read_to_string("input/15_easy.txt").unwrap();
        assert_eq!(super::solve_part_one(&input), 1320);
    }

    #[test]
    fn solves_15_1_hard() {
        let input = std::fs::read_to_string("input/15_real.txt").unwrap();
        assert_eq!(super::solve_part_one(&input), 495972);
    }

    #[test]
    fn solves_15_2_easy() {
        let input = std::fs::read_to_string("input/15_easy.txt").unwrap();
        assert_eq!(super::solve_part_two(&input), 42);
    }

    #[test]
    fn solves_15_2_hard() {
        let input = std::fs::read_to_string("input/15_real.txt").unwrap();
        assert_eq!(super::solve_part_two(&input), 42);
    }
}
