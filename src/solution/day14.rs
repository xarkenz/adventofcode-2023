use super::*;

fn roll(map: &mut Map2D, rocks: &mut [Point2D], dx: i64, dy: i64) {
    for Point2D(x, y) in rocks {
        map.put(*x, *y, b'.');

        while map.get(*x + dx, *y + dy) != b'#' {
            *x += dx;
            *y += dy;
        }

        while map.get(*x, *y) != b'.' {
            *x -= dx;
            *y -= dy;
        }

        map.put(*x, *y, b'O');
    }
}

pub fn run() {
    let rows = get_input("day14.txt").lines()
        .map(expect_line)
        .map(|line| line.into_bytes());
    let mut map = Map2D::from_rows(rows, b'#');

    let mut rocks = Vec::new();

    for y in map.min_y() ..= map.max_y() {
        for x in map.min_x() ..= map.max_x() {
            if map.get(x, y) == b'O' {
                rocks.push(Point2D(x, y));
            }
        }
    }

    roll(&mut map, &mut rocks, 0, -1);

    let mut north_beam_load: i64 = 0;
    
    for rock_location in &rocks {
        let load = map.max_y() - rock_location.y() + 1;
        north_beam_load += load;
    }

    println!("[14p1] North beam load: {north_beam_load}");

    let mut previous_loads = Vec::new();

    let billionth_load = loop {
        previous_loads.push(north_beam_load);

        roll(&mut map, &mut rocks, 0, -1);
        roll(&mut map, &mut rocks, -1, 0);
        roll(&mut map, &mut rocks, 0, 1);
        roll(&mut map, &mut rocks, 1, 0);

        north_beam_load = 0;
        for rock in &rocks {
            let load = map.max_y() - rock.y() + 1;
            north_beam_load += load;
        }

        if let Some(index) = previous_loads.iter().rposition(|&load| load == north_beam_load) {
            let interval = previous_loads.len() - index;
            if interval > 2 && index >= interval - 1 && (1..interval).all(
                |offset| previous_loads[previous_loads.len() - offset] == previous_loads[index - offset]
            ) {
                let interval_offset = (1000000000 - index) % (previous_loads.len() - index);
                let target_load = previous_loads[index + interval_offset];
                break target_load;
            }
        }
    };

    println!("[14p2] North beam load (1 billion cycles): {billionth_load}");
}
