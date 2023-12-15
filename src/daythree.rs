use std::{
    collections::{HashSet, VecDeque},
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
        Ok(Graph(graph))
    }
}

impl Graph {
    /// Returns "numbers" that are connected to the given position
    pub fn search_numbers_from_position(&self, start: (usize, usize)) -> Vec<u32> {
        let directions = [(0, 1),
            (0, -1),
            (1, 0),
            (-1, 0),
            (1, 1),
            (-1, -1),
            (1, -1),
            (-1, 1)];
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut numbers = vec![];
        queue.push_back(start);
        while let Some((i, j)) = queue.pop_front() {
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
                    queue.push_back((nx, ny));
                }
            }
        }
        for (i, row) in self.0.iter().enumerate() {
            let mut current_number = String::new();
            for (j, c) in row.iter().enumerate() {
                if c.is_numeric() && visited.contains(&(i, j)) {
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
        numbers
    }

    fn get_symbol_pos(&self, symbol: char) -> Vec<(usize, usize)> {
        let mut result = vec![];
        for (i, row) in self.0.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                if *c == symbol {
                    result.push((i, j));
                }
            }
        }
        println!("the symbol {} is at {:?}", symbol, result);
        result
    }

    pub fn search(&self) -> Vec<u32> {
        // we do a search from all symbols and mark any numbers we find as 'visited'
        let mut symbols = vec![];

        for (i, row) in self.0.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                if *c != '.' && !c.is_numeric() {
                    symbols.push((i, j));
                }
            }
        }

        let mut connected_numbers = vec![];
        for symbol_pos in symbols.into_iter() {
            let found_numbers = self.search_numbers_from_position(symbol_pos);
            connected_numbers.extend(found_numbers);
        }
        connected_numbers
    }
}

pub fn solve_part_one(input: &str) -> u32 {
    let graph = Graph::from_str(input).unwrap();
    let mut numbers = graph.search();
    numbers.sort();
    println!("The numbers are: {:?}", numbers);
    return numbers.iter().sum();
}

pub fn solve_part_two(input: &str) -> u32 {
    let graph = Graph::from_str(input).unwrap();
    let stars = graph.get_symbol_pos('*');
    let mut sum = 0;
    for star in stars {
        println!("star: {:?}", star);
        let numbers = graph.search_numbers_from_position(star);
        if numbers.len() < 2 {
            continue;
        }
        let prod = numbers.iter().product::<u32>();
        sum += prod;
    }
    sum
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
        assert_eq!(super::solve_part_two(&input), 467835);
    }

    #[test]
    fn solves_3_2_hard() {
        let input = std::fs::read_to_string("input/3_real.txt").unwrap();
        assert_eq!(super::solve_part_two(&input), 81721933);
    }
}
