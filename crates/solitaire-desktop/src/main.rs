use primitive_app::{font_paths, frame, write_png, App, Change, HEIGHT, WIDTH};
use primitive_ui::{Canvas, Input};
use solitaire_core::Board;
use solitaire_render::View;
use std::{
    fs,
    path::{Path, PathBuf},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = PathBuf::from(
        std::env::args_os()
            .nth(1)
            .unwrap_or_else(|| "build/previews".into()),
    );
    fs::create_dir_all(&output)?;
    let fonts =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../doannt-kindle-launcher/assets/fonts");
    let mut canvas = Canvas::new(WIDTH, HEIGHT, &font_paths(&fonts))?;
    let mut app = View::new(Board::new(39, false), None);
    assert_eq!(app.input(Input::Tap(90.0, 315.0)), Change::Partial);
    assert_eq!(app.board.waste.len(), 1);
    write_png(
        &output.join("solitaire.png"),
        WIDTH,
        HEIGHT,
        &frame(&app, &mut canvas, false),
    )?;
    write_png(
        &output.join("solitaire-dark.png"),
        WIDTH,
        HEIGHT,
        &frame(&app, &mut canvas, true),
    )?;
    Ok(())
}
