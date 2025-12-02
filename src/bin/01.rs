advent_of_code::solution!(1);

pub fn parse_line(input: &str, x: i64, m: i64) -> (i64, i64) {
    let result: i64;
    // first letter is L or R
    // then the rest are digits
    // if L, x = x - d
    // if R, x = x + d
    // finally, return x % m

    let (dir, dist) = input.split_at(1);
    let d: i64 = dist.parse().unwrap();
    let result_unreduced: i64 = match dir {
        "L" => x - d,
        "R" => x + d,
        _ => panic!("Invalid direction"),
    };

    let q = result_unreduced.div_euclid(m);
    let e = x.div_euclid(m);
    let mut num_rotations: i64 = (q - e).abs();
    result = result_unreduced.rem_euclid(m);
    if (dir == "L") && (x == 0) {
        // moving left and landed on zero means we completed an extra rotation
        num_rotations -= 1;
    }
    if (dir == "L") && (result == 0) {
        // moving left from zero means we completed one less rotation
        num_rotations += 1;
    }
    // if ((x == 0)) && (num_rotations > 1) {
    //     num_rotations -= 1;
    // }
    return (result, num_rotations);
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();
    let m: i64 = 100;
    let mut x: i64 = 50;
    let mut num_zeros: u64 = 0;
    let mut _d: i64 = 0;
    for line in lines {
        (x, _d) = parse_line(line, x, m);
        if x == 0 {
            num_zeros += 1;
        }
    }
    return Some(num_zeros as u64);
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();
    let m: i64 = 100;
    let mut x: i64 = 50;
    let mut d: i64;
    let mut num_zeros: u64 = 0;
    for line in lines {
        (x, d) = parse_line(line, x, m);
        // if x == 0 {
        //     num_zeros += 1 as u64;
        // }
        num_zeros += d as u64;
        // println!("Line: {}, x: {}, d: {}", line, x, d);
    }
    return Some(num_zeros as u64);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(parse_line("L100", 0, 100), (0, 1));

        assert_eq!(parse_line("R100", 0, 100), (0, 1));

        let result = parse_line("L150", 50, 100);
        assert_eq!(result, (0, 2));

        let result = parse_line("R150", 50, 100);
        assert_eq!(result, (0, 2));

        assert_eq!(parse_line("L50", 50, 100), (0, 1)); //failing

        let result = parse_line("R50", 50, 100);
        assert_eq!(result, (0, 1));

        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));

        let result2 = part_two("R50\n");
        assert_eq!(result2, Some(1));

        let result3 = part_two("L50\n");
        assert_eq!(result3, Some(1));
    }
}
