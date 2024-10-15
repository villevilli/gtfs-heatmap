use wgpu::{
    Adapter, Backends, Color, Device, Instance, InstanceDescriptor, Queue, Surface,
    SurfaceConfiguration,
};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{self, ControlFlow, EventLoop},
    window::Window,
};

#[derive(Default)]
struct App<'a> {
    window: Option<&'a Window>,
    wgpu_state: Option<WgpuState<'a>>,
}

struct WgpuState<'a> {
    instance: Instance,
    adapter: Adapter,
    surface: Surface<'a>,
    device: Device,
    queue: Queue,
    size: PhysicalSize<u32>,
    config: SurfaceConfiguration,
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &event_loop::ActiveEventLoop) {
        //This leaks memory each time application is resumed.
        self.window = Some(Box::leak(Box::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("Hello world!")
                        .with_visible(true)
                        .with_resizable(true),
                )
                .unwrap(),
        )));

        let size = self.window.unwrap().inner_size();

        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::GL,
            ..Default::default()
        });

        let surface: Surface<'a> = instance.create_surface(self.window.unwrap()).unwrap();

        let adapter =
            futures::executor::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            }))
            .unwrap();

        let (device, queue) = futures::executor::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: Default::default(),
            },
            None,
        ))
        .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        self.wgpu_state = Some(WgpuState {
            instance,
            adapter,
            surface,
            device,
            queue,
            size,
            config,
        })
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
                let output = self
                    .wgpu_state
                    .as_ref()
                    .unwrap()
                    .surface
                    .get_current_texture()
                    .unwrap();

                let view = output
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                let mut encoder = self
                    .wgpu_state
                    .as_ref()
                    .unwrap()
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("Render Encoder"),
                    });

                {
                    let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("Render Pass"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(Color {
                                    r: 0.2,
                                    g: 0.2,
                                    b: 0.0,
                                    a: 1.0,
                                }),
                                store: wgpu::StoreOp::Store,
                            },
                        })],
                        depth_stencil_attachment: None,
                        timestamp_writes: None,
                        occlusion_query_set: None,
                    });
                }
                self.wgpu_state
                    .as_ref()
                    .unwrap()
                    .queue
                    .submit(std::iter::once(encoder.finish()));

                output.present();
            }
            Resized(new_size) => {
                if new_size.width > 0 && new_size.height > 0 {
                    self.wgpu_state.as_mut().unwrap().config.width = new_size.width;
                    self.wgpu_state.as_mut().unwrap().config.height = new_size.height;

                    self.wgpu_state.as_ref().unwrap().surface.configure(
                        &self.wgpu_state.as_ref().unwrap().device,
                        &self.wgpu_state.as_ref().unwrap().config,
                    );
                }
            }
            _ => (),
        }
    }
}

fn main() {
    env_logger::init();
    let eventloop = EventLoop::new().unwrap();

    eventloop.set_control_flow(ControlFlow::Wait);

    let mut app = App::default();
    eventloop.run_app(&mut app).unwrap();
}
