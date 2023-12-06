use super::*;

const INPUT_PART_1: &[(i64, i64)] = &[
    (60, 601),
    (80, 1163),
    (86, 1559),
    (76, 1300),
];

const INPUT_PART_2: (i64, i64) = (60808676, 601116315591300);

pub fn run() {
    let mut win_possibility_product = 1;
    for &(duration, record_distance) in INPUT_PART_1 {
        let mut win_possibilities: u32 = 0;
        for hold_duration in 1..duration {
            let distance = (duration - hold_duration) * hold_duration;
            if distance > record_distance {
                win_possibilities += 1;
            }
        }
        win_possibility_product *= win_possibilities;
    }
    println!("{win_possibility_product}");

    let (duration, record_distance) = INPUT_PART_2;
    let radius = (0.25 * (duration as f64) * (duration as f64) - record_distance as f64).sqrt();
    let lower_bound = (0.5 * (duration as f64) - radius + 1.0).floor() as i64;
    let upper_bound = (0.5 * (duration as f64) + radius - 1.0).ceil() as i64;
    let possibilities = upper_bound - lower_bound + 1;
    println!("{possibilities}");
}
