use itertools::Itertools;
use std::collections::HashMap;

/// Direction is one of the four possible directions that you can travel in the maze from the current position.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
    Anywhere,
}

// Returns the mapping from position to the two directions that you can travel from that position.
fn parse_part_one(
    input: &str,
) -> (
    (usize, usize),
    HashMap<(usize, usize), Vec<Direction>>,
    Vec<Vec<char>>,
) {
    let mut mapping = HashMap::new();
    let raw_input: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut start = (0, 0);
    for i in 0..raw_input.len() {
        for j in 0..(raw_input[0]).len() {
            let c = raw_input[i][j];
            let pair = match c {
                '|' => ((i, j), vec![Direction::North, Direction::South]),
                '-' => ((i, j), vec![Direction::East, Direction::West]),
                'L' => ((i, j), vec![Direction::North, Direction::East]),
                'J' => ((i, j), vec![Direction::North, Direction::West]),
                '7' => ((i, j), vec![Direction::South, Direction::West]),
                'F' => ((i, j), vec![Direction::South, Direction::East]),
                '.' => ((i, j), vec![]),
                'S' => {
                    start = (i, j);
                    ((i, j), vec![Direction::Anywhere])
                }
                _ => panic!("Invalid character in input: {}", c),
            };
            mapping.insert(pair.0, pair.1);
        }
    }
    return (start, mapping, raw_input);
}

// Assign the start pipe one of the four directions.
fn assign_start(
    start: (usize, usize),
    bounds: (usize, usize),
    mapping: &mut HashMap<(usize, usize), Vec<Direction>>,
) {
    let (startx, starty) = start;
    let mut possible_directions = vec![];

    // check above me
    if startx > 0
        && mapping
            .get(&(startx - 1, starty))
            .unwrap()
            .contains(&Direction::South)
    {
        possible_directions.push(Direction::North);
    }
    // check below me
    if startx < bounds.0 - 1
        && mapping
            .get(&(startx + 1, starty))
            .unwrap()
            .contains(&Direction::North)
    {
        possible_directions.push(Direction::South);
    }
    // Check the one on my left
    if starty > 0
        && mapping
            .get(&(startx, starty - 1))
            .unwrap()
            .contains(&Direction::East)
    {
        // could have come west to me
        possible_directions.push(Direction::West);
    }
    // check on my right
    if starty < bounds.1 - 1
        && mapping
            .get(&(startx, starty + 1))
            .unwrap()
            .contains(&Direction::West)
    {
        possible_directions.push(Direction::East);
    }
    mapping.insert(start, possible_directions);
}

pub fn solve_part_one(input: &str) -> u32 {
    // I just need to simulate the run
    let (start, mut mapping, graph) = parse_part_one(input);
    let bounds = (graph.len(), graph[0].len());
    assign_start(start, bounds, &mut mapping);
    //println!("the mapping is {:?}", mapping.get(&start).unwrap());
    let (mut curx, mut cury) = start;
    let mut path = vec![];
    let mut last_direction = Direction::Anywhere;
    let dirs = vec![
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];
    loop {
        let destination = *dirs
            .iter()
            .filter(|d| *d != &last_direction && mapping.get(&(curx, cury)).unwrap().contains(&d))
            .collect_vec()
            .first()
            .expect("There should be at least one direction to go in");
        match destination {
            Direction::North => curx -= 1,
            Direction::South => curx += 1,
            Direction::East => cury += 1,
            Direction::West => cury -= 1,
            _ => panic!("Invalid direction"),
        }
        path.push(((curx, cury), destination.clone()));
        if (curx, cury) == start {
            break;
        }
        last_direction = match destination {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            _ => panic!("Invalid direction"),
        };
    }
    //println!("{:?}", path);
    return path.len() as u32 / 2;
}

pub fn solve_part_two(_input: &str) -> u32 {
    42
}

/// Parses the input into a list of positions and their entrace/exit pairs.

#[cfg(test)]
mod tests {
    #[test]
    fn solves_10_1_easy() {
        let input = std::fs::read_to_string("input/10_easy.txt").unwrap();
        assert_eq!(super::solve_part_one(&input), 4);
    }

    #[test]
    fn solves_10_1_hard() {
        let input = std::fs::read_to_string("input/10_real.txt").unwrap();
        assert_eq!(super::solve_part_one(&input), 6907);
    }

    #[test]
    fn solves_10_2_easy() {
        let input = std::fs::read_to_string("input/10_easy.txt").unwrap();
        assert_eq!(super::solve_part_two(&input), 42);
    }

    #[test]
    fn solves_10_2_hard() {
        let input = std::fs::read_to_string("input/10_real.txt").unwrap();
        assert_eq!(super::solve_part_two(&input), 42);
    }
}
