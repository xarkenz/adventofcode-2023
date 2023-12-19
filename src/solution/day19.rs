use super::*;

enum Category {
    X,
    M,
    A,
    S,
}

impl Category {
    fn from_str(value: &str) -> Self {
        match value {
            "x" => Self::X,
            "m" => Self::M,
            "a" => Self::A,
            "s" => Self::S,
            _ => panic!()
        }
    }
}

enum WorkflowResult {
    Accepted,
    Rejected,
    Continue(String),
}

impl WorkflowResult {
    fn from_str(value: &str) -> Self {
        match value {
            "A" => Self::Accepted,
            "R" => Self::Rejected,
            _ => Self::Continue(value.to_owned())
        }
    }
}

enum WorkflowRule {
    LessThan(Category, i64, WorkflowResult),
    GreaterThan(Category, i64, WorkflowResult),
}

impl WorkflowRule {
    fn from_str(value: &str) -> Self {
        let (rule, result) = value.split_once(':').unwrap();
        if let Some((category, threshold)) = rule.split_once('<') {
            Self::LessThan(Category::from_str(category), threshold.parse().unwrap(), WorkflowResult::from_str(result))
        }
        else if let Some((category, threshold)) = rule.split_once('>') {
            Self::GreaterThan(Category::from_str(category), threshold.parse().unwrap(), WorkflowResult::from_str(result))
        }
        else {
            panic!()
        }
    }

    fn get_result(&self, part: &Part) -> Option<&WorkflowResult> {
        match self {
            Self::LessThan(category, threshold, result) => {
                (part.get_value(category) < *threshold).then_some(result)
            },
            Self::GreaterThan(category, threshold, result) => {
                (part.get_value(category) > *threshold).then_some(result)
            },
        }
    }

    fn filter_results(&self, mut parts: PartValues) -> (PartValues, &WorkflowResult, PartValues) {
        let (category, result, splice) = match self {
            Self::LessThan(category, threshold, result) => {
                (category, result, Interval::new(parts.bounds.start(), *threshold))
            },
            Self::GreaterThan(category, threshold, result) => {
                (category, result, Interval::new(*threshold + 1, parts.bounds.end()))
            },
        };
        let filtered = parts.get_values_mut(category).splice_interval(splice);
        let mut filtered_parts = parts.clone();
        *filtered_parts.get_values_mut(category) = filtered;
        (filtered_parts, result, parts)
    }
}

struct Workflow {
    name: String,
    rules: Vec<WorkflowRule>,
    default_result: WorkflowResult,
}

impl Workflow {
    fn from_str(value: &str) -> Self {
        let (name, rest) = value.split_once('{').unwrap();
        let mut rules =  Vec::from_iter(rest.split(','));
        let default_result = WorkflowResult::from_str(rules.pop().unwrap().strip_suffix('}').unwrap());
        Self {
            name: name.to_owned(),
            rules: rules.iter().map(|&rule| WorkflowRule::from_str(rule)).collect(),
            default_result,
        }
    }

    fn get_result(&self, part: &Part) -> &WorkflowResult {
        for rule in &self.rules {
            if let Some(result) = rule.get_result(part) {
                return result;
            }
        }
        &self.default_result
    }

    fn filter_results(&self, parts: PartValues) -> Vec<(PartValues, &WorkflowResult)> {
        let mut results = Vec::new();
        let mut remaining_parts = parts;
        for rule in &self.rules {
            let (filtered_parts, result, parts) = rule.filter_results(remaining_parts);
            remaining_parts = parts;
            results.push((filtered_parts, result));
        }
        results.push((remaining_parts, &self.default_result));
        results
    }
}

#[derive(Clone)]
struct Part {
    x_value: i64,
    m_value: i64,
    a_value: i64,
    s_value: i64,
}

impl Part {
    fn from_str(value: &str) -> Self {
        let mut value_split = value.split(',');
        Self {
            x_value: value_split.next().unwrap()[3..].parse().unwrap(),
            m_value: value_split.next().unwrap()[2..].parse().unwrap(),
            a_value: value_split.next().unwrap()[2..].parse().unwrap(),
            s_value: value_split.next().unwrap()[2..].strip_suffix('}').unwrap().parse().unwrap(),
        }
    }

