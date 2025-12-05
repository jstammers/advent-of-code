advent_of_code::solution!(5);

fn parse_input(input: &str) -> (Vec<(u64,u64)>, Vec<u64>){
    //parse the input into a vector of tuples of lower and upper bounds and a vector of numbers to check
    
    let mut bounds: Vec<(u64,u64)> = Vec::new();
    let mut numbers: Vec<u64> = Vec::new();
    
    // split on double newlines
    // first part is the bounds
    // second part is the numbers to check
    let parts: Vec<&str> = input.split("\n\n").collect();
    let bounds_part = parts[0];
    let numbers_part = parts[1];
    
    for line in bounds_part.lines() {
        let nums: Vec<u64> = line.split('-').map(|x| x.parse().unwrap()).collect();
        bounds.push((nums[0], nums[1]));
    }
    
    for line in numbers_part.lines() {
        let num: u64 = line.parse().unwrap();
        numbers.push(num);
    }
    (bounds, numbers)
}
pub fn part_one(input: &str) -> Option<u64> {
    let (bounds, numbers) = parse_input(input);
    let mut count: u64 = 0;
    for num in numbers {
        for (lower, upper) in &bounds {
            if num >= *lower && num <= *upper {
                count += 1;
                break;
            }
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (bounds, numbers) = parse_input(input);
    let mut count: u64 = 0;
    // sort bounds in ascending order of lower bound
    let mut bounds = bounds;
    bounds.sort_by(|a, b| a.0.cmp(&b.0));
    //iterate through bounds and create new bounds if they overlap
    let mut merged_bounds: Vec<(u64,u64)> = Vec::new();
    for (lower, upper) in bounds {
        if merged_bounds.is_empty() {
            merged_bounds.push((lower, upper));
        } else {
            let (last_lower, last_upper) = merged_bounds.last_mut().unwrap();
            if lower <= *last_upper {
                // overlap
                *last_upper = (*last_upper).max(upper);
            } else {
                merged_bounds.push((lower, upper));
            }
        }
    }
    // now count the total numbers covered by merged bounds
    for (lower, upper) in merged_bounds {
        count += upper - lower + 1;
    }
    Some(count)
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
