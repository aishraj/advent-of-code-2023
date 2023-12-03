use self::parser::parse_input;

#[derive(Debug, PartialEq)]
pub enum Cube {
    Blue(usize),
    Red(usize),
    Green(usize),
}

mod parser {
    use itertools::Itertools;

    use super::Cube;

    pub fn parse_input(input: &str) -> Vec<Vec<Vec<Cube>>> {
        input
            .trim()
            .lines()
            .map(|line| line.trim())
            .map(|line| parse_line(line))
            .collect_vec()
    }

    pub fn parse_line(line: &str) -> Vec<Vec<Cube>> {
        line.split(";")
            .map(|inner| parse_inner(inner))
            .collect_vec()
    }

    pub fn parse_inner(mut section: &str) -> Vec<Cube> {
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
}

pub fn solve_part_one(input: &str, rgb_limits: (u32, u32, u32)) -> u32 {
    let parsed = parser::parse_input(input);
    let mut possible_games = vec![];
    for (index, line) in parsed.iter().enumerate() {
        if is_line_possible(line, rgb_limits) {
            possible_games.push(index + 1);
        }
    }
    println!("Possible games: {:?}", possible_games);
    let total = possible_games.iter().map(|item| *item as u32).sum();
    return total;
}

fn is_line_possible(parsed_line: &Vec<Vec<Cube>>, limits: (u32, u32, u32)) -> bool {
    for cube in parsed_line.iter().flatten() {
        match cube {
            Cube::Red(count) => {
                if *count > limits.0 as usize {
                    println!("Red: {}", count);
                    return false;
                }
            }
            Cube::Green(count) => {
                if *count > limits.1 as usize {
                    println!("Blue: {} and limit: {}", count, limits.1);
                    return false;
                }
            }
            Cube::Blue(count) => {
                if *count > limits.2 as usize {
                    println!("Green: {}", count);
                    return false;
                }
            }
        }
    }
    return true;
}

pub fn solve_part_two(input: &str) -> u32 {
    let parsed = parse_input(input);
    parsed
        .iter()
        .map(|line| get_power_from_line(line))
        .sum::<u32>()
}

fn get_power_from_line(parsed_line: &Vec<Vec<Cube>>) -> u32 {
    let (red, green, blue) = max_by_color(parsed_line);
    return (red * green * blue) as u32;
}

fn max_by_color(parsed_line: &Vec<Vec<Cube>>) -> (usize, usize, usize) {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for cube in parsed_line.iter().flatten() {
        match cube {
            Cube::Red(count) => {
                if *count > red {
                    red = *count;
                }
            }
            Cube::Green(count) => {
                if *count > green {
                    green = *count;
                }
            }
            Cube::Blue(count) => {
                if *count > blue {
                    blue = *count;
                }
            }
        }
    }
    return (red, green, blue);
}

#[cfg(test)]
mod tests {
    use crate::daytwo::parser::parse_line;

    #[test]
    fn test_simple_power_from_line() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let expected = 48;
        let parsed_line = parse_line(line);
        let actual = super::get_power_from_line(&parsed_line);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_if_line_is_possible() {
        let line = "Game 5: 8 green, 1 red, 12 blue; 10 green, 6 red, 13 blue; 1 red, 3 blue, 6 green; 14 blue, 2 red, 7 green";
        let expected = vec![
            vec![
                super::Cube::Green(8),
                super::Cube::Red(1),
                super::Cube::Blue(12),
            ],
            vec![
                super::Cube::Green(10),
                super::Cube::Red(6),
                super::Cube::Blue(13),
            ],
            vec![
                super::Cube::Red(1),
                super::Cube::Blue(3),
                super::Cube::Green(6),
            ],
            vec![
                super::Cube::Blue(14),
                super::Cube::Red(2),
                super::Cube::Green(7),
            ],
        ];
        let actual = parse_line(line);
        assert_eq!(actual, expected);
        let exected_possible = true;
        let actual_possible = super::is_line_possible(&actual, (12, 13, 14));
        assert_eq!(actual_possible, exected_possible);
    }

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
        let actual = parse_line(line);
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
        let actual = parse_line(line);
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
        assert_eq!(super::solve_part_one(&input, (12, 13, 14)), 3035);
    }

    #[test]
    fn solves_2_2_easy() {
        let input = std::fs::read_to_string("input/2_easy.txt").unwrap();
        assert_eq!(super::solve_part_two(&input), 2286);
    }

    #[test]
    fn solves_2_2_hard() {
        let input = std::fs::read_to_string("input/2_hard.txt").unwrap();
        assert_eq!(super::solve_part_two(&input), 66027);
    }
}
