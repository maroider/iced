use iced_raqote::{
    program,
    raqote::{DrawTarget, SolidSource},
    Backend, Debug, Renderer, Settings, Size, Viewport,
};
use winit::{
    dpi::PhysicalPosition,
    event::{Event, ModifiersState, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};
use winit_blit::{PixelBufferTyped, BGRA};

mod controls;
mod iced_conversion;

use controls::Controls;

fn main() {
    env_logger::init();

    // Initialize winit
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();

    let mut size = window.inner_size();
    let mut viewport =
        Viewport::with_physical_size(Size::new(size.width, size.height), window.scale_factor());
    let mut cursor_position = PhysicalPosition::new(-1.0, -1.0);
    let mut modifiers = ModifiersState::default();

    let mut pixbuf: PixelBufferTyped<BGRA> =
        PixelBufferTyped::new_supported(size.width, size.height, &window);
    let mut draw_target = DrawTarget::new(size.width as i32, size.height as i32);

    let mut resized = false;

    let controls = Controls::new();

    let mut debug = Debug::new();
    let mut renderer = Renderer::new(Backend::new(Settings::default()));

    let mut state = program::State::new(
        controls,
        viewport.logical_size(),
        iced_conversion::cursor_position(cursor_position, viewport.scale_factor()),
        &mut renderer,
        &mut debug,
    );

    // Run event loop
    event_loop.run(move |event, _, control_flow| {
        // You should change this if you want to render continuosly
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CursorMoved { position, .. } => {
                        cursor_position = position;
                    }
                    WindowEvent::ModifiersChanged(new_modifiers) => {
                        modifiers = new_modifiers;
                    }
                    WindowEvent::Resized(new_size) => {
                        viewport = Viewport::with_physical_size(
                            Size::new(new_size.width, new_size.height),
                            window.scale_factor(),
                        );
                        size = new_size;

                        resized = true;
                    }
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => {}
                }

                if let Some(event) =
                    iced_conversion::window_event(&event, window.scale_factor(), modifiers)
                {
                    state.queue_event(event);
                }
            }
            Event::MainEventsCleared => {
                if !state.is_queue_empty() {
                    let _ = state.update(
                        viewport.logical_size(),
                        iced_conversion::cursor_position(cursor_position, viewport.scale_factor()),
                        None,
                        &mut renderer,
                        &mut debug,
                    );

                    window.request_redraw();
                }
            }
            Event::RedrawRequested(_) => {
                if resized {
                    pixbuf = PixelBufferTyped::new_supported(size.width, size.height, &window);
                    draw_target = DrawTarget::new(size.width as i32, size.height as i32);
                    resized = false;
                }

                let program = state.program();
                let background_color = program.background_color();

                draw_target.clear(SolidSource::from_unpremultiplied_argb(
                    (background_color.a * 255.0) as u8,
                    (background_color.r * 255.0) as u8,
                    (background_color.g * 255.0) as u8,
                    (background_color.b * 255.0) as u8,
                ));

                let mouse_interaction = renderer.backend_mut().draw(
                    &mut draw_target,
                    &viewport,
                    state.primitive(),
                    &debug.overlay(),
                );

                for (dst_row, src_row) in pixbuf
                    .rows_mut()
                    .zip(draw_target.get_data().chunks_exact(size.width as usize))
                {
                    for (dst_pixel, src_pixel) in dst_row.iter_mut().zip(src_row.iter()) {
                        let [b, g, r, a] = src_pixel.to_le_bytes();
                        *dst_pixel = BGRA { b, g, r, a };
                    }
                }

                pixbuf.blit(&window).unwrap();

                window.set_cursor_icon(iced_conversion::mouse_interaction(mouse_interaction));
            }
            _ => {}
        }
    })
}
