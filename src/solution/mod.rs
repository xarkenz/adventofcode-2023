pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

use crate::util::*;

use std::io::BufRead;
use std::time::Instant;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::collections::BinaryHeap;

pub fn get_input(name: &'static str) -> std::io::BufReader<std::fs::File> {
    std::io::BufReader::new(std::fs::File::open(format!("./src/input/{name}"))
        .expect("unable to open input file"))
}

pub fn expect_line(result: std::io::Result<String>) -> String {
    result.expect("error while reading input file")
}

pub fn expect_bytes(result: std::io::Result<Vec<u8>>) -> Vec<u8> {
    result.expect("error while reading input file")
}

pub fn print_elapsed_time(label: &str, since: Instant) {
    println!("\x1b[2m{label}: {} ms\x1b[22m", since.elapsed().as_millis());
}
