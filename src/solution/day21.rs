use super::*;

const STEPS_P2: i64 = 26501365;
const MAP_SIZE: i64 = 131;

fn get_plots(source_map: &Map2D, start_point: Point2D, steps: i64, fill: bool) -> (i64, i64) {
    let mut map = source_map.clone();
    map.put_at(start_point, b'0');
    let mut points = vec![start_point];

    for step in 1.. {
        if (!fill && step > steps) || points.is_empty() {
            break;
        }

        let mut next_points = Vec::new();

        for point in points {
            for direction in [Point2D(1, 0), Point2D(0, 1), Point2D(-1, 0), Point2D(0, -1)] {
                let next_point = point + direction;
                if map.get_at(next_point) == b'.' {
                    next_points.push(next_point);

                    if next_point.manhattan_distance_to(start_point) > steps as u64 {
                        map.put_at(next_point, b',');
                    }
                    else if step % 2 == 0 {
                        map.put_at(next_point, b'0');
                    }
                    else {
                        map.put_at(next_point, b'1');
                    }
                }
            }
        }

        points = next_points;
    }

    let mut even_plots_visited = 0;
    let mut odd_plots_visited = 0;

    for tile in map.tiles() {
        match tile {
            b'0' => even_plots_visited += 1,
            b'1' => odd_plots_visited += 1,
            _ => {}
        }
    }

    if steps % 2 == 0 {
        (even_plots_visited, odd_plots_visited)
    }
    else {
        (odd_plots_visited, even_plots_visited)
    }
}

pub fn run() {
    let mut map = Map2D::from_rows(get_input("day21.txt").lines().map(expect_line).map(String::into_bytes), b'#');
    let center = map.points().find(|&point| map.get_at(point) == b'S').unwrap();
    map.put_at(center, b'.');

    let (plots_visited, _) = get_plots(&map, center, 64, false);
    println!("[21p1] Plots visited in 64 steps: {plots_visited}");

    let map_repeats = STEPS_P2 / MAP_SIZE; // 202300

    let (odd_visited_1, even_visited_1) = get_plots(&map, center, 65, false);
    let (odd_visited_2, even_visited_2) = get_plots(&map, center, 65, true);
    let (mut odd_visited_3, mut even_visited_3) = (0, 0);
    let (mut odd_visited_4, mut even_visited_4) = (0, 0);
    for start_point in [Point2D(map.min_x(), map.min_y()), Point2D(map.max_x(), map.max_y()), Point2D(map.max_x(), map.min_y()), Point2D(map.min_x(), map.max_y())] {
        let (even_visited, odd_visited) = get_plots(&map, start_point, 64, false);
        odd_visited_3 += odd_visited;
        even_visited_3 += even_visited;
        let (even_visited, odd_visited) = get_plots(&map, start_point, 64, true);
        odd_visited_4 += odd_visited;
        even_visited_4 += even_visited;
    }

    let total_plots_visited = (map_repeats + 1) * (odd_visited_1 - odd_visited_2) + map_repeats * (even_visited_3 - even_visited_4)
        + (map_repeats + 1) * (map_repeats + 1) * odd_visited_2
        + map_repeats * map_repeats * even_visited_2
        + (map_repeats + 1) * map_repeats * (odd_visited_4 + even_visited_4);
    println!("[21p2] Plots visited in {STEPS_P2} steps: {total_plots_visited}");
}
