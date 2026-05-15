use dioxus::prelude::*;

use crate::ui::components::StatementDetails;

#[component]
pub fn App() -> Element {
    rsx! {
        div { "App!" }
        div { StatementDetails {} }
    }
}
