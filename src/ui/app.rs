use anyhow::Result;

use crate::ui::components::Home;

pub fn load_ui() -> Result<()> {
    dioxus::launch(Home);
    Result::Ok(())
}
