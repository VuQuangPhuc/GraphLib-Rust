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

    pub fn huge_g(mode: String, args: Vec<String>) -> () {
        let builder = thread::Builder::new()
            .name("STACKSSSS".into())
            .stack_size(2048 * 1024 * 1024);

        let handler = builder.spawn(move || {
            for file_name in args {
                let now = Instant::now();
                let file = File::open(file_name).unwrap();
                let graph = Graph::read_from_file(&mode == "directed", file);
                let components = graph.search_for_components();
                println!("Time elapsed: {}.", now.elapsed().as_micros());
                println!("Found {} components.", components.len());
//                println!("{:?}", components);
            }
        }).unwrap();

        handler.join().unwrap();
    }
}
