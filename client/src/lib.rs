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
mod contents_map;
pub use contents_map::*;
mod status_map;
pub use status_map::*;
mod tile;
pub use tile::*;
mod contents;
pub use contents::*;
mod status;
pub use status::*;
mod logs;
pub use logs::*;
mod log;
pub use log::*;
mod inventory;
pub use inventory::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<app::Model>();
    Ok(())
}
