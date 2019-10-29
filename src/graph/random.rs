use crate::graph::Graph;

pub fn create_random_graph(graph: &mut Graph, vertices_number: i32, edge_probability: f64) -> () {
    graph_generator::create_random_graph(graph, vertices_number, edge_probability, false);
}

pub fn create_random_directed_graph(graph: &mut Graph, vertices_number: i32, edge_probability: f64) {
    graph_generator::create_random_graph(graph, vertices_number, edge_probability, true);
}

mod graph_generator {
    extern crate rand;

    use crate::graph::Graph;
    use rand::Rng;
    use crate::graph::vertex::VertexId;

    pub fn create_random_graph(graph: &mut Graph, vertices_number: i32, edge_probability: f64, directed: bool) -> () {
        let mut rng = rand::thread_rng();
        for _ in 0..vertices_number {
            graph.add_vertex();
        }

        for i in 0..vertices_number {
            let mut j = 0;
            while j < i || directed && j < vertices_number {
                let random = rng.gen_range(0, 101) as f64 / 100.0;
                if edge_probability >= random {
                    graph.add_edge(j as VertexId, i as VertexId);
                }
                j = j + 1;
            }
        }
    }
}