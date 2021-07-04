use piston_window::{Input, Input::Button, ButtonArgs, ButtonState::Release, Button::Keyboard, Key};
use rust_synth::core::control::tools::Command;
use crate::control::{Control, Mode};

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
    if key == Key::Tab {
        control.mode = Mode::Playing;
    }
    vec![]
}