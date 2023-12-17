use std::{
    cmp,
    collections::{HashSet, VecDeque},
};

use itertools::Itertools;

type Position = (i32, i32);
type Direction = (i32, i32);

fn bfs(grid: &[Vec<char>], start_pos: Position, start_dir: Direction) -> usize {
    let m = grid.len() as i32;
    let n = grid[0].len() as i32;
    let mut seen = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back((start_pos, start_dir));
    let mut visited_grid = vec![vec!['.'; n as usize]; m as usize];
    while let Some(((i, j), (dx, dy))) = q.pop_front() {
        let newx = i + dx;
        let newy = j + dy;
        if seen.contains(&(newx, newy, dx, dy)) || newx < 0 || newy < 0 || newx >= m || newy >= n {
            continue;
        }
        seen.insert((newx, newy, dx, dy));
        visited_grid[newx as usize][newy as usize] = '#';
        let tile = grid[newx as usize][newy as usize];
        let (new_dx, new_dy) = match tile {
            '/' => (-dy, -dx),
            '\\' => (dy, dx),
            '|' => {
                if dy != 0 {
                    (1, 0)
                } else {
                    (dx, dy)
                }
            }
            '-' => {
                if dx != 0 {
                    (0, 1)
                } else {
                    (dx, dy)
                }
            }
            _ => (dx, dy),
        };
        if tile == '|' && dy != 0 {
            q.push_back(((newx, newy), (-1, 0)));
        }
        if tile == '-' && dx != 0 {
            q.push_back(((newx, newy), (0, -1)));
        }
        q.push_back(((newx, newy), (new_dx, new_dy)));
    }
    visited_grid
        .iter()
        .for_each(|line| println!("{}", line.iter().collect::<String>()));
    println!("");
    let hash_count = visited_grid.iter().flatten().filter(|&&c| c == '#').count();
    return hash_count;
}

fn solve(data: Vec<String>) -> usize {
    let grid: Vec<Vec<char>> = data.iter().map(|line| line.chars().collect()).collect();
    let p1 = bfs(&grid, (0, -1), (0, 1));
    return p1;
}

pub fn solve_part_two(input: &str) -> usize {
    let input = input.lines().map(|line| line.to_string()).collect_vec();
    let input = input
        .iter()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let m = input.len();
    let n = input[0].len();
    let mut res = 0;
    for i in 0..m {
        res = cmp::max(res, bfs(&input, (i.try_into().unwrap(), -1), (0, 1)));
        res = cmp::max(res, bfs(&input, (i.try_into().unwrap(), n as i32), (0, -1)));
    }
    for j in 0..n {
        res = cmp::max(res, bfs(&input, (-1, j.try_into().unwrap()), (1, 0)));
        res = cmp::max(res, bfs(&input, (m as i32, j.try_into().unwrap()), (-1, 0)));
    }
    return res;
}

pub fn solve_part_one(input: &str) -> usize {
    let input = input.lines().map(|line| line.to_string()).collect();
    solve(input)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn solves_16_1_easy() {
        let input = std::fs::read_to_string("input/16_easy.txt").unwrap();
        assert_eq!(solve_part_one(&input), 46);
    }

    #[test]
    fn solves_16_2_easy() {
        let input = std::fs::read_to_string("input/16_easy.txt").unwrap();
        assert_eq!(solve_part_two(&input), 51);
    }

    #[test]
    fn solves_16_2_real() {
        let input = std::fs::read_to_string("input/16_real.txt").unwrap();
        assert_eq!(solve_part_two(&input), 8183);
    }

    #[test]
    fn solves_16_1_real() {
        let input = std::fs::read_to_string("input/16_real.txt").unwrap();
        assert_eq!(solve_part_one(&input), 7434);
    }
}
