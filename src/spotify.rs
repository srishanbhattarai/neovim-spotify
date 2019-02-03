pub trait SpotifyAPI {
    fn current_song(&self) -> String;
    fn play(&self);
    fn pause(&self);
    fn play_pause(&self);
    fn next(&self);
    fn previous(&self);
}

pub struct Spotify;

impl Spotify {
    pub fn new() -> impl SpotifyAPI {
        // TODO: Other platforms
        if !cfg!(target_os = "macos") {
            unimplemented!()
        }

        SpotifyOSX::new()
    }
}

pub struct SpotifyOSX;

impl SpotifyOSX {
    pub fn new() -> SpotifyOSX {
        SpotifyOSX {}
    }
}

impl SpotifyAPI for SpotifyOSX {
    // Retrieve current song.
    fn current_song(&self) -> String {
        let cmd = "
            tell application \"Spotify\"
    set currentArtist to artist of current track as string
    set currentTrack to name of current track as string

    return currentArtist & \" - \" & currentTrack
end tell
            ";

        run_osascript(cmd)
    }

    // Play if paused, and vice versa.
    fn play_pause(&self) {
        let cmd = "tell application \"Spotify\" to playpause";
        run_osascript(cmd);
    }

    // Pause if playing.
    fn pause(&self) {
        let cmd = "tell application \"Spotify\" to pause";
        run_osascript(cmd);
    }

    // Play if paused.
    fn play(&self) {
        let cmd = "tell application \"Spotify\" to play";
        run_osascript(cmd);
    }

    // Change to next track.
    fn next(&self) {
        let cmd = "tell application \"Spotify\" to next track";
        run_osascript(cmd);
    }

    // Change to next track.
    fn previous(&self) {
        let cmd = "tell application \"Spotify\" to previous track";
        run_osascript(cmd);
    }
}

// Run an AppleScript command.
fn run_osascript(script: &str) -> String {
    use std::process::Command;

    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .unwrap();

    std::str::from_utf8(&output.stdout[..]).unwrap().to_owned()
}
