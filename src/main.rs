use crate::prelude::{read_day, Aoc2024};

mod prelude;

fn main() {
    let mut puzzles: Vec<Box<dyn Aoc2024>> = Vec::new();

    for (day, puzzle) in puzzles.iter().enumerate() {
        let content = read_day((day + 1) as u8);
        println!("{}", puzzle.name());
        println!("Part a: {}", puzzle.solve_a(&content));
        println!("Part b: {}", puzzle.solve_b(&content));
        println!();
    }
}
