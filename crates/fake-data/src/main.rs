use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::exit;
use serde::Serialize;
use lipsum::lipsum;
use rand::Rng;

#[derive(Serialize, Debug)]
struct SourceText {
    pub id: String,
    pub text: String,
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: fake-data <data file path> <number of documents>");
        exit(1);
    }
    let mut data_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&args[1])
        .expect("failed to open data file");

    println!("fake-data: generating to file {}...", args[1]);
    let mut i = 0;
    let max = args[2].parse::<usize>().unwrap();
    while i < max {
        let mut rng = rand::thread_rng();
        let nw = rng.gen_range(200..4000);
        let text = lipsum(nw)
            .split_ascii_whitespace()
            .filter(|_| rand::random())
            .collect::<Vec<&str>>()
            .join(" ");
        let src = SourceText {
            id: format!("doc_id_{}", i),
            text,
        };
        let mut str = serde_json::to_string(&src).expect("Serialization failed");
        str.push('\n');
        data_file.write(str.as_bytes()).expect("Failed to write to data file");
        i += 1;
    }
}
