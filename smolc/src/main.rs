use std::fs::File;
use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    file: PathBuf,
}

fn main() {
    let args = Args::parse();
    let _file = File::open(args.file).unwrap();
}
