use std::io::Read;

use super::*;

pub fn run() {
    let mut patterns = String::new();
    get_input("day13.txt").read_to_string(&mut patterns).unwrap();

    let mut summary = 0;
    let mut summary_smudged = 0;

    'patterns: for pattern in patterns.split("\r\n\r\n") {
        let map = Map2D::from_rows(pattern.lines().map(|line| line.as_bytes().to_vec()), b' ');
        /*let get_row = |y: i64| {
            Vec::from_iter((map.min_x() ..= map.max_y()).map(|x| map.get(x, y)))
        };
        let get_column = |x: i64| {
            Vec::from_iter((map.min_y() ..= map.max_y()).map(|y| map.get(x, y)))
        };*/
        'vertical_line_search: for x in map.min_x() + 1 ..= map.max_x() {
            let (mut x1, mut x2) = (x - 1, x);
            let mut used_smudge = false;
            while x1 >= map.min_x() && x2 <= map.max_x() {
                for y in map.min_y() ..= map.max_y() {
                    if map.get(x1, y) != map.get(x2, y) {
                        if used_smudge {
                            continue 'vertical_line_search;
                        }
                        else {
                            used_smudge = true;
                        }
                    }
                }
                x1 -= 1;
                x2 += 1;
            }
            if used_smudge {
                summary_smudged += x;
            }
            else {
                summary += x;
            }
        }
        'horizontal_line_search: for y in map.min_y() + 1 ..= map.max_y() {
            let (mut y1, mut y2) = (y - 1, y);
            let mut used_smudge = false;
            while y1 >= map.min_y() && y2 <= map.max_y() {
                for x in map.min_x() ..= map.max_x() {
                    if map.get(x, y1) != map.get(x, y2) {
                        if used_smudge {
                            continue 'horizontal_line_search;
                        }
                        else {
                            used_smudge = true;
                        }
                    }
                }
                y1 -= 1;
                y2 += 1;
            }
            if used_smudge {
                summary_smudged += 100 * y;
            }
            else {
                summary += 100 * y;
            }
        }
    }

    println!("{summary}");
    println!("{summary_smudged}");
}
