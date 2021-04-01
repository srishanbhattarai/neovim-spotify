use crate::lib;
#[cfg(target_os = "macos")]
pub(crate) struct AppleMusic;

#[cfg(target_os = "macos")]
impl AppleMusic {}

#[cfg(target_os = "macos")]
impl AppleMusic {
    pub(crate) fn new() -> Self {
        AppleMusic {}
    }

    // Retrieve current song. Returns (artist, song) tuple.
    pub(crate) fn current_song(&self) -> (String, String) {
        let cmd = "
        tell application \"Music\"
    set currentArtist to artist of current track as string
    set currentTrack to name of current track as string

    return currentArtist & \" - \" & currentTrack
end tell
            ";

        let result = lib::run_osascript(cmd);
        let mut parts = result.split('-');

        // The osascript above makes it so that the unwraps are ok
        let (artist, song) = (parts.next().unwrap().trim(), parts.next().unwrap().trim());

        return (artist.to_owned(), song.to_owned());
    }

    // Play if paused, and vice versa.
    pub(crate) fn play_pause(&self) {
        let cmd = "tell application \"Music\" to playpause";
        lib::run_osascript(cmd);
    }

    // Pause if playing.
    pub(crate) fn pause(&self) {
        let cmd = "tell application \"Music\" to pause";
        lib::run_osascript(cmd);
    }

    // Play if paused.
    pub(crate) fn play(&self) {
        let cmd = "tell application \"Music\" to play";
        lib::run_osascript(cmd);
    }

    // Change to next track.
    pub(crate) fn next(&self) {
        let cmd = "tell application \"Music\" to next track";
        lib::run_osascript(cmd);
    }

    // Change to next track.
    pub(crate) fn previous(&self) {
        let cmd = "tell application \"Music\" to previous track";
        lib::run_osascript(cmd);
    }
}
