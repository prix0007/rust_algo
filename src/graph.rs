use std::collections::VecDeque;

#[derive(Debug, Eq, Hash, Clone, Copy)]
pub struct Pair {
    pub x: u8,
    pub y: u8,
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vertex {
    pub e: Pair,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Edge {
    pub x: Pair,
    pub y: Pair,
}

#[derive(Debug, Clone)]
pub struct Graph {
    pub vertex: Vec<Pair>,
    pub edges: Vec<Edge>,
}

impl Graph {
    pub fn push_vertex(&mut self, pair: Pair) {
        self.vertex.push(pair);
    }

    pub fn push_edges(&mut self, edge: Edge) {
        self.edges.push(edge);
    }

    pub fn view_map(&self) {
        println!("===================");
        println!("Vertex:");
        for vertex in &self.vertex {
            print!("| {:?} | ", vertex);
        }
        println!("\n--------------------");
        println!("Edges:");
        for edge in &self.edges {
            print!("| {:?} -> {:?} |\n ", edge.x, edge.y);
        }
        println!("\n===================");
    }
}

impl From<Pair> for Vertex {
    fn from(pair: Pair) -> Self {
        Vertex { e: pair }
    }
}

impl Vertex {
    pub fn value(&self) -> Pair {
        self.e
    }

    pub fn neighbors(&self, graph: &Graph) -> VecDeque<Vertex> {
        let mut neighbours = VecDeque::<Vertex>::new();
        for edge in graph.edges.clone().into_iter() {
            if self.e == edge.x {
                neighbours.push_back(Vertex { e: edge.y })
            }
        }
        neighbours
    }
}

impl From<(Pair, Pair)> for Edge {
    fn from(edge: (Pair, Pair)) -> Self {
        Edge {
            x: edge.0,
            y: edge.1,
        }
    }
}
