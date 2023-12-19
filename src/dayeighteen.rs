use std::collections::{BTreeSet, VecDeque};

use itertools::Itertools;

#[derive(Debug, Clone)]
enum Direction {
    Left(i64),
    Right(i64),
    Up(i64),
    Down(i64),
}

fn shoelace(vertices: &Vec<(i64, i64)>) -> f64 {
    let mut sum = 0.0;
    let x = vertices.iter().map(|(x, _)| *x).collect_vec();
    let y = vertices.iter().map(|(_, y)| *y).collect_vec();
    for i in 0..x.len() {
        let next_index = (i + 1) % x.len();
        sum += (x[i] * y[next_index]) as f64 - (x[next_index] * y[i]) as f64;
    }
    (sum / 2.0).abs()
}

fn picks(a: f64, b: usize) -> f64 {
    a + 1.0 - (b as f64) / 2.0
}

pub fn solve_part_one(input: &str) -> i64 {
    let input = parse_input(input);
    let directions = input.iter().map(|(dir, _)| dir.clone()).collect_vec();
    solve_inner(directions)
}

fn solve_inner(dirs: Vec<Direction>) -> i64 {
    let mut point = (0, 0);
    let mut points = BTreeSet::new();
    let mut locations = vec![];
    for direction in dirs.into_iter() {
        let (dir, amt) = match direction {
            Direction::Down(amount) => ((-1, 0), amount),
            Direction::Up(amount) => ((1, 0), amount),
            Direction::Left(amount) => ((0, -1), amount),
            Direction::Right(amount) => ((0, 1), amount),
        };
        for i in 0..=amt {
            let next_point = (point.0 + i * dir.0, point.1 + i * dir.1);
            points.insert(next_point);
        }
        // translate the point by the magnitude and direction
        point = (point.0 + amt * dir.0, point.1 + amt * dir.1);
        locations.push(point);
    }
    let area = shoelace(&locations);
    let b = points.len();
    let i = picks(area, b);
    (i + (b as f64)) as i64
}

pub fn solve_part_two(input: &str) -> i64 {
    let input = input
        .lines()
        .map(|line| {
            let parts = line.split(" ").collect_vec();
            let color = parts[2].to_string();
            let color = color[2..color.len() - 1].to_string();
            //convert color from hex to a u64
            println!("The color is {}", color);
            let color_number = i64::from_str_radix(&color, 16).unwrap();
            let last_char_in_last_part = color.chars().last().unwrap();
            let last_char_number = last_char_in_last_part.to_digit(10).unwrap();
            let direction = match last_char_number {
                0 => Direction::Right(color_number),
                1 => Direction::Up(color_number),
                2 => Direction::Left(color_number),
                3 => Direction::Down(color_number),
                _ => panic!("Unknown direction"),
            };
            direction
        })
        .collect_vec();
    solve_inner(input)
}

fn parse_input(input: &str) -> Vec<(Direction, String)> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(" ").collect();
            let direction = match parts[0] {
                "L" => Direction::Left(parts[1].parse().unwrap()),
                "R" => Direction::Right(parts[1].parse().unwrap()),
                "U" => Direction::Up(parts[1].parse().unwrap()),
                "D" => Direction::Down(parts[1].parse().unwrap()),
                _ => panic!("Unknown direction"),
            };
            let color = parts[2].to_string();
            let color = color[1..color.len() - 1].to_string();
            (direction, color)
        })
        .collect_vec()
}

#[cfg(test)]
mod tests {
    #[test]
    fn solves_18_1_hard() {
        let input = std::fs::read_to_string("input/18_real.txt").unwrap();
        assert_eq!(super::solve_part_one(&input), 53844);
    }

    #[test]
    fn solves_18_2_easy() {
        let input = std::fs::read_to_string("input/18_easy.txt").unwrap();
        assert_eq!(super::solve_part_two(&input), 42);
    }

    #[test]
    fn solves_18_2_hard() {
        let input = std::fs::read_to_string("input/18_real.txt").unwrap();
        assert_eq!(super::solve_part_two(&input), 42);
    }
}
