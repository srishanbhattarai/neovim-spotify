mod lyrics;
mod neovim;
mod spotify;

fn main() {
    let mut nvim = neovim::EventHandler::new();

    nvim.handle_events();
}
