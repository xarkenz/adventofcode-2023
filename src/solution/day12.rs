use super::*;

fn get_combinations(row: &[u8], groups: &[usize]) -> u64 {
    if let Some(&group) = groups.first() {
        let min_width: usize = groups.iter().sum::<usize>() + groups.len() - 1;
        let max_start = row.len() - min_width;
        
        let mut total = 0;
        for start in 0 ..= max_start {
            if row[0 .. start].iter().all(|&spring| spring != b'#') && row[start .. start + group].iter().all(|&spring| spring != b'.') {
                if let None | Some(b'.' | b'?') = row.get(start + group) {
                    if groups.len() > 1 {
                        let next_start = start + group + 1;
                        total += get_combinations(&row[next_start..], &groups[1..]);
                    }
                    else if row.get(start + group + 1 ..).map_or(true, |rest| rest.iter().all(|&spring| spring != b'#')) {
                        total += 1;
                    }
                }
            }
        }
        total
    }
    else {
        1
    }
}

fn get_combinations_p2<'a>(section: &'a [u8], groups: &'a [usize]) -> BTreeMap<&'a [usize], u64> {
    if let Some((&group, dangling_groups)) = groups.split_first() {
        if group > section.len() {
            let mut totals = BTreeMap::new();
            if !section.iter().any(|&spring| spring == b'#') {
                totals.insert(groups, 1);
            }
            totals
        }
        else {
            let max_start = section.len() - group;
            let mut totals = BTreeMap::new();
            if section.iter().all(|&spring| spring != b'#') {
                totals.insert(groups, 1);
            }
            for start in 0 ..= max_start {
                if section[0 .. start].iter().all(|&spring| spring != b'#') {
                    if let None | Some(b'?') = section.get(start + group) {
                        let next_start = start + group + 1;
                        if !dangling_groups.is_empty() && next_start + dangling_groups[0] <= section.len() {
                            // println!("recursive call: {start} -> {next_start}");
                            for (dangling, dangling_count) in get_combinations_p2(&section[next_start..], dangling_groups) {
                                if let Some(count) = totals.get_mut(&dangling) {
                                    *count += dangling_count;
                                }
                                else {
                                    totals.insert(dangling, dangling_count);
                                }
                                // println!("unpack: {dangling:?}: {totals:?}");
                            }
                        }
                        else if section.get(next_start..).map_or(true, |rest| rest.iter().all(|&spring| spring != b'#')) {
                            if let Some(count) = totals.get_mut(&dangling_groups) {
                                *count += 1;
                            }
                            else {
                                totals.insert(dangling_groups, 1);
                            }
                            // println!("(A) {dangling_groups:?} {} {start}: {totals:?}", String::from_utf8_lossy(section));
                        }
                    }
                }
            }
            totals
        }
    }
    else {
        let mut totals = BTreeMap::new();
        if !section.iter().any(|&spring| spring == b'#') {
            totals.insert(groups, 1);
        }
        totals
    }
}

pub fn run() {
    let mut combination_sum: u64 = 0;
    let mut combination_sum_p2: u64 = 0;

    for line in get_input("day12.txt").lines().map(expect_line) {
        let (row, numbers) = line.split_once(' ').unwrap();
        let row = row.as_bytes();
        let mut row_p2 = row.to_vec();
        let groups = Vec::from_iter(numbers.split(',').map(|number| number.parse::<usize>().unwrap()));
        let mut groups_p2 = groups.clone();
        for _ in 0..4 {
            row_p2.push(b'?');
            row_p2.extend_from_slice(row);
            groups_p2.extend_from_slice(&groups);
        }
        //println!("{} : {groups_p2:?}", String::from_utf8_lossy(&row_p2));
        let row_sections = Vec::from_iter(row_p2.split(|&spring| spring == b'.').filter(|section| !section.is_empty()));
        // println!("ROW: {} {groups:?}", String::from_utf8_lossy(row));
        let mut big_totals = BTreeMap::new();
        let mut next_big_totals = BTreeMap::new();
        big_totals.insert(groups_p2.as_slice(), 1);
        for section in row_sections {
            // println!("{big_totals:?}");
            for (&groups, &multiplier) in &big_totals {
                // println!("starting {groups:?}");
                let totals = get_combinations_p2(section, &groups);
                // println!("merging {totals:?} x {multiplier}");
                for (dangling, dangling_count) in totals {
                    if let Some(count) = next_big_totals.get_mut(&dangling) {
                        *count += dangling_count * multiplier;
                    }
                    else {
                        next_big_totals.insert(dangling, dangling_count * multiplier);
                    }
                }
            }
            big_totals = BTreeMap::new();
            std::mem::swap(&mut big_totals, &mut next_big_totals);
        }

        let combinations = get_combinations(row, &groups);
        let combinations_p2 = big_totals.get(&groups[0..0]).copied().unwrap_or(0);
        println!("{line} => {combinations}, {combinations_p2}");
        combination_sum += combinations;
        combination_sum_p2 += combinations_p2;
    }

    println!("{combination_sum}");
    println!("{combination_sum_p2}");
}
