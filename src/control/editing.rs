use piston_window::{Input, Input::{Button, Move}, ButtonArgs, ButtonState::*, Button::Keyboard, Key,
                    Motion, Motion::MouseCursor, ButtonState};
use crate::control::{Control, Mode, EditTarget, OscillatorTarget};
use rust_synth::core::control::synth::Command::SetPatch;
use rust_synth::core::control::tools::{Command, Patch};
use rust_synth::core::synth::{oscillator, filter};
use rust_synth::core::synth::oscillator::Specs;
use rust_synth::core::tools::arpeggiator;
use rust_synth::core::music_theory::diatonic_scale::OctaveShift;

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
        Key::D1 => set(Basic(Sine), None),
        Key::D2 => set(Basic(Saw), None),
        Key::D3 => set(Basic(Square), None),
        Key::D4 => set(Pulse(0.5), Some(OscillatorTarget::Pulse)),
        Key::D5 => set(Mix{ nvoices: 8, detune_amount: 3., specs: Saw }, Some(OscillatorTarget::Mix)),
        _ => main_menu(key,control),
    }
}

fn filter(key: Key, control: &mut Control) -> Vec<Command> {
    let mut set = |spec: filter::TypeSpec| {
        control.instrument.filter = filter::Specs{ filter_type: spec, .. Default::default() };
        update_specs(control)
    };
    use filter::TypeSpec::*;
    match key {
        Key::D1 => set(LPF),
        Key::D2 => set(HPF),
        Key::D3 => set(BPF),
        Key::D4 => set(Notch),
        _ => main_menu(key,control),
    }
}

fn arpeggiator(key: Key, control: &mut Control) -> Vec<Command> {
    use arpeggiator::builder::{Chord, Direction};
    use rust_synth::core::music_theory::diatonic_scale;
    use arpeggiator::builder::Specs;

    let mut set = |f: fn(Specs) -> Specs| {
        let old = control.arpeggiator.as_ref().map(|a| a.phrase.clone()).unwrap_or_else(|| Specs::default());
        let specs = arpeggiator::Specs {
            key: diatonic_scale::Key::C,
            phrase: f(old),
        };
        control.arpeggiator = Some(specs);
        update_specs(control)
    };

    match key {
        Key::D1 => set(|old| Specs { chord: Chord::Octaves, ..old }),
        Key::D2 => set(|old| Specs { chord: Chord::Triad, ..old }),
        Key::D3 => set(|old| Specs { chord: Chord::Fantasy, ..old }),
        Key::D4 => set(|old| Specs { chord: Chord::Tetra, ..old }),
        Key::D5 => set(|old| Specs { chord: Chord::Penta, ..old }),
        Key::F1 => set(|old| Specs { direction: Direction::Up, ..old }),
        Key::F2 => set(|old| Specs { direction: Direction::Down, ..old }),
        Key::F3 => set(|old| Specs { direction: Direction::UpDown, ..old }),
        Key::D0 => {
            control.arpeggiator = None;
            update_specs(control)
        },
        _ => main_menu(key,control),
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
        Mode::Editing(Some(Arpeggiator)) => {
            use arpeggiator::builder::Specs;
            match &mut control.arpeggiator {
                Some(arp) => {
                    let old = &arp.phrase;
                    let (octave_min, octave_max) = octaves_from_mouse(norm_x, norm_y);
                    arp.phrase = Specs { octave_min, octave_max, ..old.clone() };
                }
                _ => {}
            }
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

fn octaves_from_mouse(x: f64, y: f64) -> (OctaveShift, OctaveShift) {
    let max_range = OctaveShift::Up3 as i8 - OctaveShift::Down3 as i8;
    let range = (x * max_range as f64).floor() as i8;
    let min_offset = OctaveShift::Down3 as i8;
    let max_offset = OctaveShift::Up3 as i8;
    let offset = (y * max_range as f64 + min_offset as f64).floor() as i8;
    let bottom_octave = OctaveShift::from_i8(offset)
        .unwrap_or_else(|| panic!("Can't get OctaveShift from {}", offset));
    let top_octave_i8 = (offset + range).min(max_offset);
    let top_octave = OctaveShift::from_i8(top_octave_i8)
        .unwrap_or_else(|| panic!("Can't get OctaveShift from {}", top_octave_i8));
    (bottom_octave, top_octave)
}

fn update_specs(control: &Control) -> Vec<Command> {
    vec![Command::Instrument(SetPatch(control.instrument.clone())),
         Command::SetPatch(Patch::Arpeggiator(control.arpeggiator.clone()))]
}
