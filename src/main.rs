#![allow(unused)]
use std::error::Error;
use std::path;
use clap::Parser;

mod utils;
use utils::file_handling;
use utils::data_processing;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long, parse(from_os_str))]
    file: std::path::PathBuf,
    #[clap(short, long)]
    sep: char,

}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let raw_data = file_handling::load_file(args.file)?;
    data_processing::read_csv(raw_data);
    return Ok(());
}
