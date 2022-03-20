use std::fs;
use colored::*;

pub fn load_file(file_path: std::path::PathBuf) -> Result<String, std::io::Error>{
    if file_path.extension().unwrap() != "csv" {
        let extension_warning_msg: &str = "File is not a csv. Formatter might present errors";
        eprintln!("{}: {}", "Warning".yellow().bold(), extension_warning_msg);
    }
    let file_content = fs::read_to_string(file_path)?;
    return Ok(file_content);
}

