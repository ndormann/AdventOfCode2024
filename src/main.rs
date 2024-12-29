use crate::prelude::{read_day, Aoc2024};
use std::time::Instant;

mod prelude;
mod puzzle1;
mod puzzle10;
mod puzzle11;
mod puzzle12;
mod puzzle13;
mod puzzle14;
mod puzzle15;
mod puzzle16;
mod puzzle17;
mod puzzle18;
mod puzzle19;
mod puzzle2;
mod puzzle20;
mod puzzle21;
mod puzzle22;
mod puzzle23;
mod puzzle24;
mod puzzle25;
mod puzzle3;
mod puzzle4;
mod puzzle5;
mod puzzle6;
mod puzzle7;
mod puzzle8;
mod puzzle9;

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
    puzzles.push(Box::new(puzzle11::Puzzle11 {}));
    puzzles.push(Box::new(puzzle12::Puzzle12 {}));
    puzzles.push(Box::new(puzzle13::Puzzle13 {}));
    puzzles.push(Box::new(puzzle14::Puzzle14 {}));
    puzzles.push(Box::new(puzzle15::Puzzle15 {}));
    puzzles.push(Box::new(puzzle16::Puzzle16 {}));
    puzzles.push(Box::new(puzzle17::Puzzle17 {}));
    puzzles.push(Box::new(puzzle18::Puzzle18 {}));
    puzzles.push(Box::new(puzzle19::Puzzle19 {}));
    puzzles.push(Box::new(puzzle20::Puzzle20 {}));
    puzzles.push(Box::new(puzzle21::Puzzle21 {}));
    puzzles.push(Box::new(puzzle22::Puzzle22 {}));

    for (day, puzzle) in puzzles.iter().enumerate() {
        let content = read_day((day + 1) as u8);
        println!("{}", puzzle.name());
        let start = Instant::now();
        let res_a = puzzle.solve_a(&content);
        let duration = Instant::now() - start;
        println!("Part a: {} took {:?}", res_a, duration);
        let start = Instant::now();
        let res_b = puzzle.solve_b(&content);
        let duration = Instant::now() - start;
        println!("Part b: {} took {:?}", res_b, duration);
        println!();
    }
}
