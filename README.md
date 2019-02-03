# neovim-spotify &middot; [![Build Status](https://travis-ci.com/srishanbhattarai/neovim-spotify.svg?token=r9ZKJZspyajhDz5EguyH&branch=master)](https://travis-ci.com/srishanbhattarai/neovim-spotify)
Control Spotify through Neovim. (Currently only MacOS)

# Installation
This plugin has a dependency on Rust's pacakge manager, Cargo. There are plans to remove this limitation by providing pre-built binaries.

Use your preferred plugin manager. For example, `vim-plug`:
```vim
Plug 'srishanbhattarai/neovim-spotify', { 'do': 'cargo build --release' }
```

# Usage
Refer to the following table to find supported commands.

| Command  | Description |
|----------|-------------|
| `:SpotifyCurrentSong` | Echo the current song playing in Spotify |
| `:SpotifyNext` | Play next track |
| `:SpotifyPrevious` | Play previous track |
| `:SpotifyPlay` | Play the track, if currently paused |
| `:SpotifyPause` | Pause the track, if currently playing |


