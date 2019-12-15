use crate::intcode::Computer;
use crate::intcode_asm::Debugger;
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::fmt;
use std::num::ParseIntError;

#[aoc_generator(day15)]
fn one_line_many_numbers(input: &str) -> Result<Vec<i64>, ParseIntError> {
    input.split(',').map(str::parse).collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Oxygen,
}

impl Tile {
    fn new(n: i64) -> Self {
        match n {
            1 => Tile::Empty,
            0 => Tile::Wall,
            2 => Tile::Oxygen,
            _ => panic!("Invalid tile {}", n),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn command(self) -> i64 {
        match self {
            Direction::North => 1,
            Direction::South => 2,
            Direction::West => 3,
            Direction::East => 4,
        }
    }
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

struct Robot {
    c: Computer,
    x: i64,
    y: i64,
}

impl Robot {
    fn new(program: &[i64]) -> Self {
        Self {
            c: Computer::from(program.to_vec()),
            x: 0,
            y: 0,
        }
    }

    fn move_command(&mut self, hm: &mut TileMap, d: Direction) -> Tile {
        self.c.more_input(d.command());
        let response = Tile::new(self.c.run_until_output().expect("Robot has halted"));
        match response {
            Tile::Empty | Tile::Oxygen => {
                d.modify_mut(&mut self.x, &mut self.y);
                hm.insert((self.x, self.y), response);
            }
            Tile::Wall => {
                let (x, y) = d.modify(self.x, self.y);
                hm.insert((x, y), response);
            }
        }
        response
    }
}

type TileMap = HashMap<(i64, i64), Tile>;

#[allow(dead_code)]
fn print(m: &TileMap, robot: &Robot, d: Direction) -> String {
    let xmin = m.keys().map(|k| k.0).min().unwrap();
    let xmax = m.keys().map(|k| k.0).max().unwrap();
    let ymin = m.keys().map(|k| k.1).min().unwrap();
    let ymax = m.keys().map(|k| k.1).max().unwrap();
    let mut result = String::new();
    for y in (ymin - 5)..=(ymax + 5) {
        for x in (xmin - 5)..=(xmax + 5) {
            if (x, y) == (0, 0) {
                result += "S";
            } else if (x, y) == (robot.x, robot.y) {
                result += &format!("{}", d);
            } else {
                result += match m.get(&(x, y)) {
                    Some(Tile::Wall) => "#",
                    Some(Tile::Empty) => ".",
                    Some(Tile::Oxygen) => "*",
                    None => " ",
                }
            }
        }
        result += "\n";
    }
    result
}

#[aoc(day15, part1)]
fn find_oxygen(program: &[i64]) -> i64 {
    let mut robot = Robot::new(program);
    let mut hm = HashMap::<(i64, i64), Tile>::new();
    hm.insert((0, 0), Tile::Empty);

    let mut d = Direction::North;
    let mut steps = 0;

    while steps != 50000 {
        let backtrack = Some(&Tile::Empty) == hm.get(&d.modify(robot.x, robot.y));
        match robot.move_command(&mut hm, d) {
            Tile::Wall => d = d.turn_right(),
            Tile::Empty => {
                d = d.turn_left();
                steps += if backtrack { -1 } else { 1 };
            }
            Tile::Oxygen => {
                steps += 1;
                break;
            }
        }
    }
    //println!("{}", print(&hm, &robot, d));

    steps
}

fn flood_step(m: &mut TileMap) {
    let oxypos: Vec<(i64, i64)> = m
        .iter()
        .filter_map(|(&k, &v)| if v == Tile::Oxygen { Some(k) } else { None })
        .collect();

    for pos in oxypos {
        for d in [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ]
        .iter()
        {
            let newpos = d.modify(pos.0, pos.1);
            if let Some(Tile::Empty) = m.get(&newpos) {
                m.insert(newpos, Tile::Oxygen);
            }
        }
    }
}

#[aoc(day15, part2)]
fn flood_oxygen(program: &[i64]) -> usize {
    println!("{}", Debugger::from(program.to_vec()).assembly());

    let mut robot = Robot::new(program);
    let mut hm = HashMap::<(i64, i64), Tile>::new();
    hm.insert((0, 0), Tile::Empty);
    let mut d = Direction::North;

    for _ in 0..10000 {
        match robot.move_command(&mut hm, d) {
            Tile::Wall => d = d.turn_right(),
            _ => d = d.turn_left(),
        }
    }
    // Complete map
    // println!("{}", print(&hm, &robot, d));

    let mut minutes = 0;
    while let Some(_) = hm.iter().find(|(_, &v)| v == Tile::Empty) {
        flood_step(&mut hm);
        minutes += 1;
    }
    minutes
}

#[cfg(test)]
mod tests {
    // No tests in the puzzle text :(
}
