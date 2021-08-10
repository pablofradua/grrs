use std::fs;
use std::io::{BufRead, BufReader};
use structopt::StructOpt;

fn main() {
    let args = Cli::from_args();
    let file = fs::File::open(&args.path).expect("Could not open file");
    let reader = BufReader::new(file);
    for (index, line) in reader.lines().enumerate() {
        let line = line.expect("Error reading line");
        if line.contains(&args.pattern) {
            println!("({}): {}", index, line);
        }
    }
}

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}
