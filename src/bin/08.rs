advent_of_code::solution!(8);
fn parse_input(_input: &str) -> Vec<(u64, u64, u64)> {
    // parse the input into a vector of x,y,z coordinates
    let mut coords = Vec::new();
    for line in _input.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 3 {
            continue;
        }
        let x: u64 = parts[0].parse().unwrap();
        let y: u64 = parts[1].parse().unwrap();
        let z: u64 = parts[2].parse().unwrap();
        coords.push((x, y, z));
    }
    coords
}

fn distance(a: (u64, u64, u64), b: (u64, u64, u64)) -> f64 {
    let dx = (a.0 as i64 - b.0 as i64) as f64;
    let dy = (a.1 as i64 - b.1 as i64) as f64;
    let dz = (a.2 as i64 - b.2 as i64) as f64;
    (dx * dx + dy * dy + dz * dz).sqrt()
}
pub fn part_one(input: &str) -> Option<u64> {
    let coords = parse_input(input);
    let circuits = Vec::new();
    n = 10;
    // find the two that are closest to each other
    
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
