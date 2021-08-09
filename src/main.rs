fn main() {
}
#[derive(StructOpt)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}
