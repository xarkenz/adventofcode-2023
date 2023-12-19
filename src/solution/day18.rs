use super::*;

fn get_direction(raw: &str) -> Point2D {
    match raw {
        "R" => Point2D(1, 0),
        "D" => Point2D(0, 1),
        "L" => Point2D(-1, 0),
        "U" => Point2D(0, -1),
        _ => panic!()
    }
}

const DIRECTIONS: &[Point2D] = &[
    Point2D(1, 0),
    Point2D(0, 1),
    Point2D(-1, 0),
    Point2D(0, -1),
];

pub fn run() {
    let mut map = Map2D::new(b'.');
    let mut current_pos = Point2D(0, 0);
    map.put_at(current_pos, b'#');
    let mut volume: i64 = 1;

    let mut color_current_pos = Point2D(0, 0);
    let mut horizontals: Vec<(i64, Interval)> = Vec::new();
    let mut last_vertical_direction = Point2D(0, -1);

    for line in get_input("day18.txt").lines().map(expect_line) {
        let mut split = line.split_whitespace();
        let direction = get_direction(split.next().unwrap());
        let distance = split.next().unwrap().parse::<i64>().unwrap();
        let color = split.next().unwrap();
        let color_direction = DIRECTIONS[(color.as_bytes()[7] - b'0') as usize];
        let color_distance = i64::from_str_radix(&color[2..7], 16).unwrap();
        
        for _ in 0 .. distance {
            current_pos += direction;
            if map.get_at(current_pos) != b'#' {
                volume += 1;
            }
            map.put_at(current_pos, b'#');
        }

        if color_direction.y() == 0 {
            let y = if color_direction.x() > 0 { color_current_pos.y() } else { color_current_pos.y() + 1 };
            horizontals.push((
                y,
                Interval::new(
                    color_current_pos.x(),
                    color_current_pos.x() + color_direction.x() * color_distance,
                ),
            ));
        }
        else {
            if let Some((_, interval)) = horizontals.last_mut() {
                if color_direction != last_vertical_direction {
                    if color_direction.y() < 0 {
                        *interval = Interval::new(interval.start() + 1, interval.end());
                    }
                    else {
                        *interval = Interval::new(interval.start(), interval.end() + 1);
                    }
                }
                else if color_direction.y() > 0 {
                    *interval = Interval::new(interval.start() + 1, interval.end() + 1);
                }
            }
            last_vertical_direction = color_direction;
        }
        color_current_pos += color_direction * color_distance;
    }

    let mut points = VecDeque::new();
    points.push_back(Point2D(1, 1));

    while let Some(point) = points.pop_front() {
        if map.get_at(point) != b'#' {
            map.put_at(point, b'#');
            volume += 1;

            for neighbor in [
                point + Point2D(1, 0),
                point + Point2D(0, 1),
                point + Point2D(-1, 0),
                point + Point2D(0, -1),
            ] {
                points.push_back(neighbor);
            }
        }
    }

    println!("[18p1] {volume}");

    let mut color_volume: i64 = 0;
    let mut last_y: i64 = 0;
    let mut slice_intervals = IntervalSet::new();
    horizontals.sort_by_key(|(y, _)| *y);
    for (y, interval) in horizontals {
        color_volume += slice_intervals.cardinality() * (y - last_y);
        if interval.size() > 0 {
            // top face
            slice_intervals.apply_interval(interval);
        }
        else {
            // bottom face
            slice_intervals.splice_interval(interval.normalized());
        }
        last_y = y;
    }

    println!("[18p2] {color_volume}");
}
