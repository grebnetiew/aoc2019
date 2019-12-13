extern crate num_integer;

use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::max;
use std::collections::HashMap;

use num_integer::Integer;

#[aoc_generator(day10)]
fn dotpoundmap(input: &str) -> Vec<(isize, isize)> {
    let mut points = Vec::new();
    let mut y = 0;
    input.lines().for_each(|line| {
        let mut x = 0;
        line.chars().for_each(|ch| {
            if ch == '#' {
                points.push((x, y));
            }
            x += 1;
        });
        y += 1;
    });
    points
}

fn find_best_view(input: &[(isize, isize)]) -> ((isize, isize), usize) {
    let mut best_visible = 0;
    let mut best_visible_pos = (0, 0);
    let (max_x, max_y) = input
        .iter()
        .fold((0, 0), |mp, p| (max(mp.0, p.0), max(mp.1, p.1)));

    for &view_point in input {
        let mut visible: HashMap<(isize, isize), bool> = input.iter().map(|&p| (p, true)).collect();
        // Check if any are obscured
        for &other_asteroid in input {
            if view_point == other_asteroid {
                continue;
            }
            let gcd = (other_asteroid.0 - view_point.0).gcd(&(other_asteroid.1 - view_point.1));
            let dx = (other_asteroid.0 - view_point.0) / gcd;
            let dy = (other_asteroid.1 - view_point.1) / gcd;
            let (mut check_x, mut check_y) = other_asteroid;
            loop {
                check_x += dx;
                check_y += dy;
                if !(check_x >= 0 && check_x <= max_x && check_y >= 0 && check_y <= max_y) {
                    break;
                }
                if visible.contains_key(&(check_x, check_y)) {
                    visible.entry((check_x, check_y)).and_modify(|e| *e = false);
                }
            }
        }
        // Count the remaining ones
        let count_visible = visible.iter().filter(|&(_, &v)| v).count();
        if count_visible > best_visible {
            best_visible = count_visible;
            best_visible_pos = view_point
        }
    }
    (best_visible_pos, best_visible)
}

#[aoc(day10, part1)]
fn solver1(input: &[(isize, isize)]) -> usize {
    find_best_view(input).1 - 1 // remove own asteroid
}

#[aoc(day10, part2)]
fn solver2(input: &[(isize, isize)]) -> isize {
    let best_visible_pos = find_best_view(input).0;

    // Find viewing angle and distance of other asteroids
    let mut asteroid_properties = input
        .iter()
        .filter(|&pos| pos != &best_visible_pos)
        .map(|pos| {
            let dx = (pos.0 - best_visible_pos.0) as f64;
            let dy = (pos.1 - best_visible_pos.1) as f64;
            // Angle starts at 0 for positive y-axis and increases clockwise
            // There is a weird rounding error, so add epsilon to all angles..
            let mut angle = dy.atan2(dx) + 2.500_000_000_001 * std::f64::consts::PI;
            // Modulo seems to work strangely or not at all for floats...
            while angle > 2. * std::f64::consts::PI {
                angle -= 2. * std::f64::consts::PI;
            }
            let distance_sq = dx * dx + dy * dy;
            (*pos, angle, distance_sq)
        })
        .collect::<Vec<((isize, isize), f64, f64)>>();

    // Sort them by angle, then by distance
    asteroid_properties.sort_by(|a, b| {
        a.1.partial_cmp(&b.1)
            .unwrap()
            .then(a.2.partial_cmp(&b.2).unwrap())
    });

    // Now divide them into rounds, as in shot by laser on the nth round
    let mut rounds: Vec<Vec<(isize, isize)>> = vec![vec![]];
    let mut last_angle = -1.;
    let mut round_number = 0;
    for entry in asteroid_properties {
        // Equal angle asteroids are shot in order of distance
        // So put each consecutive asteroid in a new 'round'
        if (entry.1 - last_angle).abs() < std::f64::EPSILON {
            round_number += 1;
            // Maybe make room to store more rounds
            if rounds.len() <= round_number {
                rounds.push(vec![]);
            }
        } else {
            round_number = 0;
        }
        rounds[round_number].push(entry.0);
        last_angle = entry.1;
    }

    // Finally, find the round in which the 200th asteroid is shot
    // and then find out which one it is in that round
    round_number = 0;
    let mut shot_so_far = 0;
    while shot_so_far + rounds[round_number].len() < 200 {
        shot_so_far += rounds[round_number].len();
        round_number += 1;
    }
    let asteroid_200_pos = rounds[round_number][199 - shot_so_far];
    // If found, return position as "xxyy"
    100 * asteroid_200_pos.0 + asteroid_200_pos.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run1a() {
        assert_eq!(solver1(&dotpoundmap("......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####")), 33);
        assert_eq!(solver1(&dotpoundmap("#.#...#.#.\n.###....#.\n.#....#...\n##.#.#.#.#\n....#.#.#.\n.##..###.#\n..#...##..\n..##....##\n......#...\n.####.###.")), 35);
        assert_eq!(solver1(&dotpoundmap(".#..#..###\n####.###.#\n....###.#.\n..###.##.#\n##.##.#.#.\n....###..#\n..#.#..#.#\n#..#.#.###\n.##...##.#\n.....#.#..")), 41);
        assert_eq!(solver1(&dotpoundmap(".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##")), 210);
    }

    #[test]
    fn test_run2() {
        assert_eq!(solver2(&dotpoundmap(".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##")), 802);
    }

}
