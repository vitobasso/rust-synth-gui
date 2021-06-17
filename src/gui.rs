use piston_window::{OpenGL, PistonWindow, WindowSettings, Event::*};
use crate::keymap;
use std::sync::mpsc::Sender;
use rust_synth::core::control::manual_controller::Command;

const TITLE: &str = "Sintetizador Maravilhoso";
const WINDOW_SIZE: [f64;2] = [800., 800.];

pub fn render(synth_commands: Sender<Command>) {
    let opengl = OpenGL::V3_2;

    let mut window: PistonWindow = WindowSettings::new(TITLE, WINDOW_SIZE)
        .opengl(opengl)
        .exit_on_esc(true)
        .build().unwrap();

    while let Some(e) = window.next() {
        match &e {
            Input(input) => {
                let commands = keymap::handle_input(input, WINDOW_SIZE);
                for command in commands {
                    synth_commands.send(command).expect("Failed to send synth command")
                }
            },
            _ => (),
        }
    }
}
