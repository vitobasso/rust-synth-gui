use piston_window::{OpenGL, PistonWindow, WindowSettings, Event::*, Input, TextureSettings, Glyphs, Loop::*};
use crate::{keymap, rendering};
use std::sync::mpsc::Sender;
use rust_synth::core::control::tools::Command;
use std::path::Path;

const TITLE: &str = "Sintetizador Maravilhoso";
const WINDOW_SIZE: [f64;2] = [800., 800.];

pub fn start(synth_commands: Option<Sender<Command>>) {
    let opengl = OpenGL::V3_2;

    let mut window: PistonWindow = WindowSettings::new(TITLE, WINDOW_SIZE)
        .opengl(opengl)
        .exit_on_esc(true)
        .build().unwrap();

    let font = Path::new("assets/fonts/VT323-Regular.ttf");
    let mut glyphs = Glyphs::new(font, window.factory.clone(), TextureSettings::new()).unwrap();

    while let Some(e) = window.next() {
        match &e {
            Input(input) => {
                handle_input(&synth_commands, input)
            },
            Loop(Render(_)) => {
                rendering::draw(&mut window, &mut glyphs, &e)
            }
            _ => (),
        }
    }
}

fn handle_input(synth_commands: &Option<Sender<Command>>, input: &Input) {
    if let Some(channel) = &synth_commands {
        let commands = keymap::handle_input(input, WINDOW_SIZE);
        for command in commands {
            channel.send(command).expect("Failed to send synth command")
        }
    }
}
