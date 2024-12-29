use crate::prelude::Aoc2024;
use std::collections::HashMap;

fn blink_hashmap(stones: &HashMap<i64, i64>, target_stones: &mut HashMap<i64, i64>) {
    target_stones.clear();
    for (stone, count) in stones {
        if *stone == 0 {
            target_stones
                .entry(1)
                .and_modify(|entry| *entry += *count)
                .or_insert(*count);
        } else {
            let log: u32 = (*stone).ilog10() + 1;
            if log % 2 == 0 {
                let cut_off = 10i64.pow(log / 2);
                target_stones
                    .entry(*stone / cut_off)
                    .and_modify(|entry| *entry += *count)
                    .or_insert(*count);
                target_stones
                    .entry(*stone % cut_off)
                    .and_modify(|entry| *entry += *count)
                    .or_insert(*count);
            } else {
                target_stones
                    .entry(*stone * 2024)
                    .and_modify(|entry| *entry += *count)
                    .or_insert(*count);
            }
        }
    }
}

pub struct Puzzle11 {}

fn get_stones_as_hashmap(input: &String) -> HashMap<i64, i64> {
    let mut lookup: HashMap<i64, i64> = HashMap::new();

    let stones: Vec<i64> = input
        .lines()
        .next()
        .unwrap()
        .split(" ")
        .filter_map(|num| num.parse::<i64>().ok())
        .collect();

    for stone in &stones {
        lookup.insert(*stone, 1);
    }
    lookup
}

impl Aoc2024 for Puzzle11 {
    fn name(&self) -> String {
        "Day 11: Plutonian Pebbles".to_string()
    }

    fn solve_a(&self, input: &String) -> String {
        let mut lookup = get_stones_as_hashmap(input);
        let mut lookup_target = lookup.clone();

        for _ in 0..25 {
            blink_hashmap(&lookup, &mut lookup_target);
            let tmp = lookup;
            lookup = lookup_target;
            lookup_target = tmp;
        }

        let result: i64 = lookup.iter().map(|(_, count)| count).sum();
        result.to_string()
    }

    fn solve_b(&self, input: &String) -> String {
        let mut lookup = get_stones_as_hashmap(input);
        let mut lookup_target = lookup.clone();

        for _ in 0..75 {
            blink_hashmap(&lookup, &mut lookup_target);
            let tmp = lookup;
            lookup = lookup_target;
            lookup_target = tmp;
        }

        let result: i64 = lookup.iter().map(|(_, count)| count).sum();
        result.to_string()
    }
}
