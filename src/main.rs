mod extracter;

use clap::Parser;
use extracter::greetings;

#[derive(Parser, Debug)]
struct Cli {
    name: String,
}

fn main() {
    let args = Cli::parse();
    greetings(&args.name);
}
