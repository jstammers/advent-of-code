advent_of_code::solution!(2);
fn split_ranges(input: &str) -> Vec<(u64, u64)> {
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    // split by , then by -
    // remove newlines

    let input = input.replace('\n', "");
    for line in input.split(',') {
        let parts: Vec<&str> = line.split('-').collect();
        if parts.len() != 2 {
            continue;
        }
        let start: u64 = parts[0].parse().unwrap();
        let end: u64 = parts[1].parse().unwrap();
        ranges.push((start, end));
    }
    ranges
}

fn is_invalid(x: i64) -> bool {
    // invalid if the string representation contains a sequence of digits repeated twice

    let s = x.to_string();
    let len = s.len();
    let start_string = s[0..len / 2].to_string();
    let end_string = s[len / 2..len].to_string();
    return start_string == end_string;
}

fn is_invalid_all(x: i64) -> bool {
    // invalid if the string representation contains a sequence of digits repeated at least twice

    let s = x.to_string();
    let len = s.len();

    // Try all possible block sizes that divide the full length
    for block_size in 1..=len / 2 {
        if len % block_size != 0 {
            continue;
        }
        let block = &s[0..block_size];
        if block.repeat(len / block_size) == s {
            return true;
        }
    }

    false
}

fn count_invalid(start: u64, end: u64) -> (u64, u64) {
    let mut count: u64 = 0;
    let mut total: u64 = 0;
    for x in start..=end {
        if is_invalid(x as i64) {
            count += 1;
            total += x;
        }
    }
    (count, total)
}

fn count_all_invalid(start: u64, end: u64) -> (u64, u64) {
    let mut count: u64 = 0;
    let mut total: u64 = 0;
    for x in start..=end {
        if is_invalid_all(x as i64) {
            count += 1;
            total += x;
        }
    }
    (count, total)
}
pub fn part_one(input: &str) -> Option<u64> {
    let ranges = split_ranges(input);
    let mut _count: u64 = 0;
    let mut total: u64 = 0;
    for (start, end) in ranges {
        let (c, t) = count_invalid(start, end);
        _count += c;
        total += t;
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges = split_ranges(input);
    let mut _count: u64 = 0;
    let mut total: u64 = 0;
    for (start, end) in ranges {
        let (c, t) = count_all_invalid(start, end);
        _count += c;
        total += t;
    }
    if total == 0 { None } else { Some(total) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(count_invalid(11, 22), (2, 33));

        // * `11-22` has two invalid IDs, `*11*` and `*22*`.
        // * `95-115` has one invalid ID, `*99*`.
        // * `998-1012` has one invalid ID, `*1010*`.
        // * `1188511880-1188511890` has one invalid ID, `*1188511885*`.
        // * `222220-222224` has one invalid ID, `*222222*`.
        // * `1698522-1698528` contains no invalid IDs.
        // * `446443-446449` has one invalid ID, `*446446*`.
        // * `38593856-38593862` has one invalid ID, `*38593859*`.

        assert_eq!(count_invalid(95, 115), (1, 99));
        assert_eq!(count_invalid(998, 1012), (1, 1010));
        assert_eq!(count_invalid(1188511880, 1188511890), (1, 1188511885));
        assert_eq!(count_invalid(222220, 222224), (1, 222222));
        assert_eq!(count_invalid(1698522, 1698528), (0, 0));
        assert_eq!(count_invalid(446443, 446449), (1, 446446));
        assert_eq!(count_invalid(38593856, 38593862), (1, 38593859));

        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        // * `11-22` still has two invalid IDs, `*11*` and `*22*`.
        // * `95-115` now has two invalid IDs, `*99*` and `*111*`.
        // * `998-1012` now has two invalid IDs, `*999*` and `*1010*`.
        // * `1188511880-1188511890` still has one invalid ID, `*1188511885*`.
        // * `222220-222224` still has one invalid ID, `*222222*`.
        // * `1698522-1698528` still contains no invalid IDs.
        // * `446443-446449` still has one invalid ID, `*446446*`.
        // * `38593856-38593862` still has one invalid ID, `*38593859*`.
        // * `565653-565659` now has one invalid ID, `*565656*`.
        // * `824824821-824824827` now has one invalid ID, `*824824824*`.
        // * `2121212118-2121212124` now has one invalid ID, `*2121212121*`.
        assert_eq!(count_all_invalid(11, 22), (2, 33));
        assert_eq!(count_all_invalid(95, 115), (2, 210));
        assert_eq!(count_all_invalid(998, 1012), (2, 2009));
        assert_eq!(count_all_invalid(1188511880, 1188511890), (1, 1188511885));
        assert_eq!(count_all_invalid(222220, 222224), (1, 222222));
        assert_eq!(count_all_invalid(1698522, 1698528), (0, 0));
        assert_eq!(count_all_invalid(446443, 446449), (1, 446446));
        assert_eq!(count_all_invalid(38593856, 38593862), (1, 38593859));
        assert_eq!(count_all_invalid(565653, 565659), (1, 565656));
        assert_eq!(count_all_invalid(824824821, 824824827), (1, 824824824));
        assert_eq!(count_all_invalid(2121212118, 2121212124), (1, 2121212121));

        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
