extern crate neovim_lib;

mod neovim;

fn main() {
    let mut nvim = neovim::Nvim::new();

    // Block
    nvim.handle_events();
}
