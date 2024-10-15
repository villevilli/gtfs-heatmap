use std::ops::{Add, Div, Mul};

use wgpu::{
    include_wgsl, Adapter, Backends, Color, ColorWrites, Device, FragmentState, Instance,
    InstanceDescriptor, MultisampleState, PipelineCompilationOptions, Queue, RenderPipeline,
    Surface, SurfaceConfiguration,
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
    cx: f64,
    cy: f64,
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
    render_pipeline: RenderPipeline,
}

impl<'a> WgpuState<'a> {
    fn init(window: &'a Window) -> WgpuState<'a> {
        let size = window.inner_size();

        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::GL,
            ..Default::default()
        });

        let surface: Surface<'a> = instance.create_surface(window).unwrap();

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

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[],
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: "fs_main",
                compilation_options: PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        WgpuState {
            instance,
            adapter,
            surface,
            device,
            queue,
            size,
            config,
            render_pipeline,
        }
    }
}

impl ApplicationHandler for App<'_> {
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

        self.wgpu_state = Some(WgpuState::init(self.window.as_ref().unwrap()))
    }

    fn window_event(
        &mut self,
        event_loop: &event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
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
                    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("Render Pass"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(Color {
                                    r: self.cy.mul(0.01).sin().div(2.0).add(0.5),
                                    g: self.cx.mul(0.01).sin().div(2.0).add(0.5),
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

                    render_pass.set_pipeline(&self.wgpu_state.as_ref().unwrap().render_pipeline);

                    render_pass.draw(0..3, 0..1);
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
            CursorMoved {
                device_id: _,
                position,
            } => {
                self.cx = position.x;
                self.cy = position.y;

                self.window.as_ref().unwrap().request_redraw();
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
