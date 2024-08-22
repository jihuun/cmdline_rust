use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, author, about)]
/// Rust version of `wc`
struct Args {
    /// Input files(s)
    #[arg(default_value = "-")]
    files: Vec<String>,
    /// Show line count
    #[arg(short('l'), long("lines"))]
    lines: bool,
    /// Show word count
    #[arg(short, long)]
    words: bool,
    /// Show byte count
    #[arg(short('c'), long)]
    bytes: bool,
    /// Show charactor count
    #[arg(short('m'), long, conflicts_with("bytes"))]
    chars: bool,
}

fn run(args: Args) -> Result<()> {
    Ok(())
}

fn main() {
   if let Err(e) = run(Args::parse()) {
       eprintln!("{e}");
       std::process::exit(1);
   }
}
