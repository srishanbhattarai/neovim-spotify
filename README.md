# neovim-spotify &middot; [![Build Status](https://travis-ci.com/srishanbhattarai/neovim-spotify.svg?token=r9ZKJZspyajhDz5EguyH&branch=master)](https://travis-ci.com/srishanbhattarai/neovim-spotify)
Control Spotify through Neovim. (Currently only MacOS)

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

# License
MIT
