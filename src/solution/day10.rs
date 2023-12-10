use super::*;

const CONNECTING_PIPES: &[(Point2D, &[u8])] = &[
    (Point2D(1, 0), &[b'-', b'7', b'J']),
    (Point2D(0, 1), &[b'|', b'J', b'L']),
    (Point2D(-1, 0), &[b'-', b'L', b'F']),
    (Point2D(0, -1), &[b'|', b'F', b'7']),
];

const PIPE_DIRECTIONS: &[(u8, &[Point2D])] = &[
    (b'F', &[Point2D(1, 0), Point2D(0, 1)]),
    (b'-', &[Point2D(1, 0), Point2D(-1, 0)]),
    (b'L', &[Point2D(1, 0), Point2D(0, -1)]),
    (b'7', &[Point2D(0, 1), Point2D(-1, 0)]),
    (b'|', &[Point2D(0, 1), Point2D(0, -1)]),
    (b'J', &[Point2D(-1, 0), Point2D(0, -1)]),
];

fn get_next_point(map: &Map2D, point: Point2D, previous_point: Point2D) -> Point2D {
    let current_tile = map.get_at(point);
    let offsets = PIPE_DIRECTIONS.iter().find_map(|&(tile, offsets)| {
        (tile == current_tile).then_some(offsets)
    }).unwrap();
    offsets.iter().find_map(|&offset| {
        let offset_point = point + offset;
        (offset_point != previous_point).then_some(offset_point)
    }).unwrap()
}

fn infer_tile(map: &mut Map2D, point: Point2D) {
    let connections = Vec::from_iter(CONNECTING_PIPES.iter().map(|&(offset, tiles)| {
        tiles.contains(&map.get_at(point + offset))
    }));
    let tile = match connections.as_slice() {
        &[true, true, false, false] => b'F',
        &[true, false, true, false] => b'-',
        &[true, false, false, true] => b'L',
        &[false, true, true, false] => b'7',
        &[false, true, false, true] => b'|',
        &[false, false, true, true] => b'J',
        other => panic!("{other:?}")
    };
    map.put_at(point, tile);
}

pub fn run() {
    let mut start_point = Point2D(0, 0);

    let rows = get_input("day10.txt").lines()
        .map(expect_line)
        .enumerate()
        .map(|(y, line)| {
            if let Some(x) = line.find('S') {
                start_point = Point2D(x as i64, y as i64);
            }
            line.into_bytes()
        });

    let mut map = Map2D::from_rows(rows, b'.');
    infer_tile(&mut map, start_point);
    
    let mut previous_point_1 = start_point;
    let mut previous_point_2 = start_point;

    // A bit of a hacky way to get the first two points from the start, but it works
    let mut point_1 = get_next_point(&map, start_point, start_point);
    let mut point_2 = get_next_point(&map, start_point, point_1);

    let mut steps_taken: u64 = 0;
    let mut loop_map = Map2D::new(b'.');
    loop_map.put_at(start_point, map.get_at(start_point));

    while loop_map.get_at(point_1) == b'.' {
        loop_map.put_at(point_1, map.get_at(point_1));
        loop_map.put_at(point_2, map.get_at(point_2));

        (previous_point_1, point_1) = (point_1, get_next_point(&map, point_1, previous_point_1));
        (previous_point_2, point_2) = (point_2, get_next_point(&map, point_2, previous_point_2));

        steps_taken += 1;
    }

    println!("[10p1] Steps to farthest point: {steps_taken}");

    let mut tiles_enclosed: u64 = 0;

    for y in loop_map.min_y() ..= loop_map.max_y() {
        let mut is_inside = false;
        let mut last_elbow = None;

        for x in loop_map.min_x() ..= loop_map.max_x() {
            let point = Point2D(x, y);
            match loop_map.get_at(point) {
                b'|' => {
                    is_inside = !is_inside;
                },
                b'7' => {
                    if let Some(b'L') = last_elbow {
                        is_inside = !is_inside;
                    }
                    last_elbow = None;
                },
                b'J' => {
                    if let Some(b'F') = last_elbow {
                        is_inside = !is_inside;
                    }
                    last_elbow = None;
                },
                b'L' => {
                    last_elbow = Some(b'L');
                },
                b'F' => {
                    last_elbow = Some(b'F');
                },
                b'.' => {
                    if is_inside {
                        tiles_enclosed += 1;
                    }
                },
                _ => {}
            }
        }
    }

    println!("[10p2] Tiles enclosed by loop: {tiles_enclosed}");
}
