use itertools::Itertools;

#[derive(Debug, PartialEq)]
enum Cube {
    Blue(usize),
    Red(usize),
    Green(usize),
}

pub fn solve_part_one(input: &str, rgb_limits: (u32, u32, u32)) -> u32 {
    let parsed = parse_input(input);
    let mut impossible_games = vec![];
    for (pos, draws) in parsed.iter().enumerate() {
        'outer: for draw in draws {
            for cube in draw {
                match cube {
                    Cube::Red(count) => {
                        if *count > rgb_limits.0 as usize {
                            impossible_games.push(pos + 1);
                            break 'outer;
                        }
                    }
                    Cube::Blue(count) => {
                        if *count > rgb_limits.1 as usize {
                            impossible_games.push(pos + 1);
                            break 'outer;
                        }
                    }
                    Cube::Green(count) => {
                        if *count > rgb_limits.2 as usize {
                            impossible_games.push(pos + 1);
                            break 'outer;
                        }
                    }
                }
            }
        }
    }
    println!("Impossible games: {:?}", impossible_games);
    let mut all_games = (1..=parsed.len()).collect_vec();
    all_games.retain(|x| !impossible_games.contains(x));
    println!("Possible games: {:?}", all_games);
    let total = all_games.iter().map(|item| *item as u32).sum();
    return total;
}

pub fn solve_part_two(input: &str) -> u32 {
    42
}

fn parse_input(input: &str) -> Vec<Vec<Vec<Cube>>> {
    input
        .trim()
        .lines()
        .map(|line| line.trim())
        .map(|line| parse_line(line))
        .collect_vec()
}

fn parse_line(line: &str) -> Vec<Vec<Cube>> {
    line.split(";")
        .map(|inner| parse_inner(inner))
        .collect_vec()
}

fn parse_inner(mut section: &str) -> Vec<Cube> {
    if section.contains(":") {
        section = section.split(":").collect_vec()[1];
    }
    section = section.trim();
    let items = section
        .split(",")
        .map(|item| {
            let (count, color) = item.split_whitespace().collect_tuple().unwrap();
            let count: usize = count.parse().unwrap();
            return match color {
                "red" => Cube::Red(count),
                "blue" => Cube::Blue(count),
                "green" => Cube::Green(count),
                _ => panic!("Invalid cube color"),
            };
        })
        .collect_vec();
    return items;
}

#[cfg(test)]
mod tests {

    #[test]
    fn parse_line_whole() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let expected = vec![
            vec![super::Cube::Blue(3), super::Cube::Red(4)],
            vec![
                super::Cube::Red(1),
                super::Cube::Green(2),
                super::Cube::Blue(6),
            ],
            vec![super::Cube::Green(2)],
        ];
        let actual = super::parse_line(line);
        assert_eq!(actual, expected);
    }

    #[test]
    fn parses_another_line() {
        let line = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        let expected = vec![
            vec![
                super::Cube::Green(1),
                super::Cube::Red(3),
                super::Cube::Blue(6),
            ],
            vec![super::Cube::Green(3), super::Cube::Red(6)],
            vec![
                super::Cube::Green(3),
                super::Cube::Blue(15),
                super::Cube::Red(14),
            ],
        ];
        let actual = super::parse_line(line);
        assert_eq!(actual, expected);
    }

    #[test]
    fn solves_2_1_easy() {
        let input = std::fs::read_to_string("input/2_easy.txt").unwrap();
        assert_eq!(super::solve_part_one(&input, (12, 13, 14)), 8);
    }

    #[test]
    fn solves_2_1_hard() {
        let input = std::fs::read_to_string("input/2_hard.txt").unwrap();
        assert_eq!(super::solve_part_one(&input, (12, 13, 14)), 1141);
    }

    #[test]
    fn solves_2_2_easy() {
        let input = std::fs::read_to_string("input/2_easy.txt").unwrap();
        assert_eq!(super::solve_part_two(&input), 3);
    }

    #[test]
    fn solves_2_2_hard() {
        let input = std::fs::read_to_string("input/2_hard.txt").unwrap();
        assert_eq!(super::solve_part_two(&input), 1141);
    }
}
