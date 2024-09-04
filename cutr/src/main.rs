use anyhow::{Result,bail};
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

fn run(args: Args) -> Result<()> {
    //println!("{args:?}");
    let delim_byte = args.delimiter.as_bytes();
    if delim_byte.len() > 1 {
        // bail! -> https://docs.rs/clap-utils/latest/clap_utils/macro.bail.html
        bail!(r#"--delim "{}" must be a single byte"#, args.delimiter);
    }
    //                  |           _______ Option<&u8>
    let delimiter: u8 = *delim_byte.first().unwrap();
    //                   ^^^^^^^^^^ &[u8]
    println!("{delimiter}");
    Ok(())
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
