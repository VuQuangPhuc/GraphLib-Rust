use std::{env, io};

mod graph;

fn main() -> io::Result<()> {
    let mut args: Vec<String> = env::args().skip(1).collect();
    let operation = args[0].clone();
    args.drain(0..1);
    stack::huge_g(operation, args);

    Ok(())
}

mod stack {
    use std::thread;
    use crate::graph::Graph;
    use std::fs::File;
    use std::time::Instant;
    use crate::graph::random::{create_random_directed_graph, create_random_graph};

    pub fn huge_g(mode: String, args: Vec<String>) -> () {
        let builder = thread::Builder::new()
            .name("STACKSSSS".into())
            .stack_size(2048 * 1024 * 1024);

        let handler = builder.spawn(move || {
            if &mode == "directed" || &mode == "undirected" {
                for file_name in args {
                    let now = Instant::now();
                    let file = File::open(file_name).unwrap();
                    let graph = Graph::read_from_file(&mode == "directed", file);
                    let components = graph.search_for_components();
                    println!("Time elapsed: {}.", now.elapsed().as_micros());
                    println!("Found {} components.", components.len());
//                println!("{:?}", components);
                }
            } else if &mode == "random" {
                println!("Creating graph:");
                let vertices_number: i32 = args[0].parse().unwrap();
                let directed: bool = args[1].parse().unwrap();
                let file_name = &args[2];
                println!("Option: n = {}, directed = {}", vertices_number, directed);
                let mut graph = Graph::new(directed);
                let probability = (vertices_number as f64).log10() / vertices_number as f64;
                println!("{}", probability);
                if directed {
                    create_random_directed_graph(&mut graph, vertices_number, probability);
                } else {
                    create_random_graph(&mut graph, vertices_number, probability);
                }
                graph.write_to_file(file_name);
//                println!("{:?}", graph);
            }
        }).unwrap();

        handler.join().unwrap();
    }
}
