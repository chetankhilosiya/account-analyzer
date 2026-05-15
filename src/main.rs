mod components;
mod db;
mod parsers;
mod ui;

use anyhow::Result;
use db::DB;
use dioxus::prelude::*;
use std::path::Path;
use ui::components::App;

fn main() {
    let database;
    match load_file() {
        Ok(db) => database = db,
        Err(_) => {
            println!("Not able to create DB");
            return;
        }
    }
    load_ui(database);
}

fn load_file() -> Result<DB> {
    let file = Path::new("C:\\Users\\cheta\\Downloads\\icici_statement_1.xls");
    if !file.exists() {
        println!("File does not exist");
    }

    let parser = parsers::icici::ICICIParser::new();

    parser.parse_file(file)
}

pub fn load_ui(database: DB) {
    // dioxus::LaunchBuilder::new()
    //     .with_context(database)
    //     .launch(Home);
    asset!("/assets/dx-components-theme.css");
    dioxus::launch(App);
}

#[derive(Clone, PartialEq, Debug)]
enum Bank {
    ICICI,
    SBI,
}
