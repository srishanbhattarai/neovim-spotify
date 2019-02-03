extern crate neovim_lib;

use neovim_lib::{Neovim, NeovimApi, Session};

use crate::lyrics;
use crate::spotify::{Spotify, SpotifyAPI};

enum Messages {
    CurrentSong,
    PlayPause,
    Play,
    Pause,
    Next,
    Previous,
    Lyrics,
    Unknown(String),
}

impl From<String> for Messages {
    fn from(event: String) -> Self {
        match &event[..] {
            "current_song" => Messages::CurrentSong,
            "play_pause" => Messages::PlayPause,
            "play" => Messages::Play,
            "pause" => Messages::Pause,
            "next" => Messages::Next,
            "previous" => Messages::Previous,
            "lyrics" => Messages::Lyrics,
            _ => Messages::Unknown(event),
        }
    }
}

/// EventHandler receives RPC requests, and maps them to right Spotify and Neovim commands.
pub struct EventHandler {
    nvim: Neovim,
    spotify: Box<SpotifyAPI>,
}

impl EventHandler {
    pub fn new() -> EventHandler {
        let mut session = Session::new_parent().unwrap();
        session.set_infinity_timeout();
        let nvim = Neovim::new(session);
        let spotify = Spotify::new();

        EventHandler {
            nvim,
            spotify: Box::new(spotify),
        }
    }

    pub fn handle_events(&mut self) {
        let receiver = self.nvim.session.start_event_loop_channel();

        for (event, _values) in receiver {
            match Messages::from(event.clone()) {
                Messages::CurrentSong => {
                    let song = self.spotify.current_song();

                    self.nvim.command(&format!("echo \"{}\"", song)).unwrap();
                }

                Messages::PlayPause => {
                    self.spotify.play_pause();
                }

                Messages::Play => {
                    self.spotify.play();
                }

                Messages::Pause => {
                    self.spotify.pause();
                }

                Messages::Next => {
                    self.spotify.next();
                }

                Messages::Previous => {
                    self.spotify.previous();
                }

                Messages::Lyrics => {
                    let song = self.spotify.current_song();
                    let mut parts = song.split('-');
                    let (artist, song) =
                        (parts.next().unwrap().trim(), parts.next().unwrap().trim());

                    let lyrics = lyrics::find_lyrics(artist, song).unwrap();
                    let lyrics_vec = lyrics.split('\n').map(|s| s.to_owned()).collect();

                    self.nvim.command("vsplit new").unwrap();
                    let buf = self.nvim.get_current_buf().unwrap();
                    let buf_len = buf.line_count(&mut self.nvim).unwrap();
                    buf.set_lines(&mut self.nvim, 0, buf_len, true, lyrics_vec)
                        .unwrap();
                }

                // Handle any "Unknown" messages.
                Messages::Unknown(ev) => {
                    self.nvim
                        .command(&format!("echoerr \"{}\" Unknown command", ev))
                        .unwrap();
                }
            }
        }
    }
}
