advent_of_code::solution!(6);

fn parse_input(input: &str) -> (Vec<Vec<u64>>, Vec<char>) {
    //parse the input into a vector of vectors of numbers to check and a vector of operations
    // each vector is a column from the input and the last elecment is the operation to perform on that column
    //
    // split on newlines
    //
    let mut columns: Vec<Vec<u64>> = Vec::new();
    let mut operations: Vec<char> = Vec::new();
    let lines: Vec<&str> = input.lines().collect();
    let num_columns = lines[0].split_whitespace().count();
    let num_lines = lines.len();
    for _ in 0..num_columns {
        columns.push(Vec::new());
    }
    for (j, line) in lines.iter().enumerate() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if j == num_lines - 1 {
            // last line is operations
            for i in 0..num_columns {
                let op: char = parts[i].chars().next().unwrap();
                operations.push(op);
            }
            break;
        }
        for i in 0..num_columns {
            let num: u64 = parts[i].parse().unwrap();
            columns[i].push(num);
        }
    }
    (columns, operations)
}

fn parse_input_column_wise(input: &str) -> (Vec<Vec<u64>>, Vec<char>) {
    let lines: Vec<&str> = input.lines().collect();
    let num_lines = lines.len();

    if num_lines == 0 {
        return (Vec::new(), Vec::new());
    }

    // Last line contains operations
    let ops_line = lines[num_lines - 1];

    // Determine max column width
    let num_columns = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    // Pre-collect chars for each line to avoid nth() calls
    let lines_chars: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();

    let mut columns: Vec<Vec<u64>> = Vec::new();
    let mut operations: Vec<char> = Vec::new();

    // Build columns
    let mut current_column_digits = String::new();

    for col in 0..num_columns {
        // Collect characters in column `col` except last line (ops line)
        current_column_digits.clear();
        let mut has_digit = false;

        for row in 0..num_lines - 1 {
            if let Some(&c) = lines_chars[row].get(col) {
                if !c.is_whitespace() {
                    has_digit = true;
                    current_column_digits.push(c);
                }
            }
        }

        // Parse column if it contains any digit
        if has_digit {
            let value: u64 = current_column_digits.parse().unwrap();
            if columns.is_empty() {
                columns.push(vec![value]);
            } else {
                // Append to last column
                columns.last_mut().unwrap().push(value);
            }
        } else {
            // Start a new column group
            if !columns.is_empty() {
                columns.push(Vec::new());
            }
        }

        // Operation char for this column (could be whitespace)
        if let Some(op) = ops_line.chars().nth(col) {
            operations.push(op);
        }
    }

    // Remove empty trailing group if created
    columns.retain(|c| !c.is_empty());
    operations.retain(|c| !c.is_whitespace());

    (columns, operations)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (columns, operations) = parse_input(input);
    let mut result: u64 = 0;
    for i in 0..columns.len() {
        let col = &columns[i];
        let op = operations[i];
        let col_result: u64 = match op {
            '+' => col.iter().sum(),
            '*' => col.iter().product(),
            'm' => *col.iter().min().unwrap(),
            'M' => *col.iter().max().unwrap(),
            _ => 0,
        };
        result += col_result;
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (columns, operations) = parse_input_column_wise(input);
    let mut result: u64 = 0;
    for i in 0..columns.len() {
        let col = &columns[i];
        let op = operations[i];
        let col_result: u64 = match op {
            '+' => col.iter().sum(),
            '*' => col.iter().product(),
            'm' => *col.iter().min().unwrap(),
            'M' => *col.iter().max().unwrap(),
            _ => 0,
        };
        result += col_result;
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
