use super::*;

fn extrapolate(sequence: &[i64]) -> (i64, i64) {
    if sequence.len() <= 1 {
        (sequence[0], sequence[0])
    }
    else {
        let mut differences = Vec::new();
        for index in 1..sequence.len() {
            differences.push(sequence[index] - sequence[index - 1]);
        }
        if differences.iter().all(|difference| difference == &0) {
            (sequence[0], sequence[0])
        }
        else {
            let (backward_value, forward_value) = extrapolate(&differences);
            (sequence[0] - backward_value, sequence.last().unwrap() + forward_value)
        }
    }
}

pub fn run() {
    let lines = get_input("day09.txt").lines().map(expect_line);

    let mut forward_extrapolation_sum = 0;
    let mut backward_extrapolation_sum = 0;

    for line in lines {
        let values: Vec<i64> = line.split_whitespace().map(|number| number.parse::<i64>().unwrap()).collect();

        let (backward_value, forward_value) = extrapolate(&values);
        forward_extrapolation_sum += forward_value;
        backward_extrapolation_sum += backward_value;
    }

    println!("[09p1] Sum of forward-extrapolated values: {forward_extrapolation_sum}");
    println!("[09p2] Sum of backward-extrapolated values: {backward_extrapolation_sum}");
}
