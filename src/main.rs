use std::env::{args, Args};

use rust_synth::io;

mod gui;
mod control;
mod rendering;

fn main() {
    match midi_file_argument() {
        Some(file_name) => {
            io::start_midi(&file_name);
            gui::start(None);
        },
        None => {
            let channels = io::start_manual();
            gui::start(Some(channels));
        },
    }
}

fn midi_file_argument() -> Option<String> {
    let mut args: Args = args();
    args.next();
    args.next()
}
