advent_of_code::solution!(7);

fn propagate_beam(grid: &mut Vec<Vec<char>>, row: usize, col: usize, splits: &mut u64) {
    let rows = grid.len();
    let cols = grid[0].len();
    if row >= rows {
        return;
    }
    match grid[row][col] {
        '.' => {
            grid[row][col] = '|';
            propagate_beam(grid, row + 1, col, splits);
        }
        '^' => {
            *splits += 1;
            // split left
            if col > 0 {
                propagate_beam(grid, row + 1, col - 1, splits);
            }
            // split right
            if col + 1 < cols {
                propagate_beam(grid, row + 1, col + 1, splits);
            }
        }
        _ => {}
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    // read into a 2d grid
    // start from the position lablled 's'
    // extend a beam downwards with '|'
    // when reaching a splitter '^', start new beams to the left and right
    // terminate when reaching the end of the grid
    // count the number of times the beam splits
    // return the count

    let mut result: u64 = 0;
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let start_pos = grid
        .iter()
        .enumerate()
        .find_map(|(r, row)| row.iter().position(|&c| c == 'S').map(|c| (r, c)))?;
    let mut grid_copy = grid.clone();
    propagate_beam(&mut grid_copy, start_pos.0 + 1, start_pos.1, &mut result);
    // println!("Final grid:");
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let cols = grid[0].len();
    let start_pos = grid
        .iter()
        .enumerate()
        .find_map(|(r, row)| row.iter().position(|&c| c == 'S').map(|c| (r, c)))?;
    // create an array of length num_cols
    let mut beam_paths: Vec<u64> = vec![0; cols];
    beam_paths[start_pos.1] = 1;
    for row in grid {
        for (j, col) in row.iter().enumerate() {
            if *col == '^' {
                if j > 0 {
                    beam_paths[j - 1] += beam_paths[j];
                }
                if j + 1 < cols {
                    beam_paths[j + 1] += beam_paths[j];
                }
                beam_paths[j] = 0;
            }
        }
    }
    let result = beam_paths.iter().sum();
    // println!("Final grid:");
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
