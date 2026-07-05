use std::fs::File;
use std::path::PathBuf;

use bril_rs::load_program_from_read;
use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    file: PathBuf,
}

fn main() {
    let args = Args::parse();
    let file = File::open(&args.file).unwrap();
    let bril = load_program_from_read(file);
    println!("{:?}", bril);
}
