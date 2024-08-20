use anyhow::Result;
use clap::Parser;
use std::io::{self, BufRead, BufReader, Read};
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
    for fname in args.files {
        match fileopen(&fname) {
            Err(e) => eprintln!("{fname}: {e}"),
            Ok(mut fd) => {
                if let Some(nr_bytes) = args.bytes {
                    let mut buffer = vec![0; nr_bytes as usize];
                    let bytes_read = fd.read(&mut buffer)?;
                    println!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                } else {
                    let mut line = String::new();
                    for _ in 0..args.lines {
                        let bytes = fd.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{line}");
                        line.clear();
                    }
                }

            }
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
