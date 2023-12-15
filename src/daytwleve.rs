use itertools::FoldWhile::Continue;
use itertools::FoldWhile::Done;
use itertools::Itertools;
use memoize::memoize;

pub fn solve_part_one(input: &str) -> u64 {
    let input = parse_input(input);
    let mut res = 0;
    for (spring, groups) in input {
        res += recur(0, 0, spring, groups);
    }
    res as u64
}

pub fn solve_part_two(input: &str) -> u64 {
    let input = parse_input(input);
    let mut res = 0;
    for (rec, group) in input {
        let unfolded_spring = unfold_spring(rec.clone());
        let unfolded_group = unfold_group(group.clone());
        res += recur(0, 0, unfolded_spring, unfolded_group);
    }
    res as u64
}

fn unfold_spring(spring: String) -> String {
    spring
        .repeat(5)
        .chars()
        .collect::<Vec<_>>()
        .chunks(spring.len())
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("?")
}

fn unfold_group(group: Vec<usize>) -> Vec<usize> {
    // repeat the whole group 5 times
    let mut res = vec![];
    for _ in 0..5 {
        res.extend(group.clone().iter());
    }
    res
}

fn parse_input(input: &str) -> Vec<(String, Vec<usize>)> {
    input.lines().map(parse_input_line).collect()
}

#[memoize]
fn recur(i: usize, j: usize, all_springs: String, group_sizes: Vec<usize>) -> i64 {
    if i >= all_springs.len() {
        return if j < group_sizes.len() { 0 } else { 1 };
    }
    let rec = all_springs.chars().collect::<Vec<char>>();

    if rec[i] == '.' {
        return recur(i + 1, j, rec.iter().collect(), group_sizes.to_vec());
    } else {
        let mut res = 0;
        if rec[i] == '?' {
            res += recur(i + 1, j, rec.iter().collect(), group_sizes.to_vec());
        }
        if j >= group_sizes.len() {
            return res;
        }
        let count = (i..rec.len())
            .map(|k| rec[k])
            .fold_while(0, |count, k| {
                if count > group_sizes[j] || k == '.' || count == group_sizes[j] && k == '?' {
                    Done(count)
                } else {
                    Continue(count + 1)
                }
            })
            .into_inner();

        if count == group_sizes[j] {
            if i + count < rec.len() && rec[i + count] != '#' {
                res += recur(
                    i + count + 1,
                    j + 1,
                    rec.iter().collect(),
                    group_sizes.to_vec(),
                );
            } else {
                res += recur(i + count, j + 1, rec.iter().collect(), group_sizes.to_vec());
            }
        }
        res
    }
}

fn parse_input_line(line: &str) -> (String, Vec<usize>) {
    let (firsthalf, secondhalf) = line.split_whitespace().collect_tuple().unwrap();
    let secondhalf = secondhalf
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    (firsthalf.to_string(), secondhalf)
}

#[cfg(test)]
mod tests {
    #[test]
    fn solves_12_1_easy() {
        let input = std::fs::read_to_string("input/12_easy.txt").unwrap();
        assert_eq!(super::solve_part_one(&input), 21);
    }

    #[test]
    fn solves_12_1_hard() {
        let input = std::fs::read_to_string("input/12_real.txt").unwrap();
        assert_eq!(super::solve_part_one(&input), 7622);
    }

    #[test]
    fn solves_12_2_easy() {
        let input = std::fs::read_to_string("input/12_easy.txt").unwrap();
        assert_eq!(super::solve_part_two(&input), 525152);
    }

    #[test]
    fn solves_12_2_hard() {
        let input = std::fs::read_to_string("input/12_real.txt").unwrap();
        assert_eq!(super::solve_part_two(&input), 4964259839627);
    }
}
