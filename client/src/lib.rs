#![recursion_limit = "512"]

use wasm_bindgen::prelude::*;

mod app;
mod dungeon;
pub use dungeon::*;
mod level;
pub use level::*;
mod entities;
pub use entities::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<app::Model>();
    Ok(())
}
