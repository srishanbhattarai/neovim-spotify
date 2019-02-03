extern crate neovim_lib;

mod lyrics;
mod neovim;
mod spotify;

fn main() {
    let mut nvim = neovim::EventHandler::new();

    // Block
    nvim.handle_events();
}
