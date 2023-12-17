use std::collections::BTreeSet;

use itertools::Itertools;

pub fn solve_part_one(input: &str) -> usize {
    let grid = parse_input(input);
    let result = dijkstras_with_step_bound(&grid, 0, 3);
    result.unwrap()
}

pub fn solve_part_two(input: &str) -> usize {
    let grid = parse_input(input);
    let result = dijkstras_with_step_bound(&grid, 4, 10);
    result.unwrap()
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct State {
    cost: usize,
    position: (usize, usize),
    direction: (i32, i32),
    steps: usize,
}

impl State {
    fn new(cost: usize, position: (usize, usize), direction: (i32, i32), steps: usize) -> Self {
        Self {
            cost,
            position,
            direction,
            steps,
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
            .then_with(|| self.direction.cmp(&other.direction))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Returns the shortest path from start to end in the grid.
/// In doing so it ensures that you have only moved in a given
/// direction for the number of steps between step_bound.start and step_bound.end
/// (inclusive).
/// If no path is found, returns None.
fn dijkstras_with_step_bound(
    grid: &Vec<Vec<usize>>,
    min_steps: usize,
    max_steps: usize,
) -> Option<usize> {
    let mut queue = std::collections::BinaryHeap::new();
    // keep track of (position, direction, steps)
    let mut visited: BTreeSet<((usize, usize), (i32, i32), usize)> =
        std::collections::BTreeSet::new();
    queue.push(State::new(0, (0, 0), (0, 1), 0));
    let direction_unit_vectors = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    while let Some(State {
        cost,
        position,
        direction,
        steps,
    }) = queue.pop()
    {
        if position == (grid.len() - 1, grid[0].len() - 1) {
            return Some(cost);
        }
        if visited.contains(&(position, direction, steps)) {
            continue;
        }
        visited.insert((position, direction, steps));
        for (_i, unit_vector) in direction_unit_vectors.iter().enumerate() {
            let new_direction = *unit_vector;

            // don't go back the way we came
            if new_direction == (-direction.0, -direction.1) {
                continue;
            }

            let new_position = (
                position.0 as i32 + new_direction.0,
                position.1 as i32 + new_direction.1,
            );
            if new_position.0 < 0
                || new_position.1 < 0
                || new_position.0 >= grid.len() as i32
                || new_position.1 >= grid[0].len() as i32
            {
                continue;
            }
            let new_position = (new_position.0 as usize, new_position.1 as usize);
            let new_cost = cost + grid[new_position.0][new_position.1];
            let new_steps = if new_direction == direction {
                steps + 1
            } else {
                1
            };
            if new_steps >= min_steps && new_steps <= max_steps {
                queue.push(State::new(new_cost, new_position, new_direction, new_steps));
            }
        }
    }
    None
}

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect_vec()
        })
        .collect_vec()
}

#[cfg(test)]
mod tests {
    #[test]
    fn solves_17_1_easy() {
        let input = std::fs::read_to_string("input/17_easy.txt").unwrap();
        assert_eq!(super::solve_part_one(&input), 102);
    }

    #[test]
    fn solves_17_1_hard() {
        let input = std::fs::read_to_string("input/17_real.txt").unwrap();
        assert_eq!(super::solve_part_one(&input), 42);
    }

    #[test]
    fn solves_17_2_easy() {
        let input = std::fs::read_to_string("input/17_easy.txt").unwrap();
        assert_eq!(super::solve_part_two(&input), 42);
    }

    #[test]
    fn solves_17_2_hard() {
        let input = std::fs::read_to_string("input/17_real.txt").unwrap();
        assert_eq!(super::solve_part_two(&input), 42);
    }
}
