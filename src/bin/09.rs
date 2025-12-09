advent_of_code::solution!(9);

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    // Parse input into a vector of (x, y) coordinate tuples
    input
        .lines()
        .filter_map(|line| {
            let mut parts = line.split(',');
            if let (Some(x_str), Some(y_str)) = (parts.next(), parts.next()) {
                if let (Ok(x), Ok(y)) = (x_str.trim().parse::<u64>(), y_str.trim().parse::<u64>()) {
                    return Some((x, y));
                }
            }
            None
        })
        .collect()
}

fn area(x1: u64, y1: u64, x2: u64, y2: u64) -> u64 {
    let width = x1.abs_diff(x2);
    let height = y1.abs_diff(y2);
    (width + 1) * (height + 1)
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut coords = parse_input(input);
    // sort in increasing order by x, then by y
    coords.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
    // find the two points that maximize the area of the rectangle formed by them
    let mut max_area = 0;
    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            // println!("{:?}, {:?}", coords[i], coords[j]);
            let area_ij = area(coords[i].0, coords[i].1, coords[j].0, coords[j].1);
            if area_ij > max_area {
                max_area = area_ij;
            }
        }
    }
    Some(max_area)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut coords = parse_input(input);
    let mut edges: Vec<(&(u64, u64), &(u64, u64))> =
        coords.windows(2).map(|v| (&v[0], &v[1])).collect();

    // Add final closing edge
    edges.push((&coords[coords.len() - 1], &coords[0]));

    let mut possible_rects: Vec<((u64, u64), (u64, u64), u64)> = coords
        .iter()
        .enumerate()
        .flat_map(|(i, &p1)| {
            coords
                .iter()
                .enumerate()
                .skip(i + 1)
                .map(move |(_, &p2)| (p1, p2, area(p1.0, p1.1, p2.0, p2.1)))
        })
        .collect();
    possible_rects.sort_by_key(|(_, _, area)| *area);
    let max_area = possible_rects
        .into_iter()
        .rev()
        .find(|(p1, p2, area)| {
            edges.iter().all(|(start, end)| {
                let before = p1.0.max(p2.0) <= start.0.min(end.0);
                let after = p1.0.min(p2.0) >= start.0.max(end.0);
                let above = p1.1.min(p2.1) >= start.1.max(end.1);
                let below = p1.1.max(p2.1) <= start.1.min(end.1);
                before || after || above || below
            })
        })
        .expect("At least the smallest rectangle should fit")
        .2;
    Some(max_area)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
