use super::*;

pub fn run() {
    let mut network: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for line in get_input("day25.txt").lines().map(expect_line) {
        let (node, adjacencies) = line.split_once(": ").unwrap();
        let mut adjacencies = Vec::from_iter(adjacencies.split(' ').map(|adjacency| {
            if let Some(other_adjacencies) = network.get_mut(adjacency) {
                other_adjacencies.push(node.into());
            }
            else {
                network.insert(adjacency.into(), vec![node.into()]);
            }
            adjacency.into()
        }));
        
        if let Some(previous_adjacencies) = network.get_mut(node) {
            previous_adjacencies.append(&mut adjacencies);
        }
        else {
            network.insert(node.into(), adjacencies);
        }
    }

    let mut max_distance = 0;
    let mut max_nodes = None;

    for start_node in network.keys() {
        let mut visited: BTreeSet<&String> = BTreeSet::new();
        let mut to_visit: VecDeque<(&String, usize)> = VecDeque::new();
        to_visit.push_back((start_node, 0));

        while let Some((node, distance)) = to_visit.pop_front() {
            if visited.insert(node) {
                if distance > max_distance {
                    max_distance = distance;
                    max_nodes = Some((start_node.clone(), node.clone()));
                }

                for neighbor in network.get(node).unwrap() {
                    to_visit.push_back((neighbor, distance + 1));
                }
            }
        }
    }

    let (start_node, end_node) = max_nodes.unwrap();
    let mut levels: Vec<Vec<&String>> = std::iter::repeat_with(Vec::new).take(max_distance + 1).collect();
    let mut visited: BTreeSet<&String> = BTreeSet::new();
    let mut to_visit: VecDeque<(&String, usize)> = VecDeque::new();
    to_visit.push_back((&start_node, 0));

    while let Some((node, distance)) = to_visit.pop_front() {
        if visited.insert(node) {
            levels[distance].push(node);

            for neighbor in network.get(node).unwrap() {
                to_visit.push_back((neighbor, distance + 1));
            }
        }
    }

    let mut times_used: BTreeMap<(&String, &String), (usize, usize)> = BTreeMap::new();
    let next_nodes_from_start = network.get(&start_node).unwrap().iter()
        .filter(|neighbor| levels[1].contains(neighbor))
        .collect();
    let mut path: Vec<(&String, Vec<&String>)> = vec![(&start_node, next_nodes_from_start)];

    while let Some((node, next_nodes)) = path.last_mut() {
        let node = *node;
        if let Some(next_node) = next_nodes.pop() {
            if let Some((_, count)) = times_used.get_mut(&(node, next_node)) {
                *count += 1;
            }
            else {
                times_used.insert((node, next_node), (path.len(), 1));
            }

            let next_nodes = network.get(next_node).unwrap().iter()
                .filter(|neighbor| path.len() + 1 < levels.len() && levels[path.len() + 1].contains(neighbor))
                .collect();
            path.push((next_node, next_nodes));
        }
        else {
            path.pop();
        }
    }

    let half_max_distance = max_distance / 2;
    let mut edges = Vec::from_iter(times_used.keys().map(|&(node_1, node_2)| (node_1.clone(), node_2.clone())));
    edges.sort_unstable_by_key(|(node_1, node_2)| {
        let &(distance, count) = times_used.get(&(node_1, node_2)).unwrap();
        count * 1000 + max_distance - distance.abs_diff(half_max_distance)
    });

    let mut start_side = 0;
    let mut end_side = 0;

    let mut edge_1 = edges.len() - 1;
    let mut edge_2 = edges.len() - 2;
    let mut edge_3 = edges.len() - 3;

    while start_side == end_side {
        let mut network = network.clone();
        let edges_to_remove = [&edges[edge_1], &edges[edge_2], &edges[edge_3]];
        // println!("trying {edge_1} {edge_2} {edge_3}");
        for (node_1, node_2) in edges_to_remove {
            network.get_mut(node_1).unwrap().retain(|neighbor| neighbor != node_2);
            network.get_mut(node_2).unwrap().retain(|neighbor| neighbor != node_1);
        }

        let mut visited_from_start = BTreeSet::new();
        let mut path = vec![&start_node];

        while let Some(&node) = path.last() {
            visited_from_start.insert(node);

            if let Some(next_node) = network.get(node).unwrap().iter().find(|next_node| !visited_from_start.contains(next_node)) {
                path.push(next_node);
            }
            else {
                path.pop();
            }
        }

        let mut visited_from_end = BTreeSet::new();
        let mut path = vec![&end_node];

        while let Some(&node) = path.last() {
            visited_from_end.insert(node);
            
            if let Some(next_node) = network.get(node).unwrap().iter().find(|next_node| !visited_from_end.contains(next_node)) {
                path.push(next_node);
            }
            else {
                path.pop();
            }
        }

        start_side = visited_from_start.len();
        end_side = visited_from_end.len();

        if edge_1 >= edges.len() - 1 {
            if edge_2 >= edges.len() - 2 {
                edge_2 = edge_3;
                edge_3 -= 1;
            }
            else {
                edge_2 += 1;
            }
            edge_1 = edge_2 + 1;
        }
        else {
            edge_1 += 1;
        }
    }

    println!("[25p1] {} * {} = {}", start_side, end_side, start_side * end_side);
}