    fn get_value(&self, category: &Category) -> i64 {
        match category {
            Category::X => self.x_value,
            Category::M => self.m_value,
            Category::A => self.a_value,
            Category::S => self.s_value,
        }
    }

    fn rating(&self) -> i64 {
        self.x_value + self.m_value + self.a_value + self.s_value
    }
}

#[derive(Clone)]
struct PartValues {
    x_values: IntervalSet,
    m_values: IntervalSet,
    a_values: IntervalSet,
    s_values: IntervalSet,
    bounds: Interval,
}

impl PartValues {
    fn new(bounds: Interval) -> Self {
        Self {
            x_values: IntervalSet::new(),
            m_values: IntervalSet::new(),
            a_values: IntervalSet::new(),
            s_values: IntervalSet::new(),
            bounds,
        }
    }

    fn from_range(min: i64, max: i64) -> Self {
        let bounds = Interval::new(min, max + 1);
        let mut init_values = IntervalSet::new();
        init_values.apply_interval(bounds);
        Self {
            x_values: init_values.clone(),
            m_values: init_values.clone(),
            a_values: init_values.clone(),
            s_values: init_values,
            bounds,
        }
    }

    fn get_values(&self, category: &Category) -> &IntervalSet {
        match category {
            Category::X => &self.x_values,
            Category::M => &self.m_values,
            Category::A => &self.a_values,
            Category::S => &self.s_values,
        }
    }

    fn get_values_mut(&mut self, category: &Category) -> &mut IntervalSet {
        match category {
            Category::X => &mut self.x_values,
            Category::M => &mut self.m_values,
            Category::A => &mut self.a_values,
            Category::S => &mut self.s_values,
        }
    }

    fn apply(&mut self, other: &PartValues) {
        self.x_values.intersect(&other.x_values);
        self.m_values.intersect(&other.m_values);
        self.a_values.intersect(&other.a_values);
        self.s_values.intersect(&other.s_values);
    }

    fn get_combinations(&self) -> i64 {
        self.x_values.cardinality() * self.m_values.cardinality() * self.a_values.cardinality() * self.s_values.cardinality()
    }
}

impl std::fmt::Display for PartValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ x:{}, m:{}, a:{}, s:{} }}", self.x_values.cardinality(), self.m_values.cardinality(), self.a_values.cardinality(), self.s_values.cardinality())
    }
}

pub fn run() {
    let mut lines = get_input("day19.txt").lines().map(expect_line);

    let mut workflows: BTreeMap<String, Workflow> = BTreeMap::new();

    while let Some(line) = lines.next().and_then(|line| (!line.is_empty()).then_some(line)) {
        let workflow = Workflow::from_str(&line);
        workflows.insert(workflow.name.clone(), workflow);
    }

    let mut rating_total: i64 = 0;

    for line in lines {
        let part = Part::from_str(&line);
        
        let mut workflow = workflows.get("in").unwrap();
        let mut result = workflow.get_result(&part);
        while let WorkflowResult::Continue(next_workflow_name) = result {
            workflow = workflows.get(next_workflow_name).unwrap();
            result = workflow.get_result(&part);
        }

        if let WorkflowResult::Accepted = result {
            rating_total += part.rating();
        }
    }

    println!("[19p1] {rating_total}");

    let parts = PartValues::from_range(1, 4000);
    let mut accepted_parts = Vec::new();
    let mut workflow_queue: VecDeque<(&Workflow, PartValues)> = VecDeque::new();
    workflow_queue.push_back((workflows.get("in").unwrap(), parts));

    while let Some((workflow, parts)) = workflow_queue.pop_front() {
        for (parts, result) in workflow.filter_results(parts) {
            match result {
                WorkflowResult::Accepted => {
                    accepted_parts.push(parts);
                },
                WorkflowResult::Rejected => {},
                WorkflowResult::Continue(next_workflow_name) => {
                    workflow_queue.push_back((workflows.get(next_workflow_name).unwrap(), parts));
                },
            }
        }
    }

    let accepted_combinations: i64 = accepted_parts.iter().map(|parts| parts.get_combinations()).sum();
    println!("[19p2] {accepted_combinations}");
}
