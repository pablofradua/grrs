use structopt::StructOpt;

fn main() {
    let args = Cli::from_args();
    dbg!(args.pattern);
    dbg!(args.path);
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
