use crate::intcode::Computer;
use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day23)]
fn one_line_many_numbers(input: &str) -> Result<Vec<i64>, ParseIntError> {
    input.split(',').map(str::parse).collect()
}

struct NetPC {
    computer: Computer,
    out_len: usize,
    waiting: bool,
}

impl NetPC {
    fn new(program: &[i64], ip: i64) -> Self {
        Self {
            computer: Computer::new(program.to_vec(), vec![ip]),
            out_len: 0,
            waiting: false,
        }
    }
    fn step(&mut self) {
        let mut is_waiting = false;
        self.out_len = self.computer.advance_one_step_with(|| {
            is_waiting = true;
            -1
        });
        if is_waiting {
            self.waiting = true;
        }
    }
    fn has_packet(&self) -> bool {
        self.out_len >= 3
    }
    fn give_packet(&mut self, packet: (i64, i64)) {
        self.computer.more_input(packet.0);
        self.computer.more_input(packet.1);
        self.waiting = false;
    }
    fn output(&mut self) -> Vec<i64> {
        self.computer.output()
    }
}

#[aoc(day23, part1)]
fn solver1(program: &[i64]) -> i64 {
    let mut network: Vec<NetPC> = (0..50).map(|ip| NetPC::new(program, ip)).collect();
    loop {
        let mut packets: Vec<Vec<i64>> = Vec::new();
        for pc in network.iter_mut() {
            pc.step();
            if pc.has_packet() {
                // There'll be precisely 3 values, since we only step one at a time
                packets.push(pc.output());
            }
        }
        for packet in packets.iter() {
            if packet[0] == 255 {
                return packet[2];
            }
            network[packet[0] as usize].give_packet((packet[1], packet[2]));
        }
    }
}

#[aoc(day23, part2)]
fn solver2(program: &[i64]) -> i64 {
    let mut network: Vec<NetPC> = (0..50).map(|ip| NetPC::new(program, ip)).collect();
    let mut nat = (0, 0);
    let mut last_y_delivered = -1;
    loop {
        let mut packets: Vec<Vec<i64>> = Vec::new();
        for pc in network.iter_mut() {
            pc.step();
            if pc.has_packet() {
                // There'll be precisely 3 values, since we only step one at a time
                packets.push(pc.output());
            }
        }
        for packet in packets.iter() {
            if packet[0] == 255 {
                nat = (packet[1], packet[2]);
            } else {
                network[packet[0] as usize].give_packet((packet[1], packet[2]));
            }
        }
        if network.iter().map(|pc| pc.waiting).all(|x| x) {
            if last_y_delivered == nat.1 {
                return last_y_delivered;
            }
            network[0].give_packet(nat);
            last_y_delivered = nat.1;
        }
    }
}

#[cfg(test)]
mod tests {
    // No tests in the puzzle text :(
}
