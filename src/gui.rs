use std::sync::mpsc::Sender;

use piston_window::{OpenGL, PistonWindow, WindowSettings, Event::*, Input, Loop::*};
use rust_synth::core::control::tools::Command;
use crate::{keymap, rendering::Rendering, widgets::Widgets};

const TITLE: &str = "Sintetizador Maravilhoso";
const WINDOW_SIZE: [f64;2] = [800., 800.];

pub fn start(synth_channel: Option<Sender<Command>>) {

    let [width, height] = WINDOW_SIZE;

    let mut window: PistonWindow = WindowSettings::new(TITLE, WINDOW_SIZE)
        .graphics_api(OpenGL::V3_2)
        .exit_on_esc(true)
        .build().unwrap();

    let font_path: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/fonts/VT323-Regular.ttf");
    let mut ui = conrod_core::UiBuilder::new(WINDOW_SIZE).build();
    ui.fonts.insert_from_file(font_path).unwrap();

    let widgets = Widgets::new(&mut ui);
    let mut rendering = Rendering::new(&mut window, width as u32, height as u32);

    while let Some(event) = window.next() {
        match &event {
            Input(input, _) => {
                send_synth_commands(&synth_channel, input);
                if let Some(conrod_event) = conrod_piston::event::convert(event.clone(), width, height) {
                    ui.handle_event(conrod_event)
                }
            },
            Loop(Update(_)) => {
                let ui = &mut ui.set_widgets();
                widgets.update(ui);
            },
            Loop(Render(_)) => {
                window.draw_2d(&event, |c, g, d| {
                    if let Some(primitives) = ui.draw_if_changed() {
                        rendering.draw(primitives, c, g, d);
                    }
                });
            },
            _ => (),
        }
    }
}

fn send_synth_commands(synth_channel: &Option<Sender<Command>>, input: &Input) {
    if let Some(channel) = &synth_channel {
        let commands = keymap::handle_input(input, WINDOW_SIZE);
        for command in commands {
            channel.send(command).expect("Failed to send synth command")
        }
    }
}
