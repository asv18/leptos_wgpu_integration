use leptos::{html::Canvas, prelude::*};
use leptos::wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use crate::components::types::size::PhysicalSize;

#[allow(non_snake_case)]
#[component]
pub fn Window() -> impl IntoView {
    let canvas_node_ref = NodeRef::<Canvas>::new();

    canvas_node_ref.on_load(move |canvas| {
        let canvas = canvas.clone();
        
        spawn_local(async move {
            let canvas: leptos::web_sys::HtmlCanvasElement = canvas.dyn_into().expect("Expected canvas element");
            canvas.set_width(canvas.client_width() as u32);
            canvas.set_height(canvas.client_height() as u32);
            
            let size = PhysicalSize::<u32> {
                width: canvas.width(),
                height: canvas.height(),
            };

            let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
                backends: wgpu::Backends::default(),
                ..Default::default()
            });

            let surface = instance.create_surface(wgpu::SurfaceTarget::Canvas(canvas)).unwrap();

            let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            }).await.unwrap();

            let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web, we'll have to disable some.
                    required_limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label: None,
                    memory_hints: Default::default(),
                    trace: wgpu::Trace::Off,
                },
            ).await.unwrap();

            let surface_caps = surface.get_capabilities(&adapter);

            let surface_format = surface_caps.formats.iter()
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

            surface.configure(&device, &config);

            let frame = surface
                .get_current_texture()
                .expect("Failed to acquire next swap chain texture");
            let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

            {
                let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
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

            queue.submit(Some(encoder.finish()));
            frame.present();
        });
    });

    view! {
        <canvas
            node_ref=canvas_node_ref
            style="width: 100vw; height: 100vh; display: block"
        ></canvas>
    }
}