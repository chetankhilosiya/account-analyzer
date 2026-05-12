use dioxus::prelude::*;

use crate::db::DB;
use crate::ui::components::Table;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { "Home! Testing hot reload" }
        div { Table {} }
    }
}
