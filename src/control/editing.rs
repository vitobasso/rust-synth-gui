use piston_window::{Input, Input::{Button, Move}, ButtonArgs, ButtonState::*, Button::Keyboard, Key,
                    Motion, Motion::MouseCursor, ButtonState};
use crate::control::{Control, Mode, EditTarget, OscillatorTarget};
use rust_synth::core::control::synth::Command::SetPatch;
use rust_synth::core::control::tools::{Command, Patch};
use rust_synth::core::synth::{oscillator, filter};
use rust_synth::core::synth::oscillator::Specs;
use rust_synth::core::tools::arpeggiator;

pub fn handle_input(input: &Input, window_size: [f64;2], control: &mut Control) -> Vec<Command>{
    match input {
        Button(args) => handle_button(args, control),
        Move(motion) => handle_mouse(motion, control, window_size),
        _ => vec![],
    }
}

fn handle_button(args: &ButtonArgs, control: &mut Control) -> Vec<Command> {
    match (args.state, args.button) {
        (_, Keyboard(Key::Space)) => handle_spacebar(args.state),
        (Release, Keyboard(key)) => handle_key(key, control),
        _ => vec![],
    }
}

fn handle_spacebar(state: ButtonState) -> Vec<Command> {
    use rust_synth::core::{
        control::synth::{Command::{NoteOn, NoteOff}, id_discr},
        control::tools::Command::Instrument,
        music_theory::pitch::Pitch,
    };
    let pitch = Pitch::default();
    let id = id_discr(pitch, 0);
    match state {
        Press => vec![Instrument(NoteOn(pitch, 1., id))],
        Release => vec![Instrument(NoteOff(id))],
    }
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
    let mut set = |spec: filter::TypeSpec| {
        control.instrument.filter = filter::Specs{ filter_type: spec, .. Default::default() };
        update_specs(control)
    };
    use filter::TypeSpec::*;
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
    use arpeggiator::builder::{Chord, Direction};
    use rust_synth::core::music_theory::diatonic_scale::{self, OctaveShift::{Down1, Up1}};

    let mut set = |chord: Chord, direction: Direction| {
        let old = control.arpeggiator.clone().unwrap_or_else(|| arpeggiator::Specs::default());
        let specs = arpeggiator::Specs {
            key: diatonic_scale::Key::C,
            phrase: arpeggiator::builder::Specs {
                chord, direction, octave_min: Down1, octave_max: Up1, ..old.phrase
            },
            .. old
        };
        control.arpeggiator = Some(specs);
        update_specs(control)
    };

    match key {
        Key::Tab | Key::Escape => playing_mode(control),
        Key::D1 => set(Chord::Octaves, Direction::Up),
        Key::D2 => set(Chord::Octaves, Direction::Down),
        Key::D3 => set(Chord::Octaves, Direction::UpDown),
        Key::D4 => set(Chord::Triad, Direction::Up),
        Key::D5 => set(Chord::Triad, Direction::Down),
        Key::D6 => set(Chord::Triad, Direction::UpDown),
        Key::D7 => set(Chord::Fantasy, Direction::UpDown),
        Key::D8 => set(Chord::Tetra, Direction::Up),
        Key::D9 => set(Chord::Penta, Direction::Up),
        Key::D0 => {
            control.arpeggiator = None;
            update_specs(control)
        },
        _ => vec![],
    }
}

fn handle_mouse(motion: &Motion, control: &mut Control, window_size: [f64;2]) -> Vec<Command> {
    match motion {
        MouseCursor(x, y) =>  handle_move(*x, *y, control, window_size),
        _ => ()
    }
    update_specs(control)
}

fn handle_move(x: f64, y: f64, control: &mut Control, window_size: [f64;2]) {
    let [norm_x, norm_y] = normalized_mouse(x, y, window_size);
    use {EditTarget::*, OscillatorTarget::*};
    match control.mode {
        Mode::Editing(Some(Oscillator(Some(Pulse)))) =>
            match &mut control.instrument.oscillator {
                Specs::Pulse(cycle) =>
                    change_f64(cycle, norm_x, 0., 1.),
                _ => {}
            },
        Mode::Editing(Some(Oscillator(Some(Mix)))) =>
            match &mut control.instrument.oscillator {
                Specs::Mix { nvoices: n, detune_amount: d, .. } => {
                    change_usize(n, norm_y, 1, 40);
                    change_f64(d, norm_x, 0.001, 32.);
                },
                _ => {}
            },
        Mode::Editing(Some(Filter)) => {
            change_f64(&mut control.instrument.filter.cutoff, norm_y, 0., 1.);
            change_f64(&mut control.instrument.filter.resonance, norm_x, 0., 1.);
        },
        _ => (),
    }
}

fn normalized_mouse(x: f64, y: f64, window_size: [f64;2]) -> [f64; 2] {
    let normalized_x = x / window_size[0];
    let normalized_y = 1. - (y / window_size[1]);
    [normalized_x.min(1.).max(0.),
        normalized_y.min(1.).max(0.)]
}

fn change_f64(reference: &mut f64, normalized: f64, min: f64, max: f64) {
    let scaled = normalized * (max - min) + min;
    *reference = scaled;
}

fn change_usize(reference: &mut usize, normalized: f64, min: usize, max: usize) {
    let range = (max - min) as f64;
    let scaled = (normalized * range).floor() as usize + min;
    *reference = scaled;
}

fn playing_mode(control: &mut Control) -> Vec<Command> {
    control.mode = Mode::Playing;
    vec![]
}

fn update_specs(control: &Control) -> Vec<Command> {
    vec![Command::Instrument(SetPatch(control.instrument.clone())),
         Command::SetPatch(Patch::Arpeggiator(control.arpeggiator.clone()))]
}
