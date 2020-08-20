#![recursion_limit = "512"]

use wasm_bindgen::prelude::*;

mod model;

mod app;
pub use app::*;
mod game;
pub use game::*;
mod map;
pub use map::*;
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
mod logs;
pub use logs::*;
mod log;
pub use log::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<app::Model>();
    Ok(())
}
