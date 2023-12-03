use super::*;

use std::collections::BTreeMap;

fn is_part_symbol(tile: u8) -> bool {
    tile != b'.' && tile.is_ascii_punctuation()
}

pub fn run() {
    let lines = get_input("day03.txt").lines()
        .map(expect_line)
        .map(String::into_bytes);
    let schematic = Map2D::from_rows(lines, b'.');

    let mut part_number_sum = 0;
    let mut gear_part_number_lists: BTreeMap<Point2D, Vec<u32>> = BTreeMap::new();

    for y in schematic.min_y() ..= schematic.max_y() {
        let mut x = schematic.min_x();
        
        while x <= schematic.max_x() {
            if schematic.get(x, y).is_ascii_digit() {
                let mut number = 0;
                let mut adjacent_parts = Vec::new();

                let mut check_if_adjacent_part = |x, y| {
                    if is_part_symbol(schematic.get(x, y)) {
                        adjacent_parts.push(Point2D(x, y));
                    }
                };
                
                check_if_adjacent_part(x - 1, y - 1);
                check_if_adjacent_part(x - 1, y);
                check_if_adjacent_part(x - 1, y + 1);

                while schematic.get(x, y).is_ascii_digit() {
                    number = number * 10 + (schematic.get(x, y) - b'0') as u32;

                    check_if_adjacent_part(x, y - 1);
                    check_if_adjacent_part(x, y + 1);

                    x += 1;
                }

                check_if_adjacent_part(x, y - 1);
                check_if_adjacent_part(x, y);
                check_if_adjacent_part(x, y + 1);

                if !adjacent_parts.is_empty() {
                    part_number_sum += number;
                }

                for part in adjacent_parts {
                    if schematic.get_at(part) == b'*' {
                        if let Some(part_number_list) = gear_part_number_lists.get_mut(&part) {
                            part_number_list.push(number);
                        }
                        else {
                            gear_part_number_lists.insert(part, vec![number]);
                        }
                    }
                }
            }

            x += 1;
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
