use pollster::block_on;
use std::sync::Arc;
use wgpu::util::DeviceExt;
use winit::window::Window;

pub struct GpuState {
    surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    vertex_count: u32,
}

impl GpuState {
    pub fn new(window: Arc<Window>, vertices: &[Vertex]) -> Self {
        let size = window.inner_size();

        // Instance: Entry point for GPU backends
        let instance = wgpu::Instance::default();

        // Surface: Visual context tied to the window
        let surface = instance
            .create_surface(window.clone())
            .expect("Failed to create surface");

        // Adapter: Hardware selection
        let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
            apply_limit_buckets: false,
        }))
        .expect("Failed to find compatible graphics adapter");
        let info = adapter.get_info();
        println!("{:?}", info);
        // Device & Queue: Connection to GPU execution units
        let (device, queue) = block_on(adapter.request_device(&wgpu::DeviceDescriptor::default()))
            .expect("Failed to create logical GPU device");

        // Surface Configuration
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        // Toggle this to false to disable VSync and test uncapped FPS.
        const USE_VSYNC: bool = true;

        let present_mode = if USE_VSYNC {
            if surface_caps
                .present_modes
                .contains(&wgpu::PresentMode::Fifo)
            {
                wgpu::PresentMode::Fifo
            } else if surface_caps
                .present_modes
                .contains(&wgpu::PresentMode::Mailbox)
            {
                wgpu::PresentMode::Mailbox
            } else if surface_caps
                .present_modes
                .contains(&wgpu::PresentMode::Immediate)
            {
                wgpu::PresentMode::Immediate
            } else {
                surface_caps.present_modes[0]
            }
        } else if surface_caps
            .present_modes
            .contains(&wgpu::PresentMode::Immediate)
        {
            wgpu::PresentMode::Immediate
        } else if surface_caps
            .present_modes
            .contains(&wgpu::PresentMode::Mailbox)
        {
            wgpu::PresentMode::Mailbox
        } else {
            surface_caps.present_modes[0]
        };
        //end of vsync
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            color_space: wgpu::SurfaceColorSpace::Auto,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        let pipeline = Self::create_pipeline(&device, &config);
        let vertex_count = vertices.len() as u32;
        let vertex_buffer = if vertices.is_empty() {
            device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Fallback Vertex Buffer"),
                size: std::mem::size_of::<Vertex>() as u64,
                usage: wgpu::BufferUsages::VERTEX,
                mapped_at_creation: false,
            })
        } else {
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: vertices_to_bytes(vertices),
                usage: wgpu::BufferUsages::VERTEX,
            })
        };

        Self {
            surface,
            device,
            queue,
            config,
            size,
            pipeline,
            vertex_buffer,
            vertex_count,
        }
    }

    fn create_pipeline(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
    ) -> wgpu::RenderPipeline {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            // / gets turned into \ on windows devices
            source: wgpu::ShaderSource::Wgsl(include_str!("shader/triangle.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("render pipeline layout"),
            bind_group_layouts: &[],
            immediate_size: 0,
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[Some(Vertex::desc())],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            cache: None,
            multiview_mask: None,
        })
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn render(&mut self) -> Result<(), String> {
        let output = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(tex) => tex,
            wgpu::CurrentSurfaceTexture::Suboptimal(tex) => {
                // still usable, but reconfigure next time for best performance
                tex
            }
            wgpu::CurrentSurfaceTexture::Timeout | wgpu::CurrentSurfaceTexture::Occluded => {
                return Ok(()); // skip this frame
            }
            wgpu::CurrentSurfaceTexture::Outdated | wgpu::CurrentSurfaceTexture::Lost => {
                self.surface.configure(&self.device, &self.config);
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Validation => {
                return Err("Surface validation error".to_string());
            }
        };

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Main Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Clear Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            if self.vertex_count > 0 {
                render_pass.draw(0..self.vertex_count, 0..1);
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        self.queue.present(output);

        Ok(())
    }

    /// Replace the current vertex buffer with new vertices for the next frame.
    pub fn update_vertices(&mut self, vertices: &[Vertex]) {
        self.vertex_count = vertices.len() as u32;
        if vertices.is_empty() {
            self.vertex_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Fallback Vertex Buffer"),
                size: std::mem::size_of::<Vertex>() as u64,
                usage: wgpu::BufferUsages::VERTEX,
                mapped_at_creation: false,
            });
        } else {
            self.vertex_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: vertices_to_bytes(vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });
        }
    }

    /// Create vertex data for a rectangle in pixel coordinates (origin top-left).
    /// `window_size` is used to convert to clip space.
    pub fn rect_vertices_for_window(
        window_size: winit::dpi::PhysicalSize<u32>,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        color: [f32; 3],
    ) -> Vec<Vertex> {
        let width = window_size.width as f32;
        let height = window_size.height as f32;

        let to_clip = |x: f32, y: f32| -> [f32; 2] {
            let nx = (x / width) * 2.0 - 1.0;
            let ny = 1.0 - (y / height) * 2.0;
            [nx, ny]
        };

        let tl = to_clip(x, y);
        let tr = to_clip(x + w, y);
        let bl = to_clip(x, y + h);
        let br = to_clip(x + w, y + h);

        let mut verts = Vec::with_capacity(6);
        verts.push(Vertex { position: tl, color });
        verts.push(Vertex { position: bl, color });
        verts.push(Vertex { position: tr, color });
        verts.push(Vertex { position: tr, color });
        verts.push(Vertex { position: bl, color });
        verts.push(Vertex { position: br, color });
        verts
    }
}

// One point of a shape: where it is, and what color it is.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 3],
}

impl Vertex {
    // Describes the memory layout above so the GPU knows how to read it.
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // position -> shader location 0
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                // color -> shader location 1
                wgpu::VertexAttribute {
                    offset: std::mem::offset_of!(Vertex, color) as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

// Reinterprets a slice of Vertex as a slice of raw bytes, so it can be uploaded to the GPU.
// Safe because Vertex is #[repr(C)]
fn vertices_to_bytes(vertices: &[Vertex]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(
            vertices.as_ptr() as *const u8,
            std::mem::size_of_val(vertices),
        )
    }
}

