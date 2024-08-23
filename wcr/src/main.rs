use anyhow::Result;
use clap::{Parser};
use std::io::{self, BufRead, BufReader};
use std::fs::File;

#[derive(Parser, Debug)]
#[command(version, author, about)]
/// Rust version of `wc`
struct Args {
    /// Input files(s)
    #[arg(default_value = "-")]
    files: Vec<String>,

    /// Show line count
    //#[arg(short('l'), long("lines"), action(ArgAction::SetTrue))]
    #[arg(short('l'), long("lines"))]
    lines: bool,

    /// Show word count
    #[arg(short, long)]
    words: bool,

    /// Show byte count
    #[arg(short('c'), long)]
    bytes: bool,

    /// Show charactor count
    #[arg(short('m'), long)]
    chars: bool,
}

fn fileopen(fname: &str) -> Result<Box<dyn BufRead>> {
    match fname {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(fname)?))),
    }
}

fn run(mut args: Args) -> Result<()> {
    if [args.lines, args.words, args.bytes, args.chars].iter().all(|v| v == &false) {
        args.lines = true;
        args.words = true;
        args.bytes = true;
    }
    println!("{args:#?}");

    for fname in args.files {
        let fi: FileInfo;
        match fileopen(&fname) {
            Err(e) => eprintln!("{fname}: {e}"),
            Ok(fd) => {
                fi = get_count(fd)?;
                println!("{fname}: {fi:?}")
            },
        }
    }
    Ok(())
}

fn main() {
   if let Err(e) = run(Args::parse()) {
       eprintln!("{e}");
       std::process::exit(1);
   }
}

#[derive(Debug, PartialEq)]
struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

//fn get_count(mut fd: impl BufRead) -> Result<FileInfo> {
fn get_count<T: BufRead>(mut fd: T) -> Result<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;

    num_lines = fd.lines().count();
    num_words = fd.lines().split_whitespace().count();

    Ok(FileInfo { num_lines, num_words, num_bytes, num_chars})
}

#[cfg(test)]
mod count_tests {
    use super::{get_count, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_get_count_lines() {
        let txt = "I don't want the world.\nI just want your half.\r\n";
        let info = get_count(Cursor::new(txt));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 2,
            num_words: 10,
            num_bytes: 48,
            num_chars: 48,
        };
        assert_eq!(info.unwrap().num_lines, expected.num_lines);
    }
    #[test]
    fn test_get_count_words() {
        let txt = "I don't want the world.\nI just want your half.\r\n";
        let info = get_count(Cursor::new(txt));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 2,
            num_words: 10,
            num_bytes: 48,
            num_chars: 48,
        };
        assert_eq!(info.unwrap().num_words, expected.num_words);
    }
    #[test]
    fn test_get_count_bytes() {
        let txt = "I don't want the world.\nI just want your half.\r\n";
        let info = get_count(Cursor::new(txt));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 2,
            num_words: 10,
            num_bytes: 48,
            num_chars: 48,
        };
        assert_eq!(info.unwrap().num_bytes, expected.num_bytes);
    }
    #[test]
    fn test_get_count_chars() {
        let txt = "I don't want the world.\nI just want your half.\r\n";
        let info = get_count(Cursor::new(txt));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 2,
            num_words: 10,
            num_bytes: 48,
            num_chars: 48,
        };
        assert_eq!(info.unwrap().num_chars, expected.num_chars);
    }
    #[test]
    fn test_get_count() {
        let txt = "I don't want the world.\nI just want your half.\r\n";
        let info = get_count(Cursor::new(txt));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 2,
            num_words: 10,
            num_bytes: 48,
            num_chars: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }
}
