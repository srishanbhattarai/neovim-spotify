mod backends;
mod lyrics;
mod neovim;

fn main() {
    let mut nvim = neovim::EventHandler::new();

    nvim.handle_events();
}
