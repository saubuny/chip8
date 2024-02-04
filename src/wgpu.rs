use crate::arch;

use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    platform::wayland::WindowBuilderExtWayland,
    window::{Window, WindowBuilder},
};

const PIXEL_SIZE: f32 = 16.;
const RESOLUTION: (f32, f32) = (
    PIXEL_SIZE * arch::D_WIDTH as f32,
    PIXEL_SIZE * arch::D_HEIGHT as f32,
);

#[allow(dead_code)]
struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: Window,
    clear_color: wgpu::Color,
}

impl State {
    async fn new(window: Window) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .filter(|f| f.is_srgb())
            .next()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        let clear_color = wgpu::Color::BLACK;

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            clear_color,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    // fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
    //     todo!()
    // }

    // Return value indicates whether an event has been fully processed
    fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            _ => false,
        }
    }

    fn update(&mut self) {
        // todo!()
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // Builds a command buffer to send to the GPU
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        // Must be in block as encoder is borrowed mutably and the reference needs to be dropped
        // afterwards
        {
            // Contains the methods for drawing
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.clear_color),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }

        // Finish command buffer and send to GPU
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

pub async fn run() -> Result<(), impl std::error::Error> {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("chip8")
        .with_name("chip8", "chip8")
        .with_inner_size(winit::dpi::PhysicalSize::new(RESOLUTION.0, RESOLUTION.1))
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    let mut state = State::new(window).await;

    event_loop.run(move |event, elwt| {
        // println!("{event:?}");

        match event {
            Event::WindowEvent { event, window_id } if window_id == state.window().id() => {
                if !state.input(&event) {
                    match event {
                        WindowEvent::CloseRequested => elwt.exit(),
                        WindowEvent::RedrawRequested => {
                            state.window.pre_present_notify();
                            state.update();
                            match state.render() {
                                Ok(_) => {}
                                Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                                Err(e) => eprintln!("{:?}", e),
                            }
                        }
                        _ => (),
                    }
                }
            }
            Event::AboutToWait => {
                state.window.request_redraw();
            }
            _ => (),
        }
    })
}
