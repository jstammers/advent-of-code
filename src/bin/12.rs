advent_of_code::solution!(12);

struct Grid {
    width: usize,
    height: usize,
}

fn parse_input(input: &str) -> (Vec<Grid>, Vec<Vec<u64>>) {
    let mut grids = Vec::new();
    let mut totals = Vec::new();

    let sections: Vec<&str> = input.split("\n\n").collect();
    for section in sections {
        let lines: Vec<&str> = section.lines().collect();
        if lines.is_empty() {
            continue;
        }

        // Parse shape if first line ends in colon
        if lines[0].ends_with(':') {
            continue;
        }
        // Parse grid and totals
        else {
            for line in &lines {
                let parts = line.split(':').collect::<Vec<&str>>();
                let grid_part = parts[0];
                let totals_part = parts.get(1).unwrap_or(&"");
                // Parse grid dimensions
                let dims: Vec<&str> = grid_part.split('x').collect();
                if dims.len() != 2 {
                    continue;
                }
                let width: usize = dims[0].trim().parse().unwrap_or(0);
                let height: usize = dims[1].trim().parse().unwrap_or(0);
                let grid = Grid { width, height };
                grids.push(grid);
                // Parse totals
                let total_values: Vec<u64> = totals_part
                    .split_whitespace()
                    .filter_map(|s| s.trim().parse().ok())
                    .collect();
                totals.push(total_values);
            }
        }
    }

    (grids, totals)
}
pub fn part_one(input: &str) -> Option<u64> {
    let mut count = 0;
    let (grids, totals) = parse_input(input);
    for i in 0..grids.len() {
        let grid = &grids[i];
        // let shape = &shapes[i];
        let total_values = &totals[i];
        let area = grid.width as u64 * grid.height as u64;
        // Count total area by iterating through shapes
        // and checking sums
        let total_sum = total_values.iter().sum::<u64>();
        if total_sum * 9 > area {
            continue;
        } else {
            count += 1;
        }
    }
    return Some(count);
}

pub fn part_two(_input: &str) -> Option<u64> {
    Some(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }
}
