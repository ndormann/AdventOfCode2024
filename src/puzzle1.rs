use std::iter::zip;

use crate::prelude::Aoc2024;

fn get_lists(contents: &String) -> (Vec<i64>, Vec<i64>) {
    let (first_list, second_list): (Vec<i64>, Vec<i64>) = contents
        .split("\n")
        .filter_map(|line| {
            let mut parts = line
                .split_whitespace()
                .filter_map(|num| num.parse::<i64>().ok());
            if let (Some(a), Some(b)) = (parts.next(), parts.next()) {
                Some((a, b))
            } else {
                None
            }
        })
        .unzip();
    (first_list, second_list)
}

pub struct Puzzle1 {}

impl Aoc2024 for Puzzle1 {
    fn name(&self) -> String {
        "Day 1: Historian Hysteria".to_string()
    }

    fn solve_a(&self, input: &String) -> String {
        let (mut first_list, mut second_list) = get_lists(input);
        first_list.sort_unstable();
        second_list.sort_unstable();
        let result: i64 = zip(first_list, second_list)
            .map(|(a, b)| (a - b).abs())
            .sum();
        result.to_string()
    }

    fn solve_b(&self, input: &String) -> String {
        let (mut first_list, mut second_list) = get_lists(input);
        first_list.sort_unstable();
        second_list.sort_unstable();

        let mut second_iterator = second_list.iter().peekable();
        let mut result: i64 = 0;
        let mut last: Option<i64> = None;
        let mut count: i64;
        for id in first_list {
            if last == Some(id) {
                continue;
            }
            last = Some(id);
            while let Some(&s_id) = second_iterator.peek() {
                if s_id >= &id {
                    break;
                }
                second_iterator.next();
            }

            count = 0;
            while second_iterator.peek().map_or(false, |&&s_id| s_id == id) {
                second_iterator.next();
                count += 1;
            }
            result += count * id;
        }
        result.to_string()
    }
}
