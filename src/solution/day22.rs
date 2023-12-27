use super::*;

#[derive(Clone)]
struct Brick {
    id: usize,
    point_1: Point3D,
    point_2: Point3D,
}

impl Brick {
    fn overlaps_xy(&self, other: &Self) -> bool {
        self.point_1.x() <= other.point_2.x() && self.point_2.x() >= other.point_1.x()
            && self.point_1.y() <= other.point_2.y() && self.point_2.y() >= other.point_1.y()
    }
}

pub fn run() {
    let mut bricks: Vec<Brick> = Vec::new();
    let mut layers: BTreeMap<i64, Vec<usize>> = BTreeMap::new();

    for (id, line) in get_input("day22.txt").lines().map(expect_line).enumerate() {
        let (point_1, point_2) = line.split_once('~').unwrap();
        let mut point_1 = point_1.split(',');
        let point_1 = Point3D(point_1.next().unwrap().parse().unwrap(), point_1.next().unwrap().parse().unwrap(), point_1.next().unwrap().parse().unwrap());
        let mut point_2 = point_2.split(',');
        let point_2 = Point3D(point_2.next().unwrap().parse().unwrap(), point_2.next().unwrap().parse().unwrap(), point_2.next().unwrap().parse().unwrap());

        let brick = Brick {
            id,
            point_1,
            point_2,
        };

        bricks.push(brick);
        if let Some(layer) = layers.get_mut(&point_2.z()) {
            layer.push(id);
        }
        else {
            layers.insert(point_2.z(), vec![id]);
        }
    }

    let mut removal_cascades: BTreeMap<usize, u64> = BTreeMap::new();
    let mut settled_layers: BTreeMap<i64, Vec<(usize, Vec<usize>)>> = BTreeMap::new();

    for layer in layers.values() {
        for &id in layer {
            let mut dependencies = Vec::new();
            let mut check_z = bricks[id].point_1.z() - 1;

            while check_z > 0 {
                if let Some(layer) = settled_layers.get(&check_z) {
                    let supporting_ids = Vec::from_iter(layer.iter().filter(|&(check_id, _)| bricks[id].overlaps_xy(&bricks[*check_id])));
                    let check_dependencies = supporting_ids.iter()
                        .map(|(_, dependencies)| dependencies.clone())
                        .reduce(|mut dependencies_1, dependencies_2| {
                            dependencies_1.retain(|dependency| dependencies_2.contains(dependency));
                            dependencies_1
                        });
                    if let Some(check_dependencies) = check_dependencies {
                        dependencies = check_dependencies;
                        if supporting_ids.len() == 1 {
                            dependencies.push(supporting_ids[0].0);
                        }
                        break;
                    }
                }

                check_z -= 1;
            }

            for &dependency in &dependencies {
                if let Some(count) = removal_cascades.get_mut(&dependency) {
                    *count += 1;
                }
                else {
                    removal_cascades.insert(dependency, 1);
                }
            }

            let brick = &mut bricks[id];
            let new_layer_z = check_z + 1 + brick.point_2.z() - brick.point_1.z();
            let fall_z = brick.point_2.z() - new_layer_z;
            brick.point_1.2 -= fall_z;
            brick.point_2.2 -= fall_z;

            if let Some(layer) = settled_layers.get_mut(&new_layer_z) {
                layer.push((id, dependencies));
            }
            else {
                settled_layers.insert(new_layer_z, vec![(id, dependencies)]);
            }
        }
    }

    let non_vital_count = bricks.len() - removal_cascades.len();
    println!("[22p1] Bricks safe to disintegrate: {non_vital_count}");

    let cascade_sum: u64 = removal_cascades.values().sum();
    println!("[22p2] Sum of disintegrated brick cascades: {cascade_sum}");
}
