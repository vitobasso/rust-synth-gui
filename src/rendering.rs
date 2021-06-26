
use piston_window::{PistonWindow, TextureSettings, G2dTexture, G2dTextureContext, Context, GfxDevice, G2d};
use piston_window::texture::UpdateTexture;
use conrod_core::{image, text, render};

pub struct Rendering<'a> {
    texture_context: G2dTextureContext,
    text_texture_cache: G2dTexture,
    image_map: image::Map<G2dTexture>,
    text_vertex_data: Vec<u8>,
    glyph_cache: text::GlyphCache<'a>,
}

impl Rendering<'_> {
    pub fn new(window: &mut PistonWindow, width: u32, height: u32) -> Self {

        let mut texture_context = window.create_texture_context();

        let text_texture_cache = {
            let buffer_len = width as usize * height as usize;
            let init = vec![128; buffer_len];
            let settings = TextureSettings::new();
            G2dTexture::from_memory_alpha(&mut texture_context, &init, width as u32, height as u32, &settings)
                .unwrap()
        };

        let image_map = image::Map::new();

        let glyph_cache = text::GlyphCache::builder()
            .dimensions(width, height)
            .build();

        let text_vertex_data = vec![];

        Self { texture_context, text_texture_cache, image_map, text_vertex_data, glyph_cache }
    }

    pub fn draw(&mut self, primitives: render::Primitives, context: Context, graphics: &mut G2d, device: &mut GfxDevice) {
        fn texture_from_image<T>(img: &T) -> &T { img }
        let mut text_texture_cache = &mut self.text_texture_cache;
        let text_vertex_data = &mut self.text_vertex_data;
        let mut texture_context = &mut self.texture_context;
        let mut glyph_cache = &mut self.glyph_cache;

        let cache_queued_glyphs = |_graphics: &mut G2d, cache: &mut G2dTexture, rect: text::rt::Rect<u32>, data: &[u8]| {
            text_vertex_data.clear();
            text_vertex_data.extend(data.iter().flat_map(|&b| vec![255, 255, 255, b]));
            UpdateTexture::update(
                cache,
                &mut texture_context,
                piston_window::texture::Format::Rgba8,
                &text_vertex_data[..],
                [rect.min.x, rect.min.y],
                [rect.width(), rect.height()],
            ).expect("failed to update texture")
        };

        conrod_piston::draw::primitives(primitives, context, graphics,
                                        &mut text_texture_cache,
                                        &mut glyph_cache,
                                        &self.image_map,
                                        cache_queued_glyphs,
                                        texture_from_image);

        texture_context.encoder.flush(device);
    }
}