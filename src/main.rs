// temporary, so future days don't give warnings
#![allow(unused_imports)]

pub mod util;
pub mod solution;

pub use solution::*;

fn benchmark(run: fn()) {
    let run_time = std::time::Instant::now();
    run();
    println!("\x1b[2mTime: {} ms\x1b[22m", run_time.elapsed().as_millis());
}

fn main() {
    benchmark(day01::run);
    benchmark(day02::run);
    benchmark(day03::run);
    benchmark(day04::run);
    benchmark(day05::run);
    benchmark(day06::run);
    benchmark(day07::run);
    // benchmark(day08::run);
    // benchmark(day09::run);
    // benchmark(day10::run);
    // benchmark(day11::run);
    // benchmark(day12::run);
    // benchmark(day13::run);
    // benchmark(day14::run);
    // benchmark(day15::run);
    // benchmark(day16::run);
    // benchmark(day17::run);
    // benchmark(day18::run);
    // benchmark(day19::run);
    // benchmark(day20::run);
    // benchmark(day21::run);
    // benchmark(day22::run);
    // benchmark(day23::run);
    // benchmark(day24::run);
    // benchmark(day25::run);
}
