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
