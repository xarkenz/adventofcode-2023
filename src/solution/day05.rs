use super::*;

#[derive(Clone, Debug)]
struct Ranges(Vec<(i64, i64)>);

fn get_union(a: (i64, i64), b: (i64, i64)) -> Option<(i64, i64)> {
    let (a_start, a_end) = a;
    let (b_start, b_end) = b;
    if a_end < b_start || b_end < a_start {
        None
    }
    else {
        Some((a_start.min(b_start), a_end.max(b_end)))
    }
}

impl Ranges {
    fn with_offset(self, offset: i64) -> Self {
        Self(self.0.iter().map(|&(start, end)| (start + offset, end + offset)).collect())
    }
    
    fn add_ranges(&mut self, ranges: Ranges) {
        for (start, end) in ranges.0 {
            self.add_range(start, end);
        }
    }

    fn add_range(&mut self, start: i64, end: i64) {
        if start < end {
            let unions: Vec<(i64, i64)> = self.0.iter().filter_map(|&range| get_union(range, (start, end))).collect();
            let new_range = (unions.iter().map(|(start, _)| *start).min().unwrap_or(start), unions.iter().map(|(_, end)| *end).max().unwrap_or(end));
            self.0.retain(|&range| get_union(range, (start, end)).is_none());
            self.0.push(new_range);
        }
    }

    fn cut_ranges(&mut self, start: i64, end: i64) -> Ranges {
        let mut contained_ranges = Ranges(Vec::new());
        if start < end {
            let mut split_off_ranges = Vec::new();
            self.0.retain_mut(|(got_start, got_end)| {
                if start <= *got_start && *got_end <= end {
                    contained_ranges.add_range(*got_start, *got_end);
                    false
                }
                else if *got_start <= start && end <= *got_end {
                    contained_ranges.add_range(start, end);
                    split_off_ranges.push((end, *got_end));
                    *got_end = start;
                    true
                }
                else if *got_start < end && end <= *got_end {
                    contained_ranges.add_range(*got_start, end);
                    *got_start = end;
                    true
                }
                else if *got_start <= start && start < *got_end {
                    contained_ranges.add_range(start, *got_end);
                    *got_end = start;
                    true
                }
                else {
                    true
                }
            });
            self.0.append(&mut split_off_ranges);
        }
        contained_ranges
    }

    fn min(&self) -> Option<i64> {
        self.0.iter().map(|&(start, end)| start.min(end)).min()
    }
}

pub fn run() {
    let mut lines = get_input("day05.txt").lines().map(expect_line);

    let seeds_line = lines.next().unwrap();
    let mut seeds: Vec<i64> = seeds_line.split_ascii_whitespace().skip(1)
        .map(|num| num.parse::<i64>().unwrap()).collect();
    let mut current_ranges = Ranges(Vec::new());
    let mut seeds_iter = seeds.iter();
    while let (Some(&start), Some(&size)) = (seeds_iter.next(), seeds_iter.next()) {
        current_ranges.add_range(start, start + size);
    }

    lines.next();
    while lines.next().is_some() {
        let mut map: Vec<(i64, i64, i64)> = Vec::new();
        let mut mapped_ranges = Ranges(Vec::new());

        while let Some(map_entry) = lines.next() {
            if map_entry.is_empty() {
                break;
            }
            let mut map_entry = map_entry.splitn(3, |ch: char| ch.is_ascii_whitespace())
                .map(|num| num.parse::<i64>().unwrap());
            let dest_start = map_entry.next().unwrap();
            let src_start = map_entry.next().unwrap();
            let range_size = map_entry.next().unwrap();
            map.push((dest_start, src_start, range_size));

            mapped_ranges.add_ranges(current_ranges.cut_ranges(src_start, src_start + range_size).with_offset(dest_start - src_start));
        }
        for seed in &mut seeds {
            for &(dest_start, src_start, range_size) in &map {
                if (src_start .. src_start + range_size).contains(seed) {
                    *seed += dest_start - src_start;
                    break;
                }
            }
        }

        current_ranges.add_ranges(mapped_ranges);
    }

    let min_location_number = *seeds.iter().min().unwrap();
    println!("{min_location_number}");

    let min_range_number = current_ranges.min().unwrap();
    println!("{min_range_number}");
}
