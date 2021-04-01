mod lyrics;
mod neovim;
mod spotify;
mod lib;
mod applemusic;

fn main() {
    let mut nvim = neovim::EventHandler::new();

    nvim.handle_events();
}
