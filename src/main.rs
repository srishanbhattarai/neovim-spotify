extern crate neovim_lib;

use neovim_lib::Session;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() {
    let mut file = OpenOptions::new().append(true).open("my-file").unwrap();
    file.write_all(b"initializing").unwrap();

    let mut session = Session::new_parent().expect("Could not open session");

    let receiver = session.start_event_loop_channel();

    for (event, values) in receiver {
        let msg = format!("Event: {}, Values: {:#?}\n", event, values);

        file.write_all(msg.as_bytes()).unwrap();
    }
}
