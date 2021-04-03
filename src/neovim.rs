use crate::backends::{AppleMusic, Spotify};
use crate::lyrics;
use neovim_lib::{Neovim, NeovimApi, Session};

enum Messages {
    SpotifyLyrics,
    SpotifyCurrentSong,
    SpotifyPlayPause,
    SpotifyPlay,
    SpotifyPause,
    SpotifyNext,
    SpotifyPrevious,
    AppleMusicLyrics,
    AppleMusicCurrentSong,
    AppleMusicPlayPause,
    AppleMusicPlay,
    AppleMusicPause,
    AppleMusicNext,
    AppleMusicPrevious,
    Unknown(String),
}

impl From<String> for Messages {
    fn from(event: String) -> Self {
        match &event[..] {
            "applemusic_current_song" => Messages::AppleMusicCurrentSong,
            "applemusic_play_pause" => Messages::AppleMusicPlayPause,
            "applemusic_play" => Messages::AppleMusicPlay,
            "applemusic_pause" => Messages::AppleMusicPause,
            "applemusic_next" => Messages::AppleMusicNext,
            "applemusic_previous" => Messages::AppleMusicPrevious,
            "applemusic_lyrics" => Messages::AppleMusicLyrics,
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
                Messages::SpotifyCurrentSong => self.echo_curr_song(self.spotify.current_song()),
                Messages::SpotifyPlayPause => self.spotify.play_pause(),
                Messages::SpotifyPlay => self.spotify.play(),
                Messages::SpotifyPause => self.spotify.pause(),
                Messages::SpotifyNext => self.spotify.next(),
                Messages::SpotifyPrevious => self.spotify.previous(),
                Messages::SpotifyLyrics => self.echo_lyrics(self.spotify.current_song()),

                Messages::AppleMusicCurrentSong => self.echo_curr_song(self.applemusic.current_song()),
                Messages::AppleMusicPlayPause => self.applemusic.play_pause(),
                Messages::AppleMusicPlay => self.applemusic.play(),
                Messages::AppleMusicPause => self.applemusic.pause(),
                Messages::AppleMusicNext => self.applemusic.next(),
                Messages::AppleMusicPrevious => self.applemusic.previous(),
                Messages::AppleMusicLyrics => self.echo_lyrics(self.applemusic.current_song()),

                Messages::Unknown(ev) => {
                    self.nvim
                        .command(&format!("echoerr \"{}\" Unknown command", ev))
                        .unwrap();
                }
            }
        }
    }

    // helper function to send current song info to nvim instance.
    fn echo_curr_song(&mut self, artist_song: (String, String)) {
        let song_name = format!("{} - {}", &*artist_song.0, &*artist_song.1);

        // commands should never fail when session spawned through parent
        // if it does, it's probably best that it is fatal.
        self.nvim
            .command(&format!("echo \"{}\"", song_name))
            .unwrap();
    }

    // helper function to find lyrics and send result to nvim instance.
    fn echo_lyrics(&mut self, artist_song: (String, String)) {
        let lyrics = lyrics::find_lyrics(&*artist_song.0, &*artist_song.1);

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
            }
            None => {
                self.nvim
                    .command("echo \"Could not find lyrics\"")
                    .unwrap();
            }
        }
    }
}
