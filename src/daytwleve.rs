use itertools::FoldWhile::Continue;
use itertools::FoldWhile::Done;
use itertools::Itertools;
use memoize::memoize;

pub fn solve_part_one(input: &str) -> u64 {
    let input = parse_input(input);
    let mut res = 0;
    for (rec, group) in input {
        res += recur(0, 0, rec, group);
    }
    return res as u64;
}

pub fn solve_part_two(input: &str) -> u32 {
    42
}

fn parse_input(input: &str) -> Vec<(String, Vec<u64>)> {
    input.lines().map(|x| parse_input_line(x)).collect()
}

#[memoize]
fn recur(i: usize, j: usize, all_springs: String, group_sizes: Vec<u64>) -> i64 {
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
        if j < group_sizes.len() {
            let count = (i..rec.len())
                .map(|k| rec[k])
                .fold_while(0, |count, k| {
                    if count > group_sizes[j] as usize
                        || k == '.'
                        || count == group_sizes[j] as usize && k == '?'
                    {
                        Done(count)
                    } else {
                        Continue(count + 1)
                    }
                })
                .into_inner();

            if count == group_sizes[j] as usize {
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
        }
        return res;
    }
}

fn parse_input_line(line: &str) -> (String, Vec<u64>) {
    let (firsthalf, secondhalf) = line.split_whitespace().collect_tuple().unwrap();
    let secondhalf = secondhalf
        .split(',')
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    return (firsthalf.to_string(), secondhalf);
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
        assert_eq!(super::solve_part_one(&input), 42);
    }

    #[test]
    fn solves_12_2_hard() {
        let input = std::fs::read_to_string("input/12_real.txt").unwrap();
        assert_eq!(super::solve_part_one(&input), 42);
    }
}
