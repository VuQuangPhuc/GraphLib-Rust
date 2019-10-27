use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::time::Instant;

use crate::graph::edge::Edge;
use crate::graph::private_graph::PrivateGraph;
use crate::graph::ugraph::UndirectedGraph;
use crate::graph::vertex::{Vertex, VertexId};

mod vertex;
mod edge;

pub mod ugraph;

pub fn create_undirected_graph(file: File) -> UndirectedGraph {
    let now = Instant::now();
    let graph: UndirectedGraph = Graph::new(&file);
    println!("{}", now.elapsed().as_micros());

    graph
}

pub trait Graph {
    fn new(file: &File) -> Self;
    fn find_strong_connected_components(&self) -> ();
}

mod private_graph {
    use crate::graph::edge::{Edge, EdgeId};
    use crate::graph::vertex::{Vertex, VertexId};

    pub trait PrivateGraph {
        fn get_vertices(&self) -> &Vec<Vertex>;
        fn get_edges(&self) -> &Vec<Edge>;

        fn get_vertices_as_mut(&mut self) -> &mut Vec<Vertex>;
        fn get_edges_as_mut(&mut self) -> &mut Vec<Edge>;

        fn get_free_vertex_id(&mut self) -> VertexId {
            self.get_vertices().len() as VertexId
        }

        fn get_free_edge_id(&mut self) -> EdgeId {
            self.get_edges().len() as EdgeId
        }

        fn get_neighbours(&self, vertex: &Vertex) -> Vec<&Vertex> {
            let neighbours: Vec<&Vertex> = vertex.edges()
                .iter()
                .map(|&x| self.get_edges()[x].get_other_vertex(*vertex.id()))
                .map(|&x| &self.get_vertices()[x])
                .collect();
            neighbours
        }

        fn add_vertex(&mut self) -> VertexId {
            let id = self.get_free_vertex_id();
            let vertex = Vertex::new(id, Vec::new());
            self.get_vertices_as_mut().push(vertex);

            id
        }

        fn add_edge(&mut self, vertex_a: VertexId, vertex_b: VertexId) -> EdgeId {
            let id = self.get_free_edge_id();
            let edge = Edge::new(id, vertex_a, vertex_b);
            self.get_vertices_as_mut()[vertex_a].add_edge(id);
            self.get_vertices_as_mut()[vertex_b].add_edge(id);
            self.get_edges_as_mut().push(edge);

            id
        }
    }
}

