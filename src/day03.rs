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
    fn mv(&self, x: &mut i32, y: &mut i32) {
        match self {
            Direction::Left => *x -= 1,
            Direction::Right => *x += 1,
            Direction::Up => *y -= 1,
            Direction::Down => *y += 1,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Move(Direction, u32);

#[aoc_generator(day3)]
fn lines_to_moves(input: &str) -> Result<Vec<Vec<Move>>, ParseIntError> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|segment| -> Result<Move, ParseIntError> {
                    let (letter, number) = segment.split_at(1);
                    let dir = match letter {
                        "L" => Direction::Left,
                        "R" => Direction::Right,
                        "U" => Direction::Up,
                        _ => Direction::Down,
                    };
                    Ok(Move(dir, number.parse()?))
                })
                .collect::<Result<Vec<Move>, ParseIntError>>()
        })
        .collect()
}

#[aoc(day3, part1)]
fn solver1(input: &Vec<Vec<Move>>) -> u32 {
    let mut hm = HashMap::new();
    // add first path
    let (mut x, mut y) = (0i32, 0i32);
    for &m in &input[0] {
        let mut d = m.1;
        while d > 0 {
            m.0.mv(&mut x, &mut y);
            d -= 1;
            hm.insert((x, y), 1);
        }
    }

    let mut closest_dist = 999999;
    // second path
    x = 0;
    y = 0;
    for &m in &input[1] {
        let mut d = m.1;
        while d > 0 {
            m.0.mv(&mut x, &mut y);
            d -= 1;
            if hm.contains_key(&(x, y)) {
                let dist: u32 = (x.abs() + y.abs()) as u32;
                if closest_dist > dist {
                    closest_dist = dist;
                }
            }
        }
    }

    closest_dist
}

#[aoc(day3, part2)]
fn solver2(input: &Vec<Vec<Move>>) -> u32 {
    let mut hm = HashMap::new();
    // add first path
    let (mut x, mut y) = (0i32, 0i32);
    let mut wire = 0;
    for &m in &input[0] {
        let mut d = m.1;
        while d > 0 {
            m.0.mv(&mut x, &mut y);
            d -= 1;
            wire += 1;
            if !hm.contains_key(&(x, y)) {
                hm.insert((x, y), wire);
            }
        }
    }

    let mut closest_dist = 999999;
    // second path
    x = 0;
    y = 0;
    wire = 0;
    for &m in &input[1] {
        let mut delta = m.1;
        while delta > 0 {
            m.0.mv(&mut x, &mut y);
            delta -= 1;
            wire += 1;
            if hm.contains_key(&(x, y)) {
                let dist: u32 = (wire + hm[&(x, y)]) as u32;
                if closest_dist > dist {
                    closest_dist = dist;
                }
            }
        }
    }

    closest_dist
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
