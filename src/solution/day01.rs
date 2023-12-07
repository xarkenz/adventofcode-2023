use super::*;

pub fn run() {
    let lines = get_input("day01.txt").lines()
        .map(expect_line);

    let mut calibration_sum = 0;
    let mut real_calibration_sum = 0;

    for line in lines {
        if let Some(first_digit) = line.bytes().find(u8::is_ascii_digit) {
            let last_digit = line.bytes().rfind(u8::is_ascii_digit).unwrap();
            calibration_sum += (first_digit - b'0') as u32 * 10 + (last_digit - b'0') as u32;
        }

        // After the fact, I was made aware that there's a better solution to the problem of overlapping digits,
        // which is to replace each digit word while preserving the first and last characters.
        // That is to say, replace "one" with "o1e", "two" with "t2o", "three" with "t3e", and so on.
        // Below is the alternate approach I originally used to solve the problem.
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
    
    println!("[01p1] Sum of calibration values (digits only): {calibration_sum}");
    println!("[01p2] Sum of calibration values (digits and words): {real_calibration_sum}");
}