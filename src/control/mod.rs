use piston_window::Input;
use rust_synth::core::control::tools::Command;

mod playing;
mod editing;

#[derive(Copy, Clone, Debug)]
pub enum EditTarget {
    Oscillator, Filter, Adsr, Lfo, Arpeggiator
}

enum Mode {
    Editing(EditTarget), Playing
}

pub struct Control {
    mode: Mode,
}

impl Control {

    pub fn new() -> Self {
        Self { mode: Mode::Playing }
    }

    pub fn handle_input(&self, input: &Input, window_size: [f64;2]) -> Vec<Command> {
        match self.mode {
            Mode::Editing(target) => editing::handle_input(&input, target),
            Mode::Playing => playing::handle_input(&input, window_size),
        }
    }

}

