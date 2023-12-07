use super::*;

const INPUT_PART_1: &[(i64, i64)] = &[
    (60, 601),
    (80, 1163),
    (86, 1559),
    (76, 1300),
];

const INPUT_PART_2: (i64, i64) = (60808676, 601116315591300);

pub fn run() {
    let mut win_count_product = 1;

    for &(duration, record_distance) in INPUT_PART_1 {
        let mut win_count: u32 = 0;
        for hold_duration in 1..duration {
            let distance = (duration - hold_duration) * hold_duration;
            if distance > record_distance {
                win_count += 1;
            }
        }
        win_count_product *= win_count;
    }

    println!("[06p1] Product of ways to win each race: {win_count_product}");

    let (duration, record_distance) = INPUT_PART_2;
    
    let quadratic_radius = (0.25 * (duration as f64) * (duration as f64) - record_distance as f64).sqrt();
    let win_count_lower_bound = (0.5 * (duration as f64) - quadratic_radius + 1.0).floor() as i64;
    let win_count_upper_bound = (0.5 * (duration as f64) + quadratic_radius - 1.0).ceil() as i64;
    let win_count = win_count_upper_bound - win_count_lower_bound + 1;

    println!("[06p2] Number of ways to win longer race: {win_count}");
}
