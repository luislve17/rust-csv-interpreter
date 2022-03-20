#![allow(unused)]

use std::error::Error;
use std::path;
use clap::Parser;
use colored::*;
use regex::Regex;
use std::fs;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long, parse(from_os_str))]
    file: std::path::PathBuf,
    #[clap(short, long)]
    sep: char,

}

fn load_file(file_path: std::path::PathBuf) -> Result<String, std::io::Error>{
    if file_path.extension().unwrap() != "csv" {
        let extension_warning_msg: &str = "File is not a csv. Formatter might present errors";
        eprintln!("{}: {}", "Warning".yellow().bold(), extension_warning_msg);
    }
    let file_content = fs::read_to_string(file_path)?;
    return Ok(file_content);
}

fn resolve_quoted_items (items_row: &mut Vec<String>) -> &mut Vec<String> {
    let quote_search = items_row.iter().position(|r| r.contains("\""));
    let mut quote_left_limit = 0 as usize;
    let mut quote_right_limit = 0 as usize;
    return match quote_search {
        Some(_) => {
            quote_left_limit = quote_search.unwrap();
            quote_right_limit = items_row[(quote_left_limit + 1)..].iter().position(|r| r.contains("\"")).unwrap() + quote_left_limit + 1;

            let merged_string = items_row[quote_left_limit..(quote_right_limit + 1)].join("");
            items_row[quote_left_limit] = merged_string;
            items_row.drain((quote_left_limit + 1)..(quote_right_limit + 1));

            let row_reminder = &mut items_row[(quote_left_limit + 1)..].to_vec();
            let resolved_reminder = resolve_quoted_items(row_reminder);
            items_row.drain((quote_left_limit + 1)..);
            items_row.append(resolved_reminder);
            items_row
        }
        None => { items_row }
    }
}

fn read_csv(raw_data: String){
    let line_regex: &str = "(?P<data>.*?)\\n";
    let line_re = Regex::new(line_regex).unwrap();

    for line in line_re.captures_iter(&raw_data) {
        let mut items = line["data"].split(',').map(str::to_string).collect::<Vec<String>>();
        let processed_items = resolve_quoted_items(&mut items);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let raw_data = load_file(args.file)?;
    read_csv(raw_data);
    return Ok(());
}
