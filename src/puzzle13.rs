use crate::prelude::Aoc2024;
use ndarray::prelude::*;

#[derive(Clone, Debug)]
struct ClawMachine {
    pub a: Array1<i64>,
    pub b: Array1<i64>,
    pub target: Array1<i64>,
}

impl ClawMachine {
    fn solve_a(&self) -> Option<i64> {
        let y = (self.target[0] * self.a[1] - self.target[1] * self.a[0])
            / (self.b[0] * self.a[1] - self.b[1] * self.a[0]);
        let x = (self.target[0] - self.b[0] * y) / self.a[0];

        let a_presses = x;
        let b_presses = y;
        if a_presses * self.a[0] + b_presses * self.b[0] == self.target[0]
            && a_presses * self.a[1] + b_presses * self.b[1] == self.target[1]
        {
            Some(3 * a_presses + b_presses)
        } else {
            None
        }
    }

    fn solve_b(&self) -> Option<i64> {
        let target: Array1<i64> = array![10000000000000, 10000000000000] + self.target.view();
        let y = (target[0] * self.a[1] - target[1] * self.a[0])
            / (self.b[0] * self.a[1] - self.b[1] * self.a[0]);
        let x = (target[0] - self.b[0] * y) / self.a[0];

        let a_presses = x;
        let b_presses = y;
        if a_presses * self.a[0] + b_presses * self.b[0] == target[0]
            && a_presses * self.a[1] + b_presses * self.b[1] == target[1]
        {
            Some(3 * a_presses + b_presses)
        } else {
            None
        }
    }
}

fn parse_button(line: &str) -> Array1<i64> {
    let parts: Vec<&str> = line.split_whitespace().map(|part| part.trim()).collect();
    let x_str = parts[2];
    let y_str = parts[3];
    let x: i64 = x_str[2..x_str.len() - 1].parse().unwrap();
    let y: i64 = y_str[2..].parse().unwrap();
    array![x, y]
}

fn parse_target(line: &str) -> Array1<i64> {
    let parts: Vec<&str> = line.split_whitespace().map(|part| part.trim()).collect();
    let x_str = parts[1];
    let y_str = parts[2];
    let x: i64 = x_str[2..x_str.len() - 1].parse().unwrap();
    let y: i64 = y_str[2..].parse().unwrap();
    array![x, y]
}

pub struct Puzzle13 {}

fn get_machines(input: &String) -> Vec<ClawMachine> {
    input
        .split("\n\n")
        .map(|machine| {
            let mut lines = machine.lines();
            let a = parse_button(lines.next().unwrap());
            let b = parse_button(lines.next().unwrap());
            let target = parse_target(lines.next().unwrap());
            ClawMachine { a, b, target }
        })
        .collect()
}

impl Aoc2024 for Puzzle13 {
    fn name(&self) -> String {
        "Day 13: Claw Contraption".to_string()
    }

    fn solve_a(&self, input: &String) -> String {
        let machines = get_machines(input);

        let res: i64 = machines.iter().filter_map(ClawMachine::solve_a).sum();
        res.to_string()
    }

    fn solve_b(&self, input: &String) -> String {
        let machines = get_machines(input);

        let res: i64 = machines.iter().filter_map(ClawMachine::solve_b).sum();
        res.to_string()
    }
}
