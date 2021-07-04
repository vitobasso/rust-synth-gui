use std::path::Path;
use std::sync::mpsc::{Receiver, Sender};

use piston_window::{Event::*, Glyphs, Loop::*, OpenGL, PistonWindow, TextureSettings, WindowSettings};
use rust_synth::core::control::tools::{Command, View};

use crate::control::Control;
use crate::rendering;

const TITLE: &str = "Sintetizador Maravilhoso";
const WINDOW_SIZE: [f64;2] = [800., 800.];

pub fn start(channels: Option<(Sender<Command>, Receiver<View>)>) {
    let opengl = OpenGL::V3_2;

    let mut window: PistonWindow = WindowSettings::new(TITLE, WINDOW_SIZE)
        .opengl(opengl)
        .exit_on_esc(true)
        .build().unwrap();

    let font = Path::new("assets/fonts/VT323-Regular.ttf");
    let mut glyphs = Glyphs::new(font, window.factory.clone(), TextureSettings::new()).unwrap();

    if let Some((commands_out, view_in)) = channels {
        manual_loop(&mut window, &mut glyphs, commands_out, view_in);
    } else {
        midi_loop(&mut window);
    }
}

fn manual_loop(window: &mut PistonWindow, glyphs: &mut Glyphs, commands_out: Sender<Command>, view_in: Receiver<View>) {
    let control = Control::new();
    while let Some(e) = window.next() {
        match &e {
            Input(input) => {
                for command in control.handle_input(input, WINDOW_SIZE) {
                    commands_out.send(command).expect("Failed to send synth command")
                }
            },
            Loop(Render(_)) => {
                if let Ok(view) = view_in.try_recv() {
                    rendering::draw(view, window, glyphs, &e)
                }
            }
            _ => (),
        }
    }
}

fn midi_loop(window: &mut PistonWindow) {
    while let Some(_) = window.next() {}
}
