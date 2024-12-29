use crate::prelude::Aoc2024;

fn brute_force(
    curr: i64,
    index: i64,
    values: &Vec<i64>,
    target: i64,
    allow_concat: bool,
) -> Option<i64> {
    if index >= values.len() as i64 {
        if curr == target {
            return Some(target);
        } else {
            return None;
        }
    }
    let next_val = values[index as usize];
    let next = curr + next_val;
    if next <= target {
        if let Some(res) = brute_force(next, index + 1, values, target, allow_concat) {
            return Some(res);
        }
    }
    let next = curr * next_val;
    if next <= target {
        if let Some(res) = brute_force(next, index + 1, values, target, allow_concat) {
            return Some(res);
        }
    }
    if allow_concat {
        let next = values[index as usize] + curr * 10i64.pow(next_val.ilog(10) + 1);
        if next <= target {
            if let Some(res) = brute_force(next, index + 1, values, target, allow_concat) {
                return Some(res);
            }
        }
    }
    None
}

fn handle_line(line: &str, allow_concat: bool) -> Option<i64> {
    let mut parts = line.split(":");
    let res = parts.next().and_then(|num| num.parse::<i64>().ok())?;
    let values: Vec<i64> = parts.next().and_then(|rest| {
        rest.trim()
            .split(" ")
            .map(|num| num.parse::<i64>().ok())
            .collect()
    })?;

    // test if res can be computed with values
    brute_force(*values.first()?, 1, &values, res, allow_concat)
}

pub struct Puzzle7 {}

impl Aoc2024 for Puzzle7 {
    fn name(&self) -> String {
        "Day 7: Bridge Repair".to_string()
    }

    fn solve_a(&self, input: &String) -> String {
        let calibration_result: i64 = input
            .trim()
            .lines()
            .filter_map(|line| handle_line(line, false))
            .sum();
        calibration_result.to_string()
    }

    fn solve_b(&self, input: &String) -> String {
        let calibration_result: i64 = input
            .trim()
            .lines()
            .filter_map(|line| handle_line(line, true))
            .sum();
        calibration_result.to_string()
    }
}
