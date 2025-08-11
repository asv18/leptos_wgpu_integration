use crate::utils::helpers::js_functions::fetch_image_data;

#[allow(unused)]
pub struct Texture {
    pub texture: wgpu::Texture,
    pub sampler: wgpu::Sampler,
    pub view: wgpu::TextureView,
}

impl Texture {
    pub fn new_from_texture_data(
        queue: &wgpu::Queue,
        device: &wgpu::Device,
        texture_data: &[u8],
        texture_width: u32,
        texture_height: u32,
    ) -> Self {
        let size = wgpu::Extent3d {
            width: texture_width,
            height: texture_height,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &texture_data,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * texture_width),
                rows_per_image: Some(texture_height),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        Self {
            texture,
            sampler,
            view,
        }
    }

    pub async fn new_from_image(url: &str, queue: &wgpu::Queue, device: &wgpu::Device) -> anyhow::Result<Self, leptos::wasm_bindgen::JsValue> {
        let image_data = fetch_image_data(url).await?;

        let texture_data = image_data.data().0;
        let width = image_data.width();
        let height = image_data.height();

        Ok(Self::new_from_texture_data(queue, device, &texture_data, width, height))
    }
}
