use piston_window::{Button::*, ButtonArgs, ButtonState::*, Input, Input::*, Key, Motion, Motion::*};
use rust_synth::core::{
    control::{synth::{Command::*, Discriminator, id_discr},tools::Command::{self, *}},
    tools::{transposer::Command::*, loops::Command::*},
    music_theory::{pitch::Pitch, pitch_class::PitchClass::*},
};
use crate::control::{Control, Mode};

pub fn handle_input(input: &Input, window_size: [f64;2], control: &mut Control) -> Vec<Command> {
    match input {
        Button(args) => handle_button(args, control),
        Move(args) => handle_move(args, window_size),
        _ => vec![],
    }
}

fn handle_button(args: &ButtonArgs, control: &mut Control) -> Vec<Command> { //TODO Option<Command> ?
    match (args.state, args.button) {
        (Press, Keyboard(key))   =>
            note_on(key)
                .or_else(|| patches(key))
                .or_else(|| loop_rec(key))
                .or_else(|| tap_tempo(key))
                .or_else(|| transpose(key))
                .map_or(vec![], |v| vec![v]),
        (Release, Keyboard(key)) =>
            note_off(key)
                .or_else(|| mode(key, control))
                .map_or(vec![], |v| vec![v]),
        _ => vec![],
    }
}

fn handle_move(motion: &Motion, window_size: [f64;2]) -> Vec<Command> {
    match motion {
        MouseCursor(x, y) => {
            let norm_x = x / window_size[0] as f64;
            let norm_y = y / window_size[1] as f64;
            let command = Instrument(ModXY(norm_x, norm_y));
            vec![command]
        }
        _ => vec![],
    }
}

fn note_on(key: Key) -> Option<Command> {
    pitches(key).map(|(pitch, discr)|
        Instrument(NoteOn(pitch, 1., id_discr(pitch, discr))))
}

fn note_off(key: Key) -> Option<Command> {
    pitches(key).map(|(pitch, discr)|
        Instrument(NoteOff(id_discr(pitch, discr))))
}

fn pitches(key: Key) -> Option<(Pitch, Discriminator)> { //TODO shift => sharp pitches
    match key {
        //top row
        Key::Q =>         Some((Pitch::new(A, 4), 3)),
        Key::W =>         Some((Pitch::new(B, 4), 3)),
        Key::E =>         Some((Pitch::new(C, 5), 3)),
        Key::R =>         Some((Pitch::new(D, 5), 3)),
        Key::T =>         Some((Pitch::new(E, 5), 3)),
        Key::Y =>         Some((Pitch::new(F, 5), 3)),
        Key::U =>         Some((Pitch::new(G, 5), 3)),
        Key::I =>         Some((Pitch::new(A, 5), 3)),
        Key::O =>         Some((Pitch::new(B, 5), 3)),
        Key::P =>         Some((Pitch::new(C, 6), 3)),

        //middle row
        Key::A =>         Some((Pitch::new(A, 3), 2)),
        Key::S =>         Some((Pitch::new(B, 3), 2)),
        Key::D =>         Some((Pitch::new(C, 4), 2)),
        Key::F =>         Some((Pitch::new(D, 4), 2)),
        Key::G =>         Some((Pitch::new(E, 4), 2)),
        Key::H =>         Some((Pitch::new(F, 4), 2)),
        Key::J =>         Some((Pitch::new(G, 4), 2)),
        Key::K =>         Some((Pitch::new(A, 4), 2)),
        Key::L =>         Some((Pitch::new(B, 4), 2)),
        Key::Semicolon => Some((Pitch::new(C, 5), 2)),

        //bottom row
        Key::Z =>         Some((Pitch::new(A, 2), 1)),
        Key::X =>         Some((Pitch::new(B, 2), 1)),
        Key::C =>         Some((Pitch::new(C, 3), 1)),
        Key::V =>         Some((Pitch::new(D, 3), 1)),
        Key::B =>         Some((Pitch::new(E, 3), 1)),
        Key::N =>         Some((Pitch::new(F, 3), 1)),
        Key::M =>         Some((Pitch::new(G, 3), 1)),
        Key::Comma =>     Some((Pitch::new(A, 3), 1)),
        Key::Period =>    Some((Pitch::new(B, 3), 1)),
        Key::Slash =>     Some((Pitch::new(C, 4), 1)),

        _ => None,
    }
}

fn patches(key: Key) -> Option<Command> {
    match key {
        Key::D1 => Some(SetPatchNo(0)),
        Key::D2 => Some(SetPatchNo(1)),
        Key::D3 => Some(SetPatchNo(2)),
        Key::D4 => Some(SetPatchNo(3)),
        Key::D5 => Some(SetPatchNo(4)),
        Key::D6 => Some(SetPatchNo(5)),
        Key::D7 => Some(SetPatchNo(6)),
        Key::D8 => Some(SetPatchNo(7)),
        Key::D9 => Some(SetPatchNo(8)),
        Key::D0 => Some(SetPatchNo(9)),
        _ => None,
    }
}

fn loop_rec(key: Key) -> Option<Command> {
    match key {
        Key::F1 =>  Some(Loop(TogglePlayback(0))),
        Key::F2 =>  Some(Loop(TogglePlayback(1))),
        Key::F3 =>  Some(Loop(TogglePlayback(2))),
        Key::F4 =>  Some(Loop(TogglePlayback(3))),
        Key::F5 =>  Some(Loop(TogglePlayback(4))),
        Key::F6 =>  Some(Loop(ToggleRecording(0))),
        Key::F7 =>  Some(Loop(ToggleRecording(1))),
        Key::F8 =>  Some(Loop(ToggleRecording(2))),
        Key::F9 =>  Some(Loop(ToggleRecording(3))),
        Key::F10 => Some(Loop(ToggleRecording(4))),
        _ => None,
    }
}

fn tap_tempo(key: Key) -> Option<Command> {
    match key {
        Key::Space =>  Some(TapTempo),
        _ => None,
    }
}

fn transpose(key: Key) -> Option<Command> {
    match key {
        Key::Down =>         Some(Transposer(ShiftPitch(-12))),
        Key::Up =>           Some(Transposer(ShiftPitch(12))),
        Key::Left =>         Some(Transposer(ShiftKeyboard(-1))),
        Key::Right =>        Some(Transposer(ShiftKeyboard(1))),
        Key::Minus =>        Some(Transposer(ShiftPitch(-1))),
        Key::Equals =>       Some(Transposer(ShiftPitch(1))),
        Key::LeftBracket =>  Some(Transposer(TransposeKey(-1))),
        Key::RightBracket => Some(Transposer(TransposeKey(1))),
        _ => None,
    }
}

fn mode(key: Key, control: &mut Control) -> Option<Command> {
    if key == Key::Tab {
        control.mode = Mode::Editing(None);
    }
    None
}