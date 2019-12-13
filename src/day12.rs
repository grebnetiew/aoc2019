use aoc_runner_derive::{aoc, aoc_generator};
use num_integer::Integer;
use std::num::ParseIntError;

#[aoc_generator(day12)]
fn positions(input: &str) -> Result<Vec<Vec<i64>>, ParseIntError> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter(|&c| c.is_numeric() || c == ',' || c == '-')
                .collect::<String>()
                .split(',')
                .map(str::parse)
                .collect()
        })
        .collect()
}

fn one_coord_one_round(positions: &mut Vec<i64>, velocities: &mut Vec<i64>) {
    for i in 0..4 {
        for j in 0..4 {
            velocities[i] += (positions[j] - positions[i]).signum();
        }
    }

    for i in 0..4 {
        positions[i] += velocities[i];
    }
}

fn coordinate_loop(starting_pos: &[i64], n_rounds: usize) -> (Vec<i64>, Vec<i64>) {
    let mut positions = starting_pos.to_vec();
    let mut velocities = vec![0, 0, 0, 0];

    for _round in 0..n_rounds {
        one_coord_one_round(&mut positions, &mut velocities);
    }
    (positions, velocities)
}

fn transpose(m: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let len_outer = m.len();
    let len_inner = m[0].len();
    (0..len_inner)
        .map(|i| (0..len_outer).map(|j| m[j][i]).collect())
        .collect()
}

fn energy_after_rounds(positions: &[Vec<i64>], rounds: usize) -> i64 {
    let starting_positions_per_coord = transpose(positions);

    // Simulate some rounds
    let (pos_x, vel_x) = coordinate_loop(&starting_positions_per_coord[0], rounds);
    let (pos_y, vel_y) = coordinate_loop(&starting_positions_per_coord[1], rounds);
    let (pos_z, vel_z) = coordinate_loop(&starting_positions_per_coord[2], rounds);

    // Calculate energy
    (0..4)
        .map(|i| {
            (pos_x[i].abs() + pos_y[i].abs() + pos_z[i].abs())
                * (vel_x[i].abs() + vel_y[i].abs() + vel_z[i].abs())
        })
        .sum()
}

#[aoc(day12, part1)]
fn solver1(positions: &[Vec<i64>]) -> i64 {
    energy_after_rounds(positions, 1000)
}

fn coordinate_loop_length(starting_pos: &[i64]) -> usize {
    let starting_vel = [0, 0, 0, 0];
    let mut positions = starting_pos.to_vec();
    let mut velocities = starting_vel.to_vec();

    let mut rounds = 0;
    loop {
        one_coord_one_round(&mut positions, &mut velocities);
        rounds += 1;

        if positions == starting_pos && velocities == starting_vel {
            return rounds;
        }
    }
}

#[aoc(day12, part2)]
fn solver2(positions: &[Vec<i64>]) -> usize {
    let loop_lengths: Vec<_> = (0..3)
        .map(|i| {
            coordinate_loop_length(&positions.iter().map(|point| point[i]).collect::<Vec<_>>())
        })
        .collect();
    loop_lengths[0].lcm(&loop_lengths[1]).lcm(&loop_lengths[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_energy_after_rounds() {
        assert_eq!(
            energy_after_rounds(
                &positions(
                    "<x=-1, y=0, z=2>\n<x=2, y=-10, z=-7>\n<x=4, y=-8, z=8>\n<x=3, y=5, z=-1>"
                )
                .unwrap(),
                10
            ),
            179
        );
        assert_eq!(
            energy_after_rounds(
                &positions(
                    "<x=-8, y=-10, z=0>\n<x=5, y=5, z=10>\n<x=2, y=-7, z=3>\n<x=9, y=-8, z=-3>"
                )
                .unwrap(),
                100
            ),
            1940
        );
    }

    #[test]
    fn test_loop_lengths() {
        assert_eq!(
            solver2(
                &positions(
                    "<x=-8, y=-10, z=0>\n<x=5, y=5, z=10>\n<x=2, y=-7, z=3>\n<x=9, y=-8, z=-3>"
                )
                .unwrap()
            ),
            4_686_774_924
        );
    }
}
