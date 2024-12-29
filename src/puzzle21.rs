use crate::prelude::Aoc2024;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum NumKey {
    Empty,
    A,
    D0,
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    D9,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum DirKey {
    Empty,
    A,
    Left,
    Right,
    Up,
    Down,
}

impl From<char> for NumKey {
    fn from(value: char) -> Self {
        match value {
            'A' => NumKey::A,
            '0' => NumKey::D0,
            '1' => NumKey::D1,
            '2' => NumKey::D2,
            '3' => NumKey::D3,
            '4' => NumKey::D4,
            '5' => NumKey::D5,
            '6' => NumKey::D6,
            '7' => NumKey::D7,
            '8' => NumKey::D8,
            '9' => NumKey::D9,
            _ => NumKey::Empty,
        }
    }
}

trait Control<KeyType> {
    fn cost(&mut self, target_key: KeyType) -> usize;
    fn reset(&mut self);
}

trait PadPosition {
    fn default_key() -> Self;
    fn crash_key() -> Self;
    fn position(&self) -> (i32, i32);
}

impl PadPosition for DirKey {
    fn default_key() -> Self {
        DirKey::A
    }

    fn crash_key() -> Self {
        DirKey::Empty
    }

    fn position(&self) -> (i32, i32) {
        match self {
            DirKey::Empty => (0, 0),
            DirKey::A => (0, 2),
            DirKey::Left => (1, 0),
            DirKey::Right => (1, 2),
            DirKey::Up => (0, 1),
            DirKey::Down => (1, 1),
        }
    }
}

impl PadPosition for NumKey {
    fn default_key() -> Self {
        NumKey::A
    }

    fn crash_key() -> Self {
        NumKey::Empty
    }

    fn position(&self) -> (i32, i32) {
        match self {
            NumKey::Empty => (3, 0),
            NumKey::D0 => (3, 1),
            NumKey::A => (3, 2),
            NumKey::D1 => (2, 0),
            NumKey::D2 => (2, 1),
            NumKey::D3 => (2, 2),
            NumKey::D4 => (1, 0),
            NumKey::D5 => (1, 1),
            NumKey::D6 => (1, 2),
            NumKey::D7 => (0, 0),
            NumKey::D8 => (0, 1),
            NumKey::D9 => (0, 2),
        }
    }
}

struct TargetCode<'a> {
    input: &'a str,
    code: Vec<NumKey>,
    number: usize,
}

impl<'a> Debug for TargetCode<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.input)
    }
}

impl<'a> From<&'a str> for TargetCode<'a> {
    fn from(value: &'a str) -> Self {
        let input = value.trim();
        let number = input[0..input.len() - 1].parse().unwrap();
        let code = input.chars().map(NumKey::from).collect();
        Self {
            input,
            code,
            number,
        }
    }
}

struct Pad<KeyType> {
    prior: Box<dyn Control<DirKey>>,
    cost_map: HashMap<(KeyType, KeyType), usize>,
    current_key: KeyType,
    misses: usize,
}

#[derive(Clone, Copy)]
struct Human {}

impl Control<DirKey> for Human {
    fn cost(&mut self, _: DirKey) -> usize {
        1
    }

    fn reset(&mut self) {}
}

impl<KeyType> Pad<KeyType>
where
    KeyType: PadPosition,
{
    pub fn new(prior: Box<dyn Control<DirKey>>) -> Self {
        Self {
            prior,
            cost_map: HashMap::new(),
            current_key: KeyType::default_key(),
            misses: 0,
        }
    }

    pub fn path_cost(&mut self, dx: i32, dy: i32, x_key: DirKey, y_key: DirKey) -> usize {
        let mut cost = 0;
        for _ in 0..dx.abs() {
            cost += self.prior.cost(x_key);
        }
        for _ in 0..dy.abs() {
            cost += self.prior.cost(y_key);
        }
        cost + self.prior.cost(DirKey::A)
    }
}

impl<KeyType> Control<KeyType> for Pad<KeyType>
where
    KeyType: Copy + PadPosition + Eq + Hash,
{
    fn cost(&mut self, target_key: KeyType) -> usize {
        if let Some(&cost) = self.cost_map.get(&(self.current_key, target_key)) {
            self.current_key = target_key;
            cost
        } else {
            self.misses += 1;
            let (x, y) = self.current_key.position();
            let (target_x, target_y) = target_key.position();
            let dy = target_y - y;
            let dx = target_x - x;
            let y_key = if dy < 0 { DirKey::Left } else { DirKey::Right };
            let x_key = if dx < 0 { DirKey::Up } else { DirKey::Down };

            // option horizontal, vertical
            let hv_cost = if KeyType::crash_key().position() != (x + dx, y) {
                self.path_cost(dx, dy, x_key, y_key)
            } else {
                usize::MAX
            };

            let vh_cost = if KeyType::crash_key().position() != (x, y + dy) {
                self.path_cost(dy, dx, y_key, x_key)
            } else {
                usize::MAX
            };

            let cost = if vh_cost > hv_cost { hv_cost } else { vh_cost };
            self.cost_map.insert((self.current_key, target_key), cost);
            self.current_key = target_key;
            cost
        }
    }

    fn reset(&mut self) {
        self.current_key = KeyType::default_key();
    }
}

fn get_robot_stack(stack_size: usize) -> Pad<NumKey> {
    let human_control = Human {};

    let mut last_layer: Box<dyn Control<DirKey>> = Box::new(human_control);
    for _ in 0..stack_size {
        last_layer = Box::new(Pad::<DirKey>::new(last_layer));
    }
    Pad::<NumKey>::new(last_layer)
}

fn evaluate_target_cost(targets: &Vec<TargetCode>, control_pad: &mut Pad<NumKey>) -> Vec<usize> {
    targets
        .iter()
        .map(|target| {
            let mut cost = 0;
            control_pad.reset();
            for key in &target.code {
                cost += control_pad.cost(*key);
            }
            cost * target.number
        })
        .collect()
}

fn simulate_stack(input: &String, depth: usize) -> usize {
    let targets: Vec<TargetCode> = input
        .lines()
        .filter(|&line| !line.is_empty())
        .map(TargetCode::from)
        .collect();

    let mut control_pad = get_robot_stack(depth);

    let total_cost: usize = evaluate_target_cost(&targets, &mut control_pad)
        .iter()
        .sum();
    total_cost
}

pub struct Puzzle21 {}

impl Aoc2024 for Puzzle21 {
    fn name(&self) -> String {
        "Day 21: Keypad Conundrum".to_string()
    }

    fn solve_a(&self, input: &String) -> String {
        simulate_stack(input, 2).to_string()
    }

    fn solve_b(&self, input: &String) -> String {
        simulate_stack(input, 25).to_string()
    }
}
