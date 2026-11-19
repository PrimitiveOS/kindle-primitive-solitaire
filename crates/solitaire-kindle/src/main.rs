use solitaire_core::Board;
use solitaire_render::{seed, View};
use std::{fs, path::PathBuf};

fn main() -> Result<(), String> {
    let dir = PathBuf::from("/mnt/us/solitaire");
    let data = dir.join("deal.json");
    let board = Board::restore(&fs::read(&data).unwrap_or_default(), seed());
    primitive_app::run(View::new(board, Some(data)), &dir)
}
