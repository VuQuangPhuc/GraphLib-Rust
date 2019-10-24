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

    pub fn find_strong_connections(&self) -> Vec<Vec<VertexId>> {
        let mut index = 0;
        let mut visited: Vec<VertexId> = Vec::new();

        let mut connections: Vec<Vec<VertexId>> = Vec::new();

        for vertex in &self.vertices {
            if !visited.contains(&vertex.id()) {
                connections.push(self.visit_forward(&mut visited, vertex));
            }
        };

        let strong_connections: Vec<Vec<VertexId>> = Vec::new();
        strong_connections
    }

    fn visit_forward(&self, visited: &mut Vec<VertexId>, vertex: &Vertex) -> Vec<VertexId> {
        visited.push(*vertex.id());
        let neighbors = self.get_neighbours(vertex);
        let mut connected: Vec<VertexId> = Vec::new();
        connected.push(vertex.id().clone());

        for neighbor in neighbors {
            if !visited.contains(&neighbor.id()) {
                let mut rest = self.visit_forward(visited, neighbor);
                connected.append(& mut rest)
            }
        };

        connected
    }

    fn get_neighbours(&self, vertex: &Vertex) -> Vec<&Vertex> {
        let neighbours: Vec<&Vertex> = vertex.edges()
            .iter()
            .map(|&x| self.edges[x].get_other_vertex(*vertex.id()))
            .map(|&x| &self.vertices[x])
            .collect();
        neighbours
    }
}
