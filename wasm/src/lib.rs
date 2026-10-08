use wasm_bindgen::prelude::*;
mod jobrunner;
use wasm_bindgen_rayon::init_thread_pool;
type Res<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;
use jobrunner::*;

#[wasm_bindgen]
pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

#[wasm_bindgen]
pub async fn setup(threads: usize) -> Result<JsValue, JsValue> {
    let r = init_thread_pool(threads).await;
    let reg = get_registry();
    release_registry(reg);
    r
}