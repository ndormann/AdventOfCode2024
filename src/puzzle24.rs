use crate::prelude::Aoc2024;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::{Rc, Weak};

#[derive(Debug, PartialEq, Eq)]
enum NodeType {
    AND,
    OR,
    XOR,
    Input,
}

impl NodeType {
    fn apply(&self, a: &NodeState, b: &NodeState) -> NodeState {
        match self {
            NodeType::AND => {
                if let NodeState::Set(a) = a {
                    if let NodeState::Set(b) = b {
                        NodeState::Set(*a & *b)
                    } else {
                        NodeState::Floating
                    }
                } else {
                    NodeState::Floating
                }
            }
            NodeType::OR => {
                if let NodeState::Set(a) = a {
                    if *a {
                        NodeState::Set(*a)
                    } else if let NodeState::Set(b) = b {
                        NodeState::Set(*b)
                    } else {
                        NodeState::Floating
                    }
                } else {
                    NodeState::Floating
                }
            }
            NodeType::XOR => {
                if let NodeState::Set(a) = a {
                    if let NodeState::Set(b) = b {
                        NodeState::Set(a ^ b)
                    } else {
                        NodeState::Floating
                    }
                } else {
                    NodeState::Floating
                }
            }
            NodeType::Input => NodeState::Floating,
        }
    }
}

impl From<&str> for NodeType {
    fn from(value: &str) -> Self {
        match value {
            "AND" => NodeType::AND,
            "OR" => NodeType::OR,
            "XOR" => NodeType::XOR,
            _ => unreachable!(),
        }
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Clone, Copy)]
enum NodeState {
    Floating,
    Set(bool),
}

//#[derive(Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Debug)]
struct Node {
    state: NodeState,
    name: String,
    inputs: Option<[Weak<RefCell<Node>>; 2]>,
    outputs: Vec<Weak<RefCell<Node>>>,
    operator: NodeType,
}

impl Node {
    fn evaluate(&mut self) -> NodeState {
        if self.state == NodeState::Floating && self.inputs.is_some() {
            match &self.inputs {
                Some(inputs) => {
                    let state_a = inputs[0].upgrade().unwrap().borrow_mut().evaluate();
                    let state_b = inputs[1].upgrade().unwrap().borrow_mut().evaluate();
                    self.state = self.operator.apply(&state_a, &state_b);
                }
                _ => {}
            }
        }
        self.state
    }
}

struct Machine {
    nodes: HashMap<String, Rc<RefCell<Node>>>,
}

impl Machine {
    pub fn parse(content: &String) -> Self {
        let mut parts = content.split("\n\n");

        // Parse inputs
        let inputs_part = parts.next().unwrap();
        let mut nodes: HashMap<String, Rc<RefCell<Node>>> = inputs_part
            .lines()
            .map(|line| {
                let mut parts = line.split(":").map(str::trim);
                let name = parts.next().unwrap().to_owned();
                let state = parts.next().unwrap().parse::<u8>().unwrap() == 1;
                (
                    name.clone(),
                    Rc::new(RefCell::new(Node {
                        state: NodeState::Set(state),
                        outputs: Vec::new(),
                        inputs: None,
                        operator: NodeType::Input,
                        name,
                    })),
                )
            })
            .collect();

        if let Some(connections_part) = parts.next() {
            for line in connections_part.lines() {
                let elements: Vec<&str> = line.split_whitespace().collect();
                let name_a = elements[0].to_owned();
                let name_b = elements[2].to_owned();
                let output = elements[4].to_owned();
                let op_type = NodeType::from(elements[1]);

                // Ensure all nodes exist in `nodes`
                let node_a = nodes
                    .entry(name_a.clone())
                    .or_insert_with(|| {
                        Rc::new(RefCell::new(Node {
                            name: name_a.clone(),
                            inputs: None,
                            outputs: Vec::new(),
                            state: NodeState::Floating,
                            operator: NodeType::Input,
                        }))
                    })
                    .clone();
                let node_b = nodes
                    .entry(name_b.clone())
                    .or_insert_with(|| {
                        Rc::new(RefCell::new(Node {
                            name: name_b.clone(),
                            inputs: None,
                            outputs: Vec::new(),
                            state: NodeState::Floating,
                            operator: NodeType::Input,
                        }))
                    })
                    .clone();
                let node_out = nodes
                    .entry(output.clone())
                    .or_insert_with(|| {
                        Rc::new(RefCell::new(Node {
                            name: output.clone(),
                            inputs: None,
                            outputs: Vec::new(),
                            state: NodeState::Floating,
                            operator: NodeType::Input,
                        }))
                    })
                    .clone();
                // Add the output node to the outputs of node_a and node_b
                node_a.borrow_mut().outputs.push(Rc::downgrade(&node_out));
                node_b.borrow_mut().outputs.push(Rc::downgrade(&node_out));
                node_out.borrow_mut().inputs =
                    Some([Rc::downgrade(&node_a), Rc::downgrade(&node_b)]);
                node_out.borrow_mut().operator = op_type;
            }
        }

        Self { nodes }
    }

