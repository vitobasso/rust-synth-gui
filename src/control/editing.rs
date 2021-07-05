use piston_window::{Input, Input::Button, ButtonArgs, ButtonState::Release, Button::Keyboard, Key};
use crate::control::{Control, Mode, EditTarget};
use rust_synth::core::control::synth::Command::SetPatch;
use rust_synth::core::control::tools::Command;
use rust_synth::core::synth::{oscillator, filter};

pub fn handle_input(input: &Input, control: &mut Control) -> Vec<Command>{
    match input {
        Button(args) => handle_button(args, control),
        _ => vec![],
    }
}

fn handle_button(args: &ButtonArgs, control: &mut Control) -> Vec<Command> {
    match (args.state, args.button) {
        (Release, Keyboard(key)) => handle_key(key, control),
        _ => vec![],
    }
}

fn handle_key(key: Key, control: &mut Control) -> Vec<Command> {
    match control.mode {
        Mode::Editing(None) => main_menu(key, control),
        Mode::Editing(Some(EditTarget::Oscillator)) => oscillator(key, control),
        Mode::Editing(Some(EditTarget::Filter)) => filter(key, control),
        Mode::Editing(Some(EditTarget::Adsr)) => adsr(key, control),
        Mode::Editing(Some(EditTarget::Lfo)) => lfo(key, control),
        Mode::Editing(Some(EditTarget::Arpeggiator)) => arpeggiator(key, control),
        _ => panic!(),
    }

}

fn main_menu(key: Key, control: &mut Control) -> Vec<Command> {
    match key {
        Key::Tab | Key::Escape => control.mode = Mode::Playing,
        Key::O => control.mode = Mode::Editing(Some(EditTarget::Oscillator)),
        Key::F => control.mode = Mode::Editing(Some(EditTarget::Filter)),
        Key::E => control.mode = Mode::Editing(Some(EditTarget::Adsr)),
        Key::L => control.mode = Mode::Editing(Some(EditTarget::Lfo)),
        Key::A => control.mode = Mode::Editing(Some(EditTarget::Arpeggiator)),
        _ => (),
    }
    vec![]
}

fn oscillator(key: Key, control: &mut Control) -> Vec<Command> {
    let mut set = |specs: oscillator::Specs| {
        control.mode = Mode::Editing(None);
        control.instrument.oscillator = specs;
        let command = Command::Instrument(SetPatch(control.instrument.clone()));
        vec![command]
    };
    use oscillator::Specs::*;
    match key {
        Key::Tab | Key::Escape => playing_mode(control),
        Key::D1 => set(Sine),
        Key::D2 => set(Saw),
        Key::D3 => set(Square),
        Key::D4 => set(Pulse(0.5)),
        Key::D5 => set(Mix{ nvoices: 8, detune_amount: 3., specs: Box::new(Saw) }),
        _ => vec![],
    }
}

fn filter(key: Key, control: &mut Control) -> Vec<Command> {
    let mut set = |specs: filter::Specs| {
        control.mode = Mode::Editing(None);
        control.instrument.filter = specs;
        let command = Command::Instrument(SetPatch(control.instrument.clone()));
        vec![command]
    };
    use filter::Specs::*;
    match key {
        Key::Tab | Key::Escape => playing_mode(control),
        Key::D1 => set(LPF),
        Key::D2 => set(HPF),
        Key::D3 => set(BPF),
        Key::D4 => set(Notch),
        _ => vec![],
    }
}

fn adsr(key: Key, control: &mut Control) -> Vec<Command> {
    match key {
        Key::Tab | Key::Escape => playing_mode(control),
        _ => vec![],
    }
}

fn lfo(key: Key, control: &mut Control) -> Vec<Command> {
    match key {
        Key::Tab | Key::Escape => playing_mode(control),
        _ => vec![],
    }
}

fn arpeggiator(key: Key, control: &mut Control) -> Vec<Command> {
    match key {
        Key::Tab | Key::Escape => playing_mode(control),
        _ => vec![],
    }
}

fn playing_mode(control: &mut Control) -> Vec<Command> {
    control.mode = Mode::Playing;
    vec![]
}
