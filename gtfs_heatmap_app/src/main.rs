use wgpu::{Adapter, Instance, Surface};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{self, ControlFlow, EventLoop},
    window::Window,
};

#[derive(Default)]
struct App<'a> {
    window: Option<Window>,
    wgpu_state: Option<WgpuState<'a>>,
}

struct WgpuState<'a> {
    instance: Instance,
    adapter: Adapter,
    surface: Surface<'a>,
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &event_loop::ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("Hello world!")
                        .with_visible(true)
                        .with_resizable(true),
                )
                .unwrap(),
        )

        self.wgpu_state = Some(WgpuState{ instance: todo!(), adapter: todo!(), surface: todo!() })
    }

    fn window_event(
        &mut self,
        event_loop: &event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        use WindowEvent::*;
        match event {
            CloseRequested => {
                println!("Window is closing!");
                event_loop.exit();
            }
            RedrawRequested => {
                let handle = self.window.as_ref().unwrap();

                handle.request_redraw();
            }
            _ => (),
        }
    }
}

fn main() {
    let eventloop = EventLoop::new().unwrap();

    eventloop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    eventloop.run_app(&mut app).unwrap();
}
