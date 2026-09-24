use dioxus::prelude::*;

use crate::db::{DB, Entry};

const DATE_FMT: &str = "%d/%m/%Y";

/// A pre-formatted display row for the statement table.
#[derive(Clone)]
struct DisplayRow {
    sno: usize,
    value_date_str: String,
    transaction_date_str: String,
    description: String,
    withdrawal_display: String,
    deposit_display: String,
    balance_display: String,
}

impl DisplayRow {
    fn from_entry(entry: &Entry, idx: usize) -> Self {
        let value_date_str = entry.value_date.format(DATE_FMT).to_string();
        let transaction_date_str = entry.transaction_date.format(DATE_FMT).to_string();

        let withdrawal_display = if entry.amount.is_withdrawal() && entry.amount.value() > 0.0 {
            format!("{:.2}", entry.amount.value())
        } else {
            String::new()
        };

        let deposit_display = if entry.amount.is_deposit() && entry.amount.value() > 0.0 {
            format!("{:.2}", entry.amount.value())
        } else {
            String::new()
        };

        let balance_display = if entry.balance > 0.0 {
            format!("{:.2}", entry.balance)
        } else {
            String::new()
        };

        DisplayRow {
            sno: idx + 1,
            value_date_str,
            transaction_date_str,
            description: entry.description.clone(),
            withdrawal_display,
            deposit_display,
            balance_display,
        }
    }

    /// Check if this row matches the given filters.
    fn matches(
        &self,
        sno_filter: &str,
        value_date_filter: &str,
        transaction_date_filter: &str,
        description_filter: &str,
        withdrawal_filter: &str,
        deposit_filter: &str,
        balance_filter: &str,
    ) -> bool {
        if !sno_filter.is_empty() && !self.sno.to_string().contains(sno_filter) {
            return false;
        }
        if !value_date_filter.is_empty() && !self.value_date_str.contains(value_date_filter) {
            return false;
        }
        if !transaction_date_filter.is_empty() && !self.transaction_date_str.contains(transaction_date_filter) {
            return false;
        }
        if !description_filter.is_empty() && !self.description.contains(description_filter) {
            return false;
        }
        if !withdrawal_filter.is_empty() && !self.withdrawal_display.contains(withdrawal_filter) {
            return false;
        }
        if !deposit_filter.is_empty() && !self.deposit_display.contains(deposit_filter) {
            return false;
        }
        if !balance_filter.is_empty() && !self.balance_display.contains(balance_filter) {
            return false;
        }
        true
    }
}

/// Filter input component for a single column header.
#[component]
fn ColumnFilterInput(
    placeholder: &'static str,
    filter_value: String,
    on_filter_change: EventHandler<String>,
) -> Element {
    rsx! {
        input {
            class: "column-filter-input",
            placeholder,
            value: "{filter_value}",
            oninput: move |e| {
                on_filter_change.call(e.value());
            },
        }
    }
}

