extern crate neovim_lib;

use neovim_lib::{Neovim, NeovimApi, Session, Value};
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::sync::mpsc::Receiver;

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
    log_file: File,
}

impl Nvim {
    pub fn new() -> Nvim {
        let log_file = OpenOptions::new().append(true).open("my-file").unwrap();

        Nvim { log_file }
    }

    pub fn handle_events(&mut self) {
        let mut session = Session::new_parent().unwrap();
        let receiver = session.start_event_loop_channel();
        let mut nvim = Neovim::new(session);

        for (event, _values) in receiver {
            match Messages::from(event.clone()) {
                Messages::CurrentSong => {
                    let song = get_current_song();

                    self.log_file.write_all(&song[..]).unwrap();
                    nvim.command(&format!("echo \"{}\"", std::str::from_utf8(&song).unwrap()))
                        .unwrap();
                }
                _ => {
                    let msg = format!("Unexpected RPC: {}", event);
                    self.log_file.write_all(msg.as_bytes()).unwrap();
                }
            }
        }
    }
}

fn get_current_song() -> Vec<u8> {
    if !cfg!(target_os = "macos") {
        unimplemented!()
    }

    use std::process::Command;
    let cmd = "
tell application \"Spotify\"
    set currentArtist to artist of current track as string
    set currentTrack to name of current track as string

    return currentArtist & \" - \" & currentTrack
end tell
            ";

    let output = Command::new("osascript")
        .arg("-e")
        .arg(cmd)
        .output()
        .unwrap();

    output.stdout
}
