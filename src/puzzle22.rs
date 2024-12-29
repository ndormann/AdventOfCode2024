use crate::prelude::Aoc2024;
use std::collections::vec_deque::VecDeque;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Clone)]
struct Order {
    bananas: u8,
    sequence: [i8; 4],
}

impl Order {
    fn new(sequence: &VecDeque<i8>, bananas: u8) -> Self {
        let sequence: [i8; 4] = sequence
            .iter()
            .copied()
            .take(4)
            .collect::<Vec<i8>>()
            .try_into()
            .expect("VecDeque must have exactly 4 elements");

        Self { sequence, bananas }
    }
}

fn evolve(mut secret: usize, steps: usize) -> usize {
    for _ in 0..steps {
        secret ^= secret << 6;
        secret %= 16777216;
        secret ^= secret >> 5;
        secret %= 16777216;
        secret ^= secret << 11;
        secret %= 16777216;
    }
    secret
}

fn map(mut secret: usize, steps: usize) -> HashMap<[i8; 4], u8> {
    let mut map: HashMap<[i8; 4], u8> = HashMap::new();
    let mut buffer = VecDeque::with_capacity(4);
    let mut last_price = (secret % 10) as i8;
    for step in 0..steps {
        secret ^= secret << 6;
        secret %= 16777216;
        secret ^= secret >> 5;
        secret %= 16777216;
        secret ^= secret << 11;
        secret %= 16777216;
        let price = (secret % 10) as i8;
        let diff = price - last_price;
        last_price = price;

        match step {
            0 | 1 | 2 => buffer.push_back(diff),
            3 => {
                buffer.push_back(diff);
                let order = Order::new(&buffer, price as u8);
                if !map.contains_key(&order.sequence) {
                    map.insert(order.sequence, order.bananas);
                }
            }
            _ => {
                buffer.pop_front();
                buffer.push_back(diff);
                let order = Order::new(&buffer, price as u8);
                if !map.contains_key(&order.sequence) {
                    map.insert(order.sequence, order.bananas);
                }
            }
        }
    }
    map
}

pub struct Puzzle22 {}

impl Aoc2024 for Puzzle22 {
    fn name(&self) -> String {
        "Day 22: Monkey Market".to_string()
    }

    fn solve_a(&self, input: &String) -> String {
        let secrets: Vec<usize> = input
            .lines()
            .filter(|&line| !line.is_empty())
            .filter_map(|num| num.parse().ok())
            .collect();

        let mut sum = 0;
        for &secret in &secrets {
            sum += evolve(secret, 2000);
        }
        sum.to_string()
    }

    fn solve_b(&self, input: &String) -> String {
        let secrets: Vec<usize> = input
            .lines()
            .filter(|&line| !line.is_empty())
            .filter_map(|num| num.parse().ok())
            .collect();

        let mut rewards = Vec::new();
        for &secret in &secrets {
            rewards.push(map(secret, 2000));
        }
        let mut banana_total: HashMap<[i8; 4], usize> = HashMap::new();

        for reward in &rewards {
            for (sequence, bananas) in reward {
                if let Some(el) = banana_total.get_mut(sequence) {
                    *el += *bananas as usize;
                } else {
                    banana_total.insert(*sequence, *bananas as usize);
                }
            }
        }

        let mut result = 0;
        for (_, bananas) in banana_total {
            if bananas > result {
                result = bananas;
            }
        }
        result.to_string()
    }
}
