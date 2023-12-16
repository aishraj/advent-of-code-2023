use itertools::Itertools;

pub fn solve_part_one(input: &str) -> u32 {
    parse(input).iter().map(|s| hash_function(s)).sum()
}

pub fn solve_part_two(input: &str) -> usize {
    let groups: Vec<&str> = input.split(',').collect();

    let mut items: Vec<Vec<(String, u32)>> = vec![vec![]; 256];

    groups.iter().for_each(|it| {
        let name = if it.ends_with('-') {
            &it[..it.len() - 1]
        } else if it.chars().nth_back(1).unwrap() == '=' {
            &it[..it.len() - 2]
        } else {
            return;
        };

        let hash_value = hash_function(name);

        if it.ends_with('-') {
            items[hash_value as usize] = items[hash_value as usize]
                .clone()
                .into_iter()
                .filter(|(n, _)| n != name)
                .collect();
        } else if it.chars().nth_back(1).unwrap() == '=' {
            let length = it.chars().last().unwrap().to_digit(10).unwrap();
            let item = items[hash_value as usize]
                .clone()
                .into_iter()
                .map(|(n, v)| {
                    if n == name {
                        (n.clone(), length)
                    } else {
                        (n.clone(), v)
                    }
                })
                .collect_vec();

            if items[hash_value as usize].iter().any(|(n, _)| n == name) {
                items[hash_value as usize] = item;
            } else {
                items[hash_value as usize].push((name.to_string(), length));
            }
        }
    });

    items
        .iter()
        .enumerate()
        .flat_map(|(i, box_item)| {
            box_item
                .iter()
                .enumerate()
                .map(move |(j, (_, v))| (i + 1) * (j + 1) * (*v as usize))
        })
        .sum()
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
        assert_eq!(super::solve_part_two(&input), 145);
    }

    #[test]
    fn solves_15_2_hard() {
        let input = std::fs::read_to_string("input/15_real.txt").unwrap();
        assert_eq!(super::solve_part_two(&input), 245223);
    }
}
