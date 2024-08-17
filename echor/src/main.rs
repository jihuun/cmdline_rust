/*
//use std::env::args;
use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("Ji-Hun Kim")
        .about("Rust version of echo")
        .arg(
            Arg::new("opt_text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("opt_omit_newline")
                .short('n')
                .action(ArgAction::SetTrue)
                .help("Do not print newline"),
        )
        .get_matches();

    //println!("{:#?}", matches);
    let text: Vec<String> = 
        matches.get_many("opt_text").unwrap().cloned().collect();
    let omit_newline = matches.get_flag("opt_omit_newline");

    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });

}
*/

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// the Rust version of echo
struct Args {
    /// Input text!
    #[arg(required(true))]
    text: Vec<String>,

    /// No print new line
    #[arg(short('n'))]
    omit_newline: bool,
}

fn main() {
    let args = Args::parse();
    print!(
        "{}{}",
        args.text.join(" "),
        if args.omit_newline {""} else {"\n"}
    )
}
