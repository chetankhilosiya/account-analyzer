use dioxus::prelude::*;
use dioxus_tabular::{ColumnContext, Columns, GetRowData, Row, TableColumn};

use crate::{DB, db::Entry};

#[component]
pub fn StatementPage() -> Element {
    rsx! { "Statement Page" }
}

#[component]
pub fn StatementDetails<R: Row, C: Columns<R>>(rows: ReadSignal<Vec<R>>, columns: C) -> Element {
    // let db = use_context::<Signal<DB>>();
    // println!("{:?}", db);

    // rsx! { "Statement details" }
    // let table_context = TableContext::use_table_context(columns.column_names());
    rsx! {
        table {
            // thead {
            //     tr { {columns.render_headers(table_context)} }
            // }
            // tbody {
            //     for row in rows.iter() {
            //         tr { key: "{row.key().into()}",
            //             {columns.render_columns(table_context, &row, vec![])}
            //         }
            //     }
            // }
        }
    }
}

impl Row for Entry {
    fn key(&self) -> impl Into<String> {
        self.transaction_date.format("%d-%m-%y").to_string()
    }
}

#[derive(Clone, PartialEq)]
struct StatementAmount(f64);

impl GetRowData<StatementAmount> for Entry {
    fn get(&self) -> StatementAmount {
        StatementAmount(self.amount.value())
    }
}

#[derive(Clone, PartialEq)]
struct AmountColumn;

impl<R: Row + GetRowData<StatementAmount>> TableColumn<R> for AmountColumn {
    fn column_name(&self) -> String {
        "Amount".into()
    }

    fn render_header(&self, _context: ColumnContext, attributes: Vec<Attribute>) -> Element {
        rsx! {
            th { ..attributes,"Amount" }
        }
    }

    fn render_cell(&self, _context: ColumnContext, row: &R, attributes: Vec<Attribute>) -> Element {
        rsx! {
            td { ..attributes,"{row.get().0}" }
        }
    }
}
