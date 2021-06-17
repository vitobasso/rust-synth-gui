use std::{thread, sync::mpsc::{channel, sync_channel}};
use rust_synth::core::{synth::Sample, control::{manual_controller::{self, Command}, playback_controller}};
use rust_synth::io::{audio, midi};
use rust_synth::preset;

mod gui;
mod keymap;

fn main() {
    let out = audio::Out::initialize().unwrap_or_else(|e| panic!("{}", e));
    let sample_rate = out.sample_rate();

    let (command_out, command_in) = channel::<Command>();
    let (sound_out, sound_in) = sync_channel::<Sample>(out.buffer_size());

    thread::spawn(move || out.loop_forever(sound_in));
    match read_midi_file() {
        Some(song) => thread::spawn(move || playback_controller::loop_forever(sample_rate, song, sound_out)),
        None       => thread::spawn(move || manual_controller::loop_forever(sample_rate, preset::patches(), command_in, sound_out)),
    };

    gui::render(command_out);
}

use std::env::{args,Args};
use rust_synth::core::control::song::Song;
fn read_midi_file() -> Option<Song> {
    let mut args: Args = args();
    args.next();
    args.next().and_then(|file_name| midi::read_file(file_name.as_str()))
}
