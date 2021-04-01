use crate::applemusic::AppleMusic;
use crate::lyrics;
use crate::spotify::Spotify;
use neovim_lib::{Neovim, NeovimApi, Session};

enum Messages {
    SpotifyLyrics,
    SpotifyCurrentSong,
    SpotifyPlayPause,
    SpotifyPlay,
    SpotifyPause,
    SpotifyNext,
    SpotifyPrevious,
    MusicLyrics,
    MusicCurrentSong,
    MusicPlayPause,
    MusicPlay,
    MusicPause,
    MusicNext,
    MusicPrevious,
    Unknown(String),
}

impl From<String> for Messages {
    fn from(event: String) -> Self {
        match &event[..] {
            "applemusic_current_song" => Messages::MusicCurrentSong,
            "applemusic_play_pause" => Messages::MusicPlayPause,
            "applemusic_play" => Messages::MusicPlay,
            "applemusic_pause" => Messages::MusicPause,
            "applemusic_next" => Messages::MusicNext,
            "applemusic_previous" => Messages::MusicPrevious,
            "applemusic_lyrics" => Messages::MusicLyrics,
            "spotify_current_song" => Messages::SpotifyCurrentSong,
            "spotify_play_pause" => Messages::SpotifyPlayPause,
            "spotify_play" => Messages::SpotifyPlay,
            "spotify_pause" => Messages::SpotifyPause,
            "spotify_next" => Messages::SpotifyNext,
            "spotify_previous" => Messages::SpotifyPrevious,
            "spotify_lyrics" => Messages::SpotifyLyrics,
            _ => Messages::Unknown(event),
        }
    }
}

/// EventHandler receives RPC requests, and maps them to right Spotify and Neovim commands.
pub struct EventHandler {
    nvim: Neovim,
    spotify: Spotify,
    applemusic: AppleMusic,
}

impl EventHandler {
    pub fn new() -> EventHandler {
        // unwrap safe because new_parent always returns Ok
        let mut session = Session::new_parent().unwrap();
        session.set_infinity_timeout();
        let nvim = Neovim::new(session);
        let spotify = Spotify::new();
        let applemusic = AppleMusic::new();

        EventHandler {
            nvim,
            spotify,
            applemusic,
        }
    }

    pub fn handle_events(&mut self) {
        let receiver = self.nvim.session.start_event_loop_channel();

        for (event, _values) in receiver {
            match Messages::from(event) {
                Messages::SpotifyCurrentSong => {
                    let (artist, song) = self.spotify.current_song();

                    let song_name = format!("{} - {}", artist, song);

                    // commands should never fail when session spawned through parent
                    // if it does, it's probably best that it is fatal.
                    self.nvim
                        .command(&format!("echo \"{}\"", song_name))
                        .unwrap();
                }

                Messages::SpotifyPlayPause => {
                    self.spotify.play_pause();
                }

                Messages::SpotifyPlay => {
                    self.spotify.play();
                }

                Messages::SpotifyPause => {
                    self.spotify.pause();
                }

                Messages::SpotifyNext => {
                    self.spotify.next();
                }

                Messages::SpotifyPrevious => {
                    self.spotify.previous();
                }

                Messages::SpotifyLyrics => {
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

                Messages::MusicCurrentSong => {
                    let (artist, song) = self.applemusic.current_song();

                    let song_name = format!("{} - {}", artist, song);

                    // commands should never fail when session spawned through parent
                    // if it does, it's probably best that it is fatal.
                    self.nvim
                        .command(&format!("echo \"{}\"", song_name))
                        .unwrap();
                }

                Messages::MusicPlayPause => {
                    self.applemusic.play_pause();
                }

                Messages::MusicPlay => {
                    self.applemusic.play();
                }

                Messages::MusicPause => {
                    self.applemusic.pause();
                }

                Messages::MusicNext => {
                    self.applemusic.next();
                }

                Messages::MusicPrevious => {
                    self.applemusic.previous();
                }

                Messages::MusicLyrics => {
                    let (artist, song) = self.applemusic.current_song();
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
