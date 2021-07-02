use piston_window::{PistonWindow, Event, Context, G2d, clear, text, Glyphs, Transformed};
use rust_synth::core::control::tools::View;
use piston_window::math::Scalar;

pub type Color = [f32; 4];
const BLACK: Color = [0.0, 0.0, 0.0, 1.0];
const WHITE: Color = [1.0, 1.0, 1.0, 1.0];

pub fn draw(view: View, window: &mut PistonWindow, glyphs: &mut Glyphs, e: &Event) {
    window.draw_2d(e, |c: Context, g: &mut G2d| {
        clear(BLACK, g);

        let transposer = format!("key: {}, pitch shift: {}", view.transposer.transposed_key,
                                 view.transposer.pitch_shift);
        draw_text(transposer.as_str(), 10., 40., glyphs, c, g);

        let notes_vec = view.synth.holding_notes.values()
            .map(|pitch| format!("{}", pitch))
            .collect::<Vec<_>>();
        let notes_str = format!("notes: {}", notes_vec.join(", "));
        draw_text(notes_str.as_str(), 10., 60., glyphs, c, g);



        let patch = format!("patch: {}", view.selected_patch.to_string());
        draw_text(patch.as_str(), 10., 100., glyphs, c, g);

        let filter = format!("cutoff: {}, resonance: {}", view.synth.instrument.filter.cutoff,
                             view.synth.instrument.filter.resonance);
        draw_text(filter.as_str(), 10., 120., glyphs, c, g);



        if let Some(arp) = view.arpeggiator {
            //arp.phrase.
            let index = format!("index: {}", view.arp_index);
            draw_text(index.as_str(), 10., 160., glyphs, c, g);

            let holding = arp.holding_pitch.map(|p| format!("holding: {}", p));
            let playing = arp.playing_pitch.map(|p| format!("playing: {}", p));
            if let (Some(holding), Some(playing)) = (holding, playing) {
                let notes = format!("{}, {}", holding, playing);
                draw_text(notes.as_str(), 10., 180., glyphs, c, g);
            }
        }


        let pulse = format!("pulse: {}", view.pulse.period.as_millis().to_string());
        draw_text(pulse.as_str(), 10., 220., glyphs, c, g);

        let loops = format!("{:?}", view.loops);
        draw_text(loops.as_str(), 10., 240., glyphs, c, g);
    });
}

pub fn draw_text(text: &str, x: Scalar, y: Scalar, glyphs: &mut Glyphs, c: Context, g: &mut G2d) {
    let c2 = c.trans(x, y).zoom(0.5);
    text::Text::new_color(WHITE, 40)
        .draw(text, glyphs, &c2.draw_state, c2.transform, g).unwrap()
}
