use anyhow::{Result,bail};
use clap::{Parser};
use std::ops::Range;

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

type PositionList = Vec<Range<usize>>;

#[derive(Debug)]
pub enum Extract {
    Fields(PositionList),
    Bytes(PositionList),
    Chars(PositionList),
}

fn parse_pos(range: String) -> Result<PositionList> {
    todo!();
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



#[cfg(test)]
mod unit_tests {
    use super::parse_pos;

    #[test]
    fn test_parse_pos() {
        // The empty string is an error
        assert!(parse_pos("".to_string()).is_err());
    }
    #[test]
    fn test_parse_pos_zero() {
        // Zero is an error
        let res = parse_pos("0".to_string());
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            r#"illegal list value: "0""#
        );
    }

    #[test]
    fn test_parse_pos_zero2() {
        let res = parse_pos("0-1".to_string());
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            r#"illegal list value: "0""#
        );
    }
    #[test]
    fn test_parse_pos_plus() {
        // A leading "+" is an error
        let res = parse_pos("+1".to_string());
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            r#"illegal list value: "+1""#,
        );
    }

    #[test]
    fn test_parse_pos_plus2() {
        let res = parse_pos("+1-2".to_string());
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            r#"illegal list value: "+1-2""#,
        );
    }
    #[test]
    fn test_parse_pos_plus3() {
        let res = parse_pos("1-+2".to_string());
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            r#"illegal list value: "1-+2""#,
        );
    }

    #[test]
    fn test_parse_pos_non_number1() {
        // Any non-number is an error
        let res = parse_pos("a".to_string());
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            r#"illegal list value: "a""#
        );
    }
    #[test]
    fn test_parse_pos_non_number2() {
        let res = parse_pos("1,a".to_string());
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            r#"illegal list value: "a""#
        );
    }


    #[test]
    fn test_parse_pos_non_number3() {
        let res = parse_pos("1-a".to_string());
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            r#"illegal list value: "1-a""#,
        );
    }
    #[test]
    fn test_parse_pos_non_number4() {
        let res = parse_pos("a-1".to_string());
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            r#"illegal list value: "a-1""#,
        );
    }

    #[test]
    fn test_parse_pos_non_wonky_ranges() {
        // Wonky ranges
        let res = parse_pos("-".to_string());
        assert!(res.is_err());

        let res = parse_pos(",".to_string());
        assert!(res.is_err());

        let res = parse_pos("1,".to_string());
        assert!(res.is_err());

        let res = parse_pos("1-".to_string());
        assert!(res.is_err());

        let res = parse_pos("1-1-1".to_string());
        assert!(res.is_err());

        let res = parse_pos("1-1-a".to_string());
        assert!(res.is_err());
    }

    #[test]
    fn test_parse_pos_inc_nums() {
        // First number must be less than second
        let res = parse_pos("1-1".to_string());
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "First number in range (1) must be lower than second number (1)"
        );

        let res = parse_pos("2-1".to_string());
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "First number in range (2) must be lower than second number (1)"
        );
    }

    #[test]
    fn test_parse_pos_inc_acceptable() {
        // All the following are acceptable
        let res = parse_pos("1".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1]);

        let res = parse_pos("01".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1]);

        let res = parse_pos("1,3".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 2..3]);

        let res = parse_pos("001,0003".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 2..3]);

        let res = parse_pos("1-3".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..3]);

        let res = parse_pos("0001-03".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..3]);

        let res = parse_pos("1,7,3-5".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 6..7, 2..5]);

        let res = parse_pos("15,19-20".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![14..15, 18..20]);
    }
}
