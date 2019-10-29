use crate::graph::Graph;
use crate::graph::vertex::VertexId;
use std::fs::File;
use std::io::{LineWriter, Write};

pub fn find_connected_components(graph: &Graph) -> Vec<Vec<&VertexId>> {
    undirected_graph::find_connected_component(graph)
}

pub fn find_strongly_connected_components(graph: &Graph) -> Vec<Vec<&VertexId>> {
    directed_graph::find_strongly_connected_components(graph)
}

pub fn write_components_to_file(components: Vec<Vec<&VertexId>>) -> () {
    let file = File::create("output").expect("Could not create file when writing components.");
    let mut file = LineWriter::new(file);
    for component in components {
        let vertices: Vec<String> = component
            .iter()
            .map(|&x| format!("{}", *x as i32))
            .collect();
        let line: String = vertices.join(" ") + "\n";
        file.write_all(line.as_bytes()).expect("Could not write file.");
    }
    file.flush().expect("Could not write component file.");
}

mod undirected_graph {
    use crate::graph::Graph;
    use std::collections::HashSet;
    use crate::graph::vertex::{VertexId, Vertex};

    pub fn find_connected_component(graph: &Graph) -> Vec<Vec<&VertexId>> {
        let mut visited: HashSet<&VertexId> = HashSet::new();
        let mut components: Vec<Vec<&VertexId>> = Vec::new();

        for vertex in &graph.vertices {
            if !visited.contains(vertex.id()) {
                let mut connected: Vec<&VertexId> = vec![vertex.id()];
                connected.append(&mut visit_forward(graph, &mut visited, vertex));
                components.push(connected);
            }
        };

        components
    }

    fn visit_forward<'a>(graph: &'a Graph, visited: &mut HashSet<&'a VertexId>, vertex: &'a Vertex) -> Vec<&'a VertexId> {
        visited.insert(vertex.id());
        let mut component: Vec<&VertexId> = Vec::new();
        let neighbours = graph.get_neighbours(vertex);
        for neighbour in neighbours {
            if !visited.contains(neighbour.id()) {
                component.push(neighbour.id());
                component.append(&mut visit_forward(graph, visited, neighbour));
            }
        }

        component
    }
}

mod directed_graph {
    use crate::graph::Graph;
    use crate::graph::vertex::{VertexId, Vertex};
    use std::collections::HashSet;

    pub fn find_strongly_connected_components(graph: &Graph) -> Vec<Vec<&VertexId>> {
        let mut visited: HashSet<&VertexId> = HashSet::new();
        let mut components: Vec<Vec<&VertexId>> = Vec::new();
        let mut vertex_stack: Vec<&VertexId> = Vec::new();

        for vertex in &graph.vertices {
            if !visited.contains(vertex.id()) {
                visit_forward(graph, &mut visited, &mut vertex_stack, vertex)
            }
        };

        visited.clear();

        while vertex_stack.len() > 0 {
            let vertex = &graph.vertices[*vertex_stack.pop().unwrap()];
            if visited.contains(vertex.id()) {
                continue;
            }
            let mut component = vec![vertex.id()];
            component.append(&mut visit_backwards(graph, &mut visited, vertex));
            components.push(component);
        }

        components
    }

    fn visit_forward<'a>(graph: &'a Graph, visited: &mut HashSet<&'a VertexId>, vertex_stack: &mut Vec<&'a VertexId>, vertex: &'a Vertex) -> () {
        visited.insert(vertex.id());
        let neighbours = graph.reachable_vertices(vertex);
        for neighbour in neighbours {
            if !visited.contains(neighbour.id()) {
                visit_forward(graph, visited, vertex_stack, neighbour);
            }
        }
        vertex_stack.push(vertex.id())
    }

    fn visit_backwards<'a>(graph: &'a Graph, visited: &mut HashSet<&'a VertexId>, vertex: &'a Vertex) -> Vec<&'a VertexId> {
        visited.insert(vertex.id());
        let mut component: Vec<&VertexId> = Vec::new();
        let accessible_from = graph.accessible_from(vertex);
        for af in accessible_from {
            if !visited.contains(af.id()) {
                component.push(af.id());
                component.append(&mut visit_backwards(graph, visited, af))
            }
        };
        component
    }
}