use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io;
use std::time::Instant;

use crate::graph::edge::Edge;
use crate::graph::Graph;
use crate::graph::private_graph::PrivateGraph;
use crate::graph::vertex::{Vertex, VertexId};

#[derive(Debug)]
pub struct UndirectedGraph {
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
}

impl UndirectedGraph {
    fn visit_forward(&self, visited: &mut HashSet<VertexId>, vertex: &Vertex, connected: &mut Vec<VertexId>) -> () {
        visited.insert(*vertex.id());
        let neighbors: Vec<&Vertex> = self.get_neighbours(vertex);
        connected.push(*vertex.id());

        for neighbor in neighbors {
            if !visited.contains(neighbor.id()) {
                self.visit_forward(visited, neighbor, connected)
            }
        };
    }
}

impl Graph for UndirectedGraph {
    fn read_from_file(file: &File) -> UndirectedGraph {
        let reader = BufReader::new(file);

        let mut graph: UndirectedGraph = UndirectedGraph {
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

    fn find_strong_connected_components(&self) -> () {
        let index = 0;
        let mut visited: HashSet<VertexId> = HashSet::new();

        let mut connections: Vec<Vec<VertexId>> = Vec::new();

        let now = Instant::now();

        for vertex in &self.vertices {
            if !visited.contains(&vertex.id()) {
                let mut connected: Vec<VertexId> = Vec::new();
                self.visit_forward(&mut visited, vertex, &mut connected);
                connections.push(connected)
            }
        };

        println!("{}", now.elapsed().as_micros());
        println!("Found {} connections.", connections.len());
    }
}

impl PrivateGraph for UndirectedGraph {
    fn get_vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }

    fn get_edges(&self) -> &Vec<Edge> {
        &self.edges
    }

    fn get_vertices_as_mut(&mut self) -> &mut Vec<Vertex> {
        &mut self.vertices
    }
    fn get_edges_as_mut(&mut self) -> &mut Vec<Edge> {
        &mut self.edges
    }
}