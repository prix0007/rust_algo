use crate::graph::{Edge, Graph, Pair, Vertex};

pub fn construct_init_graph(mut new_graph: Graph) -> Graph {
    new_graph.push_vertex(Pair { x: 1, y: 1 });
    new_graph.push_vertex(Pair { x: 1, y: 2 });
    new_graph.push_vertex(Pair { x: 1, y: 3 });
    new_graph.push_vertex(Pair { x: 2, y: 1 });
    new_graph.push_vertex(Pair { x: 2, y: 2 });
    new_graph.push_vertex(Pair { x: 2, y: 3 });
    new_graph.push_vertex(Pair { x: 3, y: 1 });
    new_graph.push_vertex(Pair { x: 3, y: 2 });
    new_graph.push_vertex(Pair { x: 3, y: 3 });
    new_graph.push_vertex(Pair { x: 4, y: 1 });
    new_graph.push_vertex(Pair { x: 4, y: 2 });
    new_graph.push_vertex(Pair { x: 4, y: 3 });

    new_graph.push_edges(
        Edge {
            x: Pair { x: 1, y: 1 },
            y: Pair { x: 1, y: 2 }
        }
    );
    new_graph.push_edges(
        Edge {
            x: Pair { x: 1, y: 1 },
            y: Pair { x: 2, y: 1 }
        }
    );
    new_graph.push_edges(
        Edge {
            x: Pair { x: 1, y: 2 },
            y: Pair { x: 1, y: 1 }
        }
    );
    new_graph.push_edges(
        Edge {
            x: Pair { x: 1, y: 2 },
            y: Pair { x: 1, y: 3 }
        }
    );
    new_graph.push_edges(
        Edge {
            x: Pair { x: 1, y: 3 },
            y: Pair { x: 1, y: 2 }
        }
    );
    new_graph.push_edges(
        Edge {
            x: Pair { x: 1, y: 3 },
            y: Pair { x: 2, y: 3 }
        }
    );
    new_graph.push_edges(
        Edge {
            x: Pair { x: 2, y: 1 },
            y: Pair { x: 1, y: 1 }
        }
    );
    new_graph.push_edges(
        Edge {
            x: Pair { x: 2, y: 1 },
            y: Pair { x: 2, y: 2 }
        }
    );
    new_graph.push_edges(
        Edge {
            x: Pair { x: 2, y: 1 },
            y: Pair { x: 3, y: 1 }
        }
    );
    new_graph.push_edges(
        Edge {
            x: Pair { x: 2, y: 2 },
            y: Pair { x: 2, y: 1 }
        }
    );
    new_graph.push_edges(
        Edge {
            x: Pair { x: 2, y: 2 },
            y: Pair { x: 3, y: 2 }
        }
    );
    new_graph.push_edges(
        Edge {
            x: Pair { x: 2, y: 3 },
            y: Pair { x: 1, y: 3 }
        }
    );
    new_graph.push_edges(
        Edge {
            x: Pair { x: 2, y: 3 },
            y: Pair { x: 3, y: 3 }
        }
    );
    new_graph.push_edges(
        Edge {
            x: Pair { x: 3, y: 1 },
            y: Pair { x: 2, y: 1 }
        }
    );
    new_graph.push_edges(
        Edge {
            x: Pair { x: 3, y: 1 },
            y: Pair { x: 4, y: 1 }
        }
    );
    new_graph.push_edges(
        Edge {
            x: Pair { x: 3, y: 2 },
            y: Pair { x: 2, y: 2 }
        }
    );
    new_graph.push_edges(
        Edge {
            x: Pair { x: 3, y: 3 },
            y: Pair { x: 2, y: 3 }
        }
    );
    new_graph.push_edges(
        Edge {
            x: Pair { x: 3, y: 3 },
            y: Pair { x: 4, y: 3 }
        }
    );
    new_graph.push_edges(
        Edge {
            x: Pair { x: 4, y: 1 },
            y: Pair { x: 3, y: 1 }
        }
    );
    new_graph.push_edges(
        Edge {
            x: Pair { x: 4, y: 1 },
            y: Pair { x: 4, y: 2 }
        }
    );
    new_graph.push_edges(
        Edge {
            x: Pair { x: 4, y: 2 },
            y: Pair { x: 4, y: 1 }
        }
    );
    new_graph.push_edges(
        Edge {
            x: Pair { x: 4, y: 2 },
            y: Pair { x: 4, y: 3 }
        }
    );
    new_graph.push_edges(
        Edge {
            x: Pair { x: 4, y: 3 },
            y: Pair { x: 4, y: 2 }
        }
    );
    new_graph.push_edges(
        Edge {
            x: Pair { x: 4, y: 3 },
            y: Pair { x: 3, y: 3 }
        }
    );
   
    new_graph
}

