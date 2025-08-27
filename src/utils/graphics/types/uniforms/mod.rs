pub mod camera_uniform;
pub mod triangle_uniform;

pub fn aligned_size<T>() -> wgpu::BufferAddress {
    let size = std::mem::size_of::<T>() as wgpu::BufferAddress;
    (size + 15) & !15 // round up to multiple of 16
}
