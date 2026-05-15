use dioxus::prelude::*;

#[component]
pub fn StatementDetails() -> Element {
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
