extern crate neovim_lib;

use neovim_lib::Session;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

enum Messages {
    CurrentSong,
    Unknown,
}

impl From<String> for Messages {
    fn from(event: String) -> Self {
        match &event[..] {
            "current_song" => Messages::CurrentSong,
            _ => Messages::Unknown,
        }
    }
}

/// Neovim is the struct to handle interactions with a running Neovim instance.
pub struct Nvim {
    session: Session,
    log_file: File,
}

impl Nvim {
    pub fn new() -> Nvim {
        let session = Session::new_parent().unwrap();
        let log_file = OpenOptions::new().append(true).open("my-file").unwrap();

        Nvim { session, log_file }
    }

    pub fn handle_events(&mut self) {
        let receiver = self.session.start_event_loop_channel();

        for (event, _values) in receiver {
            match Messages::from(event.clone()) {
                Messages::CurrentSong => {
                    self.log_file.write_all(b"Fetching current song!").unwrap();
                }
                _ => {
                    let msg = format!("Unexpected RPC: {}", event);
                    self.log_file.write_all(msg.as_bytes()).unwrap();
                }
            }
        }
    }
}
