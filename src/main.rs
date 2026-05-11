mod db;
mod parsers;
mod ui;

use std::path::Path;

use anyhow::Result;
use db::DB;

fn main() {
    let ui_result = ui::load_ui();
    if ui_result.is_err() {
        println!("Unable to load UI");
    }
    let result = load_file();
    if result.is_err() {
        println!("Not able to crate DB");
    }
}

fn load_file() -> Result<DB> {
    let file = Path::new("C:\\Users\\cheta\\Downloads\\icici_statement_1.xls");
    if !file.exists() {
        println!("File does not exist");
    }

    let parser = parsers::icici::ICICIParser::new();

    parser.parse_file(file)
}

enum Bank {
    ICICI,
    SBI,
}
