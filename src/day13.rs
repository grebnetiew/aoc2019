use crate::intcode::Computer;
use aoc_runner_derive::{aoc, aoc_generator};
use std::cell::RefCell;
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
    while let (Some(x), Some(y), Some(t)) = (
        arcade.run_until_output(),
        arcade.run_until_output(),
        arcade.run_until_output(),
    ) {
        hm.insert((x, y), Tile::new(t));
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
    let paddle_x = RefCell::new(0isize);
    let ball_x = RefCell::new(0isize);

    // Joystick position depends on the ball and paddle
    let joystick = || (*ball_x.borrow() - *paddle_x.borrow()).signum();

    while let (Some(x), Some(y), Some(t)) = (
        arcade.run_until_output_with(joystick),
        arcade.run_until_output_with(joystick),
        arcade.run_until_output_with(joystick),
    ) {
        if x == -1 {
            score = t;
        } else {
            let t = Tile::new(t);
            match t {
                Tile::Ball => *ball_x.borrow_mut() = x,
                Tile::HPaddle => *paddle_x.borrow_mut() = x,
                _ => {}
            }
            hm.insert((x, y), t);
        }
    }
    score
}

#[cfg(test)]
mod tests {
    // No tests in the puzzle text :(
}
