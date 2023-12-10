use super::*;

fn advance_pos(map: &Map2D, pos: &mut Point2D, prev_pos: &mut Point2D) {
    let new_pos = match map.get_at(*pos) {
        b'-' => if pos.x() > prev_pos.x() { Point2D(pos.x() + 1, pos.y()) } else { Point2D(pos.x() - 1, pos.y()) },
        b'|' => if pos.y() > prev_pos.y() { Point2D(pos.x(), pos.y() + 1) } else { Point2D(pos.x(), pos.y() - 1) },
        b'F' => if pos.x() < prev_pos.x() { Point2D(pos.x(), pos.y() + 1) } else { Point2D(pos.x() + 1, pos.y()) },
        b'7' => if pos.x() > prev_pos.x() { Point2D(pos.x(), pos.y() + 1) } else { Point2D(pos.x() - 1, pos.y()) },
        b'J' => if pos.x() > prev_pos.x() { Point2D(pos.x(), pos.y() - 1) } else { Point2D(pos.x() - 1, pos.y()) },
        b'L' => if pos.x() < prev_pos.x() { Point2D(pos.x(), pos.y() - 1) } else { Point2D(pos.x() + 1, pos.y()) },
        ch => panic!("{}", ch as char)
    };

    *prev_pos = *pos;
    *pos = new_pos;
}

pub fn run() {
    let mut start_pos = Point2D(0, 0);

    let map = Map2D::from_rows(get_input("day10.txt").lines().map(expect_line).enumerate().map(|(y, line)| {
        if let Some(x) = line.find('S') {
            start_pos = Point2D(x as i64, y as i64);
        }
        line.into_bytes()
    }), b'.');
    
    let mut prev_pos_1 = start_pos;
    let mut prev_pos_2 = start_pos;
    let mut pos_1 = Point2D(start_pos.x() - 1, start_pos.y()); // hardcoded :(
    let mut pos_2 = Point2D(start_pos.x(), start_pos.y() - 1);
    let mut steps_taken: u64 = 1;
    let mut pipe_segments = vec![start_pos, pos_1, pos_2];

    while (pos_1.x() - pos_2.x()).abs() + (pos_1.y() - pos_2.y()).abs() > 1 {
        advance_pos(&map, &mut pos_1, &mut prev_pos_1);
        advance_pos(&map, &mut pos_2, &mut prev_pos_2);

        steps_taken += 1;

        pipe_segments.push(pos_1);
        pipe_segments.push(pos_2);
    }

    println!("{steps_taken}");

    let mut tiles_enclosed: u64 = 0;
    let mut pipe_map = Map2D::new(b'.');

    for y in map.min_y()..=map.max_y() {
        let mut is_inside = false;
        let mut last_elbow = b'X';

        for x in map.min_x()..=map.max_x() {
            let pos = Point2D(x, y);
            if pipe_segments.contains(&pos) {
                pipe_map.put_at(pos, b'*');
                match map.get_at(pos) {
                    b'|' => {
                        is_inside = !is_inside;
                    },
                    b'7' => {
                        if last_elbow == b'L' {
                            is_inside = !is_inside;
                        }
                        last_elbow = b'X';
                    },
                    b'J' => {
                        if last_elbow == b'F' {
                            is_inside = !is_inside;
                        }
                        last_elbow = b'X';
                    },
                    b'L' => {
                        last_elbow = b'L';
                    }
                    b'F' => {
                        last_elbow = b'F';
                    }
                    _ => {}
                }
            }
            else if is_inside {
                tiles_enclosed += 1;
                pipe_map.put_at(pos, b'I');
            }
            else {
                pipe_map.put_at(pos, b'O');
            }
        }
    }

    println!("{pipe_map}");
    println!("{tiles_enclosed}");
}
