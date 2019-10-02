use gfx::{format, Device};
use std::error::Error;
mod transform;
mod view_state;
mod network;
mod html;
static WINDOW_TITLE: &str = "ROWSER";



fn main() -> Result<(), Box<dyn Error>> {
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

    let font: &[u8] = include_bytes!("./OpenSans-Light.ttf");
    let mut glyph_brush = gfx_glyph::GlyphBrushBuilder::using_font_bytes(font)
        .initial_cache_size((1024, 1024))
        .build(factory.clone());

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let mut running = true;

    let mut ctrl = false;
    let mut loop_helper = spin_sleep::LoopHelper::builder().build_with_target_rate(250.0);
    let mut view_state = view_state::ViewState::default();

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
                        }
                    }
                    _ => {}
                }
            }
        });

        encoder.clear(&main_colour, [0.02, 0.02, 0.02, 1.0]);

        let (width, height, ..) = main_colour.get_dimensions();
        let (width, height) = (f32::from(width), f32::from(height));
        let scale = Scale::uniform(view_state.font_size * window.get_hidpi_factor() as f32);

        // The section is all the info needed for the glyph brush to render a 'section' of text.
        // Use `..Section::default()` to skip the bits you don't care about
        let section = gfx_glyph::Section {
            text: &view_state.text,
            scale,
            screen_position: (0.0, 0.0),
            bounds: (width / 3.15, height),
            color: [0.9, 0.3, 0.3, 1.0],
            ..Section::default()
        };

        // bounds of a section can be fetched with `pixel_bounds`
        // let _bounds: Option<Rect<i32>> = glyph_brush.pixel_bounds(section);

        // Adds a section & layout to the queue for the next call to `use_queue().draw(..)`, this
        // can be called multiple times for different sections that want to use the same
        // font and gpu cache
        // This step computes the glyph positions, this is cached to avoid unnecessary recalculation
        glyph_brush.queue(section);

        use gfx_glyph::*;
        glyph_brush.queue(Section {
            text: &view_state.text,
            scale,
            screen_position: (width / 2.0, height / 2.0),
            bounds: (width / 3.15, height),
            color: [0.3, 0.9, 0.3, 1.0],
            layout: Layout::default()
                .h_align(HorizontalAlign::Center)
                .v_align(VerticalAlign::Center),
            ..Section::default()
        });

        glyph_brush.queue(Section {
            text: &view_state.text,
            scale,
            screen_position: (width, height),
            bounds: (width / 3.15, height),
            color: [0.3, 0.3, 0.9, 1.0],
            layout: Layout::default()
                .h_align(HorizontalAlign::Right)
                .v_align(VerticalAlign::Bottom),
            ..Section::default()
        });
        glyph_brush.queue(Section {
            text: "status_line",
            scale,
            screen_position: (width, height),
            bounds: (width, height),
            color: [1.0, 1.0, 1.0, 1.0],
            layout: Layout::default()
                .h_align(HorizontalAlign::Right)
                .v_align(VerticalAlign::Bottom),
            ..Section::default()
        });

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
    println!();
    Ok(())
}
