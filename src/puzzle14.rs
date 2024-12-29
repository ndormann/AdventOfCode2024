use crate::prelude::Aoc2024;
use std::fmt::{Debug, Formatter, Write};

struct Vector {
    x: i64,
    y: i64,
}

struct Robot {
    pos: Vector,
    vel: Vector,
}

struct Board {
    width: i64,
    height: i64,
    robots: Vec<Robot>,
}

impl From<&str> for Vector {
    fn from(value: &str) -> Self {
        let mut parts = value.split("=").last().unwrap().split(",");
        let x = parts.next().unwrap().parse::<i64>().unwrap();
        let y = parts.next().unwrap().parse::<i64>().unwrap();
        Self { x, y }
    }
}

impl From<&str> for Robot {
    fn from(value: &str) -> Self {
        let mut parts = value.split_whitespace();
        let pos = Vector::from(parts.next().unwrap());
        let vel = Vector::from(parts.next().unwrap());
        Self { pos, vel }
    }
}

impl From<&str> for Board {
    fn from(value: &str) -> Self {
        Self {
            width: 101,
            height: 103,
            robots: value.lines().map(Robot::from).collect(),
        }
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.to_board() {
            for el in row {
                let c: char = match el {
                    0 => '.',
                    _ => (el as u8 + b'0') as char,
                };
                f.write_char(c)?
            }
            f.write_char('\n')?
        }
        Ok(())
    }
}

impl Board {
    fn step(&mut self, steps: i64) {
        for robot in &mut self.robots {
            robot.pos.x =
                ((robot.pos.x + robot.vel.x * steps) % self.width + self.width) % self.width;
            robot.pos.y =
                ((robot.pos.y + robot.vel.y * steps) % self.height + self.height) % self.height;
        }
    }

    fn to_board(&self) -> Vec<Vec<i64>> {
        let mut board: Vec<Vec<i64>> = Vec::new();
        let mut row: Vec<i64> = Vec::new();
        row.resize(self.width as usize, 0);
        board.resize(self.height as usize, row);
        for robot in &self.robots {
            board[robot.pos.y as usize][robot.pos.x as usize] += 1
        }
        board
    }

    fn has_line(&self) -> bool {
        for row in self.to_board() {
            let mut start = None;
            let mut len = 0;
            for (i, &el) in row.iter().enumerate() {
                if el > 0 {
                    if start.is_some() {
                        len += 1;
                    } else {
                        start = Some(i);
                        len = 0;
                    }
                } else {
                    if len > 10 {
                        return true;
                    }
                    start = None;
                }
            }
        }
        false
    }

    fn score(&mut self) -> i64 {
        let mut quad_i = 0;
        let mut quad_ii = 0;
        let mut quad_iii = 0;
        let mut quad_iv = 0;

        for robot in &self.robots {
            // println!("{}, {}", robot.pos.x, robot.pos.y);
            if robot.pos.x < self.width / 2 && robot.pos.y < self.height / 2 {
                quad_i += 1
            }
            if robot.pos.x > self.width / 2 && robot.pos.y < self.height / 2 {
                quad_ii += 1
            }
            if robot.pos.x < self.width / 2 && robot.pos.y > self.height / 2 {
                quad_iii += 1
            }
            if robot.pos.x > self.width / 2 && robot.pos.y > self.height / 2 {
                quad_iv += 1
            }
        }

        quad_i * quad_ii * quad_iii * quad_iv
    }
}

pub struct Puzzle14 {}

impl Aoc2024 for Puzzle14 {
    fn name(&self) -> String {
        "Day 14: Restroom Redoubt".to_string()
    }

    fn solve_a(&self, input: &String) -> String {
        let mut board = Board::from(input.as_str());

        board.step(100);

        board.score().to_string()
    }

    fn solve_b(&self, input: &String) -> String {
        let mut board = Board::from(input.as_str());
        for i in 0..10000 {
            board.step(1);
            if board.has_line() {
                return i.to_string();
            }
        }
        "Not found".to_string()
    }
}
