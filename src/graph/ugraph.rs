use std::collections::HashSet;
use std::time::Instant;

use crate::graph::edge::Edge;
use crate::graph::Graph;
use crate::graph::vertex::{Vertex, VertexId};

#[derive(Debug)]
pub struct UndirectedGraph {
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
}

impl UndirectedGraph {
    pub(crate) fn new(vertices: Vec<Vertex>, edges: Vec<Edge>) -> UndirectedGraph {
        UndirectedGraph {
            vertices,
            edges,
        }
    }

    fn visit_forward(&self, visited: &mut HashSet<VertexId>, vertex: &Vertex, connected: &mut Vec<VertexId>) -> () {
        visited.insert(*vertex.id());
        connected.push(*vertex.id());

        let neighbours: Vec<&Vertex> = self.get_neighbours(vertex);
        for neighbour in neighbours {
            if !visited.contains(neighbour.id()) {
                self.visit_forward(visited, neighbour, connected)
            }
        };
    }
}

impl Graph for UndirectedGraph {
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

    fn find_strongly_connected_components(&self) -> () {
        let mut visited: HashSet<VertexId> = HashSet::new();
        let mut components: Vec<Vec<VertexId>> = Vec::new();
        let now = Instant::now();

        for vertex in &self.vertices {
            if !visited.contains(&vertex.id()) {
                let mut connected: Vec<VertexId> = Vec::new();
                self.visit_forward(&mut visited, vertex, &mut connected);
                components.push(connected)
            }
        };

        println!("{}", now.elapsed().as_micros() / 1_000_000);
        println!("Found {} connections.", components.len());
    }
}
