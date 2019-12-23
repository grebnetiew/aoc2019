use crate::intcode::Computer;
use aoc_runner_derive::{aoc, aoc_generator};
//use std::collections::HashMap;
use std::num::ParseIntError;

#[aoc_generator(day19)]
fn one_line_many_numbers(input: &str) -> Result<Vec<i64>, ParseIntError> {
    input.split(',').map(str::parse).collect()
}

fn is_beam(program: &[i64], pos: (i64, i64)) -> bool {
    Computer::new(program.to_vec(), vec![pos.1, pos.0])
        .run_until_output()
        .expect("Program halted before producing output")
        == 1
}

#[aoc(day19, part1)]
fn solver1(program: &[i64]) -> i64 {
    let mut count = 0;
    for x in 0..50 {
        for y in 0..50 {
            let thisone = is_beam(program, (x, y));
            count += if thisone { 1 } else { 0 };
        }
    }
    count
}

#[aoc(day19, part2)]
fn solver2(program: &[i64]) -> i64 {
    let mut x = 150;
    let mut y = 0;
    loop {
        x += 1;
        while !is_beam(program, (x, y)) {
            y += 1;
        }
        // Top right of a possible 100x100 square
        // Test if bottom left is also in the beam
        // if so: done!
        if is_beam(program, (x - 99, y + 99)) {
            return (x - 99) * 10_000 + y;
        }
    }
}

#[cfg(test)]
mod tests {
    // No tests in the puzzle text :(
}
