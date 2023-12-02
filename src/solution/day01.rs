use super::*;

pub fn run() {
    let input = get_input("day01.txt");
    let mut calibration_sum = 0;
    let mut real_calibration_sum = 0;
    for line in input.lines().map(expect_line) {
        if let Some(first_digit) = line.bytes().find(u8::is_ascii_digit) {
            let last_digit = line.bytes().rfind(u8::is_ascii_digit).unwrap();
            calibration_sum += (first_digit - b'0') as u32 * 10 + (last_digit - b'0') as u32;
        }
        let line = line
            .replace("oneight", "oneeight")
            .replace("twone", "twoone")
            .replace("threeight", "threeeight")
            .replace("fiveight", "fiveeight")
            .replace("sevenine", "sevennine")
            .replace("eightwo", "eighttwo")
            .replace("eighthree", "eightthree")
            .replace("nineight", "nineeight")
            .replace("one", "1")
            .replace("two", "2")
            .replace("three", "3")
            .replace("four", "4")
            .replace("five", "5")
            .replace("six", "6")
            .replace("seven", "7")
            .replace("eight", "8")
            .replace("nine", "9");
        if let Some(first_digit) = line.bytes().find(u8::is_ascii_digit) {
            let last_digit = line.bytes().rfind(u8::is_ascii_digit).unwrap();
            real_calibration_sum += (first_digit - b'0') as u32 * 10 + (last_digit - b'0') as u32;
        }
    }
    println!("[01p1] Sum of calibration values: {calibration_sum}");
    println!("[01p2] Real sum of calibration values: {real_calibration_sum}");
}