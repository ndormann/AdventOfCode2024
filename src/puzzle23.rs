use crate::prelude::Aoc2024;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter, Write};

#[derive(Clone, Copy, Eq, Hash, PartialEq, Ord, PartialOrd, Debug)]
struct NameType([u8; 2]);

#[derive(Clone, Copy, Eq, Hash, PartialEq, Ord, PartialOrd)]
struct Triple([NameType; 3]);

impl Triple {
    fn new(a: NameType, b: NameType, c: NameType) -> Self {
        let mut content = [a, b, c];
        content.sort();
        Self { 0: content }
    }
}

impl Debug for Triple {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}, {}, {}", self.0[0], self.0[1], self.0[2]))
    }
}

impl From<&str> for NameType {
    fn from(value: &str) -> Self {
        Self {
            0: value
                .bytes()
                .take(2)
                .collect::<Vec<u8>>()
                .try_into()
                .expect("Name must have exactly 2 characters"),
        }
    }
}

impl NameType {
    pub fn as_str(&self) -> String {
        self.0.iter().map(|&el| char::from(el)).collect()
    }

    fn is_headhistorian(&self) -> bool {
        self.0[0] == 't' as u8
    }
}

impl Display for NameType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.0[0] as char)?;
        f.write_char(self.0[1] as char)
    }
}

#[derive(Clone)]
struct Node {
    name: NameType,
    neighbours: HashSet<NameType>, // Store neighbors by their names for simplicity
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name.as_str().as_str())?;
        f.write_str(": ")?;
        for neighbour in &self.neighbours {
            f.write_str(neighbour.as_str().as_str())?;
            f.write_str(", ")?
        }
        Ok(())
    }
}

impl Node {
    pub fn new(name: NameType) -> Self {
        Self {
            name,
            neighbours: HashSet::new(),
        }
    }
}

struct Graph {
    nodes: HashMap<NameType, Node>, // The graph holds ownership of all nodes
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn parse_connection(&mut self, input: &str) {
        let parts: Vec<&str> = input.trim().split('-').collect();
        if parts.len() != 2 {
            panic!("Input must be in the format 'XX-YY' but is {input}");
        }

        let name_a = NameType::from(parts[0]);
        let name_b = NameType::from(parts[1]);

        // Ensure both nodes exist in the graph
        self.nodes
            .entry(name_a)
            .or_insert_with(|| Node::new(name_a));
        self.nodes
            .entry(name_b)
            .or_insert_with(|| Node::new(name_b));

        // Add neighbours
        self.nodes
            .get_mut(&name_a)
            .unwrap()
            .neighbours
            .insert(name_b);
        self.nodes
            .get_mut(&name_b)
            .unwrap()
            .neighbours
            .insert(name_a);
    }

    pub fn find_largest_clique(&self) -> HashSet<NameType> {
        let mut all_nodes: HashSet<NameType> = self.nodes.keys().cloned().collect();
        let mut largest_clique = HashSet::new();

        fn bron_kerbosch(
            graph: &Graph,
            r: &mut HashSet<NameType>,
            p: &mut HashSet<NameType>,
            x: &mut HashSet<NameType>,
            largest_clique: &mut HashSet<NameType>,
        ) {
            if p.is_empty() && x.is_empty() {
                // If P and X are both empty, R is a maximal clique
                if r.len() > largest_clique.len() {
                    *largest_clique = r.clone();
                }
                return;
            }

            // Iterate over a copy of P to allow mutation during the loop
            let p_copy: Vec<_> = p.clone().into_iter().collect();
            for v in p_copy {
                let mut r_next = r.clone();
                r_next.insert(v.clone());

                let neighbors = &graph.nodes[&v].neighbours;

                let mut p_next: HashSet<NameType> = p.intersection(neighbors).cloned().collect();
                let mut x_next: HashSet<NameType> = x.intersection(neighbors).cloned().collect();

                bron_kerbosch(graph, &mut r_next, &mut p_next, &mut x_next, largest_clique);

                // Move v from P to X
                p.remove(&v);
                x.insert(v);
            }
        }

        // Start the Bron-Kerbosch algorithm
        bron_kerbosch(
            self,
            &mut HashSet::new(), // R: initially empty
            &mut all_nodes,      // P: all nodes
            &mut HashSet::new(), // X: initially empty
            &mut largest_clique,
        );

        largest_clique
    }
}

pub struct Puzzle23 {}

impl Aoc2024 for Puzzle23 {
    fn name(&self) -> String {
        "Day 23: LAN Party".to_string()
    }

    fn solve_a(&self, input: &String) -> String {
        let mut graph = Graph::new();
        for line in input.lines() {
            graph.parse_connection(line);
        }

        let mut triples: HashSet<Triple> = HashSet::new();

        for node in graph.nodes.values() {
            if !node.name.is_headhistorian() {
                continue;
            }
            if node.neighbours.len() > 2 {
                let mut set = node.neighbours.clone();
                set.insert(node.name);
                for neighbour in &node.neighbours {
                    let second_node = &graph.nodes[neighbour];
                    for neighbour in &second_node.neighbours {
                        let third_node = &graph.nodes[neighbour];
                        if third_node.neighbours.contains(&node.name) {
                            triples.insert(Triple::new(
                                node.name,
                                second_node.name,
                                third_node.name,
                            ));
                        }
                    }
                }
            }
        }

        triples.len().to_string()
    }

    fn solve_b(&self, input: &String) -> String {
        let mut graph = Graph::new();
        for line in input.lines() {
            graph.parse_connection(line);
        }
        // Find the largest clique with bron kerbosch
        let largest_clique = graph.find_largest_clique();

        let mut answer: Vec<String> = largest_clique.iter().map(NameType::to_string).collect();
        answer.sort();
        answer.join(",")
    }
}
