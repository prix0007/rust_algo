mod graph;
use std::{collections::{HashSet, VecDeque}, thread::sleep, time::Duration};

use graph::{Edge, Graph, Pair, Vertex};
mod init;
use init::construct_init_graph;

//    1  2  3  4
// 1 |s |  |__|  |
// 2 |  |__|f_|  |
// 3 |  |  |  |  |

pub fn depth_first_search(graph: &Graph, root: Vertex, objective: Vertex) -> Option<Vec<Pair>> {
    let mut visited: HashSet<Vertex> = HashSet::new();
    let mut history: Vec<Pair> = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(root);
    visited.insert(root);

    // While there is an element in the queue
    // get the first element of the vertex queue
    while let Some(current_vertex) = queue.pop_front() {
        // Added current vertex in the history of visiteds vertex
        history.push(current_vertex.value());

        // Verify if this vertex is the objective
        if current_vertex == objective {
            // Return the Optional with the history of visiteds vertex
            return Some(history);
        }

        // For each over the neighbors of current vertex
        for neighbor in current_vertex.neighbors(graph).into_iter().rev() {
            // Insert in the HashSet of visiteds if this value not exist yet
            if visited.insert(neighbor) {
                // Add the neighbor on front of queue
                queue.push_front(neighbor);
                // println!("Visited: {:?}", visited);
                // println!("Queue: {:?}", queue);
                // sleep(Duration::new(2, 0));
            }
        }
    }

    // If all vertex is visited and the objective is not found
    // return a Optional with None value
    None
}

fn main() {
    let mut new_graph = Graph {
        vertex: Vec::new(),
        edges: Vec::new(),
    };
    new_graph = construct_init_graph(new_graph);
    new_graph.view_map();
    
    let sol = depth_first_search(
        &new_graph,
        Vertex {
            e: Pair { x: 1, y: 1 },
        },
        Vertex {
            e: Pair { x: 3, y: 2 },
        },
    );
    println!("{:?}", sol);
}


//    1  2  3  4
// 1 |s     __   |
// 2 |  |__ f_|  |
// 3 |           |
//    -----------
