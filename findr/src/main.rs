use clap::{Parser, builder::PossibleValue, ValueEnum, ArgAction};
use regex::Regex;
use anyhow::Result;
use walkdir::WalkDir;

#[derive(Debug, Eq, PartialEq, Clone)]
enum EntryType {
    Dir,
    File,
    Link,
}

// implement ValueEnum trait
// https://docs.rs/clap/latest/clap/trait.ValueEnum.html
impl ValueEnum for EntryType {
    fn value_variants<'a>() -> &'a [Self] {
        &[EntryType::Dir, EntryType::File, EntryType::Link]
    }
    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            EntryType::Dir => PossibleValue::new("d"),
            EntryType::File  => PossibleValue::new("f"),
            EntryType::Link => PossibleValue::new("l"),
        })
    }
}

#[derive(Parser,Debug)]
#[command(about, version, author)]
/// Rust version of find
struct Args {
    /// Search paths
    #[arg(
        name = "PATH",
        default_value = ".",
        num_args(0..),
    )]
    paths: Vec<String>,

    /// Name
    #[arg(
        short,
        long("name"),
        value_parser(Regex::new),
        // action(ArgAction::Append),
        num_args(0..),
    )]
    names: Vec<Regex>,
    // 한 옵션에 여러개의 인자를 받고 싶을때 -> #[arg(num_args(0..))]
    // https://docs.rs/clap/latest/clap/builder/struct.Arg.html#method.num_args

    /// Entry Type
    #[arg(
        short('t'),
        long("type"),
        name = "TYPE",
        value_parser(clap::value_parser!(EntryType)),
        num_args(0..),
    )]
    entry_types: Vec<EntryType>,
}

fn is_type_matched(cur_entry: &walkdir::DirEntry, opt_type: &Vec<EntryType>) -> bool {
    // DirEntry.file_type() -> std::fs::FileType
    // https://doc.rust-lang.org/nightly/std/fs/struct.FileType.html
    for t in opt_type {
        match t {
            EntryType::Dir => {
                if cur_entry.file_type().is_dir() { return true; }
            },
            EntryType::File => {
                if cur_entry.file_type().is_file() { return true; }
            },
            EntryType::Link => {
                if cur_entry.file_type().is_symlink() { return true; }
            },
        }
    }
    false
}

fn run(args: Args) -> Result<()> {
    let opt_names = args.names;
    let opt_types = args.entry_types;

    let type_filter = |entry: &walkdir::DirEntry| {
        opt_types.is_empty()
            || opt_types.iter().any(|entry_type| match entry_type {
                EntryType::Dir => entry.file_type().is_dir(),
                EntryType::File => entry.file_type().is_file(),
                EntryType::Link => entry.file_type().is_symlink(),
            })
    };

    let name_filter = |entry: &walkdir::DirEntry| {
        opt_names.is_empty()
            || opt_names.iter().any(|re| {
                re.is_match(&entry.file_name().to_string_lossy(),)
            })
    };

    for p in args.paths {
        let entries = WalkDir::new(p)
            .into_iter()
            //.filter_map(|e| e.ok()) // -> Ok() 인것만 필터링해서 iterater 반환
            .filter_map(|e| match e { // Result<> 타입을 Option<> 으로 변환필요
                Err(e) => {
                    eprintln!("{e}");
                    None
                }
                Ok(e) => Some(e)
            })
            .filter(type_filter) // filter 에 전달하는 closure는 bool 타입 리턴해야함.
            .filter(name_filter)
            .map(|entry| entry.path().display().to_string()) // vec(DirEntry)  타입을 vec(String) 타입으로 변환
            .collect::<Vec<_>>();

        println!("{}", entries.join("\n"));
    }
    Ok(())
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
