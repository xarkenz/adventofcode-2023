use super::*;

fn get_beam_tile(direction: Point2D) -> u8 {
    match direction {
        Point2D(1, 0) => b'>',
        Point2D(0, 1) => b'v',
        Point2D(-1, 0) => b'<',
        Point2D(0, -1) => b'^',
        _ => panic!()
    }
}

fn energize_from(map: &Map2D, point: Point2D, direction: Point2D) -> u64 {
    let mut energized = Map2D::new(b'.');
    let mut beams = Vec::new();
    beams.push((point, direction));

    while !beams.is_empty() {
        let mut next_beams = Vec::new();

        for &(point, direction) in &beams {
            let beam_tile = get_beam_tile(direction);
            if (map.min_x() ..= map.max_x()).contains(&point.x()) && (map.min_y() ..= map.max_y()).contains(&point.y()) && energized.get_at(point) != beam_tile {
                energized.put_at(point, beam_tile);

                match map.get_at(point) {
                    b'/' => {
                        let new_direction = if direction.y() == 0 {
                            Point2D(direction.y(), -direction.x())
                        }
                        else {
                            Point2D(-direction.y(), direction.x())
                        };
                        next_beams.push((point + new_direction, new_direction));
                    },
                    b'\\' => {
                        let new_direction = if direction.y() == 0 {
                            Point2D(-direction.y(), direction.x())
                        }
                        else {
                            Point2D(direction.y(), -direction.x())
                        };
                        next_beams.push((point + new_direction, new_direction));
                    },
                    b'-' => {
                        if direction.x() == 0 {
                            let new_direction = Point2D(-direction.y(), direction.x());
                            next_beams.push((point + new_direction, new_direction));
                            let new_direction = Point2D(direction.y(), -direction.x());
                            next_beams.push((point + new_direction, new_direction));
                        }
                        else {
                            next_beams.push((point + direction, direction));
                        }
                    },
                    b'|' => {
                        if direction.y() == 0 {
                            let new_direction = Point2D(-direction.y(), direction.x());
                            next_beams.push((point + new_direction, new_direction));
                            let new_direction = Point2D(direction.y(), -direction.x());
                            next_beams.push((point + new_direction, new_direction));
                        }
                        else {
                            next_beams.push((point + direction, direction));
                        }
                    },
                    b'.' => {
                        next_beams.push((point + direction, direction));
                    },
                    _ => {}
                }
            }
        }

        beams = next_beams;
    }

    let mut energized_count: u64 = 0;

    for y in energized.min_y() ..= energized.max_y() {
        for x in energized.min_x() ..= energized.max_x() {
            if energized.get(x, y) != b'.' {
                energized_count += 1;
            }
        }
    }

    energized_count
}

pub fn run() {
    let rows = get_input("day16.txt").lines()
        .map(expect_line)
        .map(String::into_bytes);

    let map = Map2D::from_rows(rows, b' ');

    let energized_count = energize_from(&map, Point2D(map.min_x(), map.min_y()), Point2D(1, 0));

    println!("[16p1] Number of energized tiles: {energized_count}");

    let mut max_energized_count = 0;

    for x in map.min_x() ..= map.max_x() {
        let energized_count = energize_from(&map, Point2D(x, map.min_y()), Point2D(0, 1));
        max_energized_count = max_energized_count.max(energized_count);
        let energized_count = energize_from(&map, Point2D(x, map.max_y()), Point2D(0, -1));
        max_energized_count = max_energized_count.max(energized_count);
    }

    for y in map.min_y() ..= map.max_y() {
        let energized_count = energize_from(&map, Point2D(map.min_x(), y), Point2D(1, 0));
        max_energized_count = max_energized_count.max(energized_count);
        let energized_count = energize_from(&map, Point2D(map.max_x(), y), Point2D(-1, 0));
        max_energized_count = max_energized_count.max(energized_count);
    }

    println!("[16p2] Maximum number of energized tiles: {max_energized_count}");
}
