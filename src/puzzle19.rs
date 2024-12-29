use crate::prelude::Aoc2024;
use std::collections::HashMap;

struct PatternStore<'a> {
    towels: Vec<&'a str>,
    patterns: HashMap<&'a str, usize>,
}

impl<'a> PatternStore<'a> {
    pub fn new(towels: Vec<&'a str>) -> Self {
        let patterns = HashMap::new();
        Self { towels, patterns }
    }

    fn resolve(&mut self, design: &'a str) -> usize {
        if design.is_empty() {
            return 1;
        }
        if self.patterns.contains_key(design) {
            return self.patterns[design];
        }
        let mut count = 0;
        for i in 1..design.len() + 1 {
            let substring = &design[0..i];
            if self.towels.contains(&substring) {
                count += self.resolve(&design[i..design.len()]);
            }
        }
        self.patterns.insert(design, count);
        count
    }
}

pub struct Puzzle19 {}

impl Aoc2024 for Puzzle19 {
    fn name(&self) -> String {
        "Day 19: Linen Layout".to_string()
    }

    fn solve_a(&self, input: &String) -> String {
        let mut parts = input.split("\n\n");
        let towels: Vec<&str> = parts.next().unwrap().split(",").map(str::trim).collect();
        let designs: Vec<&str> = parts.next().unwrap().split("\n").map(str::trim).collect();
        let mut pattern_store = PatternStore::new(towels);

        let mut possible_patterns = 0;
        for (_, design) in designs.iter().enumerate() {
            if pattern_store.resolve(design) > 0 {
                possible_patterns += 1;
            }
        }
        possible_patterns.to_string()
    }

    fn solve_b(&self, input: &String) -> String {
        let mut parts = input.split("\n\n");
        let towels: Vec<&str> = parts.next().unwrap().split(",").map(str::trim).collect();
        let designs: Vec<&str> = parts.next().unwrap().split("\n").map(str::trim).collect();
        let mut pattern_store = PatternStore::new(towels);

        let mut possible_designs = 0;
        for (_, design) in designs.iter().enumerate() {
            possible_designs += pattern_store.resolve(design);
        }
        possible_designs.to_string()
    }
}
