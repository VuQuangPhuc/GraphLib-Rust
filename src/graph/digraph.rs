use crate::graph::vertex::{Vertex, VertexId};
use crate::graph::edge::Edge;
use crate::graph::Graph;
use std::collections::HashSet;

pub struct DiGraph {
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
    strong_components: Vec<Vec<Vertex>>,
}

impl DiGraph {
    pub fn new(vertices: Vec<Vertex>, edges: Vec<Edge>) -> DiGraph {
        DiGraph {
            vertices,
            edges,
            strong_components: Vec::new(),
        }
    }

    fn search_forward(&self) -> Vec<Vertex> {
        let mut visited: HashSet<VertexId> = HashSet::new();
        let mut vertex_stack: Vec<VertexId> = Vec::new();
        for vertex in &self.vertices {
            if !visited.contains(&vertex.id()) {
                let connected: Vec<VertexId> = self.visit_forward(&mut visited, &mut vertex_stack, vertex);
            }
        };

        println!("{:?}", vertex_stack);

        vertex_stack.iter().map(|&x| self.vertices[x].clone()).collect()
    }

    fn visit_forward(&self, visited: &mut HashSet<VertexId>, vertex_stack: &mut Vec<VertexId>, vertex: &Vertex) -> Vec<VertexId> {
        println!("Visit {}", vertex.id());
        visited.insert(*vertex.id());
        let mut connected = vec![*vertex.id()];
        let neighbours: Vec<&Vertex> = self.get_neighbours(vertex);
        for neighbour in neighbours {
            if !visited.contains(neighbour.id()) {
                connected.append(&mut self.visit_forward(visited, vertex_stack, neighbour))
            }
        };
        vertex_stack.push(*vertex.id());
        println!("Adding thing {} to stack", vertex.id());
        connected
    }
}

impl Graph for DiGraph {
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

        self.search_forward();
    }
}