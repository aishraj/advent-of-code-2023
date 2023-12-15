pub fn solve_part_one(input: &str) -> i32 {
    let parsed_input = parse(input);
    let extrapolated = parsed_input
        .iter()
        .map(extrapolate)
        .collect::<Vec<_>>();
    extrapolated.iter().sum()
}

pub fn solve_part_two(input: &str) -> i32 {
    let parsed = parse(input);
    let extrapolated = parsed
        .iter()
        .map(|numbers| extrapolate_second(numbers))
        .collect::<Vec<_>>();
    extrapolated.iter().sum()
}

fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn extrapolate(numbers: &Vec<i32>) -> i32 {
    let diffs = find_diffs(numbers);
    if diffs.iter().all(|&x| x == 0) {
        *numbers.last().unwrap()
    } else {
        *numbers.last().unwrap() + extrapolate(&diffs)
    }
}

fn extrapolate_second(numbers: &[i32]) -> i32 {
    let diffs = find_diffs(numbers);
    if diffs.iter().all(|&x| x == 0) {
        *numbers.first().unwrap()
    } else {
        *numbers.first().unwrap() - extrapolate_second(&diffs)
    }
}

fn find_diffs(numbers: &[i32]) -> Vec<i32> {
    numbers
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn solves_9_1_easy() {
        let input = std::fs::read_to_string("input/9_easy.txt").unwrap();
        assert_eq!(super::solve_part_one(&input), 114);
    }

    #[test]
    fn solves_9_1_hard() {
        let input = std::fs::read_to_string("input/9_real.txt").unwrap();
        assert_eq!(super::solve_part_one(&input), 2098530125);
    }

    #[test]
    fn solves_9_2_easy() {
        let input = std::fs::read_to_string("input/9_easy.txt").unwrap();
        assert_eq!(super::solve_part_two(&input), 2);
    }

    #[test]
    fn solves_9_2_hard() {
        let input = std::fs::read_to_string("input/9_real.txt").unwrap();
        assert_eq!(super::solve_part_two(&input), 1016);
    }
}
