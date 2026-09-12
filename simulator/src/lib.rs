mod components;

pub use components::{Memory, MemoryError, RegFile, RegReadError, RegWriteError};

#[derive(Default)]
pub struct Simulator;
