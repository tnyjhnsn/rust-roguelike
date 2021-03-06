#![recursion_limit = "512"]
#![feature(or_patterns)]

use wasm_bindgen::prelude::*;

mod model;

mod app;
pub use app::*;
mod map;
pub use map::*;
mod tile_map;
pub use tile_map::*;
mod tile;
pub use tile::*;
mod logs;
pub use logs::*;
mod log;
pub use log::*;
mod armour;
pub use armour::*;
mod inventory;
pub use inventory::*;
mod stats;
pub use stats::*;
mod combat_stats;
pub use combat_stats::*;
mod attr_stats;
pub use attr_stats::*;
mod level_stats;
pub use level_stats::*;
mod dialog;
pub use dialog::*;
mod list;
pub use list::*;
mod list_item;
pub use list_item::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<app::Game>();
    Ok(())
}
