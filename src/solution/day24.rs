use super::*;

pub fn run() {
    let mut trajectories = Vec::new();

    for line in get_input("day24.txt").lines().map(expect_line) {
        let (position, velocity) = line.split_once(" @ ").unwrap();

        let mut position = position.split(", ");
        let (x, y, z) = (
            position.next().unwrap().parse::<f64>().unwrap(),
            position.next().unwrap().parse::<f64>().unwrap(),
            position.next().unwrap().parse::<f64>().unwrap(),
        );

        let mut velocity = velocity.split(", ");
        let (dx, dy, dz) = (
            velocity.next().unwrap().parse::<f64>().unwrap(),
            velocity.next().unwrap().parse::<f64>().unwrap(),
            velocity.next().unwrap().parse::<f64>().unwrap(),
        );

        trajectories.push((x, y, z, dx, dy, dz));
    }

    let mut collided_xy: u64 = 0;

    for (index, &(x1, y1, z1, dx1, dy1, dz1)) in trajectories.iter().enumerate() {
        for &(x2, y2, _z2, dx2, dy2, _dz2) in trajectories[index + 1 ..].iter() {
            if dx2 * dy1 != dx1 * dy2 {
                let t = (dx2 * (y2 - y1) - dy2 * (x2 - x1)) / (dx2 * dy1 - dx1 * dy2);
                let s = (dx1 * (y2 - y1) - dy1 * (x2 - x1)) / (dx2 * dy1 - dx1 * dy2);
                let cx = x1 + dx1 * t;
                let cy = y1 + dy1 * t;
                if t > 0.0 && s > 0.0 && 2e14 <= cx && cx <= 4e14 && 2e14 <= cy && cy <= 4e14 {
                    collided_xy += 1;
                }
            }
        }
    }

    println!("[24p1] {collided_xy}");

    // unknowns: x, y, z, dx, dy, dz, t
    /*let mut system = EquationSystem::new(trajectories.len() * 3, 8);
    for index in 0..trajectories.len() {
        //
    }*/
}
