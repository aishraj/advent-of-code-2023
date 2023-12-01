pub fn solve_part_one(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .map(|line| {
            let digits: Vec<_> = line
                .chars()
                .filter(|c| c.is_digit(10))
                .map(|c| c.to_digit(10).unwrap())
                .collect();
            println!("Digits is {:?}", digits);
            let product = (digits.first().unwrap() * 10) + digits.last().unwrap();
            return product;
        })
        .sum()
}

fn find_all(input: &str, substring: &str) -> Vec<usize> {
    let mut positions = vec![];
    let mut start = 0;
    while let Some(pos) = input[start..].find(substring) {
        positions.push(start + pos);
        start += pos + 1;
    }
    positions
}

pub fn solve_part_two(input: &str) -> u32 {
    let words = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    input
        .trim()
        .lines()
        .map(|line| {
            let mut digit_pos_pairs = vec![];
            for word in words.iter() {
                for index in find_all(line, word) {
                    digit_pos_pairs.push((convert_to_num(word), index));
                }
            }
            let digits_pos: Vec<_> = line
                .chars()
                .enumerate()
                .filter(|(_pos, c)| c.is_digit(10))
                .map(|(pos, c)| (c.to_digit(10).unwrap(), pos))
                .collect();
            digit_pos_pairs.extend(digits_pos);
            digit_pos_pairs.sort_by(|a, b| a.1.cmp(&b.1));

            digit_pos_pairs
        })
        .map(|pairs| {
            let numbers: Vec<_> = pairs.iter().map(|p| p.0).collect();
            let product = (numbers.first().unwrap() * 10) + numbers.last().unwrap();
            return product;
        })
        .sum()
}

fn convert_to_num(word: &str) -> u32 {
    match word {
        "zero" => 0,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => panic!("Invalid word"),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn solves_1_1_easy() {
        let input = std::fs::read_to_string("input/1_easy.txt").unwrap();
        assert_eq!(super::solve_part_one(&input), 142);
    }

    #[test]
    fn solves_1_1_hard() {
        let input = std::fs::read_to_string("input/1_real.txt").unwrap();
        assert_eq!(super::solve_part_one(&input), 55816);
    }

    #[test]
    fn solves_1_2_easy() {
        let input = std::fs::read_to_string("input/1_easy_2.txt").unwrap();
        assert_eq!(super::solve_part_two(&input), 281);
    }

    #[test]
    fn solves_1_2_hard() {
        let input = std::fs::read_to_string("input/1_real.txt").unwrap();
        assert_eq!(super::solve_part_two(&input), 54980);
    }
}
