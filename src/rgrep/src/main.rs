use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Search text
    text: String,

    /// Search file
    file: String,
}

fn main() {
    let args = Cli::parse();

    println!("{:?}", args);
}