    pub fn evaluate(&mut self) {
        for node in self.nodes.values_mut() {
            if node.borrow().name.starts_with('z') {
                node.borrow_mut().evaluate();
            }
        }
    }

    pub fn variable(&self, var_name: char) -> usize {
        let mut output_nodes: Vec<&Rc<RefCell<Node>>> = self
            .nodes
            .values()
            .filter_map(|node| {
                let name = &node.borrow().name;
                if name.starts_with(var_name) {
                    Some(node)
                } else {
                    None
                }
            })
            .collect();
        output_nodes.sort_by(|a, b| b.borrow().name.cmp(&a.borrow().name));

        let mut total: usize = 0;
        for node in output_nodes {
            total <<= 1;
            if node.borrow().state == NodeState::Set(true) {
                total += 1;
            }
        }
        total
    }
}

pub struct Puzzle24 {}

impl Aoc2024 for Puzzle24 {
    fn name(&self) -> String {
        "Day 24: Crossed Wires".to_string()
    }

    fn solve_a(&self, input: &String) -> String {
        let mut machine = Machine::parse(input);
        machine.evaluate();
        machine.variable('z').to_string()
    }

    fn solve_b(&self, input: &String) -> String {
        // solve by finding wrong circuits through knowledge of adder and circuit design
        // originally I wanted to implement a complete circuit solver, but dropped it due to time constraints
        // this is the reason for the Rc<RefCell<Node>>> though...
        let mut machine = Machine::parse(input);
        machine.evaluate();
        let nodes: Vec<&Rc<RefCell<Node>>> = machine.nodes.values().collect();
        let mut wrong: HashSet<String> = HashSet::new();

        for node in &nodes {
            let _node = node.borrow();
            if _node.operator != NodeType::Input {
                if _node.name.starts_with('z') && _node.operator != NodeType::XOR {
                    wrong.insert(node.borrow().name.clone());
                }
                let op1_name = _node.inputs.clone().unwrap()[0]
                    .upgrade()
                    .unwrap()
                    .borrow()
                    .name
                    .clone();
                let op2_name = _node.inputs.clone().unwrap()[1]
                    .upgrade()
                    .unwrap()
                    .borrow()
                    .name
                    .clone();
                if _node.operator == NodeType::XOR
                    && !['x', 'y', 'z'].contains(&op1_name.chars().next().unwrap())
                    && !['x', 'y', 'z'].contains(&op2_name.chars().next().unwrap())
                    && !['x', 'y', 'z'].contains(&_node.name.chars().next().unwrap())
                {
                    wrong.insert(node.borrow().name.clone());
                }
                if _node.operator == NodeType::AND && !(op1_name == "x00" || op2_name == "x00") {
                    for inner_node in &nodes {
                        if inner_node.borrow().operator != NodeType::OR
                            && inner_node.borrow().operator != NodeType::Input
                        {
                            let op1_name = inner_node.borrow().inputs.clone().unwrap()[0]
                                .upgrade()
                                .unwrap()
                                .borrow()
                                .name
                                .clone();
                            let op2_name = inner_node.borrow().inputs.clone().unwrap()[1]
                                .upgrade()
                                .unwrap()
                                .borrow()
                                .name
                                .clone();
                            if op1_name == _node.name || op2_name == _node.name {
                                wrong.insert(node.borrow().name.clone());
                            }
                        }
                    }
                }
                if _node.operator == NodeType::XOR {
                    for inner_node in &nodes {
                        if inner_node.borrow().operator == NodeType::OR {
                            let op1_name = inner_node.borrow().inputs.clone().unwrap()[0]
                                .upgrade()
                                .unwrap()
                                .borrow()
                                .name
                                .clone();
                            let op2_name = inner_node.borrow().inputs.clone().unwrap()[1]
                                .upgrade()
                                .unwrap()
                                .borrow()
                                .name
                                .clone();
                            if op1_name == _node.name || op2_name == _node.name {
                                wrong.insert(node.borrow().name.clone());
                            }
                        }
                    }
                }
            }
        }
        let mut wrong: Vec<String> = wrong.into_iter().collect();
        wrong.sort();
        wrong[0..8].join(",")
    }
}
