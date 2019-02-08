# neovim-spotify &middot; [![Build Status](https://travis-ci.com/srishanbhattarai/neovim-spotify.svg?token=r9ZKJZspyajhDz5EguyH&branch=master)](https://travis-ci.com/srishanbhattarai/neovim-spotify)
> Control Spotify through Neovim. (Currently only MacOS).

If you want to learn how to create Neovim plugins with Rust, read the accompanying blog posts [here](https://medium.com/@srishanbhattarai/a-detailed-guide-to-writing-your-first-neovim-plugin-in-rust-a81604c606b1).

# Installation
Use your preferred plugin manager. For example, `vim-plug`:
```vim
Plug 'srishanbhattarai/neovim-spotify', { 'do': 'bash install.sh' }
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
| `:SpotifyLyrics` | Find the lyrics for the current track | 

# TODOs
* Better error handling - get rid of `unwrap()`s everywhere.
* Non-blocking strategy to handle RPC requests.

# License
MIT
