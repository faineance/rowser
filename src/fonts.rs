use gfx_glyph::*;

use font_loader::system_fonts;

fn load_font_bytes<'a>(property: system_fonts::FontProperty) -> Font<'a> {
    gfx_glyph::Font::from_bytes(system_fonts::get(&property).unwrap().0).unwrap()
}
pub fn load_fonts<'a>() -> [Font<'a>;4] {
    [
        load_font_bytes(system_fonts::FontPropertyBuilder::new().family("Arial").build()),
        load_font_bytes(system_fonts::FontPropertyBuilder::new().bold().build()),
        load_font_bytes(system_fonts::FontPropertyBuilder::new().italic().build()),
        load_font_bytes(system_fonts::FontPropertyBuilder::new().monospace().build()),
    ]
}
