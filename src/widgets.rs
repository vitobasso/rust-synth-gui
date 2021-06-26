use conrod_core::{widget, Widget, UiCell, Ui};
use conrod_core::color::Colorable;
use conrod_core::position::Positionable;

pub struct Widgets {
    ids: Ids,
}

widget_ids! {
    struct Ids {
        canvas,
        title,
    }
}

impl Widgets {

    pub fn new(ui: &mut Ui) -> Self {
        let ids = Ids::new(ui.widget_id_generator());
        Self { ids }
    }

    pub fn update(&self, ui: &mut UiCell) {
        widget::Canvas::new()
            .pad(30.)
            .scroll_kids_vertically()
            .set(self.ids.canvas, ui);

        widget::Text::new("Sintetizador Maravilhoso")
            .color(conrod_core::color::WHITE)
            .font_size(42)
            .mid_top_of(self.ids.canvas)
            .set(self.ids.title, ui);
    }

}