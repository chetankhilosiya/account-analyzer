use dioxus::prelude::*;

use crate::db::{DB, Entry};

const DATE_FMT: &str = "%d/%m/%Y";

/// A pre-formatted display row for the statement table.
#[derive(Clone)]
struct DisplayRow {
    withdrawal_display: String,
    deposit_display: String,
    balance_display: String,
}

impl DisplayRow {
    fn from_entry(entry: &Entry) -> Self {
        let withdrawal_display = if entry.amount.is_withdrawal() && entry.amount.value() > 0.0 {
            format!("- INR {:.2}", entry.amount.value())
        } else {
            "-".to_string()
        };

        let deposit_display = if entry.amount.is_deposit() && entry.amount.value() > 0.0 {
            format!("+ INR {:.2}", entry.amount.value())
        } else {
            "-".to_string()
        };

        let balance_display = if entry.balance > 0.0 {
            format!("INR {:.2}", entry.balance)
        } else {
            "-".to_string()
        };

        DisplayRow {
            withdrawal_display,
            deposit_display,
            balance_display,
        }
    }
}

/// Display all ICICI bank statement entries in a tabular view.
#[component]
pub fn IciciStatementTable() -> Element {
    let records = move || DB.read().records().clone();

    let records = records();
    if records.is_empty() {
        return rsx! {
            div { class: "empty-state", p { "No transactions found. Load a bank statement to view data." } }
        };
    }

    // Pre-format display values before entering rsx (let bindings not allowed inside rsx!)
    let indexed_records: Vec<(usize, Entry, DisplayRow)> = records
        .iter()
        .enumerate()
        .map(|(idx, entry)| {
            (idx, entry.clone(), DisplayRow::from_entry(entry))
        })
        .collect();

    rsx! {
        div { class: "icici-statement-table",
            h2 { class: "table-title", "ICICI Bank Statement" }
            table { class: "statement-table",
                thead {
                    tr {
                        th { "S No." }
                        th { "Value Date" }
                        th { "Transaction Date" }
                        th { "Transaction Remarks" }
                        th { "Withdrawal Amount (INR)" }
                        th { "Deposit Amount (INR)" }
                        th { "Balance (INR)" }
                    }
                }
                tbody {
                    for (idx, entry, display) in indexed_records.iter() {
                        tr { class: "entry-row", key: "{idx}",
                            td { class: "col-sno", "{idx + 1}" }
                            td { class: "col-date date", "{entry.value_date.format(DATE_FMT)}" }
                            td { class: "col-date date", "{entry.transaction_date.format(DATE_FMT)}" }
                            td { class: "col-description description",
                                span { class: "description-text", "{entry.description}" }
                            }
                            td { class: "col-withdrawal amount-cell", "{display.withdrawal_display}" }
                            td { class: "col-deposit amount-cell", "{display.deposit_display}" }
                            td { class: "col-balance amount-cell", "{display.balance_display}" }
                        }
                    }
                }
            }
        }
    }
}
