use crate::intcode::Computer;
use crate::intcode_asm::Debugger;
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::num::ParseIntError;

#[aoc_generator(day17)]
fn one_line_many_numbers(input: &str) -> Result<Vec<i64>, ParseIntError> {
    input.split(',').map(str::parse).collect()
}

// From day 10...
fn dotpoundmap(input: &[Vec<i64>]) -> (Vec<(i64, i64)>, (i64, i64), Direction) {
    let mut points = Vec::new();
    let mut robot_pos = (0, 0);
    let mut d = Direction::North;

    for (y, line) in input.iter().enumerate() {
        for (x, &ch) in line.iter().enumerate().filter(|&(_, &ch)| ch != 46) {
            if ch == 35 {
                points.push((x as i64, y as i64));
            } else {
                robot_pos = (x as i64, y as i64);
                d = match ch as u8 as char {
                    '<' => Direction::West,
                    '>' => Direction::East,
                    '^' => Direction::North,
                    'v' => Direction::South,
                    _ => panic!("Unknown direction {}", ch),
                }
            }
        }
    }
    (points, robot_pos, d)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn turn_left(self) -> Self {
        match self {
            Direction::East => Direction::North,
            Direction::West => Direction::South,
            Direction::North => Direction::West,
            Direction::South => Direction::East,
        }
    }
    fn turn_right(self) -> Self {
        match self {
            Direction::East => Direction::South,
            Direction::West => Direction::North,
            Direction::North => Direction::East,
            Direction::South => Direction::West,
        }
    }
    fn modify_mut(self, x: &mut i64, y: &mut i64) {
        match self {
            Direction::East => *x += 1,
            Direction::West => *x -= 1,
            Direction::North => *y -= 1,
            Direction::South => *y += 1,
        }
    }

    fn modify(self, mut x: i64, mut y: i64) -> (i64, i64) {
        self.modify_mut(&mut x, &mut y);
        (x, y)
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::East => ">",
                Direction::West => "<",
                Direction::North => "^",
                Direction::South => "v",
            }
        )
    }
}

type TileMap = HashMap<(i64, i64), i64>;

#[allow(dead_code)]
fn print(m: &TileMap) -> String {
    let xmin = m.keys().map(|k| k.0).min().unwrap();
    let xmax = m.keys().map(|k| k.0).max().unwrap();
    let ymin = m.keys().map(|k| k.1).min().unwrap();
    let ymax = m.keys().map(|k| k.1).max().unwrap();
    let mut result = String::new();
    for y in (ymin - 5)..=(ymax + 5) {
        for x in (xmin - 5)..=(xmax + 5) {
            if (x, y) == (0, 0) {
                result += "S";
            } else {
                result += match m.get(&(x, y)) {
                    Some(35) => "#",
                    Some(46) => ".",
                    Some(_) => "*",
                    None => " ",
                }
            }
        }
        result += "\n";
    }
    result
}

#[aoc(day17, part1)]
fn solver1(program: &[i64]) -> i64 {
    let mut scanner = Computer::from(program.to_vec());
    let ascii_map = scanner.run();
    let (scaffolds, _robot, _dir) = dotpoundmap(
        &ascii_map
            .split(|&n| n == 10)
            .map(Vec::from)
            .collect::<Vec<_>>(),
    );

    // Let's do it the brute force way
    let hm: HashMap<(i64, i64), bool> = scaffolds.iter().map(|&p| (p, true)).collect();

    scaffolds
        .iter()
        .filter_map(|pos| {
            if is_crossing(&hm, *pos) {
                Some(pos.0 * pos.1)
            } else {
                None
            }
        })
        .sum()
}

fn is_crossing(hm: &HashMap<(i64, i64), bool>, pos: (i64, i64)) -> bool {
    *hm.get(&Direction::North.modify(pos.0, pos.1))
        .unwrap_or(&false)
        && *hm
            .get(&Direction::South.modify(pos.0, pos.1))
            .unwrap_or(&false)
        && *hm
            .get(&Direction::East.modify(pos.0, pos.1))
            .unwrap_or(&false)
        && *hm
            .get(&Direction::West.modify(pos.0, pos.1))
            .unwrap_or(&false)
}

#[derive(Debug)]
enum Instruction {
    Forward(usize),
    TurnLeft,
    TurnRight,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::TurnRight => write!(f, "R"),
            Instruction::TurnLeft => write!(f, "L"),
            Instruction::Forward(amt) => write!(f, "{}", amt),
        }
    }
}

#[aoc(day17, part2)]
fn solver2(program: &[i64]) -> i64 {
    println!("{}", Debugger::from(program.to_vec()).assembly());
    let mut scanner = Computer::from(program.to_vec());
    let ascii_map = scanner.run();
    let (scaffolds, mut pos, mut dir) = dotpoundmap(
        &ascii_map
            .split(|&n| n == 10)
            .map(Vec::from)
            .collect::<Vec<_>>(),
    );

    let hm: HashMap<(i64, i64), bool> = scaffolds.iter().map(|&p| (p, true)).collect();
    let mut ins = Vec::<Instruction>::new();

    // Initial turn to face the path
    while !(*hm.get(&dir.modify(pos.0, pos.1)).unwrap_or(&false)) {
        dir = dir.turn_right();
        ins.push(Instruction::TurnRight);
    }

    // Loop of Forward X turn T
    loop {
        let mut distance = 0;
        while *hm.get(&dir.modify(pos.0, pos.1)).unwrap_or(&false) {
            dir.modify_mut(&mut pos.0, &mut pos.1);
            distance += 1;
        }
        ins.push(Instruction::Forward(distance));

        // Check left and right
        if *hm
            .get(&dir.turn_left().modify(pos.0, pos.1))
            .unwrap_or(&false)
        {
            dir = dir.turn_left();
            ins.push(Instruction::TurnLeft);
        } else if *hm
            .get(&dir.turn_right().modify(pos.0, pos.1))
            .unwrap_or(&false)
        {
            dir = dir.turn_right();
            ins.push(Instruction::TurnRight);
        } else {
            // Destination reached
            break;
        }
    }

    println!("{:?}", ins);

    // Solve by hand :)

    // Make input string
    let input_str = "B,C,B,C,A,A,C,B,A,B\n\
                     L,10,R,8,R,8\n\
                     L,10,R,8,R,6,R,10\n\
                     L,12,R,8,L,12\n\
                     n\n\
                     "
    .to_string();

    // Make the actual robot
    let mut robot_program = program.to_vec();
    robot_program[0] = 2;
    let mut robot = Computer::new(
        robot_program,
        input_str.chars().rev().map(|c| c as u8 as i64).collect(),
    );
    let output = robot.run();
    println!(
        "{}",
        output.iter().map(|i| *i as u8 as char).collect::<String>()
    );
    output[output.len() - 1]
}

#[cfg(test)]
mod tests {
    // No tests in the puzzle text :(
}
