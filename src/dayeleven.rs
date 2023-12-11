pub fn solve_part_one(input: &str) -> usize {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    expand(&mut grid);
    let mut galaxies = Vec::new();
    for (row, line) in grid.iter().enumerate() {
        for (col, &cell) in line.iter().enumerate() {
            if cell == '#' {
                galaxies.push((row, col));
            }
        }
    }
    let mut total = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let (r1, c1) = galaxies[i];
            let (r2, c2) = galaxies[j];
            total += (r1 as i32 - r2 as i32).abs() + (c1 as i32 - c2 as i32).abs();
        }
    }
    total as usize
}

pub fn solve_part_two(input: &str) -> u32 {
    42
}

fn expand(grid: &mut Vec<Vec<char>>) {
    let length = grid.len();
    let width = grid[0].len();

    let mut no_galaxy_row = Vec::new();
    let mut no_galaxy_col = Vec::new();

    for i in 0..length {
        if !grid[i].contains(&'#') {
            no_galaxy_row.push(i);
        }
    }

    for i in 0..width {
        let to_check: Vec<char> = grid.iter().map(|row| row[i]).collect();
        if !to_check.contains(&'#') {
            no_galaxy_col.push(i);
        }
    }

    no_galaxy_row.reverse();
    no_galaxy_col.reverse();

    for &index in &no_galaxy_row {
        let new_row = vec!['.'; width];
        grid.insert(index, new_row);
    }

    let new_length = grid.len();
    for &index in &no_galaxy_col {
        for row in grid.iter_mut().take(new_length) {
            row.insert(index, '.');
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn solves_11_1_easy() {
        let input = std::fs::read_to_string("input/11_easy.txt").unwrap();
        assert_eq!(super::solve_part_one(&input), 374);
    }

    #[test]
    fn solves_11_1_hard() {
        let input = std::fs::read_to_string("input/11_hard.txt").unwrap();
        assert_eq!(super::solve_part_one(&input), 42);
    }

    #[test]
    fn solves_11_2_easy() {
        let input = std::fs::read_to_string("input/11_easy.txt").unwrap();
        assert_eq!(super::solve_part_one(&input), 42);
    }

    #[test]
    fn solves_11_2_hard() {
        let input = std::fs::read_to_string("input/11_hard.txt").unwrap();
        assert_eq!(super::solve_part_one(&input), 42);
    }
}
