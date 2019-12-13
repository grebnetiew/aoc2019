use crate::intcode::Computer;
use aoc_runner_derive::{aoc, aoc_generator};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::num::ParseIntError;

#[aoc_generator(day13)]
fn one_line_many_numbers(input: &str) -> Result<Vec<isize>, ParseIntError> {
    input.split(',').map(str::parse).collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Block,
    HPaddle,
    Ball,
}

impl Tile {
    fn new(n: isize) -> Self {
        match n {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::HPaddle,
            4 => Tile::Ball,
            _ => panic!("Invalid tile {}", n),
        }
    }
}

#[aoc(day13, part1)]
fn count_blocks(program: &[isize]) -> usize {
    let mut arcade = Computer::new(program.to_vec(), vec![]);
    let mut hm = HashMap::<(isize, isize), Tile>::new();
    while let Some(x) = arcade.run_until_output() {
        let maybe_y = arcade.run_until_output();
        if let Some(t) = arcade.run_until_output() {
            // The computer will not output None (which means it has halted)
            // and then output Some() again, so we can safely unwrap y
            hm.insert((x, maybe_y.unwrap()), Tile::new(t));
        }
    }
    hm.iter().filter(|&(_, &v)| v == Tile::Block).count()
}

#[aoc(day13, part2)]
fn breakout(program: &[isize]) -> isize {
    let mut program = program.to_vec();
    program[0] = 2; // insert two coins! greedy
    let mut arcade = Computer::new(program, vec![]);

    let mut hm = HashMap::<(isize, isize), Tile>::new();

    let mut score = 0;
    let paddle_x = RefCell::new(0);
    let ball_x = RefCell::new(0);

    // Joystick position depends on the ball and paddle
    let joystick = || match paddle_x.borrow().cmp(&ball_x.borrow()) {
        Ordering::Less => 1,
        Ordering::Greater => -1,
        Ordering::Equal => 0,
    };

    while let Some(x) = arcade.run_until_output_with(joystick) {
        let maybe_y = arcade.run_until_output_with(joystick);
        if let Some(t) = arcade.run_until_output_with(joystick) {
            if x == -1 {
                score = t;
            } else {
                let t = Tile::new(t);
                match t {
                    Tile::Ball => *ball_x.borrow_mut() = x,
                    Tile::HPaddle => *paddle_x.borrow_mut() = x,
                    _ => {}
                }
                hm.insert((x, maybe_y.unwrap()), t);
            }
        }
    }
    score
}

#[cfg(test)]
mod tests {
    // No tests in the puzzle text :(
}
