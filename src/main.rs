#[macro_use]
extern crate html5ever;
#[macro_use]
extern crate gfx;
extern crate font_loader;
use gfx::{format, Device};
use gfx_glyph::*;
use std::env;
use std::error::Error;
mod fonts;
mod html;
mod render;
mod transform;
mod view_state;

use font_loader::system_fonts;

use gfx::traits::FactoryExt;
static WINDOW_TITLE: &str = "ROWSER";

gfx_defines! {
    pipeline rectpipe {
        rect :gfx::Global<[f32;4]> = "rect",
        color :gfx::Global<[f32;4]> = "color",
        out: gfx::BlendTarget<format::Srgba8> = ("target0", gfx::state::ColorMask::all(), gfx::preset::blend::ALPHA),
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let raw_html = reqwest::get(&args[1])?.text()?;
    let blocks = html::parse_html(raw_html);

    // println!("{:?}", blocks);
    env_logger::init();
    let mut events_loop = glutin::EventsLoop::new();

    let window_builder = glutin::WindowBuilder::new()
        .with_title(WINDOW_TITLE)
        .with_dimensions((1024, 576).into());
    let context = glutin::ContextBuilder::new();
    let (window_ctx, mut device, mut factory, mut main_colour, mut main_depth) =
        gfx_window_glutin::init::<format::Srgba8, format::DepthStencil>(
            window_builder,
            context,
            &events_loop,
        )
        .unwrap();
    let window = window_ctx.window();

    let mut glyph_brush = gfx_glyph::GlyphBrushBuilder::using_fonts(fonts::load_fonts().to_vec())
        .initial_cache_size((1024, 1024))
        .build(factory.clone());

    let rect_shaders = factory
        .create_shader_set(
            include_bytes!("rect_150.glslv"),
            include_bytes!("rect_150.glslf"),
        )
        .expect("Error compiling parsers");
    let rect_rasterizer = gfx::state::Rasterizer::new_fill();
    let rect_pso = factory
        .create_pipeline_state(
            &rect_shaders,
            gfx::Primitive::TriangleStrip,
            rect_rasterizer,
            rectpipe::new(),
        )
        .expect("rect_pso");

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let mut running = true;

    let mut ctrl = false;
    let mut loop_helper = spin_sleep::LoopHelper::builder().build_with_target_rate(250.0);
    let mut view_state = view_state::ViewState::default();

    view_state.text = "".to_string();

    while running {
        loop_helper.loop_start();

        events_loop.poll_events(|event| {
            use glutin::*;

            if let Event::WindowEvent { event, .. } = event {
                match event {
                    WindowEvent::CloseRequested => running = false,
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(keypress),
                                ..
                            },
                        ..
                    } => match keypress {
                        VirtualKeyCode::Escape => running = false,
                        VirtualKeyCode::Back => {
                            view_state.text.pop();
                        }
                        VirtualKeyCode::LControl | VirtualKeyCode::RControl => ctrl = true,
                        _ => (),
                    },
                    WindowEvent::MouseInput {
                        state: ElementState::Pressed,
                        ..
                    } => {
                        // view_state.click()
                    }
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Released,
                                ..
                            },
                        ..
                    } => ctrl = false,
                    WindowEvent::ReceivedCharacter(c) => {
                        if c != '\u{7f}' && c != '\u{8}' {
                            view_state.text.push(c);
                        }
                    }
                    WindowEvent::Resized(size) => {
                        window_ctx.resize(size.to_physical(window.get_hidpi_factor()));
                        gfx_window_glutin::update_views(
                            &window_ctx,
                            &mut main_colour,
                            &mut main_depth,
                        );
                    }
                    WindowEvent::MouseWheel {
                        delta: MouseScrollDelta::LineDelta(_, y),
                        modifiers: ModifiersState { ctrl, shift, .. },
                        ..
                    } => {
                        if ctrl && shift {
                            view_state.rotate(y)
                        } else if ctrl && !shift {
                            view_state.zoom(y);
                        } else if shift {
                            view_state.scale(y);
                        } else {
                            view_state.scroll(
                                view_state.font_size * window.get_hidpi_factor() as f32 * y,
                            );
                        }
                    }
                    _ => {}
                }
            }
        });

        encoder.clear(&main_colour, [1.0, 1.0, 1.0, 1.0]);

        let (width, height, ..) = main_colour.get_dimensions();
        let (width, height) = (f32::from(width), f32::from(height));
        let scale = Scale::uniform(view_state.font_size * window.get_hidpi_factor() as f32);

        let section = gfx_glyph::Section {
            text: &view_state.text,
            scale,
            screen_position: (0.0, 0.0),
            bounds: (width, height),
            color: [0.9, 0.3, 0.3, 1.0],
            ..Section::default()
        };
        let slice = gfx::Slice {
            start: 0,
            end: 4,
            buffer: gfx::IndexBuffer::Auto,
            base_vertex: 0,
            instances: None,
        };
        let r = ((0., 3.), (50., 34.), (543., 23.), (100., 34.));
        // Convert from screen to opengl coords
        let r = (
            (
                2.0 * ((r.0).0 / width - 0.5),
                2.0 * (0.5 - (r.0).1 / height),
            ),
            (
                2.0 * ((r.1).0 / width - 0.5),
                2.0 * (0.5 - (r.1).1 / height),
            ),
        );

        encoder.draw(
            &slice,
            &rect_pso,
            &rectpipe::Data {
                rect: [(r.0).0, (r.0).1, (r.1).0, (r.1).1],
                color: [0., 1., 1., 1.],
                out: main_colour.clone(),
            },
        );

        // bounds of a section can be fetched with `pixel_bounds`
        // let _bounds: Option<Rect<i32>> = glyph_brush.pixel_bounds(section);

        // Adds a section & layout to the queue for the next call to `use_queue().draw(..)`, this
        // can be called multiple times for different sections that want to use the same
        // font and gpu cache
        // This step computes the glyph positions, this is cached to avoid unnecessary recalculation
        glyph_brush.queue(section);

        glyph_brush.queue(render::render(&blocks, scale, (width, height)));

        // glyph_brush.queue(Section {
        //     text: &view_state.text,
        //     scale,
        //     screen_position: (width / 2.0, height / 2.0),
        //     bounds: (width / 3.15, height),
        //     color: [0.3, 0.9, 0.3, 1.0],
        //     layout: Layout::default()
        //         .h_align(HorizontalAlign::Center)
        //         .v_align(VerticalAlign::Center),
        //     ..Section::default()
        // });

        // glyph_brush.queue(Section {
        //     text: &view_state.text,
        //     scale,
        //     screen_position: (width, height),
        //     bounds: (width / 3.15, height),
        //     color: [0.3, 0.3, 0.9, 1.0],
        //     layout: Layout::default()
        //         .h_align(HorizontalAlign::Right)
        //         .v_align(VerticalAlign::Bottom),
        //     ..Section::default()
        // });
        // Finally once per frame you want to actually draw all the sections you've submitted
        // with `queue` calls.
        //
        // Drawing in the case the text is unchanged from the previous frame (a common case)
        // is essentially free as the vertices are reused & gpu cache updating interaction
        // can be skipped.
        glyph_brush
            .use_queue()
            .transform(transform::generate_transform(width, height, &view_state))
            .draw(&mut encoder, &main_colour)?;
        encoder.flush(&mut device);
        window_ctx.swap_buffers()?;
        device.cleanup();

        if let Some(rate) = loop_helper.report_rate() {
            window.set_title(&format!("{} - {:.0}FPS", WINDOW_TITLE, rate));
        }

        loop_helper.loop_sleep();
    }
    Ok(())
}
