use super::*;

pub fn run() {
    let rows = get_input("day11.txt").lines()
        .map(expect_line);

    let mut map = Map2D::new(b'.');
    let mut galaxies = Vec::new();
    let mut gap_rows = Vec::new();
    let mut gap_columns = Vec::new();

    for (y, row) in rows.enumerate() {
        if row.find('#').is_none() {
            gap_rows.push(y as i64);
        }
        else {
            for (x, ch) in row.chars().enumerate() {
                if ch == '#' {
                    let point = Point2D(x as i64, y as i64);
                    galaxies.push(point);
                    map.put_at(point, b'#');
                }
            }
        }
    }
    'columns: for x in map.min_x() ..= map.max_x() {
        for y in map.min_y() ..= map.max_y() {
            if map.get(x, y) == b'#' {
                continue 'columns;
            }
        }
        gap_columns.push(x);
    }

    let mut distance_sum_p1: u64 = 0;
    let mut distance_sum_p2: u64 = 0;

    for (index_1, galaxy_1) in galaxies.iter().enumerate() {
        for (index_2, galaxy_2) in galaxies.iter().enumerate() {
            if index_1 > index_2 {
                let mut distance_p1 = galaxy_1.manhattan_distance_to(*galaxy_2);
                let mut distance_p2 = distance_p1;

                let x_interval = Interval::new_normalize(galaxy_1.x(), galaxy_2.x());
                let y_interval = Interval::new_normalize(galaxy_1.y(), galaxy_2.y());
                for &gap_column in &gap_columns {
                    if x_interval.contains(gap_column) {
                        distance_p1 += 1;
                        distance_p2 += 999999;
                    }
                }
                for &gap_row in &gap_rows {
                    if y_interval.contains(gap_row) {
                        distance_p1 += 1;
                        distance_p2 += 999999;
                    }
                }

                distance_sum_p1 += distance_p1;
                distance_sum_p2 += distance_p2;
            }
        }
    }

    println!("[11p1] Sum of compressed galaxy distances: {distance_sum_p1}");
    println!("[11p2] Sum of uncompressed galaxy distances: {distance_sum_p2}");
}
