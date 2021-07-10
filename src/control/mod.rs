use piston_window::Input;
use rust_synth::core::control::tools::Command;
use rust_synth::core::synth::instrument;
use rust_synth::core::tools::arpeggiator;

mod playing;
mod editing;

#[derive(Copy, Clone, Debug)]
pub enum Mode {
    Editing(Option<EditTarget>), Playing
}

pub struct Control {
    pub mode: Mode,
    instrument: instrument::Specs,
    arpeggiator: Option<arpeggiator::Specs>,
}

impl Control {

    pub fn new() -> Self {
        Self { mode: Mode::Playing, instrument: Default::default(), arpeggiator: None }
    }

    pub fn handle_input(&mut self, input: &Input, window_size: [f64;2]) -> Vec<Command> {
        match self.mode {
            Mode::Editing(_) => editing::handle_input(&input, window_size, self),
            Mode::Playing => playing::handle_input(&input, window_size, self),
        }
    }

}

#[derive(Copy, Clone, Debug)]
pub enum EditTarget {
    Oscillator(Option<OscillatorTarget>),
    Filter,
    Arpeggiator,
}

#[derive(Copy, Clone, Debug)]
pub enum OscillatorTarget {
    Pulse, Mix
}
