mod components;

pub use components::{RegFile, RegReadError, RegWriteError};

#[derive(Default)]
pub struct Simulator;
