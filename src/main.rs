mod components;
mod db;
mod parsers;
mod ui;

use anyhow::Result;
use db::{Bank, Database};
use dioxus::prelude::*;
use std::path::Path;
use ui::components::StatementDetails;

fn main() {
    asset!("/assets/dx-components-theme.css");
    dioxus::launch(App);
}

#[component]
pub fn App() -> Element {
    static DB: GlobalSignal<Database> = Signal::global(|| Database::new(Bank::ICICI));
    match load_file() {
        Ok(db) => {
            DB.write().set_bank(db.clone().bank());
            DB.write().set_records(db.clone().records());
        }
        Err(_) => {
            println!("Not able to create DB");
        }
    }
    rsx! {
        div { "App!" }
        div { StatementDetails {} }
    }
}

fn load_file() -> Result<Database> {
    let file = Path::new("C:\\Users\\cheta\\Downloads\\icici_statement_1.xls");
    if !file.exists() {
        println!("File does not exist");
    }

    let parser = parsers::icici::ICICIParser::new();

    parser.parse_file(file)
}
