use super::*;

pub fn run() {
    let map = Map2D::from_rows(get_input("day23.txt").lines().map(expect_line).map(String::into_bytes), b'#');
    
    let mut path = vec![Point2D(1, 0)];
    let mut frontier = map.clone();
    let mut max_path_length = 0;
    
    'main_loop: while let Some(&point) = path.last() {
        if point.y() == map.max_y() {
            max_path_length = max_path_length.max(path.len() - 1);
        }

        let tile = frontier.get_at(point);
        let possible_directions: &[_] = match tile {
            b'>' => &[Point2D(1, 0)],
            b'v' => &[Point2D(0, 1)],
            b'<' => &[Point2D(-1, 0)],
            b'^' => &[Point2D(0, -1)],
            _ => &[Point2D(1, 0), Point2D(0, 1), Point2D(-1, 0), Point2D(0, -1)]
        };

        // println!("{point} {} {}", tile as char, possible_directions.iter().map(|point| format!("{point}")).collect::<Vec<_>>().join(" "));

        frontier.put_at(point, match tile {
            b'>' => b'E',
            b'v' => b'S',
            b'<' => b'W',
            b'^' => b'N',
            _ => b'O',
        });

        for &direction in possible_directions {
            let next_point = point + direction;

            match (direction, frontier.get_at(next_point)) {
                (_, b'#' | b'O' | b'E' | b'S' | b'W' | b'N') => continue,
                (Point2D(1, 0), b'<') => continue,
                (Point2D(0, 1), b'^') => continue,
                (Point2D(-1, 0), b'>') => continue,
                (Point2D(0, -1), b'v') => continue,
                _ => {
                    path.push(next_point);
                    continue 'main_loop;
                }
            }
        }

        path.pop();
    }

    println!("[23p1] {max_path_length}");
    run_p2();
}

pub fn run_p2() {
    let map = Map2D::from_rows(get_input("day23.txt").lines().map(expect_line).map(String::into_bytes), b'#');
    
    let mut forks: BTreeMap<Point2D, Vec<(Point2D, u64)>> = BTreeMap::new();
    let mut forks_to_explore = VecDeque::new();
    forks_to_explore.push_back(Point2D(1, 0));
    
    while let Some(fork_to_explore) = forks_to_explore.pop_front() {
        let mut frontier = map.clone();
        frontier.put_at(fork_to_explore, b'O');
        let mut connected_forks = Vec::new();
        let mut paths: VecDeque<(Point2D, u64)> = VecDeque::from_iter([Point2D(1, 0), Point2D(0, 1), Point2D(-1, 0), Point2D(0, -1)].into_iter()
            .map(|direction| (fork_to_explore + direction, 1)));

        while let Some((point, steps)) = paths.pop_front() {
            if map.get_at(point) == b'#' {
                continue;
            }

            frontier.put_at(point, b'O');
            let mut next_points = Vec::new();

            for direction in [Point2D(1, 0), Point2D(0, 1), Point2D(-1, 0), Point2D(0, -1)] {
                let next_point = point + direction;

                if !(matches!(frontier.get_at(next_point), b'#' | b'O')) {
                    next_points.push(next_point);
                }
            }

            match next_points.as_slice() {
                &[] => if point.y() == map.min_y() || point.y() == map.max_y() {
                    connected_forks.push((point, steps));
                },
                &[next_point] => {
                    paths.push_back((next_point, steps + 1))
                },
                &[..] => {
                    connected_forks.push((point, steps))
                },
            }
        }

        forks_to_explore.extend(connected_forks.iter().filter_map(|&(fork, _)| (!forks.contains_key(&fork)).then_some(fork)));
        forks.insert(fork_to_explore, connected_forks);
    }

    let mut path: Vec<(u64, Point2D, Vec<(Point2D, u64)>)> = Vec::new();
    path.push((0, Point2D(1, 0), forks.get(&Point2D(1, 0)).unwrap().clone()));
    let mut max_path_length = 0;

    while let Some((steps, _, next_forks)) = path.last_mut() {
        let steps = *steps;
        if let Some((fork, steps_to_fork)) = next_forks.pop() {
            if fork.y() == map.max_y() {
                max_path_length = max_path_length.max(steps + steps_to_fork);
            }

            let mut next_forks = forks.get(&fork).unwrap().clone();
            let path_clone = path.clone();
            next_forks.retain(|(fork, _)| !path_clone.iter().any(|(_, point, _)| point == fork));
            if !next_forks.is_empty() {
                path.push((steps + steps_to_fork, fork, next_forks));
            }
        }
        else {
            path.pop();
        }
    }

    println!("[23p2] {max_path_length}");
}
