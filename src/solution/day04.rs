use super::*;

pub fn run() {
    let lines = get_input("day04.txt").lines().map(expect_line);

    let mut scratch_card_point_sum: u64 = 0;
    let mut scratch_card_matches: Vec<u64> = Vec::new();
    
    for line in lines {
        let winning_numbers: Vec<u64> = line[10..39].split_whitespace()
            .filter_map(|number| (!number.is_empty()).then(|| number.parse().unwrap()))
            .collect();

        let mut matches = 0;
        for number in line[42..].split_whitespace() {
            if !number.is_empty() {
                let number = number.parse().unwrap();
                if winning_numbers.contains(&number) {
                    matches += 1;
                }
            }
        }

        if matches > 0 {
            scratch_card_point_sum += 1 << (matches - 1)
        }

        scratch_card_matches.push(matches);
    }

    // My initial approach was a queue of scratch card indices and a separate scratch card count, which,
    // due to the magnitude of the number of scratch cards, is pretty inefficient
    // I decided to change to the better approach of storing a count for each card because otherwise
    // the computation here would take almost a second lol
    let mut scratch_card_counts: Vec<u64> = std::iter::repeat(1)
        .take(scratch_card_matches.len())
        .collect();

    for (index, matches) in scratch_card_matches.iter().enumerate() {
        let first = index + 1;
        let last = index + *matches as usize;
        let count_to_add = scratch_card_counts[index];

        for count in &mut scratch_card_counts[first..=last] {
            *count += count_to_add;
        }
    }

    let scratch_card_count_sum: u64 = scratch_card_counts.iter().sum();

    println!("[04p1] Sum of scratch card points: {scratch_card_point_sum}");
    println!("[04p2] Number of scratch cards: {scratch_card_count_sum}");
}
