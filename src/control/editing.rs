use piston_window::{Input, Input::{Button, Move}, ButtonArgs, ButtonState::Release, Button::Keyboard,
                    Key, Motion, Motion::MouseRelative};
use crate::control::{Control, Mode, EditTarget, OscillatorTarget};
use rust_synth::core::control::synth::Command::SetPatch;
use rust_synth::core::control::tools::Command;
use rust_synth::core::synth::{oscillator, filter};
use rust_synth::core::synth::oscillator::Specs;

pub fn handle_input(input: &Input, window_size: [f64;2], control: &mut Control) -> Vec<Command>{
    match input {
        Button(args) => handle_button(args, control),
        Move(motion) => handle_mouse(motion, control, window_size),
        _ => vec![],
    }
}

fn handle_button(args: &ButtonArgs, control: &mut Control) -> Vec<Command> {
    match (args.state, args.button) {
        (Release, Keyboard(key)) => handle_key(key, control),
        _ => vec![],
    }
}

fn handle_mouse(motion: &Motion, control: &mut Control, window_size: [f64;2]) -> Vec<Command> {
    match motion {
        MouseRelative(x, y) =>  handle_move(*x, *y, control, window_size),
        _ => ()
    }
    update_specs(control)
}

fn handle_move(x: f64, y: f64, control: &mut Control, window_size: [f64;2]) {
    let [norm_x, norm_y] = normalized_mouse(x, y, window_size);
    let osc = &mut control.instrument.oscillator;
    use {EditTarget::*, OscillatorTarget::*};
    match control.mode {
        Mode::Editing(Some(Oscillator(Some(Pulse)))) => {
            match osc {
                Specs::Pulse(cycle) =>
                    change_f64(cycle, norm_x, 0., 1.),
                _ => {}
            }
        },
        Mode::Editing(Some(Oscillator(Some(Mix)))) => {
            match osc {
                Specs::Mix { nvoices: n, detune_amount: d, .. } => {
                    change_usize(n, norm_y, 1, 20);
                    change_f64(d, norm_x, 0.001, 2.)
                },
                _ => {}
            }
        },
        Mode::Playing => (),
        _ => (),
    }
}

fn normalized_mouse(x: f64, y: f64, window_size: [f64;2]) -> [f64; 2] {
    let normalized_x = 4. * x / window_size[0];
    let normalized_y = -4. * y / window_size[1];
    [normalized_x, normalized_y]
}

fn change_f64(reference: &mut f64, normalized: f64, min: f64, max: f64) {
    let range = max - min;
    let change = normalized * range + min;
    *reference = (*reference + change).max(min).min(max);
}

fn change_usize(reference: &mut usize, normalized: f64, min: usize, max: usize) {
    let range = (max - min) as f64;
    let change = (normalized * range + min as f64).floor() as usize;
    *reference = (*reference + change).max(min).min(max);
}

fn handle_key(key: Key, control: &mut Control) -> Vec<Command> {
    use EditTarget::*;
    match control.mode {
        Mode::Editing(None) => main_menu(key, control),
        Mode::Editing(Some(Oscillator(_))) => oscillator(key, control),
        Mode::Editing(Some(Filter)) => filter(key, control),
        Mode::Editing(Some(Arpeggiator)) => arpeggiator(key, control),
        _ => panic!(),
    }
}

fn main_menu(key: Key, control: &mut Control) -> Vec<Command> {
    match key {
        Key::Tab | Key::Escape => control.mode = Mode::Playing,
        Key::O => control.mode = Mode::Editing(Some(EditTarget::Oscillator(None))),
        Key::F => control.mode = Mode::Editing(Some(EditTarget::Filter)),
        Key::A => control.mode = Mode::Editing(Some(EditTarget::Arpeggiator)),
        _ => (),
    }
    vec![]
}

fn oscillator(key: Key, control: &mut Control) -> Vec<Command> {
    let mut set = |specs: oscillator::Specs, edit_target: Option<OscillatorTarget>| {
        control.instrument.oscillator = specs;
        control.mode =  Mode::Editing(Some(EditTarget::Oscillator(edit_target)));
        update_specs(control)
    };
    use oscillator::{Specs::*, Basic::*};
    match key {
        Key::Tab | Key::Escape => playing_mode(control),
        Key::D1 => set(Basic(Sine), None),
        Key::D2 => set(Basic(Saw), None),
        Key::D3 => set(Basic(Square), None),
        Key::D4 => set(Pulse(0.5), Some(OscillatorTarget::Pulse)),
        Key::D5 => set(Mix{ nvoices: 8, detune_amount: 3., specs: Saw }, Some(OscillatorTarget::Mix)),
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

fn update_specs(control: &Control) -> Vec<Command> {
    let command = Command::Instrument(SetPatch(control.instrument.clone()));
    vec![command]
}
