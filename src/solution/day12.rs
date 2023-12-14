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

fn _get_combinations_p2_bad<'a>(section: &'a [u8], groups: &'a [usize]) -> BTreeMap<&'a [usize], u64> {
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
                            for (dangling, dangling_count) in _get_combinations_p2_bad(&section[next_start..], dangling_groups) {
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

fn check_groups<'a>(row: &[u8], groups: &'a [usize]) -> Option<(usize, &'a [usize])> {
    let mut end = row.iter().position(|&spring| spring == b'?').unwrap_or(row.len());
    if end < row.len() {
        while end > 0 && row.get(end - 1).map_or(false, |&spring| spring == b'#') {
            end -= 1;
        }
    }

    let mut group_count = 0;
    for row_group in row[..end].split(|&spring| spring == b'.') {
        if !row_group.is_empty() {
            if group_count < groups.len() && row_group.len() == groups[group_count] {
                group_count += 1;
            }
            else {
                return None;
            }
        }
    }

    Some((end, &groups[group_count..]))
}

fn get_combinations_p2<'a>(row: &[u8], groups: &'a [usize], memo: &mut BTreeMap<(usize, &'a [usize]), u64>) -> u64 {
    if let Some((start, remaining_groups)) = check_groups(row, groups) {
        if remaining_groups.is_empty() {
            if start >= row.len() || !row[start..].iter().any(|&spring| spring == b'#') {
                1
            }
            else {
                0
            }
        }
        else if start + remaining_groups.len() + remaining_groups.iter().sum::<usize>() - 1 > row.len() {
            0
        }
        else if let Some(memoized_total) = memo.get(&(start, remaining_groups)) {
            *memoized_total
        }
        else {
            let mut next_row: Box<[u8]> = row.into();
            let mut total = 0;

            if row[start] == b'?' && start + remaining_groups.len() + remaining_groups.iter().sum::<usize>() <= row.len() {
                next_row[start] = b'.';
                total += get_combinations_p2(next_row.as_ref(), groups, memo);
                next_row[start] = b'?';
            }
            
            'try_group: {
                for spring in &mut next_row[start .. start + remaining_groups[0]] {
                    match *spring {
                        b'.' => break 'try_group,
                        b'?' => *spring = b'#',
                        _ => {}
                    }
                }
                if let Some(next_spring) = next_row.get_mut(start + remaining_groups[0]) {
                    match *next_spring {
                        b'#' => break 'try_group,
                        b'?' => *next_spring = b'.',
                        _ => {}
                    }
                }
                total += get_combinations_p2(next_row.as_ref(), groups, memo);
            }

            memo.insert((start, remaining_groups), total);

            total
        }
    }
    else {
        0
    }
}

pub fn run() {
    let mut folded_combination_sum: u64 = 0;
    let mut unfolded_combination_sum: u64 = 0;

    for line in get_input("day12.txt").lines().map(expect_line) {
        let (row, numbers) = line.split_once(' ').unwrap();

        let folded_row = row.as_bytes();
        let mut unfolded_row = folded_row.to_vec();

        let folded_groups = Vec::from_iter(numbers.split(',').map(|number| number.parse::<usize>().unwrap()));
        let mut unfolded_groups = folded_groups.clone();

        for _ in 0..4 {
            unfolded_row.push(b'?');
            unfolded_row.extend_from_slice(folded_row);
            unfolded_groups.extend_from_slice(&folded_groups);
        }

        // let row_sections = Vec::from_iter(row_p2.split(|&spring| spring == b'.').filter(|section| !section.is_empty()));
        // let mut big_totals = BTreeMap::new();
        // let mut next_big_totals = BTreeMap::new();
        // big_totals.insert(groups_p2.as_slice(), 1);
        // for section in row_sections {
        //     for (&groups, &multiplier) in &big_totals {
        //         let totals = _get_combinations_p2_bad(section, &groups);
        //         for (dangling, dangling_count) in totals {
        //             if let Some(count) = next_big_totals.get_mut(&dangling) {
        //                 *count += dangling_count * multiplier;
        //             }
        //             else {
        //                 next_big_totals.insert(dangling, dangling_count * multiplier);
        //             }
        //         }
        //     }
        //     big_totals = BTreeMap::new();
        //     std::mem::swap(&mut big_totals, &mut next_big_totals);
        // }
        // let combinations_p2 = big_totals.get(&groups[0..0]).copied().unwrap_or(0);

        let folded_combinations = get_combinations_p2(folded_row, &folded_groups, &mut BTreeMap::new());
        folded_combination_sum += folded_combinations;

        let unfolded_combinations = get_combinations_p2(&unfolded_row, &unfolded_groups, &mut BTreeMap::new());
        unfolded_combination_sum += unfolded_combinations;
    }

    println!("[12p1] Folded combinations: {folded_combination_sum}");
    println!("[12p2] Unfolded combinations: {unfolded_combination_sum}");
}
