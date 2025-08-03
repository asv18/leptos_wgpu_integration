use std::sync::Arc;
use std::str::FromStr;
use wgpu::util::DeviceExt;

use crate::utils::types::keycode::KeyCode;
use crate::utils::types::size::PhysicalSize;

pub struct State<'a> {
    // portion of config structure
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
    canvas: Arc<leptos::web_sys::HtmlCanvasElement>,

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

        let shader = wgpu::include_wgsl!("./shaders/buffer_shader.wgsl");

        // handle buffers

        // handle rendering
        let render_pipeline = Self::generate_render_pipeline(
    shader, 
            &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            }),
            &device, 
            &config
        );

        Ok(Self {
            surface,
            device,
            queue,
            config,
            is_surface_configured: false,
            canvas,
            render_pipeline,
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

    fn generate_render_pipeline<>(source: wgpu::ShaderModuleDescriptor, layout: &wgpu::PipelineLayout, device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> wgpu::RenderPipeline {
        let shader = device.create_shader_module(source);
        
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"), // entry point in our wgsl code
                buffers: &[], // any buffers we may require
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
                topology: wgpu::PrimitiveTopology::TriangleList, // defining our topology as a list of triangles
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
        }

        self.queue.submit([encoder.finish()]);
        output.present();

        Ok(())
    }
}

// challenge impls
// impl<'a> State<'a> {
//     pub fn mouse_challenge(&mut self, loc: PhysicalSize<u32>) {
//         let x = loc.width as f64 / self.canvas_size.width as f64;
//         let y = loc.height as f64 / self.canvas_size.height as f64;
//         let z = (loc.width + loc.height) as f64 / (self.canvas_size.width + self.canvas_size.height) as f64;

//         self.clear_color = wgpu::Color {
//             r: x,
//             g: y,
//             b: z,
//             a: 1.0,
//         }
//     }
// }