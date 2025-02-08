mod colors;
mod ray_tracing;

use std::sync::mpsc::Sender;

pub use colors::*;
pub use ray_tracing::*;

use crate::ScreenChunk;

pub trait View {
    fn step(&mut self, tx: Sender<ScreenChunk>, width: u32, height: u32);
}
