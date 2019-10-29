use crate::graph::vertex::{Vertex, VertexId};
use crate::graph::edge::{Edge, EdgeId};
use std::fs::File;
use std::io::{BufReader, BufRead, Write, LineWriter};
use std::io;

mod vertex;
mod edge;

pub mod components;
pub mod random;

macro_rules! vertex {
    ($id:expr) => {
        Vertex::new($id, Vec::new())
    };
}

macro_rules! edge {
    ($id:expr, $start:expr, $end:expr) => {
        Edge::new($id, $start, $end)
    };
}

#[derive(Debug)]
pub struct Graph {
    directed: bool,
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
}

impl Graph {
    pub fn new(directed: bool) -> Graph {
        Graph {
            directed,
            vertices: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn read_from_file(directed: bool, file: File) -> Graph {
        let mut graph = Graph::new(directed);
        let reader = BufReader::new(file);

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
        };

        graph
    }

    pub fn search_for_components(&self) -> Vec<Vec<&VertexId>> {
        if self.directed {
            components::find_strongly_connected_components(&self)
        } else {
            components::find_connected_components(&self)
        }
    }

    fn add_vertex(&mut self) -> VertexId {
        let id = self.get_free_vertex_id();
        let vertex = vertex!(id);
        self.vertices.push(vertex);

        id
    }

    fn get_free_vertex_id(&self) -> VertexId {
        self.vertices.len()
    }

    fn add_edge(&mut self, start: VertexId, end: VertexId) -> EdgeId {
        let id = self.get_free_edge_id();
        let edge = edge!(id, start, end);
        self.vertices[start].add_edge(id);
        self.vertices[end].add_edge(id);
        self.edges.push(edge);

        id
    }

    fn get_free_edge_id(&self) -> EdgeId {
        self.edges.len()
    }

    fn get_neighbours(&self, vertex: &Vertex) -> Vec<&Vertex> {
        vertex.edges()
            .iter()
            .map(|&x| {
                &self.vertices[*self.edges[x].get_other_vertex(vertex.id())]
            })
            .collect()
    }

    fn reachable_vertices(&self, vertex: &Vertex) -> Vec<&Vertex> {
        vertex.edges()
            .iter()
            .filter(|&&x| {
                vertex.id() == self.edges[x].start()
            })
            .map(|&x| {
                &self.vertices[*self.edges[x].end()]
            })
            .collect()
    }

    fn accessible_from(&self, vertex: &Vertex) -> Vec<&Vertex> {
        vertex.edges()
            .iter()
            .filter(|&&x| {
                vertex.id() == self.edges[x].end()
            })
            .map(|&x| {
                &self.vertices[*self.edges[x].start()]
            })
            .collect()
    }

    pub fn write_to_file(&self, filename: &str) -> () {
        let file = File::create(filename).unwrap();
        let mut file = LineWriter::new(file);
        file.write_all(format!("{}\n", self.vertices.len()).as_bytes()).expect("Could not write vertex number to file.");
        for edge in &self.edges {
            file.write_all(format!("{}\n", edge).as_bytes()).expect("Could not write edge to file.");
        }
        file.flush().expect("Could not write file.");
    }
}
