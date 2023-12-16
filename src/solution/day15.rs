use super::*;

fn hash(step: &str) -> usize {
    let mut current_value = 0;

    for ch in step.chars() {
        current_value += ch as usize;
        current_value *= 17;
        current_value %= 256;
    }

    current_value
}

pub fn run() {
    let steps = get_input("day15.txt").split(b',').map(expect_bytes);
    
    let mut hash_sum = 0;
    let mut boxes: Vec<Vec<(String, usize)>> = std::iter::repeat_with(Vec::new).take(256).collect();

    for step in steps {
        let mut step: String = String::from_utf8_lossy(&step).into();

        hash_sum += hash(&step);

        if let Some((label, focal_length)) = step.split_once('=') {
            let hash_index = hash(label);
            let focal_length: usize = focal_length.parse().unwrap();
            if let Some(previous_lens) = boxes[hash_index].iter_mut().find(|lens| &lens.0 == label) {
                previous_lens.1 = focal_length;
            }
            else {
                boxes[hash_index].push((label.to_owned(), focal_length));
            }
        }
        else if step.ends_with('-') {
            step.pop();
            let hash_index = hash(&step);
            if let Some(lens_index) = boxes[hash_index].iter_mut().position(|lens| &lens.0 == &step) {
                boxes[hash_index].remove(lens_index);
            }
        }
    }

    println!("[15p1] Sum of HASH values: {hash_sum}");

    let mut focusing_power_sum = 0;

    for (box_index, lenses) in boxes.iter().enumerate() {
        for (lens_index, (_label, focal_length)) in lenses.iter().enumerate() {
            focusing_power_sum += (box_index + 1) * (lens_index + 1) * focal_length;
        }
    }

    println!("[15p2] Sum of lens focusing powers: {focusing_power_sum}");
}
