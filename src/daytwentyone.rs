use std::collections::{BTreeSet, HashSet, VecDeque};

use itertools::Itertools;

pub fn solve_part_one(input: &str, max_generation: usize) -> usize {
    let grid = parse_input(input);
    let num_grid_reachable = perform_bfs(&grid, max_generation);
    num_grid_reachable
}

fn perform_bfs(grid: &[Vec<char>], max_steps: usize) -> usize {
    let (start_x, start_y) = grid
        .iter()
        .enumerate()
        .flat_map(|(x, row)| row.iter().enumerate().map(move |(y, &cell)| (x, y, cell)))
        .find(|&(_, _, cell)| cell == 'S')
        .map(|(x, y, _)| (x, y))
        .unwrap();

    let mut ans = HashSet::new();
    let mut seen = HashSet::new();
    seen.insert((start_x, start_y));
    let mut queue = VecDeque::new();
    queue.push_back((start_x, start_y, 0));

    while let Some((x, y, steps)) = queue.pop_front() {
        if steps % 2 == 0 {
            ans.insert((x, y));
        }
        if steps >= max_steps {
            continue;
        }

        let dirs = vec![(0, 1), (1, 0), (-1, 0), (0, -1)];
        let valid_neighbors = dirs
            .iter()
            .filter_map(|(dx, dy)| {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && nx < grid.len() as i32 && ny >= 0 && ny < grid[0].len() as i32 {
                    Some((nx as usize, ny as usize))
                } else {
                    None
                }
            })
            .filter(|(nx, ny)| grid[*nx][*ny] != '#' && !seen.contains(&(*nx, *ny)))
            .collect_vec();
        for (nx, ny) in valid_neighbors {
            seen.insert((nx, ny));
            queue.push_back((nx, ny, steps + 1));
        }
    }

    ans.len()
}

fn find_start(grid: &[Vec<char>]) -> Option<(usize, usize)> {
    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == 'S' {
                return Some((x, y));
            }
        }
    }
    None
}

pub fn solve_part_two(input: &str) -> u32 {
    42
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn solves_21_1_easy() {
        let input = std::fs::read_to_string("input/21_easy.txt").unwrap();
        assert_eq!(super::solve_part_one(&input, 6), 16);
    }

    #[test]
    fn solves_21_1_hard() {
        let input = std::fs::read_to_string("input/21_real.txt").unwrap();
        assert_eq!(super::solve_part_one(&input, 64), 3820);
    }

    #[test]
    fn solves_21_2_easy() {
        let input = std::fs::read_to_string("input/21_easy.txt").unwrap();
        assert_eq!(super::solve_part_two(&input), 42);
    }

    #[test]
    fn solves_21_2_hard() {
        let input = std::fs::read_to_string("input/21_real.txt").unwrap();
        assert_eq!(super::solve_part_two(&input), 42);
    }
}
