use super::*;

use std::collections::BTreeMap;

pub fn run() {
    let mut lines = get_input("day08.txt").lines().map(expect_line);
    let instructions = lines.next().unwrap();

    let mut location = String::from("AAA");
    let mut ghost_locations: Vec<String> = Vec::new();
    let mut ghost_targets: Vec<String> = Vec::new();
    let mut forks: BTreeMap<String, (String, String)> = BTreeMap::new();

    for line in lines {
        if !line.is_empty() {
            let from_location = line[0..3].to_owned();
            let left = line[7..10].to_owned();
            let right = line[12..15].to_owned();

            if from_location.ends_with('A') {
                ghost_locations.push(from_location.clone())
            }
            if from_location.ends_with('Z') {
                ghost_targets.push(from_location.clone())
            }

            forks.insert(from_location, (left, right));
        }
    }

    let mut steps_taken: u64 = 0;

    let mut go_left = instructions.chars().cycle().map(|ch| ch == 'L');
    while &location != "ZZZ" {
        let (left, right) = forks.get(&location).unwrap();
        if go_left.next().unwrap() {
            location = left.clone();
        }
        else {
            location = right.clone();
        }
        steps_taken += 1;
    }

    println!("{steps_taken}");

    let mut simultaneous_steps = 1;

    for location in &ghost_locations {
        let mut location = location.clone();
        let mut steps_taken = 0;

        let mut go_left = instructions.chars().cycle().map(|ch| ch == 'L');
        while !location.ends_with('Z') {
            let (left, right) = forks.get(&location).unwrap();
            if go_left.next().unwrap() {
                location = left.clone();
            }
            else {
                location = right.clone();
            }
            steps_taken += 1;
        }

        let divisor = gcd(simultaneous_steps, steps_taken);
        simultaneous_steps *= steps_taken;
        simultaneous_steps /= divisor;
    }

    println!("{simultaneous_steps}");
}
