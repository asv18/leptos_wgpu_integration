use std::sync::Arc;
use std::str::FromStr;
use wgpu::util::DeviceExt;

use crate::utils::graphics::types::buffers::{TriangleUniform, Vertex};

use super::types::keycode::KeyCode;
use super::types::size::PhysicalSize;

pub struct State<'a> {
    // portion of config structure
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
    canvas: Arc<leptos::web_sys::HtmlCanvasElement>,
    
    // portion for buffers
    triangle_buffer: wgpu::Buffer,
    triangle_uniforms: Vec<TriangleUniform>,
    bind_group: wgpu::BindGroup,
    vertex_buffer: wgpu::Buffer,
    vertices: Vec<Vertex>,

    index_buffer: wgpu::Buffer,
    indices: Vec<u16>,

    num_instances: u32,
    // portion of render structure
    surface: wgpu::Surface<'a>,
    render_pipeline: wgpu::RenderPipeline,
    device: wgpu::Device,
    queue: wgpu::Queue,
    clear_color: wgpu::Color,
}

impl<'a> State<'a> {
    pub async fn new(canvas: Arc<leptos::web_sys::HtmlCanvasElement>) -> anyhow::Result<State<'a>> {
        // handle initialization
        let canvas_size = PhysicalSize::<u32> {
            width: canvas.width(),
            height: canvas.height(),
        };

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = instance.create_surface(wgpu::SurfaceTarget::Canvas(canvas.as_ref().clone()))?;

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }).await?;

        let (device, queue) = adapter.request_device(
        &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_webgl2_defaults(),
                label: None,
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            },
        ).await?;

        let config = Self::generate_config(&adapter, &surface, &canvas_size);

        // handle shaders
        let clear_color = wgpu::Color {
            r: 0.1,
            g: 0.2,
            b: 0.3,
            a: 1.0,
        };

        let shader = wgpu::include_wgsl!("./shaders/vertex_index_buffer.wgsl");

        let (vertices, indices) = create_circle_vertices(0.5, 24, 0.3, 0.0, std::f32::consts::PI * 2.0);

        let mut triangle_uniforms = Vec::new();
        
        use rand::Rng;

        let aspect = canvas_size.width as f32 / canvas_size.height as f32;
        let mut rng = rand::thread_rng();

        let num_instances = 100;

        for _i in 0..num_instances {
            let scale = rng.gen_range(0.2..0.5) / aspect;
            let triangle_uniform = TriangleUniform::new([rng.gen_range(0.0..=1.0), rng.gen_range(0.0..=1.0), rng.gen_range(0.0..=1.0), 1.0], [scale, scale], [rng.gen_range(-0.9..=0.9), rng.gen_range(-0.9..=0.9)]);

            triangle_uniforms.push(triangle_uniform);
        }

        // handle buffers
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let triangle_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Buffer"),
            contents: bytemuck::cast_slice(&triangle_uniforms),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("Bind group layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            },
        );

        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some("Bind group"),
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: triangle_buffer.as_entire_binding(),
                    },
                ],
            }
        );

        // let aspect = canvas_size.width as f32 / canvas_size.height as f32; 

        // handle rendering
        let render_pipeline = Self::generate_render_pipeline(
    shader, 
            &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            }),
            &device, 
            &config,
            vec![Vertex::desc()],
        );

        Ok(Self {
            surface,
            device,
            queue,
            config,
            is_surface_configured: false,
            canvas,
            render_pipeline,
            triangle_buffer,
            triangle_uniforms,
            // triangle_bind_group,
            bind_group,
            vertex_buffer,
            vertices,

            index_buffer,
            indices,

            num_instances,
            // vertices,
            // triangle_uniform_buffers,
            // triangle_bind_groups,
            // challenge_render_pipeline,
            clear_color,
            // toggle: false,
        })
    }

    fn generate_config(adapter: &wgpu::Adapter, surface: &wgpu::Surface, canvas_size: &PhysicalSize<u32>) -> wgpu::SurfaceConfiguration {
        let surface_caps = surface.get_capabilities(adapter);

        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: canvas_size.width,
            height: canvas_size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        }
    }

    fn generate_render_pipeline<>(source: wgpu::ShaderModuleDescriptor, layout: &wgpu::PipelineLayout, device: &wgpu::Device, config: &wgpu::SurfaceConfiguration, vertex_buffer: Vec<wgpu::VertexBufferLayout>) -> wgpu::RenderPipeline {
        let shader = device.create_shader_module(source);
        
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"), // entry point in our wgsl code
                buffers: &vertex_buffer, // any buffers we may require
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState { // defining our fragment
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState { // defining the targets for our fragment
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // defining every three vertices as a triangle
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // having our code read vertices CCW
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            // continued ...
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        })
    }

    pub fn resize(&mut self, new_canvas: Arc<leptos::web_sys::HtmlCanvasElement>) {
        if new_canvas.width() > 0 && new_canvas.height() > 0 {
            self.canvas = new_canvas;
            self.config.width = self.canvas.width();
            self.config.height = self.canvas.height();

            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;

            // self.polygon_buffer.resize_polygon(&self.device, &self.canvas_size);
        }
    }

    // # TODO: handle key
    pub fn handle_key(&mut self, event: leptos::web_sys::KeyboardEvent) -> Result<(), <KeyCode as ::core::str::FromStr>::Err> {
        let code = KeyCode::from_str(&event.key().to_ascii_lowercase()).unwrap_or(KeyCode::Unknown);

        // if needed can uncomment to handle the same key being pressed over and over
        // if event.repeat() {
        //     return Ok(());
        // }

        match code {
            // KeyCode::KeyCodeSpace => {
            //     self.toggle = !self.toggle;
            // },
            _ => {
                leptos::logging::log!("{:?}", code);
                // self.camera_controller.process_events(&code, true);
            }
        }

        Ok(())
    }
    
    // # TODO: update
    pub fn update(&mut self) {

    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        // self.canvas.as_ref().

        if !self.is_surface_configured {
            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;
        }

        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);

            // for item in &self.triangle_items {
            //     render_pass.set_bind_group(0, &item.bind_group, &[]);
            //     render_pass.draw(0..3, 0..1);
            // }

            // self.queue.write_buffer(&self.triangle_buffer, 0, bytemuck::cast_slice(&self.triangle_uniforms));

            // render_pass.set_bind_group(0, &self.triangle_bind_group, &[]);
            render_pass.set_bind_group(0, &self.bind_group, &[]);

            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.indices.len() as u32, 0, 0..self.num_instances);

            // render_pass.draw(0..3, 0..self.triangle_uniforms.len() as u32);
        }

        self.queue.submit([encoder.finish()]);
        output.present();

        Ok(())
    }
}

