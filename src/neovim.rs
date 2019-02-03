extern crate neovim_lib;

use neovim_lib::{Neovim, NeovimApi, Session};
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

use crate::spotify::{Spotify, SpotifyAPI};

enum Messages {
    CurrentSong,
    PlayPause,
    Play,
    Pause,
    Next,
    Previous,
    Unknown,
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
            _ => Messages::Unknown,
        }
    }
}

/// EventHandler receives RPC requests, and maps them to right Spotify and Neovim commands.
pub struct EventHandler {
    log_file: File,
    nvim: Neovim,
    spotify: Box<SpotifyAPI>,
}

impl EventHandler {
    pub fn new() -> EventHandler {
        let session = Session::new_parent().unwrap();
        let nvim = Neovim::new(session);
        let log_file = OpenOptions::new().append(true).open("my-file").unwrap();
        let spotify = Spotify::new();

        EventHandler {
            nvim,
            log_file,
            spotify: Box::new(spotify),
        }
    }

    pub fn handle_events(&mut self) {
        let receiver = self.nvim.session.start_event_loop_channel();

        for (event, _values) in receiver {
            match Messages::from(event.clone()) {
                Messages::CurrentSong => {
                    let song = self.spotify.current_song();

                    self.log_file.write_all(song.as_bytes()).unwrap();
                    self.nvim.command(&format!("echo \"{}\"", song)).unwrap();
                }

                Messages::PlayPause => {
                    self.log_file.write_all(b"play pause\n").unwrap();
                    self.spotify.play_pause();
                },

                Messages::Play => {
                    self.log_file.write_all(b"play\n").unwrap();
                    self.spotify.play();
                },

                Messages::Pause => {
                    self.log_file.write_all(b"pause\n").unwrap();
                    self.spotify.pause();
                },

                Messages::Next => {
                    self.log_file.write_all(b"next\n").unwrap();
                    self.spotify.next();
                },

                Messages::Previous => {
                    self.log_file.write_all(b"previous\n").unwrap();
                    self.spotify.previous();
                },

                // Handle any "Unknown" messages.
                Messages::Unknown => {
                    let msg = format!("Unexpected RPC: {}", event);
                    self.log_file.write_all(msg.as_bytes()).unwrap();
                }
            }
        }
    }
}
