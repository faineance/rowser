use crate::html::{Block, BlockClass};
use gfx_glyph::*;

pub fn render_block<'a>(block: Block) -> SectionText<'a> {
    let scale = match block.class {
        BlockClass::H1 => 2.,
        BlockClass::H2 => 1.5,
        BlockClass::H3 => 1.17,
        BlockClass::H4 => 1.,
        BlockClass::H5 => 0.83,
        BlockClass::H6 => 0.67,
        _ => 1.,
    };
    unimplemented!()
}

pub fn render<'a>(blocks: Vec<Block>) -> VariedSection<'a> {
    VariedSection {
        text: vec![],
        ..VariedSection::default()
    }
}
