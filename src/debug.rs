use sdl2::{pixels::Color, rect::Rect, render::Canvas, ttf::Font, video::Window};
use std::{
    collections::{BTreeMap, HashMap},
    fmt::Debug,
};

/// A Minecraft-esque debug screen to render certain values.
pub struct DebugRenderer<'a> {
    font: &'a Font<'a, 'a>,
    pub items: BTreeMap<&'static str, &'a dyn Debug>,
}

impl<'a> DebugRenderer<'a> {
    pub fn new(font: &'a Font) -> Self {
        Self {
            font,
            items: BTreeMap::new(),
        }
    }

    pub fn render_to_canvas(self, canvas: &mut Canvas<Window>) {
        let len = self.items.len();
        if len == 0 {
            return;
        }
        let texture_creator = canvas.texture_creator();
        let mut offset = 0u32;
        for item in self.items.into_iter() {
            let font_surf = self
                .font
                .render(&format!("{0}: {1:?}", item.0, item.1))
                .shaded(Color::RGB(255, 255, 255), Color::RGBA(0, 0, 0, 50))
                .unwrap();

            let (wide, tall) = (font_surf.width(), font_surf.height());
            let position_scale = Rect::new(50, 50 + i32::try_from(offset).unwrap(), wide, tall);

            let texture = font_surf.as_texture(&texture_creator).unwrap();
            canvas.copy(&texture, None, position_scale);
            offset += tall;
        }
        // dropped here!
    }
}
