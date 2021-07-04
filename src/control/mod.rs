use piston_window::Input;
use rust_synth::core::control::tools::Command;

mod playing;
mod editing;

pub enum EditTarget {
    Oscillator, Filter, Adsr, Lfo, Arpeggiator
}

enum Mode {
    Editing(Option<EditTarget>), Playing
}

pub struct Control {
    mode: Mode,
}

impl Control {

    pub fn new() -> Self {
        Self { mode: Mode::Playing }
    }

    pub fn handle_input(&mut self, input: &Input, window_size: [f64;2]) -> Vec<Command> {
        match self.mode {
            Mode::Editing(_) => editing::handle_input(&input, self),
            Mode::Playing => playing::handle_input(&input, window_size, self),
        }
    }

}

