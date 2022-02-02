
use std::collections::{HashMap};

#[derive(Debug, Eq, Hash)]
struct Pair {
    x: u8,
    y: u8
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}


#[derive(Debug)]
struct Graph {
    graph: HashMap<Pair, Vec<Pair>>,
}

impl Graph {

    pub fn push(&mut self, key: Pair, value: Vec<Pair>){
        self.graph.insert(key, value);
    }

    pub fn view_map(&self) {
        for (key, adjacent) in &self.graph {
            println!("{:?}: {:?}", key, adjacent);
        }
    }
}

fn push_fom_values(graph: &mut Graph, x1: u8, y1: u8, ax1: u8, ay1: u8, ax2: u8, ay2: u8) {
    graph.push(Pair {
        x: x1,
        y: y1
    }, vec!(
        Pair {
            x: ax1,
            y: ay1
        },
        Pair {
            x: ax2,
            y: ay2
        }
    ));
}

fn main() {
    let mut new_graph = Graph { graph: HashMap::new() };
    
    new_graph.view_map();
}

// (1,1): set([(1,2), (2,1)]),
// (1,2): set([(2,2), (1,3)]),
// (1,3): set([(1,4), (1,2)]),
// (1,4): set([(1,3), (2,4)]),
// (2,1): set([(1,1), (1,3)]),
// (2,2): set([(2,3), (1,2)]),
// (2,3): set([(2,2)]), 
// (2,4): set([(1,4), (3,4)]),
// (3,1): set([(2,1), (3,2)]),
// (3,2): set([(3,1), (3,3)]),
// (3,3): set([(3,2), (3,4)]),
// (3,4): set([(2,4), (3,3)]),