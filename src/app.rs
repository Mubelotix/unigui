use crate::*;

pub trait App: Widget {}

pub fn run<App: crate::app::App + 'static>(mut app: App) -> ! {
    use winit::{
        event::*,
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    };

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut state = futures::executor::block_on(WgpuBackend::new(&window, include_bytes!("graphics/ressources/Inconsolata-Regular.ttf")));
    let mut window_size = window.inner_size();

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => *control_flow = ControlFlow::Exit,
            WindowEvent::Resized(physical_size) => {
                window_size = physical_size.to_owned();
                state.resize(*physical_size);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                window_size = (*new_inner_size).to_owned();
                state.resize(**new_inner_size);
            }
            _ => {}
        },
        Event::RedrawRequested(_) => {
            app.update();
            app.allocate_area(
                (window_size.width as usize, window_size.height as usize),
                (window_size.width as usize, window_size.height as usize),
            );
            app.render(Area::new(Rect::sized(0.0, 0.0, 1920.0, 1080.0), &mut state));

            state.update();
            match state.render() {
                Ok(_) => {}
                // Recreate the swap_chain if lost
                Err(wgpu::SwapChainError::Lost) => state.resize(state.size),
                // The system is out of memory, we should probably quit
                Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                // All other errors (Outdated, Timeout) should be resolved by the next frame
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::MainEventsCleared => {
            // RedrawRequested will only trigger once, unless we manually request it.
            window.request_redraw();
        }
        _ => {}
    });
}
