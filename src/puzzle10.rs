use crate::prelude::Aoc2024;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Position {
    x: usize,
    y: usize,
}
#[derive(Eq, PartialEq, Clone, Debug)]
struct Node {
    pub peaks: HashSet<Position>,
    pub height: usize,
    pub rank: usize,
}

impl Node {
    pub fn new(x: usize, y: usize, ch: &u8) -> Self {
        let height = (ch - b'0') as usize;
        let peaks = if height == 9 {
            HashSet::from([Position { x, y }])
        } else {
            HashSet::new()
        };
        let rank = if height == 9 { 1 } else { 0 };
        Self {
            peaks,
            rank,
            height,
        }
    }
}

pub struct Puzzle10 {}

fn flood_fill(input: &String) -> Vec<Vec<RefCell<Node>>> {
    let board: Vec<Vec<RefCell<Node>>> = input
        .lines()
        .map(str::trim)
        .enumerate()
        .map(|(x, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .map(|(y, el)| Node::new(x, y, el).into())
                .collect()
        })
        .collect();

    let mut current_nodes: HashMap<Position, &RefCell<Node>> = HashMap::new();
    let size = board.len();
    for i in 0..size {
        for j in 0..size {
            let node = &board[i][j];
            if node.borrow().height == 9 {
                current_nodes.insert(Position { x: i, y: j }, node);
            }
        }
    }
    let mut next_nodes: HashMap<Position, &RefCell<Node>> = current_nodes.clone();
    let mut current_height = 8;
    let directions = [(0, 1), (1, 0), (-1, 0), (0, -1)];
    loop {
        current_nodes.clear();
        for (pos, node) in next_nodes.iter() {
            let &Position { x, y } = pos;
            for (dx, dy) in directions {
                let next_x = x as i64 + dx;
                let next_y = y as i64 + dy;
                if next_x >= 0
                    && next_y >= 0
                    && (next_x as usize) < size
                    && (next_y as usize) < size
                {
                    let candidate = &board[next_x as usize][next_y as usize];
                    if candidate.borrow().height == current_height {
                        candidate
                            .borrow_mut()
                            .peaks
                            .extend(node.borrow().peaks.clone());
                        candidate.borrow_mut().rank += node.borrow().rank;
                        current_nodes.insert(
                            Position {
                                x: next_x as usize,
                                y: next_y as usize,
                            },
                            candidate,
                        );
                    }
                }
            }
        }
        let tmp = next_nodes;
        next_nodes = current_nodes;
        current_nodes = tmp;
        if current_height == 0 {
            break;
        }
        current_height -= 1;
    }
    board
}

impl Aoc2024 for Puzzle10 {
    fn name(&self) -> String {
        "Day 10: Hoof It".to_string()
    }

    fn solve_a(&self, input: &String) -> String {
        let nodes = flood_fill(input);
        let result: usize = nodes
            .iter()
            .map(|line| {
                line.iter()
                    .map(|node| node.borrow().peaks.len())
                    .sum::<usize>()
            })
            .sum();
        result.to_string()
    }

    fn solve_b(&self, input: &String) -> String {
        let nodes = flood_fill(input);
        let result: usize = nodes
            .iter()
            .map(|line| line.iter().map(|node| node.borrow().rank).sum::<usize>())
            .sum();
        result.to_string()
    }
}
