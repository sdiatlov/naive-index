use std::env;
use std::process::exit;
use std::time::SystemTime;

mod naive_index;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("Usage: naive-index <data file path> <word1> <word2>");
        exit(1);
    }
    println!("naive-index: load and search");
    let t_start = SystemTime::now();
    let index = naive_index::SearchIndex::new(args[1].as_str());
    let t_loaded = SystemTime::now();
    let found = index.search(args[2].as_str(), args[3].as_str());
    let t_found = SystemTime::now();
    println!("Documents found: {:?}", found);
    println!("Number of documents found: {}", found.len());
    println!("Data loaded in {} ms", t_loaded.duration_since(t_start).unwrap().as_millis());
    println!("Search in {} micros", t_found.duration_since(t_loaded).unwrap().as_micros());
}
