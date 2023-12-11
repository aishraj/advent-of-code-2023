pub fn solve(input: &str, expansion: usize) -> usize {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut galaxies = Vec::new();
    for (row, line) in grid.iter().enumerate() {
        for (col, &cell) in line.iter().enumerate() {
            if cell == '#' {
                galaxies.push((row, col));
            }
        }
    }

    expand(&mut grid, &mut galaxies, expansion);
    let mut total = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let (r1, c1) = galaxies[i];
            let (r2, c2) = galaxies[j];
            total += (r1 as i64 - r2 as i64).abs() + (c1 as i64 - c2 as i64).abs();
        }
    }
    total as usize
}

pub fn solve_part_one(input: &str) -> usize {
    solve(input, 2)
}

pub fn solve_part_two(input: &str) -> usize {
    solve(input, 1000000)
}

fn expand(grid: &mut Vec<Vec<char>>, galaxies: &mut Vec<(usize, usize)>, expansion: usize) {
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

    for r in no_galaxy_row {
        for g in &mut *galaxies {
            if g.0 > r {
                g.0 += expansion - 1
            }
        }
    }
    for c in no_galaxy_col {
        for g in &mut *galaxies {
            if g.1 > c {
                g.1 += expansion - 1
            }
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
        assert_eq!(super::solve_part_one(&input), 9312968);
    }

    #[test]
    fn solves_11_2_easy() {
        let input = std::fs::read_to_string("input/11_easy.txt").unwrap();
        assert_eq!(super::solve(&input, 100), 8410);
    }

    #[test]
    fn solves_11_2_hard() {
        let input = std::fs::read_to_string("input/11_hard.txt").unwrap();
        assert_eq!(super::solve_part_two(&input), 597714117556);
    }
}
