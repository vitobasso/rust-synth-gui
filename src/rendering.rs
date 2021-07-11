use std::collections::HashMap;
use piston_window::{PistonWindow, Event, Context, G2d, clear, text, rectangle, Glyphs, Transformed};
use piston_window::math::Scalar;
use rust_synth::core::control::tools;
use rust_synth::core::control::synth::Id;
use rust_synth::core::synth::{filter, oscillator};
use rust_synth::core::tools::{arpeggiator, transposer};
use rust_synth::core::music_theory::{Hz, pitch::Pitch, diatonic_scale};
use crate::control::Mode;
use rust_synth::core::music_theory::rhythm::Note;

pub type Color = [f32; 4];
const BLACK: Color = [0.0, 0.0, 0.0, 1.0];
const WHITE: Color = [1.0, 1.0, 1.0, 1.0];

pub fn draw(view: tools::View, mode: Mode, window: &mut PistonWindow, glyphs: &mut Glyphs, e: &Event) {
    window.draw_2d(e, |c: Context, g: &mut G2d| {
        clear(BLACK, g);

        draw_mode(mode, 10., 20., glyphs, c, g);

        draw_oscillator(view.synth.instrument.oscillator, 10., 60., glyphs, c, g);
        draw_filter(view.synth.instrument.filter, 10., 80., glyphs, c, g);

        draw_text("volume", 10., 100., glyphs, c, g);
        draw_meter_vertical(view.synth.instrument.volume, 80., 100., c, g);

        if let Some(arp) = view.arpeggiator {
            draw_arpeggiator(arp, view.arp_index, 10., 160., glyphs, c, g);
        }

        let pulse = format!("pulse: {}", view.pulse.period.as_millis().to_string());
        draw_text(pulse.as_str(), 10., 280., glyphs, c, g);

        let loops = format!("{:?}", view.loops);
        draw_text(loops.as_str(), 10., 300., glyphs, c, g);

        draw_transposer(view.transposer, 10., 740., glyphs, c, g);
        draw_notes(view.synth.holding_notes, 10., 760., glyphs, c, g);
    });
}

pub fn draw_mode(mode: Mode, x: Scalar, y: Scalar, glyphs: &mut Glyphs, c: Context, g: &mut G2d) {
    draw_text(format!("{:?}", mode).as_str(), x, y, glyphs, c, g);
}

pub fn draw_oscillator(view: oscillator::View, x: Scalar, y: Scalar, glyphs: &mut Glyphs, c: Context, g: &mut G2d) {
    use oscillator::View::*;
    match view {
        Sine => draw_text("sine", x, y, glyphs, c, g),
        Saw => draw_text("saw", x, y, glyphs, c, g),
        Square => draw_text("square", x, y, glyphs, c, g),
        Pulse(value) => {
            draw_text("pulse", x, y, glyphs, c, g);
            draw_meter_horizontal(value, x + 80., y, c, g);
        }
        Mix { voices } => {
            draw_text("mix", x, y, glyphs, c, g);
            let spread: Vec<Hz> = voices.iter().map(|v| v.tuning).collect();
            for v in spread {
                draw_meter_horizontal(v / 4., x + 150., y, c, g);
            }
        }
    }
}

pub fn draw_filter(view: filter::View, x: Scalar, y: Scalar, glyphs: &mut Glyphs, c: Context, g: &mut G2d) {
    draw_text(format!("{:?}", view.filter_type).as_str(), x, y, glyphs, c, g);
    draw_text("cutoff:", x + 60., y, glyphs, c, g);
    draw_meter_vertical(view.cutoff, x + 140., y, c, g);
    draw_text("resonance:", x + 180., y, glyphs, c, g);
    draw_meter_vertical(view.resonance, x + 300., y, c, g);
}

fn draw_arpeggiator(view: arpeggiator::View, index: f64, x: Scalar, y: Scalar, glyphs: &mut Glyphs, c: Context, g: &mut G2d) {
    draw_text("arpeggiator:", x, y, glyphs, c, g);
    if let Some(holding) = view.holding_pitch {
        draw_text(format!("holding: {}", holding).as_str(), x, y + 20., glyphs, c, g);
    }
    if let Some(playing) = view.playing_pitch {
        draw_text(format!("playing: {}", playing).as_str(), x, y + 40., glyphs, c, g);
    }

    draw_phrase(view.phrase, x + 200., y + 40., c, g);
    draw_meter_horizontal(index * 18., x + 200., y, c, g);
}

fn draw_phrase(phrase: Vec<Note>, x: Scalar, y: Scalar, c: Context, g: &mut G2d) {
    let mut offset = 0.;
    for note in phrase {
        let width = note.duration as u8 as f64 * 4.;
        let degree = diatonic_scale::degree_from(note.pitch) as f64;
        draw_rectangle(width, 4., x + offset, y - degree * 4., c, g);
        offset += width;
    }
}

fn draw_transposer(view: transposer::State, x: Scalar, y: Scalar, glyphs: &mut Glyphs, c: Context, g: &mut G2d) {
    let transposer = format!("key: {}, pitch shift: {}", view.transposed_key,
                             view.pitch_shift);
    draw_text(transposer.as_str(), x, y, glyphs, c, g);
}

fn draw_notes(view: HashMap<Id, Pitch>, x: Scalar, y: Scalar, glyphs: &mut Glyphs, c: Context, g: &mut G2d) {
    let notes_vec = view.values()
        .map(|pitch| format!("{}", pitch))
        .collect::<Vec<_>>();
    let notes_str = format!("notes: {}", notes_vec.join(", "));
    draw_text(notes_str.as_str(), x, y, glyphs, c, g);
}

pub fn draw_text(text: &str, x: Scalar, y: Scalar, glyphs: &mut Glyphs, c: Context, g: &mut G2d) {
    let c2 = c.trans(x, y).zoom(0.5);
    text::Text::new_color(WHITE, 40)
        .draw(text, glyphs, &c2.draw_state, c2.transform, g).unwrap();
}

pub fn draw_meter_vertical(value: f64, x: Scalar, y: Scalar, c: Context, g: &mut G2d) {
    let c2 = c.trans(x, y);
    let rect = [0., -value * 14., 10., 4.];
    rectangle(WHITE, rect, c2.transform, g);
}

pub fn draw_meter_horizontal(value: f64, x: Scalar, y: Scalar, c: Context, g: &mut G2d) {
    let c2 = c.trans(x, y);
    let rect = [value * 14., -10., 4., 10.];
    rectangle(WHITE, rect, c2.transform, g);
}

pub fn draw_rectangle(width: Scalar, height: Scalar, x: Scalar, y: Scalar, c: Context, g: &mut G2d) {
    let c2 = c.trans(x, y);
    let rect = [0., 0., width, height];
    rectangle(WHITE, rect, c2.transform, g);
}