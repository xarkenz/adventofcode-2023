use super::*;

#[derive(PartialEq, Eq, Ord)]
struct State {
    heat_loss: u64,
    points: Vec<Point2D>,
    direction: Point2D,
}

impl State {
    fn new(heat_loss: u64, points: Vec<Point2D>, direction: Point2D) -> Self {
        Self {
            heat_loss,
            points,
            direction,
        }
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.heat_loss.partial_cmp(&other.heat_loss).map(std::cmp::Ordering::reverse)
    }
}

fn get_condition_index(direction: Point2D, distance: i64) -> usize {
    (distance as usize - 1) * 4 + match direction {
        Point2D(1, 0) => 0,
        Point2D(0, 1) => 1,
        Point2D(-1, 0) => 2,
        Point2D(0, -1) => 3,
        _ => panic!()
    }
}

fn get_min_heat_loss(map: &Map2D, min_distance: i64, max_distance: i64) -> u64 {
    let start = Point2D(map.min_x(), map.min_y());
    let end = Point2D(map.max_x(), map.max_y());

    let mut frontier: BTreeMap<Point2D, [Option<u64>; 40]> = BTreeMap::new();
    let mut states = BinaryHeap::new();
    states.push(State::new(0, vec![start], Point2D(0, 0)));
    let mut best_solution = None;

    while let Some(State { heat_loss, points, direction }) = states.pop() {
        let point = *points.last().unwrap();
        let min_heat_loss = heat_loss + point.manhattan_distance_to(end);
        if best_solution.as_ref().map_or(false, |(_, best_heat_loss)| min_heat_loss >= *best_heat_loss) {
            continue;
        }
        else if point == end {
            if best_solution.as_ref().map_or(true, |(_, best_heat_loss)| heat_loss < *best_heat_loss) {
                best_solution = Some((points.clone(), heat_loss));
            }
            continue;
        }
        let next_directions = if direction == Point2D(0, 0) {
            [Point2D(1, 0), Point2D(0, 1)]
        }
        else {
            [Point2D(direction.y(), -direction.x()), Point2D(-direction.y(), direction.x())]
        };
        for next_direction in next_directions {
            let mut next_point = point;
            let mut next_heat_loss = heat_loss;
            for next_distance in 1 ..= max_distance {
                next_point += next_direction;
                if !map.is_within_bounds(next_point) {
                    break;
                }
                next_heat_loss += (map.get_at(next_point) - b'0') as u64;
                if next_distance < min_distance {
                    continue;
                }

                let index = get_condition_index(next_direction, next_distance);
                if let Some(prev_condition) = frontier.get_mut(&next_point) {
                    if let Some(prev_heat_loss) = &mut prev_condition[index] {
                        if next_heat_loss >= *prev_heat_loss {
                            continue;
                        }
                        *prev_heat_loss = next_heat_loss;
                    }
                    else {
                        prev_condition[index] = Some(next_heat_loss);
                    }
                }
                else {
                    let mut next_condition = [None; 40];
                    next_condition[index] = Some(next_heat_loss);
                    frontier.insert(next_point, next_condition);
                }

                let mut next_points = points.clone();
                next_points.push(next_point);
                states.push(State::new(next_heat_loss, next_points, next_direction));
            }
        }
    }

    let (_path, min_heat_loss) = best_solution.unwrap();

    min_heat_loss
}

pub fn run() {
    let rows = get_input("day17.txt").lines()
        .map(expect_line)
        .map(|row| row.into_bytes());

    let map = Map2D::from_rows(rows, b'.');

    let min_heat_loss = get_min_heat_loss(&map, 1, 3);
    println!("[17p1] Minimal heat loss (normal crucible): {min_heat_loss}");
    
    let min_heat_loss = get_min_heat_loss(&map, 4, 10);
    println!("[17p2] Minimal heat loss (ultra crucible): {min_heat_loss}");
}
