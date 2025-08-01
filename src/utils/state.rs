use std::sync::Arc;
use std::str::FromStr;
use leptos::web_sys;
use wgpu::util::DeviceExt;

use crate::utils::types::buffers::{Vertex, polygon_vertex::PolygonVertex};
use crate::utils::types::camera::camera::{Camera};
use crate::utils::types::camera::camera_controller::CameraController;
use crate::utils::types::camera::camera_uniform::CameraUniform;
use crate::utils::types::keycode::KeyCode;
use crate::utils::types::{size::PhysicalSize, buffers::polygon_buffer::PolygonBuffer};

pub struct State {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
    canvas: Arc<web_sys::HtmlCanvasElement>,
    canvas_size: PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    polygon_buffer: PolygonBuffer<PolygonVertex>,

    camera: Camera,
    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_controller: CameraController,
    camera_bind_group: wgpu::BindGroup,
    // challenge variables
    // challenge_render_pipeline: wgpu::RenderPipeline,
    clear_color: wgpu::Color,

    toggle: bool,
}

impl State {
    pub async fn new(canvas: Arc<web_sys::HtmlCanvasElement>) -> anyhow::Result<State> {
        // handle initialization
        let canvas_size = PhysicalSize::<u32> {
            width: canvas.width(),
            height: canvas.height(),
        };

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::BROWSER_WEBGPU,
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

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: canvas_size.width,
            height: canvas_size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        // handle shaders
        let clear_color = wgpu::Color {
            r: 0.1,
            g: 0.2,
            b: 0.3,
            a: 1.0,
        };

        let shader = wgpu::include_wgsl!("./shaders/buffer_shader.wgsl");

        // let challenge_shader = wgpu::include_wgsl!("./shaders/challenge_shader.wgsl");
        // let challenge_render_pipeline = Self::generate_render_pipeline(challenge_shader, &render_pipeline_layout, &device, &config); #helloworldmeepmorp

        let aspect = canvas_size.width as f32 / canvas_size.height as f32;

        // handle buffers
        let polygon_buffer = PolygonBuffer::polygon_from_sides(&device, 5, 0.5, aspect);

        // handle camera
        let camera = Camera::new(
            // position the camera 1 unit up and 2 units back
            // +z is out of the screen
            (0.0, 0.0, 2.0).into(),
            // have it look at the origin
            (0.0, 0.0, 0.0).into(),
            // which way is "up"
            cgmath::Vector3::unit_y(),
            aspect,
            45.0,
            0.1,
            100.0,
        );


        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&camera);

        let camera_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[camera_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("camera_bind_group_layout"),
        });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                }
            ],
            label: Some("camera_bind_group"),
        });

        let camera_controller = CameraController::new(0.2);

        // handle rendering
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[
                &camera_bind_group_layout
            ],
            push_constant_ranges: &[],
        });

        let render_pipeline = Self::generate_render_pipeline::<PolygonVertex>(shader, &render_pipeline_layout, &device, &config);

        Ok(Self {
            surface,
            device,
            queue,
            config,
            is_surface_configured: false,
            canvas,
            canvas_size,
            render_pipeline,
            camera,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            camera_controller,
            // challenge_render_pipeline,
            clear_color,
            toggle: false,
            polygon_buffer,
        })
    }

    fn generate_render_pipeline<T: Vertex>(source: wgpu::ShaderModuleDescriptor, layout: &wgpu::PipelineLayout, device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> wgpu::RenderPipeline {
        let shader = device.create_shader_module(source);
        
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"), // entry point in our wgsl code
                buffers: &[
                    T::desc(),
                ], // any buffers we may require
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
            // continued ...
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            // continued ...
            depth_stencil: None, // 1.
            multisample: wgpu::MultisampleState {
                count: 1, // 2.
                mask: !0, // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview: None, // 5.
            cache: None, // 6.
        })
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.canvas_size = new_size;
            self.config.width = self.canvas_size.width;
            self.config.height = self.canvas_size.height;

            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;

            // self.polygon_buffer.resize_polygon(&self.device, &self.canvas_size);
        }
    }

    // pub fn canvas(&self) -> &leptos::web_sys::HtmlCanvasElement {
    //     &self.canvas
    // }

    // # TODO: handle key
    pub fn handle_key(&mut self, event: leptos::web_sys::KeyboardEvent) -> Result<(), <KeyCode as ::core::str::FromStr>::Err> {
        let code = KeyCode::from_str(&event.key().to_ascii_lowercase())?;

        // for now - if needed can remove to handle the same key being pressed over and over
        if event.repeat() {
            return Ok(());
        }

        match code {
            KeyCode::KeyCodeSpace => {
                self.toggle = !self.toggle;
            },
            _ => {
                // self.camera_controller.process_events(&code, true);
            }
        }

        Ok(())
    }
    
    // # TODO: update
    pub fn update(&mut self) {
        self.camera_controller.update_camera(&mut self.camera);
        self.camera_uniform.update_view_proj(&self.camera);
        self.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[self.camera_uniform]));
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
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);

            render_pass.set_vertex_buffer(0, self.polygon_buffer.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.polygon_buffer.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.polygon_buffer.num_indices, 0, 0..1);
        }

        self.queue.submit([encoder.finish()]);
        output.present();

        Ok(())
    }
}

// challenge impls
#[allow(dead_code)]
impl State {
    pub fn mouse_challenge(&mut self, loc: PhysicalSize<u32>) {
        let x = loc.width as f64 / self.canvas_size.width as f64;
        let y = loc.height as f64 / self.canvas_size.height as f64;
        let z = (loc.width + loc.height) as f64 / (self.canvas_size.width + self.canvas_size.height) as f64;

        self.clear_color = wgpu::Color {
            r: x,
            g: y,
            b: z,
            a: 1.0,
        }
    }
}