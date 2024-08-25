use anyhow::{anyhow, Result};
use clap::Parser;
use std::io::{self, BufRead, BufReader};
use std::fs::File;

#[derive(Parser, Debug)]
#[command(version, author, about)]
/// Rust version of uniq
struct Args {
    /// Input file
    #[arg(default_value = "-")]
    in_file: String,

    /// Output file
    #[arg()]
    out_file: Option<String>,
    // 인자로 파일명 두개를 전달하면 두번재는 out_file이 됨.
    // 만약 인자로 파일을 하나만 전달하면 out_file 인자 값은 None

    /// Show count
    #[arg(short('c'), long("count"))]
    count: bool,
}

fn file_open(fname: &str) -> Result<Box<dyn BufRead>> {
    match fname {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))) ,
        _ => Ok(Box::new(BufReader::new(File::open(fname)?))) ,
    }

}

fn format_field(val: u32, opt: bool) -> String {
    if opt {
        format!("{:>4} ", val)
    } else {
        "".to_string()
    }
}

fn run(args: Args) -> Result<()> {
    //println!("{:?}", args);
    let mut fd = file_open(&args.in_file)
        .map_err(|e| anyhow!("{}: {e}", args.in_file))?;
    let mut buf = String::new();
    let mut prev = String::new();
    let mut count: u32 = 0;
    let print_cnt = |count: u32, text: &str| {
        if count > 0 {
            print!("{}{text}", format_field(count, args.count));
        }
    };

    loop {
        let bytes = fd.read_line(&mut buf)?;
        if bytes == 0 {
            break;
        }
        if buf != prev {
            print_cnt(count, &prev);
            prev = buf.clone();
            count = 0;
        }
        count += 1;
        buf.clear();
    }
    print_cnt(count, &prev);
    Ok(())
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
