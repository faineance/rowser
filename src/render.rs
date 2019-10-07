use crate::html::{Block, BlockClass, Span};
use gfx_glyph::*;

// pub fn render_span<'a>(span: Span) ->
pub fn render_block<'a>(scale: rusttype::Scale, block: &'a Block) -> Vec<SectionText<'a>> {
    let _scale = match block.class {
        BlockClass::H1 => 2.,
        BlockClass::H2 => 1.5,
        BlockClass::H3 => 1.17,
        BlockClass::H4 => 1.,
        BlockClass::H5 => 0.83,
        BlockClass::H6 => 0.67,
        _ => 1.,
    };
    let mut sections = vec![];
    for span in block.content.iter() {
        match span {
            Span::Text { content, class } => sections.push(SectionText {
                text: content.as_str(),
                scale: rusttype::Scale {
                    x: scale.x * _scale,
                    y: scale.y * _scale,
                },
                color: [0.0, 0.0, 0.0, 1.0],
                ..SectionText::default()
            }),
        }
    }
    return sections;
}

pub fn render<'a>(
    blocks: &'a Vec<Block>,
    scale: rusttype::Scale,
    bounds: (f32, f32),
) -> VariedSection<'a> {
    VariedSection {
        text: blocks
            .iter()
            .map(|b| render_block(scale, b))
            .flatten()
            .collect(),
        layout: Layout::default()
            .h_align(HorizontalAlign::Left)
            .v_align(VerticalAlign::Top),
        bounds,
        ..VariedSection::default()
    }
}
