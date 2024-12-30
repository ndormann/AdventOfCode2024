use crate::prelude::Aoc2024;

const LOCK_HEIGHT: u8 = 5;

#[derive(Debug)]
struct Blueprint {
    layout: [u8; 5],
    is_key: bool,
}

pub struct Puzzle25 {}

fn get_blueprints(input: &String) -> Vec<Blueprint> {
    let parts = input.split("\n\n");
    parts
        .map(|blueprint| {
            let blueprint: Vec<Vec<bool>> = blueprint
                .trim()
                .lines()
                .map(|line| line.bytes().map(|el| el == b'.').collect::<Vec<bool>>())
                .collect();
            // it's a key if the top left element is '.'
            let is_key = blueprint[0][0];
            let size = blueprint.len();
            let mut layout = Vec::new();
            for j in 0..blueprint[0].len() {
                for i in 0..size {
                    if is_key ^ blueprint[i][j] {
                        layout.push(i as u8 - 1);
                        break;
                    }
                }
            }
            if is_key {
                layout = layout.iter().map(|x| LOCK_HEIGHT - x).collect();
            }
            Blueprint {
                is_key,
                layout: layout.try_into().unwrap(),
            }
        })
        .collect()
}

fn key_fits_lock(key: &Blueprint, lock: &Blueprint) -> bool {
    key.layout
        .iter()
        .zip(lock.layout)
        .all(|(k, l)| k + l <= LOCK_HEIGHT)
}

impl Aoc2024 for Puzzle25 {
    fn name(&self) -> String {
        "Day 25: Code Chronicle".to_string()
    }

    fn solve_a(&self, input: &String) -> String {
        let blueprints = get_blueprints(input);
        let keys: Vec<&Blueprint> = blueprints
            .iter()
            .filter(|blueprint| blueprint.is_key)
            .collect();

        let mut fits = 0;
        for lock in blueprints.iter().filter(|blueprint| !blueprint.is_key) {
            for key in &keys {
                if key_fits_lock(key, lock) {
                    fits += 1;
                }
            }
        }
        fits.to_string()
    }

    fn solve_b(&self, _input: &String) -> String {
        "None".to_string()
    }
}
