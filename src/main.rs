mod components;
mod db;
mod parsers;
mod ui;

use anyhow::Result;
use db::{Bank, Database};
use dioxus::prelude::*;
use std::path::Path;
use ui::components::StatementPage;

static DB: GlobalSignal<Database> = Signal::global(|| Database::new(Bank::ICICI));

fn main() {
    asset!("/assets/dx-components-theme.css");
    dioxus::launch(App);
}

#[component]
pub fn App() -> Element {
    match load_file() {
        Ok(db) => {
            DB.with_mut(|d| *d = db);
        }
        Err(_) => {
            println!("Not able to create DB");
        }
    }
    rsx! {
        div { "App!" }
        div { StatementPage {} }
    }
}

fn load_file() -> Result<Database> {
    let file = Path::new("./data/icici_statement_1.xls");
    if !file.exists() {
        println!("File does not exist");
    }

    let parser = parsers::icici::ICICIParser::new();

    parser.parse_file(file)
}