fn create_circle_vertices(
    radius: f32,
    num_subdivisions: u16,
    inner_radius: f32,
    start_angle: f32,
    end_angle: f32,
) -> (Vec<Vertex>, Vec<u16>) {
    // 2 triangles per subdivision, 3 verts per tri, 2 values (xy) each.
    // let num_vertices = num_subdivisions * 3 * 2;
    let mut indices = Vec::with_capacity((num_subdivisions * 6) as usize);
    let mut vertex_data = Vec::with_capacity((num_subdivisions * 2 * 3 * 2) as usize);

    // 2 vertices per subdivision
    //
    // 0--1 4
    // | / /|
    // |/ / |
    // 2 3--5
    for i in 0..num_subdivisions {
        let base = i * 6;

        let angle1 = start_angle + (i as f32 + 0.0) * (end_angle - start_angle) / num_subdivisions as f32;
        let angle2 = start_angle + (i as f32 + 1.0) * (end_angle - start_angle) / num_subdivisions as f32;

        let c1 = angle1.cos();
        let s1 = angle1.sin();
        let c2 = angle2.cos();
        let s2 = angle2.sin();

        // first triangle
        vertex_data.push(Vertex {
            position: [c1 * radius, s1 * radius],
        });
        vertex_data.push(Vertex {
            position: [c2 * radius, s2 * radius],
        });
        vertex_data.push(Vertex {
            position: [c1 * inner_radius, s1 * inner_radius],
        });
        
        indices.push(base + 0);
        indices.push(base + 1);
        indices.push(base + 2);

        // second triangle
        vertex_data.push(Vertex {
            position: [c1 * inner_radius, s1 * inner_radius],
        });
        vertex_data.push(Vertex {
            position: [c2 * radius, s2 * radius],
        });
        vertex_data.push(Vertex {
            position: [c2 * inner_radius, s2 * inner_radius],
        });
        
        indices.push(base + 3);
        indices.push(base + 4);
        indices.push(base + 5);
    }

    (vertex_data, indices)
}
