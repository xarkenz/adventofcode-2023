use super::*;

struct Module {
    name: String,
    is_flip_flop: bool,
    flip_state: bool,
    inputs: BTreeMap<String, bool>,
    outputs: Vec<String>,
}

impl Module {
    fn flip_flop(name: String, outputs: Vec<String>) -> Self {
        Self {
            name,
            is_flip_flop: true,
            flip_state: false,
            inputs: BTreeMap::new(),
            outputs,
        }
    }

    fn conjunction(name: String, outputs: Vec<String>) -> Self {
        Self {
            name,
            is_flip_flop: false,
            flip_state: false,
            inputs: BTreeMap::new(),
            outputs,
        }
    }

    fn handle_pulse(&mut self, source: String, pulse: bool) -> Option<bool> {
        if self.is_flip_flop {
            if !pulse {
                self.flip_state = !self.flip_state;
                Some(self.flip_state)
            }
            else {
                None
            }
        }
        else {
            self.inputs.insert(source, pulse);
            Some(!self.inputs.values().all(|&pulse| pulse))
        }
    }
}

pub fn run() {
    let mut modules: BTreeMap<String, Module> = BTreeMap::new();
    let mut broadcast_outputs: Vec<String> = Vec::new();
    let mut rx_parent = String::new();

    for line in get_input("day20.txt").lines().map(expect_line) {
        let (name, outputs) = line.split_once(" -> ").unwrap();
        let outputs = Vec::from_iter(outputs.split(", ").map(String::from));

        if let Some(name) = name.strip_prefix('%') {
            modules.insert(name.into(), Module::flip_flop(name.into(), outputs));
        }
        else if let Some(name) = name.strip_prefix('&') {
            if let Some("rx") = outputs.get(0).map(String::as_str) {
                rx_parent = name.into();
            }

            modules.insert(name.into(), Module::conjunction(name.into(), outputs));
        }
        else {
            broadcast_outputs = outputs;
        }
    }

    for name in Vec::from_iter(modules.keys().cloned()) {
        for output in modules.get(&name).unwrap().outputs.clone() {
            if let Some(module) = modules.get_mut(&output) {
                module.inputs.insert(name.clone(), false);
            }
        }
    }

    let mut rx_dependencies: BTreeMap<String, (Option<u64>, Option<u64>)> = modules.get(&rx_parent).unwrap().inputs.keys()
        .map(|input| (input.clone(), (None, None)))
        .collect();

    let mut high_count: u64 = 0;
    let mut low_count: u64 = 0;

    for button_count in 1.. {
        // 1 low for button -> broadcaster
        if button_count <= 1000 {
            low_count += 1;
        }
        else if button_count >= 20000 && rx_dependencies.values().all(|&(_, cycle)| cycle.is_some()) {
            break;
        }

        let mut pulses: VecDeque<(String, bool, String)> = broadcast_outputs.iter()
            .map(|name| ("broadcaster".into(), false, name.clone()))
            .collect();

        while let Some((source, pulse, destination)) = pulses.pop_front() {
            if button_count <= 1000 {
                if pulse {
                    high_count += 1;
                }
                else {
                    low_count += 1;
                }
            }

            if let Some(module) = modules.get_mut(&destination) {
                if let Some(pulse) = module.handle_pulse(source, pulse) {
                    pulses.extend(module.outputs.iter().map(|output| (destination.clone(), pulse, output.clone())));

                    if let Some((last_button_count, last_cycle)) = rx_dependencies.get_mut(&destination) {
                        if pulse {
                            if let Some(last_button_count) = last_button_count {
                                *last_cycle = Some(button_count - *last_button_count);
                            }
                            *last_button_count = Some(button_count);
                        }
                    }
                }
            }
        }
    }

    let rx_button_count = rx_dependencies.values().fold(1, |combined_cycle, &(_, cycle)| {
        lcm(combined_cycle, cycle.unwrap())
    });

    println!("[20p1] {high_count} high * {low_count} low = {} total", high_count * low_count);
    println!("[20p2] {rx_button_count}");
}
