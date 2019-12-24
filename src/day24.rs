use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[aoc_generator(day24)]
fn dotpoundmap(input: &str) -> HashMap<(i32, i32), Tile> {
    let mut points = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, _) in line.chars().enumerate().filter(|&(_, ch)| ch == '#') {
            points.insert((x as i32, y as i32), Tile::Bug);
        }
    }
    points
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    #[allow(dead_code)]
    Empty,
    Bug,
}

fn map_to_num(hm: &HashMap<(i32, i32), Tile>) -> u32 {
    let mut result = 0;
    for i in 0..25 {
        if let Some(Tile::Bug) = hm.get(&(i % 5, i / 5)) {
            result += 1 << i;
        }
    }
    result
}

fn evolve(hm: &HashMap<(i32, i32), Tile>) -> HashMap<(i32, i32), Tile> {
    let mut new_map = HashMap::new();
    for i in 0..5 {
        for j in 0..5 {
            let mut adjacent = 0;
            if let Some(Tile::Bug) = hm.get(&(i - 1, j)) {
                adjacent += 1;
            }
            if let Some(Tile::Bug) = hm.get(&(i + 1, j)) {
                adjacent += 1;
            }
            if let Some(Tile::Bug) = hm.get(&(i, j - 1)) {
                adjacent += 1;
            }
            if let Some(Tile::Bug) = hm.get(&(i, j + 1)) {
                adjacent += 1;
            }
            match hm.get(&(i, j)) {
                Some(Tile::Bug) => {
                    // It dies unless adjacent == 1
                    if adjacent == 1 {
                        new_map.insert((i, j), Tile::Bug);
                    };
                }
                _ => {
                    // A new one is adjacent in 1, 2
                    if adjacent == 1 || adjacent == 2 {
                        new_map.insert((i, j), Tile::Bug);
                    };
                }
            }
        }
    }
    new_map
}

#[aoc(day24, part1)]
fn solver1(program: &HashMap<(i32, i32), Tile>) -> u32 {
    let mut seen = HashMap::<u32, bool>::new();
    let mut current_map: HashMap<(i32, i32), Tile> = (*program).clone();
    while let None = seen.get(&map_to_num(&current_map)) {
        seen.insert(map_to_num(&current_map), true);
        current_map = evolve(&current_map);
    }
    map_to_num(&current_map)
}

fn adjacent(pos: (i32, i32, i32)) -> Vec<(i32, i32, i32)> {
    let mut result = Vec::new();
    let (d, x, y) = pos;
    // left
    if x == 0 {
        result.push((d - 1, 1, 2));
    } else if x == 3 && y == 2 {
        for j in 0..5 {
            result.push((d + 1, 4, j));
        }
    } else {
        result.push((d, x - 1, y));
    }
    // right
    if x == 4 {
        result.push((d - 1, 3, 2));
    } else if x == 1 && y == 2 {
        for j in 0..5 {
            result.push((d + 1, 0, j));
        }
    } else {
        result.push((d, x + 1, y));
    }
    // up
    if y == 0 {
        result.push((d - 1, 2, 1));
    } else if y == 3 && x == 2 {
        for i in 0..5 {
            result.push((d + 1, i, 4));
        }
    } else {
        result.push((d, x, y - 1));
    }
    // down
    if y == 4 {
        result.push((d - 1, 2, 3));
    } else if y == 1 && x == 2 {
        for i in 0..5 {
            result.push((d + 1, i, 0));
        }
    } else {
        result.push((d, x, y + 1));
    }
    result
}

fn evolve2(hm: &HashMap<(i32, i32, i32), Tile>) -> HashMap<(i32, i32, i32), Tile> {
    let mut new_map = HashMap::new();
    let min_depth = hm.keys().map(|k| k.0).min().unwrap();
    let max_depth = hm.keys().map(|k| k.0).max().unwrap();
    for d in (min_depth - 1)..=(max_depth + 1) {
        for i in 0..5 {
            for j in 0..5 {
                if i == 2 && j == 2 {
                    continue;
                }
                let adj = adjacent((d, i, j)).iter().filter_map(|p| hm.get(p)).count();
                match hm.get(&(d, i, j)) {
                    Some(Tile::Bug) => {
                        // It dies unless adjacent == 1
                        if adj == 1 {
                            new_map.insert((d, i, j), Tile::Bug);
                        };
                    }
                    _ => {
                        // A new one is adjacent in 1, 2
                        if adj == 1 || adj == 2 {
                            new_map.insert((d, i, j), Tile::Bug);
                        };
                    }
                }
            }
        }
    }
    new_map
}

#[aoc(day24, part2)]
fn solver2(program: &HashMap<(i32, i32), Tile>) -> usize {
    inner_solver2(program, 200)
}

fn inner_solver2(program: &HashMap<(i32, i32), Tile>, amount: usize) -> usize {
    let mut current_map: HashMap<(i32, i32, i32), Tile> =
        program.iter().map(|(k, &v)| ((0, k.0, k.1), v)).collect();
    for _ in 0..amount {
        current_map = evolve2(&current_map);
    }
    current_map.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run1() {
        assert_eq!(
            solver1(&dotpoundmap("....#\n#..#.\n#..##\n..#..\n#....")),
            2_129_920
        );
    }

    #[test]
    fn test_adjacent() {
        assert_eq!(adjacent((0, 3, 3)).len(), 4);
        assert_eq!(adjacent((0, 3, 0)).len(), 4);
        assert_eq!(adjacent((0, 4, 0)).len(), 4);
        assert_eq!(adjacent((0, 0, 0)).len(), 4);
        assert_eq!(adjacent((0, 2, 2)).len(), 4);
        assert_eq!(adjacent((0, 0, 4)).len(), 4);
        assert_eq!(adjacent((0, 3, 2)).len(), 8);
        assert_eq!(adjacent((0, 2, 3)).len(), 8);
        assert_eq!(adjacent((0, 1, 2)).len(), 8);
        assert_eq!(adjacent((0, 2, 1)).len(), 8);
    }

    #[test]
    fn test_run2() {
        assert_eq!(
            inner_solver2(&dotpoundmap("....#\n#..#.\n#..##\n..#..\n#...."), 10),
            99
        );
    }
}
