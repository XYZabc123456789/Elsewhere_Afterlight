use super::*;
use futures::channel::oneshot;
use serde_wasm_bindgen::*;
use slotmap::*;
use std::any::Any;
use std::sync::{Mutex, MutexGuard, OnceLock};
mod registry;
pub use registry::*;
mod jobmachinery;
pub use jobmachinery::*;

#[derive(Debug, Copy, Clone)]
#[wasm_bindgen]
pub enum JobKind {
    NOP,
    MIRROR,
}

impl Job {
    /// Common dispatch for all known jobs
    fn execute(&self, input: Option<JobValue>) -> Res<Option<JobValue>> {
        use JobKind::*;
        match self.kind {
            NOP => Ok(None),
            MIRROR => mirror(input),
        }
    }
}

pub fn mirror(i: Option<JobValue>) -> Res<Option<JobValue>> {
    Ok(i)
}