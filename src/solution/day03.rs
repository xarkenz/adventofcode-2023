use super::*;

use std::collections::BTreeMap;

pub fn run() {
    let schematic: Vec<Vec<u8>> = get_input("day03.txt").lines().map(expect_line)
        .map(String::into_bytes)
        .collect();
    let width = schematic[0].len();
    let height = schematic.len();
    let mut part_number_sum = 0;
    let mut gear_part_number_lists: BTreeMap<(usize, usize), Vec<u32>> = BTreeMap::new();
    for row in 0..height {
        let mut col = 0;
        while col < width {
            if schematic[row][col].is_ascii_digit() {
                let mut number = 0;
                let mut adjacent_parts: Vec<(usize, usize)> = Vec::new();
                let mut check_if_adjacent_part = |row: usize, col: usize| {
                    if schematic[row][col] != b'.' && schematic[row][col].is_ascii_punctuation() {
                        adjacent_parts.push((row, col));
                    }
                };
                if col > 0 {
                    check_if_adjacent_part(row, col - 1);
                    if row > 0 { check_if_adjacent_part(row - 1, col - 1); }
                    if row < height - 1 { check_if_adjacent_part(row + 1, col - 1); }
                }
                while col < width && schematic[row][col].is_ascii_digit() {
                    number = number * 10 + (schematic[row][col] - b'0') as u32;
                    if row > 0 { check_if_adjacent_part(row - 1, col); }
                    if row < height - 1 { check_if_adjacent_part(row + 1, col); }
                    col += 1;
                }
                if col < width {
                    check_if_adjacent_part(row, col);
                    if row > 0 { check_if_adjacent_part(row - 1, col); }
                    if row < height - 1 { check_if_adjacent_part(row + 1, col); }
                }
                if !adjacent_parts.is_empty() {
                    part_number_sum += number;
                }
                for (part_row, part_col) in adjacent_parts {
                    if schematic[part_row][part_col] == b'*' {
                        if let Some(part_number_list) = gear_part_number_lists.get_mut(&(part_row, part_col)) {
                            part_number_list.push(number);
                        }
                        else {
                            gear_part_number_lists.insert((part_row, part_col), vec![number]);
                        }
                    }
                }
            }
            else {
                col += 1;
            }
        }
    }
    let mut gear_ratio_sum = 0;
    for part_number_list in gear_part_number_lists.values() {
        if part_number_list.len() == 2 {
            gear_ratio_sum += part_number_list[0] * part_number_list[1];
        }
    }
    println!("[03p1] Sum of part numbers: {part_number_sum}");
    println!("[03p2] Sum of gear ratios: {gear_ratio_sum}");
}
