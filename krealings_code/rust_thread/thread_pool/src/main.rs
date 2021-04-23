use threadpool::ThreadPool;
use std::env::args;
use std::process::exit;
use std::thread;


fn main() {

    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run <numtasks>");
        exit(1);
    }
    let pool = ThreadPool::new(4 as usize);
    for i in 0 .. args[1].parse::<i32>().unwrap(){
        println!("Executing thread {}", i);
        pool.execute(|| {
            println!("I ({:?}) am working on a task: ", thread::current().id());
        });
    }

    // main will wait forever (well not guaranteed forever)
    thread::park();
}


