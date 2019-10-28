use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::time::Instant;

use crate::graph::ugraph::UndirectedGraph;
use crate::graph::vertex::{VertexId, Vertex};
use crate::graph::edge::{Edge, EdgeId};
use crate::graph::digraph::DiGraph;

mod vertex;
mod edge;

pub mod ugraph;
pub mod digraph;

pub fn create_undirected_graph(file: File) -> UndirectedGraph {
    let now = Instant::now();
    let mut graph: UndirectedGraph = UndirectedGraph::new(Vec::new(), Vec::new());
    graph.read_from_file(&file);
    println!("{}", now.elapsed().as_micros());

    graph
}

pub fn create_directed_graph(file: File) -> DiGraph {
    let now = Instant::now();
    let mut graph: DiGraph = DiGraph::new(Vec::new(), Vec::new());
    graph.read_from_file(&file);
    println!("{}", now.elapsed().as_micros());

    graph
}

pub trait Graph {
    fn read_from_file(&mut self, file: &File) -> () {
        let reader = BufReader::new(file);

        let mut first = true;
        for line in reader.lines().filter_map(io::Result::ok) {
            if first {
                let count: i32 = line.parse().unwrap();
                for _ in 0..count {
                    self.add_vertex();
                }
                first = false;
            } else {
                let vertices = line.trim().split(' ').flat_map(str::parse::<VertexId>).collect::<Vec<_>>();
                self.add_edge(vertices[0], vertices[1]);
            }
        }
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

    fn get_free_vertex_id(&mut self) -> VertexId {
        self.get_vertices().len() as VertexId
    }

    fn get_free_edge_id(&mut self) -> EdgeId {
        self.get_edges().len() as EdgeId
    }

    fn get_vertices(&self) -> &Vec<Vertex>;
    fn get_edges(&self) -> &Vec<Edge>;

    fn get_vertices_as_mut(&mut self) -> &mut Vec<Vertex>;
    fn get_edges_as_mut(&mut self) -> &mut Vec<Edge>;


    fn get_neighbours(&self, vertex: &Vertex) -> Vec<&Vertex> {
        let neighbours: Vec<&Vertex> = vertex.edges()
            .iter()
            .map(|&x| self.get_edges()[x].get_other_vertex(*vertex.id()))
            .map(|&x| &self.get_vertices()[x])
            .collect();
        neighbours
    }

    fn find_strongly_connected_components(&self) -> ();
}