/// Display all ICICI bank statement entries in a tabular view with column filters.
#[component]
pub fn IciciStatementTable() -> Element {
    let records = move || DB.read().records().clone();

    let records = records();
    if records.is_empty() {
        return rsx! {
            div { class: "empty-state", p { "No transactions found. Load a bank statement to view data." } }
        };
    }

    // Column filter state using signals
    let mut sno_filter = use_signal(|| String::new());
    let mut value_date_filter = use_signal(|| String::new());
    let mut transaction_date_filter = use_signal(|| String::new());
    let mut description_filter = use_signal(|| String::new());
    let mut withdrawal_filter = use_signal(|| String::new());
    let mut deposit_filter = use_signal(|| String::new());
    let mut balance_filter = use_signal(|| String::new());

    // Pre-format display values
    let indexed_records: Vec<DisplayRow> = records
        .iter()
        .enumerate()
        .map(|(idx, entry)| DisplayRow::from_entry(entry, idx))
        .collect();

    // Filter the records based on current filter state
    let filtered_records = {
        let sno_f = sno_filter().clone();
        let vd_f = value_date_filter().clone();
        let td_f = transaction_date_filter().clone();
        let desc_f = description_filter().clone();
        let wd_f = withdrawal_filter().clone();
        let dep_f = deposit_filter().clone();
        let bal_f = balance_filter().clone();

        indexed_records
            .iter()
            .filter(|row| {
                row.matches(
                    &sno_f,
                    &vd_f,
                    &td_f,
                    &desc_f,
                    &wd_f,
                    &dep_f,
                    &bal_f,
                )
            })
            .cloned()
            .collect::<Vec<_>>()
    };

    rsx! {
        div { class: "icici-statement-table",
            h2 { class: "table-title", "ICICI Bank Statement" }
            table { class: "statement-table table",
                thead {
                    tr { class: "header-row",
                        th {
                            span { class: "th-label", "S No." }
                            ColumnFilterInput {
                                placeholder: "Filter...",
                                filter_value: sno_filter.read().clone(),
                                on_filter_change: move |val| {
                                    sno_filter.set(val);
                                },
                            }
                        }
                        th {
                            span { class: "th-label", "Value Date" }
                            ColumnFilterInput {
                                placeholder: "Filter...",
                                filter_value: value_date_filter.read().clone(),
                                on_filter_change: move |val| {
                                    value_date_filter.set(val);
                                },
                            }
                        }
                        th {
                            span { class: "th-label", "Transaction Date" }
                            ColumnFilterInput {
                                placeholder: "Filter...",
                                filter_value: transaction_date_filter.read().clone(),
                                on_filter_change: move |val| {
                                    transaction_date_filter.set(val);
                                },
                            }
                        }
                        th {
                            span { class: "th-label", "Transaction Remarks" }
                            ColumnFilterInput {
                                placeholder: "Filter...",
                                filter_value: description_filter.read().clone(),
                                on_filter_change: move |val| {
                                    description_filter.set(val);
                                },
                            }
                        }
                        th {
                            span { class: "th-label", "Withdrawal (INR)" }
                            ColumnFilterInput {
                                placeholder: "Filter...",
                                filter_value: withdrawal_filter.read().clone(),
                                on_filter_change: move |val| {
                                    withdrawal_filter.set(val);
                                },
                            }
                        }
                        th {
                            span { class: "th-label", "Deposit (INR)" }
                            ColumnFilterInput {
                                placeholder: "Filter...",
                                filter_value: deposit_filter.read().clone(),
                                on_filter_change: move |val| {
                                    deposit_filter.set(val);
                                },
                            }
                        }
                        th {
                            span { class: "th-label", "Balance (INR)" }
                            ColumnFilterInput {
                                placeholder: "Filter...",
                                filter_value: balance_filter.read().clone(),
                                on_filter_change: move |val| {
                                    balance_filter.set(val);
                                },
                            }
                        }
                    }
                }
                tbody {
                    for row in filtered_records.iter() {
                        tr { class: "entry-row", key: "{row.sno}",
                            td { class: "col-sno", "{row.sno}" }
                            td { class: "col-date date", "{row.value_date_str}" }
                            td { class: "col-date date", "{row.transaction_date_str}" }
                            td { class: "col-description description",
                                span { class: "description-text", "{row.description}" }
                            }
                            td { class: "col-withdrawal amount-cell",
                                if !row.withdrawal_display.is_empty() {
                                    span { class: "amount withdrawal", "- INR {row.withdrawal_display}" }
                                } else {
                                    "-"
                                }
                            }
                            td { class: "col-deposit amount-cell",
                                if !row.deposit_display.is_empty() {
                                    span { class: "amount deposit", "+ INR {row.deposit_display}" }
                                } else {
                                    "-"
                                }
                            }
                            td { class: "col-balance amount-cell",
                                if !row.balance_display.is_empty() {
                                    span { class: "amount balance", "INR {row.balance_display}" }
                                } else {
                                    "-"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
