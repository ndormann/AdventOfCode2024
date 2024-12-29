use crate::prelude::Aoc2024;
use gcd::Gcd;
use std::collections::HashSet;
use std::fmt::{Debug, Formatter, Write};

#[derive(PartialEq, Eq, Clone, Copy)]
enum Field {
    FREE,
    ANTINODE,
    NODE(u8),
}

impl From<&u8> for Field {
    fn from(value: &u8) -> Self {
        match value {
            b'.' => Field::FREE,
            c => Field::NODE(*c),
        }
    }
}

impl Debug for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Field::FREE => f.write_char('.'),
            Field::NODE(c) => f.write_char(*c as char),
            Field::ANTINODE => f.write_char('#'),
        }
    }
}

pub struct Puzzle8 {}

impl Puzzle8 {
    fn count_special_nodes(input: &String) -> (usize, usize) {
        let mut board: Vec<Vec<Field>> = input
            .lines()
            .map(str::trim)
            .map(|line| line.as_bytes().iter().map(Field::from).collect())
            .collect();

        let mut simple_antinodes: HashSet<(i64, i64)> = HashSet::new();
        let mut harmonic_antinodes: HashSet<(i64, i64)> = HashSet::new();

        let mut nodes: HashSet<u8> = HashSet::new();
        for row in &board {
            for el in row {
                if let Field::NODE(c) = el {
                    nodes.insert(*c);
                }
            }
        }

        let size = board.len();
        let within_bounds = move |x: i64, y: i64| -> bool {
            x >= 0 && x < size as i64 && y >= 0 && y < size as i64
        };

        for node in nodes {
            let mut positions: Vec<(i64, i64)> = Vec::new();
            for i in 0..board.len() {
                for j in 0..board.len() {
                    if board[i][j] == Field::NODE(node) {
                        positions.push((i as i64, j as i64));
                    }
                }
            }
            let positions = positions;
            if positions.len() < 2 {
                continue;
            }
            for i in 0..positions.len() - 1 {
                for j in i + 1..positions.len() {
                    let a = positions[i];
                    let b = positions[j];

                    let first = (2 * a.0 - b.0, 2 * a.1 - b.1);
                    let second = (2 * b.0 - a.0, 2 * b.1 - a.1);
                    simple_antinodes.insert(first);
                    simple_antinodes.insert(second);

                    let diff = ((a.0 - b.0).abs() as u64, (a.1 - b.1).abs() as u64);
                    let divider: i64 = diff.0.gcd(diff.1) as i64;
                    let diff = ((a.0 - b.0) / divider, (a.1 - b.1) / divider);
                    harmonic_antinodes.insert(a);
                    let mut next = (a.0 - diff.0, a.1 - diff.1);
                    while within_bounds(next.0, next.1) {
                        harmonic_antinodes.insert(next);
                        next = (next.0 - diff.0, next.1 - diff.1);
                    }
                    next = (a.0 + diff.0, a.1 + diff.1);
                    while within_bounds(next.0, next.1) {
                        harmonic_antinodes.insert(next);
                        next = (next.0 + diff.0, next.1 + diff.1);
                    }
                }
            }
        }
        let mut simple_count = 0;
        for (x, y) in simple_antinodes {
            if x >= 0 && x < size as i64 && y >= 0 && y < size as i64 {
                board[x as usize][y as usize] = Field::ANTINODE;
                simple_count += 1;
            }
        }
        let mut harmonic_count = 0;
        for (x, y) in harmonic_antinodes {
            if x >= 0 && x < size as i64 && y >= 0 && y < size as i64 {
                board[x as usize][y as usize] = Field::ANTINODE;
                harmonic_count += 1;
            }
        }
        (simple_count, harmonic_count)
    }
}

impl Aoc2024 for Puzzle8 {
    fn name(&self) -> String {
        "Day 8: Resonant Collinearity".to_string()
    }

    fn solve_a(&self, input: &String) -> String {
        let (simple_antinodes, _) = Self::count_special_nodes(input);
        simple_antinodes.to_string()
    }

    fn solve_b(&self, input: &String) -> String {
        let (_, harmonitc_antinodes) = Self::count_special_nodes(input);
        harmonitc_antinodes.to_string()
    }
}
