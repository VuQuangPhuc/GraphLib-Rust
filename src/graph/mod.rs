#![allow(dead_code)]

use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use crate::graph::vertex::{VertexId, Vertex};
use crate::graph::edge::{EdgeId, Edge};

mod vertex;
mod edge;

pub fn create_graph(file: File) -> Graph {
    let now = Instant::now();
    let graph = Graph::read_from_file(&file);
    println!("{}", now.elapsed().as_micros());

    graph
}

#[derive(Debug)]
pub struct Graph {
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
}

impl Graph {
    fn read_from_file(file: &File) -> Self {
        let reader = BufReader::new(file);

        let mut graph: Graph = Graph {
            vertices: Vec::new(),
            edges: Vec::new(),
        };

        let mut first = true;
        for line in reader.lines().filter_map(io::Result::ok) {
            if first {
                let count: i32 = line.parse().unwrap();
                for _ in 0..count {
                    graph.add_vertex();
                }
                first = false;
            } else {
                let vertices = line.trim().split(' ').flat_map(str::parse::<VertexId>).collect::<Vec<_>>();
                graph.add_edge(vertices[0], vertices[1]);
            }
        }

        graph
    }

    fn add_vertex(&mut self) -> VertexId {
        let id = self.get_free_vertex_id();
        let vertex = Vertex::new(id, Vec::new());
        self.vertices.push(vertex);

        id
    }

    fn add_edge(&mut self, vertex_a: VertexId, vertex_b: VertexId) -> EdgeId {
        let id = self.get_free_edge_id();
        let edge = Edge::new(id, vertex_a, vertex_b);
        self.vertices[vertex_a].add_edge(id);
        self.vertices[vertex_b].add_edge(id);
        self.edges.push(edge);

        id
    }

    fn get_free_vertex_id(&self) -> VertexId {
        self.vertices.len() as VertexId
    }

    fn get_free_edge_id(&self) -> EdgeId {
        self.edges.len() as EdgeId
    }
}
