use crate::lyrics;
use crate::spotify::Spotify;
use neovim_lib::{Neovim, NeovimApi, Session};

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
    spotify: Spotify,
}

impl EventHandler {
    pub fn new() -> EventHandler {
        // unwrap safe because new_parent always returns Ok
        let mut session = Session::new_parent().unwrap();
        session.set_infinity_timeout();
        let nvim = Neovim::new(session);
        let spotify = Spotify::new();

        EventHandler { nvim, spotify }
    }

    pub fn handle_events(&mut self) {
        let receiver = self.nvim.session.start_event_loop_channel();

        for (event, _values) in receiver {
            match Messages::from(event) {
                Messages::CurrentSong => {
                    let (artist, song) = self.spotify.current_song();

                    let song_name = format!("{} - {}", artist, song);

                    // commands should never fail when session spawned through parent
                    // if it does, it's probably best that it is fatal.
                    self.nvim
                        .command(&format!("echo \"{}\"", song_name))
                        .unwrap();
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
                    let (artist, song) = self.spotify.current_song();
                    let lyrics = lyrics::find_lyrics(&artist, &song);

                    match lyrics {
                        Some(lyrics) => {
                            let lyrics_vec = lyrics.split('\n').map(|s| s.to_owned()).collect();

                            // If the following commands cannot be executed with a parent
                            // neovim instance, it probably makes sense to die
                            self.nvim.command("vsplit lyrics.txt").unwrap();
                            let buf = self.nvim.get_current_buf().unwrap();
                            let buf_len = buf.line_count(&mut self.nvim).unwrap();
                            buf.set_lines(&mut self.nvim, 0, buf_len, true, lyrics_vec)
                                .unwrap();
                            self.nvim.command("setlocal nomodifiable").unwrap();
                        }
                        None => {
                            self.nvim
                                .command(&format!("echo \"Could not find lyrics\""))
                                .unwrap();
                        }
                    }
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
