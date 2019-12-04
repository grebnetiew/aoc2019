use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::num::ParseIntError;

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn apply_one_step_to(&self, x: &mut i32, y: &mut i32) {
        match self {
            Direction::Left => *x -= 1,
            Direction::Right => *x += 1,
            Direction::Up => *y -= 1,
            Direction::Down => *y += 1,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Move {
    direction: Direction,
    distance: u32,
}

impl Move {
    fn new(direction: Direction, distance: u32) -> Self {
        Self {
            direction,
            distance,
        }
    }

    fn maybe_new(dir: Direction, dis: Result<u32, ParseIntError>) -> Result<Self, ParseIntError> {
        Ok(Self::new(dir, dis?))
    }
}

#[aoc_generator(day3)]
fn lines_to_moves(input: &str) -> Result<Vec<Vec<Move>>, ParseIntError> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|segment| {
                    let (letter, number) = segment.split_at(1);
                    let dir = match letter {
                        "L" => Direction::Left,
                        "R" => Direction::Right,
                        "U" => Direction::Up,
                        _ => Direction::Down,
                    };
                    Move::maybe_new(dir, number.parse())
                })
                .collect()
        })
        .collect()
}

fn crossings(input: &Vec<Vec<Move>>) -> HashMap<(i32, i32), u32> {
    let mut hm = HashMap::new();
    // add first path
    let (mut x, mut y) = (0i32, 0i32);
    let mut wire = 0;
    for &m in &input[0] {
        for _d in 0..m.distance {
            m.direction.apply_one_step_to(&mut x, &mut y);
            wire += 1;
            hm.entry((x, y)).or_insert(wire);
        }
    }

    let mut crossings = HashMap::new();
    // second path
    x = 0;
    y = 0;
    wire = 0;
    for &m in &input[1] {
        for _d in 0..m.distance {
            m.direction.apply_one_step_to(&mut x, &mut y);
            wire += 1;
            if hm.contains_key(&(x, y)) {
                let dist = wire + hm[&(x, y)];
                crossings.entry((x, y)).or_insert(dist);
            }
        }
    }

    crossings
}

#[aoc(day3, part1)]
fn solver1(input: &Vec<Vec<Move>>) -> u32 {
    crossings(input)
        .keys()
        .map(|&(x, y)| x.abs() as u32 + y.abs() as u32)
        .min()
        .unwrap()
}

#[aoc(day3, part2)]
fn solver2(input: &Vec<Vec<Move>>) -> u32 {
    *crossings(input).values().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run1() {
        assert_eq!(
            solver1(&lines_to_moves("R8,U5,L5,D3\nU7,R6,D4,L4").unwrap()),
            6
        );
        assert_eq!(
            solver1(
                &lines_to_moves(
                    "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"
                )
                .unwrap()
            ),
            159
        );
        assert_eq!(solver1(&lines_to_moves("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7").unwrap()), 135);
    }

    #[test]
    fn test_run2() {
        assert_eq!(
            solver2(&lines_to_moves("R8,U5,L5,D3\nU7,R6,D4,L4").unwrap()),
            30
        );
        assert_eq!(
            solver2(
                &lines_to_moves(
                    "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"
                )
                .unwrap()
            ),
            610
        );
        assert_eq!(solver2(&lines_to_moves("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7").unwrap()), 410);
    }
}
