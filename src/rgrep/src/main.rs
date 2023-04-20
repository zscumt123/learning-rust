use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Search text
    text: String,

    /// Search file
    file: String,
}
fn main() -> Result<()> {
    let args = Cli::parse();
    let file = File::open(args.file)?;
    let reader = BufReader::new(file);
    let reg_str = format!("{}", args.text);
    let re = Regex::new(&reg_str).unwrap();
    for (i, line) in reader.lines().enumerate() {
        let text = line.unwrap();

        if re.is_match(&text) {
            let mut s = String::new();
            let mut prev: usize = 0;
            for mat in re.find_iter(&text) {
                s.push_str(&text[prev..mat.start()]);
                let n = String::from(&text[mat.start()..mat.end()]).red().bold();
                println!("{}", n);
                s.push_str(&n);
                prev = mat.end();
            }
            s.push_str(&text[prev..]);
            println!("{}: {}", i, s);
        }
    }
    println!("{}", "abc".red().bold());
    Ok(())
}
