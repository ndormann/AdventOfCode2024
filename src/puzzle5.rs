use crate::prelude::Aoc2024;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

fn get_correct<'a, I>(lines: I, rules: &HashMap<i64, Vec<i64>>, correct: bool) -> Vec<Vec<i64>>
where
    I: Iterator<Item = &'a str>,
{
    let orders: Vec<Vec<i64>> = lines
        .skip_while(|line| line.trim() != "")
        .skip(1)
        .filter_map(|line| {
            let order: Vec<i64> = line
                .split(",")
                .filter_map(|num| num.parse::<i64>().ok())
                .collect();
            let mut forbidden: HashSet<i64> = HashSet::new();
            for &el in &order {
                if forbidden.contains(&el) {
                    if correct {
                        return None;
                    } else {
                        return Some(order);
                    }
                }
                if let Some(before) = rules.get(&el) {
                    forbidden.extend(before);
                }
            }
            if correct {
                return Some(order);
            } else {
                return None;
            }
        })
        .collect();
    orders
}

fn find_corrected_middle(mut order: Vec<i64>, rules: &HashMap<i64, Vec<i64>>) -> i64 {
    order.sort_by(|a, b| {
        if a == b {
            return Ordering::Equal;
        }
        if let Some(after) = rules.get(a) {
            if after.contains(b) {
                return Ordering::Less;
            }
        }
        Ordering::Greater
    });
    order[order.len() / 2]
}

pub struct Puzzle5 {}

impl Puzzle5 {
    fn get_rules(input: &String) -> HashMap<i64, Vec<i64>> {
        input
            .lines()
            .take_while(|line| line.trim() != "")
            .filter_map(|line| {
                let mut parts = line.split("|").filter_map(|num| num.parse::<i64>().ok());
                if let (Some(before), Some(after)) = (parts.next(), parts.next()) {
                    Some((before, after))
                } else {
                    None
                }
            })
            .fold(HashMap::new(), |mut acc, (before, after)| {
                acc.entry(after)
                    .and_modify(|element| element.push(before))
                    .or_insert(vec![before]);
                acc
            })
    }
}

impl Aoc2024 for Puzzle5 {
    fn name(&self) -> String {
        "Day 5: Print Queue".to_string()
    }

    fn solve_a(&self, input: &String) -> String {
        let rules = Self::get_rules(input);
        let middle_sum: i64 = get_correct(input.lines(), &rules, true)
            .iter()
            .map(|order| order[order.len() / 2])
            .sum();
        middle_sum.to_string()
    }

    fn solve_b(&self, input: &String) -> String {
        let rules = Self::get_rules(input);
        let middle_sum: i64 = get_correct(input.lines(), &rules, false)
            .iter()
            .map(|order| find_corrected_middle(order.clone(), &rules))
            .sum();
        middle_sum.to_string()
    }
}
