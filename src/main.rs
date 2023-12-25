// temporary, so future days don't give warnings
#![allow(unused_imports)]
#![allow(dead_code)]

pub mod util;
pub mod solution;

pub use solution::*;

fn main() {
    let start_time = std::time::Instant::now();

    day01::run();
    day02::run();
    day03::run();
    day04::run();
    day05::run();
    day06::run();
    day07::run();
    day08::run();
    day09::run();
    day10::run();
    day11::run();
    day12::run();
    day13::run();
    day14::run();
    day15::run();
    day16::run();
    day17::run();
    day18::run();
    day19::run();
    day20::run();
    // day21::run();
    // day22::run();
    // day23::run();
    day24::run();
    day25::run();

    print_elapsed_time("Finished in", start_time);
}
