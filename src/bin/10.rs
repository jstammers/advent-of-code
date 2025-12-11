use good_lp::{Expression, Solution, SolverModel, constraint, default_solver, variables};
use regex::Regex;
use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(10);

#[derive(Debug, Clone)]
pub struct JoltageMachine {
    pub buttons: Vec<Vec<u16>>,
    pub target_joltages: Vec<u16>,
    pub initial_state: u16,
    pub toggle_values: Vec<u16>,
}

pub fn parse_input(input: &str) -> Vec<JoltageMachine> {
    let mut machines = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if let Some(machine) = parse_machine_line(line) {
            machines.push(machine);
        }
    }

    machines
}

fn parse_machine_line(line: &str) -> Option<JoltageMachine> {
    // Example: [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    // Find the indicator pattern [.##.] and parse to get value
    let indicator_regex = Regex::new(r"^\[([.#]+)\]").ok()?;
    let mut initial_state: u16 = 0;
    let after_indicator = if let Some(caps) = indicator_regex.captures(line) {
        let state_str = caps.get(1)?.as_str();
        let state_str: String = state_str
            .chars()
            .rev()
            .map(|c| if c == '#' { '1' } else { '0' })
            .collect();
        initial_state = u16::from_str_radix(&state_str, 2).unwrap();
        &line[caps.get(0)?.end()..].trim()
    } else {
        line
    };

    // Find the target joltages at the end: {3,5,4,7}
    let target_regex = Regex::new(r"\{([0-9,]+)\}$").ok()?;
    let target_caps = target_regex.captures(after_indicator)?;
    let target_str = target_caps.get(1)?.as_str();

    let target_joltages: Vec<u16> = target_str
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    // Remove the target part to get the buttons part
    let buttons_part = &after_indicator[..target_caps.get(0)?.start()].trim();

    // Parse buttons: (3) (1,3) (2) (2,3) (0,2) (0,1)
    let button_regex = Regex::new(r"\(([0-9,]+)\)").ok()?;
    let mut buttons = Vec::new();
    let mut toggle_values = Vec::new();
    for caps in button_regex.captures_iter(buttons_part) {
        let button_str = caps.get(1)?.as_str();
        let button_counters: Vec<u16> = button_str
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        let toggle_value = button_counters.iter().fold(0u16, |acc, &x| acc | (1 << x));
        if !button_counters.is_empty() {
            buttons.push(button_counters);
            toggle_values.push(toggle_value);
        }
    }

    Some(JoltageMachine {
        buttons,
        target_joltages,
        initial_state,
        toggle_values,
    })
}

fn min_pushes(initial_state: u16, toggle_values: Vec<u16>) -> u64 {
    let mut queue: VecDeque<(u16, u64)> = VecDeque::new();
    let mut visited: HashSet<(u16, u64)> = HashSet::new();

    queue.push_back((initial_state, 0));
    visited.insert((initial_state, 0));

    while let Some((current_state, pushes)) = queue.pop_front() {
        for &toggle in &toggle_values {
            let next_state = current_state ^ toggle;
            let next_pushes = pushes + 1;
            // Found a cycle back to the initial state
            if next_state == 0 {
                return next_pushes;
            }

            if visited.insert((next_state, next_pushes)) {
                queue.push_back((next_state, next_pushes));
            }
        }
    }

    u64::MAX
}

fn build_columns_from_toggles(toggle_values: Vec<u16>, rows: usize) -> Vec<Vec<u16>> {
    let mut cols = Vec::with_capacity(toggle_values.len());
    for &t in &toggle_values {
        let mut col = vec![0u16; rows];
        for bit in 0..rows {
            if (t >> bit) & 1 == 1 {
                col[bit] = 1;
            }
        }
        cols.push(col);
    }
    cols
}

fn solve_lp_relaxation(buttons: Vec<Vec<u16>>, target_joltages: Vec<u16>) -> Option<u64> {
    // Start a new problem

    // Create one variable per button (continuous version)
    // use add_integer_variable() if you want integrality
    variables! {
        vars: 0 <= x[buttons.len()] (integer)
    };

    // Objective = sum of variables
    let objective: Expression = x.iter().copied().sum();
    let mut constraints = Vec::new();
    // Add constraints A x = b
    for (row, &target) in target_joltages.iter().enumerate() {
        let mut expr = Expression::from(0);

        for (j, button) in buttons.iter().enumerate() {
            if button[row] == 1 {
                expr = expr + x[j];
            }
        }
        constraints.push(constraint!(expr == target));
        // pb = pb.with(expr.eq(target as f64));
    }
    let solution = constraints
        .iter()
        .fold(vars.minimise(objective).using(default_solver), |pb, c| {
            pb.with(c.clone())
        })
        .solve()
        .unwrap();

    // Extract the total number of pushes
    let total: f64 = x.iter().map(|&var| solution.value(var)).sum();

    Some(total as u64)
}

pub fn min_pushes_joltages(buttons: Vec<Vec<u16>>, target_joltages: Vec<u16>) -> u16 {
    let rows = target_joltages.len();
    // quick infeasibility: any column larger than b in a row is still usable; no direct infeasibility here
    // initial residual:
    let start = target_joltages.clone();
    // canonicalize residual to a compact representation for HashSet
    fn pack_residual(r: &Vec<u16>) -> Vec<u16> {
        r.clone()
    } // use Vec as key (requires Hash, Eq)

    let mut q: VecDeque<(Vec<u16>, u16)> = VecDeque::new();
    let mut seen: HashSet<Vec<u16>> = HashSet::new();

    q.push_back((start.clone(), 0));
    seen.insert(pack_residual(&start));

    while let Some((res, steps)) = q.pop_front() {
        if res.iter().all(|&v| v == 0) {
            return steps;
        }
        for col in buttons.iter() {
            // check if we can subtract this column (componentwise)
            let mut ok = true;
            for i in 0..rows {
                if col[i] > res[i] {
                    ok = false;
                    break;
                }
            }
            if !ok {
                continue;
            }

            // form new residual
            let mut next = res.clone();
            for i in 0..rows {
                next[i] -= col[i];
            }

            if seen.insert(pack_residual(&next)) {
                q.push_back((next, steps + 1));
            }
        }
    }
    0
}

pub fn part_one(input: &str) -> Option<u64> {
    let machines = parse_input(input);
    let mut total_pushes = 0;
    for machine in machines {
        let count = min_pushes(machine.initial_state, machine.toggle_values);
        total_pushes += count;
    }
    Some(total_pushes)
}

pub fn part_two(input: &str) -> Option<u64> {
    let machines = parse_input(input);
    let mut total_pushes = 0;
    for machine in machines {
        let columns = build_columns_from_toggles(machine.toggle_values, machine.target_joltages.len());
        let pushes = solve_lp_relaxation(columns, machine.target_joltages);
        total_pushes += pushes.unwrap_or(0);
    }
    Some(total_pushes as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(min_pushes(6, vec![8, 10, 4, 12, 5, 3]), 2);
        assert_eq!(min_pushes(8, vec![29, 12, 17, 7, 30]), 3);
        assert_eq!(min_pushes(46, vec![31, 25, 55, 6]), 2);
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
