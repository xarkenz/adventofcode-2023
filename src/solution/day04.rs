use super::*;

use std::collections::VecDeque;

pub fn run() {
    let mut scratch_card_point_sum: u32 = 0;

    let scratch_card_matches: Vec<u32> = get_input("day04.txt").lines().map(expect_line).map(|line| {
        let mut split = line.split_ascii_whitespace();
        split.next();
        split.next();

        let mut winning_numbers = Vec::new();
        loop {
            match split.next() {
                Some("|") => break,
                Some(number) => {
                    winning_numbers.push(number.parse::<u32>().unwrap());
                },
                None => panic!(),
            }
        }

        let mut matches: u32 = 0;
        while let Some(number) = split.next() {
            let number = number.parse::<u32>().unwrap();
            if winning_numbers.contains(&number) {
                matches += 1;
            }
        }

        let points = if matches == 0 { 0 } else { 1 << (matches - 1) };
        scratch_card_point_sum += points;

        matches
    }).collect();

    let mut scratch_cards: VecDeque<u32> = (0 .. scratch_card_matches.len() as u32).collect();
    let mut scratch_card_count: u32 = 0;
    while let Some(index) = scratch_cards.pop_front() {
        scratch_card_count += 1;
        let matches = scratch_card_matches[index as usize];
        scratch_cards.extend((index + 1) ..= (index + matches));
    }

    println!("[04p1] Sum of scratch card points: {scratch_card_point_sum}");
    println!("[04p2] Number of scratch cards: {scratch_card_count}");
}
