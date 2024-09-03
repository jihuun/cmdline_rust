use clap::{Parser};

#[derive(Parser, Debug)]
#[command(author, about, version)]
/// Rust version of cut
struct Args {
    /// input files
    #[arg(default_value("-"))]
    files: Vec<String>,

    /// seperater
    #[arg(short, long, default_value("\t"))]
    delimiter: String,

    #[command(flatten)] // flatten 은 ArgsExtract를 Args에 병합
    extract: ArgsExtract,
}

#[derive(Debug, clap::Args)]
#[group(required = true, multiple = false)] // group은 clap::ArgGroup 생성
struct ArgsExtract {
    /// seleted field
    #[arg(short, long)]
    fields: Option<String>,

    /// seleted bytes
    #[arg(short, long)]
    bytes: Option<String>,

    /// seleted charactors
    #[arg(short, long)]
    chars: Option<String>,
}

fn main() {
    let args = Args::parse();
    println!("{args:?}");
}
