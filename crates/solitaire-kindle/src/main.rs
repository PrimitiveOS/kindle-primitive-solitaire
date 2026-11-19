use solitaire_render::View;
use std::path::PathBuf;

fn main() -> Result<(), String> {
    let dir = PathBuf::from("/mnt/us/solitaire");
    primitive_app::run(View::load(dir.join("deal.json")), &dir)
}
