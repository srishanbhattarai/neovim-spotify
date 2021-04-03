# neovim-spotify &middot; [![Build Status](https://travis-ci.com/srishanbhattarai/neovim-spotify.svg?token=r9ZKJZspyajhDz5EguyH&branch=master)](https://travis-ci.com/srishanbhattarai/neovim-spotify)
> Control Spotify and Apple Music through Neovim (for MacOS)

If you want to learn how to create Neovim plugins with Rust, read the accompanying blog post [here](https://medium.com/@srishanbhattarai/a-detailed-guide-to-writing-your-first-neovim-plugin-in-rust-a81604c606b1).

# Installation
Use your preferred plugin manager. Run `install.sh` as a post-installation step, which will download and install the pre-built macOS binary.

For example, for `vim-plug`, you can put in the following line into your `.vimrc`:
```vim
Plug 'srishanbhattarai/neovim-spotify', { 'do': 'bash install.sh' }
```

Note: When updating this plugin, please restart Vim before runnning the commands to make the plugin use the updated binaries.

# Usage
Refer to the following table to find supported commands.

| Command                    | Description                                               |
|----------------------------|-----------------------------------------------------------|
| `:SpotifyCurrentSong`      | Spotify: Echo the current song playing                    |
| `:SpotifyNext`             | Spotify: Play next track                                  |
| `:SpotifyPrevious`         | Spotify: Play previous track                              |
| `:SpotifyPlay`             | Spotify: Play the track, if currently paused              |
| `:SpotifyPause`            | Spotify: Pause the track, if currently playing            |
| `:SpotifyLyrics`           | Spotify: Find the lyrics for the current track            |
|                            |                                                           |
| `:AppleMusicCurrentSong`   | Apple Music: Echo the current song playing in             |
| `:AppleMusicNext`          | Apple Music: Play next track                              |
| `:AppleMusicPrevious`      | Apple Music: Play previous track                          |
| `:AppleMusicPlay`          | Apple Music: Play the track, if currently paused          |
| `:AppleMusicPause`         | Apple Music: Pause the track, if currently playing        |
| `:AppleMusicLyrics`        | Apple Music: Find the lyrics for the current track        |

# License
MIT
