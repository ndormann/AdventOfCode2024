use crate::prelude::Aoc2024;
use regex::Regex;
pub struct Puzzle3 {}

impl Aoc2024 for Puzzle3 {
    fn name(&self) -> String {
        "Day 3: Mull It Over".to_string()
    }

    fn solve_a(&self, input: &String) -> String {
        let mul_regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
        let matches = mul_regex.find_iter(&*input);

        let mut result_a: i64 = 0;

        // Part 1: Count compute muls
        for mul in matches {
            let mul_str = mul.as_str();
            let rest: Vec<i64> = mul_str
                .get(4..mul.len() - 1)
                .unwrap()
                .split(",")
                .filter_map(|num| num.parse::<i64>().ok())
                .collect();
            result_a += rest[0] * rest[1];
        }
        result_a.to_string()
    }

    fn solve_b(&self, input: &String) -> String {
        let mul_regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|don't|do").unwrap();
        let matches = mul_regex.find_iter(&*input);

        let mut result_b: i64 = 0;
        let mut enabled: i64 = 1;

        // Part 2
        for mul in matches {
            let mul_str = mul.as_str();
            if mul_str.starts_with("don") {
                enabled = 0;
            } else if mul_str.starts_with("do") {
                enabled = 1;
            } else {
                let rest: Vec<i64> = mul_str
                    .get(4..mul.len() - 1)
                    .unwrap()
                    .split(",")
                    .filter_map(|num| num.parse::<i64>().ok())
                    .collect();
                result_b += enabled * rest[0] * rest[1];
            }
        }
        result_b.to_string()
    }
}
