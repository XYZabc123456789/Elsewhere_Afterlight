use wasm_bindgen::prelude::*;
pub use wasm_bindgen_rayon::init_thread_pool as initThreadPool;
use rayon::prelude::*;

#[wasm_bindgen]
pub fn add(left: i32, right: i32) -> i32 {
    left + right
}