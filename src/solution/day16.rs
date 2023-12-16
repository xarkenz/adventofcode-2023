use super::*;

fn get_direction_tile(direction: Point2D) -> u8 {
    match direction {
        Point2D(1, 0) => b'>',
        Point2D(0, 1) => b'v',
        Point2D(-1, 0) => b'<',
        Point2D(0, -1) => b'^',
        _ => panic!()
    }
}

fn get_beam_energized_count(map: &Map2D, point: Point2D, direction: Point2D) -> usize {
    let mut beam_trace = Map2D::new(b'.');
    let mut beam_heads = Vec::new();
    beam_heads.push((point, direction));

    let mut energized_count = 0;

    while !beam_heads.is_empty() {
        let mut next_beams = Vec::new();

        for &(point, direction) in &beam_heads {
            let direction_tile = get_direction_tile(direction);

            if map.is_within_bounds(point) && beam_trace.get_at(point) != direction_tile {
                if beam_trace.get_at(point) == b'.' {
                    energized_count += 1;
                }
                beam_trace.put_at(point, direction_tile);

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

        beam_heads = next_beams;
    }

    energized_count
}

pub fn run() {
    let rows = get_input("day16.txt").lines()
        .map(expect_line)
        .map(String::into_bytes);

    let map = Map2D::from_rows(rows, b' ');

    let energized_count = get_beam_energized_count(&map, Point2D(map.min_x(), map.min_y()), Point2D(1, 0));

    println!("[16p1] Number of energized tiles: {energized_count}");

    let mut max_energized_count = 0;

    for x in map.x_values() {
        let energized_count_from_top = get_beam_energized_count(&map, Point2D(x, map.min_y()), Point2D(0, 1));
        max_energized_count = max_energized_count.max(energized_count_from_top);
        let energized_count_from_bottom = get_beam_energized_count(&map, Point2D(x, map.max_y()), Point2D(0, -1));
        max_energized_count = max_energized_count.max(energized_count_from_bottom);
    }

    for y in map.y_values() {
        let energized_count_from_left = get_beam_energized_count(&map, Point2D(map.min_x(), y), Point2D(1, 0));
        max_energized_count = max_energized_count.max(energized_count_from_left);
        let energized_count_from_right = get_beam_energized_count(&map, Point2D(map.max_x(), y), Point2D(-1, 0));
        max_energized_count = max_energized_count.max(energized_count_from_right);
    }

    println!("[16p2] Maximum number of energized tiles: {max_energized_count}");
}
