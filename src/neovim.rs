extern crate neovim_lib;

use neovim_lib::{Neovim, NeovimApi, Session};

use crate::spotify::{Spotify, SpotifyAPI};

enum Messages {
    CurrentSong,
    PlayPause,
    Play,
    Pause,
    Next,
    Previous,
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
        let session = Session::new_parent().unwrap();
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
                },

                Messages::Play => {
                    self.spotify.play();
                },

                Messages::Pause => {
                    self.spotify.pause();
                },

                Messages::Next => {
                    self.spotify.next();
                },

                Messages::Previous => {
                    self.spotify.previous();
                },

                // Handle any "Unknown" messages.
                Messages::Unknown(ev) => {
                    self.nvim.command(&format!("echoerr \"{}\" Unknown command", ev)).unwrap();
                }
            }
        }
    }
}
