use anyhow::Result;
use clap::Parser;
use std::io::{self, BufRead, BufReader};
use std::fs::File;

#[derive(Parser, Debug)]
#[command(author, version, about)]
/// Rust version of head
struct Arguments {
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
        conflicts_with("bytes"),
        value_parser = clap::value_parser!(u64).range(1..)
    )]
    lines: u64,

    /// number of bytes to print
    #[arg(
        short('c'),
        long("bytes"),
        value_parser = clap::value_parser!(u64).range(1..)
    )]
    bytes: Option<u64>,
}

fn fileopen(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn run(args: Arguments) -> Result<()> {
    let opt_n: u64 = args.lines;
    let opt_c: u64 = args.bytes.unwrap_or_else(|| 0);

    for fname in args.files {
        match fileopen(&fname) {
            Ok(bufread) => {
                //println!("{fname}: opened! ({opt_n}, {opt_c})");
                for (linenum, read) in bufread.lines().enumerate() {
                    if linenum == opt_n.try_into().unwrap() {
                        break;
                    }
                    let line = read?;
                    println!("{line}");
                }
            },
            Err(e) => eprintln!("{fname}: {e}"),
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = run(Arguments::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
