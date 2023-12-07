use super::*;

pub fn run() {
    let mut lines = get_input("day05.txt").lines().map(expect_line);

    let seeds_line = lines.next().unwrap();
    let mut current_values: Vec<i64> = seeds_line.split_whitespace().skip(1)
        .map(|num| num.parse().unwrap()).collect();

    let mut current_interval_set = IntervalSet::new();
    let mut seeds_iter = current_values.iter();
    while let (Some(&start), Some(&size)) = (seeds_iter.next(), seeds_iter.next()) {
        current_interval_set.apply_interval(Interval::new(start, start + size));
    }

    lines.next();
    while lines.next().is_some() {
        let mut map = Vec::new();
        let mut mapped_interval_set = IntervalSet::new();

        while let Some(map_entry) = lines.next() {
            if map_entry.is_empty() {
                break;
            }

            let mut map_entry = map_entry.split_whitespace()
                .map(|num| num.parse::<i64>().unwrap());

            let dest_start = map_entry.next().unwrap();
            let src_start = map_entry.next().unwrap();
            let size = map_entry.next().unwrap();

            let src_interval = Interval::new(src_start, src_start + size);
            let dest_interval = Interval::new(dest_start, dest_start + size);

            map.push((src_interval, dest_interval));

            mapped_interval_set.apply(&current_interval_set.splice_interval(src_interval).with_offset(dest_start - src_start));
        }

        for value in &mut current_values {
            if let Some((src_interval, dest_interval)) = map.iter().find(|(src_interval, _)| src_interval.contains(*value)) {
                *value += dest_interval.start() - src_interval.start();
            }
        }

        current_interval_set.apply(&mapped_interval_set);
    }

    let min_location_number_p1 = *current_values.iter()
        .min().unwrap();
    let min_location_number_p2 = current_interval_set.intervals().iter()
        .map(|interval| interval.start())
        .min().unwrap();

    println!("[05p1] Lowest location number (seed list): {min_location_number_p1}");
    println!("[05p2] Lowest location number (seed ranges): {min_location_number_p2}");
}
