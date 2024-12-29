use crate::prelude::{read_day, Aoc2024};

mod prelude;
mod puzzle1;
mod puzzle2;
mod puzzle3;
mod puzzle4;
mod puzzle5;
mod puzzle6;
mod puzzle7;
mod puzzle8;
mod puzzle9;
mod puzzle10;

fn main() {
    let mut puzzles: Vec<Box<dyn Aoc2024>> = Vec::new();

    puzzles.push(Box::new(puzzle1::Puzzle1 {}));
    puzzles.push(Box::new(puzzle2::Puzzle2 {}));
    puzzles.push(Box::new(puzzle3::Puzzle3 {}));
    puzzles.push(Box::new(puzzle4::Puzzle4 {}));
    puzzles.push(Box::new(puzzle5::Puzzle5 {}));
    puzzles.push(Box::new(puzzle6::Puzzle6 {}));
    puzzles.push(Box::new(puzzle7::Puzzle7 {}));
    puzzles.push(Box::new(puzzle8::Puzzle8 {}));
    puzzles.push(Box::new(puzzle9::Puzzle9 {}));
    puzzles.push(Box::new(puzzle10::Puzzle10 {}));

    for (day, puzzle) in puzzles.iter().enumerate() {
        let content = read_day((day + 1) as u8);
        println!("{}", puzzle.name());
        println!("Part a: {}", puzzle.solve_a(&content));
        println!("Part b: {}", puzzle.solve_b(&content));
        println!();
    }
}
