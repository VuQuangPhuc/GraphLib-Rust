#![recursion_limit = "600000"]

use std::{env, io, thread};
use std::alloc::handle_alloc_error;
use std::fs::File;

mod graph;

fn main() -> io::Result<()> {
    let mut args: Vec<String> = env::args().skip(1).collect();
    let operation = args[0].clone();
    args.drain(0..1);

    println!("{}", operation);

    stack::huge_g(args);

    Ok(())
}

mod stack {
    use std::fs::File;
    use std::{thread, io};

    use crate::graph;

    pub fn huge_g(args: Vec<String>) -> () {
        let builder = thread::Builder::new()
            .name("STACKSSSS".into())
            .stack_size(2048 * 1024 * 1024);

        let handler = builder.spawn(|| {
            for f in args {
                println!("---------------------------------------");
                let file = File::open(f).unwrap();
                let graph = graph::create_graph(file);
                graph.find_strong_connections();
            }
        }).unwrap();

        handler.join().unwrap();
    }
}
