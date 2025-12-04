advent_of_code::solution!(3);

// pub fn largest_joltage(batteries: &str) -> u64 {
//     let chars: Vec<char> = batteries.chars().collect();

//     let mut l1: char = '0';
//     let mut l2: char = '0';
//     let mut max_joltage: u64 = 0;
//     for (index, c) in chars.iter().enumerate() {
//         if *c > l1 && index < chars.len() - 1 {
//             l2 = '0'; // reset l2 when we find a new l1
//             l1 = *c;
//         } else if *c > l2 {
//             l2 = *c;
//         }
//     }
//     let joltage_str = format!("{}{}", l1, l2);
//     if let Ok(joltage) = joltage_str.parse::<u64>() {
//         max_joltage = joltage;
//     }

//     max_joltage
// }

pub fn largest_joltage(batteries: &str, depth: usize) -> u64 {
    let chars: Vec<char> = batteries.chars().collect();

    fn build_max(chars: &[char], depth: usize) -> String {
        if depth == 0 || chars.is_empty() || chars.len() < depth {
            return String::new();
        }
        if depth == 1 {
            if let Some(&c) = chars.iter().max() {
                return c.to_string();
            }
            return String::new();
        }

        // We must leave (depth-1) chars after the chosen one, so the
        // last allowed start index is chars.len() - depth.
        let last_start = chars.len() - depth;
        let mut max_c = chars[0];
        let mut max_idx = 0usize;
        for (i, &c) in chars.iter().enumerate().take(last_start + 1) {
            if c > max_c {
                max_c = c;
                max_idx = i;
            }
        }

        let mut s = max_c.to_string();
        s.push_str(&build_max(&chars[(max_idx + 1)..], depth - 1));
        s
    }

    let joltage_str = build_max(&chars, depth);
    joltage_str.parse::<u64>().unwrap_or(0)
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.lines();
    let mut total: u64 = 0;
    let depth = 2;
    for line in lines {
        total += largest_joltage(line, depth);
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let _lines = input.lines();
    let mut total: u64 = 0;
    let depth = 12;
    for line in input.lines() {
        total += largest_joltage(line, depth);
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {

        // * In `*98*7654321111111`, you can make the largest joltage possible, *`98`*, by turning on the first two batteries.
        // * In `*8*1111111111111*9*`, you can make the largest joltage possible by turning on the batteries labeled `8` and `9`, producing *`89`* jolts.
        // * In `2342342342342*78*`, you can make *`78`* by turning on the last two batteries (marked `7` and `8`).
        // * In `818181*9*1111*2*111`, the largest joltage you can produce is *`92`*. 

        assert_eq!(largest_joltage("9876543211111", 2), 98);
        assert_eq!(largest_joltage("81111111111119", 2), 89);
        assert_eq!(largest_joltage("234234234234278", 2), 78);
        assert_eq!(largest_joltage("818181911112111", 2), 92);
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {

//         * In `*987654321111*111`, the largest joltage can be found by turning on everything except some `1`s at the end to produce `*987654321111*`.
            // * In the digit sequence `*81111111111*111*9*`, the largest joltage can be found by turning on everything except some `1`s, producing `*811111111119*`.
        // * In `23*4*2*34234234278*`, the largest joltage can be found by turning on everything except a `2` battery, a `3` battery, and another `2` battery near the start to produce `*434234234278*`.
        // * In `*8*1*8*1*8*1*911112111*`, the joltage `*888911112111*` is produced by turning on everything except some `1`s near the front.

        // The total output joltage is now much larger: `987654321111` + `811111111119` + `434234234278` + `888911112111` = `*3121910778619*`.

        assert_eq!(largest_joltage("987654321111111", 12), 987654321111);
        assert_eq!(largest_joltage("81111111111119", 12), 811111111119);
        assert_eq!(largest_joltage("234234234234278", 12),434234234278);
        assert_eq!(largest_joltage("818181911112111", 12),888911112111);


        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
