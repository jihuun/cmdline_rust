use clap::{Parser, builder::PossibleValue, ValueEnum};
use regex::Regex;
use anyhow::Result;

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
    #[arg(name = "PATH", default_value = ".")]
    paths: Vec<String>,

    /// Name
    #[arg(short, long)]
    names: Vec<Regex>,

    /// Entry Type
    #[arg(
        short('t'),
        long("type"),
        name = "TYPE",
        value_parser(clap::value_parser!(EntryType)),
    )]
    entry_types: Vec<EntryType>,
}

fn run(args: Args) -> Result<()> {
    println!("{args:?}");
    Ok(())
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
