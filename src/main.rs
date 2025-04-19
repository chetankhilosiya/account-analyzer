mod db;
mod parsers;

use std::path::Path;

use anyhow::Result;
use db::DB;

fn main() {
    let file = Path::new("C:\\Users\\cheta\\Downloads\\icici_statement_1.xls");
    if !file.exists() {
        println!("File does not exist");
    }

    let parser = parsers::icici::ICICIParser::new();

    let result: Result<DB> = parser.parse_file(file);
    if result.is_err() {
        println!("Not able to crate DB");
    }
}

enum Bank {
    ICICI,
    SBI,
}
