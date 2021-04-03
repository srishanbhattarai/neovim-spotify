#[cfg(target_os = "macos")]
pub(crate) fn run_osascript(script: &str) -> String {
    use std::process::Command;

    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .expect("Could not run osascript cmd");

    std::str::from_utf8(&output.stdout[..])
        .expect("Could not obtain stdout from osascript output")
        .to_owned()
}

macro_rules! osascript_impl {
    ($T:ty, $V:literal) => {
        #[cfg(target_os = "macos")]
        impl $T {
            pub(crate) fn new() -> Self {
                Self {}
            }

            // Retrieve current song. Returns (artist, song) tuple.
            pub(crate) fn current_song(&self) -> (String, String) {
                let cmd = format!("
        tell application \"{}\"
    set currentArtist to artist of current track as string
    set currentTrack to name of current track as string

    return currentArtist & \" - \" & currentTrack
end tell
            ", $V);

                let result = run_osascript(&cmd);
                let mut parts = result.split('-');

                // The osascript above makes it so that the unwraps are ok
                let (artist, song) = (parts.next().unwrap().trim(), parts.next().unwrap().trim());

                return (artist.to_owned(), song.to_owned());
            }

            // Play if paused, and vice versa.
            pub(crate) fn play_pause(&self) {
                let cmd = format!("tell application \"{}\" to playpause", $V);
                run_osascript(&cmd);
            }

            // Pause if playing.
            pub(crate) fn pause(&self) {
                let cmd = format!("tell application \"{}\" to pause", $V);
                run_osascript(&cmd);
            }

            // Play if paused.
            pub(crate) fn play(&self) {
                let cmd = format!("tell application \"{}\" to play", $V);
                run_osascript(&cmd);
            }

            // Change to next track.
            pub(crate) fn next(&self) {
                let cmd = format!("tell application \"{}\" to next track", $V);
                run_osascript(&cmd);
            }

            // Change to next track.
            pub(crate) fn previous(&self) {
                let cmd = format!("tell application \"{}\" to previous track", $V);
                run_osascript(&cmd);
            }
        }
    };
}

pub(crate) struct Spotify;
pub(crate) struct AppleMusic;

osascript_impl!(Spotify, "Spotify");
osascript_impl!(AppleMusic, "Music");
