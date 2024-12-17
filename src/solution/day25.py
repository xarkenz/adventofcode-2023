import random

components: dict[str, list[str]] = {}

with open("src/input/day25.txt") as f:
    for line in f.readlines():
        component, adjacencies = line.strip().split(": ")
        components[component] = adjacencies.split()

for component, adjacencies in components.copy().items():
    for next_group in adjacencies:
        if next_group not in components:
            components[next_group] = [component]
        elif component not in components[next_group]:
            components[next_group].append(component)

while True:
    groups = {component: (1, adjacencies.copy()) for component, adjacencies in components.items()}
    while len(groups) > 2:
        group = random.choice(list(groups.keys()))
        count, adjacencies = groups.pop(group)
        next_group = random.choice(adjacencies)
        next_count, next_adjacencies = groups[next_group]
        while group in next_adjacencies:
            next_adjacencies.remove(group)
        for adjacency in adjacencies:
            if adjacency != next_group:
                next_adjacencies.append(adjacency)
            adjacency_adjacencies = groups[adjacency][1]
            while group in adjacency_adjacencies:
                adjacency_adjacencies[adjacency_adjacencies.index(group)] = next_group
        next_count += count
        groups[next_group] = next_count, next_adjacencies
    print(groups)
    group1, group2 = list(groups.keys())
    group1_count, group1_adjacencies = groups[group1]
    group2_count, group2_adjacencies = groups[group2]
    if len(group1_adjacencies) != len(group2_adjacencies):
        print("bad")
        break
    elif len(group1_adjacencies) == 3:
        print(f"{group1_count} * {group2_count} = {group1_count * group2_count}")
        break
