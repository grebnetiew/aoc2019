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
                .map(|n| n.parse())
                .collect()
        })
        .collect()
}

#[aoc(day12, part1)]
fn solver1(positions: &[Vec<i64>]) -> i64 {
    let mut positions = positions.to_vec();
    let mut velocities = vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
    for _n in 0..1000 {
        // calculate velocities
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..3 {
                    if positions[i][k] != positions[j][k] {
                        if positions[i][k] < positions[j][k] {
                            velocities[i][k] += 1;
                        } else {
                            velocities[i][k] -= 1;
                        }
                    }
                }
            }
        }
        // Update positions
        for i in 0..4 {
            for k in 0..3 {
                positions[i][k] += velocities[i][k];
            }
        }
    }
    // Calculate energy
    (0..4)
        .map(|i| -> i64 {
            positions[i].iter().cloned().map(i64::abs).sum::<i64>()
                * velocities[i].iter().cloned().map(i64::abs).sum::<i64>()
        })
        .sum()
}

#[aoc(day12, part2)]
fn solver2(positions: &[Vec<i64>]) -> i64 {
    let mut positions = positions.to_vec();
    let mut velocities = vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];

    let mut px_hist = Vec::new();
    let mut py_hist = Vec::new();
    let mut pz_hist = Vec::new();
    let mut vx_hist = Vec::new();
    let mut vy_hist = Vec::new();
    let mut vz_hist = Vec::new();

    let (mut loop_px, mut loop_py, mut loop_pz) = (0i64, 0i64, 0i64);
    let (mut loop_vx, mut loop_vy, mut loop_vz) = (0i64, 0i64, 0i64);
    let mut n = 0;
    while loop_px == 0
        || loop_py == 0
        || loop_pz == 0
        || loop_vx == 0
        || loop_vy == 0
        || loop_vz == 0
    {
        // calculate velocities
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..3 {
                    if positions[i][k] != positions[j][k] {
                        if positions[i][k] < positions[j][k] {
                            velocities[i][k] += 1;
                        } else {
                            velocities[i][k] -= 1;
                        }
                    }
                }
            }
        }
        // Update positions
        for i in 0..4 {
            for k in 0..3 {
                positions[i][k] += velocities[i][k];
            }
        }
        let pos_x = (0..4).map(|i| positions[i][0]).collect::<Vec<_>>();
        let pos_y = (0..4).map(|i| positions[i][1]).collect::<Vec<_>>();
        let pos_z = (0..4).map(|i| positions[i][2]).collect::<Vec<_>>();

        let vel_x = (0..4).map(|i| velocities[i][0]).collect::<Vec<_>>();
        let vel_y = (0..4).map(|i| velocities[i][1]).collect::<Vec<_>>();
        let vel_z = (0..4).map(|i| velocities[i][2]).collect::<Vec<_>>();

        if loop_px == 0 {
            if let Some(i) = px_hist.iter().position(|v| v == &pos_x) {
                loop_px = n;
                println!("Loop px is ({}, {}) {}", i, n, n - i as i64);
            }
        }
        if loop_py == 0 {
            if let Some(i) = py_hist.iter().position(|v| v == &pos_y) {
                loop_py = n;
                println!("Loop py is ({}, {}) {}", i, n, n - i as i64);
            }
        }
        if loop_pz == 0 {
            if let Some(i) = pz_hist.iter().position(|v| v == &pos_z) {
                loop_pz = n;
                println!("Loop pz is ({}, {}) {}", i, n, n - i as i64);
            }
        }
        if loop_vx == 0 {
            if let Some(i) = vx_hist.iter().position(|v| v == &vel_x) {
                loop_vx = n;
                println!("Loop vx is ({}, {}) {}", i, n, n - i as i64);
            }
        }
        if loop_vy == 0 {
            if let Some(i) = vy_hist.iter().position(|v| v == &vel_y) {
                loop_vy = n;
                println!("Loop vy is ({}, {}) {}", i, n, n - i as i64);
            }
        }
        if loop_vz == 0 {
            if let Some(i) = vz_hist.iter().position(|v| v == &vel_z) {
                loop_vz = n;
                println!("Loop vz is ({}, {}) {}", i, n, n - i as i64);
            }
        }

        px_hist.push(pos_x);
        py_hist.push(pos_y);
        pz_hist.push(pos_z);
        vx_hist.push(vel_x);
        vy_hist.push(vel_y);
        vz_hist.push(vel_z);
        n += 1;
    }
    loop_px
        .lcm(&loop_py)
        .lcm(&loop_pz)
        .lcm(&loop_vx)
        .lcm(&loop_vy)
        .lcm(&loop_vz)
}

#[cfg(test)]
mod tests {
    // No tests in the puzzle text :(
}
