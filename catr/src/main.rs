/*
use clap::{Arg, ArgAction, Command};

#[derive(Debug)]
struct Args {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

fn get_args() -> Args {
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("Ji-Hun Kim")
        .about("Rust version of cat")
        .arg(
            Arg::new("files")
                .value_name("FILES")
                .help("file list")
                .num_args(1..)
                .default_value("-"),
            )
        .arg(
            Arg::new("number")
                .short('n')
                .long("number")
                .action(ArgAction::SetTrue)
                .help("line number including empty lines")
                .conflicts_with("number_nonblank")
            )
        .arg(
            Arg::new("number_nonblank")
                .short('b')
                .long("number-nonblank")
                .action(ArgAction::SetTrue)
                .help("line number no blank")
            )
        .get_matches();

    Args {
        files: matches.get_many("files").unwrap().cloned().collect(),
        number_lines: matches.get_flag("number"),
        number_nonblank_lines: matches.get_flag("number_nonblank"),
    }
}
*/
use clap::Parser;
use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Parser, Debug)]
#[command(author, version, about)]
/// Rust version of cat
struct Args {
    /// file list
    #[arg(value_name = "FILES", default_value = "-")]
    files: Vec<String>,
    
    /// line number including empty lines
    #[arg(
        short('n'),
        long("number"),
        conflicts_with("number_nonblank_lines")
    )]
    number_lines: bool,

    /// line number except blank line 
    #[arg(
        short('b'),
        long("number-nonblank")
    )]
    number_nonblank_lines: bool,
}

/*
 * file::open() 은 std::fs::File 타입을 리턴, 파일 타입과 std::io::stdin 타입은 모두 Read Trait를
 * 구현한다. 
 * Read 트레이트는 BufReader 구조체는 BufRead 트레이트를 구현함.
 * BufRead는 Read를 구현함.(?) 
 *
 * https://youblog.tistory.com/147#user-content-%EB%B2%84%ED%8D%BC%EB%A7%81%EA%B3%BC-%EC%8A%A4%ED%8A%B8%EB%A6%BC
 * BufReader / BufWriter는 성능향상을 위한 버퍼링 기능이다. 아래와 같이 사용 가능!
 * 
 * use std::fs::File;
 * use std::io::{BufReader, BufWriter};
 *
 * let file = File::open("example.txt").expect("fail to open!")
 * let reader = BufReader::new(file);
 *
 * for read in reader.lines() {
 *      println!("{}", read.expect("fail to read"));
 * //                  ^^^^ Result<> 타입
 * }
 */
fn fileopen(fname: &str) -> Result<Box<dyn BufRead>> {
    //                         ^^^^^^^^^^^^^^^^ Trait Object 
    //                         BufRead 트레이트를 구현하는 모든 타입을 리턴할수 있다.
    match fname {
        // cat에서 "-" 옵션 의미:
        // If file is a single dash (‘-’) or absent, cat reads from the stdin.
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),       // from stdin
        _ => Ok(Box::new(BufReader::new(File::open(fname)?))),  // from the file
    }
}

/*
 * 에러발생 케이스:
 * Permission denied (os error 13)
 * No such file or directory (os error 2)
 */
fn run(_args: Args) -> Result<()> {
    let opt_n: bool = _args.number_lines;
    let opt_b: bool = _args.number_nonblank_lines;
    for fname in _args.files {
        match fileopen(&fname) {
            Err(e) => eprintln!("{fname}: fail to open: {e}"),
            Ok(buf_reader) => {
                let mut lnum: i32 = 0;
                for read in buf_reader.lines() {
                    let txt = read?;
                //            ^^^^ Result<std::string::String, std::io::Error> 타입.
                //            unwrap필요, ?사용
                    let mut prefix = String::from("");
                    if opt_n || (opt_b && !txt.is_empty()) {
                        lnum = lnum + 1;
                        prefix = format!("{lnum:>6}\t");
                    }
                    println!("{}{}", prefix, txt);
                }
            }
        }
    }
    Ok(())
}

fn main() {
    //let args = get_args();
    //let args = Args::parse();
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
