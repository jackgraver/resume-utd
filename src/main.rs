mod tui;
mod data;

use std::io;

fn main() -> io::Result<()> {
    tui::run_app()
}