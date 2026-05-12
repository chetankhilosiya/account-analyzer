use dioxus::prelude::*;

use crate::db::DB;

#[component]
pub fn Table() -> Element {
    // let db = use_context::<Signal<DB>>();
    // println!("{:?}", db);
    rsx! {
        table {
            thead {
                tr {
                    th { "Amount" }
                    th { "Amount Type" }
                }
            }
            tbody {
                tr {
                    td { "100.34" }
                    td { "Withdrawal" }
                }
            }
        }
    }
}
