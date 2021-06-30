use piston_window::{PistonWindow, Event, Context, G2d, clear, text, Glyphs, Transformed};

pub type Color = [f32; 4];
const BLACK: Color = [0.0, 0.0, 0.0, 1.0];
const WHITE: Color = [1.0, 1.0, 1.0, 1.0];

pub fn draw(window: &mut PistonWindow, glyphs: &mut Glyphs, e: &Event) {
    window.draw_2d(e, |c: Context, g: &mut G2d| {
        clear(BLACK, g);
        let c2 = c.trans(10., 20.).zoom(0.5);
        text::Text::new_color(WHITE, 40)
            .draw("hello", glyphs, &c2.draw_state, c2.transform, g)
    });
}