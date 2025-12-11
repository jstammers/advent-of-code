use std::collections::HashMap;
advent_of_code::solution!(11);

fn count_paths(
    map: &HashMap<String, Vec<String>>,
    start_key: &str,
    end_key: &str,
) -> u64 {
    if start_key == end_key {
        return 1;
    }
    
    let mut memo = HashMap::new();
    count_paths_dp(map, start_key, end_key, &mut memo)
}

fn count_paths_dp(
    map: &HashMap<String, Vec<String>>,
    current: &str,
    end_key: &str,
    memo: &mut HashMap<String, u64>,
) -> u64 {
    if let Some(&cached) = memo.get(current) {
        return cached;
    }
    
    let paths = map.get(current)
        .map(|neighbors| {
            neighbors.iter()
                .map(|neighbor| {
                    if neighbor == end_key { 1 } else { count_paths_dp(map, neighbor, end_key, memo) }
                })
                .sum()
        })
        .unwrap_or(0);
    
    memo.insert(current.to_string(), paths);
    paths
}

pub fn parse_input(input: &str) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split(": ").collect();
        if parts.len() == 2 {
            let key = parts[0].trim().to_string();
            let values: Vec<String> = parts[1].split(' ').map(|s| s.trim().to_string()).collect();
            map.insert(key, values);
        }
    }
    map
}
pub fn part_one(input: &str) -> Option<u64> {
    let data = parse_input(input);
    let steps = count_paths(&data, &"you".to_string(), &"out".to_string());
    Some(steps)
}

pub fn part_two(input: &str) -> Option<u64> {
    let data = parse_input(input);
    // Count from svr to fft
    let steps1 = count_paths(&data, &"svr".to_string(), &"fft".to_string());
    //Count from fft to dac
    let steps2 = count_paths(&data, &"fft".to_string(), &"dac".to_string());
    // // Count from dac to out
    let steps3 = count_paths(&data, &"dac".to_string(), &"out".to_string());
    let steps = steps1 * steps2 * steps3;
    Some(steps)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let example = "svr: aaa bbb
        aaa: fft
        fft: ccc
        bbb: tty
        tty: ccc
        ccc: ddd eee
        ddd: hub
        hub: fff
        eee: dac
        dac: fff
        fff: ggg hhh
        ggg: out
        hhh: out
        ";
        let result = part_two(&example);
        assert_eq!(result, Some(2));
    }
}
