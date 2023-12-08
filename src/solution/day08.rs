use super::*;

use std::collections::BTreeMap;

#[derive(Copy, Clone, Debug)]
enum Step {
    Left,
    Right,
}

impl From<char> for Step {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("{value}")
        }
    }
}

pub fn run() {
    let mut lines = get_input("day08.txt").lines().map(expect_line);

    let steps_line = lines.next().unwrap();
    let get_steps = || steps_line.chars().map(Step::from);

    let mut forks: BTreeMap<String, (String, String)> = BTreeMap::new();
    let mut ghostly_locations: Vec<String> = Vec::new();

    for line in lines {
        if !line.is_empty() {
            let fork_location = line[0..3].to_owned();
            let left = line[7..10].to_owned();
            let right = line[12..15].to_owned();

            if fork_location.ends_with('A') {
                ghostly_locations.push(fork_location.clone())
            }

            forks.insert(fork_location, (left, right));
        }
    }

    let mut corporeal_location = String::from("AAA");
    let mut corporeal_steps_taken: u64 = 0;

    let mut steps = get_steps();
    while &corporeal_location != "ZZZ" {
        let (left, right) = forks.get(&corporeal_location).unwrap();

        corporeal_location = match steps.next().unwrap() {
            Step::Left => left.clone(),
            Step::Right => right.clone(),
        };
        corporeal_steps_taken += 1;
    }

    println!("[08p1] Steps taken: {corporeal_steps_taken}");

    // The fact that this works frustrates me to no end. The problem gives *no* guarantee
    // that the L/R steps and the paths will line up perfectly to make each ghost
    // periodically visit the same ending location, and yet that is the case.
    // The way I found out was by printing when and where ghosts visited ending locations.
    // TL;DR: This is not a general solution, but it works for the input given.

    let mut ghostly_steps_lcm = 1;

    for ghost_location in &ghostly_locations {
        let mut ghost_location = ghost_location.clone();
        let mut ghost_steps_taken = 0;

        let mut steps = get_steps();
        while !ghost_location.ends_with('Z') {
            let (left, right) = forks.get(&ghost_location).unwrap();
            
            ghost_location = match steps.next().unwrap() {
                Step::Left => left.clone(),
                Step::Right => right.clone(),
            };
            ghost_steps_taken += 1;
        }

        ghostly_steps_lcm = lcm(ghostly_steps_lcm, ghost_steps_taken);
    }

    println!("[08p2] Ghostly steps taken: {ghostly_steps_lcm}");
}
