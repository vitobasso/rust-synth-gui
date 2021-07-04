use piston_window::{Input, Input::Button, ButtonArgs, ButtonState::Release, Button::Keyboard, Key};
use rust_synth::core::control::tools::Command;
use crate::control::{Control, Mode, EditTarget};

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
        Mode::Editing(None) => handle_from_main(key, control),
        Mode::Editing(Some(EditTarget::Oscillator)) => handle_from_oscillator(key, control),
        Mode::Editing(Some(EditTarget::Filter)) => handle_from_filter(key, control),
        Mode::Editing(Some(EditTarget::Adsr)) => handle_from_adsr(key, control),
        Mode::Editing(Some(EditTarget::Lfo)) => handle_from_lfo(key, control),
        Mode::Editing(Some(EditTarget::Arpeggiator)) => handle_from_arpeggiator(key, control),
        _ => panic!(),
    }

}

fn handle_from_main(key: Key, control: &mut Control) -> Vec<Command> {
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

fn handle_from_oscillator(key: Key, control: &mut Control) -> Vec<Command> {
    match key {
        Key::Tab | Key::Escape => control.mode = Mode::Playing,
        _ => (),
    }
    vec![]
}

fn handle_from_filter(key: Key, control: &mut Control) -> Vec<Command> {
    match key {
        Key::Tab | Key::Escape => control.mode = Mode::Playing,
        _ => (),
    }
    vec![]
}

fn handle_from_adsr(key: Key, control: &mut Control) -> Vec<Command> {
    match key {
        Key::Tab | Key::Escape => control.mode = Mode::Playing,
        _ => (),
    }
    vec![]
}

fn handle_from_lfo(key: Key, control: &mut Control) -> Vec<Command> {
    match key {
        Key::Tab | Key::Escape => control.mode = Mode::Playing,
        _ => (),
    }
    vec![]
}

fn handle_from_arpeggiator(key: Key, control: &mut Control) -> Vec<Command> {
    match key {
        Key::Tab | Key::Escape => control.mode = Mode::Playing,
        _ => (),
    }
    vec![]
}