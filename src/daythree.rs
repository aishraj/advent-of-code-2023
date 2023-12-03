use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};

pub struct Graph(Vec<Vec<char>>);

impl FromStr for Graph {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut graph = vec![];
        for line in s.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c);
            }
            graph.push(row);
        }
        return Ok(Graph(graph));
    }
}

impl Graph {
    pub fn search(&self) -> Graph {
        // we do a search from all symbols and mark any numbers we find as 'visited'
        let mut output = self.0.clone();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        for (i, row) in self.0.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                if *c != '.' && !c.is_numeric() {
                    queue.push_back((i, j));
                }
            }
        }

        let directions = vec![
            (0, 1),
            (0, -1),
            (1, 0),
            (-1, 0),
            (1, 1),
            (-1, -1),
            (1, -1),
            (-1, 1),
        ];
        while let Some((i, j)) = queue.pop_front() {
            let c = self.0[i][j];
            println!("x y c - {} {} {}", i, j, c);
            visited.insert((i, j));
            for (dx, dy) in directions.iter() {
                let nx = i as i32 + dx;
                let ny = j as i32 + dy;
                if nx < 0 || ny < 0 {
                    continue;
                }
                let nx = nx as usize;
                let ny = ny as usize;
                if ny >= self.0[0].len() || nx >= self.0.len() {
                    continue;
                }
                let c = self.0[nx][ny];
                if c.is_numeric() || (!c.is_numeric() && c != '.') {
                    if visited.contains(&(nx, ny)) {
                        continue;
                    }
                    println!("nx: {} ny: {} c: {}", nx, ny, c);
                    queue.push_back((nx, ny));
                }
            }
            println!("\n\n");
        }

        for i in 0..self.0.len() {
            for j in 0..self.0[0].len() {
                let c = self.0[i][j];
                if c.is_numeric() && !visited.contains(&(i, j)) {
                    output[i][j] = '.';
                }
            }
        }
        return Graph(output);
    }

    pub fn extract_numbers(&self) -> Vec<u32> {
        let mut numbers = vec![];
        for row in self.0.iter() {
            let mut current_number = String::new();
            for c in row.iter() {
                if c.is_numeric() {
                    current_number.push(*c);
                } else if !current_number.is_empty() {
                    numbers.push(current_number.parse::<u32>().unwrap());
                    current_number = String::new();
                }
            }
            if !current_number.is_empty() {
                numbers.push(current_number.parse::<u32>().unwrap());
            }
        }
        return numbers;
    }
}

pub fn solve_part_one(input: &str) -> u32 {
    let graph = Graph::from_str(input).unwrap();
    let graph = graph.search();
    let mut numbers = graph.extract_numbers();
    numbers.sort();
    println!("The numbers are: {:?}", numbers);
    return numbers.iter().sum();
}

pub fn solve_part_two(input: &str) -> u32 {
    42
}

fn parse_input(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn solves_3_1_easy() {
        let input = std::fs::read_to_string("input/3_easy.txt").unwrap();
        let expected = 4361;
        let actual = super::solve_part_one(&input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn solves_3_1_hard() {
        let input = std::fs::read_to_string("input/3_real.txt").unwrap();
        let actual = super::solve_part_one(&input);
        assert_eq!(actual, 522726);
    }

    #[test]
    fn solves_3_2_easy() {
        let input = std::fs::read_to_string("input/3_easy.txt").unwrap();
        assert_eq!(super::solve_part_two(&input), 42);
    }

    #[test]
    fn solves_3_2_hard() {
        let input = std::fs::read_to_string("input/3_real.txt").unwrap();
        assert_eq!(super::solve_part_two(&input), 42);
    }
}
