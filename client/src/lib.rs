#![recursion_limit = "512"]

use wasm_bindgen::prelude::*;

mod app;
pub use app::*;
mod dungeon;
pub use dungeon::*;
mod tile_map;
pub use tile_map::*;
mod entity_map;
pub use entity_map::*;
mod status_map;
pub use status_map::*;
mod tile;
pub use tile::*;
mod entity;
pub use entity::*;
mod status;
pub use status::*;
mod viewport;
pub use viewport::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<app::Model>();
    Ok(())
}
