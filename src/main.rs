extern crate neovim_lib;

mod neovim;
mod spotify;

fn main() {
    let mut nvim = neovim::EventHandler::new();

    // Block
    nvim.handle_events();
}
