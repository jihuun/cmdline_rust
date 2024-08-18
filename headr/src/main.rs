use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
/// Rust version of head
struct Args {
    /// file list
    #[arg(
        value_name = "FILES",
        default_value = "-"
    )]
    files: Vec<String>,

    /// number of lines to print
    #[arg(
        short('n'),
        long("lines"),
        default_value = "10",
        conflicts_with("bytes")
    )]
    lines: u64,

    /// number of bytes to print
    #[arg(
        short('c'),
        long("bytes")
    )]
    bytes: Option<u64>,
}

fn main() {
    let args = Args::parse();
    println!("{:#?}", args);
}
