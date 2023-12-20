use std::collections::VecDeque;

use itertools::Itertools;

enum Direction {
    Left(i64),
    Right(i64),
    Up(i64),
    Down(i64),
}

pub fn solve_part_one(input: &str) -> u32 {
    let input = parse_input(input);
    let mut grid = vec![vec!['.'; 10000]; 10000];
    let mut cur_x: i64 = 5000;
    let mut cur_y: i64 = 5000;
    let _point = (cur_x, cur_y);
    let mut wall_count = 0;
    for (direction, _) in input.into_iter() {
        match direction {
            Direction::Down(amount) => {
                for i in (cur_x + 1)..=(cur_x + amount) {
                    grid[i as usize][cur_y as usize] = '#';
                    wall_count += 1;
                }
                cur_x += amount;
            }
            Direction::Up(amount) => {
                for i in ((cur_x - amount)..cur_x).rev() {
                    println!("Setting ({}, {}) to #", i, cur_y);
                    grid[i as usize][cur_y as usize] = '#';

                    wall_count += 1;
                }
                cur_x -= amount;
            }
            Direction::Left(amount) => {
                for i in (((cur_y as i64 - amount) as usize)..cur_y as usize).rev() {
                    grid[cur_x as usize][i as usize] = '#';
                    wall_count += 1;
                }
                cur_y -= amount;
            }
            Direction::Right(amount) => {
                for i in (cur_y + 1)..=(cur_y + amount) {
                    grid[cur_x as usize][i as usize] = '#';
                    wall_count += 1;
                }
                cur_y += amount;
            }
        }
    }
    // Print the grid
    for line in &grid {
        for c in line {
            print!("{}", c);
        }
        println!();
    }
    println!("Wall count: {}", wall_count);

    // from curx, cury look for a point that is not a wall
    // check all 8 directions
    let directions: Vec<(i64, i64)> = vec![
        (0, 1),
        (1, 0),
        (-1, 0),
        (0, -1),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];
    let m = grid.len();
    let n = grid[0].len();

    let (mut inx, mut iny) = (cur_x, cur_y);
    for direction in directions {
        let (nx, ny) = ((cur_x as i64 + direction.0), (cur_y as i64 + direction.1));
        if nx < 0 || ny < 0 || nx >= m as i64 || ny >= n as i64 {
            continue;
        }
        if grid[nx as usize][ny as usize] == '#' {
            continue;
        }
        inx = nx as i64;
        iny = ny as i64;
        break;
    }
    println!("Found entrance at ({}, {})", inx, iny);

    // Run flood fill from the entrance
    let mut visited = vec![vec![false; 10000]; 10000];
    let mut queue = VecDeque::new();
    queue.push_back((inx, iny));
    let mut count = 0;
    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();
        if visited[x as usize][y as usize] {
            continue;
        }
        visited[x as usize][y as usize] = true;
        count += 1;
        let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        for direction in directions {
            let (nx, ny) = ((x as i64 + direction.0), (y as i64 + direction.1));
            if nx < 0 || ny < 0 || nx >= m as i64 || ny >= n as i64 {
                continue;
            }
            if grid[nx as usize][ny as usize] == '#' {
                continue;
            }
            queue.push_back((nx, ny));
        }
    }
    println!("Counted {} points", count);
    println!("Total count: {}", count + wall_count);
    count + wall_count
}

pub fn solve_part_two(_input: &str) -> u32 {
    42
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
