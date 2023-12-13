pub fn solve_part_one(input: &str) -> u64 {
    input
        .split("\n\n")
        .map(|x| solve(x.to_string(), 0))
        .sum::<usize>() as u64
}

pub fn solve_part_two(input: &str) -> u64 {
    input
        .split("\n\n")
        .map(|x| solve(x.to_string(), 2))
        .sum::<usize>() as u64
}

fn solve(pattern: String, wiggle_room: usize) -> usize {
    let pattern: Vec<Vec<char>> = pattern.lines().map(|x| x.chars().collect()).collect();
    match reflect(pattern.clone(), wiggle_room) {
        Some(row) => 100 * (row + 1),
        None => match reflect(transpose(pattern), wiggle_room) {
            Some(col) => col + 1,
            None => 0,
        },
    }
}

fn reflect(pattern: Vec<Vec<char>>, wiggle_room: usize) -> Option<usize> {
    let len = pattern.len();
    (0..len - 1).find(|&i| {
        let c = (0..len)
            .filter_map(|j| {
                let pos = i as i64 + 1 + (i as i64 - j as i64);
                if pos < 0 || pos >= len as i64 {
                    None
                } else if pattern[j] != pattern[pos as usize] {
                    Some(
                        pattern[j]
                            .iter()
                            .zip(pattern[pos as usize].iter())
                            .filter(|&(a, b)| a != b)
                            .count(),
                    )
                } else {
                    None
                }
            })
            .sum::<usize>();
        c == wiggle_room
    })
}

fn transpose<T: Clone>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    for i in 0..v[0].len() {
        let mut row = Vec::new();
        for j in 0..v.len() {
            row.push(v[j][i].clone());
        }
        result.push(row);
    }
    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn solves_13_1_easy() {
        let input = std::fs::read_to_string("input/13_easy.txt").unwrap();
        assert_eq!(super::solve_part_one(&input), 405);
    }

    #[test]
    fn solves_13_1_hard() {
        let input = std::fs::read_to_string("input/13_real.txt").unwrap();
        assert_eq!(super::solve_part_one(&input), 26957);
    }

    #[test]
    fn solves_13_2_easy() {
        let input = std::fs::read_to_string("input/13_easy.txt").unwrap();
        assert_eq!(super::solve_part_two(&input), 400);
    }

    #[test]
    fn solves_13_2_hard() {
        let input = std::fs::read_to_string("input/13_real.txt").unwrap();
        assert_eq!(super::solve_part_two(&input), 42695);
    }
}
