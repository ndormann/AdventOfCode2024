use crate::prelude::Aoc2024;

fn is_safe<'a, I>(levels: I) -> bool
where
    I: Iterator<Item = &'a i64>,
{
    let mut last = None;
    let mut direction: Option<i64> = None;

    for &current in levels {
        if let Some(last_value) = last {
            let diff: i64 = current - last_value;

            if diff.abs() < 1 || diff.abs() > 3 {
                return false;
            }

            if let Some(dir) = direction {
                if dir.signum() != diff.signum() {
                    return false;
                }
            } else {
                direction = Some(diff);
            }
        }
        last = Some(current);
    }
    true
}

fn is_safe_with_dampener(levels: &[i64]) -> bool {
    // Check if the full report is safe
    if is_safe(levels.iter()) {
        return true;
    }

    // Try removing each level and check if it becomes safe
    for i in 0..levels.len() {
        let filtered_levels = levels
            .iter()
            .enumerate()
            .filter(|&(idx, _)| idx != i)
            .map(|(_, level)| level);

        if is_safe(filtered_levels) {
            return true;
        }
    }
    false
}

pub struct Puzzle2 {}

impl Puzzle2 {
    fn get_reports(input: &String) -> Vec<Vec<i64>> {
        input
            .trim()
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .filter_map(|num| num.parse::<i64>().ok())
                    .collect()
            })
            .collect()
    }
}

impl Aoc2024 for Puzzle2 {
    fn name(&self) -> String {
        "Day 2: Red-Nosed Reports".to_string()
    }

    fn solve_a(&self, input: &String) -> String {
        let reports = Self::get_reports(input);

        // Part 1: Count safe reports without the Problem Dampener
        let safe_count_part1 = reports
            .iter()
            .filter(|report| is_safe(report.iter()))
            .count();
        safe_count_part1.to_string()
    }

    fn solve_b(&self, input: &String) -> String {
        let reports = Self::get_reports(input);

        // Part 2: Count safe reports with the Problem Dampener
        let safe_count_part2 = reports
            .iter()
            .filter(|&report| is_safe_with_dampener(report))
            .count();
        safe_count_part2.to_string()
    }
}
