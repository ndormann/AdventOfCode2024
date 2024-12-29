use std::{env, fs};

pub trait Aoc2024 {
    fn name(&self) -> String;
    fn solve_a(&self, input: &String) -> String;
    fn solve_b(&self, input: &String) -> String;
}

pub fn read_day(day: u8) -> String {
    let cwd = env::current_dir().unwrap();
    let filepath = cwd.join("data").join(format!("input{day}.txt"));
    let f = fs::read_to_string(&filepath);
    f.expect(format!("could not open input file @ {:?}", filepath).as_str())
}
