advent_of_code::solution!(4);

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    grid
}

fn count_neighbors(grid: &Vec<Vec<char>>, row: usize, col: usize) -> u64 {
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut count = 0;
    for (dr, dc) in directions.iter() {
        let new_row = row as isize + dr;
        let new_col = col as isize + dc;
        if new_row >= 0
            && new_row < grid.len() as isize
            && new_col >= 0
            && new_col < grid[0].len() as isize
        {
            if grid[new_row as usize][new_col as usize] == '@' {
                count += 1;
            }
        }
    }
    count
}

fn recreate_graph(grid: &Vec<Vec<char>>, idx_to_remove: Vec<(u64, u64)>) -> Vec<Vec<char>> {
    let mut new_grid = grid.clone();
    for (row, col) in idx_to_remove {
        new_grid[row as usize][col as usize] = '.';
    }
    new_grid
}

fn count_all_neighbors(grid: &Vec<Vec<char>>) -> (i32, Vec<(u64, u64)>) {
    let mut idx_to_remove = Vec::new();
    let mut total_count: i32 = 0;
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            if cell == '@' {
                let c = count_neighbors(grid, row_idx, col_idx);
                if c < 4 {
                    idx_to_remove.push((row_idx as u64, col_idx as u64));
                    total_count += 1;
                }
            }
        }
    }
    return (total_count, idx_to_remove);
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_grid(input);
    let (count, _) = count_all_neighbors(&grid);
    Some(count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = parse_grid(input);
    let mut total_count: i32 = 0;

    loop {
        let (count, idx_to_remove) = count_all_neighbors(&grid);
        if count == 0 {
            break ;
        }
        total_count += count;
        grid = recreate_graph(&grid, idx_to_remove);
    }
    Some(total_count as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
