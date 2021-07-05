use piston_window::{Input, Input::{Button, Move}, ButtonArgs, ButtonState::Release, Button::Keyboard,
                    Key, Motion, Motion::MouseRelative};
use crate::control::{Control, Mode, EditTarget};
use rust_synth::core::control::synth::Command::SetPatch;
use rust_synth::core::control::tools::Command;
use rust_synth::core::synth::{oscillator, filter};

pub fn handle_input(input: &Input, window_size: [f64;2], control: &mut Control) -> Vec<Command>{
    match input {
        Button(args) => handle_button(args, control),
        Move(motion) => handle_move(motion, control, window_size),
        _ => vec![],
    }
}

fn handle_button(args: &ButtonArgs, control: &mut Control) -> Vec<Command> {
    match (args.state, args.button) {
        (Release, Keyboard(key)) => handle_key(key, control),
        _ => vec![],
    }
}

fn handle_move(motion: &Motion, control: &mut Control, window_size: [f64;2]) -> Vec<Command> {
    match motion {
        MouseRelative(x, y) =>  {
            let _norm_x = 4. * x / window_size[0];
            let norm_y = 4. * y / window_size[1];
            let previous = control.instrument.volume;
            let new = (previous - norm_y).max(0.).min(1.);
            control.instrument.volume = new;
            update_specs(control)
        },
        _ => vec![]
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
        update_specs(control)
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
        update_specs(control)
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

fn update_specs(control: &mut Control) -> Vec<Command> {
    let command = Command::Instrument(SetPatch(control.instrument.clone()));
    vec![command]
}
